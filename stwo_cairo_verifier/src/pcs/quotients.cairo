use stwo_cairo_verifier::circle::CirclePoint;
use stwo_cairo_verifier::SecureField;


#[derive(Drop)]
pub struct PointSample {
    pub point: CirclePoint<SecureField>,
    pub value: SecureField,
}
