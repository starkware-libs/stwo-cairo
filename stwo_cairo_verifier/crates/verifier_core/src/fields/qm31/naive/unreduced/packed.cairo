use core::ops::{AddAssign, SubAssign};
use super::super::super::super::cm31::{CM31, MulByCM31Trait};
use super::super::super::super::m31::{AddM31Trait, M31Trait, MulByM31Trait};
use super::super::super::{M31, PackedUnreducedQM31Trait, QM31};

pub type WM31 = (M31, M31);

/// A packed representation of an unreduced [`WM31`] as a single `felt252`.
///
/// This format enables performing two arithmetic operations (multiplication, addition,
/// or subtraction) simultaneously. However, explicit packing and unpacking steps are required
/// to convert between `WM31` and `PackedUnreducedWM31`.
#[derive(Copy, Drop, Debug)]
pub struct PackedUnreducedWM31 {
    /// Encodes two potentially unreduced M31 elements within a single `felt252`.
    /// Stored as `a + (b << 128)`.
    ///
    /// Bounds:
    /// - `a` must be in the range [0, 2^128).
    /// - `b` must be in the range [0, 2^123 + 17 * 2^64).
    pub inner: felt252,
}

pub impl PackedWM31byM31Impl of MulByM31Trait<PackedUnreducedWM31> {
    /// Multiplies a [`PackedUnreducedWM31`] by an [`M31`], returning a new [`PackedUnreducedWM31`].
    ///
    /// Typically, both operands are reduced 31-bit elements, yielding a 62-bit result.
    /// The packed form of `self` allows two multiplications to be executed concurrently.
    #[inline]
    fn mul_m31(self: PackedUnreducedWM31, rhs: M31) -> PackedUnreducedWM31 {
        PackedUnreducedWM31 { inner: self.inner * rhs.inner.into() }
    }
}

pub impl PackedWM31AddM31Impl of AddM31Trait<PackedUnreducedWM31> {
    #[inline]
    fn add_m31(self: PackedUnreducedWM31, rhs: M31) -> PackedUnreducedWM31 {
        PackedUnreducedWM31 { inner: self.inner + rhs.into() }
    }
}

#[generate_trait]
pub impl PackedUnreducedWM31Impl of PackedUnreducedWM31Trait {
    /// Returns a zero element with each coordinate set to `P*P*P`.
    ///
    /// Using [`large_zero`] instead of direct zero initialization prevents underflow during
    /// subtraction operations. Starting from [`large_zero`], it's safe to perform up to `2^16*P`
    /// additions or subtractions of results produced by [`mul_m31`] and [`cmul_31`] without risking
    /// overflow or underflow, as well as up to 2^12 additions or subtractions of results produced
    /// by repeated applications of such muls.
    #[inline]
    fn large_zero() -> PackedUnreducedWM31 {
        // Stores `2^16*P*P*P + (2^16*P*P*P << 128)`.
        const PPP_PPP: felt252 = 0x1fffffff400000017fffffff000000001fffffff400000017fffffff0000;
        PackedUnreducedWM31 { inner: PPP_PPP }
    }

    #[inline]
    fn reduce(self: PackedUnreducedWM31) -> WM31 {
        let u256 { low: a, high: b } = self.inner.into();
        (M31Trait::reduce_u128(a).into(), M31Trait::reduce_u128(b).into())
    }
}

pub impl PackedUnreducedWM31Add of Add<PackedUnreducedWM31> {
    #[inline]
    fn add(lhs: PackedUnreducedWM31, rhs: PackedUnreducedWM31) -> PackedUnreducedWM31 {
        PackedUnreducedWM31 { inner: lhs.inner + rhs.inner }
    }
}

pub impl PackedUnreducedWM31Sub of Sub<PackedUnreducedWM31> {
    /// Subtracts two [`PackedUnreducedWM31`] elements, returning a new [`PackedUnreducedWM31`].
    /// Note that there is a potential underflow here, `large_zero` should be used to prevent this.
    #[inline]
    fn sub(lhs: PackedUnreducedWM31, rhs: PackedUnreducedWM31) -> PackedUnreducedWM31 {
        PackedUnreducedWM31 { inner: lhs.inner - rhs.inner }
    }
}

pub impl WM31IntoPackedUnreducedWM31 of Into<WM31, PackedUnreducedWM31> {
    #[inline]
    fn into(self: WM31) -> PackedUnreducedWM31 {
        const POW2_128: felt252 = 0x100000000000000000000000000000000;
        let (a, b) = self;
        let a_felt: felt252 = a.into();
        let b_felt: felt252 = b.into();
        PackedUnreducedWM31 { inner: a_felt + b_felt * POW2_128 }
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
    pub a: PackedUnreducedWM31,
    pub b: PackedUnreducedWM31,
}

pub impl PackedQM31byM31Impl of MulByM31Trait<PackedUnreducedQM31> {
    #[inline]
    fn mul_m31(self: PackedUnreducedQM31, rhs: M31) -> PackedUnreducedQM31 {
        PackedUnreducedQM31 { a: self.a.mul_m31(rhs), b: self.b.mul_m31(rhs) }
    }
}

pub impl PackedQM31byCM31Impl of MulByCM31Trait<PackedUnreducedQM31> {
    #[inline]
    fn mul_cm31(self: PackedUnreducedQM31, rhs: CM31) -> PackedUnreducedQM31 {
        let (a, b) = (self.a, self.b);
        let (x, y) = (rhs.a, rhs.b);

        PackedUnreducedQM31 { a: a.mul_m31(x) - b.mul_m31(y), b: a.mul_m31(y) + b.mul_m31(x) }
    }
}

pub impl PackedQM31AddM31Impl of AddM31Trait<PackedUnreducedQM31> {
    #[inline]
    fn add_m31(self: PackedUnreducedQM31, rhs: M31) -> PackedUnreducedQM31 {
        PackedUnreducedQM31 { a: self.a.add_m31(rhs), b: self.b }
    }
}

pub impl PackedUnreducedQM31Impl of PackedUnreducedQM31Trait {
    /// Returns a zero element with each coordinate set to `P*P*P`.
    #[inline]
    fn large_zero() -> PackedUnreducedQM31 {
        PackedUnreducedQM31 {
            a: PackedUnreducedWM31Trait::large_zero(), b: PackedUnreducedWM31Trait::large_zero(),
        }
    }

    #[inline]
    fn reduce(self: PackedUnreducedQM31) -> QM31 {
        let (a, c) = self.a.reduce();
        let (b, d) = self.b.reduce();
        QM31 { a: CM31 { a: a, b: b }, b: CM31 { a: c, b: d } }
    }
}

pub impl PackedUnreducedQM31AddAssign of AddAssign<PackedUnreducedQM31, PackedUnreducedQM31> {
    #[inline]
    fn add_assign(ref self: PackedUnreducedQM31, rhs: PackedUnreducedQM31) {
        self = self + rhs
    }
}

pub impl PackedUnreducedQM31SubAssign of SubAssign<PackedUnreducedQM31, PackedUnreducedQM31> {
    #[inline]
    fn sub_assign(ref self: PackedUnreducedQM31, rhs: PackedUnreducedQM31) {
        self = self - rhs
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
        let ((a, b), (c, d)) = ((self.a.a, self.a.b), (self.b.a, self.b.b));
        PackedUnreducedQM31 { a: (a, c).into(), b: (b, d).into() }
    }
}
