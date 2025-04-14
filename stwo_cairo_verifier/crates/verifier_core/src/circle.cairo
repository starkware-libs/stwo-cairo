use core::num::traits::{One, WrappingAdd, WrappingMul, WrappingSub, Zero};
use crate::channel::{Channel, ChannelTrait};
use crate::circle_mul_table::{
    M31_CIRCLE_GEN_MUL_TABLE_BITS_0_TO_5, M31_CIRCLE_GEN_MUL_TABLE_BITS_12_TO_17,
    M31_CIRCLE_GEN_MUL_TABLE_BITS_18_TO_23, M31_CIRCLE_GEN_MUL_TABLE_BITS_24_TO_29,
    M31_CIRCLE_GEN_MUL_TABLE_BITS_6_TO_11,
};
use crate::fields::Invertible;
use crate::fields::cm31::CM31;
use crate::fields::m31::{M31, M31Impl};
use crate::fields::qm31::{P4, QM31, QM31Trait};
use super::utils::pow2;

/// A generator for the circle group over [`M31`].
pub const M31_CIRCLE_GEN: CirclePoint<M31> = CirclePoint {
    x: M31 { inner: 0x2 }, y: M31 { inner: 0x4B94532F },
};

pub const M31_CIRCLE_LOG_ORDER: u32 = 31;

/// Equals `2^31`.
pub const M31_CIRCLE_ORDER: u32 = 0x80000000;

/// Equals `2^31 - 1`.
pub const M31_CIRCLE_ORDER_BIT_MASK: u32 = 0x7fffffff;

/// A point on the complex circle. Treated as an additive group.
#[derive(Drop, Copy, Debug, PartialEq)]
pub struct CirclePoint<F> {
    pub x: F,
    pub y: F,
}

pub trait CirclePointTrait<
    F, +Add<F>, +Sub<F>, +Mul<F>, +Drop<F>, +Copy<F>, +Zero<F>, +One<F>, +PartialEq<F>,
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
        self: @CirclePoint<F>,
    ) -> u32 {
        // we only need the x-coordinate to check order since the only point
        // with x=1 is the circle's identity
        let mut res = 0;
        let mut cur = self.x.clone();
        while cur != One::one() {
            cur = Self::double_x(cur);
            res += 1;
        }
        res
    }
}

impl CirclePointAdd<F, +Add<F>, +Sub<F>, +Mul<F>, +Drop<F>, +Copy<F>> of Add<CirclePoint<F>> {
    /// Performs the operation of the circle as a group with additive notation.
    #[inline]
    fn add(lhs: CirclePoint<F>, rhs: CirclePoint<F>) -> CirclePoint<F> {
        CirclePoint { x: lhs.x * rhs.x - lhs.y * rhs.y, y: lhs.x * rhs.y + lhs.y * rhs.x }
    }
}

impl CirclePointNeg<F, +Neg<F>, +Drop<F>, +Copy<F>> of Neg<CirclePoint<F>> {
    fn neg(a: CirclePoint<F>) -> CirclePoint<F> {
        CirclePoint { x: a.x, y: -a.y }
    }
}

pub impl CirclePointM31Impl of CirclePointTrait<M31> {}

#[generate_trait]
pub impl CirclePointQM31AddCirclePointM31Impl of CirclePointQM31AddCirclePointM31Trait {
    fn add_circle_point_m31(self: CirclePoint<QM31>, rhs: CirclePoint<M31>) -> CirclePoint<QM31> {
        CirclePoint {
            x: self.x.mul_m31(rhs.x) - self.y.mul_m31(rhs.y),
            y: self.x.mul_m31(rhs.y) + self.y.mul_m31(rhs.x),
        }
    }
}

pub impl CirclePointQM31Impl of CirclePointTrait<QM31> {}

#[generate_trait]
pub impl ChannelGetRandomCirclePointImpl of ChannelGetRandomCirclePointTrait {
    /// Returns a random QM31 circle point.
    fn get_random_point(ref self: Channel) -> CirclePoint<QM31> {
        let t = self.draw_felt();
        let t_squared = t * t;
        let t_squared_plus_1_inv = (t_squared + One::one()).inverse();
        let x = (One::one() - t_squared) * t_squared_plus_1_inv;
        let y = (t + t) * t_squared_plus_1_inv;
        CirclePoint { x, y }
    }
}

impl CirclePointQM31PartialOrd of PartialOrd<CirclePoint<QM31>> {
    fn lt(lhs: CirclePoint<QM31>, rhs: CirclePoint<QM31>) -> bool {
        lhs.x < rhs.x || (lhs.x == rhs.x && lhs.y < rhs.y)
    }
}

#[generate_trait]
pub impl ComplexConjugateImpl of ComplexConjugateTrait {
    fn complex_conjugate(self: CirclePoint<QM31>) -> CirclePoint<QM31> {
        CirclePoint { x: self.x.complex_conjugate(), y: self.y.complex_conjugate() }
    }
}

/// Represents the coset `initial + <step>`.
#[derive(Copy, Clone, Debug, PartialEq, Drop)]
pub struct Coset {
    pub initial_index: CirclePointIndex,
    pub step_size: CirclePointIndex,
    pub log_size: u32,
}

#[generate_trait]
pub impl CosetImpl of CosetTrait {
    fn new(initial_index: CirclePointIndex, log_size: u32) -> Coset {
        let step_size = CirclePointIndexImpl::subgroup_gen(log_size);
        Coset { initial_index, step_size, log_size }
    }

    fn index_at(self: @Coset, index: usize) -> CirclePointIndex {
        *self.initial_index + self.step_size.mul(index)
    }

    fn double(self: @Coset) -> Coset {
        assert!(*self.log_size > 0);
        Coset {
            initial_index: *self.initial_index + *self.initial_index,
            step_size: *self.step_size + *self.step_size,
            log_size: *self.log_size - 1,
        }
    }

    #[inline]
    fn at(self: @Coset, index: usize) -> CirclePoint<M31> {
        self.index_at(index).to_point()
    }

    /// Returns the size of the coset.
    fn size(self: @Coset) -> usize {
        pow2(*self.log_size)
    }

    /// Creates a coset of the form `G_2n + <G_n>`.
    ///
    /// For example, for `n=8`, we get the point indices `[1,3,5,7,9,11,13,15]`.
    fn odds(log_size: u32) -> Coset {
        Self::new(CirclePointIndexImpl::subgroup_gen(log_size + 1), log_size)
    }

    /// Creates a coset of the form `G_4n + <G_n>`.
    ///
    /// For example, for `n=8`, we get the point indices `[1,5,9,13,17,21,25,29]`.
    /// Its conjugate will be `[3,7,11,15,19,23,27,31]`.
    fn half_odds(log_size: u32) -> Coset {
        Self::new(CirclePointIndexImpl::subgroup_gen(log_size + 2), log_size)
    }
}

/// Integer `i` that represent the circle point `i * M31_CIRCLE_GEN`.
///
/// Treated as an additive ring modulo `1 << M31_CIRCLE_LOG_ORDER`.
#[derive(Copy, Debug, Drop)]
pub struct CirclePointIndex {
    /// The index, stored as an unreduced `u32` for performance reasons.
    index: u32,
}

#[generate_trait]
pub impl CirclePointIndexImpl of CirclePointIndexTrait {
    fn new(index: u32) -> CirclePointIndex {
        CirclePointIndex { index }
    }

    fn zero() -> CirclePointIndex {
        CirclePointIndex { index: 0 }
    }

    fn generator() -> CirclePointIndex {
        CirclePointIndex { index: 1 }
    }

    fn reduce(self: @CirclePointIndex) -> CirclePointIndex {
        CirclePointIndex { index: *self.index & M31_CIRCLE_ORDER_BIT_MASK }
    }

    fn subgroup_gen(log_size: u32) -> CirclePointIndex {
        assert!(log_size <= M31_CIRCLE_LOG_ORDER);
        CirclePointIndex { index: pow2(M31_CIRCLE_LOG_ORDER - log_size) }
    }

    // TODO(andrew): When associated types are supported, support the Mul<Self, u32>.
    fn mul(self: @CirclePointIndex, scalar: u32) -> CirclePointIndex {
        CirclePointIndex { index: (*self.index).wrapping_mul(scalar) }
    }

    fn index(self: @CirclePointIndex) -> u32 {
        self.reduce().index
    }

    fn to_point(self: @CirclePointIndex) -> CirclePoint<M31> {
        const NZ_2_POW_24: NonZero<u32> = 0b1000000000000000000000000;
        const NZ_2_POW_18: NonZero<u32> = 0b1000000000000000000;
        const NZ_2_POW_12: NonZero<u32> = 0b1000000000000;
        const NZ_2_POW_6: NonZero<u32> = 0b1000000;

        // No need to call `reduce()`.
        // Start with MSBs since small domains (more common) have LSBs equal 0.
        let (bits_24_to_31, bits_0_to_23) = DivRem::div_rem(*self.index, NZ_2_POW_24);
        let (bits_30_to_31, bits_24_to_29) = DivRem::div_rem(bits_24_to_31, NZ_2_POW_6);
        let mut res = *M31_CIRCLE_GEN_MUL_TABLE_BITS_24_TO_29.span()[bits_24_to_29];
        if bits_0_to_23 != 0 {
            let (bits_18_to_23, bits_0_to_17) = DivRem::div_rem(bits_0_to_23, NZ_2_POW_18);
            res = res + *M31_CIRCLE_GEN_MUL_TABLE_BITS_18_TO_23.span()[bits_18_to_23];
            if bits_0_to_17 != 0 {
                let (bits_12_to_17, bits_0_to_11) = DivRem::div_rem(bits_0_to_17, NZ_2_POW_12);
                res = res + *M31_CIRCLE_GEN_MUL_TABLE_BITS_12_TO_17.span()[bits_12_to_17];
                if bits_0_to_11 != 0 {
                    let (bits_6_to_11, bits_0_to_5) = DivRem::div_rem(bits_0_to_11, NZ_2_POW_6);
                    res = res + *M31_CIRCLE_GEN_MUL_TABLE_BITS_6_TO_11.span()[bits_6_to_11];
                    if bits_0_to_5 != 0 {
                        res = res + *M31_CIRCLE_GEN_MUL_TABLE_BITS_0_TO_5.span()[bits_0_to_5];
                    }
                }
            }
        }

        // Note this applies the appropriate transformation based on the two highest bits.
        // The highest bit has no effect. The 30th bit indicates weather to take the antipode.
        if bits_30_to_31 == 0b11 || bits_30_to_31 == 0b01 {
            res = CirclePoint { x: -res.x, y: -res.y };
        }

        res
    }
}

impl CirclePointIndexAdd of Add<CirclePointIndex> {
    #[inline]
    fn add(lhs: CirclePointIndex, rhs: CirclePointIndex) -> CirclePointIndex {
        CirclePointIndex { index: lhs.index.wrapping_add(rhs.index) }
    }
}

impl CirclePointIndexNeg of Neg<CirclePointIndex> {
    #[inline]
    fn neg(a: CirclePointIndex) -> CirclePointIndex {
        CirclePointIndex { index: M31_CIRCLE_ORDER.wrapping_sub(a.index) }
    }
}

impl CirclePointIndexPartialEx of PartialEq<CirclePointIndex> {
    fn eq(lhs: @CirclePointIndex, rhs: @CirclePointIndex) -> bool {
        lhs.index() == rhs.index()
    }

    fn ne(lhs: @CirclePointIndex, rhs: @CirclePointIndex) -> bool {
        lhs.index() != rhs.index()
    }
}

#[cfg(test)]
mod tests {
    use crate::fields::m31::m31;
    use super::{
        CirclePoint, CirclePointIndex, CirclePointIndexImpl, CirclePointM31Impl,
        CirclePointQM31AddCirclePointM31Impl, CirclePointQM31Impl, Coset, CosetImpl, M31_CIRCLE_GEN,
    };


    #[test]
    fn test_to_point() {
        let index = CirclePointIndex { index: 0b01111111111111111111111111111111 };
        assert_eq!(index.to_point(), -M31_CIRCLE_GEN);
        let index = CirclePointIndex { index: 0b00111111111111111111111111111111 };
        assert_eq!(index.to_point(), CirclePoint { x: -M31_CIRCLE_GEN.x, y: M31_CIRCLE_GEN.y });
    }


    #[test]
    fn test_to_point_with_unreduced_index() {
        // All 32 bits are `1`.
        let index = CirclePointIndex { index: 0b11111111111111111111111111111111 };

        assert_eq!(index.to_point(), -M31_CIRCLE_GEN);
    }

    #[test]
    fn test_add_1() {
        let g4 = CirclePoint { x: m31(0), y: m31(1) };
        assert_eq!(g4 + g4, CirclePoint { x: -m31(1), y: m31(0) });
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
    fn test_coset_index_at() {
        let coset = Coset {
            initial_index: CirclePointIndexImpl::new(16777216),
            log_size: 5,
            step_size: CirclePointIndexImpl::new(67108864),
        };

        let result = coset.index_at(8);

        assert_eq!(result, CirclePointIndexImpl::new(553648128));
    }

    #[test]
    fn test_coset_constructor() {
        let result = CosetImpl::new(CirclePointIndexImpl::new(16777216), 5);

        assert_eq!(
            result,
            Coset {
                initial_index: CirclePointIndexImpl::new(16777216),
                log_size: 5,
                step_size: CirclePointIndexImpl::new(67108864),
            },
        );
    }

    #[test]
    fn test_coset_double() {
        let coset = Coset {
            initial_index: CirclePointIndexImpl::new(16777216),
            step_size: CirclePointIndexImpl::new(67108864),
            log_size: 5,
        };

        let result = coset.double();

        assert_eq!(
            result,
            Coset {
                initial_index: CirclePointIndexImpl::new(33554432),
                step_size: CirclePointIndexImpl::new(134217728),
                log_size: 4,
            },
        );
    }

    #[test]
    fn test_coset_at() {
        let coset = Coset {
            initial_index: CirclePointIndexImpl::new(16777216),
            step_size: CirclePointIndexImpl::new(67108864),
            log_size: 5,
        };

        let result = coset.at(17);

        assert_eq!(result, CirclePoint { x: m31(7144319), y: m31(1742797653) });
    }

    #[test]
    fn test_coset_size() {
        let coset = Coset {
            initial_index: CirclePointIndexImpl::new(16777216),
            step_size: CirclePointIndexImpl::new(67108864),
            log_size: 5,
        };

        let result = coset.size();

        assert_eq!(result, 32);
    }
}

