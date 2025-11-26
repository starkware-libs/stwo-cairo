use stwo_verifier_utils::zip_eq::zip_eq;
use crate::channel::{Channel, ChannelTrait};
use crate::circle::CirclePoint;
use crate::fields::m31::M31;
use crate::fields::qm31::{QM31, QM31Serde};
use crate::fri::{FriProof, FriVerifierTrait};
use crate::pcs::quotients::fri_answers;
use crate::utils::{
    ArrayImpl, ColumnsIndicesByLogDegreeBound, ColumnsIndicesPerTreeByLogDegreeBound, DictImpl,
    group_columns_by_degree_bound, pad_and_transpose_columns_by_log_deg_bound_per_tree,
};
use crate::vcs::MerkleHasher;
use crate::vcs::verifier::{MerkleDecommitment, MerkleVerifier, MerkleVerifierTrait};
use crate::verifier::VerificationError;
use crate::{ColumnSpan, Hash, TreeArray, TreeSpan, queries};
use super::PcsConfig;

/// Sanity check that the proof of work is not negligible.
pub const MIN_POW_BITS: u32 = 20;

/// Sampled mask values.
///
/// Each column is sampled at one of the following point sets:
/// - `[]` (indicating an unused preprocessed column)
/// - `[oods_point]`
/// - `[oods_point - g, oods_point]`
///   where `g` is the trace generator for that column.
///
/// `AIR::eval_composition_polynomial_at_point` ensures that each column has the correct
/// number of samples, while `CommitmentSchemeVerifierTrait::verify_values` ensures that the sampled
/// values themselves are valid.
pub type SampledValues = TreeSpan<ColumnSpan<Span<QM31>>>;

/// For each tree, stores all queried trace values, ordered first by descending column size,
/// then by column index, and finally by query position.
///
/// For illustration:
/// Suppose the first tree has two columns (c_1, c_3) with log degree bound 5,
/// one column (c_2) with log degree bound 4, and the queries are at positions 5 and 122.
///
/// In this case, `queried_values[0]` would be:
///
/// [
///     // size 5 columns
///     c_1@5, c_3@5, c_1@122, c_3@122,
///     // size 4 columns
///     c_2@fold(5), c_2@fold(122),
/// ],
///
/// where `c_1@5` denotes the value of column 1 at query position 5,
/// and `fold(5)` denotes the folded position of query 5.
pub type QueriedValues = TreeArray<Span<M31>>;

#[derive(Drop, Serde)]
pub struct CommitmentSchemeProof {
    pub config: PcsConfig,
    pub commitments: TreeSpan<Hash>,
    pub sampled_values: SampledValues,
    pub decommitments: TreeArray<MerkleDecommitment<MerkleHasher>>,
    pub queried_values: QueriedValues,
    pub proof_of_work_nonce: u64,
    pub fri_proof: FriProof,
}

/// The verifier side of a FRI polynomial commitment scheme. See [super].
#[derive(Drop)]
pub struct CommitmentSchemeVerifier {
    pub trees: TreeArray<MerkleVerifier<MerkleHasher>>,
}

#[generate_trait]
pub impl CommitmentSchemeVerifierImpl of CommitmentSchemeVerifierTrait {
    fn new() -> CommitmentSchemeVerifier {
        CommitmentSchemeVerifier { trees: array![] }
    }

    /// Returns the column indices grouped by log degree bound (outer), then by tree (inner).
    fn column_indices_per_tree_by_degree_bound(
        self: @CommitmentSchemeVerifier,
    ) -> ColumnsIndicesPerTreeByLogDegreeBound {
        let mut columns_by_log_deg_bound_per_tree = array![];
        for tree in self.trees.span() {
            columns_by_log_deg_bound_per_tree.append(*tree.column_indices_by_log_deg_bound);
        }

        pad_and_transpose_columns_by_log_deg_bound_per_tree(
            columns_by_log_deg_bound_per_tree.span(),
        )
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
        log_blowup_factor: u32,
    ) {
        // Mix the commitment root into the Fiat-Shamir channel.
        channel.mix_commitment(commitment);

        let column_indices_by_log_deg_bound = group_columns_by_degree_bound(degree_bound_by_column);
        self
            .trees
            .append(
                MerkleVerifier {
                    root: commitment,
                    tree_height: log_blowup_factor + column_indices_by_log_deg_bound.len() - 1,
                    column_indices_by_log_deg_bound,
                },
            );
    }

    /// Verifies that `proof.sampled_values` corresponds to the evaluations
    /// of the committed polynomials at `oods_point`.
    fn verify_values(
        self: CommitmentSchemeVerifier,
        oods_point: CirclePoint<QM31>,
        proof: CommitmentSchemeProof,
        ref channel: Channel,
    ) {
        let CommitmentSchemeProof {
            config,
            commitments: _,
            sampled_values,
            decommitments,
            queried_values: queried_values_per_tree,
            proof_of_work_nonce,
            fri_proof,
        } = proof;

        mix_sampled_values(sampled_values, ref channel);

        let random_coeff = channel.draw_secure_felt();
        let fri_config = config.fri_config;

        // For flat AIRs (with split composition polynomial), the FRI column log degree bounds can
        // be derived solely from the trace log degree bounds, since these bounds encompass the
        // bounds of both the interaction trace and the composition trace.
        let column_log_degree_bounds = get_column_log_degree_bounds(
            *self.trees[1].column_indices_by_log_deg_bound,
        )
            .span();

        // FRI commitment phase on OODS quotients.
        let mut fri_verifier = FriVerifierTrait::commit(
            ref channel, fri_config, fri_proof, column_log_degree_bounds,
        );

        // Verify proof of work.
        assert!(config.pow_bits >= MIN_POW_BITS);
        assert!(
            channel.verify_pow_nonce(config.pow_bits, proof_of_work_nonce),
            "{}",
            VerificationError::QueriesProofOfWork,
        );
        channel.mix_u64(proof_of_work_nonce);

        // Get FRI query positions.
        let (mut query_positions_by_log_size, queries) = fri_verifier
            .sample_query_positions(ref channel);

        // Verify Merkle decommitments.
        for (tree, (queried_values, decommitment)) in zip_eq(
            self.trees.span(), zip_eq(queried_values_per_tree.span(), decommitments),
        ) {
            tree.verify(ref query_positions_by_log_size, *queried_values, decommitment);
        }

        // Answer FRI queries.

        let fri_answers = fri_answers(
            self.column_indices_per_tree_by_degree_bound(),
            fri_config.log_blowup_factor,
            oods_point,
            sampled_values,
            random_coeff,
            query_positions_by_log_size,
            queried_values_per_tree,
        );

        fri_verifier.decommit(queries, fri_answers);
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

/// Retrieves the trace LDE log size from the commitment scheme’s tree array.
///
/// Marked with `#[inline(never)]` to avoid a const-folding bug in the compiler.
#[inline(never)]
pub fn get_trace_lde_log_size(
    commitment_scheme_trees: @TreeArray<MerkleVerifier<MerkleHasher>>,
) -> u32 {
    let boxed_triplet: @Box<[MerkleVerifier<MerkleHasher>; 3]> = commitment_scheme_trees
        .span()
        .try_into()
        .unwrap();
    let [_preprocessed_merkle_verifier, trace_merkle_verifier, interaction_trace_merkle_verifier] =
        boxed_triplet
        .as_snapshot()
        .unbox();

    let trace_lde_log_size = *trace_merkle_verifier.tree_height;
    assert!(trace_lde_log_size == *interaction_trace_merkle_verifier.tree_height);
    trace_lde_log_size
}

/// Returns all column log bounds sorted in descending order.
#[inline]
fn get_column_log_degree_bounds(
    mut column_indices_by_log_deg_bound: ColumnsIndicesByLogDegreeBound,
) -> Array<u32> {
    let mut log_degree_bounds = array![];
    let mut log_degree_bound = column_indices_by_log_deg_bound.len();
    while let Some(columns_of_degree_bound_per_tree) = column_indices_by_log_deg_bound.pop_back() {
        log_degree_bound -= 1;

        if !columns_of_degree_bound_per_tree.is_empty() {
            log_degree_bounds.append(log_degree_bound);
        }
    }

    log_degree_bounds
}
