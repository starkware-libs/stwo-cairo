use core::option::OptionTrait;
use core::traits::TryInto;
use stwo_cairo_verifier::fields::m31::{M31, m31};
use super::utils::pow;

pub const M31_CIRCLE_GEN: CirclePointM31 =
    CirclePointM31 { x: M31 { inner: 2 }, y: M31 { inner: 1268011823 }, };

#[derive(Drop, Copy, Debug, PartialEq, Eq)]
pub struct CirclePointM31 {
    pub x: M31,
    pub y: M31,
}

#[generate_trait]
pub impl CirclePointM31Impl of CirclePointM31Trait {
    fn zero() -> CirclePointM31 {
        CirclePointM31 { x: m31(1), y: m31(0) }
    }

    fn mul(self: @CirclePointM31, mut scalar: u32) -> CirclePointM31 {
        let mut result = Self::zero();
        let mut cur = *self;
        while scalar > 0 {
            if scalar & 1 == 1 {
                result = result + cur;
            }
            cur = cur + cur;
            scalar = scalar / 2;
        };
        result
    }
}

impl CirclePointM31Add of Add<CirclePointM31> {
    fn add(lhs: CirclePointM31, rhs: CirclePointM31) -> CirclePointM31 {
        CirclePointM31 { x: lhs.x * rhs.x - lhs.y * rhs.y, y: lhs.x * rhs.y + lhs.y * rhs.x }
    }
}

#[test]
fn test_generator() {
    let mut result = M31_CIRCLE_GEN.mul(pow(2, 30).try_into().unwrap());
    let expected_result = CirclePointM31 { x: -m31(1), y: m31(0) };
    assert_eq!(expected_result, result);
}
