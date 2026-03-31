use core::array::SpanIter;
use core::dict::Felt252Dict;
use core::iter::{IntoIterator, Iterator};
use crate::Hash;
use crate::channel::{Channel, ChannelTrait};
use crate::circle::{CirclePoint, CirclePointM31Impl, CosetImpl};
use crate::fields::BatchInvertible;
use crate::fields::m31::M31;
use crate::fields::qm31::{QM31, QM31Serde, QM31Trait, QM31_EXTENSION_DEGREE};
use crate::poly::circle::{CanonicCosetImpl, CircleDomain, CircleDomainImpl};
use crate::poly::line::{LineDomain, LineDomainImpl, LineEvaluationImpl, LinePoly};
use crate::poly::utils::fri_fold;
use crate::queries::{Queries, QueriesImpl};
use crate::utils::{ArrayImpl, OptionImpl, SpanExTrait, bit_reverse_index, pow2};
use crate::vcs::MerkleHasher;
use crate::vcs::verifier::{MerkleDecommitment, MerkleVerifier, MerkleVerifierTrait};

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
    pub fold_step: u32,
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
    /// `log_bound` should be the committed circle polynomial log
    /// degree bound.
    fn commit(
        ref channel: Channel, config: FriConfig, proof: FriProof, log_bound: u32,
    ) -> FriVerifier {
        // TODO(Leo): remove once support for fold_step is complete.
        assert!(config.fold_step == 1);

        let FriProof {
            first_layer: first_layer_proof, inner_layers: mut inner_layer_proofs, last_layer_poly,
        } = proof;

        channel.mix_commitment(first_layer_proof.commitment);

        let commitment_domain_log_size = log_bound + config.log_blowup_factor;
        let commitment_domain = CanonicCosetImpl::new(commitment_domain_log_size).circle_domain();

        let first_layer = FriFirstLayerVerifier {
            log_blowup_factor: config.log_blowup_factor,
            log_bound,
            commitment_domain,
            proof: first_layer_proof,
            folding_alpha: channel.draw_secure_felt(),
            // The circle-to-line fold is always equal to `config.fold_step`.
            fold_step: config.fold_step,
        };

        let mut inner_layers = array![];
        let mut layer_log_bound = log_bound - config.fold_step;
        let mut layer_domain = LineDomainImpl::new_unchecked(
            CosetImpl::half_odds(layer_log_bound + config.log_blowup_factor),
        );

        let n_inner_layers = inner_layer_proofs.len();
        for (layer_index, layer_proof) in inner_layer_proofs.into_iter().enumerate() {
            channel.mix_commitment(*layer_proof.commitment);

            // Compute the folding step for this layer.
            let fold_step = if layer_index == n_inner_layers - 1 {
                // At the last inner layer, fold by the amount needed to reach exactly the
                // last layer degree bound.
                let remaining = layer_log_bound - config.log_last_layer_degree_bound;
                assert!(
                    1 <= remaining && remaining <= config.fold_step,
                    "{}",
                    FriVerificationError::InvalidNumFriLayers,
                );
                remaining
            } else {
                config.fold_step
            };

            inner_layers
                .append(
                    FriInnerLayerVerifier {
                        log_degree_bound: layer_log_bound,
                        domain: layer_domain,
                        folding_alpha: channel.draw_secure_felt(),
                        layer_index,
                        proof: layer_proof,
                        fold_step,
                    },
                );

            layer_log_bound -= fold_step;

            layer_domain = layer_domain.repeated_double(fold_step);
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
    fn decommit(self: FriVerifier, queries: Queries, first_layer_query_evals: Span<QM31>) {
        let first_layer_sparse_evals = decommit_first_layer(
            @self, queries, first_layer_query_evals,
        );

        let inner_layer_queries = queries.fold(self.config.fold_step);

        let (last_layer_queries, last_layer_query_evals) = decommit_inner_layers(
            @self, inner_layer_queries, first_layer_sparse_evals,
        );

        decommit_last_layer(self, last_layer_queries, last_layer_query_evals)
    }

    /// Samples and returns query positions mapped by column log size.
    ///
    /// Output is of the form `(unique_log_sizes, queries_by_log_size)`.
    fn sample_query_positions(self: @FriVerifier, ref channel: Channel) -> Queries {
        let query_domain_log_size = self.first_layer.commitment_domain.log_size();
        let n_queries = *self.config.n_queries;
        let queries = QueriesImpl::generate(ref channel, query_domain_log_size, n_queries);
        queries
    }
}

/// Verifies the first layer decommitment.
///
/// Returns the queries and first layer folded column evaluations needed for
/// verifying the remaining layers.
fn decommit_first_layer(
    verifier: @FriVerifier, queries: Queries, first_layer_query_evals: Span<QM31>,
) -> SparseEvaluation {
    verifier.first_layer.verify(queries, first_layer_query_evals)
}

/// Verifies all inner layer decommitments.
///
/// Returns the queries and query evaluations needed for verifying the last FRI layer.
fn decommit_inner_layers(
    verifier: @FriVerifier, queries: Queries, mut first_layer_sparse_eval: SparseEvaluation,
) -> (Queries, Array<QM31>) {
    let mut layer_query_evals = first_layer_sparse_eval
        .fold_circle(
            *verifier.first_layer.folding_alpha,
            *verifier.first_layer.commitment_domain,
            *verifier.config.fold_step,
        );

    let mut layer_queries = queries;
    for layer in verifier.inner_layers.span() {
        let (folded_queries, mut folded_query_evals) = layer
            .verify_and_fold(queries: layer_queries, evals_at_queries: layer_query_evals.span());

        layer_queries = folded_queries;
        layer_query_evals = folded_query_evals;
    }

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
    /// The degree bound of the circle polynomial committed in the first layer.
    log_bound: u32,
    /// The commitment domain of the circle polynomial committed in the first layer.
    commitment_domain: CircleDomain,
    folding_alpha: QM31,
    proof: FriLayerProof,
    fold_step: u32,
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
        self: @FriFirstLayerVerifier, queries: Queries, mut query_evals: Span<QM31>,
    ) -> SparseEvaluation {
        let log_size = self.commitment_domain.log_size();
        assert!(queries.log_domain_size == log_size);

        let mut fri_witness = (*self.proof.fri_witness).into_iter();
        // For decommitment, each QM31 col must be split into its constituent M31 coordinate cols.
        let mut decommitted_values = array![];

        let (column_decommitment_positions, sparse_evaluation) =
            compute_decommitment_positions_and_rebuild_evals(
            queries, query_evals, ref fri_witness, *self.fold_step,
        );

        for subset_eval in sparse_evaluation.subset_evals.span() {
            for eval in subset_eval.span() {
                // Split the QM31 into its M31 coordinate values.
                let [v0, v1, v2, v3] = (*eval).to_fixed_array();
                decommitted_values.append(v0);
                decommitted_values.append(v1);
                decommitted_values.append(v2);
                decommitted_values.append(v3);
            };
        }

        // Check all proof evals have been consumed.
        assert!(
            fri_witness.next().is_none(), "{}", FriVerificationError::FirstLayerEvaluationsInvalid,
        );

        let degree_bound_by_column = ArrayImpl::new_repeated(
            n: QM31_EXTENSION_DEGREE, v: *self.log_bound,
        );
        let merkle_verifier = MerkleVerifier {
            root: *self.proof.commitment,
            tree_height: log_size,
            column_log_deg_bounds: degree_bound_by_column.span(),
        };

        merkle_verifier
            .verify(
                column_decommitment_positions,
                decommitted_values.span(),
                self.proof.decommitment.clone(),
            );

        sparse_evaluation
    }
}

#[derive(Drop, Debug)]
struct FriInnerLayerVerifier {
    log_degree_bound: u32,
    domain: LineDomain,
    folding_alpha: QM31,
    layer_index: usize,
    proof: @FriLayerProof,
    fold_step: u32,
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
            queries, evals_at_queries, ref fri_witness, *self.fold_step,
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
                decommitted_values.append(v0);
                decommitted_values.append(v1);
                decommitted_values.append(v2);
                decommitted_values.append(v3);
            };
        }

        let column_log_size = self.domain.log_size();
        let degree_bound_by_column = ArrayImpl::new_repeated(
            n: QM31_EXTENSION_DEGREE, v: *self.log_degree_bound,
        );
        let merkle_verifier = MerkleVerifier {
            root: **self.proof.commitment,
            tree_height: column_log_size,
            column_log_deg_bounds: degree_bound_by_column.span(),
        };

        merkle_verifier
            .verify(
                decommitment_positions,
                decommitted_values.span(),
                (*self.proof.decommitment).clone(),
            );

        let folded_queries = queries.fold(*self.fold_step);
        let folded_evals = sparse_evaluation
            .fold_line(*self.folding_alpha, *self.domain, *self.fold_step);

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

    /// Folds evaluations of a degree `d` univariate polynomial into evaluations of a degree
    /// `d / 2^fold_step` univariate polynomial.
    fn fold_line(
        self: @SparseEvaluation, fold_alpha: QM31, source_domain: LineDomain, fold_step: u32,
    ) -> Array<QM31> {
        let n_twiddles_per_query = pow2(fold_step) - 1;

        // Collect all x-coordinates across all queries (these are the inverse twiddles).
        let mut all_x_coords = array![];
        for subset_domain_initial_index in *self.subset_domain_initial_indexes {
            let mut query_x_coords = array![];
            let fold_domain_initial = source_domain.coset.index_at(*subset_domain_initial_index);
            let fold_domain = LineDomainImpl::new_unchecked(
                CosetImpl::new(fold_domain_initial, fold_step),
            );
            let mut j: usize = 0;
            while j < fold_domain.size() {
                query_x_coords.append(fold_domain.at(bit_reverse_index(j, fold_step)));
                j += 2;
            }
            query_x_coords.append_span(fold_x_coords(query_x_coords.span(), fold_step));
            all_x_coords.append_span(query_x_coords.span());
        }

        // Compute the twiddles by batch-inverting all x-coordinates at once. The twiddles are laid
        // out in flat form, as [query][fold][twiddle_in_fold].
        let all_twiddles = BatchInvertible::batch_inverse(all_x_coords);
        let mut all_twiddles = all_twiddles.span();

        // Fold each query's coset using its slice of precomputed twiddles.
        let mut folded_eval = array![];
        for subset_eval in self.subset_evals.span() {
            let (query_twiddles, rest) = all_twiddles.split_at(n_twiddles_per_query);
            all_twiddles = rest;
            folded_eval.append(fold_coset(subset_eval.span(), query_twiddles, fold_alpha));
        }

        folded_eval
    }

    /// Folds evaluations of a degree `d` circle polynomial into evaluations of a
    /// degree `d / 2^fold_step` univariate polynomial.
    fn fold_circle(
        self: @SparseEvaluation, fold_alpha: QM31, source_domain: CircleDomain, fold_step: u32,
    ) -> Array<QM31> {
        let n_twiddles_per_query = pow2(fold_step) - 1;

        // Collect all twiddle coordinates across all queries: first fold uses y-coordinates
        // (circle-to-line), remaining folds use x-coordinates.
        let mut all_coords = array![];
        for subset_domain_initial_index in *self.subset_domain_initial_indexes {
            let fold_domain_initial = source_domain.index_at(*subset_domain_initial_index);
            let circle_fold_domain = CircleDomainImpl::new(
                CosetImpl::new(fold_domain_initial, fold_step - 1),
            );
            // Collect the even-index points of the folding circle domain.
            let mut circle_fold_points = array![];
            let mut i: usize = 0;
            // The first fold's inverse twiddles are the y-coordinates of `circle_fold_points`.
            while i < circle_fold_domain.size() {
                let p = circle_fold_domain.at(bit_reverse_index(i, fold_step));
                circle_fold_points.append(p);
                all_coords.append(p.y);
                i += 2;
            }
            // The remaining inverse twiddles are the x-coordinates of even-index points of
            // `circle_fold_points`, and their doublings.
            if fold_step > 1 {
                let mut x_coords = array![];
                let mut circle_fold_points = circle_fold_points.span();
                while let Some(boxed_pair) = circle_fold_points.multi_pop_front::<2>() {
                    let [p, _]: [CirclePoint; 2] = boxed_pair.unbox();
                    x_coords.append(p.x);
                }
                x_coords.append_span(fold_x_coords(x_coords.span(), fold_step));
                all_coords.append_span(x_coords.span());
            }
        }

        // Batch invert all twiddles at once.
        let all_twiddles = BatchInvertible::batch_inverse(all_coords);
        let mut all_twiddles = all_twiddles.span();

        // Fold each query's coset using its slice of precomputed twiddles.
        let mut res = array![];
        for subset_eval in self.subset_evals.span() {
            let (query_twiddles, rest) = all_twiddles.split_at(n_twiddles_per_query);
            all_twiddles = rest;
            res.append(fold_coset(subset_eval.span(), query_twiddles, fold_alpha));
        }

        res
    }
}

/// Computes the x-coordinates for fold levels `1..fold_step` by repeatedly applying `double_x`
/// to every other element of the previous level.
///
/// `coset_x_coords` contains the x-coordinates for fold level 0.
/// Returns the flat x-coordinates (not yet inverted) for levels `1..fold_step`, in the layout
/// expected by [`fold_coset`].
pub fn fold_x_coords(coset_x_coords: Span<M31>, fold_step: u32) -> Span<M31> {
    let mut prev_layer = coset_x_coords;
    let mut res = array![];
    for _ in 1..fold_step {
        let mut k: usize = 0;
        while k < prev_layer.len() {
            res.append(CirclePointM31Impl::double_x(*prev_layer[k]));
            k += 2;
        }
        let next_len = prev_layer.len() / 2;
        // Slice the last `next_len` elements of the current fold.
        prev_layer = res.span().slice(res.len() - next_len, next_len);
    }
    res.span()
}

/// Folds `2^n` evaluations into a single evaluation using precomputed twiddles.
///
/// # Arguments
///
/// * `eval` contains evaluations on either a circle domain (for the circle-to-line case) or
/// a coset (for the line-to-line case).
/// * `twiddles` is a flat span of twiddle factors laid out as:
///     `[fold_0_tw_0, ..., fold_0_tw_{2^(n-1)-1}, fold_1_tw_0, ..., fold_{n-1}_tw_0]`
/// where `fold_i_tw_j` denotes the j-th twiddle in the i-th fold. The i-th fold has `2^(n-1-i)`
/// twiddles.
/// * `alpha`: the random folding factor.
///
/// The function assumes that `eval` is of length `2^n`, and `twiddles` is of length `2^n - 1`.
pub fn fold_coset(eval: Span<QM31>, mut twiddles: Span<M31>, alpha: QM31) -> QM31 {
    let mut current_eval = eval;
    let mut folding_alpha = alpha;
    #[cairofmt::skip]
    while current_eval.len() > 1 {
        let mut next_eval = array![];
        while let Some(boxed_pair) = current_eval.multi_pop_front::<2>() {
            let [v0, v1]: [QM31; 2] = boxed_pair.unbox();
            let itwid = *twiddles.pop_front().unwrap();
            next_eval.append(fri_fold(v0, v1, itwid, folding_alpha));
        }
        folding_alpha = folding_alpha * folding_alpha;
        current_eval = next_eval.span();
    };
    assert!(twiddles.is_empty());
    *current_eval[0]
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
