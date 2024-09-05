use stwo_cairo_verifier::circle::CirclePoint;
use stwo_cairo_verifier::SecureField;
use stwo_cairo_verifier::fields::m31::M31;
use stwo_cairo_verifier::queries::SparseSubCircleDomain;
use stwo_cairo_verifier::fri::evaluation::SparseCircleEvaluation;
use stwo_cairo_verifier::pcs::verifier::VerificationError;


#[derive(Drop)]
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
    Result::Ok(array![])
}
