use core::traits::TryInto;
use core::option::OptionTrait;
use core::array::ArrayTrait;
use stwo_cairo_verifier::circle::CirclePoint;
use stwo_cairo_verifier::SecureField;
use stwo_cairo_verifier::fields::m31::M31;
use stwo_cairo_verifier::queries::{SparseSubCircleDomain, get_sparse_sub_circle_domain_dict};
use stwo_cairo_verifier::fri::evaluation::SparseCircleEvaluation;
use stwo_cairo_verifier::pcs::verifier::VerificationError;
use core::dict::Felt252Dict;
use stwo_cairo_verifier::sort::{GreaterThan, iterate_sorted};

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
    while let Option::Some((maximum, i)) = iterate_sorted(column_log_sizes.span(), ref upper_bound, @GreaterThan {}) {
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
    Result::Err(VerificationError::Error)
}