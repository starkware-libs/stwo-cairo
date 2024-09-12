use core::traits::TryInto;
use core::option::OptionTrait;
use core::array::ArrayTrait;
use stwo_cairo_verifier::circle::{Coset, CirclePoint};
use stwo_cairo_verifier::SecureField;
use stwo_cairo_verifier::fields::m31::M31;
use stwo_cairo_verifier::queries::{SparseSubCircleDomain, get_sparse_sub_circle_domain_dict, SubCircleDomain};
use stwo_cairo_verifier::pcs::verifier::VerificationError;
use core::dict::Felt252Dict;
use stwo_cairo_verifier::sort::{GreaterThan, iterate_sorted};
use stwo_cairo_verifier::fri::evaluation::{CircleEvaluation,SparseCircleEvaluation};


use stwo_cairo_verifier::fields::qm31::{QM31, qm31};
use stwo_cairo_verifier::fields::m31::m31;
use stwo_cairo_verifier::poly::circle::{CircleDomain, CircleDomainImpl};
use core::result::ResultTrait;

#[derive(Drop, Copy, Debug)]
pub struct PointSample {
    pub point: CirclePoint<SecureField>,
    pub value: SecureField,
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

    let mut upper_bound = Option::None;
    let mut last_maximum = Option::None;
    while let (Option::Some(maximum), Option::Some(i)) = iterate_sorted(@column_log_sizes, upper_bound, @GreaterThan {}) {
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
        upper_bound = Option::Some(maximum);
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