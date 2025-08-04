//! Software only implementation of CM31 field (i.e no QM31 opcode).
use core::num::traits::{One, Zero};
use core::ops::{AddAssign, MulAssign, SubAssign};
use super::super::m31::{M31, M31InnerT, M31Trait, m31};
use super::super::{BatchInvertible, Invertible};
use super::{CM31Trait, PackedUnreducedCM31Trait};

#[derive(Copy, Drop, Debug, PartialEq, Serde)]
pub struct CM31 {
    pub a: M31,
    pub b: M31,
}

pub impl CM31BatchInvertibleImpl of BatchInvertible<CM31> {}

pub impl CM31InvertibleImpl of Invertible<CM31> {
    fn inverse(self: CM31) -> CM31 {
        let denom_inverse: M31 = (self.a * self.a + self.b * self.b).inverse();
        CM31 { a: self.a * denom_inverse, b: -self.b * denom_inverse }
    }
}

pub impl CM31Impl of CM31Trait {
    // TODO(andrew): When associated types are supported, support `Mul<CM31, M31>`.
    #[inline]
    fn mul_m31(self: CM31, rhs: M31) -> CM31 {
        CM31 { a: self.a * rhs, b: self.b * rhs }
    }

    // TODO(andrew): When associated types are supported, support `Sub<CM31, M31>`.
    #[inline]
    fn sub_m31(self: CM31, rhs: M31) -> CM31 {
        CM31 { a: self.a - rhs, b: self.b }
    }

    fn unpack(self: CM31) -> (M31, M31) {
        (self.a, self.b)
    }

    fn pack(a: M31, b: M31) -> CM31 {
        CM31 { a, b }
    }
}

pub impl CM31Add of core::traits::Add<CM31> {
    #[inline]
    fn add(lhs: CM31, rhs: CM31) -> CM31 {
        CM31 { a: lhs.a + rhs.a, b: lhs.b + rhs.b }
    }
}

pub impl CM31Sub of core::traits::Sub<CM31> {
    #[inline]
    fn sub(lhs: CM31, rhs: CM31) -> CM31 {
        CM31 { a: lhs.a - rhs.a, b: lhs.b - rhs.b }
    }
}

pub impl CM31Mul of core::traits::Mul<CM31> {
    #[inline]
    fn mul(lhs: CM31, rhs: CM31) -> CM31 {
        CM31 { a: lhs.a * rhs.a - lhs.b * rhs.b, b: lhs.a * rhs.b + lhs.b * rhs.a }
    }
}

pub impl CM31AddAssign of AddAssign<CM31, CM31> {
    #[inline]
    fn add_assign(ref self: CM31, rhs: CM31) {
        self = self + rhs
    }
}

pub impl CM31SubAssign of SubAssign<CM31, CM31> {
    #[inline]
    fn sub_assign(ref self: CM31, rhs: CM31) {
        self = self - rhs
    }
}

pub impl CM31MulAssign of MulAssign<CM31, CM31> {
    #[inline]
    fn mul_assign(ref self: CM31, rhs: CM31) {
        self = self * rhs
    }
}

pub impl CM31Zero of Zero<CM31> {
    fn zero() -> CM31 {
        cm31_const::<0, 0>()
    }

    fn is_zero(self: @CM31) -> bool {
        (*self).a.is_zero() && (*self).b.is_zero()
    }

    fn is_non_zero(self: @CM31) -> bool {
        !self.is_zero()
    }
}

pub impl CM31One of One<CM31> {
    fn one() -> CM31 {
        cm31_const::<1, 0>()
    }
    fn is_one(self: @CM31) -> bool {
        (*self).a.is_one() && (*self).b.is_zero()
    }
    fn is_non_one(self: @CM31) -> bool {
        !self.is_one()
    }
}

pub impl M31IntoCM31 of core::traits::Into<M31, CM31> {
    #[inline]
    fn into(self: M31) -> CM31 {
        CM31 { a: self, b: m31(0) }
    }
}

pub impl CM31Neg of Neg<CM31> {
    #[inline]
    fn neg(a: CM31) -> CM31 {
        CM31 { a: -a.a, b: -a.b }
    }
}

/// A packed representation of an unreduced [`CM31`] as a single `felt252`.
///
/// This format enables performing two arithmetic operations (multiplication, addition,
/// or subtraction) simultaneously. However, explicit packing and unpacking steps are required
/// to convert between `CM31` and `PackedUnreducedCM31`.
#[derive(Copy, Drop, Debug)]
pub struct PackedUnreducedCM31 {
    /// Encodes two potentially unreduced M31 elements within a single `felt252`.
    /// Stored as `a + (b << 128)`.
    ///
    /// Bounds:
    /// - `a` must be in the range [0, 2^128).
    /// - `b` must be in the range [0, 2^123 + 17 * 2^64).
    pub inner: felt252,
}

pub impl PackedUnreducedCM31Impl of PackedUnreducedCM31Trait {
    /// Multiplies a [`PackedUnreducedCM31`] by an [`M31`], returning a new [`PackedUnreducedCM31`].
    ///
    /// Typically, both operands are reduced 31-bit elements, yielding a 62-bit result.
    /// The packed form of `self` allows two multiplications to be executed concurrently.
    #[inline]
    fn mul_m31(self: PackedUnreducedCM31, rhs: M31) -> PackedUnreducedCM31 {
        PackedUnreducedCM31 { inner: self.inner * rhs.inner.into() }
    }

    /// Returns a zero element with each coordinate set to `P*P*P`.
    ///
    /// Using [`large_zero`] instead of direct zero initialization prevents underflow during
    /// subtraction operations. Starting from [`large_zero`], it's safe to perform up to `P`
    /// additions or subtractions of results produced by [`mul_m31`] without risking overflow or
    /// underflow.
    #[inline]
    fn large_zero() -> PackedUnreducedCM31 {
        // Stores `P*P*P + (P*P*P << 128)`.
        const PPP_PPP: felt252 = 0x1fffffff400000017fffffff000000001fffffff400000017fffffff;
        PackedUnreducedCM31 { inner: PPP_PPP }
    }

    #[inline]
    fn reduce(self: PackedUnreducedCM31) -> CM31 {
        let u256 { low: a, high: b } = self.inner.into();
        CM31 { a: M31Trait::reduce_u128(a).into(), b: M31Trait::reduce_u128(b).into() }
    }
}

pub impl PackedUnreducedCM31Add of Add<PackedUnreducedCM31> {
    #[inline]
    fn add(lhs: PackedUnreducedCM31, rhs: PackedUnreducedCM31) -> PackedUnreducedCM31 {
        PackedUnreducedCM31 { inner: lhs.inner + rhs.inner }
    }
}

pub impl PackedUnreducedCM31Sub of Sub<PackedUnreducedCM31> {
    /// Subtracts two [`PackedUnreducedCM31`] elements, returning a new [`PackedUnreducedCM31`].
    /// Note that there is a potential underflow here, `large_zero` should be used to prevent this.
    #[inline]
    fn sub(lhs: PackedUnreducedCM31, rhs: PackedUnreducedCM31) -> PackedUnreducedCM31 {
        PackedUnreducedCM31 { inner: lhs.inner - rhs.inner }
    }
}

pub impl CM31IntoPackedUnreducedCM31 of Into<CM31, PackedUnreducedCM31> {
    #[inline]
    fn into(self: CM31) -> PackedUnreducedCM31 {
        const POW2_128: felt252 = 0x100000000000000000000000000000000;
        let a_felt: felt252 = self.a.into();
        let b_felt: felt252 = self.b.into();
        PackedUnreducedCM31 { inner: a_felt + b_felt * POW2_128 }
    }
}

#[inline]
pub fn cm31_const<const W0: M31InnerT, const W1: M31InnerT>() -> CM31 nopanic {
    CM31 { a: M31 { inner: W0 }, b: M31 { inner: W1 } }
}
