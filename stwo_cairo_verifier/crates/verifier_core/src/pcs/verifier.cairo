use stwo_verifier_utils::zip_eq::zip_eq;
use core::iter::{IntoIterator, Iterator};
use crate::channel::{Channel, ChannelTrait};
use crate::circle::CirclePoint;
use crate::fields::m31::M31;
use crate::fields::qm31::{QM31, QM31Serde};
use crate::fri::{FriProof, FriVerifierImpl};
use crate::pcs::quotients::{PointSample, fri_answers};
use crate::utils::{
    ArrayImpl, ColumnsIndicesPerTreeByDegreeBound, DictImpl, group_columns_by_degree_bound,
    pad_and_transpose_columns_by_deg_bound_per_tree,
};
use crate::vcs::MerkleHasher;
use crate::vcs::verifier::{MerkleDecommitment, MerkleVerifier, MerkleVerifierTrait};
use crate::verifier::VerificationError;
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

    /// Returns the column indices grouped by log degree bound (outer), then by tree (inner).
    fn column_indices_per_tree_by_degree_bound(
        self: @CommitmentSchemeVerifier,
    ) -> ColumnsIndicesPerTreeByDegreeBound {
        let mut columns_by_deg_bound_per_tree = array![];
        for tree in self.trees.span() {
            columns_by_deg_bound_per_tree.append(*tree.column_indices_by_deg_bound);
        }

        pad_and_transpose_columns_by_deg_bound_per_tree(columns_by_deg_bound_per_tree.span())
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
        channel.mix_root(commitment);

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

        let mut flattened_sampled_values = array![];

        for sampled_values in sampled_values {
            for column_sampled_values in *sampled_values {
                for sampled_value in *column_sampled_values {
                    flattened_sampled_values.append(*sampled_value);
                };
            };
        }

        channel.mix_felts(flattened_sampled_values.span());

        let random_coeff = channel.draw_secure_felt();
        let fri_config = self.config.fri_config;

        let column_indices_per_tree_by_degree_bound = self
            .column_indices_per_tree_by_degree_bound();

        let column_log_bounds = get_column_log_bounds(column_indices_per_tree_by_degree_bound)
            .span();

        // FRI commitment phase on OODS quotients.
        let mut fri_verifier = FriVerifierImpl::commit(
            ref channel, fri_config, fri_proof, column_log_bounds,
        );

        // Verify proof of work.
        assert!(
            channel.mix_and_check_pow_nonce(self.config.pow_bits, proof_of_work_nonce),
            "{}",
            VerificationError::QueriesProofOfWork,
        );

        // Get FRI query positions.
        let (unique_column_log_sizes, mut query_positions_by_log_size) = fri_verifier
            .sample_query_positions(ref channel);

        // Verify Merkle decommitments.
        for (tree, (queried_values, decommitment)) in zip_eq(self.trees.span(), zip_eq(queried_values.span(), decommitments)) {
            // The Merkle implementation pops values from the query position dict so it has to
            // be duplicated.
            let query_positions = query_positions_by_log_size.clone_subset(unique_column_log_sizes);

            tree.verify(query_positions, *queried_values, decommitment);
        }

        // Answer FRI queries.
        let samples = get_flattened_samples(sampled_points, sampled_values);

        let fri_answers = fri_answers(
            column_indices_per_tree_by_degree_bound,
            fri_config.log_blowup_factor,
            samples,
            random_coeff,
            query_positions_by_log_size,
            queried_values.span(),
        );

        fri_verifier.decommit(fri_answers);
    }
}

/// Returns all column log bounds sorted in descending order.
#[inline]
fn get_column_log_bounds(
    mut column_indices_per_tree_by_degree_bound: ColumnsIndicesPerTreeByDegreeBound,
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

#[inline]
fn get_flattened_samples(
    sampled_points: TreeArray<ColumnArray<Array<CirclePoint<QM31>>>>,
    mut sampled_values: TreeSpan<ColumnSpan<Span<QM31>>>,
) -> TreeSpan<ColumnSpan<Array<PointSample>>> {
    let mut res = array![];
    assert!(sampled_points.len() == sampled_values.len());
    for (tree_points, tree_values) in sampled_points.span().into_iter().zip(sampled_values) {
        let mut tree_samples = array![];
        assert!(tree_points.len() == tree_values.len());
        for (column_points, column_values) in tree_points.into_iter().zip(tree_values) {
            let mut column_samples = array![];
            assert!(column_points.len() == column_values.len());
            for (point, value) in column_points.into_iter().zip(column_values) {
                column_samples.append(PointSample { point: *point, value: *value });
            }

            tree_samples.append(column_samples);
        }
        res.append(tree_samples.span());
    }
    res.span()
}
