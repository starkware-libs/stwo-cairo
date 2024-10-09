use stwo_cairo_verifier::fields::m31::{M31, M31Impl};
use stwo_cairo_verifier::fields::cm31::CM31;
use stwo_cairo_verifier::fields::qm31::{QM31Impl, QM31, QM31Trait};
use super::utils::pow;
use core::num::traits::zero::Zero;
use core::num::traits::one::One;

pub const M31_CIRCLE_GEN: CirclePoint<M31> =
    CirclePoint { x: M31 { inner: 2 }, y: M31 { inner: 1268011823 }, };

pub const QM31_CIRCLE_GEN: CirclePoint<QM31> =
    CirclePoint {
        x: QM31 {
            a: CM31 { a: M31 { inner: 1 }, b: M31 { inner: 0 }, },
            b: CM31 { a: M31 { inner: 478637715 }, b: M31 { inner: 513582971 } }
        },
        y: QM31 {
            a: CM31 { a: M31 { inner: 992285211 }, b: M31 { inner: 649143431 } },
            b: CM31 { a: M31 { inner: 740191619 }, b: M31 { inner: 1186584352 } }
        },
    };

pub const CIRCLE_LOG_ORDER: u32 = 31;

// `CIRCLE_ORDER` equals 2^31
pub const CIRCLE_ORDER: u32 = 2147483648;

// `CIRCLE_ORDER_BIT_MASK` equals 2^31 - 1
pub const CIRCLE_ORDER_BIT_MASK: u32 = 0x7fffffff;

// `U32_BIT_MASK` equals 2^32 - 1
pub const U32_BIT_MASK: u64 = 0xffffffff;

/// A point on the complex circle. Treated as an additive group.
#[derive(Drop, Copy, Debug, PartialEq, Eq)]
pub struct CirclePoint<F> {
    pub x: F,
    pub y: F
}

pub trait CirclePointTrait<
    F, +Add<F>, +Sub<F>, +Mul<F>, +Drop<F>, +Copy<F>, +Zero<F>, +One<F>, +PartialEq<F>
> {
    // Returns the neutral element of the circle.
    fn zero() -> CirclePoint<F> {
        CirclePoint { x: One::one(), y: Zero::zero() }
    }

    /// Applies the circle's x-coordinate doubling map.
    fn double_x(x: F) -> F {
        let sqx = x * x;
        sqx + sqx - One::one()
    }

    /// Returns the log order of a point.
    ///
    /// All points have an order of the form `2^k`.
    fn log_order(
        self: @CirclePoint<F>
    ) -> u32 {
        // we only need the x-coordinate to check order since the only point
        // with x=1 is the circle's identity
        let mut res = 0;
        let mut cur = self.x.clone();
        while cur != One::one() {
            cur = Self::double_x(cur);
            res += 1;
        };
        res
    }

    fn mul(
        self: @CirclePoint<F>, ref scalar: u128
    ) -> CirclePoint<
        F
    > {
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

impl CirclePointAdd<F, +Add<F>, +Sub<F>, +Mul<F>, +Drop<F>, +Copy<F>> of Add<CirclePoint<F>> {
    /// Performs the operation of the circle as a group with additive notation.
    fn add(lhs: CirclePoint<F>, rhs: CirclePoint<F>) -> CirclePoint<F> {
        CirclePoint { x: lhs.x * rhs.x - lhs.y * rhs.y, y: lhs.x * rhs.y + lhs.y * rhs.x }
    }
}

pub impl CirclePointM31Impl of CirclePointTrait<M31> {}

pub impl CirclePointQM31Impl of CirclePointTrait<QM31> {}

#[generate_trait]
pub impl ComplexConjugateImpl of ComplexConjugateTrait {
    fn complex_conjugate(self: CirclePoint<QM31>) -> CirclePoint<QM31> {
        CirclePoint { x: self.x.complex_conjugate(), y: self.y.complex_conjugate() }
    }
}

/// Represents the coset `initial + <step>`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Drop)]
pub struct Coset {
    // This is an index in the range [0, 2^31)
    pub initial_index: usize,
    pub step_size: usize,
    pub log_size: u32,
}

#[generate_trait]
pub impl CosetImpl of CosetTrait {
    fn new(initial_index: usize, log_size: u32) -> Coset {
        assert!(initial_index < CIRCLE_ORDER);
        let step_size = pow(2, CIRCLE_LOG_ORDER - log_size);
        Coset { initial_index, step_size, log_size }
    }

    fn index_at(self: @Coset, index: usize) -> usize {
        let index_times_step = (core::integer::u32_wide_mul(*self.step_size, index) & U32_BIT_MASK)
            .try_into()
            .unwrap();
        let result = core::integer::u32_wrapping_add(*self.initial_index, index_times_step);
        result & CIRCLE_ORDER_BIT_MASK
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

    fn at(self: @Coset, index: usize) -> CirclePoint<M31> {
        let mut scalar = self.index_at(index).into();
        M31_CIRCLE_GEN.mul(ref scalar)
    }

    /// Returns the size of the coset.
    fn size(self: @Coset) -> usize {
        pow(2, *self.log_size)
    }

    /// Creates a coset of the form `G_2n + <G_n>`.
    ///
    /// For example, for `n=8`, we get the point indices `[1,3,5,7,9,11,13,15]`.
    fn odds(log_size: u32) -> Coset {
        let subgroup_generator_index = Self::subgroup_generator_index(log_size);
        Self::new(subgroup_generator_index, log_size)
    }

    /// Creates a coset of the form `G_4n + <G_n>`.
    ///
    /// For example, for `n=8`, we get the point indices `[1,5,9,13,17,21,25,29]`.
    /// Its conjugate will be `[3,7,11,15,19,23,27,31]`.
    fn half_odds(log_size: u32) -> Coset {
        Self::new(Self::subgroup_generator_index(log_size + 2), log_size)
    }

    fn subgroup_generator_index(log_size: u32) -> u32 {
        assert!(log_size <= CIRCLE_LOG_ORDER);
        pow(2, CIRCLE_LOG_ORDER - log_size)
    }
}


#[cfg(test)]
mod tests {
    use super::{M31_CIRCLE_GEN, CIRCLE_ORDER, CirclePoint, CirclePointM31Impl, Coset, CosetImpl};
    use core::option::OptionTrait;
    use core::array::ArrayTrait;
    use core::traits::TryInto;
    use super::{CirclePointQM31Impl, QM31_CIRCLE_GEN};
    use stwo_cairo_verifier::fields::m31::{m31, M31};
    use stwo_cairo_verifier::fields::qm31::{qm31, QM31, QM31One};
    use stwo_cairo_verifier::utils::pow;

    #[test]
    fn test_add_1() {
        let i = CirclePoint { x: m31(0), y: m31(1) };
        let result = i + i;

        assert_eq!(result, CirclePoint { x: -m31(1), y: m31(0) });
    }

    #[test]
    fn test_add_2() {
        let point_1 = CirclePoint { x: m31(750649172), y: m31(1991648574) };
        let point_2 = CirclePoint { x: m31(1737427771), y: m31(309481134) };
        let result = point_1 + point_2;

        assert_eq!(result, CirclePoint { x: m31(1476625263), y: m31(1040927458) });
    }

    #[test]
    fn test_zero_1() {
        let result = CirclePointM31Impl::zero();

        assert_eq!(result, CirclePoint { x: m31(1), y: m31(0) });
    }

    #[test]
    fn test_zero_2() {
        let point_1 = CirclePoint { x: m31(750649172), y: m31(1991648574) };
        let point_2 = CirclePointM31Impl::zero();
        let result = point_1 + point_2;

        assert_eq!(result, point_1.clone());
    }

    #[test]
    fn test_mul_1() {
        let point_1 = CirclePoint { x: m31(750649172), y: m31(1991648574) };
        let mut scalar = 5;
        let result = point_1.mul(ref scalar);

        assert_eq!(result, point_1 + point_1 + point_1 + point_1 + point_1);
    }

    #[test]
    fn test_mul_2() {
        let point_1 = CirclePoint { x: m31(750649172), y: m31(1991648574) };
        let mut scalar = 8;
        let result = point_1.mul(ref scalar);

        assert_eq!(
            result, point_1 + point_1 + point_1 + point_1 + point_1 + point_1 + point_1 + point_1
        );
    }

    #[test]
    fn test_mul_3() {
        let point_1 = CirclePoint { x: m31(750649172), y: m31(1991648574) };
        let mut scalar = 418776494;
        let result = point_1.mul(ref scalar);

        assert_eq!(result, CirclePoint { x: m31(1987283985), y: m31(1500510905) });
    }

    #[test]
    fn test_generator_order() {
        let half_order = CIRCLE_ORDER / 2;
        let mut scalar = half_order.into();
        let mut result = M31_CIRCLE_GEN.mul(ref scalar);

        // Assert `M31_CIRCLE_GEN^{2^30}` equals `-1`.
        assert_eq!(result, CirclePoint { x: -m31(1), y: m31(0) });
    }

    #[test]
    fn test_generator() {
        let mut scalar = pow(2, 30).try_into().unwrap();
        let mut result = M31_CIRCLE_GEN.mul(ref scalar);

        assert_eq!(result, CirclePoint { x: -m31(1), y: m31(0) });
    }

    #[test]
    fn test_coset_index_at() {
        let coset = Coset { initial_index: 16777216, log_size: 5, step_size: 67108864 };
        let result = coset.index_at(8);

        assert_eq!(result, 553648128);
    }

    #[test]
    fn test_coset_constructor() {
        let result = CosetImpl::new(16777216, 5);

        assert_eq!(result, Coset { initial_index: 16777216, log_size: 5, step_size: 67108864 });
    }

    #[test]
    fn test_coset_double() {
        let coset = Coset { initial_index: 16777216, step_size: 67108864, log_size: 5 };
        let result = coset.double();

        assert_eq!(result, Coset { initial_index: 33554432, step_size: 134217728, log_size: 4 });
    }

    #[test]
    fn test_coset_at() {
        let coset = Coset { initial_index: 16777216, step_size: 67108864, log_size: 5 };
        let result = coset.at(17);

        assert_eq!(result, CirclePoint { x: m31(7144319), y: m31(1742797653) });
    }

    #[test]
    fn test_coset_size() {
        let coset = Coset { initial_index: 16777216, step_size: 67108864, log_size: 5 };
        let result = coset.size();

        assert_eq!(result, 32);
    }

    #[test]
    fn test_qm31_circle_gen() {
        let P4: u128 = 21267647892944572736998860269687930881;

        let first_prime = 2;
        let last_prime = 368140581013;
        let prime_factors: Array<(u128, u32)> = array![
            (first_prime, 33),
            (3, 2),
            (5, 1),
            (7, 1),
            (11, 1),
            (31, 1),
            (151, 1),
            (331, 1),
            (733, 1),
            (1709, 1),
            (last_prime, 1),
        ];

        let product = iter_product(first_prime, @prime_factors, last_prime);

        assert_eq!(product, P4 - 1);

        assert_eq!(
            QM31_CIRCLE_GEN.x * QM31_CIRCLE_GEN.x + QM31_CIRCLE_GEN.y * QM31_CIRCLE_GEN.y,
            QM31One::one()
        );

        let mut scalar = P4 - 1;
        assert_eq!(QM31_CIRCLE_GEN.mul(ref scalar), CirclePointQM31Impl::zero());

        let mut i = 0;
        while i < prime_factors.len() {
            let (p, _) = *prime_factors.at(i);
            let mut scalar = (P4 - 1) / p.into();
            assert_ne!(QM31_CIRCLE_GEN.mul(ref scalar), CirclePointQM31Impl::zero());

            i = i + 1;
        }
    }

    fn iter_product(
        first_prime: u128, prime_factors: @Array<(u128, u32)>, last_prime: u128
    ) -> u128 {
        let mut accum_product: u128 = 1;
        accum_product = accum_product
            * pow(first_prime.try_into().unwrap(), 31).into()
            * 4; // * 2^33
        let mut i = 1;
        while i < prime_factors.len() - 1 {
            let (prime, exponent): (u128, u32) = *prime_factors.at(i);
            accum_product = accum_product * pow(prime.try_into().unwrap(), exponent).into();
            i = i + 1;
        };
        accum_product = accum_product * last_prime;
        accum_product
    }
}

