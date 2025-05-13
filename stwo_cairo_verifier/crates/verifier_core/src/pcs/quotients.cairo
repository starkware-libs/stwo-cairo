use core::array::ArrayImpl;
use core::dict::{Felt252Dict, Felt252DictEntryTrait};
use core::iter::{IntoIterator, Iterator};
use core::nullable::{Nullable, NullableTrait, null};
use core::num::traits::{One, Zero};
use crate::circle::{CirclePoint, CirclePointIndexImpl, CosetImpl, M31_CIRCLE_LOG_ORDER};
use crate::fields::BatchInvertible;
use crate::fields::cm31::{CM31, CM31Trait};
use crate::fields::m31::{M31, UnreducedM31};
use crate::fields::qm31::{PackedUnreducedQM31, PackedUnreducedQM31Trait, QM31, QM31Trait};
use crate::poly::circle::{CanonicCosetImpl, CircleDomainImpl, CircleEvaluationImpl};
use crate::utils::{ArrayImpl as ArrayUtilImpl, SpanImpl, bit_reverse_index, pack4};
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
    // Group columns by log size.
    // TODO(andrew): Refactor. When columns are in descending order this is not needed.
    let mut log_size_00_columns = array![];
    let mut log_size_01_columns = array![];
    let mut log_size_02_columns = array![];
    let mut log_size_03_columns = array![];
    let mut log_size_04_columns = array![];
    let mut log_size_05_columns = array![];
    let mut log_size_06_columns = array![];
    let mut log_size_07_columns = array![];
    let mut log_size_08_columns = array![];
    let mut log_size_09_columns = array![];
    let mut log_size_10_columns = array![];
    let mut log_size_11_columns = array![];
    let mut log_size_12_columns = array![];
    let mut log_size_13_columns = array![];
    let mut log_size_14_columns = array![];
    let mut log_size_15_columns = array![];
    let mut log_size_16_columns = array![];
    let mut log_size_17_columns = array![];
    let mut log_size_18_columns = array![];
    let mut log_size_19_columns = array![];
    let mut log_size_20_columns = array![];
    let mut log_size_21_columns = array![];
    let mut log_size_22_columns = array![];
    let mut log_size_23_columns = array![];
    let mut log_size_24_columns = array![];
    let mut log_size_25_columns = array![];
    let mut log_size_26_columns = array![];
    let mut log_size_27_columns = array![];
    let mut log_size_28_columns = array![];
    let mut log_size_29_columns = array![];
    let mut log_size_30_columns = array![];

    let mut n_columns_per_tree = array![];
    let mut column = 0;
    loop {
        let mut interaction_column_sizes = if let Some(interaction_column_sizes) =
            log_size_per_column
            .pop_front() {
            (*interaction_column_sizes).span()
        } else {
            break Ok(());
        };

        let mut res_dict = Default::default();
        let loop_res = loop {
            let column_log_size = if let Some(column_log_size) = interaction_column_sizes
                .pop_front() {
                column_log_size
            } else {
                break Ok(());
            };

            let (res_dict_entry, value) = res_dict.entry((*column_log_size).into());
            res_dict = res_dict_entry.finalize(NullableTrait::new(value.deref_or(0) + 1));

            // TODO(andrew): Order by most common for performance. i.e. check log size 16->26 first.
            match *column_log_size {
                00 => log_size_00_columns.append(column),
                01 => log_size_01_columns.append(column),
                02 => log_size_02_columns.append(column),
                03 => log_size_03_columns.append(column),
                04 => log_size_04_columns.append(column),
                05 => log_size_05_columns.append(column),
                06 => log_size_06_columns.append(column),
                07 => log_size_07_columns.append(column),
                08 => log_size_08_columns.append(column),
                09 => log_size_09_columns.append(column),
                10 => log_size_10_columns.append(column),
                11 => log_size_11_columns.append(column),
                12 => log_size_12_columns.append(column),
                13 => log_size_13_columns.append(column),
                14 => log_size_14_columns.append(column),
                15 => log_size_15_columns.append(column),
                16 => log_size_16_columns.append(column),
                17 => log_size_17_columns.append(column),
                18 => log_size_18_columns.append(column),
                19 => log_size_19_columns.append(column),
                20 => log_size_20_columns.append(column),
                21 => log_size_21_columns.append(column),
                22 => log_size_22_columns.append(column),
                23 => log_size_23_columns.append(column),
                24 => log_size_24_columns.append(column),
                25 => log_size_25_columns.append(column),
                26 => log_size_26_columns.append(column),
                27 => log_size_27_columns.append(column),
                28 => log_size_28_columns.append(column),
                29 => log_size_29_columns.append(column),
                30 => log_size_30_columns.append(column),
                _ => { break Err(VerificationError::InvalidStructure('invalid size')); },
            }
            column += 1;
        };

        if loop_res.is_err() {
            break loop_res;
        }

        let mut n_columns_per_log_size: Array<usize> = array![];
        for log_size in (0..31_u32) {
            n_columns_per_log_size
                .append(res_dict.get(30_felt252 - log_size.into()).deref_or(0))
                .try_into()
                .unwrap();
        }
        n_columns_per_tree.append(n_columns_per_log_size.span());
    }?;

    let mut columns_per_log_size_rev = array![
        log_size_30_columns, log_size_29_columns, log_size_28_columns, log_size_27_columns,
        log_size_26_columns, log_size_25_columns, log_size_24_columns, log_size_23_columns,
        log_size_22_columns, log_size_21_columns, log_size_20_columns, log_size_19_columns,
        log_size_18_columns, log_size_17_columns, log_size_16_columns, log_size_15_columns,
        log_size_14_columns, log_size_13_columns, log_size_12_columns, log_size_11_columns,
        log_size_10_columns, log_size_09_columns, log_size_08_columns, log_size_07_columns,
        log_size_06_columns, log_size_05_columns, log_size_04_columns, log_size_03_columns,
        log_size_02_columns, log_size_01_columns, log_size_00_columns,
    ]
        .into_iter();

    let mut answers = array![];
    let mut log_size = M31_CIRCLE_LOG_ORDER;
    let mut one_per_tree = array![];
    for _ in 0..n_columns_per_tree.len() {
        one_per_tree.append(1);
    }
    loop {
        let columns = match columns_per_log_size_rev.next() {
            Some(columns) => columns,
            None => { break Ok(()); },
        };
        log_size -= 1;

        let n_columns = tree_take_n(ref n_columns_per_tree, one_per_tree.span());

        if columns.is_empty() {
            continue;
        }

        // Collect samples and queried values for the columns.
        let mut samples = array![];

        for column in columns {
            samples.append(samples_per_column[column]);
        }

        let answer = fri_answers_for_log_size(
            log_size,
            samples,
            random_coeff,
            query_positions_per_log_size.get(log_size.into()).deref(),
            ref queried_values,
            n_columns,
        );

        match answer {
            Ok(answer) => answers.append(answer),
            Err(err) => { break Err(err); },
        };
    }?;

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

pub fn fri_answers_for_log_size(
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
pub fn accumulate_row_quotients(
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
        quotient_accumulator = QM31Trait::fma(quotient_accumulator, batch_coeff, quotient);
    }

    quotient_accumulator
}

fn quotient_denominator_inverses(
    sample_batches: Span<ColumnSampleBatch>, domain_point: CirclePoint<M31>,
) -> Array<CM31> {
    let mut denominators = array![];

    for sample_batch in sample_batches {
        // Extract Pr, Pi.
        let [a, b, c, d] = sample_batch.point.x.to_array();
        let prx = CM31Trait::pack(a.into(), b.into());
        let pix = CM31Trait::pack(c.into(), d.into());
        let [a, b, c, d] = sample_batch.point.y.to_array();
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
pub impl QuotientConstantsImpl of QuotientConstantsTrait {
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
pub impl ColumnSampleBatchImpl of ColumnSampleBatchTrait {
    /// Groups all column samples by sampled point.
    ///
    /// `samples_per_column[i]` represents all point samples for column `i`.
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

        // TODO(andrew): Remove. Only sorting since rust verifier sorts groups by point.
        let sorted_points = point_set.sort_ascending();
        let mut groups = array![];

        for point in sorted_points {
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
pub impl ComplexConjugateLineCoeffsImpl of ComplexConjugateLineCoeffsTrait {
    fn new(
        sample_point: @CirclePoint<QM31>, sample_value: QM31, alpha: QM31,
    ) -> ComplexConjugateLineCoeffs {
        let alpha_mul_a = alpha * neg_twice_imaginary_part(@sample_value);
        let alpha_mul_c = alpha * neg_twice_imaginary_part(sample_point.y);
        let alpha_mul_b = QM31Trait::fms(sample_value, alpha_mul_c, alpha_mul_a * *sample_point.y);

        // TODO(andrew): These alpha multiplications are expensive.
        // Think they can be saved and done all at once.
        ComplexConjugateLineCoeffs {
            alpha_mul_a: alpha_mul_a.into(),
            alpha_mul_b: alpha_mul_b.into(),
            alpha_mul_c: alpha_mul_c.into(),
        }
    }
}
pub fn get_alpha_mul_a(self: ComplexConjugateLineCoeffs) -> QM31 {
    self.alpha_mul_a.reduce()
}
pub fn get_alpha_mul_b(self: ComplexConjugateLineCoeffs) -> QM31 {
    self.alpha_mul_b.reduce()
}
pub fn get_alpha_mul_c(self: ComplexConjugateLineCoeffs) -> QM31 {
    self.alpha_mul_c.reduce()
}

/// Returns `complex_conjugate(v) - v`.
#[inline]
pub fn neg_twice_imaginary_part(v: @QM31) -> QM31 {
    let [_, _, c, d] = v.to_array();
    let v = QM31Trait::from_array([0, 0, c, d]);
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
        let [y_identifier, _, _, _] = key.y.to_array();
        pack4(y_identifier.into(), (*key.x).to_array())
    }
}
