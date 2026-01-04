use core::array::ArrayImpl;
use core::num::traits::{One, Zero};
use stwo_verifier_utils::zip_eq::zip_eq;
use crate::circle::{
    CirclePoint, CirclePointIndexImpl, CirclePointIndexTrait, CirclePointQM31AddCirclePointM31Trait,
    CosetImpl, M31_CIRCLE_LOG_ORDER,
};
use crate::fields::cm31::{CM31, CM31Trait, MulByCM31Trait};
use crate::fields::m31::{M31, M31Zero, MulByM31Trait};
use crate::fields::qm31::{PackedUnreducedQM31, PackedUnreducedQM31Trait, QM31, QM31One, QM31Trait};
use crate::fields::{BatchInvertible, Invertible};
use crate::poly::circle::{
    CanonicCosetImpl, CanonicCosetTrait, CircleDomainImpl, CircleEvaluationImpl,
};
use crate::utils::{
    ArrayImpl as ArrayUtilImpl, ColumnsIndicesPerTreeByLogDegreeBound, SpanExTrait, SpanImpl,
    bit_reverse_index, pack_qm31,
};
use crate::{TreeArray, TreeSpan};
use super::verifier::{QueriedValues, SampledValues};


#[cfg(test)]
mod test;

/// Computes the OOD quotients at the query positions.
///
/// # Arguments
///
/// * `column_indices_per_tree_by_degree_bound`: The column indices grouped by tree and by log
/// degree bound.
/// * `oods_point`: The OOD point.
/// * `samples_per_column_per_tree`: OOD samples (only the eval) for each column in each tree.
/// * `random_coeff`: Verifier randomness for folding multiple columns' quotients together.
/// * `max_log_degree_bound`: The max log degree of a committed polynomial.
/// * `log_blowup_factor`: The FRI log blowup factor parameter.
/// * `query_positions`: Query positions, as indices in the largest LDE domain (i.e. the lifting
/// domain).
/// * `queried_values_per_tree`: For each tree, contains all queried trace values.
pub fn fri_answers(
    mut column_indices_per_tree_by_degree_bound: ColumnsIndicesPerTreeByLogDegreeBound,
    log_blowup_factor: u32,
    oods_point: CirclePoint<QM31>,
    sample_values_per_column_per_tree: SampledValues,
    random_coeff: QM31,
    query_positions: Span<usize>,
    queried_values_per_tree: QueriedValues,
    max_log_degree_bound: u32,
) -> Span<QM31> {
    // Note that `log_size` is equal to 1 + largest log size of a trace column (the additional 1
    // comes from calling `len()` on `column_indices_per_tree_by_degree_bound`).
    // Check that the largest log size of a trace column is <= `M31_CIRCLE_LOG_ORDER` - 1.
    assert!(
        max_log_degree_bound + log_blowup_factor <= M31_CIRCLE_LOG_ORDER, "log_size is too large",
    );
    let mut queried_values_per_tree = queried_values_per_tree.span();
    // Add to each sample value the corresponding random coefficient power.
    let samples_with_randomness: Span<Span<Span<(QM31, QM31)>>> = build_samples_with_randomness(
        sample_values_per_column_per_tree, random_coeff,
    );
    // Build the array `sample_batches_by_log_degree_bound`: for i ∈ [0, max_log_degree_bound],
    // `sample_batches_by_log_degree_bound[i]` contains a triple consisting of:
    // * the sample batches for all columns of log degree bound i.
    // * An array containing the number of columns of log size i in each tree.
    // * the quotient constants corresponding the sample batches of log size i.
    let mut sample_batches_by_log_degree_bound = array![];
    let lifting_log_size = max_log_degree_bound + log_blowup_factor;
    let lifting_domain = CanonicCosetImpl::new(lifting_log_size);

    let trace_step = CanonicCosetImpl::new(max_log_degree_bound).coset.step.mul(1).to_point();
    let prev_oods_point = oods_point.add_circle_point_m31(-trace_step);
    for column_indices_per_tree_for_degree_bound in column_indices_per_tree_by_degree_bound {
        let (sample_batches, n_cols_per_tree) = sample_batches_for_degree_bound(
            column_indices_per_tree_for_degree_bound,
            samples_with_randomness,
            oods_point,
            prev_oods_point,
        );
        let quotient_constants = QuotientConstantsImpl::gen(sample_batches);
        sample_batches_by_log_degree_bound
            .append((sample_batches, n_cols_per_tree, quotient_constants));
    }

    // Compute the fri answers.
    let mut answers: Array<QM31> = array![];
    for pos in query_positions {
        let mut fri_answer_for_position: QM31 = Zero::zero();
        let domain_point = lifting_domain
            .circle_domain()
            .at(bit_reverse_index(*pos, lifting_log_size));
        for (
            sample_batches, n_columns_per_tree, quotient_constants,
        ) in sample_batches_by_log_degree_bound
            .span() {
            let queried_values_at_row = tree_take_n(
                ref queried_values_per_tree, n_columns_per_tree.span(),
            );
            fri_answer_for_position +=
                accumulate_row_quotients(
                    *sample_batches, queried_values_at_row.span(), quotient_constants, domain_point,
                );
        }
        answers.append(fri_answer_for_position);
    }

    for queried_values in queried_values_per_tree {
        assert!(queried_values.is_empty())
    }

    answers.span()
}

/// Gathers sample batches and column counts for a given degree bound.
fn sample_batches_for_degree_bound(
    column_indices_per_tree: @TreeSpan<Span<usize>>,
    sample_values_with_rand: Span<Span<Span<(QM31, QM31)>>>,
    oods_point: CirclePoint<QM31>,
    prev_oods_point: CirclePoint<QM31>,
) -> (Span<ColumnSampleBatch>, TreeArray<usize>) {
    /// The (column index, evaluation) pairs at the out of domain point 'Z'.
    let mut indexed_evaluations_at_point = array![];
    /// The (column index, evaluation) pairs at the point `Z-g`.
    let mut indexed_evaluations_at_prev_point = array![];
    /// TODO(Leo): add periodicity checks in next PRs.

    let mut n_columns_per_tree = array![];
    let mut index = 0;
    for (column_indices, samples_per_column) in zip_eq(
        *column_indices_per_tree, sample_values_with_rand,
    ) {
        for column_idx in column_indices {
            // Note that samples_per_column[*column] can be an empty array.
            let mut sample_values_at_column = *samples_per_column[*column_idx];
            if sample_values_at_column.len() == 2 {
                let [(prev_point_sample, prev_point_rand), (point_sample, point_rand)] =
                    sample_values_at_column
                    .multi_pop_front::<2>()
                    .unwrap()
                    .unbox();

                indexed_evaluations_at_prev_point
                    .append((index, prev_point_sample, prev_point_rand));
                indexed_evaluations_at_point.append((index, point_sample, point_rand));
            } else if sample_values_at_column.len() == 1 {
                let (point_sample, rand): (QM31, QM31) = *(sample_values_at_column
                    .first()
                    .unwrap());

                indexed_evaluations_at_point.append((index, point_sample, rand));
            } else {
                assert!(sample_values_at_column.is_empty(), "Unexpected number of samples");
            }
            index += 1;
        }

        n_columns_per_tree.append(column_indices.len());
    }

    let mut sample_batches_by_point: Array<ColumnSampleBatch> = array![];
    if !indexed_evaluations_at_point.is_empty() {
        sample_batches_by_point
            .append(
                ColumnSampleBatch {
                    point: oods_point, cols_vals_and_pows: indexed_evaluations_at_point,
                },
            );
    }

    if !indexed_evaluations_at_prev_point.is_empty() {
        sample_batches_by_point
            .append(
                ColumnSampleBatch {
                    point: prev_oods_point, cols_vals_and_pows: indexed_evaluations_at_prev_point,
                },
            );
    }

    (sample_batches_by_point.span(), n_columns_per_tree)
}

fn build_samples_with_randomness(
    sample_values_per_column_per_tree: SampledValues, random_coeff: QM31,
) -> Span<Span<Span<(QM31, QM31)>>> {
    let mut samples_with_randomness_per_tree = array![];
    let mut random_pow: QM31 = One::one();
    for sample_values_per_column in sample_values_per_column_per_tree {
        let mut index = 0;
        let mut new_samples_per_col = array![];
        for sample_values in sample_values_per_column {
            let mut new_samples = array![];
            // TODO(Leo): add periodicity checks in the next PR.
            for val in sample_values {
                new_samples.append((*val, random_pow));
                random_pow *= random_coeff;
            }
            new_samples_per_col.append(new_samples.span());
            index += 1;
        }
        samples_with_randomness_per_tree.append(new_samples_per_col.span());
    }
    samples_with_randomness_per_tree.span()
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
    sample_batches_by_point: Span<ColumnSampleBatch>,
    queried_values_at_row: Span<M31>,
    quotient_constants: @QuotientConstants,
    domain_point: CirclePoint<M31>,
) -> QM31 {
    let denominator_inverses = quotient_denominator_inverses(sample_batches_by_point, domain_point);
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
    fn gen(sample_batches_by_point: Span<ColumnSampleBatch>) -> QuotientConstants {
        let mut point_constants = array![];

        for sample_batch in sample_batches_by_point {
            assert!(
                *sample_batch.point.y != (*sample_batch.point.y).complex_conjugate(),
                "Cannot evaluate a line with a single point ({:?}).",
                sample_batch.point,
            );

            // The coefficients (a_i, b_i, c_i) are the coefficients of the line
            //   c_i * F(q) - a_i * q.y - b_i through
            // through (p.y, v_i) and (conj(p.y), conj(v_i)) are:
            //   c_i =               conj(p.y) - p.y = -2u * Im(p.y),
            //   a_i =               conj(v_i) - v_i = -2u * Im(v_i),
            //   b_i = conj(p.y)*v_i - conj(v_i)*p.y = -2u * (Re(v_i)*Im(p.y) - Re(p.y)*Im(v_i)).
            // Note that c_i = c depends only on p.y and not on the value v_i.
            // We have to compute and store c * α^i for each i; we compute these directly,
            // without calculating each α^i. We use these to accumulate the sums
            //   Σ_i (c * α^i) * Im(v_i)
            //   Σ_i (c * α^i) * Re(v_i)
            // From these sums we then construct `alpha_mul_a_sum` and `alpha_mul_b_sum` by
            //   `alpha_mul_a_sum` = (Σ_i (c * α^i) * Im(v_i)) / Im(p.y)
            //   `alpha_mul_b_sum` = (Σ_i (c * α^i) * Re(v_i)) - (`alpha_mul_a_sum` * Re(p.y)).

            let [re_py_a, re_py_b, im_py_a, im_py_b] = sample_batch.point.y.to_fixed_array();
            let re_py = CM31Trait::pack(re_py_a, re_py_b);
            let im_py_inv = CM31Trait::pack(im_py_a, im_py_b).inverse();

            let c = QM31Trait::from_fixed_array(
                [M31Zero::zero(), M31Zero::zero(), im_py_a, im_py_b],
            );
            let minus_two_c = -(c + c);
            let mut alpha_mul_c_mul_im_sum = PackedUnreducedQM31Trait::large_zero();
            let mut alpha_mul_c_mul_re_sum = PackedUnreducedQM31Trait::large_zero();
            let mut indexed_alpha_mul_c: Array<(usize, PackedUnreducedQM31)> = array![];

            for (column_idx, sample_value, random_pow) in sample_batch.cols_vals_and_pows.span() {
                let [re_cv_a, re_cv_b, im_cv_a, im_cv_b] = sample_value.to_fixed_array();
                let alpha_mul_c = minus_two_c * *random_pow;
                let re_cv = CM31Trait::pack(re_cv_a, re_cv_b);
                let im_cv = CM31Trait::pack(im_cv_a, im_cv_b);
                let alpha_mul_c_packed: PackedUnreducedQM31 = alpha_mul_c.into();

                alpha_mul_c_mul_re_sum += alpha_mul_c_packed.mul_cm31(re_cv);
                alpha_mul_c_mul_im_sum += alpha_mul_c_packed.mul_cm31(im_cv);
                indexed_alpha_mul_c.append((*column_idx, alpha_mul_c_packed));
            }

            let alpha_mul_c_mul_im_sum_reduced: PackedUnreducedQM31 = alpha_mul_c_mul_im_sum
                .reduce()
                .into();
            let alpha_mul_a_sum = alpha_mul_c_mul_im_sum_reduced.mul_cm31(im_py_inv);
            let alpha_mul_b_sum = alpha_mul_c_mul_re_sum - alpha_mul_a_sum.mul_cm31(re_py);

            point_constants
                .append(
                    PointQuotientConstants {
                        alpha_mul_a_sum, alpha_mul_b_sum, indexed_alpha_mul_c,
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
    pub cols_vals_and_pows: Array<(usize, QM31, QM31)>,
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
