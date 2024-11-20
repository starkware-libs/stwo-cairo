use core::array::SpanIter;
use core::dict::Felt252Dict;
use core::iter::{IntoIterator, Iterator};
use core::num::traits::CheckedSub;
use crate::channel::{Channel, ChannelTrait};
use crate::circle::CosetImpl;
use crate::fields::m31::M31;
use crate::fields::qm31::{QM31, QM31Trait, QM31Zero, QM31_EXTENSION_DEGREE};
use crate::fields::{BatchInvertible, Invertible};
use crate::poly::circle::{
    CanonicCosetImpl, CircleDomain, CircleDomainImpl, SparseCircleEvaluation,
    SparseCircleEvaluationImpl,
};
use crate::poly::line::{LineDomain, LineDomainImpl};
use crate::poly::line::{
    LineEvaluation, LineEvaluationImpl, SparseLineEvaluation, SparseLineEvaluationImpl,
};
use crate::poly::line::{LinePoly, LinePolyImpl};
use crate::poly::utils::{ibutterfly};
use crate::queries::SparseSubCircleDomain;
use crate::queries::{Queries, QueriesImpl};
use crate::utils::{ArrayImpl, OptionImpl, SpanExTrait, bit_reverse_index, pow2};
use crate::vcs::hasher::PoseidonMerkleHasher;
use crate::vcs::verifier::{MerkleDecommitment, MerkleVerifier, MerkleVerifierTrait};
use crate::{ColumnArray};

/// Fold step size for circle polynomials.
pub const CIRCLE_TO_LINE_FOLD_STEP: u32 = 1;

/// Equals `2^CIRCLE_TO_LINE_FOLD_STEP`.
pub const CIRCLE_TO_LINE_FOLD_FACTOR: usize = 2;

/// Fold step size for univariate polynomials.
pub const FOLD_STEP: u32 = 1;

/// Equals `2^FOLD_STEP`.
pub const FOLD_FACTOR: usize = 2;

#[derive(Clone, Copy, Drop, Debug)]
pub struct FriConfig {
    pub log_blowup_factor: u32,
    pub log_last_layer_degree_bound: u32,
    pub n_queries: usize,
}

#[derive(Drop, Debug)]
pub struct FriVerifier {
    config: FriConfig,
    first_layer: FriFirstLayerVerifier,
    inner_layers: Array<FriInnerLayerVerifier>,
    last_layer_domain: LineDomain,
    last_layer_poly: LinePoly,
    queries: Option<Queries>,
}

#[generate_trait]
pub impl FriVerifierImpl of FriVerifierTrait {
    /// Verifies the commitment stage of FRI.
    ///
    /// `column_log_bounds` should be the committed circle polynomial log
    /// degree bounds in descending order.
    fn commit(
        ref channel: Channel, config: FriConfig, proof: FriProof, column_log_bounds: Span<u32>,
    ) -> Result<FriVerifier, FriVerificationError> {
        let FriProof { first_layer: first_layer_proof,
        inner_layers: mut inner_layer_proofs,
        last_layer_poly } =
            proof;

        channel.mix_digest(first_layer_proof.commitment);

        let mut column_commitment_domains = array![];

        for column_log_bound in column_log_bounds {
            let commitment_domain_log_size = *column_log_bound + config.log_blowup_factor;
            let commitment_domain = CanonicCosetImpl::new(commitment_domain_log_size)
                .circle_domain();
            column_commitment_domains.append(commitment_domain);
        };

        let first_layer = FriFirstLayerVerifier {
            column_log_bounds,
            column_commitment_domains: column_commitment_domains.span(),
            proof: first_layer_proof,
            folding_alpha: channel.draw_felt(),
        };

        // Bounds are sorted in descending.
        // TODO(andrew): There is no check that it's sorted. Add check.
        let max_column_log_bound = *column_log_bounds.first().unwrap();

        let mut layer_index = 0;
        let mut inner_layers = array![];
        let mut layer_log_bound = max_column_log_bound - CIRCLE_TO_LINE_FOLD_STEP;
        let mut layer_domain = LineDomainImpl::new_unchecked(
            CosetImpl::half_odds(layer_log_bound + config.log_blowup_factor),
        );

        loop {
            let proof = match inner_layer_proofs.pop_front() {
                Option::Some(proof) => proof,
                Option::None => { break Result::Ok(()); },
            };

            channel.mix_digest(*proof.commitment);

            inner_layers
                .append(
                    FriInnerLayerVerifier {
                        log_degree_bound: layer_log_bound,
                        domain: layer_domain,
                        folding_alpha: channel.draw_felt(),
                        layer_index,
                        proof,
                    },
                );

            layer_log_bound = match layer_log_bound.checked_sub(FOLD_STEP) {
                Option::Some(layer_log_bound) => layer_log_bound,
                Option::None => { break Result::Err(FriVerificationError::InvalidNumFriLayers); },
            };

            layer_index += 1;
            layer_domain = layer_domain.double();
        }?;

        if layer_log_bound != config.log_last_layer_degree_bound {
            return Result::Err(FriVerificationError::InvalidNumFriLayers);
        }

        if last_layer_poly.len() != pow2(config.log_last_layer_degree_bound) {
            return Result::Err(FriVerificationError::LastLayerDegreeInvalid);
        }

        channel.mix_felts(last_layer_poly.coeffs.span());

        Result::Ok(
            FriVerifier {
                config,
                first_layer,
                inner_layers,
                last_layer_domain: layer_domain,
                last_layer_poly,
                queries: Option::None,
            },
        )
    }

    /// Verifies the decommitment stage of FRI.
    ///
    /// The query evals need to be provided in the same order as their commitment.
    fn decommit(
        self: FriVerifier, first_layer_query_evals: ColumnArray<Span<QM31>>,
    ) -> Result<(), FriVerificationError> {
        let queries = self.queries.expect('queries not sampled');

        let (inner_layer_queries, first_layer_folded_evals) = decommit_first_layer(
            @self, queries, first_layer_query_evals,
        )?;

        let (last_layer_queries, last_layer_query_evals) = decommit_inner_layers(
            @self, inner_layer_queries, first_layer_folded_evals,
        )?;

        decommit_last_layer(self, last_layer_queries, last_layer_query_evals)
    }

    /// Samples and returns query positions mapped by column log size.
    ///
    /// Output is of the form `(unique_log_sizes, queries_by_log_size)`.
    fn sample_query_positions(
        ref self: FriVerifier, ref channel: Channel,
    ) -> (Span<u32>, Felt252Dict<Nullable<Span<usize>>>) {
        // The sizes of input circle polynomial commitment domains.
        let mut column_log_sizes = array![];

        for column_commitment_domain in self.first_layer.column_commitment_domains {
            column_log_sizes.append(column_commitment_domain.log_size());
        };

        let unique_column_log_sizes = column_log_sizes.dedup().span();

        // Column are sorted in descending order by size.
        let max_column_log_size = *unique_column_log_sizes.first().unwrap();
        let n_queries = self.config.n_queries;
        let queries = QueriesImpl::generate(ref channel, max_column_log_size, n_queries);
        self.queries = Option::Some(queries);

        (unique_column_log_sizes, get_query_positions_by_log_size(queries, unique_column_log_sizes))
    }
}

/// Verifies the first layer decommitment.
///
/// Returns the queries and first layer folded column evaluations needed for
/// verifying the remaining layers.
fn decommit_first_layer(
    verifier: @FriVerifier, queries: Queries, first_layer_query_evals: ColumnArray<Span<QM31>>,
) -> Result<(Queries, ColumnArray<Span<QM31>>), FriVerificationError> {
    verifier.first_layer.verify_and_fold(queries, first_layer_query_evals)
}

/// Verifies all inner layer decommitments.
///
/// Returns the queries and query evaluations needed for verifying the last FRI layer.
fn decommit_inner_layers(
    verifier: @FriVerifier, queries: Queries, mut first_layer_folded_evals: ColumnArray<Span<QM31>>,
) -> Result<(Queries, Array<QM31>), FriVerificationError> {
    let first_layer_fold_alpha = *verifier.first_layer.folding_alpha;
    let first_layer_fold_alpha_pow_fold_factor = first_layer_fold_alpha * first_layer_fold_alpha;

    let mut inner_layers = verifier.inner_layers.span();
    let mut layer_queries = queries.fold(CIRCLE_TO_LINE_FOLD_STEP);
    let mut layer_query_evals = ArrayImpl::new_repeated(layer_queries.len(), QM31Zero::zero());
    let mut first_layer_folded_evals = first_layer_folded_evals.span();
    let mut first_layer_column_bounds = *verifier.first_layer.column_log_bounds;

    loop {
        let layer = match inner_layers.pop_front() {
            Option::Some(layer) => layer,
            Option::None => { break Result::Ok(()); },
        };

        let circle_poly_degree_bound = *layer.log_degree_bound + CIRCLE_TO_LINE_FOLD_STEP;

        // Check for evals committed in the first layer that need to be folded into this layer.
        while let Option::Some(_) = first_layer_column_bounds
            .next_if_eq(@circle_poly_degree_bound) {
            let mut folded_column_evals = *first_layer_folded_evals.pop_front().unwrap();
            let mut updated_layer_query_evals = array![];

            while let (Option::Some(curr_layer_eval), Option::Some(folded_column_eval)) =
                (layer_query_evals.pop_front(), folded_column_evals.pop_front()) {
                // TODO(andrew): As Ilya pointed out using the first layer's folding
                // alpha here might not be sound. Investigate.
                updated_layer_query_evals
                    .append(
                        curr_layer_eval * first_layer_fold_alpha_pow_fold_factor
                            + *folded_column_eval,
                    );
            };

            layer_query_evals = updated_layer_query_evals;
        };

        match layer.verify_and_fold(layer_queries, layer_query_evals.span()) {
            Result::Ok((
                next_layer_queries, next_layer_query_evals,
            )) => {
                layer_queries = next_layer_queries;
                layer_query_evals = next_layer_query_evals;
            },
            Result::Err(error) => { break Result::Err(error); },
        };
    }?;

    // Check all values have been consumed.
    assert!(first_layer_column_bounds.is_empty());
    assert!(first_layer_folded_evals.is_empty());

    Result::Ok((layer_queries, layer_query_evals))
}

/// Verifies the last layer.decommit_last_layer(self,
fn decommit_last_layer(
    verifier: FriVerifier, mut queries: Queries, mut query_evals: Array<QM31>,
) -> Result<(), FriVerificationError> {
    let FriVerifier { last_layer_domain, last_layer_poly, .. } = verifier;

    // TODO(andrew): Note depending on the proof parameters, doing FFT on the last layer poly vs
    // pointwize evaluation is less efficient.
    let last_layer_evals = last_layer_poly.evaluate(last_layer_domain).values;
    let domain_log_size = last_layer_domain.log_size();

    loop {
        let (query, query_eval) = match (queries.positions.pop_front(), query_evals.pop_front()) {
            (Option::Some(query), Option::Some(query_eval)) => (query, query_eval),
            _ => { break Result::Ok(()); },
        };

        // TODO(andrew): Makes more sense for the proof to provide coeffs in natural order and
        // the FFT return evals in bit-reversed order to prevent this unnessesary bit-reverse.
        let last_layer_eval_i = bit_reverse_index(*query, domain_log_size);

        if query_eval != *last_layer_evals[last_layer_eval_i] {
            break Result::Err(FriVerificationError::LastLayerEvaluationsInvalid);
        }
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
    };

    query_positions_by_log_size
}

/// A FRI proof.
#[derive(Drop, Serde)]
pub struct FriProof {
    pub first_layer: FriLayerProof,
    pub inner_layers: Span<FriLayerProof>,
    pub last_layer_poly: LinePoly,
}

#[derive(Drop, Debug)]
struct FriFirstLayerVerifier {
    /// The list of degree bounds of all circle polynomials commited in the first layer.
    column_log_bounds: Span<u32>,
    /// The commitment domain all the circle polynomials in the first layer.
    column_commitment_domains: Span<CircleDomain>,
    folding_alpha: QM31,
    proof: FriLayerProof,
}

#[generate_trait]
impl FriFirstLayerVerifierImpl of FriFirstLayerVerifierTrait {
    /// Verifies the layer's merkle decommitment and returns the the folded queries and query evals.
    ///
    /// # Errors
    ///
    /// An `Err` will be returned if:
    /// * The proof doesn't store enough evaluations.
    /// * The merkle decommitment is invalid.
    ///
    /// # Panics
    ///
    /// Panics if:
    /// * The queries are sampled on the wrong domain.
    /// * There are an invalid number of provided column evals.
    fn verify_and_fold(
        self: @FriFirstLayerVerifier,
        mut queries: Queries,
        mut query_evals_by_column: ColumnArray<Span<QM31>>,
    ) -> Result<(Queries, ColumnArray<Span<QM31>>), FriVerificationError> {
        // Columns are provided in descending order by size.
        let max_column_log_size = (*self.column_commitment_domains).first().unwrap().log_size();
        assert!(queries.log_domain_size == max_column_log_size);

        let mut column_commitment_domains = *self.column_commitment_domains;
        let mut fri_witness = (*self.proof.fri_witness).into_iter();
        let mut decommitment_positions_by_log_size: Felt252Dict = Default::default();
        // For decommitment, each QM31 col must be split into its constituent M31 coordinate cols.
        let mut decommitment_coordinate_columns = array![];
        let mut decommitment_coordinate_column_log_sizes = array![];
        let mut folded_evals_by_column = array![];

        loop {
            let (column_domain, column_query_evals) =
                match (column_commitment_domains.pop_front(), query_evals_by_column.pop_front()) {
                (Option::Some(domain), Option::Some(evals)) => (domain, evals),
                (Option::None, Option::None) => { break Result::Ok(()); },
                _ => { panic!() },
            };

            let column_domain_log_size = column_domain.log_size();
            let n_folds = queries.log_domain_size - column_domain_log_size;

            if n_folds != 0 {
                queries = queries.fold(n_folds);
            }

            let (column_decommitment_positions, sparse_evaluation) =
                match compute_decommitment_positions_and_rebuild_evals(
                    queries, column_query_evals, ref fri_witness, CIRCLE_TO_LINE_FOLD_STEP,
                ) {
                Result::Ok(res) => res,
                Result::Err(_) => {
                    break Result::Err(FriVerificationError::FirstLayerEvaluationsInvalid);
                },
            };

            // Columns of the same size have the same decommitment positions.
            // TODO(andrew): Do without nullable.
            decommitment_positions_by_log_size
                .insert(
                    column_domain_log_size.into(),
                    NullableTrait::new(column_decommitment_positions),
                );

            let mut coord_col_0 = array![];
            let mut coord_col_1 = array![];
            let mut coord_col_2 = array![];
            let mut coord_col_3 = array![];

            for subset_eval in sparse_evaluation.subset_evals.span() {
                for eval in subset_eval.span() {
                    // Split the QM31 into its M31 coordinate values.
                    let [v0, v1, v2, v3] = (*eval).to_array();
                    coord_col_0.append(v0);
                    coord_col_1.append(v1);
                    coord_col_2.append(v2);
                    coord_col_3.append(v3);
                };
            };

            decommitment_coordinate_columns.append(coord_col_0);
            decommitment_coordinate_columns.append(coord_col_1);
            decommitment_coordinate_columns.append(coord_col_2);
            decommitment_coordinate_columns.append(coord_col_3);

            decommitment_coordinate_column_log_sizes.append(column_domain_log_size);
            decommitment_coordinate_column_log_sizes.append(column_domain_log_size);
            decommitment_coordinate_column_log_sizes.append(column_domain_log_size);
            decommitment_coordinate_column_log_sizes.append(column_domain_log_size);
        }?;

        // Check all proof evals have been consumed.
        if fri_witness.next().is_some() {
            return Result::Err(FriVerificationError::FirstLayerEvaluationsInvalid);
        }

        let merkle_verifier = MerkleVerifier {
            root: *self.proof.commitment,
            column_log_sizes: decommitment_coordinate_column_log_sizes,
        };

        if let Result::Err(_) = merkle_verifier
            .verify(
                decommitment_positions_by_log_size,
                decommitment_coordinate_columns.span(),
                (*self.proof.decommitment).clone(),
            ) {
            return Result::Err(FriVerificationError::FirstLayerCommitmentInvalid);
        }

        let folded_queries = queries.fold(CIRCLE_TO_LINE_FOLD_STEP);

        Result::Ok((folded_queries, folded_evals_by_column))
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
    /// Verifies the layer's merkle decommitment and returns the the folded queries and query evals.
    fn verify_and_fold(
        self: @FriInnerLayerVerifier, queries: Queries, evals_at_queries: Span<QM31>,
    ) -> Result<(Queries, Array<QM31>), FriVerificationError> {
        assert_eq!(queries.log_domain_size, self.domain.log_size());

        let mut fri_witness = (**self.proof.fri_witness).into_iter();

        let (decommitment_positions, sparse_evaluation) =
            match compute_decommitment_positions_and_rebuild_evals(
                queries, evals_at_queries, ref fri_witness, FOLD_STEP,
            ) {
            Result::Ok(res) => res,
            Result::Err(_) => {
                return Result::Err(FriVerificationError::InnerLayerEvaluationsInvalid);
            },
        };

        // Check all proof evals have been consumed.
        if fri_witness.next().is_some() {
            return Result::Err(FriVerificationError::InnerLayerEvaluationsInvalid);
        }

        // TODO(andrew): Consider seperating into seperate function.
        let mut coord_col_0 = array![];
        let mut coord_col_1 = array![];
        let mut coord_col_2 = array![];
        let mut coord_col_3 = array![];

        for subset_eval in sparse_evaluation.subset_evals.span() {
            for eval in subset_eval.span() {
                // Split the QM31 into its M31 coordinate values.
                let [v0, v1, v2, v3] = (*eval).to_array();
                coord_col_0.append(v0);
                coord_col_1.append(v1);
                coord_col_2.append(v2);
                coord_col_3.append(v3);
            };
        };

        // For decommitment, a QM31 col must be split into its constituent M31 coordinate cols.
        let decommitment_coordinate_columns = array![
            coord_col_0, coord_col_1, coord_col_2, coord_col_3,
        ];

        let column_log_size = self.domain.log_size();
        let merkle_verifier = MerkleVerifier {
            root: **self.proof.commitment,
            column_log_sizes: ArrayImpl::new_repeated(QM31_EXTENSION_DEGREE, column_log_size),
        };

        let mut decommitment_positions_dict: Felt252Dict<Nullable<Span<usize>>> =
            Default::default();
        decommitment_positions_dict
            .insert(self.domain.log_size().into(), NullableTrait::new(decommitment_positions));

        if let Result::Err(_) = merkle_verifier
            .verify(
                decommitment_positions_dict,
                decommitment_coordinate_columns.span(),
                (*self.proof.decommitment).clone(),
            ) {
            return Result::Err(FriVerificationError::InnerLayerCommitmentInvalid);
        }

        let folded_queries = queries.fold(FOLD_STEP);
        let folded_evals = sparse_evaluation.fold_line(*self.folding_alpha, *self.domain);

        Result::Ok((folded_queries, folded_evals))
    }
}

/// Returns a column's merkle tree decommitment positions and re-builds the evaluations needed by
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
) -> Result<(Span<usize>, SparseEvaluation), InsufficientWitnessError> {
    let fold_factor = pow2(fold_step);
    let mut query_evals_iter = query_evals.into_iter();

    let mut decommitment_positions = array![];
    let mut subset_evals = array![];
    let mut subset_domain_index_initials = array![];

    let mut query_positions = queries.positions;
    let mut folded_query_positions = queries.fold(fold_step).positions;

    loop {
        let folded_query_position = match folded_query_positions.pop_front() {
            Option::Some(position) => *position,
            Option::None => { break Result::Ok(()); },
        };

        let subset_start = folded_query_position * fold_factor;
        let subset_end = subset_start + fold_factor;
        let mut subset_decommitment_positions = (subset_start..subset_end).into_iter();
        let mut subset_eval = array![];

        // Extract the subset eval and decommitment positions.
        // TODO(andrew): Handle the error.
        let loop_res = loop {
            let decommitment_position = match subset_decommitment_positions.next() {
                Option::Some(position) => position,
                Option::None => { break Result::Ok(()); },
            };

            decommitment_positions.append(decommitment_position);

            // If the decommitment position is a query position: take the value from `query_evals`,
            // else: take the value from `witness_evals`.
            subset_eval
                .append(
                    *match query_positions.next_if_eq(@decommitment_position) {
                        Option::Some(_) => query_evals_iter.next().unwrap(),
                        Option::None => match witness_evals_iter.next() {
                            Option::Some(witness_eval) => witness_eval,
                            Option::None => { break Result::Err(InsufficientWitnessError {}); },
                        },
                    },
                );
        };

        // TODO(andrew): Is there an easier way to break if an inner loop returns an error i.e. like
        // `loop { .. }?;` but for breaking a loop?.
        if let Result::Err(error) = loop_res {
            break Result::Err(error);
        }

        subset_evals.append(subset_eval);
        subset_domain_index_initials
            .append(bit_reverse_index(subset_start, queries.log_domain_size));
    };

    // Sanity check all the values have been consumed.
    assert!(query_positions.is_empty());
    assert!(query_evals_iter.next().is_none());

    let sparse_evaluation = SparseEvaluationImpl::new(
        subset_evals, subset_domain_index_initials.span(),
    );

    Result::Ok((decommitment_positions.span(), sparse_evaluation))
}

#[derive(Drop)]
struct InsufficientWitnessError {}

/// Foldable subsets of evaluations on a circle polynomial or univariate polynomial.
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
        assert_eq!(subset_evals.len(), subset_domain_initial_indexes.len());
        SparseEvaluation { subset_evals, subset_domain_initial_indexes }
    }

    /// Folds evaluations of a degree `d` polynomial into evaluations of a degree `d/2` polynomial.
    fn fold_line(
        self: SparseEvaluation, fold_alpha: QM31, source_domain: LineDomain,
    ) -> Array<QM31> {
        let mut domain_initials = array![];

        for subset_domain_initial_index in self.subset_domain_initial_indexes {
            domain_initials.append(source_domain.at(*subset_domain_initial_index));
        };

        let mut domain_initials_inv = BatchInvertible::batch_inverse(domain_initials);
        let mut res = array![];

        for subset_eval in self.subset_evals {
            let x_inv = domain_initials_inv.pop_front().unwrap();
            let values: Box<[QM31; FOLD_FACTOR]> = *subset_eval.span().try_into().unwrap();
            let [f_at_x, f_at_neg_x] = values.unbox();
            let (f0, f1) = ibutterfly(f_at_x, f_at_neg_x, x_inv);
            res.append(f0 + fold_alpha * f1);
        };

        res
    }

    /// Folds evaluations of a degree `d` circle polynomial into evaluations of a
    /// degree `d/2` univariate polynomial.
    fn fold_circle(
        self: SparseEvaluation, fold_alpha: QM31, source_domain: CircleDomain,
    ) -> Array<QM31> {
        let mut domain_initial_ys = array![];

        for subset_domain_initial_index in self.subset_domain_initial_indexes {
            domain_initial_ys.append(source_domain.at(*subset_domain_initial_index).y);
        };

        let mut domain_initial_ys_inv = BatchInvertible::batch_inverse(domain_initial_ys);
        let mut res = array![];

        for subset_eval in self.subcircle_evals {
            let y_inv = domain_initial_ys_inv.pop_front().unwrap();
            let values: Box<[QM31; CIRCLE_TO_LINE_FOLD_FACTOR]> = *subset_eval
                .span()
                .try_into()
                .unwrap();
            let [f_at_p, f_at_neg_p] = values.unbox();
            let (f0, f1) = ibutterfly(f_at_p, f_at_neg_p, y_inv);
            res.append(f0 + fold_alpha * f1);
        };

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
    pub decommitment: MerkleDecommitment<PoseidonMerkleHasher>,
    pub commitment: felt252,
}

#[derive(Debug, Drop)]
pub enum FriVerificationError {
    InvalidNumFriLayers,
    FirstLayerEvaluationsInvalid,
    FirstLayerCommitmentInvalid,
    InnerLayerEvaluationsInvalid,
    InnerLayerCommitmentInvalid,
    LastLayerDegreeInvalid,
    LastLayerEvaluationsInvalid,
}
// #[cfg(test)]
// mod test {
//     use crate::channel::ChannelTrait;
//     use crate::circle::{CirclePointIndexImpl, Coset, CosetImpl};
//     use crate::fields::qm31::qm31;
//     use crate::poly::circle::{
//         CircleDomain, CircleEvaluationImpl, SparseCircleEvaluation, SparseCircleEvaluationImpl,
//     };
//     use crate::poly::line::LineDomainImpl;
//     use crate::poly::line::LinePoly;
//     use crate::poly::line::{LineEvaluation, SparseLineEvaluation, SparseLineEvaluationImpl};
//     use crate::queries::{Queries, QueriesImpl};
//     use crate::vcs::verifier::MerkleDecommitment;
//     use super::{FriConfig, FriLayerProof, FriProof, FriVerificationError, FriVerifierImpl};

//     type ProofValues = (FriConfig, FriProof, Array<u32>, Queries, Array<SparseCircleEvaluation>);

//     #[test]
//     fn valid_proof_with_constant_last_layer_passes_verification() {
//         let (config, proof, bounds, queries, decommitted_values) =
//         proof_with_constant_last_layer();

//         let mut channel = ChannelTrait::new(0x00);
//         let verifier = FriVerifierImpl::commit(ref channel, config, proof, bounds).unwrap();

//         verifier.decommit_on_queries(@queries, decommitted_values).unwrap();
//     }

//     #[test]
//     fn valid_proof_passes_verification() {
//         let (config, proof, bounds, queries, decommitted_values) =
//         proof_with_linear_last_layer();

//         let mut channel = ChannelTrait::new(0x00);
//         let verifier = FriVerifierImpl::commit(ref channel, config, proof, bounds).unwrap();

//         verifier.decommit_on_queries(@queries, decommitted_values).unwrap();
//     }

//     #[test]
//     fn valid_mixed_degree_proof_passes_verification() {
//         let (config, proof, bounds, queries, decommitted_values) = proof_with_mixed_degree_1();

//         let mut channel = ChannelTrait::new(0x00);
//         let verifier = FriVerifierImpl::commit(ref channel, config, proof, bounds).unwrap();

//         verifier.decommit_on_queries(@queries, decommitted_values).unwrap();
//     }

//     #[test]
//     fn valid_mixed_degree_end_to_end_proof_passes_verification() {
//         let (config, proof, bounds, decommitted_values) = proof_with_mixed_degree_2();
//         let mut channel = ChannelTrait::new(0x00);
//         let mut verifier = FriVerifierImpl::commit(ref channel, config, proof, bounds).unwrap();

//         let mut channel = ChannelTrait::new(0x00);
//         let (_, _, _) = verifier.sample_query_positions(ref channel);

//         verifier.decommit(decommitted_values).unwrap();
//     }

//     #[test]
//     fn proof_with_removed_layer_fails_verification() {
//         let (config, proof, bounds, _queries, _decommitted_values) = proof_with_mixed_degree_1();
//         let mut invalid_config = config;
//         invalid_config.log_last_layer_degree_bound -= 1;
//         let mut channel = ChannelTrait::new(0x00);

//         let result = FriVerifierImpl::commit(ref channel, invalid_config, proof, bounds);

//         match result {
//             Result::Ok(_) => { panic!("Verifier should return InvalidNumFriLayers"); },
//             Result::Err(error) => { assert!(error == FriVerificationError::InvalidNumFriLayers);
//             },
//         }
//     }

//     #[test]
//     fn proof_with_added_layer_fails_verification() {
//         let (config, proof, bounds, _queries, _decommitted_values) = proof_with_mixed_degree_1();

//         let mut invalid_config = config;
//         invalid_config.log_last_layer_degree_bound += 1;

//         let mut channel = ChannelTrait::new(0x00);
//         let result = FriVerifierImpl::commit(ref channel, invalid_config, proof, bounds);

//         match result {
//             Result::Ok(_) => { panic!("Verifier should return InvalidNumFriLayers"); },
//             Result::Err(error) => { assert!(error == FriVerificationError::InvalidNumFriLayers);
//             },
//         }
//     }

//     #[test]
//     fn proof_with_invalid_inner_layer_evaluation_fails_verification() {
//         let (config, proof, bounds, queries, decommitted_values) =
//             proof_with_last_layer_of_degree_four();

//         // Create an invalid proof by removing an evaluation from the second layer's proof
//         let inner_layers = @proof.inner_layers;
//         let invalid_proof = {
//             let mut invalid_inner_layers = array![];
//             // Keep the first layer as it is
//             invalid_inner_layers.append(proof.inner_layers[0].clone());

//             // Modify the second layer
//             let mut invalid_evals_subset = array![];
//             let mut i = 1;
//             while i < (*inner_layers[1].evals_subset).len() {
//                 invalid_evals_subset.append(inner_layers[1].evals_subset[i].clone());
//                 i += 1;
//             };
//             invalid_inner_layers
//                 .append(
//                     FriLayerProof {
//                         evals_subset: invalid_evals_subset.span(),
//                         decommitment: inner_layers[1].decommitment.clone(),
//                         commitment: *inner_layers[1].commitment,
//                     },
//                 );

//             // Keep the rest of the layers as they are
//             let mut i = 2;
//             while i < proof.inner_layers.len() {
//                 invalid_inner_layers.append(proof.inner_layers[i].clone());
//                 i += 1;
//             };

//             FriProof {
//                 inner_layers: invalid_inner_layers.span(),
//                 last_layer_poly: proof.last_layer_poly.clone(),
//             }
//         };

//         let mut channel = ChannelTrait::new(0x00);
//         let verifier = FriVerifierImpl::commit(ref channel, config, invalid_proof,
//         bounds.clone())
//             .unwrap();
//         let verification_result = verifier.decommit_on_queries(@queries, decommitted_values);

//         match verification_result {
//             Result::Ok(_) => { panic!("Verifier should return InnerLayerEvaluationsInvalid"); },
//             Result::Err(error) => {
//                 assert!(error == FriVerificationError::InnerLayerEvaluationsInvalid);
//             },
//         }
//     }

//     #[test]
//     fn proof_with_invalid_inner_layer_decommitment_fails_verification() {
//         let (config, proof, bounds, queries, decommitted_values) =
//             proof_with_last_layer_of_degree_four();

//         // Create an invalid proof by modifying the committed values in the second layer.
//         let inner_layers = @proof.inner_layers;
//         let invalid_proof = {
//             let mut invalid_inner_layers = array![];
//             // Keep the first layer as it is
//             invalid_inner_layers.append(proof.inner_layers[0].clone());

//             // Modify the second layer
//             let mut invalid_evals_subset = array![
//                 *inner_layers[1].evals_subset[0] + qm31(1, 0, 0, 0),
//             ];
//             let mut i = 1;
//             while i < (*inner_layers[1].evals_subset).len() {
//                 invalid_evals_subset.append(inner_layers[1].evals_subset[i].clone());
//                 i += 1;
//             };
//             invalid_inner_layers
//                 .append(
//                     FriLayerProof {
//                         evals_subset: invalid_evals_subset.span(),
//                         decommitment: inner_layers[1].decommitment.clone(),
//                         commitment: *inner_layers[1].commitment,
//                     },
//                 );

//             // Keep the rest of the layers as they are
//             let mut i = 2;
//             while i < proof.inner_layers.len() {
//                 invalid_inner_layers.append(proof.inner_layers[i].clone());
//                 i += 1;
//             };

//             FriProof {
//                 inner_layers: invalid_inner_layers.span(),
//                 last_layer_poly: proof.last_layer_poly.clone(),
//             }
//         };
//         let mut channel = ChannelTrait::new(0x00);
//         let verifier = FriVerifierImpl::commit(ref channel, config, invalid_proof,
//         bounds).unwrap();

//         let verification_result = verifier.decommit_on_queries(@queries, decommitted_values);

//         match verification_result {
//             Result::Ok(_) => { panic!("Verifier should return InnerLayerCommitmentInvalid"); },
//             Result::Err(error) => {
//                 assert!(error == FriVerificationError::InnerLayerCommitmentInvalid);
//             },
//         }
//     }

//     #[test]
//     fn proof_with_invalid_last_layer_degree_fails_verification() {
//         let (config, mut proof, bounds, _, _) = proof_with_last_layer_of_degree_four();

//         proof
//             .last_layer_poly =
//                 LinePoly {
//                     coeffs: array![qm31(1, 0, 0, 0), qm31(1, 0, 0, 0)], ..proof.last_layer_poly,
//                 };

//         let mut channel = ChannelTrait::new(0x00);
//         let verifier = FriVerifierImpl::commit(ref channel, config, proof, bounds);
//         match verifier {
//             Result::Ok(_) => { panic!("Verifier should return LastLayerDegreeInvalid"); },
//             Result::Err(error) => {
//                 assert!(error == FriVerificationError::LastLayerDegreeInvalid);
//             },
//         }
//     }

//     #[should_panic]
//     #[test]
//     fn decommit_queries_on_invalid_domain_fails_verification() {
//         let (config, proof, bounds, queries, decommitted_values) =
//             proof_with_last_layer_of_degree_four();

//         let mut invalid_queries = queries;
//         invalid_queries.log_domain_size -= 1;

//         let mut channel = ChannelTrait::new(0x00);
//         let verifier_result = FriVerifierImpl::commit(ref channel, config, proof, bounds);
//         match verifier_result {
//             Result::Ok(verifier) => {
//                 verifier.decommit_on_queries(@invalid_queries, decommitted_values).unwrap();
//             },
//             Result::Err(_) => {},
//         }
//     }

//     //////////////////////////////////////////////////////
//     // Proofs extracted from Stwo's rust implementation //
//     //////////////////////////////////////////////////////

//     fn proof_with_constant_last_layer() -> ProofValues {
//         let config = FriConfig {
//             log_blowup_factor: 1, log_last_layer_degree_bound: 0, n_queries: 1,
//         };

//         let proof = FriProof {
//             inner_layers: array![
//                 FriLayerProof {
//                     evals_subset: array![qm31(1654551922, 1975507039, 724492960,
//                     302041406)].span(), decommitment: MerkleDecommitment {
//                         hash_witness: array![
//                             0x02894fb64f5b5ad74ad6868ded445416d52840c2c4a36499f0eb37a03841bfc8,
//                             0x05d3f79e2cfd15b605e1e8eb759aa79e775e89df7c4ae5966efe3b96d3554003,
//                         ]
//                             .span(),
//                         column_witness: array![].span(),
//                     },
//                     commitment:
//                     0x03e5bad5822d062c05ff947d282dc2d56a6a420d14f2f74972bb5b01287731a7,
//                 },
//                 FriLayerProof {
//                     evals_subset: array![qm31(1396531676, 750161390, 1275165237, 1824394799)]
//                         .span(),
//                     decommitment: MerkleDecommitment {
//                         hash_witness: array![
//                             0x0539eb6bd5d99019f938130703ddfd97aaa9f46dea9714f9ed75528babb4db55,
//                         ]
//                             .span(),
//                         column_witness: array![].span(),
//                     },
//                     commitment:
//                     0x078189f0ad5c044994f4b3100183203ed10545891f2459770dde4af4b9c2def7,
//                 },
//             ]
//                 .span(),
//             last_layer_poly: LinePoly {
//                 coeffs: array![qm31(1030963115, 122157260, 1848484002, 1387601044)], log_size: 0,
//             },
//         };
//         let bounds = array![3];

//         let queries = Queries { positions: array![5], log_domain_size: 4 };
//         let domain = CircleDomain {
//             half_coset: Coset {
//                 initial_index: CirclePointIndexImpl::new(603979776),
//                 step_size: CirclePointIndexImpl::new(2147483648),
//                 log_size: 0,
//             },
//         };
//         let decommitted_values = array![
//             SparseCircleEvaluation {
//                 subcircle_evals: array![
//                     CircleEvaluationImpl::new(
//                         domain, array![qm31(1990458477, 0, 0, 0), qm31(1966717173, 0, 0, 0)],
//                     ),
//                 ],
//             },
//         ];

//         (config, proof, bounds, queries, decommitted_values)
//     }

//     fn proof_with_linear_last_layer() -> ProofValues {
//         let config = FriConfig {
//             log_blowup_factor: 1, log_last_layer_degree_bound: 1, n_queries: 1,
//         };

//         let proof = FriProof {
//             inner_layers: array![
//                 FriLayerProof {
//                     evals_subset: array![qm31(1654551922, 1975507039, 724492960,
//                     302041406)].span(), decommitment: MerkleDecommitment {
//                         hash_witness: array![
//                             0x02894fb64f5b5ad74ad6868ded445416d52840c2c4a36499f0eb37a03841bfc8,
//                             0x05d3f79e2cfd15b605e1e8eb759aa79e775e89df7c4ae5966efe3b96d3554003,
//                         ]
//                             .span(),
//                         column_witness: array![].span(),
//                     },
//                     commitment:
//                     0x03e5bad5822d062c05ff947d282dc2d56a6a420d14f2f74972bb5b01287731a7,
//                 },
//             ]
//                 .span(),
//             last_layer_poly: LinePoly {
//                 coeffs: array![
//                     qm31(1166420758, 1481024254, 705780805, 948549530),
//                     qm31(1166420758, 1481024254, 705780805, 948549530),
//                 ],
//                 log_size: 1,
//             },
//         };
//         let bounds = array![3];

//         let queries = Queries { positions: array![5], log_domain_size: 4 };
//         let domain = CircleDomain {
//             half_coset: Coset {
//                 initial_index: CirclePointIndexImpl::new(603979776),
//                 step_size: CirclePointIndexImpl::new(2147483648),
//                 log_size: 0,
//             },
//         };
//         let decommitted_values = array![
//             SparseCircleEvaluation {
//                 subcircle_evals: array![
//                     CircleEvaluationImpl::new(
//                         domain, array![qm31(1990458477, 0, 0, 0), qm31(1966717173, 0, 0, 0)],
//                     ),
//                 ],
//             },
//         ];

//         (config, proof, bounds, queries, decommitted_values)
//     }

//     fn proof_with_last_layer_of_degree_four() -> ProofValues {
//         let config = FriConfig {
//             log_blowup_factor: 1, log_last_layer_degree_bound: 2, n_queries: 1,
//         };
//         let proof = FriProof {
//             inner_layers: array![
//                 FriLayerProof {
//                     evals_subset: array![qm31(421951112, 668736057, 785571716,
//                     551382471)].span(), decommitment: MerkleDecommitment {
//                         hash_witness: array![
//                             0x07434e99a997fed5183f02e248b2d77ce047e45a63418dd8039630b139d72487,
//                             0x01e1aafd718c486b5e9b35927b27a6eb71ef97cdc7009c9f246647db63a7960c,
//                             0x0718cb047c50ba071b9a4696537695f273f42a7af8bfb0e465190b905548f754,
//                             0x040db6d0f16909d1daaf710e3fa9663ef52419ac5ae5433c915ac5939809eb79,
//                             0x06eb066c6e21999bc152bbac0a4b93c6c80b702f6ff7860be62cc10b89aa8352,
//                         ]
//                             .span(),
//                         column_witness: array![].span(),
//                     },
//                     commitment:
//                     0x07bc3121028865ac7ce98ec2cdbc6b4716ef91880374f6a8e93661fe51a759dc,
//                 },
//                 FriLayerProof {
//                     evals_subset: array![qm31(1535376180, 1101522806, 788857635, 1882322924)]
//                         .span(),
//                     decommitment: MerkleDecommitment {
//                         hash_witness: array![
//                             0x036ecb4e522350744312fa6da1ed85f6b7975885983a1baf9563feae7b7f799a,
//                             0x017bdda6c344feddd93884211b626ca806da73bfa55cd7eef54b687dd744651a,
//                             0x038d80d42b4192fd30dc894d5a26f3db757da5313c7940685058641091eb6d71,
//                             0x0406355da40056abcf1278c92f3ab9aa52ca028fe437e6dbe15cdbcc7b83eed0,
//                         ]
//                             .span(),
//                         column_witness: array![].span(),
//                     },
//                     commitment:
//                     0x046198bc34caa0b046234fa46ef591327a6864cb8a373bc13ce2cc9b3d5f3720,
//                 },
//                 FriLayerProof {
//                     evals_subset: array![qm31(419894864, 130791138, 1485752547,
//                     11937027)].span(), decommitment: MerkleDecommitment {
//                         hash_witness: array![
//                             0x0454d5cffc792c2308fb8dcf992c255f0535048b7bfbe9d08c1c3ae92178cd16,
//                             0x071f311ea2e00f2e44066f0a577f27e62648b66152afa3122e0aebe7420fbcd2,
//                             0x037c8315cf3525ea7097be7b687f44f9f0cecf1054ec553e183f0a9d2bd0b5d7,
//                         ]
//                             .span(),
//                         column_witness: array![].span(),
//                     },
//                     commitment:
//                     0x0344b5796a1b37e154053d94532921bd1dc9db98b454d0a7537974e2b9fc17b5,
//                 },
//             ]
//                 .span(),
//             last_layer_poly: LinePoly {
//                 coeffs: array![
//                     qm31(1449311934, 1632038525, 278574869, 690369710),
//                     qm31(1449311934, 1632038525, 278574869, 690369710),
//                     qm31(1449311934, 1632038525, 278574869, 690369710),
//                     qm31(1449311934, 1632038525, 278574869, 690369710),
//                 ],
//                 log_size: 2,
//             },
//         };
//         let bounds = array![6];
//         let queries = Queries { positions: array![5], log_domain_size: 7 };
//         let decommitted_values = array![
//             SparseCircleEvaluation {
//                 subcircle_evals: array![
//                     CircleEvaluationImpl::new(
//                         CircleDomain {
//                             half_coset: Coset {
//                                 initial_index: CirclePointIndexImpl::new(545259520),
//                                 step_size: CirclePointIndexImpl::new(2147483648),
//                                 log_size: 0,
//                             },
//                         },
//                         array![qm31(1464849549, 0, 0, 0), qm31(35402781, 0, 0, 0)],
//                     ),
//                 ],
//             },
//         ];
//         (config, proof, bounds, queries, decommitted_values)
//     }

//     fn proof_with_mixed_degree_1() -> ProofValues {
//         let config = FriConfig {
//             log_blowup_factor: 1, log_last_layer_degree_bound: 2, n_queries: 2,
//         };

//         let proof = FriProof {
//             inner_layers: array![
//                 FriLayerProof {
//                     evals_subset: array![
//                         qm31(1332072020, 1609661801, 1897498023, 686558487),
//                         qm31(886239056, 1157828441, 2019876782, 1060063104),
//                     ]
//                         .span(),
//                     decommitment: MerkleDecommitment {
//                         hash_witness: array![
//                             0x07434e99a997fed5183f02e248b2d77ce047e45a63418dd8039630b139d72487,
//                             0x020bcf949298f97180c360f6d55c2f65c19b9f3811c917d0368fe7203b53abcc,
//                             0x01e1aafd718c486b5e9b35927b27a6eb71ef97cdc7009c9f246647db63a7960c,
//                             0x062dd5d3993b66c78baf3608a2ed3de1ad865d0b174e006c8047b91fde98e462,
//                             0x0718cb047c50ba071b9a4696537695f273f42a7af8bfb0e465190b905548f754,
//                             0x055191c91b0668bab9271863162448c3396e8c2fc29f61bb621858210f4d0771,
//                             0x040db6d0f16909d1daaf710e3fa9663ef52419ac5ae5433c915ac5939809eb79,
//                             0x06ff62ebff373bc63508ad4c9c9997e38aa91331e1159c2809d81fd20b7a07e3,
//                         ]
//                             .span(),
//                         column_witness: array![].span(),
//                     },
//                     commitment:
//                     0x07bc3121028865ac7ce98ec2cdbc6b4716ef91880374f6a8e93661fe51a759dc,
//                 },
//                 FriLayerProof {
//                     evals_subset: array![
//                         qm31(1263943078, 172354194, 2127081660, 1999316363),
//                         qm31(1311532324, 582549508, 1702052122, 36581530),
//                     ]
//                         .span(),
//                     decommitment: MerkleDecommitment {
//                         hash_witness: array![
//                             0x05b7057376e8da41e7d1da285482571944f47848332279ce0de6b5ceeb21cb22,
//                             0x00e3a1b78a35229bb9a60ad0d1bab478f52a087cd15c51dfa83cd47fc6bb7334,
//                             0x013b2a8963d1de05e52484bf6e62fe25855780625d8e6f831cbac73801339267,
//                             0x0163c94c52552862374f1d7b09036d7cf74b4d59914c4393503cfd9bc49d53d3,
//                             0x0668d865abd1cb2b868c20784728cd48a6c4cbd926da318ce8814c5dae779fd0,
//                             0x0774e25d9d61fc18f3c2a365213b81fd36cced1626e02afc4dbe4aef52021769,
//                         ]
//                             .span(),
//                         column_witness: array![].span(),
//                     },
//                     commitment:
//                     0x06a3f2b104508429f6b74edcd62044afb4f618302a382f281fee118b12dc9dbd,
//                 },
//                 FriLayerProof {
//                     evals_subset: array![
//                         qm31(1660083285, 865580381, 2025493291, 1151079474),
//                         qm31(24828450, 1304266370, 129024597, 1635057579),
//                     ]
//                         .span(),
//                     decommitment: MerkleDecommitment {
//                         hash_witness: array![
//                             0x03c70415b07af713627bd1405e284be50c30ce6b628fb6a7d2accd3bb631c04c,
//                             0x062c36d3bd5fec84f54c5e835935a923db06521e937e3fbdd99cd9cd9701a329,
//                             0x0414361ae7771b465d1ed7241c6a9c383e19cee6db3230e16164ded0da216c4d,
//                             0x03abab172aeee1c04052395036cc50a51fb2497cfd307c96f32e718c4b3639cc,
//                         ]
//                             .span(),
//                         column_witness: array![].span(),
//                     },
//                     commitment:
//                     0x03d2565deb5099be20df825aabfe4678a0922d6b4a988d23a553c9f06b5bf96e,
//                 },
//             ]
//                 .span(),
//             last_layer_poly: LinePoly {
//                 coeffs: array![
//                     qm31(1365318542, 1863705492, 1698090260, 381798840),
//                     qm31(1365318542, 1863705492, 1698090260, 381798840),
//                     qm31(1365318542, 1863705492, 1698090260, 381798840),
//                     qm31(1365318542, 1863705492, 1698090260, 381798840),
//                 ],
//                 log_size: 2,
//             },
//         };
//         let bounds = array![6, 5, 4];
//         let queries = Queries { positions: array![7, 70], log_domain_size: 7 };
//         let decommitted_values = array![
//             SparseCircleEvaluation {
//                 subcircle_evals: array![
//                     CircleEvaluationImpl::new(
//                         CircleDomain {
//                             half_coset: Coset {
//                                 initial_index: CirclePointIndexImpl::new(1619001344),
//                                 step_size: CirclePointIndexImpl::new(2147483648),
//                                 log_size: 0,
//                             },
//                         },
//                         array![qm31(83295654, 0, 0, 0), qm31(666640840, 0, 0, 0)],
//                     ),
//                     CircleEvaluationImpl::new(
//                         CircleDomain {
//                             half_coset: Coset {
//                                 initial_index: CirclePointIndexImpl::new(1652555776),
//                                 step_size: CirclePointIndexImpl::new(2147483648),
//                                 log_size: 0,
//                             },
//                         },
//                         array![qm31(1598588979, 0, 0, 0), qm31(1615371031, 0, 0, 0)],
//                     ),
//                 ],
//             },
//             SparseCircleEvaluation {
//                 subcircle_evals: array![
//                     CircleEvaluationImpl::new(
//                         CircleDomain {
//                             half_coset: Coset {
//                                 initial_index: CirclePointIndexImpl::new(1090519040),
//                                 step_size: CirclePointIndexImpl::new(2147483648),
//                                 log_size: 0,
//                             },
//                         },
//                         array![qm31(985597997, 0, 0, 0), qm31(139496415, 0, 0, 0)],
//                     ),
//                     CircleEvaluationImpl::new(
//                         CircleDomain {
//                             half_coset: Coset {
//                                 initial_index: CirclePointIndexImpl::new(1157627904),
//                                 step_size: CirclePointIndexImpl::new(2147483648),
//                                 log_size: 0,
//                             },
//                         },
//                         array![qm31(1718103579, 0, 0, 0), qm31(1537119660, 0, 0, 0)],
//                     ),
//                 ],
//             },
//             SparseCircleEvaluation {
//                 subcircle_evals: array![
//                     CircleEvaluationImpl::new(
//                         CircleDomain {
//                             half_coset: Coset {
//                                 initial_index: CirclePointIndexImpl::new(33554432),
//                                 step_size: CirclePointIndexImpl::new(2147483648),
//                                 log_size: 0,
//                             },
//                         },
//                         array![qm31(1645691043, 0, 0, 0), qm31(2009531552, 0, 0, 0)],
//                     ),
//                     CircleEvaluationImpl::new(
//                         CircleDomain {
//                             half_coset: Coset {
//                                 initial_index: CirclePointIndexImpl::new(167772160),
//                                 step_size: CirclePointIndexImpl::new(2147483648),
//                                 log_size: 0,
//                             },
//                         },
//                         array![qm31(354887788, 0, 0, 0), qm31(934393698, 0, 0, 0)],
//                     ),
//                 ],
//             },
//         ];

//         (config, proof, bounds, queries, decommitted_values)
//     }

//     fn proof_with_mixed_degree_2() -> (
//         FriConfig, FriProof, Array<u32>, Array<SparseCircleEvaluation>,
//     ) {
//         let config = FriConfig {
//             log_blowup_factor: 1, log_last_layer_degree_bound: 2, n_queries: 3,
//         };

//         let proof = FriProof {
//             inner_layers: array![
//                 FriLayerProof {
//                     evals_subset: array![
//                         qm31(1398603058, 1957874897, 461138270, 1700080921),
//                         qm31(393493522, 576752954, 1963336729, 1268892468),
//                         qm31(97718382, 739321442, 646668452, 906233770),
//                     ]
//                         .span(),
//                     decommitment: MerkleDecommitment {
//                         hash_witness: array![
//                             0x0220da6892f2906e76c2713dc027eba3b2df3dfc6c680d354061eb59372822d5,
//                             0x020bcf949298f97180c360f6d55c2f65c19b9f3811c917d0368fe7203b53abcc,
//                             0x0367082a2edcf72c44ec838abbd372aa27d39ecc3387791bf686a712db309846,
//                             0x028514dd0ce02e8e3266b17b788f200d1ae49cc5f007fe3bd98e90529192aac3,
//                             0x062dd5d3993b66c78baf3608a2ed3de1ad865d0b174e006c8047b91fde98e462,
//                             0x04c76a6b839945fb3cdab23a3c01333a0fa755eaa0631d76fc2d7f77cb9dbeb8,
//                             0x03af1609280ef18b58dfe676fa9ac9288ebc4f2a48f511fe714b059c487455da,
//                             0x01c0a53fdf814604afe54aebd2a6d2880b072e217367b3adcc8a9bc14269015f,
//                             0x06ff62ebff373bc63508ad4c9c9997e38aa91331e1159c2809d81fd20b7a07e3,
//                         ]
//                             .span(),
//                         column_witness: array![].span(),
//                     },
//                     commitment:
//                     0x07bc3121028865ac7ce98ec2cdbc6b4716ef91880374f6a8e93661fe51a759dc,
//                 },
//                 FriLayerProof {
//                     evals_subset: array![
//                         qm31(1943731343, 338094235, 1579129158, 1325042400),
//                         qm31(1311532324, 582549508, 1702052122, 36581530),
//                         qm31(1561129265, 1456838851, 1325040656, 1580898325),
//                     ]
//                         .span(),
//                     decommitment: MerkleDecommitment {
//                         hash_witness: array![
//                             0x021bdc711e4823702cb7da701c301ebc832ffc967a21932d66f7998e9efbbf46,
//                             0x00e3a1b78a35229bb9a60ad0d1bab478f52a087cd15c51dfa83cd47fc6bb7334,
//                             0x0046d76cf189a1c1a9aad123f2c6a447af2c9fa4a7f58e11cd1852c00011a74b,
//                             0x03f8cb35e41d5291f1539b1cd73b018d6510aa85ba3bc9720e6014aa95ec4248,
//                             0x002d5922250cdbfedf908cabd24a158e9bdbb3de503e7636376c8a74921b8d41,
//                             0x0774e25d9d61fc18f3c2a365213b81fd36cced1626e02afc4dbe4aef52021769,
//                         ]
//                             .span(),
//                         column_witness: array![].span(),
//                     },
//                     commitment:
//                     0x06a3f2b104508429f6b74edcd62044afb4f618302a382f281fee118b12dc9dbd,
//                 },
//                 FriLayerProof {
//                     evals_subset: array![
//                         qm31(1219072197, 1782590850, 228657378, 784891462),
//                         qm31(24828450, 1304266370, 129024597, 1635057579),
//                         qm31(715531896, 1292811410, 725451910, 1608811481),
//                     ]
//                         .span(),
//                     decommitment: MerkleDecommitment {
//                         hash_witness: array![
//                             0x058978bedb6abe931a3de1cdff9bce0f7e7ac7b14c9c2107ea66679874a67e9a,
//                             0x027cf2f25d11835dbf4d3e0b0a3ab32f5b75de4f9904d1873115ecb5f2bd0555,
//                             0x03abab172aeee1c04052395036cc50a51fb2497cfd307c96f32e718c4b3639cc,
//                         ]
//                             .span(),
//                         column_witness: array![].span(),
//                     },
//                     commitment:
//                     0x03d2565deb5099be20df825aabfe4678a0922d6b4a988d23a553c9f06b5bf96e,
//                 },
//             ]
//                 .span(),
//             last_layer_poly: LinePoly {
//                 coeffs: array![
//                     qm31(1365318542, 1863705492, 1698090260, 381798840),
//                     qm31(1365318542, 1863705492, 1698090260, 381798840),
//                     qm31(1365318542, 1863705492, 1698090260, 381798840),
//                     qm31(1365318542, 1863705492, 1698090260, 381798840),
//                 ],
//                 log_size: 2,
//             },
//         };
//         let bounds = array![6, 5, 4];
//         let decommitted_values = array![
//             SparseCircleEvaluation {
//                 subcircle_evals: array![
//                     CircleEvaluationImpl::new(
//                         CircleDomain {
//                             half_coset: Coset {
//                                 initial_index: CirclePointIndexImpl::new(209715200),
//                                 step_size: CirclePointIndexImpl::new(2147483648),
//                                 log_size: 0,
//                             },
//                         },
//                         array![qm31(1784241578, 0, 0, 0), qm31(714402375, 0, 0, 0)],
//                     ),
//                     CircleEvaluationImpl::new(
//                         CircleDomain {
//                             half_coset: Coset {
//                                 initial_index: CirclePointIndexImpl::new(578813952),
//                                 step_size: CirclePointIndexImpl::new(2147483648),
//                                 log_size: 0,
//                             },
//                         },
//                         array![qm31(673384396, 0, 0, 0), qm31(475618425, 0, 0, 0)],
//                     ),
//                     CircleEvaluationImpl::new(
//                         CircleDomain {
//                             half_coset: Coset {
//                                 initial_index: CirclePointIndexImpl::new(981467136),
//                                 step_size: CirclePointIndexImpl::new(2147483648),
//                                 log_size: 0,
//                             },
//                         },
//                         array![qm31(315059915, 0, 0, 0), qm31(558088919, 0, 0, 0)],
//                     ),
//                 ],
//             },
//             SparseCircleEvaluation {
//                 subcircle_evals: array![
//                     CircleEvaluationImpl::new(
//                         CircleDomain {
//                             half_coset: Coset {
//                                 initial_index: CirclePointIndexImpl::new(419430400),
//                                 step_size: CirclePointIndexImpl::new(2147483648),
//                                 log_size: 0,
//                             },
//                         },
//                         array![qm31(142767236, 0, 0, 0), qm31(537984732, 0, 0, 0)],
//                     ),
//                     CircleEvaluationImpl::new(
//                         CircleDomain {
//                             half_coset: Coset {
//                                 initial_index: CirclePointIndexImpl::new(1157627904),
//                                 step_size: CirclePointIndexImpl::new(2147483648),
//                                 log_size: 0,
//                             },
//                         },
//                         array![qm31(1718103579, 0, 0, 0), qm31(1537119660, 0, 0, 0)],
//                     ),
//                     CircleEvaluationImpl::new(
//                         CircleDomain {
//                             half_coset: Coset {
//                                 initial_index: CirclePointIndexImpl::new(1962934272),
//                                 step_size: CirclePointIndexImpl::new(2147483648),
//                                 log_size: 0,
//                             },
//                         },
//                         array![qm31(2124636505, 0, 0, 0), qm31(1506525049, 0, 0, 0)],
//                     ),
//                 ],
//             },
//             SparseCircleEvaluation {
//                 subcircle_evals: array![
//                     CircleEvaluationImpl::new(
//                         CircleDomain {
//                             half_coset: Coset {
//                                 initial_index: CirclePointIndexImpl::new(838860800),
//                                 step_size: CirclePointIndexImpl::new(2147483648),
//                                 log_size: 0,
//                             },
//                         },
//                         array![qm31(1014591066, 0, 0, 0), qm31(1931899148, 0, 0, 0)],
//                     ),
//                     CircleEvaluationImpl::new(
//                         CircleDomain {
//                             half_coset: Coset {
//                                 initial_index: CirclePointIndexImpl::new(167772160),
//                                 step_size: CirclePointIndexImpl::new(2147483648),
//                                 log_size: 0,
//                             },
//                         },
//                         array![qm31(354887788, 0, 0, 0), qm31(934393698, 0, 0, 0)],
//                     ),
//                     CircleEvaluationImpl::new(
//                         CircleDomain {
//                             half_coset: Coset {
//                                 initial_index: CirclePointIndexImpl::new(1778384896),
//                                 step_size: CirclePointIndexImpl::new(2147483648),
//                                 log_size: 0,
//                             },
//                         },
//                         array![qm31(509977960, 0, 0, 0), qm31(1887908506, 0, 0, 0)],
//                     ),
//                 ],
//             },
//         ];
//         (config, proof, bounds, decommitted_values)
//     }
// }


