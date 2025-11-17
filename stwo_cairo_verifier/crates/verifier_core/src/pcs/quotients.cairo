use core::array::ArrayImpl;
use core::dict::Felt252Dict;
use core::nullable::{FromNullableResult, Nullable, match_nullable};
use core::num::traits::{One, Zero};
use stwo_verifier_utils::zip_eq::zip_eq;
use crate::circle::{
    CirclePoint, CirclePointIndexImpl, CirclePointQM31AddCirclePointM31Trait, CosetImpl,
    M31_CIRCLE_LOG_ORDER,
};
use crate::fields::BatchInvertible;
use crate::fields::cm31::CM31;
use crate::fields::m31::{M31, M31Zero, MulByM31Trait};
use crate::fields::qm31::{PackedUnreducedQM31, PackedUnreducedQM31Trait, QM31, QM31Trait};
use crate::poly::circle::{CanonicCosetImpl, CircleDomainImpl, CircleEvaluationImpl};
use crate::utils::{
    ArrayImpl as ArrayUtilImpl, ColumnsIndicesPerTreeByLogDegreeBound, SpanExTrait, SpanImpl,
    bit_reverse_index, pack_qm31,
};
use crate::{ColumnSpan, TreeArray, TreeSpan};


#[cfg(test)]
mod test;

/// Computes the OOD quotients at the query positions.
///
/// # Arguments
///
/// * `column_indices_per_tree_by_degree_bound`: The column indices grouped by tree and by log
/// degree bound.
/// * `log_blowup_factor`: The FRI log blowup factor parameter.
/// * `samples_per_column_per_tree`: OOD samples (i.e. point and eval) for each column in each tree.
/// * `random_coeff`: Verifier randomness for folding multiple columns' quotients together.
/// * `query_positions_per_log_size`: Query positions mapped by log commitment domain size.
/// * `queried_values_per_tree`: For each tree, contains all queried trace values, ordered first
pub fn fri_answers(
    mut column_indices_per_tree_by_degree_bound: ColumnsIndicesPerTreeByLogDegreeBound,
    log_blowup_factor: u32,
    oods_point: CirclePoint<QM31>,
    sample_values_per_column_per_tree: TreeSpan<ColumnSpan<Span<QM31>>>,
    random_coeff: QM31,
    mut query_positions_per_log_size: Felt252Dict<Nullable<Span<usize>>>,
    mut queried_values_per_tree: TreeSpan<Span<M31>>,
) -> Array<Span<QM31>> {
    // Note that `log_size` is equal to 1 + largest log size of a trace column (the additional 1
    // comes from calling `len()` on `column_indices_per_tree_by_degree_bound`).
    let mut log_size = column_indices_per_tree_by_degree_bound.len() + log_blowup_factor;
    // Check that the largest log size of a trace column is <= `M31_CIRCLE_LOG_ORDER` - 1.
    assert!(log_size <= M31_CIRCLE_LOG_ORDER, "log_size is too large");

    let mut answers = array![];
    while let Some(columns_per_tree) = column_indices_per_tree_by_degree_bound.pop_back() {
        log_size = log_size - 1;

        let queries_for_log_size =
            match match_nullable(query_positions_per_log_size.get(log_size.into())) {
            FromNullableResult::NotNull(value) => value.unbox(),
            FromNullableResult::Null => {
                // Skip processing this log size if it does not have any associated queries.
                continue;
            },
        };

        let Some((sample_batches_by_point, n_columns_per_tree)) = sample_batches_for_log_size(
            columns_per_tree,
            sample_values_per_column_per_tree,
            oods_point,
            log_size,
            log_blowup_factor,
        ) else {
            continue;
        };

        answers
            .append(
                fri_answers_for_log_size(
                    log_size,
                    sample_batches_by_point,
                    random_coeff,
                    queries_for_log_size,
                    ref queried_values_per_tree,
                    n_columns_per_tree,
                ),
            );
    }

    for queried_values in queried_values_per_tree {
        assert!(queried_values.is_empty())
    }

    answers
}

/// Gathers sample batches and column counts for a given log size.
///
/// Returns `None` if there are no columns for this log size, otherwise returns
/// `Some((sample_batches_by_point, n_columns_per_tree))`.
///
/// # Assumptions
///
/// Each column is sampled at one of the following sets of points:
/// - `[]`
/// - `[oods_point]`
/// - `[oods_point - g, oods_point]`
///
/// where `g` is the trace generator corresponding to the given column.
///
fn sample_batches_for_log_size(
    columns_per_tree: @TreeSpan<Span<usize>>,
    sample_values_per_column_per_tree: TreeSpan<ColumnSpan<Span<QM31>>>,
    oods_point: CirclePoint<QM31>,
    log_size: u32,
    log_blowup_factor: u32,
) -> Option<(Array<ColumnSampleBatch>, TreeArray<usize>)> {
    /// The (column index, evaluation) pairs at the out of domain point 'Z'.
    let mut indexed_evaluations_at_point = array![];
    /// The (column index, evaluation) pairs at the point `Z-g`.
    let mut indexed_evaluations_at_prev_point = array![];

    let mut n_columns_per_tree = array![];
    let mut index = 0;
    for (columns, samples_per_column) in zip_eq(
        *columns_per_tree, sample_values_per_column_per_tree,
    ) {
        for column in columns {
            // Note that samples_per_column[*column] can be an empty array.
            let sample_values_at_column = *samples_per_column[*column];

            if let Some(point_box) = sample_values_at_column.try_into() {
                let [point_sample]: [QM31; 1] = (*point_box).unbox();

                indexed_evaluations_at_point.append((index, point_sample));
            } else if let Some(tuple_box) = sample_values_at_column.try_into() {
                let [prev_point_sample, point_sample]: [QM31; 2] = (*tuple_box).unbox();

                indexed_evaluations_at_prev_point.append((index, prev_point_sample));
                indexed_evaluations_at_point.append((index, point_sample));
            } else {
                assert!(sample_values_at_column.is_empty(), "Unexpected number of samples");
            }
            index += 1;
        }

        n_columns_per_tree.append(columns.len());
    }

    if index == 0 {
        return Option::None;
    }

    let mut sample_batches_by_point: Array<ColumnSampleBatch> = array![];
    if !indexed_evaluations_at_point.is_empty() {
        sample_batches_by_point
            .append(
                ColumnSampleBatch {
                    point: oods_point, columns_and_values: indexed_evaluations_at_point,
                },
            );
    }

    if !indexed_evaluations_at_prev_point.is_empty() {
        let trace_gen = CanonicCosetImpl::new(log_size - log_blowup_factor).coset.step;
        let prev_point = oods_point.add_circle_point_m31(-trace_gen.mul(1).to_point());

        sample_batches_by_point
            .append(
                ColumnSampleBatch {
                    point: prev_point, columns_and_values: indexed_evaluations_at_prev_point,
                },
            );
    }

    Option::Some((sample_batches_by_point, n_columns_per_tree))
}

fn fri_answers_for_log_size(
    log_size: u32,
    sample_batches_by_point: Array<ColumnSampleBatch>,
    random_coeff: QM31,
    mut query_positions: Span<usize>,
    ref queried_values_per_tree: TreeSpan<Span<M31>>,
    n_columns_per_tree: TreeArray<usize>,
) -> Span<QM31> {
    let quotient_constants = QuotientConstantsImpl::gen(@sample_batches_by_point, random_coeff);
    let commitment_domain = CanonicCosetImpl::new(log_size).circle_domain();
    let mut quotient_evals_at_queries = array![];

    for query_position in query_positions {
        let queried_values_at_row = tree_take_n(
            ref queried_values_per_tree, n_columns_per_tree.span(),
        );
        quotient_evals_at_queries
            .append(
                accumulate_row_quotients(
                    @sample_batches_by_point,
                    queried_values_at_row.span(),
                    @quotient_constants,
                    commitment_domain.at(bit_reverse_index(*query_position, log_size)),
                ),
            );
    }

    quotient_evals_at_queries.span()
}

/// Computes the OOD quotients for a single query and single column size.
///
/// # Arguments
///
/// * `sampled_batches`: OOD column samples grouped by eval point.
/// * `query_evals_by_column`: Sampled query evals by trace column.
/// * `query_index`: The index of the query to compute the quotients for.
/// * `domain_point`: The domain point the query corresponds to.
#[inline(always)]
fn accumulate_row_quotients(
    sample_batches_by_point: @Array<ColumnSampleBatch>,
    queried_values_at_row: Span<M31>,
    quotient_constants: @QuotientConstants,
    domain_point: CirclePoint<M31>,
) -> QM31 {
    let denominator_inverses = quotient_denominator_inverses(
        sample_batches_by_point.span(), domain_point,
    );
    let domain_point_y: M31 = domain_point.y;
    let mut quotient_accumulator: QM31 = Zero::zero();

    for (point_constants, denom_inv) in zip_eq(
        quotient_constants.point_constants, denominator_inverses,
    ) {
        let PointQuotientConstants {
            alpha_mul_a_sum, alpha_mul_b_sum, indexed_alpha_mul_c,
        } = point_constants;

        // `minus_numerator` is offset by `PackedUnreducedQM31Trait::large_zero()` via
        // `alpha_mul_b_sum`. This ensures the subtraction below does not underflow.
        let mut minus_numerator = alpha_mul_a_sum.mul_m31(domain_point_y) + *alpha_mul_b_sum;
        for (column_index, alpha_mul_c) in indexed_alpha_mul_c.span() {
            let query_eval_at_column = *queried_values_at_row.at(*column_index);

            // The numerator is a line equation passing through
            //   (sample_point.y, sample_value), (conj(sample_point.y), conj(sample_value))
            // evaluated at (domain_point.y, value).
            // When substituting a polynomial in this line equation, we get a polynomial
            // with a root at sample_point and conj(sample_point) if the original polynomial
            // had the values sample_value and conj(sample_value) at these points.
            minus_numerator -= alpha_mul_c.mul_m31(query_eval_at_column);
        }

        // Subtract the accumulated linear term.

        let minus_quotient = minus_numerator.reduce().mul_cm31(denom_inv);
        quotient_accumulator = quotient_accumulator - minus_quotient;
    }

    quotient_accumulator
}

/// Computes the denominators of the FRI quotients for a given `domain_point`
/// (corresponding to a query index).
///
/// For each `sample_point` the value at `domain_point` of the line
/// passing through `(sample_point, conj(sample_point))`.
///
/// Conjugation is taken coordinate-wise conjugation with respect to CM31.
fn quotient_denominator_inverses(
    sample_batches: Span<ColumnSampleBatch>, domain_point: CirclePoint<M31>,
) -> Array<CM31> {
    let mut denominators = array![];

    for sample_batch in sample_batches {
        // For a sample point `P: CirclePoint<QM31>` domain point `D: CirclePoint<M31>`, the
        // denominator is given by
        //   (Pr.x - D.x) * Pi.y - (Pr.y - D.y) * Py.x
        // where Pr, Pi are the real and imaginary parts of P, both of type `CirclePoint<CM31>`.
        let denominator = QM31Trait::fused_quotient_denominator(
            *sample_batch.point.x, *sample_batch.point.y, domain_point.x, domain_point.y,
        );
        denominators.append(denominator);
    }

    BatchInvertible::batch_inverse(denominators)
}

/// Holds the precomputed constant values used in each quotient evaluation, grouped evaluation
/// point.
#[derive(Debug, Drop)]
pub struct QuotientConstants {
    /// The constants for each mask item.
    pub point_constants: Array<PointQuotientConstants>,
}

/// Constants associated with a batch of samples for a given evaluation point and domain size.
///
/// # Overview
/// To prove that `F(p) = value`, we apply *two-point quotienting* at `p`
/// and its conjugate `conj(p)`. The numerator of the quotient is:
///
///     c * F(q) - a * q.y - b
///
/// where `(a, b, c)` are the coefficients of the line through
/// `(p.y, value)` and `(conj(p.y), conj(value))`, ensuring the numerator
/// vanishes at both `p` and `conj(p)`.
///
/// Since `F` is a polynomial over the base field, we also have:
///
///     F(conj(p)) = conj(F(p))
///
/// # Batched Evaluation Proofs
/// In batched evaluation proof verification, the verifier computes a pseudo-random
/// linear combination of these quotients:
///
///     Σ (α^i * (c_i * F_i(q) - a_i * q.y - b_i))
///
/// which expands to:
///
///     Σ (α^i * c_i * F_i(q)) - q.y * Σ (α^i * a_i) - Σ (α^i * b_i)
///
/// To evaluate this expression efficiently at the query point `q`, we compute
/// the following for each batch:
///
/// - `alpha_mul_a_sum`: Σ (α^i * a_i)
/// - `alpha_mul_b_sum`: Σ (α^i * b_i)
/// - `indexed_alpha_mul_c`: list of `(column index, α^i * c_i)` pairs
///
/// where i ∈ [index_of_first_sample_in_the_batch, index_of_first_sample_in_the_next_batch).
#[derive(Debug, Drop)]
pub struct PointQuotientConstants {
    /// Σ (α^i * a_i) across all samples in the batch.
    pub alpha_mul_a_sum: PackedUnreducedQM31,
    /// Σ (α^i * b_i) across all samples in the batch.
    pub alpha_mul_b_sum: PackedUnreducedQM31,
    /// Pairs of `(column index, α^i * c_i)` for every sample.
    pub indexed_alpha_mul_c: Array<(usize, PackedUnreducedQM31)>,
}

#[generate_trait]
impl QuotientConstantsImpl of QuotientConstantsTrait {
    fn gen(
        sample_batches_by_point: @Array<ColumnSampleBatch>, random_coeff: QM31,
    ) -> QuotientConstants {
        let mut point_constants = array![];

        let mut alpha: QM31 = One::one();
        for sample_batch in sample_batches_by_point.span() {
            assert!(
                *sample_batch.point.y != (*sample_batch.point.y).complex_conjugate(),
                "Cannot evaluate a line with a single point ({:?}).",
                sample_batch.point,
            );

            let neg_dbl_im_py = neg_twice_imaginary_part(sample_batch.point.y);
            let mut alpha_mul_a_sum = PackedUnreducedQM31Trait::large_zero();
            let mut alpha_mul_b_sum = PackedUnreducedQM31Trait::large_zero();
            let mut indexed_alpha_mul_c: Array<(usize, PackedUnreducedQM31)> = array![];

            for (column_idx, column_value) in sample_batch.columns_and_values.span() {
                let alpha_mul_a = alpha * neg_twice_imaginary_part(column_value);
                let alpha_mul_c = alpha * neg_dbl_im_py;
                let alpha_mul_b = QM31Trait::fused_mul_sub(
                    *column_value, alpha_mul_c, alpha_mul_a * *sample_batch.point.y,
                );
                alpha_mul_a_sum += alpha_mul_a.into();
                alpha_mul_b_sum += alpha_mul_b.into();
                indexed_alpha_mul_c.append((*column_idx, alpha_mul_c.into()));
                alpha = alpha * random_coeff;
            }

            point_constants
                .append(
                    PointQuotientConstants {
                        alpha_mul_a_sum: alpha_mul_a_sum.reduce().into(),
                        alpha_mul_b_sum: alpha_mul_b_sum,
                        indexed_alpha_mul_c,
                    },
                );
        }

        QuotientConstants { point_constants }
    }
}

/// A batch of column samplings at a point.
#[derive(Debug, Drop, PartialEq)]
pub struct ColumnSampleBatch {
    /// The point at which the columns are sampled.
    pub point: CirclePoint<QM31>,
    /// The sampled column indices and their values at the point.
    pub columns_and_values: Array<(usize, QM31)>,
}

/// Returns `complex_conjugate(v) - v`.
#[inline]
pub fn neg_twice_imaginary_part(v: @QM31) -> QM31 {
    let [_, _, c, d] = v.to_fixed_array();
    let v = QM31Trait::from_fixed_array([M31Zero::zero(), M31Zero::zero(), c, d]);
    -(v + v)
}

/// A circle point encoding to index into [`Felt252Dict`].
#[generate_trait]
pub impl CirclePointQM31Key of CirclePointQM31KeyTrait {
    fn encode(key: @CirclePoint<QM31>) -> felt252 {
        let encoded_y = pack_qm31(Zero::zero(), *key.y);
        pack_qm31(encoded_y, *key.x)
    }
}

/// Pops `ns[i]` elements from the `trees[i]` and returns them as a flat array.
fn tree_take_n<T, +Clone<T>, +Drop<T>>(
    ref trees: TreeSpan<Span<T>>, mut ns: TreeSpan<usize>,
) -> Array<T> {
    let mut res: Array<T> = array![];
    let mut new_trees = array![];
    for (values, n) in zip_eq(trees, ns) {
        let mut values = *values;
        res.append_span(values.pop_front_n(*n));
        new_trees.append(values);
    }

    trees = new_trees.span();
    res
}
