use core::traits::TryInto;
use core::option::OptionTrait;
use core::array::ArrayTrait;
use stwo_cairo_verifier::circle::CirclePoint;
use stwo_cairo_verifier::SecureField;
use stwo_cairo_verifier::fields::m31::M31;
use stwo_cairo_verifier::queries::SparseSubCircleDomain;
use stwo_cairo_verifier::fri::evaluation::SparseCircleEvaluation;
use stwo_cairo_verifier::pcs::verifier::VerificationError;


#[derive(Drop, Copy, Debug)]
pub struct PointSample {
    pub point: CirclePoint<SecureField>,
    pub value: SecureField,
}

pub fn fri_answers(
    column_log_sizes: Array<u32>,
    samples: Array<Array<PointSample>>,
    random_coeff: SecureField,
    query_domain_per_log_size: Array<(core::felt252, SparseSubCircleDomain)>,
    queried_values_per_column: Array<Span<M31>>
) -> Result<Array<SparseCircleEvaluation>, VerificationError> {
    let mut results = array![];
    let mut fails = Option::None;

    let mut samples_vec = array![];
    let mut queried_values_per_column_vec = array![];

    let mut upper_bound = Option::None;
    let mut last_maximum = Option::None;
    while let (Option::Some(maximum), Option::Some(i)) = get_maximum(@column_log_sizes, upper_bound) {
        if last_maximum.is_some() && maximum == last_maximum.unwrap() {
            samples_vec.append(samples.at(i));
            queried_values_per_column_vec.append(queried_values_per_column.at(i));
        } else {
            match fri_answers_for_log_size(
                last_maximum.unwrap(),
                @samples_vec,
                random_coeff,
                query_domain_on(query_domain_per_log_size.span(), last_maximum.unwrap()).unwrap(),
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
    Result::Err(VerificationError::Error)
}

fn query_domain_on(
    query_domain_per_log_size: Span<(core::felt252, SparseSubCircleDomain)>,
    log_size: u32,
) -> Option<@SparseSubCircleDomain> {
    let mut j = 0;
    let mut result = Option::None;
    while j < query_domain_per_log_size.len() {
        let (log_size_index, value) = query_domain_per_log_size.at(j);
        if *log_size_index == log_size.into() {
            result = Option::Some(value);
            break;
        }
        j = j + 1;
    };
    result
}

fn get_maximum(arr: @Array<u32>, upper_bound: Option<u32>) -> (Option<u32>, Option<u32>) {
    let mut maximum = Option::None;
    let mut index = Option::None;

    let mut i = 0;
    while i < arr.len() {
        let upper_bound_condition = if let Option::Some(upper_bound) = upper_bound {
            upper_bound > *arr[i]
        } else {
            true
        };
        let lower_bound_condition = if let Option::Some(maximum) = maximum {
            *arr[i] > maximum
        } else {
            true
        };
        if upper_bound_condition && lower_bound_condition {
            maximum = Option::Some(*arr[i]);
            index = Option::Some(i);
        }
        i += 1;
    };
    (maximum, index)
}