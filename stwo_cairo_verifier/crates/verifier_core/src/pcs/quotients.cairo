use core::array::ArrayImpl;
use core::dict::{Felt252Dict, Felt252DictEntryTrait};
use core::nullable::{FromNullableResult, Nullable, NullableTrait, match_nullable, null};
use core::num::traits::{One, Zero};
use stwo_verifier_utils::zip_eq::zip_eq;
use crate::circle::{CirclePoint, CirclePointIndexImpl, CosetImpl, M31_CIRCLE_LOG_ORDER};
use crate::fields::BatchInvertible;
use crate::fields::cm31::{CM31, CM31Trait};
use crate::fields::m31::{M31, M31Zero, MulByM31Trait};
use crate::fields::qm31::{PackedUnreducedQM31, PackedUnreducedQM31Trait, QM31, QM31Trait};
use crate::poly::circle::{CanonicCosetImpl, CircleDomainImpl, CircleEvaluationImpl};
use crate::utils::{
    ArrayImpl as ArrayUtilImpl, ColumnsIndicesPerTreeByLogDegreeBound, SpanImpl, bit_reverse_index,
    pack_qm31,
};
use crate::{ColumnSpan, TreeArray, TreeSpan};


#[cfg(test)]
mod test;

/// An OOD sample
#[derive(Copy, Debug, Drop)]
pub struct PointSample {
    pub point: CirclePoint<QM31>,
    pub value: QM31,
}

/// Computes the OOD quotients at the query positions.
///
/// # Arguments
///
/// * `log_size_per_column`: The log size of the commitment domain for each column.
/// * `samples_per_column`: OOD samples (i.e. point and eval) for each column.
/// * `random_coeff`: Verifier randomness for folding multiple columns quotients together.
/// * `query_positions_per_log_size`: Query positions mapped by log commitment domain size.
/// * `queried_values`: Evals of each column at the columns corresponding query positions.
pub fn fri_answers(
    mut column_indices_per_tree_by_degree_bound: ColumnsIndicesPerTreeByLogDegreeBound,
    log_blowup_factor: u32,
    samples_per_column_per_tree: TreeSpan<ColumnSpan<Array<PointSample>>>,
    random_coeff: QM31,
    mut query_positions_per_log_size: Felt252Dict<Nullable<Span<usize>>>,
    mut queried_values: TreeSpan<Span<M31>>,
) -> Array<Span<QM31>> {
    let mut log_size = column_indices_per_tree_by_degree_bound.len() + log_blowup_factor;
    // Check that the largest log size of a trace column is <= `M31_CIRCLE_LOG_ORDER` - 1.
    // Note that `log_size` is equal to 1 + largest log size of a trace column (the additional 1
    // comes from calling `len()` on `column_indices_per_tree_by_degree_bound`).
    assert!(log_size <= M31_CIRCLE_LOG_ORDER, "log_size is too large");

    let mut answers = array![];
    while let Some(columns_per_tree) = column_indices_per_tree_by_degree_bound.pop_back() {
        log_size = log_size - 1;

        // Collect the column samples and the number of columns in each tree.
        let mut samples = array![];
        let mut n_columns_per_tree = array![];
        for (columns, samples_per_column) in zip_eq(columns_per_tree, samples_per_column_per_tree) {
            for column in columns {
                samples.append(samples_per_column[*column]);
            }
            n_columns_per_tree.append(columns.len());
        }

        if samples.is_empty() {
            continue;
        }

        answers
            .append(
                fri_answers_for_log_size(
                    log_size,
                    samples,
                    random_coeff,
                    query_positions_per_log_size.get(log_size.into()).deref(),
                    ref queried_values,
                    n_columns_per_tree,
                ),
            );
    }

    answers
}

fn fri_answers_for_log_size(
    log_size: u32,
    samples_per_column: Array<@Array<PointSample>>,
    random_coeff: QM31,
    mut query_positions: Span<usize>,
    ref queried_values: TreeSpan<Span<M31>>,
    n_columns_per_tree: TreeArray<usize>,
) -> Span<QM31> {
    let sample_batches_by_point = ColumnSampleBatchImpl::group_by_point(samples_per_column);
    let quotient_constants = QuotientConstantsImpl::gen(@sample_batches_by_point, random_coeff);
    let commitment_domain = CanonicCosetImpl::new(log_size).circle_domain();
    let mut quotient_evals_at_queries = array![];

    for query_position in query_positions {
        let queried_values_at_row = tree_take_n(ref queried_values, n_columns_per_tree.span());
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
            alpha_mul_a_sum, alpha_mul_b_sum, indexed_alpha_mul_c, batch_random_coeff,
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
        quotient_accumulator =
            QM31Trait::fused_mul_sub(quotient_accumulator, *batch_random_coeff, minus_quotient);
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
        // For a sample point `P: CirclePoint<QM31>` compute its real part Pr and its imaginary part
        // Pi, both of type `CirclePoint<CM31>` (i.e. real and imaginary parts are with respect to
        // CM31).
        let [a, b, c, d] = sample_batch.point.x.to_fixed_array();
        let prx = CM31Trait::pack(a.into(), b.into());
        let pix = CM31Trait::pack(c.into(), d.into());
        let [a, b, c, d] = sample_batch.point.y.to_fixed_array();
        let pry = CM31Trait::pack(a.into(), b.into());
        let piy = CM31Trait::pack(c.into(), d.into());
        denominators.append(prx.sub_m31(domain_point.x) * piy - pry.sub_m31(domain_point.y) * pix);
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

/// Holds constants associated with a batch of samples for a given evaluation point and domain size.
///
/// # Overview
/// To prove that `F(p) = value`, we apply *two-point quotienting* at `point`
/// and its conjugate `conj(p)`. The numerator of the quotient is:
///
///     c * F(q) - a * q.y - b
///
/// where `(a, b, c)` are the coefficients of the line passing through `(p.y, value)` and
/// `(conj(p.y), conj(value))`, ensuring that the numerator evaluates to zero at p and conj(p).
///
/// Note: since `F` is a polynomial defined over the base field, we have
///     F(conj(point)) = conj(F(point))
///
/// # Batched Evaluation Proofs
/// In batched evaluation proof verification, the verifier computes a pseudo-random
/// linear combination of these quotients:
///
///     Σ (α^(i+1) * (c_i * F_i(q) - a_i * q.y - b_i))
///
/// which expands to:
///
///     Σ (α^(i+1) * c_i * F_i(q)) - q.y * Σ (α^(i+1) * a_i) - Σ (α^(i+1) * b_i)
///
/// To evaluate this expression efficiently at the query point `q`, we store:
///
/// - `alpha_mul_a_sum`: Σ (α^(i+1) * a_i) across all samples.
/// - `alpha_mul_b_sum`: Σ (α^(i+1) * b_i) across all samples.
/// - `indexed_alpha_mul_c`: Pairs `(column index, α^(i+1) * c_i)` for each sample.
/// - `batch_random_coeff`: The pseudo-random coefficient used to generate the linear combination.
///   For each batch, we exponentiate it by the number of columns (`α^(#columns)`)
///   to merge multiple batches consistently.
///
#[derive(Debug, Drop)]
pub struct PointQuotientConstants {
    /// Σ (α^(i+1) * a_i) across all samples in the batch.
    pub alpha_mul_a_sum: PackedUnreducedQM31,
    /// Σ (α^(i+1) * b_i) across all samples in the batch.
    pub alpha_mul_b_sum: PackedUnreducedQM31,
    /// Pairs of `(column index, α^i * c_i)` for every sample.
    pub indexed_alpha_mul_c: Array<(usize, PackedUnreducedQM31)>,
    /// The random coefficient `α^(#columns)` used in the linear combination above.
    pub batch_random_coeff: QM31,
}

#[generate_trait]
impl QuotientConstantsImpl of QuotientConstantsTrait {
    fn gen(
        sample_batches_by_point: @Array<ColumnSampleBatch>, random_coeff: QM31,
    ) -> QuotientConstants {
        let mut point_constants = array![];

        for sample_batch in sample_batches_by_point.span() {
            // TODO(ShaharS): Add salt. This assertion will fail at a probability of 1 to 2^62.
            // Use a better solution.
            assert!(
                *sample_batch.point.y != (*sample_batch.point.y).complex_conjugate(),
                "Cannot evaluate a line with a single point ({:?}).",
                sample_batch.point,
            );

            let neg_dbl_im_py = neg_twice_imaginary_part(sample_batch.point.y);

            let mut alpha: QM31 = One::one();
            let mut alpha_mul_a_sum = PackedUnreducedQM31Trait::large_zero();
            let mut alpha_mul_b_sum = PackedUnreducedQM31Trait::large_zero();
            let mut indexed_alpha_mul_c: Array<(usize, PackedUnreducedQM31)> = array![];

            for (column_idx, column_value) in sample_batch.columns_and_values.span() {
                alpha = alpha * random_coeff;
                let alpha_mul_a = alpha * neg_twice_imaginary_part(column_value);
                let alpha_mul_c = alpha * neg_dbl_im_py;
                let alpha_mul_b = QM31Trait::fused_mul_sub(
                    *column_value, alpha_mul_c, alpha_mul_a * *sample_batch.point.y,
                );
                alpha_mul_a_sum += alpha_mul_a.into();
                alpha_mul_b_sum += alpha_mul_b.into();
                indexed_alpha_mul_c.append((*column_idx, alpha_mul_c.into()));
            }

            point_constants
                .append(
                    PointQuotientConstants {
                        alpha_mul_a_sum: alpha_mul_a_sum.reduce().into(),
                        alpha_mul_b_sum: alpha_mul_b_sum,
                        indexed_alpha_mul_c,
                        batch_random_coeff: alpha,
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

#[generate_trait]
impl ColumnSampleBatchImpl of ColumnSampleBatchTrait {
    /// Groups all column samples by sampled point.
    ///
    /// `samples_per_column[i]` represents all point samples for column `i`.
    ///
    /// The ordering of the groups is dictated by the ordering of the samples. Therefore the prover
    /// and verifier must agree on the ordering of columns and points (offsets) for each column.
    /// NOTE: PrefixSum columns (LogUp) are sampled at a single point `Z` and a single offset `-1`
    /// -> `Z-g`. the current ordering for these columns is: [Z-g, Z].
    fn group_by_point(samples_per_column: Array<@Array<PointSample>>) -> Array<ColumnSampleBatch> {
        // Samples grouped by point.
        let mut grouped_samples: Felt252Dict<Nullable<Array<(usize, QM31)>>> = Default::default();
        let mut point_set: Array<CirclePoint<QM31>> = array![];

        let mut column = 0_usize;

        for samples in samples_per_column {
            // TODO(andrew): Almost all columns have a single sample at the OODS point.
            // Handling this case specifically is more optimal than using the dictionary.
            for sample in samples.span() {
                let point_key = CirclePointQM31Key::encode(sample.point);
                let (entry, value) = grouped_samples.entry(point_key);

                let mut point_samples = match match_nullable(value) {
                    FromNullableResult::Null => {
                        // This is the first time we've seen this point, add it to the point set.
                        point_set.append(*sample.point);
                        array![]
                    },
                    FromNullableResult::NotNull(value) => value.unbox(),
                };
                point_samples.append((column, *sample.value));
                grouped_samples = entry.finalize(NullableTrait::new(point_samples));
            }

            column += 1;
        }

        let mut groups = array![];

        for point in point_set {
            let point_key = CirclePointQM31Key::encode(@point);
            let (entry, columns_and_values) = grouped_samples.entry(point_key);
            let columns_and_values = columns_and_values.deref();

            grouped_samples = entry.finalize(null());
            groups.append(ColumnSampleBatch { point, columns_and_values });
        }

        groups
    }
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
        let [y_identifier, _, _, _] = key.y.to_fixed_array();
        pack_qm31(y_identifier.into(), *key.x)
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
