use core::array::ArrayImpl;
use core::dict::{Felt252Dict, Felt252DictEntryTrait, SquashedFelt252DictTrait};
use core::iter::{IntoIterator, Iterator};
use core::nullable::{Nullable, NullableTrait, null};
use core::num::traits::{One, Zero};
use crate::circle::{CirclePoint, CirclePointIndexImpl, CosetImpl, M31_CIRCLE_LOG_ORDER};
use crate::fields::BatchInvertible;
use crate::fields::cm31::{CM31, CM31Trait};
use crate::fields::m31::{M31, M31Zero, UnreducedM31};
use crate::fields::qm31::{PackedUnreducedQM31, PackedUnreducedQM31Trait, QM31, QM31Trait};
use crate::poly::circle::{CanonicCosetImpl, CircleDomainImpl, CircleEvaluationImpl};
use crate::utils::{
    ArrayImpl as ArrayUtilImpl, ColumnsByLogSize, SpanImpl, bit_reverse_index,
    group_column_trees_by_log_size, pack4,
};
use crate::verifier::VerificationError;
use crate::{ColumnSpan, TreeArray, TreeSpan};


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
    mut log_size_per_column: TreeSpan<@Array<u32>>,
    samples_per_column: ColumnSpan<Array<PointSample>>,
    random_coeff: QM31,
    mut query_positions_per_log_size: Felt252Dict<Nullable<Span<usize>>>,
    mut queried_values: TreeArray<Span<M31>>,
) -> Result<Array<Span<QM31>>, VerificationError> {
    let ColumnsByLogSize {
        mut columns_by_log_size_per_tree,
    } = group_column_trees_by_log_size(log_size_per_column);

    let mut answers = array![];
    for i in (0..M31_CIRCLE_LOG_ORDER) {
        let log_size = (M31_CIRCLE_LOG_ORDER - 1) - i;

        // Collect the column samples and the number of columns in each tree.
        let mut samples = array![];
        let mut n_columns_per_tree = array![];

        // In each iteration we pop the the columns corresponding to `log_size` from each tree, so
        // we need to prepare `columns_by_log_size_per_tree` for the next iteration.
        let mut next_columns_by_log_size_per_tree = array![];
        for columns_by_log_size in columns_by_log_size_per_tree {
            let mut columns_by_log_size = *columns_by_log_size;
            let columns = columns_by_log_size.pop_back().unwrap();
            next_columns_by_log_size_per_tree.append(columns_by_log_size);

            for column in columns {
                samples.append(samples_per_column[*column]);
            }
            n_columns_per_tree.append(columns.len());
        }
        columns_by_log_size_per_tree = next_columns_by_log_size_per_tree.span();

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
                )
                    .expect('fri_answers_for_log_size'),
            );
    }

    Ok(answers)
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
) -> Result<Span<QM31>, VerificationError> {
    let sample_batches = ColumnSampleBatchImpl::group_by_point(samples_per_column);
    let quotient_constants = QuotientConstantsImpl::gen(@sample_batches, random_coeff);
    let commitment_domain = CanonicCosetImpl::new(log_size).circle_domain();
    let mut quotient_evals_at_queries = array![];

    for query_position in query_positions {
        let queried_values_at_row = tree_take_n(ref queried_values, n_columns.span());
        quotient_evals_at_queries
            .append(
                accumulate_row_quotients(
                    @sample_batches,
                    queried_values_at_row.span(),
                    @quotient_constants,
                    commitment_domain.at(bit_reverse_index(*query_position, log_size)),
                ),
            );
    }

    Ok(quotient_evals_at_queries.span())
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
    sample_batches: @Array<ColumnSampleBatch>,
    queried_values_at_row: Span<M31>,
    quotient_constants: @QuotientConstants,
    domain_point: CirclePoint<M31>,
) -> QM31 {
    let n_batches = sample_batches.len();
    // TODO(andrew): Unnessesary asserts, remove.
    assert!(n_batches == quotient_constants.line_coeffs.len());
    assert!(n_batches == quotient_constants.batch_random_coeffs.len());

    let denominator_inverses = quotient_denominator_inverses(sample_batches.span(), domain_point);
    let domain_point_y: UnreducedM31 = domain_point.y.into();

    let mut quotient_accumulator: QM31 = Zero::zero();

    for batch_i in 0..n_batches {
        let line_coeffs = quotient_constants.line_coeffs[batch_i];
        let sample_batch_columns_and_values = sample_batches[batch_i].columns_and_values;
        let batch_size = sample_batch_columns_and_values.len();
        assert!(batch_size == line_coeffs.len());
        let mut numerator: PackedUnreducedQM31 = PackedUnreducedQM31Trait::large_zero();

        for sample_i in 0..batch_size {
            let (column_index, _) = sample_batch_columns_and_values[sample_i];
            let query_eval_at_column = *queried_values_at_row.at(*column_index);

            let ComplexConjugateLineCoeffs {
                alpha_mul_a, alpha_mul_b, alpha_mul_c,
            } = *line_coeffs[sample_i];
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

        let batch_coeff = *quotient_constants.batch_random_coeffs[batch_i];
        let denom_inv = *denominator_inverses[batch_i];
        let quotient = numerator.reduce().mul_cm31(denom_inv);
        quotient_accumulator =
            QM31Trait::fused_mul_add(quotient_accumulator, batch_coeff, quotient);
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

/// Holds the precomputed constant values used in each quotient evaluation.
#[derive(Debug, Drop)]
pub struct QuotientConstants {
    /// The line coefficients for each quotient numerator term.
    pub line_coeffs: Array<Array<ComplexConjugateLineCoeffs>>,
    /// The random coefficients used to linearly combine the batched quotients.
    ///
    /// For each sample batch we compute random_coeff^(number of columns in the batch),
    /// which is used to linearly combine multiple batches together.
    pub batch_random_coeffs: Array<QM31>,
}

#[generate_trait]
impl QuotientConstantsImpl of QuotientConstantsTrait {
    fn gen(sample_batches: @Array<ColumnSampleBatch>, random_coeff: QM31) -> QuotientConstants {
        let mut line_coeffs = array![];
        let mut batch_random_coeffs = array![];

        for sample_batch in sample_batches.span() {
            // TODO(ShaharS): Add salt. This assertion will fail at a probability of 1 to 2^62.
            // Use a better solution.
            assert!(
                *sample_batch.point.y != (*sample_batch.point.y).complex_conjugate(),
                "Cannot evaluate a line with a single point ({:?}).",
                sample_batch.point,
            );

            let mut alpha: QM31 = One::one();
            let mut batch_line_coeffs = array![];

            for (_, column_value) in sample_batch.columns_and_values.span() {
                alpha = alpha * random_coeff;
                batch_line_coeffs
                    .append(
                        ComplexConjugateLineCoeffsImpl::new(
                            sample_batch.point, **column_value, alpha,
                        ),
                    );
            }

            batch_random_coeffs.append(alpha);
            line_coeffs.append(batch_line_coeffs);
        }

        QuotientConstants { line_coeffs, batch_random_coeffs }
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
                let (entry, point_samples) = grouped_samples.entry(point_key);

                // Check if we've seen the point before.
                if point_samples.is_null() {
                    point_set.append(*sample.point);
                }

                let mut point_samples = point_samples.deref_or(array![]);
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

#[cfg(test)]
mod tests {
    use core::array::ArrayImpl;
    use core::dict::Felt252Dict;
    use core::nullable::NullableTrait;
    use crate::circle::{CirclePoint, CirclePointIndexImpl, CosetImpl};
    use crate::fields::m31::m31;
    use crate::fields::qm31::{PackedUnreducedQM31Trait, QM31, qm31_const};
    use crate::poly::circle::{CanonicCosetImpl, CircleDomainImpl, CircleEvaluationImpl};
    use crate::utils::DictImpl;
    use super::{
        ColumnSampleBatch, ColumnSampleBatchImpl, ComplexConjugateLineCoeffsImpl, PointSample,
        QuotientConstantsImpl, accumulate_row_quotients, fri_answers, fri_answers_for_log_size,
    };

    #[test]
    fn test_fri_answers_for_log_size() {
        let log_size = 5;
        let p0 = qm31_circle_gen();
        let p1 = p0 + qm31_circle_gen();
        let p2 = p1 + qm31_circle_gen();
        let sample0 = PointSample { point: p0, value: qm31_const::<0, 1, 2, 3>() };
        let sample1 = PointSample { point: p1, value: qm31_const::<1, 2, 3, 4>() };
        let sample2 = PointSample { point: p2, value: qm31_const::<2, 3, 4, 5>() };
        let col0_samples = array![sample0, sample1, sample2];
        let col1_samples = array![sample0];
        let col2_samples = array![sample0, sample2];
        let samples_by_column = array![@col0_samples, @col1_samples, @col2_samples];
        let random_coeff = qm31_const::<9, 8, 7, 6>();
        let query_positions = array![4, 5, 6, 7].span();
        let col0_query_values = array![m31(1), m31(2), m31(3), m31(4)].span();
        let col1_query_values = array![m31(1), m31(1), m31(2), m31(3)].span();
        let col2_query_values = array![m31(1), m31(1), m31(1), m31(2)].span();
        let mut query_evals = array![col0_query_values, col1_query_values, col2_query_values];
        let n_columns = array![1, 1, 1];

        let res = fri_answers_for_log_size(
            log_size, samples_by_column, random_coeff, query_positions, ref query_evals, n_columns,
        )
            .unwrap();

        assert!(
            res == array![
                qm31_const::<1791980583, 1709376644, 1911116353, 1204412580>(),
                qm31_const::<1417689272, 1640898968, 1760036812, 1705156550>(),
                qm31_const::<503725777, 621939055, 1324380556, 1450763049>(),
                qm31_const::<1895961752, 170000503, 1562444038, 1465755799>(),
            ]
                .span(),
        );
    }

    #[test]
    fn test_fri_answers() {
        let col0_log_size = 5;
        let col1_log_size = 7;
        let log_size_tree = array![@array![], @array![], @array![col0_log_size, col1_log_size]];
        let p0 = qm31_circle_gen();
        let p1 = qm31_circle_gen() + qm31_circle_gen();
        let sample0 = PointSample { point: p0, value: qm31_const::<0, 1, 2, 3>() };
        let sample1 = PointSample { point: p1, value: qm31_const::<1, 2, 3, 4>() };
        let col0_samples = array![sample0, sample1];
        let col1_samples = array![sample0];
        let samples_per_column = array![col0_samples, col1_samples];
        let random_coeff = qm31_const::<9, 8, 7, 6>();
        let col0_query_positions = array![4, 5].span();
        let col1_query_positions = array![6, 7].span();
        let mut query_domain_per_log_size: Felt252Dict = Default::default();
        query_domain_per_log_size.insert(5, NullableTrait::new(col0_query_positions));
        query_domain_per_log_size.replace(7, NullableTrait::new(col1_query_positions));

        let query_evals = array![
            array![].span(), array![].span(), array![m31(3), m31(7), m31(9), m31(2)].span(),
        ];

        let res = fri_answers(
            log_size_tree.span(),
            samples_per_column.span(),
            random_coeff,
            query_domain_per_log_size,
            query_evals,
        )
            .unwrap();

        assert!(
            res == array![
                array![
                    qm31_const::<1791306293, 1053124067, 158259497, 452720916>(),
                    qm31_const::<212478330, 1383090185, 1622369493, 599681801>(),
                ]
                    .span(),
                array![
                    qm31_const::<834593128, 54438530, 120431711, 2027138945>(),
                    qm31_const::<1820575540, 1615656673, 695030281, 674192396>(),
                ]
                    .span(),
            ],
        );
    }

    #[test]
    fn test_complex_conjugate_line_coeffs_impl() {
        let point = qm31_circle_gen();
        let value = qm31_const::<9, 8, 7, 6>();
        let alpha = qm31_const::<2, 3, 4, 5>();

        let res = ComplexConjugateLineCoeffsImpl::new(@point, value, alpha);

        assert!(res.alpha_mul_a.reduce() == qm31_const::<126, 2147483415, 8, 2147483581>());
        assert!(
            res.alpha_mul_b.reduce() == qm31_const::<20238140, 1378415613, 17263450, 142791233>(),
        );
        assert!(
            res.alpha_mul_c.reduce() == qm31_const::<865924731, 72415967, 2011255989, 1549931113>(),
        );
    }

    #[test]
    fn test_column_sample_batch_group_by_point() {
        let p0 = qm31_circle_gen();
        let p1 = p0 + qm31_circle_gen();
        let p2 = p1 + qm31_circle_gen();
        let sample0 = PointSample { point: p0, value: qm31_const::<0, 1, 2, 3>() };
        let sample1 = PointSample { point: p1, value: qm31_const::<1, 2, 3, 4>() };
        let sample2 = PointSample { point: p2, value: qm31_const::<2, 3, 4, 5>() };
        let col0_samples = array![sample0, sample1, sample2];
        let col1_samples = array![sample0];
        let col2_samples = array![sample0, sample2];
        let samples_per_column = array![@col0_samples, @col1_samples, @col2_samples];

        let grouped_samples = ColumnSampleBatchImpl::group_by_point(samples_per_column);

        assert!(
            grouped_samples == array![
                ColumnSampleBatch {
                    point: sample0.point,
                    columns_and_values: array![
                        (0, @sample0.value), (1, @sample0.value), (2, @sample0.value),
                    ],
                },
                ColumnSampleBatch {
                    point: sample1.point, columns_and_values: array![(0, @sample1.value)],
                },
                ColumnSampleBatch {
                    point: sample2.point,
                    columns_and_values: array![(0, @sample2.value), (2, @sample2.value)],
                },
            ],
        )
    }

    #[test]
    fn test_accumulate_row_quotients() {
        let alpha = qm31_const::<4, 3, 2, 1>();
        let domain = CircleDomainImpl::new(CosetImpl::new(CirclePointIndexImpl::new(1), 0));
        let queried_values_at_row = array![m31(5), m31(1)].span();
        let p0 = qm31_circle_gen();
        let p1 = qm31_circle_gen() + qm31_circle_gen();
        let sample_batches = array![
            ColumnSampleBatch {
                point: p0, columns_and_values: array![(0, @qm31_const::<0, 1, 2, 3>())],
            },
            ColumnSampleBatch {
                point: p1, columns_and_values: array![(1, @qm31_const::<1, 2, 3, 4>())],
            },
        ];
        let quotient_constants = QuotientConstantsImpl::gen(@sample_batches, alpha);

        let res = accumulate_row_quotients(
            @sample_batches, queried_values_at_row, @quotient_constants, domain.at(0),
        );

        assert_eq!(res, qm31_const::<545815778, 838613809, 1761463254, 2019099482>());
    }

    // Test used to benchmark step counts.
    #[test]
    fn test_fri_answers_with_1000_columns() {
        // NOTE: Forge fails if these are declated `const ...`.
        let log_size: u32 = 16;
        let n_queries: usize = 20;
        let n_columns: usize = 1000;
        let random_coeff = qm31_const::<9, 8, 7, 6>();
        assert!(n_columns >= 3, "First three columns are manually created");
        let mut query_positions = array![];
        for query_position in 0..n_queries {
            query_positions.append(query_position);
        }
        let p0 = qm31_circle_gen();
        let p1 = p0 + qm31_circle_gen();
        let p2 = p1 + qm31_circle_gen();
        let sample0 = PointSample { point: p0, value: qm31_const::<0, 1, 2, 3>() };
        let sample1 = PointSample { point: p1, value: qm31_const::<1, 2, 3, 4>() };
        let sample2 = PointSample { point: p2, value: qm31_const::<2, 3, 4, 5>() };
        let col0_samples = array![sample0, sample1, sample2];
        let col1_samples = array![sample0];
        let col2_samples = array![sample0, sample2];
        let mut samples_per_column = array![@col0_samples, @col1_samples, @col2_samples];
        let mut query_values = array![];

        for i in 0..n_queries {
            for _ in 0..n_columns {
                query_values.append(m31(i));
            }
        }
        for _ in samples_per_column.len()..n_columns {
            samples_per_column.append(@col1_samples);
        }

        let n_columns = array![0, n_columns, 0];
        let mut query_evals = array![array![].span(), query_values.span(), array![].span()];

        let _res = fri_answers_for_log_size(
            log_size,
            samples_per_column,
            random_coeff,
            query_positions.span(),
            ref query_evals,
            n_columns,
        );
    }

    /// Returns a generator for the circle group over [`QM31`].
    fn qm31_circle_gen() -> CirclePoint<QM31> {
        let x = qm31_const::<0x1, 0x0, 0x1C876E93, 0x1E9CA77B>();
        let y = qm31_const::<0x3B25121B, 0x26B12487, 0x2C1E6D83, 0x46B9D720>();
        CirclePoint { x, y }
    }
}
