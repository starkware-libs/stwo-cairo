use super::super::super::{M31, QM31, PackedUnreducedQM31Trait};
use super::super::super::super::cm31::CM31;
use super::super::super::super::m31::M31Trait;
use core::ops::{AddAssign};

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

#[generate_trait]
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


/// Stores an unreduced [`QM31`] packed into two `felt252`.
///
/// This is used for PackedUnreducedQM31Trait::large_zero() + \sum PackedUnreducedQM31 * m31.
///
/// This more efficient than [`UnreducedQM31`] for the case above since it requires two (rather than
/// 4) felt252 operations per addition or M31 multiplication.
#[derive(Copy, Drop, Debug)]
pub struct PackedUnreducedQM31 {
    pub a: PackedUnreducedCM31,
    pub b: PackedUnreducedCM31,
}

pub impl PackedUnreducedQM31Impl of PackedUnreducedQM31Trait {
    #[inline]
    fn mul_m31(self: PackedUnreducedQM31, rhs: M31) -> PackedUnreducedQM31 {
        PackedUnreducedQM31 { a: self.a.mul_m31(rhs), b: self.b.mul_m31(rhs) }
    }

    /// Returns a zero element with each coordinate set to `P*P*P`.
    #[inline]
    fn large_zero() -> PackedUnreducedQM31 {
        PackedUnreducedQM31 {
            a: PackedUnreducedCM31Trait::large_zero(), b: PackedUnreducedCM31Trait::large_zero(),
        }
    }

    #[inline]
    fn reduce(self: PackedUnreducedQM31) -> QM31 {
        QM31 { a: self.a.reduce(), b: self.b.reduce() }
    }
}

pub impl PackedUnreducedQM31AddAssign of AddAssign<PackedUnreducedQM31, PackedUnreducedQM31> {
    #[inline]
    fn add_assign(ref self: PackedUnreducedQM31, rhs: PackedUnreducedQM31) {
        self = self + rhs
    }
}

pub impl PackedUnreducedQM31Add of Add<PackedUnreducedQM31> {
    #[inline]
    fn add(lhs: PackedUnreducedQM31, rhs: PackedUnreducedQM31) -> PackedUnreducedQM31 {
        PackedUnreducedQM31 { a: lhs.a + rhs.a, b: lhs.b + rhs.b }
    }
}

pub impl PackedUnreducedQM31Sub of Sub<PackedUnreducedQM31> {
    #[inline]
    fn sub(lhs: PackedUnreducedQM31, rhs: PackedUnreducedQM31) -> PackedUnreducedQM31 {
        PackedUnreducedQM31 { a: lhs.a - rhs.a, b: lhs.b - rhs.b }
    }
}

pub impl QM31IntoPackedUnreducedQM31 of Into<QM31, PackedUnreducedQM31> {
    #[inline]
    fn into(self: QM31) -> PackedUnreducedQM31 {
        PackedUnreducedQM31 { a: self.a.into(), b: self.b.into() }
    }
}