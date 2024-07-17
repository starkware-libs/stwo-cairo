use stwo_cairo_verifier::fields::m31::{M31, m31};
use super::utils::pow;

pub const M31_CIRCLE_GEN: CirclePointM31 =
    CirclePointM31 { x: M31 { inner: 2 }, y: M31 { inner: 1268011823 }, };

pub const CIRCLE_LOG_ORDER: u32 = 31;

// `CIRCLE_ORDER` equals 2^31
pub const CIRCLE_ORDER: u32 = 2147483648;

// `CIRCLE_ORDER_BIT_MASK` equals 2^31 - 1
pub const CIRCLE_ORDER_BIT_MASK: u32 = 0x7fffffff;

#[derive(Drop, Copy, Debug, PartialEq, Eq)]
pub struct CirclePointM31 {
    pub x: M31,
    pub y: M31,
}

#[generate_trait]
pub impl CirclePointM31Impl of CirclePointM31Trait {
    // Returns the neutral element of the circle.
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
    // The operation of the circle as a group with additive notation.
    fn add(lhs: CirclePointM31, rhs: CirclePointM31) -> CirclePointM31 {
        CirclePointM31 { x: lhs.x * rhs.x - lhs.y * rhs.y, y: lhs.x * rhs.y + lhs.y * rhs.x }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Drop)]
pub struct Coset {
    // This is an index in the range [0, 2^31)
    pub initial_index: usize,
    pub step_size: usize,
    pub log_size: u32,
}

#[generate_trait]
pub impl CosetImpl of CosetTrait {
    fn index_at(self: @Coset, index: usize) -> usize {
        let index_times_step = core::integer::u32_wide_mul(*self.step_size, index);
        let result = ((*self.initial_index).into() + index_times_step);
        (result & CIRCLE_ORDER_BIT_MASK.into()).try_into().unwrap()
    }

    fn new(initial_index: usize, log_size: u32) -> Coset {
        assert!(initial_index < CIRCLE_ORDER);
        let step_size = pow(2, CIRCLE_LOG_ORDER - log_size);
        Coset { initial_index, step_size, log_size }
    }

    fn double(self: @Coset) -> Coset {
        assert!(*self.log_size > 0);
        let double_initial_index = *self.initial_index * 2;
        let double_step_size = *self.step_size * 2;
        let new_log_size = *self.log_size - 1;

        Coset {
            initial_index: double_initial_index & CIRCLE_ORDER_BIT_MASK,
            step_size: double_step_size & CIRCLE_ORDER_BIT_MASK,
            log_size: new_log_size
        }
    }

    fn at(self: @Coset, index: usize) -> CirclePointM31 {
        M31_CIRCLE_GEN.mul(self.index_at(index))
    }

    fn size(self: @Coset) -> usize {
        pow(2, *self.log_size)
    }
}


#[cfg(test)]
mod tests {
    use super::{M31_CIRCLE_GEN, CIRCLE_ORDER, CirclePointM31, CirclePointM31Impl, Coset, CosetImpl};
    use stwo_cairo_verifier::fields::m31::m31;

    #[test]
    fn test_add_1() {
        let i = CirclePointM31 { x: m31(0), y: m31(1) };
        let result = i + i;
        let expected_result = CirclePointM31 { x: -m31(1), y: m31(0) };

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_add_2() {
        let point_1 = CirclePointM31 { x: m31(750649172), y: m31(1991648574) };
        let point_2 = CirclePointM31 { x: m31(1737427771), y: m31(309481134) };
        let result = point_1 + point_2;
        let expected_result = CirclePointM31 { x: m31(1476625263), y: m31(1040927458) };

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_zero_1() {
        let result = CirclePointM31Impl::zero();
        let expected_result = CirclePointM31 { x: m31(1), y: m31(0) };
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_zero_2() {
        let point_1 = CirclePointM31 { x: m31(750649172), y: m31(1991648574) };
        let point_2 = CirclePointM31Impl::zero();
        let expected_result = point_1.clone();
        let result = point_1 + point_2;

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_mul_1() {
        let point_1 = CirclePointM31 { x: m31(750649172), y: m31(1991648574) };
        let result = point_1.mul(5);
        let expected_result = point_1 + point_1 + point_1 + point_1 + point_1;

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_mul_2() {
        let point_1 = CirclePointM31 { x: m31(750649172), y: m31(1991648574) };
        let result = point_1.mul(8);
        let mut expected_result = point_1 + point_1;
        expected_result = expected_result + expected_result;
        expected_result = expected_result + expected_result;

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_mul_3() {
        let point_1 = CirclePointM31 { x: m31(750649172), y: m31(1991648574) };
        let result = point_1.mul(418776494);
        let expected_result = CirclePointM31 { x: m31(1987283985), y: m31(1500510905) };

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_generator_order() {
        let half_order = CIRCLE_ORDER / 2;
        let mut result = M31_CIRCLE_GEN.mul(half_order);
        let expected_result = CirclePointM31 { x: -m31(1), y: m31(0) };

        // Assert `M31_CIRCLE_GEN^{2^30}` equals `-1`.
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_coset_index_at() {
        let coset = Coset { initial_index: 16777216, log_size: 5, step_size: 67108864 };
        let result = coset.index_at(8);
        let expected_result = 553648128;
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_coset_constructor() {
        let result = CosetImpl::new(16777216, 5);
        let expected_result = Coset { initial_index: 16777216, log_size: 5, step_size: 67108864 };
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_coset_double() {
        let coset = Coset { initial_index: 16777216, step_size: 67108864, log_size: 5 };
        let result = coset.double();

        let expected_result = Coset { initial_index: 33554432, step_size: 134217728, log_size: 4 };
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_coset_at() {
        let coset = Coset { initial_index: 16777216, step_size: 67108864, log_size: 5 };
        let result = coset.at(17);
        let expected_result = CirclePointM31 { x: m31(7144319), y: m31(1742797653) };
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_coset_size() {
        let coset = Coset { initial_index: 16777216, step_size: 67108864, log_size: 5 };
        let result = coset.size();
        let expected_result = 32;
        assert_eq!(result, expected_result);
    }
}

