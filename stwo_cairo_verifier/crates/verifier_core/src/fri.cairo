use core::array::SpanIter;
use core::dict::Felt252Dict;
use core::iter::{IntoIterator, Iterator};
use stwo_verifier_utils::zip_eq::zip_eq;
use crate::channel::{Channel, ChannelTrait};
use crate::circle::CosetImpl;
use crate::fields::BatchInvertible;
use crate::fields::qm31::{QM31, QM31Serde, QM31Trait, QM31_EXTENSION_DEGREE};
use crate::poly::circle::{CanonicCosetImpl, CircleDomain, CircleDomainImpl};
use crate::poly::line::{LineDomain, LineDomainImpl, LineEvaluationImpl, LinePoly};
use crate::poly::utils::fri_fold;
use crate::queries::{Queries, QueriesImpl};
use crate::utils::{
    ArrayImpl, OptionImpl, SpanExTrait, bit_reverse_index, group_columns_by_degree_bound, pow2,
};
use crate::vcs::MerkleHasher;
use crate::vcs::verifier::{MerkleDecommitment, MerkleVerifier, MerkleVerifierTrait};
use crate::{ColumnArray, Hash};

/// Fold step size for circle polynomials.
pub const CIRCLE_TO_LINE_FOLD_STEP: u32 = 1;

/// Equals `2^CIRCLE_TO_LINE_FOLD_STEP`.
pub const CIRCLE_TO_LINE_FOLD_FACTOR: usize = 2;

/// Fold step size for univariate polynomials.
pub const FOLD_STEP: u32 = 1;

/// Equals `2^FOLD_STEP`.
pub const FOLD_FACTOR: usize = 2;

#[derive(Drop, Serde, Copy)]
pub struct FriConfig {
    pub log_blowup_factor: u32,
    pub log_last_layer_degree_bound: u32,
    pub n_queries: usize,
}
#[generate_trait]
pub impl FriConfigImpl of FriConfigTrait {
    fn mix_into(self: @FriConfig, ref channel: Channel) {
        let FriConfig { log_blowup_factor, log_last_layer_degree_bound, n_queries } = self;
        channel.mix_u64((*log_blowup_factor).into());
        channel.mix_u64((*n_queries).into());
        channel.mix_u64((*log_last_layer_degree_bound).into());
    }
}

#[derive(Drop)]
pub struct FriVerifier {
    config: FriConfig,
    first_layer: FriFirstLayerVerifier,
    inner_layers: Array<FriInnerLayerVerifier>,
    last_layer_domain: LineDomain,
    last_layer_poly: LinePoly,
}

#[generate_trait]
pub impl FriVerifierImpl of FriVerifierTrait {
    /// Verifies the commitment stage of FRI.
    ///
    /// `column_log_bounds` should be the committed circle polynomial log
    /// degree bounds in descending order.
    fn commit(
        ref channel: Channel, config: FriConfig, proof: FriProof, column_log_bounds: Span<u32>,
    ) -> FriVerifier {
        let FriProof {
            first_layer: first_layer_proof, inner_layers: mut inner_layer_proofs, last_layer_poly,
        } = proof;

        channel.mix_commitment(first_layer_proof.commitment);

        let mut column_commitment_domains = array![];

        for column_log_bound in column_log_bounds {
            let commitment_domain_log_size = *column_log_bound + config.log_blowup_factor;
            let commitment_domain = CanonicCosetImpl::new(commitment_domain_log_size)
                .circle_domain();
            column_commitment_domains.append(commitment_domain);
        }

        let first_layer = FriFirstLayerVerifier {
            log_blowup_factor: config.log_blowup_factor,
            column_log_bounds,
            column_commitment_domains: column_commitment_domains.span(),
            proof: first_layer_proof,
            folding_alpha: channel.draw_secure_felt(),
        };

        // Bounds are stored in descending order.
        let max_column_log_bound = *column_log_bounds.first().unwrap();

        let mut inner_layers = array![];
        let mut layer_log_bound = max_column_log_bound - CIRCLE_TO_LINE_FOLD_STEP;
        let mut layer_domain = LineDomainImpl::new_unchecked(
            CosetImpl::half_odds(layer_log_bound + config.log_blowup_factor),
        );

        for (layer_index, layer_proof) in inner_layer_proofs.into_iter().enumerate() {
            channel.mix_commitment(*layer_proof.commitment);

            inner_layers
                .append(
                    FriInnerLayerVerifier {
                        log_degree_bound: layer_log_bound,
                        domain: layer_domain,
                        folding_alpha: channel.draw_secure_felt(),
                        layer_index,
                        proof: layer_proof,
                    },
                );

            layer_log_bound -= FOLD_STEP;

            layer_domain = layer_domain.double();
        }
        assert!(
            layer_log_bound == config.log_last_layer_degree_bound,
            "{}",
            FriVerificationError::InvalidNumFriLayers,
        );

        assert!(
            last_layer_poly.log_size == config.log_last_layer_degree_bound,
            "{}",
            FriVerificationError::LastLayerDegreeInvalid,
        );

        channel.mix_felts(last_layer_poly.coeffs.span());

        FriVerifier {
            config, first_layer, inner_layers, last_layer_domain: layer_domain, last_layer_poly,
        }
    }

    /// Verifies the decommitment stage of FRI.
    ///
    /// The first layer query evals need to be provided in the same order as their commitment.
    fn decommit(
        self: FriVerifier, queries: Queries, first_layer_query_evals: ColumnArray<Span<QM31>>,
    ) {
        let first_layer_sparse_evals = decommit_first_layer(
            @self, queries, first_layer_query_evals,
        );

        let inner_layer_queries = queries.fold(CIRCLE_TO_LINE_FOLD_STEP);

        let (last_layer_queries, last_layer_query_evals) = decommit_inner_layers(
            @self, inner_layer_queries, first_layer_sparse_evals,
        );

        decommit_last_layer(self, last_layer_queries, last_layer_query_evals)
    }

    /// Samples and returns query positions mapped by column log size.
    ///
    /// Output is of the form `(unique_log_sizes, queries_by_log_size)`.
    fn sample_query_positions(
        self: @FriVerifier, ref channel: Channel,
    ) -> (Felt252Dict<Nullable<Span<usize>>>, Queries) {
        // The sizes of input circle polynomial commitment domains.
        let mut column_log_sizes = array![];

        for column_commitment_domain in self.first_layer.column_commitment_domains {
            column_log_sizes.append(column_commitment_domain.log_size());
        }

        let column_log_sizes = column_log_sizes.span();

        // Column are sorted in descending order by size.
        let max_column_log_size = *column_log_sizes.first().unwrap();
        let n_queries = *self.config.n_queries;
        let queries = QueriesImpl::generate(ref channel, max_column_log_size, n_queries);

        (get_query_positions_by_log_size(queries, column_log_sizes), queries)
    }
}

/// Verifies the first layer decommitment.
///
/// Returns the queries and first layer folded column evaluations needed for
/// verifying the remaining layers.
fn decommit_first_layer(
    verifier: @FriVerifier, queries: Queries, first_layer_query_evals: ColumnArray<Span<QM31>>,
) -> ColumnArray<SparseEvaluation> {
    verifier.first_layer.verify(queries, first_layer_query_evals)
}

/// Verifies all inner layer decommitments.
///
/// Returns the queries and query evaluations needed for verifying the last FRI layer.
fn decommit_inner_layers(
    verifier: @FriVerifier,
    queries: Queries,
    mut first_layer_sparse_evals: ColumnArray<SparseEvaluation>,
) -> (Queries, Array<QM31>) {
    let mut first_layer_column_bounds = *verifier.first_layer.column_log_bounds;
    let mut first_layer_iter = zip_eq(
        first_layer_sparse_evals, *verifier.first_layer.column_commitment_domains,
    );
    let (Some((sparse_eval, column_domain)), Some(_first_layer_log_degree_bound)) = (
        first_layer_iter.next(), first_layer_column_bounds.pop_front(),
    ) else {
        panic!("at least one inner layer required");
    };

    let mut layer_query_evals = sparse_eval
        .fold_circle(*verifier.first_layer.folding_alpha, *column_domain);

    let mut layer_queries = queries;
    for layer in verifier.inner_layers.span() {
        let (folded_queries, mut folded_query_evals) = layer
            .verify_and_fold(queries: layer_queries, evals_at_queries: layer_query_evals.span());

        // If the first layer commitment contains a circle polynomial with the same degree bound as
        // this layer (prior to folding), fold it into the current layer.
        if first_layer_column_bounds.next_if_eq(layer.log_degree_bound).is_some() {
            let (sparse_eval, column_domain) = first_layer_iter.next().unwrap();

            let folding_alpha = *layer.folding_alpha;
            let mut folded_column_evals = sparse_eval.fold_circle(folding_alpha, *column_domain);
            let folding_alpha_squared = folding_alpha * folding_alpha;
            folded_query_evals = zip_eq(folded_query_evals, folded_column_evals)
                .map(
                    |tuple| {
                        let (folded_eval, folded_column_eval) = tuple;
                        QM31Trait::fused_mul_add(
                            folded_eval, folding_alpha_squared, folded_column_eval,
                        )
                    },
                )
                .collect();
        }

        layer_queries = folded_queries;
        layer_query_evals = folded_query_evals;
    }

    // Check all values have been consumed.
    assert!(first_layer_column_bounds.is_empty());
    assert!(first_layer_iter.next().is_none());

    (layer_queries, layer_query_evals)
}

/// Verifies the last layer.
#[inline]
fn decommit_last_layer(verifier: FriVerifier, mut queries: Queries, mut query_evals: Array<QM31>) {
    let FriVerifier { last_layer_poly, .. } = verifier;

    let single_value_box: Box<[QM31; 1]> = *last_layer_poly
        .coeffs
        .span()
        .try_into()
        .unwrap_or_else(|| panic!("{}", FriVerificationError::LastLayerLogDegreeMustBeZero));
    let [expected_last_layer_value] = single_value_box.unbox();

    for query_eval in query_evals {
        assert!(
            query_eval == expected_last_layer_value,
            "{}",
            FriVerificationError::LastLayerEvaluationsInvalid,
        );
    }
}

/// Returns the column query positions needed for verification.
///
/// The column log sizes must be unique and in descending order.
/// Returned column query positions are mapped by their log size.
fn get_query_positions_by_log_size(
    mut queries: Queries, mut unique_column_log_sizes: Span<u32>,
) -> Felt252Dict<Nullable<Span<usize>>> {
    let mut query_positions_by_log_size: Felt252Dict<Nullable<Span<usize>>> = Default::default();

    for column_log_size in unique_column_log_sizes {
        let n_folds = queries.log_domain_size - *column_log_size;

        if n_folds != 0 {
            queries = queries.fold(n_folds);
        }

        query_positions_by_log_size
            .insert((*column_log_size).into(), NullableTrait::new(queries.positions));
    }

    query_positions_by_log_size
}

/// A FRI proof.
#[derive(Drop, Serde)]
pub struct FriProof {
    pub first_layer: FriLayerProof,
    pub inner_layers: Span<FriLayerProof>,
    pub last_layer_poly: LinePoly,
}

#[derive(Drop)]
struct FriFirstLayerVerifier {
    /// The log blowup factor for all the columns in the first layer.
    log_blowup_factor: u32,
    /// The list of degree bounds of all circle polynomials committed in the first layer.
    column_log_bounds: Span<u32>,
    /// The commitment domains of all the circle polynomials in the first layer, sorted in
    /// descending order by size.
    column_commitment_domains: Span<CircleDomain>,
    folding_alpha: QM31,
    proof: FriLayerProof,
}

#[generate_trait]
impl FriFirstLayerVerifierImpl of FriFirstLayerVerifierTrait {
    /// Verifies the first layer's Merkle decommitment, and returns the evaluations needed for
    /// folding the columns to their corresponding layer.
    ///
    /// # Panics
    ///
    /// Panics if:
    /// * The proof doesn't store enough evaluations.
    /// * The queries are sampled on the wrong domain.
    /// * There are an invalid number of provided column evals.
    /// * The Merkle decommitment is invalid.
    fn verify(
        self: @FriFirstLayerVerifier,
        queries: Queries,
        mut query_evals_by_column: ColumnArray<Span<QM31>>,
    ) -> ColumnArray<SparseEvaluation> {
        // Columns are provided in descending order by size.
        let max_column_log_size = (*self.column_commitment_domains).first().unwrap().log_size();
        assert!(queries.log_domain_size == max_column_log_size);

        let mut column_queries = queries;
        let mut column_commitment_domains = *self.column_commitment_domains;
        let mut fri_witness = (*self.proof.fri_witness).into_iter();
        let mut decommitment_positions_by_log_size: Felt252Dict = Default::default();
        // For decommitment, each QM31 col must be split into its constituent M31 coordinate cols.
        let mut degree_bound_by_column = array![];
        let mut sparse_evals_by_column = array![];
        let mut decommitted_values = array![];

        for (column_domain, column_query_evals) in zip_eq(
            column_commitment_domains, query_evals_by_column,
        ) {
            let column_domain_log_size = column_domain.log_size();
            let n_folds = column_queries.log_domain_size - column_domain_log_size;

            if n_folds != 0 {
                column_queries = column_queries.fold(n_folds);
            }

            let (column_decommitment_positions, sparse_evaluation) =
                compute_decommitment_positions_and_rebuild_evals(
                column_queries, column_query_evals, ref fri_witness, CIRCLE_TO_LINE_FOLD_STEP,
            );

            // Columns of the same size have the same decommitment positions.
            // TODO(andrew): Do without nullable.
            decommitment_positions_by_log_size
                .insert(
                    column_domain_log_size.into(),
                    NullableTrait::new(column_decommitment_positions),
                );

            for subset_eval in sparse_evaluation.subset_evals.span() {
                for eval in subset_eval.span() {
                    // Split the QM31 into its M31 coordinate values.
                    let [v0, v1, v2, v3] = (*eval).to_fixed_array();
                    decommitted_values.append(v0.into());
                    decommitted_values.append(v1.into());
                    decommitted_values.append(v2.into());
                    decommitted_values.append(v3.into());
                };
            }

            let column_degree_bound = column_domain_log_size - *self.log_blowup_factor;
            degree_bound_by_column.append(column_degree_bound);
            degree_bound_by_column.append(column_degree_bound);
            degree_bound_by_column.append(column_degree_bound);
            degree_bound_by_column.append(column_degree_bound);

            sparse_evals_by_column.append(sparse_evaluation);
        }
        // Check all proof evals have been consumed.
        assert!(
            fri_witness.next().is_none(), "{}", FriVerificationError::FirstLayerEvaluationsInvalid,
        );

        let column_indices_by_log_deg_bound = group_columns_by_degree_bound(
            degree_bound_by_column.span(),
        );
        let merkle_verifier = MerkleVerifier {
            root: *self.proof.commitment,
            tree_height: max_column_log_size,
            column_indices_by_log_deg_bound,
        };

        merkle_verifier
            .verify(
                ref decommitment_positions_by_log_size,
                decommitted_values.span(),
                self.proof.decommitment.clone(),
            );

        sparse_evals_by_column
    }
}

#[derive(Drop, Debug)]
struct FriInnerLayerVerifier {
    log_degree_bound: u32,
    domain: LineDomain,
    folding_alpha: QM31,
    layer_index: usize,
    proof: @FriLayerProof,
}

#[generate_trait]
impl FriInnerLayerVerifierImpl of FriInnerLayerVerifierTrait {
    /// Verifies the layer's Merkle decommitment and returns the the folded queries and query evals.
    ///
    /// # Panics
    ///
    /// Panics if the Merkle decommitment is invalid.
    fn verify_and_fold(
        self: @FriInnerLayerVerifier, queries: Queries, evals_at_queries: Span<QM31>,
    ) -> (Queries, Array<QM31>) {
        assert!(queries.log_domain_size == self.domain.log_size());

        let mut fri_witness = (**self.proof.fri_witness).into_iter();

        let (decommitment_positions, sparse_evaluation) =
            compute_decommitment_positions_and_rebuild_evals(
            queries, evals_at_queries, ref fri_witness, FOLD_STEP,
        );

        // Check all proof evals have been consumed.
        assert!(
            fri_witness.next().is_none(), "{}", FriVerificationError::InnerLayerEvaluationsInvalid,
        );

        let mut decommitted_values = array![];
        for subset_eval in sparse_evaluation.subset_evals.span() {
            for eval in subset_eval.span() {
                // Split the QM31 into its M31 coordinate values.
                let [v0, v1, v2, v3] = (*eval).to_fixed_array();
                decommitted_values.append(v0.into());
                decommitted_values.append(v1.into());
                decommitted_values.append(v2.into());
                decommitted_values.append(v3.into());
            };
        }

        let column_log_size = self.domain.log_size();
        let degree_bound_by_column = ArrayImpl::new_repeated(
            n: QM31_EXTENSION_DEGREE, v: *self.log_degree_bound,
        );
        let column_indices_by_log_deg_bound = group_columns_by_degree_bound(
            degree_bound_by_column.span(),
        );
        let merkle_verifier = MerkleVerifier {
            root: **self.proof.commitment,
            tree_height: column_log_size,
            column_indices_by_log_deg_bound,
        };

        let mut decommitment_positions_dict: Felt252Dict<Nullable<Span<usize>>> =
            Default::default();
        decommitment_positions_dict
            .insert(column_log_size.into(), NullableTrait::new(decommitment_positions));

        merkle_verifier
            .verify(
                ref decommitment_positions_dict,
                decommitted_values.span(),
                (*self.proof.decommitment).clone(),
            );

        let folded_queries = queries.fold(FOLD_STEP);
        let folded_evals = sparse_evaluation.fold_line(*self.folding_alpha, *self.domain);

        (folded_queries, folded_evals)
    }
}

/// Returns a column's Merkle tree decommitment positions and re-builds the evaluations needed by
/// the verifier for folding and decommitment.
///
/// # Panics
///
/// Panics if the number of queries doesn't match the number of query evals.
fn compute_decommitment_positions_and_rebuild_evals(
    mut queries: Queries,
    mut query_evals: Span<QM31>,
    ref witness_evals_iter: SpanIter<QM31>,
    fold_step: u32,
) -> (Span<usize>, SparseEvaluation) {
    let fold_factor = pow2(fold_step);

    let mut decommitment_positions = array![];
    let mut subset_evals = array![];
    let mut subset_domain_start_indices = array![];

    let mut query_positions = queries.positions;
    let mut folded_query_positions = queries.fold(fold_step).positions;

    for folded_query_position in folded_query_positions {
        let subset_start = *folded_query_position * fold_factor;
        let subset_end = subset_start + fold_factor;
        let mut subset_decommitment_positions = (subset_start..subset_end).into_iter();
        let mut subset_eval = array![];

        // Extract the subset eval and decommitment positions.
        for decommitment_position in subset_decommitment_positions {
            decommitment_positions.append(decommitment_position);

            // If the decommitment position is a query position: take the value from `query_evals`,
            // else: take the value from `witness_evals`.
            subset_eval
                .append(
                    *match query_positions.next_if_eq(@decommitment_position) {
                        Some(_) => query_evals.pop_front().unwrap(),
                        None => witness_evals_iter.next().unwrap(),
                    },
                );
        }

        subset_evals.append(subset_eval);

        subset_domain_start_indices
            .append(bit_reverse_index(subset_start, queries.log_domain_size));
    }

    // Sanity check all the values have been consumed.
    assert!(query_positions.is_empty());
    assert!(query_evals.is_empty());

    let sparse_evaluation = SparseEvaluationImpl::new(
        subset_evals, subset_domain_start_indices.span(),
    );

    (decommitment_positions.span(), sparse_evaluation)
}

/// Foldable subsets of evaluations on a circle polynomial or univariate polynomial.
#[derive(Drop)]
struct SparseEvaluation {
    // TODO(andrew): Perhaps subset isn't the right word. Coset, Subgroup?
    subset_evals: Array<Array<QM31>>,
    subset_domain_initial_indexes: Span<usize>,
}

#[generate_trait]
impl SparseEvaluationImpl of SparseEvaluationTrait {
    fn new(
        subset_evals: Array<Array<QM31>>, subset_domain_initial_indexes: Span<usize>,
    ) -> SparseEvaluation {
        assert!(subset_evals.len() == subset_domain_initial_indexes.len());
        SparseEvaluation { subset_evals, subset_domain_initial_indexes }
    }

    /// Folds evaluations of a degree `d` univariate polynomial into evaluations of a degree `d/2`
    /// univariate polynomial.
    fn fold_line(
        self: @SparseEvaluation, fold_alpha: QM31, source_domain: LineDomain,
    ) -> Array<QM31> {
        let mut domain_initials = array![];

        for subset_domain_initial_index in *self.subset_domain_initial_indexes {
            domain_initials.append(source_domain.at(*subset_domain_initial_index));
        }

        let mut domain_initials_inv = BatchInvertible::batch_inverse(domain_initials);
        let mut res = array![];

        for (subset_eval, x_inv) in zip_eq(self.subset_evals.span(), domain_initials_inv.span()) {
            let values: Box<[QM31; FOLD_FACTOR]> = *subset_eval.span().try_into().unwrap();
            let [f_at_x, f_at_neg_x] = values.unbox();
            res.append(fri_fold(f_at_x, f_at_neg_x, *x_inv, fold_alpha));
        }

        res
    }

    /// Folds evaluations of a degree `d` circle polynomial into evaluations of a
    /// degree `d/2` univariate polynomial.
    fn fold_circle(
        self: @SparseEvaluation, fold_alpha: QM31, source_domain: CircleDomain,
    ) -> Array<QM31> {
        let mut domain_initial_ys = array![];

        for subset_domain_initial_index in *self.subset_domain_initial_indexes {
            domain_initial_ys.append(source_domain.at(*subset_domain_initial_index).y);
        }

        let mut domain_initial_ys_inv = BatchInvertible::batch_inverse(domain_initial_ys);
        let mut res = array![];

        for (subset_eval, y_inv) in zip_eq(self.subset_evals.span(), domain_initial_ys_inv.span()) {
            let values: Box<[QM31; CIRCLE_TO_LINE_FOLD_FACTOR]> = *subset_eval
                .span()
                .try_into()
                .unwrap();
            let [f_at_p, f_at_neg_p] = values.unbox();
            res.append(fri_fold(f_at_p, f_at_neg_p, *y_inv, fold_alpha));
        }

        res
    }
}

/// Proof of an individual FRI layer.
#[derive(Drop, Clone, Debug, Serde)]
pub struct FriLayerProof {
    /// Values that the verifier needs but cannot deduce from previous computations, in the
    /// order they are needed. This complements the values that were queried. These must be
    /// supplied directly to the verifier.
    pub fri_witness: Span<QM31>,
    pub decommitment: MerkleDecommitment<MerkleHasher>,
    pub commitment: Hash,
}

#[derive(Debug, Drop)]
pub enum FriVerificationError {
    InvalidNumFriLayers,
    FirstLayerEvaluationsInvalid,
    InnerLayerEvaluationsInvalid,
    LastLayerDegreeInvalid,
    LastLayerEvaluationsInvalid,
    LastLayerLogDegreeMustBeZero,
}

impl FriVerificationErrorDisplay of core::fmt::Display<FriVerificationError> {
    fn fmt(
        self: @FriVerificationError, ref f: core::fmt::Formatter,
    ) -> Result<(), core::fmt::Error> {
        match self {
            FriVerificationError::InvalidNumFriLayers => write!(f, "Invalid number of FRI layers"),
            FriVerificationError::FirstLayerEvaluationsInvalid => write!(
                f, "Invalid First layer evaluations",
            ),
            FriVerificationError::InnerLayerEvaluationsInvalid => write!(
                f, "Invalid inner layer evaluations",
            ),
            FriVerificationError::LastLayerDegreeInvalid => write!(f, "Invalid last layer degree"),
            FriVerificationError::LastLayerEvaluationsInvalid => write!(
                f, "Invalid last layer evaluations",
            ),
            FriVerificationError::LastLayerLogDegreeMustBeZero => write!(
                f, "Last layer log degree must be zero",
            ),
        }
    }
}
