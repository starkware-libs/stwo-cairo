use core::dict::Felt252Dict;
use core::iter::{IntoIterator, Iterator};
use crate::channel::{Channel, ChannelTrait};
use crate::circle::CirclePoint;
use crate::fields::m31::M31;
use crate::fields::qm31::{QM31, QM31Impl};
use crate::fri::{FriProof, FriVerifierImpl};
use crate::pcs::quotients::{PointSample, fri_answers};
use crate::utils::{ArrayImpl, DictImpl};
use crate::vcs::hasher::PoseidonMerkleHasher;
use crate::vcs::verifier::{MerkleDecommitment, MerkleVerifier, MerkleVerifierTrait};
use crate::verifier::{FriVerificationErrorIntoVerificationError, VerificationError};
use crate::{ColumnArray, TreeArray};
use super::PcsConfig;

// TODO(andrew): Change all `Array` types to `Span`.
#[derive(Drop)]
pub struct CommitmentSchemeProof<HashT> {
    pub commitments: TreeArray<HashT>,
    /// Sampled mask values.
    pub sampled_values: TreeArray<ColumnArray<Array<QM31>>>,
    pub decommitments: TreeArray<MerkleDecommitment<PoseidonMerkleHasher>>,
    /// All queried trace values.
    pub queried_values: TreeArray<Span<M31>>,
    pub proof_of_work_nonce: u64,
    pub fri_proof: FriProof,
}


impl CommitmentSchemeProofSerde<
    HashT, +Drop<HashT>, +core::serde::Serde<HashT>, +core::traits::Destruct<HashT>,
> of core::serde::Serde<CommitmentSchemeProof<HashT>> {
    fn serialize(self: @CommitmentSchemeProof<HashT>, ref output: Array<felt252>) {
        self.commitments.serialize(ref output);
        self.sampled_values.serialize(ref output);
        self.decommitments.serialize(ref output);
        self.queried_values.serialize(ref output);
        self.proof_of_work_nonce.serialize(ref output);
        self.fri_proof.serialize(ref output);
    }

    fn deserialize(ref serialized: Span<felt252>) -> Option<CommitmentSchemeProof<HashT>> {
        Option::Some(
            CommitmentSchemeProof {
                commitments: Serde::<TreeArray<HashT>>::deserialize(ref serialized)?,
                sampled_values: Serde::<
                    TreeArray<ColumnArray<Array<QM31>>>,
                >::deserialize(ref serialized)?,
                decommitments: Serde::<
                    TreeArray<MerkleDecommitment<PoseidonMerkleHasher>>,
                >::deserialize(ref serialized)?,
                queried_values: Serde::<TreeArray<Span<M31>>>::deserialize(ref serialized)?,
                proof_of_work_nonce: Serde::<u64>::deserialize(ref serialized)?,
                fri_proof: Serde::<FriProof>::deserialize(ref serialized)?,
            },
        )
    }
}


/// The verifier side of a FRI polynomial commitment scheme. See [super].
// TODO(andrew): Make generic on MerkleChannel.
#[derive(Drop)]
pub struct CommitmentSchemeVerifier {
    pub trees: Array<MerkleVerifier<PoseidonMerkleHasher>>,
    pub config: PcsConfig,
}

#[generate_trait]
pub impl CommitmentSchemeVerifierImpl of CommitmentSchemeVerifierTrait {
    fn new(config: PcsConfig) -> CommitmentSchemeVerifier {
        CommitmentSchemeVerifier { trees: array![], config }
    }

    /// Returns the log sizes of each column in each commitment tree.
    fn column_log_sizes(self: @CommitmentSchemeVerifier) -> TreeArray<@Array<u32>> {
        let mut res = array![];
        for tree in self.trees.span() {
            res.append(tree.column_log_sizes);
        };
        res
    }

    /// Reads a commitment from the prover.
    // TODO(andrew): Make commitment MerkleHasher hash type.
    // TODO(andrew): Make channel MerkleChannel generic channel.
    fn commit(
        ref self: CommitmentSchemeVerifier,
        commitment: felt252,
        log_sizes: Span<u32>,
        ref channel: Channel,
    ) {
        channel.mix_digest(commitment);
        let mut extended_log_sizes = array![];
        for log_size in log_sizes {
            extended_log_sizes.append(*log_size + self.config.fri_config.log_blowup_factor);
        };
        self
            .trees
            .append(MerkleVerifier { root: commitment, column_log_sizes: extended_log_sizes });
    }

    fn verify_values(
        self: CommitmentSchemeVerifier,
        sampled_points: TreeArray<ColumnArray<Array<CirclePoint<QM31>>>>,
        proof: CommitmentSchemeProof<felt252>,
        ref channel: Channel,
    ) -> Result<(), VerificationError> {
        let CommitmentSchemeProof {
            commitments: _,
            sampled_values,
            decommitments,
            queried_values,
            proof_of_work_nonce,
            fri_proof,
        } = proof;

        let mut flattened_sampled_values = array![];

        for sampled_values in sampled_values.span() {
            for column_sampled_values in sampled_values.span() {
                for sampled_value in column_sampled_values.span() {
                    flattened_sampled_values.append(*sampled_value);
                };
            };
        };

        channel.mix_felts(flattened_sampled_values.span());

        let random_coeff = channel.draw_felt();
        let column_log_sizes = self.column_log_sizes();
        let fri_config = self.config.fri_config;
        let log_blowup_factor = fri_config.log_blowup_factor;
        let column_log_bounds = get_column_log_bounds(@column_log_sizes, log_blowup_factor).span();

        // FRI commitment phase on OODS quotients.
        let mut fri_verifier =
            match FriVerifierImpl::commit(ref channel, fri_config, fri_proof, column_log_bounds) {
            Result::Ok(fri_verifier) => fri_verifier,
            Result::Err(err) => { return Result::Err(VerificationError::Fri(err)); },
        };

        // Verify proof of work.
        channel.mix_nonce(proof_of_work_nonce);

        if !channel.check_proof_of_work(self.config.pow_bits) {
            return Result::Err(VerificationError::ProofOfWork);
        }

        // Get FRI query positions.
        let (unique_column_log_sizes, mut query_positions_by_log_size) = fri_verifier
            .sample_query_positions(ref channel);

        // Verify merkle decommitments.
        let mut decommitments = decommitments.into_iter();

        let n_trees = self.trees.len();
        let mut tree_i = 0;
        loop {
            if tree_i == n_trees {
                break Result::Ok(());
            }

            let tree = self.trees[tree_i];
            let decommitment = decommitments.next().unwrap();
            let queried_values = *queried_values[tree_i];

            // TODO(andrew): Unfortunately the current merkle implementation pops values from the
            // query position dict so it has to be duplicated.
            if let Result::Err(err) = tree
                .verify(
                    query_positions_by_log_size.clone_subset(unique_column_log_sizes),
                    queried_values,
                    decommitment,
                ) {
                break Result::Err(VerificationError::Merkle(err));
            }

            tree_i += 1;
        }?;

        // Check iterators have been fully consumed.
        assert!(decommitments.next().is_none());

        // Answer FRI queries.
        let samples = get_flattened_samples(sampled_points, sampled_values);

        let fri_answers = fri_answers(
            column_log_sizes.span(),
            samples.span(),
            random_coeff,
            query_positions_by_log_size,
            queried_values,
        )?;

        if let Result::Err(err) = fri_verifier.decommit(fri_answers) {
            return Result::Err(VerificationError::Fri(err));
        }

        Result::Ok(())
    }
}

/// Returns all column log bounds deduped and sorted in ascending order.
#[inline]
fn get_column_log_bounds(
    column_log_sizes: @TreeArray<@ColumnArray<u32>>, log_blowup_factor: u32,
) -> Array<u32> {
    // The max column degree bound.
    const MAX_LOG_BOUND: u32 = 29;

    let mut bounds_set: Felt252Dict<bool> = Default::default();

    for tree_column_log_sizes in column_log_sizes.span() {
        for column_log_size in (*tree_column_log_sizes).span() {
            let column_log_bound = *column_log_size - log_blowup_factor;
            assert!(column_log_bound <= MAX_LOG_BOUND);
            bounds_set.insert(column_log_bound.into(), true);
        };
    };

    let mut bounds = array![];

    let mut i = MAX_LOG_BOUND;
    while i != 0 {
        if bounds_set.get(i.into()) {
            bounds.append(i);
        }
        i -= 1;
    };

    bounds
}

#[inline]
fn get_flattened_samples(
    sampled_points: TreeArray<ColumnArray<Array<CirclePoint<QM31>>>>,
    sampled_values: TreeArray<ColumnArray<Array<QM31>>>,
) -> ColumnArray<Array<PointSample>> {
    let mut res = array![];
    let n_trees = sampled_points.len();
    assert!(sampled_points.len() == sampled_values.len());

    let mut tree_i = 0;
    while tree_i < n_trees {
        let tree_points = sampled_points[tree_i];
        let tree_values = sampled_values[tree_i];
        assert!(tree_points.len() == tree_values.len());
        let n_columns = tree_points.len();

        let mut column_i = 0;
        while column_i < n_columns {
            let column_points = tree_points[column_i];
            let column_values = tree_values[column_i];

            let n_samples = column_points.len();
            assert!(column_points.len() == column_values.len());
            let mut column_samples = array![];

            let mut sample_i = 0;
            while sample_i < n_samples {
                let point = *column_points[sample_i];
                let value = *column_values[sample_i];
                column_samples.append(PointSample { point, value });
                sample_i += 1;
            };

            res.append(column_samples);
            column_i += 1;
        };

        tree_i += 1;
    };
    res
}
