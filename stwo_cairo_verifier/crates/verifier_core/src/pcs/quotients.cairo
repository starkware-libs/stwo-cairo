use bounded_int::BoundedInt;
use core::array::ArrayImpl;
use core::dict::{Felt252Dict, Felt252DictEntryTrait, SquashedFelt252DictTrait};
use core::iter::{IntoIterator, Iterator};
use core::nullable::{FromNullableResult, Nullable, NullableTrait, match_nullable, null};
use core::num::traits::{One, Zero};
use crate::circle::{CirclePoint, CirclePointIndexImpl, CosetImpl, M31_CIRCLE_LOG_ORDER};
use crate::fields::BatchInvertible;
use crate::fields::cm31::{CM31, CM31Trait};
use crate::fields::m31::{M31, M31Zero, UnreducedM31};
use crate::fields::qm31::{PackedUnreducedQM31, PackedUnreducedQM31Trait, QM31, QM31Trait};
use crate::poly::circle::{CanonicCosetImpl, CircleDomainImpl, CircleEvaluationImpl};
use crate::utils::{ArrayImpl as ArrayUtilImpl, SpanImpl, bit_reverse_index, pack4};
use crate::{ColumnSpan, TreeArray, TreeSpan};

#[cfg(test)]
mod test;

/// Pads all the trees in `columns_by_log_size_per_tree` to the length of the longest tree
/// and transposes the arrays from [tree][log_size][column] to [log_size][tree][column].
///
/// # Arguments
///
/// * `columns_by_log_size_per_tree`: The columns by log size per tree.
///
/// # Returns
///
/// * `columns_per_tree_by_log_size`: The columns per tree by log size.
fn pad_and_transpose_columns_by_log_size_per_tree(
    mut columns_by_log_size_per_tree: TreeSpan<Span<Span<usize>>>,
) -> Array<TreeArray<Span<usize>>> {
    let mut empty_span = array![].span();
    let mut columns_per_tree_by_log_size = array![];

    loop {
        // In each iteration we pop the the columns corresponding to `log_size` from each tree, so
        // we need to prepare `columns_by_log_size_per_tree` for the next iteration.
        let mut next_columns_by_log_size_per_tree = array![];

        let mut done = true;
        let mut columns_per_tree = array![];
        for columns_by_log_size in columns_by_log_size_per_tree {
            let mut columns_by_log_size = *columns_by_log_size;
            columns_per_tree
                .append(
                    if let Some(columns) = columns_by_log_size.pop_front() {
                        done = false;
                        *columns
                    } else {
                        empty_span
                    },
                );
            next_columns_by_log_size_per_tree.append(columns_by_log_size);
        }

        if done {
            break;
        }

        columns_by_log_size_per_tree = next_columns_by_log_size_per_tree.span();
        columns_per_tree_by_log_size.append(columns_per_tree);
    }

    columns_per_tree_by_log_size
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
    mut columns_by_log_size_per_tree: TreeSpan<Span<Span<usize>>>,
    samples_per_column_per_tree: TreeSpan<ColumnSpan<Array<PointSample>>>,
    random_coeff: QM31,
    mut query_positions_per_log_size: Felt252Dict<Nullable<Span<usize>>>,
    mut queried_values: TreeArray<Span<M31>>,
) -> Array<Span<QM31>> {
    let columns_per_tree_by_log_size = pad_and_transpose_columns_by_log_size_per_tree(
        columns_by_log_size_per_tree,
    );

    let mut log_size = columns_per_tree_by_log_size.len();
    assert!(log_size <= M31_CIRCLE_LOG_ORDER, "log_size is too large");

    let mut columns_per_tree_by_log_size = columns_per_tree_by_log_size.span();
    let mut answers = array![];
    while let Some(columns_per_tree) = columns_per_tree_by_log_size.pop_back() {
        log_size = log_size - 1;

        // Collect the column samples and the number of columns in each tree.
        let mut samples = array![];
        let mut n_columns_per_tree = array![];
        for (columns, samples_per_column) in columns_per_tree
            .into_iter()
            .zip(samples_per_column_per_tree) {
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

/// Takes `n[i]` elements from the i'th `tree` and returns them as a single array.
fn tree_take_n<T, +Clone<T>, +Drop<T>>(
    ref tree: TreeArray<Span<T>>, mut n: TreeSpan<usize>,
) -> Array<T> {
    let mut res: Array<T> = array![];
    let mut new_tree = array![];
    for mut values in tree {
        res.append_span(values.pop_front_n(*n.pop_front().unwrap()));
        new_tree.append(values);
    }

    tree = new_tree;
    res
}

fn fri_answers_for_log_size(
    log_size: u32,
    samples_per_column: Array<@Array<PointSample>>,
    random_coeff: QM31,
    mut query_positions: Span<usize>,
    ref queried_values: TreeArray<Span<M31>>,
    n_columns: TreeArray<usize>,
) -> Span<QM31> {
    let sample_batches_by_point = ColumnSampleBatchImpl::group_by_point(samples_per_column);
    let quotient_constants = QuotientConstantsImpl::gen(@sample_batches_by_point, random_coeff);
    let commitment_domain = CanonicCosetImpl::new(log_size).circle_domain();
    let mut quotient_evals_at_queries = array![];

    for query_position in query_positions {
        let queried_values_at_row = tree_take_n(ref queried_values, n_columns.span());
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
/// * `domain_point`: The domain point the query corresponts to.
#[inline(always)]
fn accumulate_row_quotients(
    sample_batches_by_point: @Array<ColumnSampleBatch>,
    queried_values_at_row: Span<M31>,
    quotient_constants: @QuotientConstants,
    domain_point: CirclePoint<M31>,
) -> QM31 {
    let n_batches = sample_batches_by_point.len();
    // TODO(andrew): Unnessesary asserts, remove.
    assert!(n_batches == quotient_constants.point_constants.len());

    let denominator_inverses = quotient_denominator_inverses(
        sample_batches_by_point.span(), domain_point,
    );
    let domain_point_y: UnreducedM31 = domain_point.y.into();

    let mut quotient_accumulator: QM31 = Zero::zero();

    let mut denominator_inverses_iter = denominator_inverses.span().into_iter();

    for point_constants in quotient_constants.point_constants.span() {
        let mut numerator: PackedUnreducedQM31 = PackedUnreducedQM31Trait::large_zero();

        for (column_index, line_coeffs) in point_constants.indexed_line_coeffs.span() {
            let query_eval_at_column = *queried_values_at_row.at(*column_index);
            let ComplexConjugateLineCoeffs { alpha_mul_a, alpha_mul_b, alpha_mul_c } = *line_coeffs;

            // The numerator is a line equation passing through
            //   (sample_point.y, sample_value), (conj(sample_point), conj(sample_value))
            // evaluated at (domain_point.y, value).
            // When substituting a polynomial in this line equation, we get a polynomial
            // with a root at sample_point and conj(sample_point) if the original polynomial
            // had the values sample_value and conj(sample_value) at these points.
            // TODO(andrew): `alpha_mul_b` can be moved out of the loop.
            // TODO(andrew): The whole `linear_term` can be moved out of the loop.
            let linear_term = alpha_mul_a.mul_m31(domain_point_y) + alpha_mul_b;
            numerator += alpha_mul_c.mul_m31(query_eval_at_column.into()) - linear_term;
        }

        let denom_inv = *denominator_inverses_iter.next().unwrap();
        let quotient = numerator.reduce().mul_cm31(denom_inv);
        quotient_accumulator =
            QM31Trait::fused_mul_add(
                quotient_accumulator, *point_constants.batch_random_coeffs, quotient,
            );
    }

    quotient_accumulator
}

fn quotient_denominator_inverses(
    sample_batches: Span<ColumnSampleBatch>, domain_point: CirclePoint<M31>,
) -> Array<CM31> {
    let mut denominators = array![];

    for sample_batch in sample_batches {
        // Extract Pr, Pi.
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

/// Holds the precomputed constants for a given evaluation point.
#[derive(Debug, Drop)]
pub struct PointQuotientConstants {
    /// Pair of (column index, line coefficients) for every sample.
    pub indexed_line_coeffs: Array<(usize, ComplexConjugateLineCoeffs)>,
    /// The random coefficients used to linearly combine the batched quotients.
    ///
    /// For each sample batch we compute random_coeff^(number of columns in the batch),
    /// which is used to linearly combine multiple batches together.
    /// this is the coefficient for this batch of samples.
    pub batch_random_coeffs: QM31,
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

            let mut alpha: QM31 = One::one();
            let mut indexed_line_coeffs = array![];

            for (column_idx, column_value) in sample_batch.columns_and_values.span() {
                alpha = alpha * random_coeff;
                indexed_line_coeffs
                    .append(
                        (
                            *column_idx,
                            ComplexConjugateLineCoeffsImpl::new(
                                sample_batch.point, **column_value, alpha,
                            ),
                        ),
                    );
            }

            point_constants
                .append(PointQuotientConstants { indexed_line_coeffs, batch_random_coeffs: alpha });
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
    pub columns_and_values: Array<(usize, @QM31)>,
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
        let mut grouped_samples: Felt252Dict<Nullable<Array<(usize, @QM31)>>> = Default::default();
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
                point_samples.append((column, sample.value));
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

/// The coefficients of a line between a point and its complex conjugate. Specifically,
/// `a, b, and c, s.t. a*x + b - c*y = 0` for (x,y) being (sample.y, sample.value) and
/// (conj(sample.y), conj(sample.value)).
/// Relies on the fact that every polynomial F over the base
/// field holds: F(p*) == F(p)* (* being the complex conjugate).
#[derive(Copy, Debug, Drop)]
struct ComplexConjugateLineCoeffs {
    alpha_mul_a: PackedUnreducedQM31,
    alpha_mul_b: PackedUnreducedQM31,
    alpha_mul_c: PackedUnreducedQM31,
}

#[generate_trait]
impl ComplexConjugateLineCoeffsImpl of ComplexConjugateLineCoeffsTrait {
    fn new(
        sample_point: @CirclePoint<QM31>, sample_value: QM31, alpha: QM31,
    ) -> ComplexConjugateLineCoeffs {
        let alpha_mul_a = alpha * neg_twice_imaginary_part(@sample_value);
        let alpha_mul_c = alpha * neg_twice_imaginary_part(sample_point.y);
        let alpha_mul_b = QM31Trait::fused_mul_sub(
            sample_value, alpha_mul_c, alpha_mul_a * *sample_point.y,
        );

        // TODO(andrew): These alpha multiplications are expensive.
        // Think they can be saved and done all at once.
        ComplexConjugateLineCoeffs {
            alpha_mul_a: alpha_mul_a.into(),
            alpha_mul_b: alpha_mul_b.into(),
            alpha_mul_c: alpha_mul_c.into(),
        }
    }
}

/// Returns `complex_conjugate(v) - v`.
#[inline]
pub fn neg_twice_imaginary_part(v: @QM31) -> QM31 {
    let [_, _, c, d] = v.to_fixed_array();
    let v = QM31Trait::from_fixed_array([M31Zero::zero(), M31Zero::zero(), c, d]);
    -(v + v)
}

#[derive(Copy, Debug, Drop)]
pub struct PointSample {
    pub point: CirclePoint<QM31>,
    pub value: QM31,
}

/// A circle point encoding to index into [`Felt252Dict`].
#[generate_trait]
pub impl CirclePointQM31Key of CirclePointQM31KeyTrait {
    fn encode(key: @CirclePoint<QM31>) -> felt252 {
        let [y_identifier, _, _, _] = key.y.to_fixed_array();
        pack4(y_identifier.into(), (*key.x).to_fixed_array())
    }
}
