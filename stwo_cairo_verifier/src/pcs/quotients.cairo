use core::array::ArrayImpl;
use core::dict::{Felt252Dict, Felt252DictEntryTrait};
use core::iter::{IntoIterator, Iterator};
use core::nullable::{Nullable, NullableTrait, null};
use core::num::traits::{One, Zero};
use stwo_cairo_verifier::circle::{
    CosetImpl, CirclePointIndexImpl, CirclePoint, M31_CIRCLE_LOG_ORDER
};
use stwo_cairo_verifier::fields::FieldBatchInverse;
use stwo_cairo_verifier::fields::cm31::{CM31, CM31Impl};
use stwo_cairo_verifier::fields::m31::{M31, UnreducedM31};
use stwo_cairo_verifier::fields::qm31::{
    QM31, QM31Impl, PackedUnreducedQM31, PackedUnreducedQM31Impl
};
use stwo_cairo_verifier::poly::circle::{
    CircleEvaluationImpl, CircleDomain, CircleDomainImpl, SparseCircleEvaluation,
    SparseCircleEvaluationImpl, CanonicCosetImpl
};
use stwo_cairo_verifier::queries::{
    SparseSubCircleDomain, SparseSubCircleDomainImpl, SubCircleDomainImpl
};
use stwo_cairo_verifier::utils::{bit_reverse_index, pack4, ArrayImpl as ArrayUtilImpl};
use stwo_cairo_verifier::verifier::VerificationError;

pub fn fri_answers(
    log_size_per_column: @Array<u32>,
    samples_per_column: @Array<Array<PointSample>>,
    random_coeff: QM31,
    mut query_domain_per_log_size: Felt252Dict<Nullable<@SparseSubCircleDomain>>,
    queried_values_per_column: @Array<Array<M31>>,
) -> Result<Array<SparseCircleEvaluation>, VerificationError> {
    let n_columns = log_size_per_column.len();
    assert!(n_columns == samples_per_column.len());
    assert!(n_columns == queried_values_per_column.len());

    // Group columns by log size.
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

    let mut column = 0;
    loop {
        if column == n_columns {
            break Result::Ok(());
        }

        // TODO(andrew): Order by most common for performance. i.e. check log size 16->26 first.
        match *log_size_per_column[column] {
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
            _ => { break Result::Err(VerificationError::InvalidStructure('invalid size')); },
        }
        column += 1;
    }?;

    let mut columns_per_log_size_rev = array![
        log_size_30_columns,
        log_size_29_columns,
        log_size_28_columns,
        log_size_27_columns,
        log_size_26_columns,
        log_size_25_columns,
        log_size_24_columns,
        log_size_23_columns,
        log_size_22_columns,
        log_size_21_columns,
        log_size_20_columns,
        log_size_19_columns,
        log_size_18_columns,
        log_size_17_columns,
        log_size_16_columns,
        log_size_15_columns,
        log_size_14_columns,
        log_size_13_columns,
        log_size_12_columns,
        log_size_11_columns,
        log_size_10_columns,
        log_size_09_columns,
        log_size_08_columns,
        log_size_07_columns,
        log_size_06_columns,
        log_size_05_columns,
        log_size_04_columns,
        log_size_03_columns,
        log_size_02_columns,
        log_size_01_columns,
        log_size_00_columns,
    ]
        .into_iter();
    let mut answers = array![];

    let mut log_size = M31_CIRCLE_LOG_ORDER;
    loop {
        let columns = match columns_per_log_size_rev.next() {
            Option::Some(columns) => columns,
            Option::None => { break Result::Ok(()); },
        };
        log_size -= 1;

        if columns.is_empty() {
            continue;
        }

        // Collect samples and queried values for the columns.
        let mut samples = array![];
        let mut queried_values = array![];

        for column in columns {
            samples.append(samples_per_column[column]);
            queried_values.append(queried_values_per_column[column]);
        };

        let answer = fri_answers_for_log_size(
            log_size,
            samples,
            random_coeff,
            query_domain_per_log_size.get(log_size.into()).deref(),
            queried_values
        );

        match answer {
            Result::Ok(answer) => answers.append(answer),
            Result::Err(err) => { break Result::Err(err); },
        };
    }?;

    Result::Ok(answers)
}

fn fri_answers_for_log_size(
    log_size: u32,
    samples_per_column: Array<@Array<PointSample>>,
    random_coeff: QM31,
    query_domain: @SparseSubCircleDomain,
    queried_values_per_column: Array<@Array<M31>>,
) -> Result<SparseCircleEvaluation, VerificationError> {
    let query_domain_size = query_domain.size();
    let mut queried_values_per_column_iter = queried_values_per_column.span().into_iter();

    loop {
        // Check the queries column values have the correct size.
        if let Option::Some(column_queries) = queried_values_per_column_iter.next() {
            if (*column_queries).len() != query_domain_size {
                break Result::Err(VerificationError::InvalidStructure('queried vals'));
            }
        } else {
            break Result::Ok(());
        }
    }?;

    let commitment_domain = CanonicCosetImpl::new(log_size).circle_domain();
    let sample_batches = ColumnSampleBatchImpl::group_by_point(samples_per_column);
    let quotient_consts = QuotientConstantsImpl::gen(@sample_batches, random_coeff);
    let mut evals = array![];
    let mut column_eval_offset = 0;

    for subdomain in query_domain
        .domains
        .span() {
            let domain = subdomain.to_circle_domain(commitment_domain);
            let domain_size = domain.size();
            let domain_log_size = domain.log_size();
            let denominator_inverses = quotient_denominator_inverses(@sample_batches, domain);
            let mut values = array![];

            for row in 0
                ..domain_size {
                    let domain_point = domain.at(bit_reverse_index(row, domain_log_size));
                    let value = accumulate_row_quotients(
                        @sample_batches,
                        @queried_values_per_column,
                        @quotient_consts,
                        @denominator_inverses,
                        row,
                        domain_size,
                        column_eval_offset,
                        domain_point,
                    );
                    values.append(value);
                };

            // Progress the read offset.
            column_eval_offset += domain_size;

            let eval = CircleEvaluationImpl::new(domain, values);
            evals.append(eval);
        };

    assert!(column_eval_offset == query_domain_size);

    Result::Ok(SparseCircleEvaluationImpl::new(evals))
}

#[inline]
fn accumulate_row_quotients(
    sample_batches: @Array<ColumnSampleBatch>,
    queried_values_per_column: @Array<@Array<M31>>,
    quotient_constants: @QuotientConstants,
    denominator_inverses: @Array<CM31>,
    row: u32,
    domain_size: u32,
    queries_values_offset: u32,
    domain_point: CirclePoint<M31>,
) -> QM31 {
    let n_batches = sample_batches.len();
    // TODO(andrew): Unnessesary asserts, remove.
    assert!(n_batches == quotient_constants.line_coeffs.len());
    assert!(n_batches == quotient_constants.batch_random_coeffs.len());
    let column_row = queries_values_offset + row;
    let domain_point_y: UnreducedM31 = domain_point.y.into();

    let mut row_accumulator: QM31 = Zero::zero();

    for batch_i in 0
        ..n_batches {
            let line_coeffs = quotient_constants.line_coeffs[batch_i];
            let sample_batch_columns_and_values = sample_batches[batch_i].columns_and_values;
            let batch_size = sample_batch_columns_and_values.len();
            assert!(batch_size == line_coeffs.len());
            let mut numerator: PackedUnreducedQM31 = PackedUnreducedQM31Impl::large_zero();

            for sample_i in 0
                ..batch_size {
                    let (column_index, _) = sample_batch_columns_and_values[sample_i];
                    let column = *queried_values_per_column.at(*column_index);
                    let column_value = *column.at(column_row);
                    let ComplexConjugateLineCoeffs { alpha_mul_a, alpha_mul_b, alpha_mul_c } =
                        *line_coeffs[sample_i];
                    // The numerator is a line equation passing through
                    //   (sample_point.y, sample_value), (conj(sample_point), conj(sample_value))
                    // evaluated at (domain_point.y, value).
                    // When substituting a polynomial in this line equation, we get a polynomial
                    // with a root at sample_point and conj(sample_point) if the original polynomial
                    // had the values sample_value and conj(sample_value) at these points.
                    // TODO(andrew): `alpha_mul_b` can be moved out of the loop.
                    // TODO(andrew): The whole `linear_term` can be moved out of the loop.
                    let linear_term = alpha_mul_a.mul_m31(domain_point_y) + alpha_mul_b;
                    numerator += alpha_mul_c.mul_m31(column_value.into()) - linear_term;
                };

            let batch_coeff = *quotient_constants.batch_random_coeffs[batch_i];
            let denom_inv = *denominator_inverses[batch_i * domain_size + row];
            // Computes `row_accumulator * batch_coeff + numerator * denom_inv`.
            row_accumulator =
                QM31Impl::fma(row_accumulator, batch_coeff, numerator.reduce().mul_cm31(denom_inv));
        };

    row_accumulator
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

        for sample_batch in sample_batches
            .span() {
                // TODO(ShaharS): Add salt. This assertion will fail at a probability of 1 to 2^62.
                // Use a better solution.
                assert!(
                    *sample_batch.point.y != (*sample_batch.point.y).complex_conjugate(),
                    "Cannot evaluate a line with a single point ({:?}).",
                    sample_batch.point
                );

                let mut alpha: QM31 = One::one();
                let mut batch_line_coeffs = array![];

                for (_, column_value) in sample_batch
                    .columns_and_values
                    .span() {
                        alpha = alpha * random_coeff;
                        batch_line_coeffs
                            .append(
                                ComplexConjugateLineCoeffsImpl::new(
                                    sample_batch.point, **column_value, alpha
                                )
                            );
                    };

                batch_random_coeffs.append(alpha);
                line_coeffs.append(batch_line_coeffs);
            };

        QuotientConstants { line_coeffs, batch_random_coeffs }
    }
}

/// Returns a flattened 2d array (`denom[batch_index][domain_index]`).
///
/// Array is flattened to facilitate a batch inverse.
fn quotient_denominator_inverses(
    sample_batches: @Array<ColumnSampleBatch>, domain: CircleDomain,
) -> Array<CM31> {
    let mut domain_points_bit_rev = array![];

    for i in 0
        ..domain
            .size() {
                let i_bit_rev = bit_reverse_index(i, domain.log_size());
                domain_points_bit_rev.append(domain.at(i_bit_rev));
            };

    let mut flat_denominators = array![];

    // We want a `P` to be on a line that passes through a point `Pr + uPi` in `QM31^2`,
    // and its conjugate `Pr - uPi`. Thus, `Pr - P` is parallel to `Pi`. Or,
    // `(Pr - P).x * Pi.y - (Pr - P).y * Pi.x = 0`.
    for sample_batch in sample_batches
        .span() {
            // Extract `Pr, Pi`.
            let prx = *sample_batch.point.x.a;
            let pry = *sample_batch.point.y.a;
            let pix = *sample_batch.point.x.b;
            let piy = *sample_batch.point.y.b;

            for domain_point in domain_points_bit_rev
                .span() {
                    let denom = prx.sub_m31(*domain_point.x) * piy
                        - pry.sub_m31(*domain_point.y) * pix;
                    flat_denominators.append(denom);
                };
        };

    FieldBatchInverse::batch_inverse(flat_denominators)
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
    fn group_by_point(samples_per_column: Array<@Array<PointSample>>) -> Array<ColumnSampleBatch> {
        // Samples grouped by point.
        let mut grouped_samples: Felt252Dict<Nullable<Array<(usize, @QM31)>>> = Default::default();
        let mut point_set: Array<CirclePoint<QM31>> = array![];

        let mut column = 0_usize;

        for samples in samples_per_column {
            // TODO(andrew): Almost all columns have a single sample at the OODS point.
            // Handling this case specifically is more optimal than using the dictionary.
            for sample in samples
                .span() {
                    let point_key = CirclePointQM31Key::encode(sample.point);
                    let (entry, point_samples) = grouped_samples.entry(point_key);

                    // Check if we've seen the point before.
                    if point_samples.is_null() {
                        point_set.append(*sample.point);
                    };

                    let mut point_samples = point_samples.deref_or(array![]);
                    point_samples.append((column, sample.value));
                    grouped_samples = entry.finalize(NullableTrait::new(point_samples));
                };

            column += 1;
        };

        // TODO(andrew): Remove. Only sorting since rust verifier sorts groups by point.
        let sorted_points = point_set.sort_ascending();
        let mut groups = array![];

        for point in sorted_points {
            let point_key = CirclePointQM31Key::encode(@point);
            let (entry, columns_and_values) = grouped_samples.entry(point_key);
            let columns_and_values = columns_and_values.deref();
            grouped_samples = entry.finalize(null());
            groups.append(ColumnSampleBatch { point, columns_and_values });
        };

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
        sample_point: @CirclePoint<QM31>, sample_value: QM31, alpha: QM31
    ) -> ComplexConjugateLineCoeffs {
        let alpha_mul_a = alpha * neg_twice_imaginary_part(@sample_value);
        let alpha_mul_c = alpha * neg_twice_imaginary_part(sample_point.y);
        let alpha_mul_b = QM31Impl::fms(sample_value, alpha_mul_c, alpha_mul_a * *sample_point.y);

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
    let b = *v.b;
    QM31 { a: Zero::zero(), b: -(b + b) }
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
        let y_identifier = (*key.y.a.a.inner).into();
        pack4(y_identifier, (*key.x).to_array())
    }
}

#[cfg(test)]
mod tests {
    use core::array::ArrayImpl;
    use core::dict::Felt252Dict;
    use core::nullable::{NullableTrait};
    use stwo_cairo_verifier::circle::{QM31_CIRCLE_GEN, CosetImpl, CirclePointIndexImpl};
    use stwo_cairo_verifier::fields::cm31::cm31;
    use stwo_cairo_verifier::fields::m31::m31;
    use stwo_cairo_verifier::fields::qm31::{qm31, PackedUnreducedQM31Impl};
    use stwo_cairo_verifier::fri::CIRCLE_TO_LINE_FOLD_STEP;
    use stwo_cairo_verifier::poly::circle::{
        SparseCircleEvaluationImpl, CanonicCosetImpl, CircleDomainImpl, CircleEvaluationImpl
    };
    use stwo_cairo_verifier::queries::SubCircleDomainImpl;
    use stwo_cairo_verifier::queries::{SparseSubCircleDomain, SubCircleDomain};
    use stwo_cairo_verifier::utils::{pow, DictImpl};
    use super::{
        PointSample, fri_answers_for_log_size, ColumnSampleBatch, ColumnSampleBatchImpl,
        ComplexConjugateLineCoeffsImpl, quotient_denominator_inverses, accumulate_row_quotients,
        QuotientConstantsImpl, fri_answers
    };


    #[test]
    fn test_fri_answers_for_log_size() {
        let log_size = 5;
        let commitment_domain = CanonicCosetImpl::new(log_size).circle_domain();
        let p0 = QM31_CIRCLE_GEN;
        let p1 = p0 + QM31_CIRCLE_GEN;
        let p2 = p1 + QM31_CIRCLE_GEN;
        let sample0 = PointSample { point: p0, value: qm31(0, 1, 2, 3) };
        let sample1 = PointSample { point: p1, value: qm31(1, 2, 3, 4) };
        let sample2 = PointSample { point: p2, value: qm31(2, 3, 4, 5) };
        let col0_samples = array![sample0, sample1, sample2];
        let col1_samples = array![sample0];
        let col2_samples = array![sample0, sample2];
        let samples_per_column = array![@col0_samples, @col1_samples, @col2_samples];
        let random_coeff = qm31(9, 8, 7, 6);
        let sub_domain0 = SubCircleDomain { coset_index: 2, log_size: 1 };
        let sub_domain1 = SubCircleDomain { coset_index: 3, log_size: 1 };
        let query_domain = SparseSubCircleDomain {
            domains: array![sub_domain0, sub_domain1], large_domain_log_size: log_size
        };
        let col0_query_values = array![m31(1), m31(2), m31(3), m31(4)];
        let col1_query_values = array![m31(1), m31(1), m31(2), m31(3)];
        let col2_query_values = array![m31(1), m31(1), m31(1), m31(2)];
        let queried_values_per_column = array![
            @col0_query_values, @col1_query_values, @col2_query_values
        ];

        let res = fri_answers_for_log_size(
            log_size, samples_per_column, random_coeff, @query_domain, queried_values_per_column
        )
            .unwrap();

        assert!(
            res == SparseCircleEvaluationImpl::new(
                array![
                    CircleEvaluationImpl::new(
                        (@sub_domain0).to_circle_domain(commitment_domain),
                        array![
                            qm31(1655798290, 1221610097, 1389601557, 962654234),
                            qm31(638770057, 234503953, 730529691, 1759474677)
                        ]
                    ),
                    CircleEvaluationImpl::new(
                        (@sub_domain1).to_circle_domain(commitment_domain),
                        array![
                            qm31(812355951, 1467349841, 519312011, 1870584702),
                            qm31(1802072315, 1125204194, 422281582, 1308225981)
                        ]
                    ),
                ]
            )
        );
    }

    #[test]
    fn test_fri_answers() {
        let col0_log_size = 5;
        let col1_log_size = 7;
        let log_size_per_column = array![col0_log_size, col1_log_size];
        let col0_commitment_domain = CanonicCosetImpl::new(col0_log_size).circle_domain();
        let col1_commitment_domain = CanonicCosetImpl::new(col1_log_size).circle_domain();
        let p0 = QM31_CIRCLE_GEN;
        let p1 = QM31_CIRCLE_GEN + QM31_CIRCLE_GEN;
        let sample0 = PointSample { point: p0, value: qm31(0, 1, 2, 3) };
        let sample1 = PointSample { point: p1, value: qm31(1, 2, 3, 4) };
        let col0_samples = array![sample0, sample1];
        let col1_samples = array![sample0];
        let samples_per_column = array![col0_samples, col1_samples];
        let random_coeff = qm31(9, 8, 7, 6);
        let col0_sub_domain = SubCircleDomain { coset_index: 2, log_size: 1 };
        let col0_query_domain = SparseSubCircleDomain {
            domains: array![col0_sub_domain], large_domain_log_size: col0_log_size
        };
        let col1_sub_domain = SubCircleDomain { coset_index: 3, log_size: 1 };
        let col1_query_domain = SparseSubCircleDomain {
            domains: array![col1_sub_domain], large_domain_log_size: col1_log_size
        };
        let mut query_domain_per_log_size: Felt252Dict = Default::default();
        query_domain_per_log_size.insert(5, NullableTrait::new(@col0_query_domain));
        query_domain_per_log_size.replace(7, NullableTrait::new(@col1_query_domain));
        let col0_query_values = array![m31(9), m31(2)];
        let col1_query_values = array![m31(3), m31(7)];
        let queried_values_per_column = array![col0_query_values, col1_query_values];

        let res = fri_answers(
            @log_size_per_column,
            @samples_per_column,
            random_coeff,
            query_domain_per_log_size,
            @queried_values_per_column
        )
            .unwrap();

        assert!(
            res == array![
                SparseCircleEvaluationImpl::new(
                    array![
                        CircleEvaluationImpl::new(
                            (@col1_sub_domain).to_circle_domain(col1_commitment_domain),
                            array![
                                qm31(1791306293, 1053124067, 158259497, 452720916),
                                qm31(212478330, 1383090185, 1622369493, 599681801),
                            ]
                        ),
                    ]
                ),
                SparseCircleEvaluationImpl::new(
                    array![
                        CircleEvaluationImpl::new(
                            (@col0_sub_domain).to_circle_domain(col0_commitment_domain),
                            array![
                                qm31(834593128, 54438530, 120431711, 2027138945),
                                qm31(1820575540, 1615656673, 695030281, 674192396)
                            ]
                        ),
                    ]
                )
            ]
        );
    }

    #[test]
    fn test_complex_conjugate_line_coeffs_impl() {
        let point = QM31_CIRCLE_GEN;
        let value = qm31(9, 8, 7, 6);
        let alpha = qm31(2, 3, 4, 5);

        let res = ComplexConjugateLineCoeffsImpl::new(@point, value, alpha);

        assert!(res.alpha_mul_a.reduce() == qm31(126, 2147483415, 8, 2147483581));
        assert!(res.alpha_mul_b.reduce() == qm31(20238140, 1378415613, 17263450, 142791233));
        assert!(res.alpha_mul_c.reduce() == qm31(865924731, 72415967, 2011255989, 1549931113));
    }

    #[test]
    fn test_quotient_denominator_inverses() {
        let domain = CircleDomainImpl::new(CosetImpl::new(CirclePointIndexImpl::new(1), 0));
        let p0 = QM31_CIRCLE_GEN;
        let p1 = QM31_CIRCLE_GEN + QM31_CIRCLE_GEN;
        let samples = array![
            ColumnSampleBatch { point: p0, columns_and_values: array![] },
            ColumnSampleBatch { point: p1, columns_and_values: array![] },
        ];

        let denominator_inverses = quotient_denominator_inverses(@samples, domain);

        assert!(
            denominator_inverses == array![
                cm31(432303227, 706927115),
                cm31(1241984415, 2002674046),
                cm31(710698435, 1874662077),
                cm31(805681622, 1895046717)
            ]
        );
    }

    #[test]
    fn test_column_sample_batch_group_by_point() {
        let p0 = QM31_CIRCLE_GEN;
        let p1 = p0 + QM31_CIRCLE_GEN;
        let p2 = p1 + QM31_CIRCLE_GEN;
        let sample0 = PointSample { point: p0, value: qm31(0, 1, 2, 3) };
        let sample1 = PointSample { point: p1, value: qm31(1, 2, 3, 4) };
        let sample2 = PointSample { point: p2, value: qm31(2, 3, 4, 5) };
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
                        (0, @sample0.value), (1, @sample0.value), (2, @sample0.value)
                    ],
                },
                ColumnSampleBatch {
                    point: sample2.point,
                    columns_and_values: array![(0, @sample2.value), (2, @sample2.value)],
                },
                ColumnSampleBatch {
                    point: sample1.point, columns_and_values: array![(0, @sample1.value)],
                },
            ]
        )
    }

    #[test]
    fn test_accumulate_row_quotients() {
        let alpha = qm31(4, 3, 2, 1);
        let domain = CircleDomainImpl::new(CosetImpl::new(CirclePointIndexImpl::new(1), 0));
        let column1_query_values = array![m31(1), m31(9)];
        let column0_query_values = array![m31(5), m31(3)];
        let queried_values_per_column = array![@column0_query_values, @column1_query_values];
        let p0 = QM31_CIRCLE_GEN;
        let p1 = QM31_CIRCLE_GEN + QM31_CIRCLE_GEN;
        let sample_batches = array![
            ColumnSampleBatch { point: p0, columns_and_values: array![(0, @qm31(0, 1, 2, 3))] },
            ColumnSampleBatch { point: p1, columns_and_values: array![(1, @qm31(1, 2, 3, 4))] }
        ];
        let quotient_constants = QuotientConstantsImpl::gen(@sample_batches, alpha);
        let denominator_inverses = quotient_denominator_inverses(@sample_batches, domain);
        let row = 0;

        let res = accumulate_row_quotients(
            @sample_batches,
            @queried_values_per_column,
            @quotient_constants,
            @denominator_inverses,
            row,
            domain.size(),
            0,
            domain.at(0)
        );

        assert!(res == qm31(545815778, 838613809, 1761463254, 2019099482));
    }

    #[test]
    fn test_accumulate_row_quotients_with_offset() {
        let alpha = qm31(4, 3, 2, 1);
        let domain = CircleDomainImpl::new(CosetImpl::new(CirclePointIndexImpl::new(1), 0));
        let column1_query_values = array![m31(1), m31(9), m31(2), m31(5)];
        let column0_query_values = array![m31(5), m31(3), m31(3), m31(1)];
        let queried_values_per_column = array![@column0_query_values, @column1_query_values];
        let p0 = QM31_CIRCLE_GEN;
        let p1 = QM31_CIRCLE_GEN + QM31_CIRCLE_GEN;
        let sample_batches = array![
            ColumnSampleBatch { point: p0, columns_and_values: array![(0, @qm31(0, 1, 2, 3))] },
            ColumnSampleBatch { point: p1, columns_and_values: array![(1, @qm31(1, 2, 3, 4))] }
        ];
        let quotient_constants = QuotientConstantsImpl::gen(@sample_batches, alpha);
        let denominator_inverses = quotient_denominator_inverses(@sample_batches, domain);
        let column_eval_offset = 2;
        let row = 1;

        let res = accumulate_row_quotients(
            @sample_batches,
            @queried_values_per_column,
            @quotient_constants,
            @denominator_inverses,
            row,
            domain.size(),
            column_eval_offset,
            domain.at(1)
        );

        assert!(res == qm31(1352199520, 303329565, 518279043, 1496238271));
    }

    // Test used to benchmark step counts.
    #[test]
    fn test_fri_answers_with_1000_columns() {
        // TODO(andrew): Note Forge fails if these are declated `const ...`.
        let log_size = 16;
        let n_queries = 20;
        let n_columns = 1000;
        let log_fold_step = CIRCLE_TO_LINE_FOLD_STEP;
        let random_coeff = qm31(9, 8, 7, 6);
        assert!(n_queries < pow(2, log_size), "Query indices need to be unique");
        assert!(n_columns >= 3, "First three columns are manually created");
        let mut query_subdomains = array![];
        for coset_index in 0
            ..n_queries {
                query_subdomains.append(SubCircleDomain { coset_index, log_size: log_fold_step })
            };
        let query_domain = SparseSubCircleDomain {
            domains: query_subdomains, large_domain_log_size: log_size
        };
        let p0 = QM31_CIRCLE_GEN;
        let p1 = p0 + QM31_CIRCLE_GEN;
        let p2 = p1 + QM31_CIRCLE_GEN;
        let sample0 = PointSample { point: p0, value: qm31(0, 1, 2, 3) };
        let sample1 = PointSample { point: p1, value: qm31(1, 2, 3, 4) };
        let sample2 = PointSample { point: p2, value: qm31(2, 3, 4, 5) };
        let col0_samples = array![sample0, sample1, sample2];
        let col1_samples = array![sample0];
        let col2_samples = array![sample0, sample2];
        let mut samples_per_column = array![@col0_samples, @col1_samples, @col2_samples];
        let mut col_query_values = array![];
        for i in 0..n_queries * pow(2, log_fold_step) {
            col_query_values.append(m31(i));
        };
        let col0_query_values = col_query_values.clone();
        let col1_query_values = col_query_values.clone();
        let col2_query_values = col_query_values.clone();
        let mut queried_values_per_column = array![
            @col0_query_values, @col1_query_values, @col2_query_values
        ];
        for _ in samples_per_column.len()
            ..n_columns {
                samples_per_column.append(@col1_samples);
                queried_values_per_column.append(@col_query_values);
            };

        let _res = fri_answers_for_log_size(
            log_size, samples_per_column, random_coeff, @query_domain, queried_values_per_column
        );
    }
}
