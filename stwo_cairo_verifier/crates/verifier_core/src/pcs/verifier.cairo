use core::dict::Felt252Dict;
use core::iter::{IntoIterator, Iterator};
use crate::channel::{Channel, ChannelTrait};
use crate::circle::CirclePoint;
use crate::fields::m31::M31;
use crate::fields::qm31::{QM31, QM31Serde};
use crate::fri::{FriProof, FriVerifierImpl};
use crate::pcs::quotients::{PointSample, fri_answers};
use crate::utils::{ArrayImpl, DictImpl, columns_by_log_size};
use crate::vcs::MerkleHasher;
use crate::vcs::verifier::{MerkleDecommitment, MerkleVerifier, MerkleVerifierTrait};
use crate::verifier::{FriVerificationErrorIntoVerificationError, VerificationError};
use crate::{ColumnArray, ColumnSpan, Hash, TreeArray, TreeSpan};
use super::PcsConfig;

#[derive(Drop, Serde)]
pub struct CommitmentSchemeProof {
    pub config: PcsConfig,
    pub commitments: TreeSpan<Hash>,
    /// Sampled mask values.
    pub sampled_values: TreeSpan<ColumnSpan<Span<QM31>>>,
    pub decommitments: TreeArray<MerkleDecommitment<MerkleHasher>>,
    /// All queried trace values.
    pub queried_values: TreeArray<Span<M31>>,
    pub proof_of_work_nonce: u64,
    pub fri_proof: FriProof,
}


/// The verifier side of a FRI polynomial commitment scheme. See [super].
#[derive(Drop)]
pub struct CommitmentSchemeVerifier {
    pub trees: Array<MerkleVerifier<MerkleHasher>>,
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
        }
        res
    }

    /// Reads a commitment from the prover.
    fn commit(
        ref self: CommitmentSchemeVerifier,
        commitment: Hash,
        log_sizes: Span<u32>,
        ref channel: Channel,
    ) {
        channel.mix_root(commitment.clone());
        let mut extended_log_sizes = array![];
        for log_size in log_sizes {
            extended_log_sizes.append(*log_size + self.config.fri_config.log_blowup_factor);
        }

        let columns_by_log_size = columns_by_log_size(extended_log_sizes.span());
        self
            .trees
            .append(
                MerkleVerifier {
                    root: commitment, column_log_sizes: extended_log_sizes, columns_by_log_size,
                },
            );
    }

    fn verify_values(
        self: CommitmentSchemeVerifier,
        sampled_points: TreeArray<ColumnArray<Array<CirclePoint<QM31>>>>,
        proof: CommitmentSchemeProof,
        ref channel: Channel,
    ) -> Result<(), VerificationError> {
        let CommitmentSchemeProof {
            config: _,
            commitments: _,
            sampled_values,
            decommitments,
            queried_values,
            proof_of_work_nonce,
            fri_proof,
        } = proof;

        let mut flattened_sampled_values = array![];

        for sampled_values in sampled_values {
            for column_sampled_values in *sampled_values {
                for sampled_value in *column_sampled_values {
                    flattened_sampled_values.append(*sampled_value);
                };
            };
        }

        channel.mix_felts(flattened_sampled_values.span());

        let random_coeff = channel.draw_felt();
        let column_log_sizes = self.column_log_sizes();
        let fri_config = self.config.fri_config;
        let log_blowup_factor = fri_config.log_blowup_factor;
        let column_log_bounds = get_column_log_bounds(@column_log_sizes, log_blowup_factor).span();

        // FRI commitment phase on OODS quotients.
        let mut fri_verifier =
            match FriVerifierImpl::commit(ref channel, fri_config, fri_proof, column_log_bounds) {
            Ok(fri_verifier) => fri_verifier,
            Err(err) => { return Err(VerificationError::Fri(err)); },
        };

        // Verify proof of work.
        if !channel.mix_and_check_pow_nonce(self.config.pow_bits, proof_of_work_nonce) {
            return Err(VerificationError::QueriesProofOfWork);
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
                break Ok(());
            }

            let tree = self.trees[tree_i];
            let decommitment = decommitments.next().unwrap();
            let queried_values = *queried_values[tree_i];

            // The merkle implementation pops values from the query position dict so it has to be
            // duplicated.
            let query_positions = query_positions_by_log_size.clone_subset(unique_column_log_sizes);

            if let Err(err) = tree.verify(query_positions, queried_values, decommitment) {
                break Err(VerificationError::Merkle(err));
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

        if let Err(err) = fri_verifier.decommit(fri_answers) {
            return Err(VerificationError::Fri(err));
        }

        Ok(())
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
    }

    let mut bounds = array![];

    let mut i = MAX_LOG_BOUND;
    while i != 0 {
        if bounds_set.get(i.into()) {
            bounds.append(i);
        }
        i -= 1;
    }

    bounds
}

#[inline]
fn get_flattened_samples(
    sampled_points: TreeArray<ColumnArray<Array<CirclePoint<QM31>>>>,
    sampled_values: TreeSpan<ColumnSpan<Span<QM31>>>,
) -> ColumnArray<Array<PointSample>> {
    let mut res = array![];
    let n_trees = sampled_points.len();
    assert!(sampled_points.len() == sampled_values.len());

    let mut tree_i = 0;
    while tree_i < n_trees {
        let tree_points = sampled_points[tree_i];
        let tree_values = *sampled_values[tree_i];
        assert!(tree_points.len() == tree_values.len());
        let n_columns = tree_points.len();

        let mut column_i = 0;
        while column_i < n_columns {
            let column_points = tree_points[column_i];
            let column_values = *tree_values[column_i];

            let n_samples = column_points.len();
            assert!(column_points.len() == column_values.len());
            let mut column_samples = array![];

            let mut sample_i = 0;
            while sample_i < n_samples {
                let point = *column_points[sample_i];
                let value = *column_values[sample_i];
                column_samples.append(PointSample { point, value });
                sample_i += 1;
            }

            res.append(column_samples);
            column_i += 1;
        }

        tree_i += 1;
    }
    res
}
