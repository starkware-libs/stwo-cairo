use core::dict::Felt252Dict;
use core::iter::{IntoIterator, Iterator};
use core::nullable::NullableTrait;
use crate::channel::{Channel, ChannelTrait};
use crate::circle::CirclePoint;
use crate::poly::circle::{CircleEvaluation, SparseCircleEvaluation};
use crate::fields::m31::M31;
use crate::fields::qm31::{QM31, QM31Impl};
use crate::fri::{FOLD_FACTOR, FOLD_STEP, FriProof, FriVerifierImpl};
use crate::pcs::quotients::{PointSample, fri_answers};
use crate::queries::{SparseSubCircleDomainImpl, get_folded_query_positions};
use crate::utils::ArrayImpl;
use crate::vcs::hasher::PoseidonMerkleHasher;
use crate::vcs::verifier::{MerkleDecommitment, MerkleVerifier, MerkleVerifierTrait};
use crate::verifier::{FriVerificationErrorIntoVerificationError, VerificationError};
use crate::{ColumnArray, TreeArray};
use super::PcsConfig;

// TODO(andrew): Change all `Array` types to `Span`.
#[derive(Drop, Serde)]
pub struct CommitmentSchemeProof {
    pub commitments: TreeArray<felt252>,
    /// Sampled mask values.
    pub sampled_values: TreeArray<ColumnArray<Array<QM31>>>,
    pub decommitments: TreeArray<MerkleDecommitment<PoseidonMerkleHasher>>,
    /// All queried trace values.
    pub queried_values: TreeArray<ColumnArray<Array<M31>>>,
    pub missing_quotient_values: Span<QM31>,
    pub quotients_commitment: felt252,
    pub quotient_decommitment: MerkleDecommitment<PoseidonMerkleHasher>,
    pub proof_of_work_nonce: u64,
    pub fri_proof: FriProof,
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
    fn column_log_sizes(self: @CommitmentSchemeVerifier) -> Array<@Array<u32>> {
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
        log_sizes: @Array<u32>,
        ref channel: Channel,
    ) {
        channel.mix_digest(commitment);
        let mut extended_log_sizes = array![];
        for log_size in log_sizes.span() {
            extended_log_sizes.append(*log_size + self.config.fri_config.log_blowup_factor);
        };
        self
            .trees
            .append(MerkleVerifier { root: commitment, column_log_sizes: extended_log_sizes });
    }

    fn verify_values(
        self: @CommitmentSchemeVerifier,
        sampled_points: TreeArray<ColumnArray<Array<CirclePoint<QM31>>>>,
        proof: CommitmentSchemeProof,
        ref channel: Channel,
    ) -> Result<(), VerificationError> {
        let CommitmentSchemeProof { commitments: _,
        sampled_values,
        decommitments,
        queried_values,
        proof_of_work_nonce,
        missing_quotient_values,
        quotients_commitment,
        quotient_decommitment,
        fri_proof } =
            proof;

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
        let log_blowup_factor = *self.config.fri_config.log_blowup_factor;
        let column_log_bounds = get_column_log_bounds(@column_log_sizes, log_blowup_factor);

        channel.mix_digest(quotients_commitment);

        // FRI commitment phase on OODS quotients.
        let mut fri_verifier =
            match FriVerifierImpl::commit(
                ref channel, *self.config.fri_config, fri_proof, column_log_bounds,
            ) {
            Result::Ok(fri_verifier) => fri_verifier,
            Result::Err(err) => { return Result::Err(VerificationError::Fri(err)); },
        };

        // Verify proof of work.
        channel.mix_nonce(proof_of_work_nonce);

        if !channel.check_proof_of_work(*self.config.pow_bits) {
            return Result::Err(VerificationError::ProofOfWork);
        }

        // Get FRI query domains.
        let (
            mut queries_per_log_size, mut fri_query_domain_per_log_size, fri_query_domain_log_sizes,
        ) =
            fri_verifier
            .column_query_positions(ref channel);

        let n_trees = self.trees.len();

        // Verify merkle decommitments.
        let mut decommitments = decommitments.into_iter();

        let mut tree_i = 0;
        loop {
            if tree_i == n_trees {
                break Result::Ok(());
            }

            let tree = self.trees[tree_i];
            let decommitment = decommitments.next().unwrap();
            let queried_values = queried_values[tree_i];

            // TODO(andrew): Do better.
            let mut queries_per_log_size_cpy = Default::default();

            for log_size in fri_query_domain_log_sizes {
                let log_size = (*log_size).into();
                queries_per_log_size_cpy.insert(log_size, queries_per_log_size.get(log_size));
            };

            if let Result::Err(err) = tree
                .verify(queries_per_log_size_cpy, queried_values, decommitment) {
                break Result::Err(VerificationError::Merkle(err));
            }

            tree_i += 1;
        }?;

        // Check iterators have been fully consumed.
        assert!(decommitments.next().is_none());

        // Answer FRI queries.
        let samples = get_flattened_samples(sampled_points, sampled_values);

        let mut flattened_column_log_sizes = array![];

        for column_log_sizes in column_log_sizes {
            for log_size in column_log_sizes.span() {
                flattened_column_log_sizes.append(*log_size);
            };
        };

        // TODO(andrew): Flattening not nessesary. Check how costly.
        let flattened_query_values = get_flattened_query_values(queried_values);

        // TODO(andrew): Do better.
        let mut queries_per_log_size_cpy = Default::default();

        for log_size in fri_query_domain_log_sizes {
            let log_size = (*log_size).into();
            queries_per_log_size_cpy.insert(log_size, queries_per_log_size.get(log_size));
        };

        let fri_answers = fri_answers(
            @flattened_column_log_sizes,
            @samples,
            random_coeff,
            queries_per_log_size,
            @flattened_query_values,
            missing_quotient_values,
        )?;

        let mut quotient_column_log_sizes = array![];

        for log_size in fri_query_domain_log_sizes {
            quotient_column_log_sizes.append(*log_size);
            quotient_column_log_sizes.append(*log_size);
            quotient_column_log_sizes.append(*log_size);
            quotient_column_log_sizes.append(*log_size);
        };

        let quotients_tree = MerkleVerifier {
            root: quotients_commitment, column_log_sizes: quotient_column_log_sizes,
        };

        let (opening_positions_per_log_size, decomposed_fri_answers) = get_fri_answers_decommitment_values(fri_answers.span(), ref queries_per_log_size_cpy, fri_query_domain_log_sizes);
        if let Result::Err(err) = quotients_tree
                .verify(opening_positions_per_log_size, @decomposed_fri_answers, quotient_decommitment) {
                return Result::Err(VerificationError::Merkle(err));
            }

        if let Result::Err(err) = fri_verifier.decommit(fri_answers) {
            return Result::Err(VerificationError::Fri(err));
        }

        Result::Ok(())
    }
}

fn get_fri_answers_decommitment_values(
    fri_answers: Span<SparseCircleEvaluation>,
    ref queries_per_log_size: Felt252Dict<Nullable<Span<usize>>>,
    log_sizes: Span<u32>,
) -> (Felt252Dict<Nullable<Span<usize>>>, ColumnArray<Array<M31>>) {
    let mut opening_positions_per_log_size: Felt252Dict<Nullable<Span<usize>>> = Default::default();

    for log_size in log_sizes {
        let log_size = (*log_size).into();
        let query_positions = queries_per_log_size.get(log_size).deref();
        let mut opening_positions = array![];
        for folded_position in get_folded_query_positions(query_positions, FOLD_STEP) {
            let start_position = folded_position * FOLD_FACTOR;
            let end_position = start_position + FOLD_FACTOR;
            for position in start_position..end_position {
                opening_positions.append(position);
            };
        };
        opening_positions_per_log_size.insert(log_size, NullableTrait::new(opening_positions.span()));
    };

    let mut decomposed_fri_answers = array![];

    for fri_answers_secure_column in fri_answers {
        let mut c0 = array![];
        let mut c1 = array![];
        let mut c2 = array![];
        let mut c3 = array![];
        for subcircle_eval in fri_answers_secure_column.subcircle_evals.span() {
            for value in subcircle_eval.bit_reversed_values.span() {
                let [v0, v1, v2, v3] = (*value).to_array();
                c0.append(v0);
                c1.append(v1);
                c2.append(v2);
                c3.append(v3);
            }
        };
        decomposed_fri_answers.append(c0);
        decomposed_fri_answers.append(c1);
        decomposed_fri_answers.append(c2);
        decomposed_fri_answers.append(c3);
    };

    (opening_positions_per_log_size, decomposed_fri_answers)
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

#[inline]
fn get_flattened_query_values(
    query_values: TreeArray<ColumnArray<Array<M31>>>,
) -> ColumnArray<Array<M31>> {
    let mut res = array![];
    for query_values in query_values {
        for column_query_values in query_values {
            res.append(column_query_values);
        };
    };
    res
}
