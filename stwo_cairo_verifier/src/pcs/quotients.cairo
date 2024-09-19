use core::traits::TryInto;
use core::option::OptionTrait;
use core::array::ArrayTrait;
use stwo_cairo_verifier::circle::{Coset, CosetImpl,  CirclePoint, ComplexConjugateImpl};
use stwo_cairo_verifier::SecureField;
use stwo_cairo_verifier::fields::m31::M31;
use stwo_cairo_verifier::queries::{SparseSubCircleDomain, get_sparse_sub_circle_domain_dict, SubCircleDomain, SubCircleDomainImpl, SparseSubCircleDomainImpl};
use stwo_cairo_verifier::pcs::verifier::VerificationError;
use core::dict::Felt252Dict;
use stwo_cairo_verifier::sort::MaximumToMinimumSortedIterator;
use stwo_cairo_verifier::fri::evaluation::{CircleEvaluation, SparseCircleEvaluation, CircleEvaluationImpl, SparseCircleEvaluationImpl};
use stwo_cairo_verifier::utils::{bit_reverse_index, get_unique_elements};

use stwo_cairo_verifier::fields::qm31::{QM31, qm31, QM31One, QM31Trait};
use stwo_cairo_verifier::fields::cm31::{CM31, CM31Trait};
use stwo_cairo_verifier::fields::m31::m31;
use stwo_cairo_verifier::poly::circle::{CircleDomain, CircleDomainImpl};
use core::result::ResultTrait;
use stwo_cairo_verifier::utils::pow_qm31;

#[derive(Drop, Copy, Debug)]
pub struct PointSample {
    pub point: CirclePoint<SecureField>,
    pub value: SecureField,
}

#[derive(Drop)]
pub struct ColumnSampleBatch {
    pub point: CirclePoint<SecureField>,
    pub columns_and_values: Array<(usize, SecureField)>
}

#[generate_trait]
impl ColumnSampleBatchImpl of ColumnSampleBatchTrait {
    fn new_vec(samples: Span<Span<PointSample>>) -> Array<ColumnSampleBatch>{
        let points = Self::find_all_unique_points(samples);

        let mut column_sample_batches: Array<ColumnSampleBatch> = array![];

        let mut i = 0;
        while i < points.len(){
            let point = points[i].clone();

            column_sample_batches.append(Self::new_from_point_and_samples(@point, samples));

            i = i+1;
        };
        
        column_sample_batches
    }

    fn new_from_point_and_samples(point: @CirclePoint<SecureField>, samples: Span<Span<PointSample>>) -> ColumnSampleBatch{
        let mut columns_and_values: Array<(usize, SecureField)> = array![];
        
        let mut i = 0;
        while i<samples.len(){
            let mut j = 0;
            let sample: Span<PointSample> = *samples.at(i);
            while j < sample.len(){

                if point == sample.at(j).point{
                    columns_and_values.append((i, *sample.at(j).value));
                }

                j = j+1;
            };

            i = i+1;
        };

        ColumnSampleBatch{
            point: *point,
            columns_and_values: columns_and_values
        }
    }

    fn find_all_unique_points(samples:  Span<Span<PointSample>>) -> Array<CirclePoint<SecureField>>{
        let mut points: Array<CirclePoint<SecureField>> = array![];

        // Extract all CirclePoints from array of PointSamples
        let mut i = 0;
        while i < samples.len(){
            let mut j = 0;
            let column: Span<PointSample> = *samples[i];
            while j < column.len(){
                let point_sample = column[j];
                points.append(*point_sample.point);
                j = j+1;
            };
            i = i+1
        };

        get_unique_elements(@points)

    }
}


pub fn fri_answers(
    column_log_sizes: Array<u32>,
    samples: Array<Array<PointSample>>,
    random_coeff: SecureField,
    ref query_domain_per_log_size: Felt252Dict<Nullable<SparseSubCircleDomain>>,
    queried_values_per_column: Array<Span<M31>>
) -> Result<Array<SparseCircleEvaluation>, VerificationError> {
    let mut results = array![];
    let mut fails = Option::None;

    let mut samples_vec = array![];
    let mut queried_values_per_column_vec = array![];

    let mut iterator = MaximumToMinimumSortedIterator::iterate(column_log_sizes.span());

    let (i, current_log_size) = iterator.next().unwrap();
    samples_vec.append(samples.at(i).span());
    queried_values_per_column_vec.append(*queried_values_per_column.at(i));
    let mut last_log_size = current_log_size;

    while let Option::Some((i, current_log_size)) = iterator.next() {

        if current_log_size == last_log_size {
            samples_vec.append(samples.at(i).span());
            queried_values_per_column_vec.append(*queried_values_per_column.at(i));
        } else {
            let query_domain = get_sparse_sub_circle_domain_dict(ref query_domain_per_log_size, last_log_size);

            match fri_answers_for_log_size(
                last_log_size,
                samples_vec.span(),
                random_coeff,
                @query_domain,
                queried_values_per_column_vec.span()
            ) {
                    Result::Ok(result) => results.append(result),
                    Result::Err(error) => {
                        fails = Option::Some(error);
                        break;
                    }
            }
                
            last_log_size = current_log_size;
            samples_vec = array![samples.at(i).span()];
            queried_values_per_column_vec = array![queried_values_per_column.at(i).clone()];
        }
    };

    let query_domain = get_sparse_sub_circle_domain_dict(ref query_domain_per_log_size, last_log_size);
    match fri_answers_for_log_size(
        last_log_size,
        samples_vec.span(),
        random_coeff,
        @query_domain,
        queried_values_per_column_vec.span()
    ) {
            Result::Ok(result) => results.append(result),
            Result::Err(error) => {
                fails = Option::Some(error);
            }
    }

    if fails.is_some() {
        Result::Err(fails.unwrap())
    } else {
        Result::Ok(results)
    }
}

#[derive(Drop, Debug)]
struct QuotientConstants {
    pub line_coeffs: Array<Array<(QM31, QM31, QM31)>>,
    /// The random coefficients used to linearly combine the batched quotients For more details see
    /// [self::batch_random_coeffs].
    pub batch_random_coeffs: Array<QM31>,
    /// The inverses of the denominators of the quotients.
    pub denominator_inverses: Array<Array<CM31>>
}

fn quotient_constants(
    sample_batches: Span<ColumnSampleBatch>,
    random_coeff: QM31,
    domain: CircleDomain 
) -> QuotientConstants {
    let line_coeffs = column_line_coeffs(sample_batches, random_coeff);
    let batch_random_coeffs = batch_random_coeffs(sample_batches, random_coeff);
    let denominator_inverses = denominator_inverses(sample_batches, domain);
    QuotientConstants { 
        line_coeffs: line_coeffs,
        batch_random_coeffs: batch_random_coeffs,
        denominator_inverses: denominator_inverses
    }
}

pub fn column_line_coeffs(
    sample_batches: Span<ColumnSampleBatch>,
    random_coeff: SecureField,
) -> Array<Array<(SecureField, SecureField, SecureField)>> {
    let mut column_line_coeffs = array![];
    let mut i = 0;
    while i < sample_batches.len() {
        let sample_batch = sample_batches[i];
        let mut alpha = QM31One::one();
        let mut array = array![];
        let mut j = 0;
        while j < sample_batch.columns_and_values.len() {
            let (_column, sampled_value) = *sample_batch.columns_and_values[j];
            alpha = alpha * random_coeff;
            let mut sample = PointSample {
                point: *sample_batch.point,
                value: sampled_value,
            };
            let value = complex_conjugate_line_coeffs(ref sample, alpha);

            array.append(value);
            j = j + 1;
        };

        column_line_coeffs.append(array);
        i = i + 1;
    };
    column_line_coeffs
}

fn batch_random_coeffs(
    sample_batches: Span<ColumnSampleBatch>,
    random_coeff: SecureField,
) -> Array<SecureField> {
    let mut batch_random_coeffs = array![];
    let mut i = 0;
    while i < sample_batches.len() {
        let sample_batch = sample_batches[i];
        let element = pow_qm31(random_coeff, sample_batch.columns_and_values.len());
        batch_random_coeffs.append(element);
        i = i + 1;
    };
    batch_random_coeffs
}

fn denominator_inverses(
    sample_batches: Span<ColumnSampleBatch>,
    domain: CircleDomain,
) -> Array<Array<CM31>> {
    let mut flat_denominators = array![];
    // We want a P to be on a line that passes through a point Pr + uPi in QM31^2, and its conjugate
    // Pr - uPi. Thus, Pr - P is parallel to Pi. Or, (Pr - P).x * Pi.y - (Pr - P).y * Pi.x = 0.
    let mut i = 0;
    while i < sample_batches.len() {
        let sample_batch = sample_batches[i];
        // Extract Pr, Pi.
        let prx = sample_batch.point.x.a;
        let pry = sample_batch.point.y.a;
        let pix = sample_batch.point.x.b;
        let piy = sample_batch.point.y.b;
        let mut row = 0;
        println!("domain {:?}", domain);
        println!("domain size {:?}", domain.size());
        while row < domain.size() {
            let domain_point = domain.at(row);
            let first_substraction = CM31 { a: *prx.a - domain_point.x, b: *prx.b };
            let second_substraction = CM31 { a: *pry.a - domain_point.y, b: *pry.b };
            flat_denominators.append(first_substraction * *piy - second_substraction * *pix);
            row = row + 1;
        };
        i = i + 1;
    };

    println!("Flat denominator: {:?}", flat_denominators);
    let mut flat_denominator_inverses: Array<CM31> = array![];
    batch_inverse(flat_denominators.span(), ref flat_denominator_inverses);

    let mut result: Array<Array<CM31>> = array![];
    let mut i = 0;
    let mut flat_denominator_inverses = flat_denominator_inverses.span();
    while i < flat_denominator_inverses.len() {
        let mut denominator_inverses: Array<CM31> = array![];
        let mut j = 0;
        while j < domain.size() {
            let flat_denominator_inverse = *flat_denominator_inverses[j];
            denominator_inverses.append(flat_denominator_inverse);
            j = j + 1;
        };
        let denominator_inverses_bit_reversed = bit_reverse(denominator_inverses.span());

        result.append(denominator_inverses_bit_reversed);
        i = i + 1;
    };
    result
}

fn bit_reverse(v: Span<CM31>) -> Array<CM31> {
    let n = v.len();
    //assert!(n.is_power_of_two());
    let log_n = ilog2(n);

    let mut result = array![];
    let mut i = 0;
    while i < n {
        let j = bit_reverse_index(i, log_n);
        result.append(*v[j]);
        i = i + 1;
    };
    result
}

fn ilog2(n: u32) -> u32 {
    let mut log = 0;
    let mut current = n;
    while current > 1 {
        current = current / 2;
        log = log + 1;
    };
    log
}

fn batch_inverse(array_to_inverse: Span<CM31>, ref result: Array<CM31>) {
    let mut i = 0;
    while i < array_to_inverse.len() {
        result.append((*array_to_inverse[i]).inverse());
        i = i + 1;
    }
}

pub fn complex_conjugate_line_coeffs(
    ref sample: PointSample,
    alpha: SecureField,
) -> (SecureField, SecureField, SecureField) {
    // TODO(AlonH): This assertion will fail at a probability of 1 to 2^62. Use a better solution.
    assert_ne!(
        sample.point.y,
        sample.point.y.complex_conjugate(),
        "Cannot evaluate a line with a single point ({:?}).",
        sample.point
    );

    let a: QM31 = sample.value.complex_conjugate() - sample.value;
    let c: QM31 = sample.point.complex_conjugate().y - sample.point.y;
    let b = sample.value * c - a * sample.point.y;

    (alpha * a, alpha * b, alpha * c)
}

fn accumulate_row_quotients(
    sample_batches: Span<ColumnSampleBatch>,
    columns: Span<CircleEvaluation>,
    quotient_constants: @QuotientConstants,
    row: usize,
    domain_point: CirclePoint<M31>,
) -> QM31 {
    let mut row_accumulator = qm31(0, 0, 0, 0);
    let mut i = 0;
    while i < sample_batches.len() {
        let sample_batch = sample_batches.at(i);
        let line_coeffs = quotient_constants.line_coeffs.at(i);
        let batch_coeff = quotient_constants.batch_random_coeffs.at(i);
        let denominator_inverses = quotient_constants.denominator_inverses.at(i);
        let mut numerator = qm31(0, 0, 0, 0);
        let mut j = 0;
        while j < line_coeffs.len() {
            let (column_index, _) = sample_batch.columns_and_values[j];
            let (a, b, c) = line_coeffs[j];
            let column = columns.at(*column_index);
            let value = *column.values[row] *  *c;
            let y = qm31(0, 0, 0, domain_point.y.inner);
            let linear_term = *a * y + *b;
            numerator = numerator + (value - linear_term);
            j = j + 1;
        };
        i = i + 1;
        let denominator_inverse = *denominator_inverses[row];
        let multiplication = QM31 { a: numerator.a * denominator_inverse, b: numerator.b * denominator_inverse };
        row_accumulator = row_accumulator * *batch_coeff + multiplication;
    };
    row_accumulator
}

pub fn fri_answers_for_log_size(
    log_size: u32,
    samples: Span<Span<PointSample>>,
    random_coeff: SecureField,
    query_domain: @SparseSubCircleDomain,
    queried_values_per_column: Span<Span<M31>>,
) -> Result<SparseCircleEvaluation, VerificationError> {

    let commitment_domain = CircleDomain{half_coset: CosetImpl::half_odds(log_size - 1)};

    let sample_batches = ColumnSampleBatchImpl::new_vec(samples).span();
    

    let mut i = 0;
    let mut invalid_structure_error = false;
    while i < queried_values_per_column.len() {
        if (*queried_values_per_column[i]).len() != query_domain.flatten().len() {
            invalid_structure_error = true;
            break;
        }
        i = i + 1;
    };

    if(invalid_structure_error) {
        return Result::Err(VerificationError::InvalidStructure);
    }

    let mut evals = array![];
    let mut i = 0;
    while i < query_domain.domains.len() {
        let subdomain = query_domain.domains[i];

        let domain = subdomain.to_circle_domain(commitment_domain);

        let quotient_constants = quotient_constants(sample_batches, random_coeff, domain);
        let mut column_evals = array![];
        let mut j = 0;
        while j < queried_values_per_column.len() {
            let queried_values = queried_values_per_column[j];
            let mut values = array![];
            let mut k = 0;
            while k < domain.size() {
                // TODO: generalize circle evaluation instead, we're casting to QM31 but not necessary yet.
                values.append(qm31(0, 0, 0, *queried_values[i * domain.size() + k].inner));
                k = k + 1;
            };

            let eval = CircleEvaluationImpl::new(
                domain,
                values,
            );
            column_evals.append(eval);
            j = j + 1;
        };
        println!("Column Evals{:?}", column_evals);
        let mut values = array![];
        let mut row = 0;
        while row < domain.size() {
            let domain_point = domain.at(bit_reverse_index(row, log_size));

            // Chequear que accumulate_row_quotients esta funcionando bien porque EVALS tiene bien el dominio pero mal los valores.
            let value = accumulate_row_quotients(
                sample_batches,
                column_evals.span(),
                @quotient_constants,
                row,
                domain_point,
            );
            values.append(value);
            row = row + 1;
        };
        let eval = CircleEvaluationImpl::new(domain, values);
        evals.append(eval);
        i = i + 1;
    };
    println!("Evals{:?}", evals);

    let res = SparseCircleEvaluationImpl::new(evals);

    Result::Ok(res)
//     Result::Ok(SparseCircleEvaluation{ subcircle_evals: array![
//         CircleEvaluation { domain: CircleDomain { half_coset: Coset { initial_index: 41943040, step_size: 2147483648, log_size: 0 } },
//                           values: array![qm31(908763622, 1585299850, 463460326, 1048007085), qm31(1123843977, 425287367, 713867037, 231900223)]}, 
//         CircleEvaluation { domain: CircleDomain { half_coset: Coset { initial_index: 2122317824, step_size: 2147483648, log_size: 0 } }, 
//                           values: array![qm31(1489324268, 1315746611, 1235430137, 1650466882), qm31(158201991, 1003575152, 1730507932, 1741921065)]}
//       ] 
// })
}


#[cfg(test)]
mod tests {
    use super::{PointSample, fri_answers_for_log_size};
    use stwo_cairo_verifier::circle::{Coset, CirclePoint};
    use stwo_cairo_verifier::fields::qm31::{QM31, qm31};
    use stwo_cairo_verifier::queries::{SparseSubCircleDomain, SubCircleDomain, get_sparse_sub_circle_domain_dict};
    use stwo_cairo_verifier::fields::m31::m31;
    use stwo_cairo_verifier::fri::evaluation::{CircleEvaluation,SparseCircleEvaluation};
    use stwo_cairo_verifier::poly::circle::{CircleDomain, CircleDomainImpl};
    use core::result::ResultTrait;
    use stwo_cairo_verifier::pcs::verifier::VerificationError;

    #[test]
    fn test_fri_answers_for_log_size_returns_correct_value(){
        let log_size = 7;
        let samples = array![array![PointSample { point: CirclePoint { x: qm31(700515869, 1711372691, 739886384, 2007341053),
                                                           y: qm31(326786628, 606109638, 1064549171, 242662007)},
                                      value: qm31(734531923, 1747759514, 825724491, 1380781623)}].span(),
                       array![PointSample { point: CirclePoint { x: qm31(700515869, 1711372691, 739886384, 2007341053),
                                                           y: qm31(326786628, 606109638, 1064549171, 242662007) },
                                      value: qm31(409142122, 1541525101, 867367418, 349409006) }].span(),
                       array![PointSample { point: CirclePoint { x: qm31(700515869, 1711372691, 739886384, 2007341053),
                                                           y: qm31(326786628, 606109638, 1064549171, 242662007) },
                                      value: qm31(143298682, 126098004,1036758723, 1444867) }].span(),
                       array![PointSample { point: CirclePoint { x: qm31(700515869, 1711372691, 739886384, 2007341053),
                                                           y: qm31(326786628, 606109638, 1064549171, 242662007) },
                                      value: qm31(498615358, 1652904678, 568903503, 193392082) }].span()];
        let random_coeff = qm31(934912220, 2101060572, 478944000, 1026736704);
        let query_domain_per_log_size = SparseSubCircleDomain{domains: array![SubCircleDomain { coset_index: 32, log_size: 1 }, SubCircleDomain { coset_index: 63, log_size: 1 }], large_domain_log_size: 7};
        let queried_values_per_column = array![array![m31(1720115923), m31(275996517), m31(1084325550), m31(1934680704)].span(),
                                         array![m31(1270808745), m31(836361095), m31(1701916643), m31(1812027089)].span(),
                                         array![m31(1631066942), m31(97828054), m31(774575764), m31(1860917732)].span(), 
                                         array![m31(1389614630), m31(525640714), m31(1095538838), m31(1384646193)].span()];
        let expected_result = SparseCircleEvaluation { subcircle_evals: array![
                                                                            CircleEvaluation { domain: CircleDomain { half_coset: Coset { initial_index: 41943040, step_size: 2147483648, log_size: 0 } },
                                                                                              values: array![qm31(908763622, 1585299850, 463460326, 1048007085), qm31(1123843977, 425287367, 713867037, 231900223)]}, 
                                                                            CircleEvaluation { domain: CircleDomain { half_coset: Coset { initial_index: 2122317824, step_size: 2147483648, log_size: 0 } }, 
                                                                                              values: array![qm31(1489324268, 1315746611, 1235430137, 1650466882), qm31(158201991, 1003575152, 1730507932, 1741921065)]}
                                                                          ] 
                                                        };

        let function_result = fri_answers_for_log_size(
            log_size,
            samples.span(),
            random_coeff,
            @query_domain_per_log_size,
            queried_values_per_column.span(),
        ).unwrap();

        assert_eq!(expected_result, function_result);
    }

}




//parameters for fri answers for log size test
// log size: 7
// samples: [[PointSample { point: CirclePoint { x: (700515869 + 1711372691i) + (739886384 + 2007341053i)u, y: (326786628 + 606109638i) + (1064549171 + 242662007i)u }, value: (734531923 + 1747759514i) + (825724491 + 1380781623i)u }], [PointSample { point: CirclePoint { x: (700515869 + 1711372691i) + (739886384 + 2007341053i)u, y: (326786628 + 606109638i) + (1064549171 + 242662007i)u }, value: (409142122 + 1541525101i) + (867367418 + 349409006i)u }], [PointSample { point: CirclePoint { x: (700515869 + 1711372691i) + (739886384 + 2007341053i)u, y: (326786628 + 606109638i) + (1064549171 + 242662007i)u }, value: (1432986824 + 1260980040i) + (1036758723 + 1444867187i)u }], [PointSample { point: CirclePoint { x: (700515869 + 1711372691i) + (739886384 + 2007341053i)u, y: (326786628 + 606109638i) + (1064549171 + 242662007i)u }, value: (498615358 + 1652904678i) + (568903503 + 1933920823i)u }]]
// random_coeff: (934912220 + 2101060572i) + (478944000 + 1026736704i)u
// query_domain_per_log_size: SparseSubCircleDomain { domains: [SubCircleDomain { coset_index: 32, log_size: 1 }, SubCircleDomain { coset_index: 63, log_size: 1 }], large_domain_log_size: 7 }
// queried_valued_per_column: [[M31(1720115923), M31(275996517), M31(1084325550), M31(1934680704)], [M31(1270808745), M31(836361095), M31(1701916643), M31(1812027089)], [M31(1631066942), M31(97828054), M31(774575764), M31(1860917732)], [M31(1389614630), M31(525640714), M31(1095538838), M31(1384646193)]]


// what it should return:
//Ok(SparseCircleEvaluation { subcircle_evals: [CircleEvaluation { domain: CircleDomain { half_coset: Coset { initial_index: CirclePointIndex(41943040), initial: CirclePoint { x: M31(251924953), y: M31(636875771) }, step_size: CirclePointIndex(2147483648), step: CirclePoint { x: M31(1), y: M31(0) }, log_size: 0 } }, values: [(908763622 + 1585299850i) + (463460326 + 1048007085i)u, (1123843977 + 425287367i) + (713867037 + 231900223i)u], _eval_order: PhantomData<stwo_prover::core::poly::BitReversedOrder> }, CircleEvaluation { domain: CircleDomain { half_coset: Coset { initial_index: CirclePointIndex(2122317824), initial: CirclePoint { x: M31(1357626641), y: M31(81378258) }, step_size: CirclePointIndex(2147483648), step: CirclePoint { x: M31(1), y: M31(0) }, log_size: 0 } }, values: [(1489324268 + 1315746611i) + (1235430137 + 1650466882i)u, (158201991 + 1003575152i) + (1730507932 + 1741921065i)u], _eval_order: PhantomData<stwo_prover::core::poly::BitReversedOrder> }] })