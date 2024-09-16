use core::traits::TryInto;
use core::option::OptionTrait;
use core::array::ArrayTrait;
use stwo_cairo_verifier::circle::{Coset, CosetImpl,  CirclePoint};
use stwo_cairo_verifier::SecureField;
use stwo_cairo_verifier::fields::m31::M31;
use stwo_cairo_verifier::queries::{SparseSubCircleDomain, get_sparse_sub_circle_domain_dict, SubCircleDomain};
use stwo_cairo_verifier::pcs::verifier::VerificationError;
use core::dict::Felt252Dict;
use stwo_cairo_verifier::sort::MaximumToMinimumSortedIterator;
use stwo_cairo_verifier::fri::evaluation::{CircleEvaluation,SparseCircleEvaluation};
use stwo_cairo_verifier::utils::get_unique_elements;

use stwo_cairo_verifier::fields::qm31::{QM31, qm31};
use stwo_cairo_verifier::fields::m31::m31;
use stwo_cairo_verifier::poly::circle::{CircleDomain, CircleDomainImpl};
use core::result::ResultTrait;

#[derive(Drop, Copy, Debug)]
pub struct PointSample {
    pub point: CirclePoint<SecureField>,
    pub value: SecureField,
}

#[derive(Drop)]
pub struct ColumnSampleBatch{
    pub point: CirclePoint<SecureField>,
    pub columns_and_values: Array<(usize, SecureField)>
}

#[generate_trait]
impl ColumnSampleBatchImpl of ColumnSampleBatchTrait {
    // !todo: polinomiopolinomio polinomio
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

    let mut last_maximum = Option::None;
    let mut iterator = MaximumToMinimumSortedIterator::iterate(column_log_sizes.span());
    while let Option::Some((maximum, i)) = iterator.next() {
        if last_maximum.is_some() && maximum == last_maximum.unwrap() {
            samples_vec.append(samples.at(i));
            queried_values_per_column_vec.append(queried_values_per_column.at(i));
        } else {
            let query_domain = get_sparse_sub_circle_domain_dict(ref query_domain_per_log_size, last_maximum.unwrap());

            match fri_answers_for_log_size(
                last_maximum.unwrap(),
                @samples_vec,
                random_coeff,
                @query_domain,
                @queried_values_per_column_vec
            ) {
                    Result::Ok(result) => results.append(result),
                    Result::Err(error) => {
                        fails = Option::Some(error);
                        break;
                    }
            }
                
            last_maximum = Option::Some(maximum);
            samples_vec = array![samples.at(i)];
            queried_values_per_column_vec = array![queried_values_per_column.at(i)];
        }
    };
    if fails.is_some() {
        Result::Err(fails.unwrap())
    } else {
        Result::Ok(results)
    }
}


pub fn fri_answers_for_log_size(
    log_size: u32,
    samples: @Array<@Array<PointSample>>,
    random_coeff: SecureField,
    query_domain: @SparseSubCircleDomain,
    queried_values_per_column: @Array<@Span<M31>>,
) -> Result<SparseCircleEvaluation, VerificationError> {

    //TODO: Build this circledomain using the coset.odds method in the rust implementation.
    let commitment_domain = CircleDomain{half_coset: CosetImpl::odds(log_size)};

    // implementar columnsamplebatch que tiene un circlePoint y un vec de vec de PointSample
    //let sample_batches = ColumnSampleBatch::new_vec(samples);



    // for queried_values in queried_values_per_column {
    //     if queried_values.len() != query_domain.flatten().len() {
    //         return Err(VerificationError::InvalidStructure(
    //             "Insufficient number of queried values".to_string(),
    //         ));
    //     }
    // }
    // let mut queried_values_per_column = queried_values_per_column
    //     .iter()
    //     .map(|q| q.iter())
    //     .collect_vec();

    // let mut evals = Vec::new();
    // for subdomain in query_domain.iter() {
    //     let domain = subdomain.to_circle_domain(&commitment_domain);
    //     let quotient_constants = quotient_constants(&sample_batches, random_coeff, domain);
    //     let mut column_evals = Vec::new();
    //     for queried_values in queried_values_per_column.iter_mut() {
    //         let eval = CircleEvaluation::new(
    //             domain,
    //             queried_values.take(domain.size()).copied().collect_vec(),
    //         );
    //         column_evals.push(eval);
    //     }

    //     let mut values = Vec::new();
    //     for row in 0..domain.size() {
    //         let domain_point = domain.at(bit_reverse_index(row, log_size));
    //         let value = accumulate_row_quotients(
    //             &sample_batches,
    //             &column_evals.iter().collect_vec(),
    //             &quotient_constants,
    //             row,
    //             domain_point,
    //         );
    //         values.push(value);
    //     }
    //     let eval = CircleEvaluation::new(domain, values);
    //     evals.push(eval);
    // }

    // let res = SparseCircleEvaluation::new(evals);
    // if !queried_values_per_column.iter().all(|x| x.is_empty()) {
    //     return Err(VerificationError::InvalidStructure(
    //         "Too many queried values".to_string(),
    //     ));
    // }
    // Ok(res)


//     return Result::Ok(SparseCircleEvaluation { subcircle_evals: array![
//         CircleEvaluation { domain: CircleDomain { half_coset: Coset { initial_index: 41943040, step_size: 2147483648, log_size: 0 } },
//                           values: array![qm31(908763622, 1585299850, 463460326, 1048007085), qm31(1123843977, 425287367, 713867037, 231900223)]}, 
//         CircleEvaluation { domain: CircleDomain { half_coset: Coset { initial_index: 2122317824, step_size: 2147483648, log_size: 0 } }, 
//                           values: array![qm31(1489324268, 1315746611, 1235430137, 1650466882), qm31(158201991, 1003575152, 1730507932, 1741921065)]}
//       ] 
// });
    Result::Err(VerificationError::Error)
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
        let samples = array![@array![PointSample { point: CirclePoint { x: qm31(700515869, 1711372691, 739886384, 2007341053),
                                                           y: qm31(326786628, 606109638, 1064549171, 242662007)},
                                      value: qm31(734531923, 1747759514, 825724491, 1380781623)}],
                       @array![PointSample { point: CirclePoint { x: qm31(700515869, 1711372691, 739886384, 2007341053), 
                                                           y: qm31(326786628, 606109638, 1064549171, 242662007) },
                                      value: qm31(409142122, 1541525101, 867367418, 349409006) }], 
                       @array![PointSample { point: CirclePoint { x: qm31(700515869, 1711372691, 739886384, 2007341053), 
                                                           y: qm31(326786628, 606109638, 1064549171, 242662007) },
                                      value: qm31(143298682, 126098004,1036758723, 1444867) }],
                       @array![PointSample { point: CirclePoint { x: qm31(700515869, 1711372691, 739886384, 2007341053), 
                                                           y: qm31(326786628, 606109638, 1064549171, 242662007) },
                                      value: qm31(498615358, 1652904678, 568903503, 193392082) }]];
        let random_coeff = qm31(934912220, 2101060572, 478944000, 1026736704);
        let query_domain_per_log_size = SparseSubCircleDomain{domains: array![SubCircleDomain { coset_index: 32, log_size: 1 }, SubCircleDomain { coset_index: 63, log_size: 1 }], large_domain_log_size: 7};
        let queried_values_per_column = array![@array![m31(1720115923), m31(275996517), m31(1084325550), m31(1934680704)].span(),
                                         @array![m31(1270808745), m31(836361095), m31(1701916643), m31(1812027089)].span(),
                                         @array![m31(1631066942), m31(97828054), m31(774575764), m31(1860917732)].span(), 
                                         @array![m31(1389614630), m31(525640714), m31(1095538838), m31(1384646193)].span()];
        let expected_result = SparseCircleEvaluation { subcircle_evals: array![
                                                                            CircleEvaluation { domain: CircleDomain { half_coset: Coset { initial_index: 41943040, step_size: 2147483648, log_size: 0 } },
                                                                                              values: array![qm31(908763622, 1585299850, 463460326, 1048007085), qm31(1123843977, 425287367, 713867037, 231900223)]}, 
                                                                            CircleEvaluation { domain: CircleDomain { half_coset: Coset { initial_index: 2122317824, step_size: 2147483648, log_size: 0 } }, 
                                                                                              values: array![qm31(1489324268, 1315746611, 1235430137, 1650466882), qm31(158201991, 1003575152, 1730507932, 1741921065)]}
                                                                          ] 
                                                        };

        let function_result = fri_answers_for_log_size(
            log_size,
            @samples,
            random_coeff,
            @query_domain_per_log_size,
            @queried_values_per_column,
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