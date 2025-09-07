use stwo_verifier_utils::zip_eq::zip_eq;
use crate::channel::{Channel, ChannelTrait};
use crate::circle::CirclePoint;
use crate::fields::m31::M31;
use crate::fields::qm31::{QM31, QM31Serde};
use crate::fri::{FriProof, FriVerifierTrait};
use crate::pcs::quotients::{PointSample, fri_answers};
use crate::utils::{
    ArrayImpl, ColumnsIndicesPerTreeByLogDegreeBound, DictImpl, group_columns_by_degree_bound,
    pad_and_transpose_columns_by_log_deg_bound_per_tree,
};
use crate::vcs::MerkleHasher;
use crate::vcs::verifier::{MerkleDecommitment, MerkleVerifier, MerkleVerifierTrait};
use crate::verifier::VerificationError;
use crate::{ColumnArray, ColumnSpan, Hash, TreeArray, TreeSpan, queries};
use super::PcsConfig;

/// Sanity check that the proof of work is not negligible.
pub const MIN_POW_BITS: u32 = 10;

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
    pub trees: TreeArray<MerkleVerifier<MerkleHasher>>,
    pub config: PcsConfig,
}

#[generate_trait]
pub impl CommitmentSchemeVerifierImpl of CommitmentSchemeVerifierTrait {
    fn new(config: PcsConfig) -> CommitmentSchemeVerifier {
        CommitmentSchemeVerifier { trees: array![], config }
    }

    /// Returns the column indices grouped by log degree bound (outer), then by tree (inner).
    fn column_indices_per_tree_by_degree_bound(
        self: @CommitmentSchemeVerifier,
    ) -> ColumnsIndicesPerTreeByLogDegreeBound {
        let mut columns_by_deg_bound_per_tree = array![];
        for tree in self.trees.span() {
            columns_by_deg_bound_per_tree.append(*tree.column_indices_by_deg_bound);
        }

        pad_and_transpose_columns_by_log_deg_bound_per_tree(columns_by_deg_bound_per_tree.span())
    }

    /// Reads a Merkle root commitment for a set of columns from the prover and registers it with
    /// the verifier.
    ///
    /// This function mixes the commitment root into the Fiat-Shamir channel and appends a new
    /// MerkleVerifier for this commitment to the verifier's state.
    ///
    /// # Arguments
    /// * `commitment` - The Merkle root of the committed columns.
    /// * `degree_bound_by_column` - The log degree bounds for each column in the commitment.
    /// * `channel` - The Fiat-Shamir channel used for mixing and randomness.
    fn commit(
        ref self: CommitmentSchemeVerifier,
        commitment: Hash,
        degree_bound_by_column: ColumnSpan<u32>,
        ref channel: Channel,
    ) {
        // Mix the commitment root into the Fiat-Shamir channel.
        channel.mix_commitment(commitment);

        let column_indices_by_deg_bound = group_columns_by_degree_bound(degree_bound_by_column);
        self
            .trees
            .append(
                MerkleVerifier {
                    root: commitment,
                    tree_height: self.config.fri_config.log_blowup_factor
                        + column_indices_by_deg_bound.len()
                        - 1,
                    column_indices_by_deg_bound,
                },
            );
    }

    /// Verifies that `proof.sampled_values` corresponds to the evaluations
    /// of the committed polynomials at `sampled_points`.
    ///
    /// For each column of the trace, `sampled_points` contains translations
    /// of the OOD point by a multiple of the coset step corresponding
    /// to that column.
    fn verify_values(
        self: CommitmentSchemeVerifier,
        sampled_points: TreeArray<ColumnArray<Array<CirclePoint<QM31>>>>,
        proof: CommitmentSchemeProof,
        ref channel: Channel,
    ) {
        let CommitmentSchemeProof {
            config: _,
            commitments: _,
            sampled_values,
            decommitments,
            queried_values,
            proof_of_work_nonce,
            fri_proof,
        } = proof;

        mix_sampled_values(sampled_values, ref channel);

        let random_coeff = channel.draw_secure_felt();
        let fri_config = self.config.fri_config;

        let column_indices_per_tree_by_degree_bound = self
            .column_indices_per_tree_by_degree_bound();

        let column_log_degree_bounds = get_column_log_degree_bounds(
            column_indices_per_tree_by_degree_bound,
        )
            .span();

        // FRI commitment phase on OODS quotients.
        let mut fri_verifier = FriVerifierTrait::commit(
            ref channel, fri_config, fri_proof, column_log_degree_bounds,
        );

        // Verify proof of work.
        assert!(self.config.pow_bits >= MIN_POW_BITS);
        assert!(
            channel.mix_and_check_pow_nonce(self.config.pow_bits, proof_of_work_nonce),
            "{}",
            VerificationError::QueriesProofOfWork,
        );

        // Get FRI query positions.
        let (unique_column_log_sizes, mut query_positions_by_log_size, queries) = fri_verifier
            .sample_query_positions(ref channel);

        // Verify Merkle decommitments.
        for (tree, (queried_values, decommitment)) in zip_eq(
            self.trees.span(), zip_eq(queried_values.span(), decommitments),
        ) {
            // The Merkle implementation pops values from the query position dict so it has to
            // be duplicated.
            let query_positions = query_positions_by_log_size.clone_subset(unique_column_log_sizes);

            tree.verify(query_positions, *queried_values, decommitment);
        }

        // Answer FRI queries.
        let samples = zip_samples(sampled_points, sampled_values);

        let fri_answers = fri_answers(
            column_indices_per_tree_by_degree_bound,
            fri_config.log_blowup_factor,
            samples,
            random_coeff,
            query_positions_by_log_size,
            queried_values.span(),
        );

        fri_verifier.decommit(fri_answers, queries);
    }

    /// Gets the trace log size from the commitment scheme. Temporarily workarounds a compiler issue.
    fn get_trace_log_size(self: CommitmentSchemeVerifier) -> u32 {
        // The following loop is a workaround for a compiler issue. Directly accessing the trees at
        // indices 1 and 2 causes the compiler to think this code will always panic.
        // The following loop doesn't directly access these indices, so the code won't panic.
        let mut second_tree_height: u32 = 0;
        // Setting third_tree_height to a large value ensures that if there is no third tree, the assert
        // below will fail.
        let mut third_tree_height: u32 = 1000000;
        for (tree_counter, tree) in self.trees.span().into_iter().enumerate() {
            match tree_counter {
                0 => {},
                1 => { second_tree_height = *tree.tree_height; },
                2 => { third_tree_height = *tree.tree_height; },
                3 => { break; },
                _ => {},
            }
        }
        assert!(second_tree_height == third_tree_height);
        second_tree_height
    }
}

#[inline]
fn mix_sampled_values(sampled_values: TreeSpan<ColumnSpan<Span<QM31>>>, ref channel: Channel) {
    let mut flattened_sampled_values = array![];

    for sampled_values in sampled_values {
        for column_sampled_values in *sampled_values {
            for sampled_value in *column_sampled_values {
                flattened_sampled_values.append(*sampled_value);
            };
        };
    }

    channel.mix_felts(flattened_sampled_values.span());
}

/// Returns all column log bounds sorted in descending order.
#[inline]
fn get_column_log_degree_bounds(
    mut column_indices_per_tree_by_degree_bound: ColumnsIndicesPerTreeByLogDegreeBound,
) -> Array<u32> {
    let mut degree_bounds = array![];
    let mut degree_bound = column_indices_per_tree_by_degree_bound.len();
    while let Some(columns_of_degree_bound_per_tree) = column_indices_per_tree_by_degree_bound
        .pop_back() {
        degree_bound -= 1;
        for columns_of_degree_bound_in_tree in columns_of_degree_bound_per_tree {
            if !(*columns_of_degree_bound_in_tree).is_empty() {
                degree_bounds.append(degree_bound);
                break;
            }
        }
    }

    degree_bounds
}


/// Given sampled points and values by column per tree, zips them into samples by column per tree.
#[inline]
fn zip_samples(
    sampled_points_by_column_per_tree: TreeArray<ColumnArray<Array<CirclePoint<QM31>>>>,
    sampled_values_by_column_per_tree: TreeSpan<ColumnSpan<Span<QM31>>>,
) -> TreeSpan<ColumnSpan<Array<PointSample>>> {
    zip_eq(sampled_points_by_column_per_tree, sampled_values_by_column_per_tree)
        .map(
            |tuple: (ColumnArray<Array<CirclePoint<QM31>>>, @ColumnSpan<Span<QM31>>)| -> ColumnSpan<
                Array<PointSample>,
            > {
                let (points_by_column, values_by_column) = tuple;
                zip_eq(points_by_column, *values_by_column)
                    .map(
                        |tuple: (Array<CirclePoint<QM31>>, @Span<QM31>)| -> Array<PointSample> {
                            let (points, values) = tuple;
                            zip_eq(points, *values)
                                .map(
                                    |tuple: (CirclePoint<QM31>, @QM31)| {
                                        let (point, value) = tuple;
                                        PointSample { point, value: *value }
                                    },
                                )
                                .collect()
                        },
                    )
                    .collect::<ColumnArray<Array<PointSample>>>()
                    .span()
            },
        )
        .collect::<TreeArray<ColumnSpan<Array<PointSample>>>>()
        .span()
}
