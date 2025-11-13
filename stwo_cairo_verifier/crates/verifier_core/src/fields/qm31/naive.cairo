//! Software only implementation of QM31 field (i.e no QM31 opcode).
use core::num::traits::one::One;
use core::num::traits::zero::Zero;
use core::ops::{AddAssign, MulAssign, SubAssign};
use super::super::Invertible;
use super::super::cm31::{CM31, MulByCM31Trait};
use super::super::m31::{M31, M31InnerT, MulByM31Trait};
// TODO(Gali): Remove.
#[allow(unused_imports)]
use super::{QM31Trait, QM31_EXTENSION_DEGREE};

mod unreduced;

pub use unreduced::{
    PackedQM31byM31Impl, PackedUnreducedQM31, PackedUnreducedQM31Impl, mul_cm_using_unreduced,
};

// Represents a + u*b.
#[derive(Copy, Drop, Debug, PartialEq)]
pub struct QM31 {
    a: CM31,
    b: CM31,
}

impl QM31InvertibleImpl of Invertible<QM31> {
    fn inverse(self: QM31) -> QM31 {
        let b2 = self.b * self.b;
        let ib2 = CM31 { a: -b2.b, b: b2.a };
        let denom = self.a * self.a - (b2 + b2 + ib2);
        let denom_inverse = denom.inverse();
        QM31 { a: self.a, b: -self.b }.mul_cm31(denom_inverse)
    }
}

pub impl QM31MulByM31Impl of MulByM31Trait<QM31> {
    #[inline]
    fn mul_m31(self: QM31, rhs: M31) -> QM31 {
        QM31 { a: self.a.mul_m31(rhs), b: self.b.mul_m31(rhs) }
    }
}

pub impl QM31MulByCM31Impl of MulByCM31Trait<QM31> {
    #[inline]
    fn mul_cm31(self: QM31, rhs: CM31) -> QM31 {
        QM31 { a: self.a * rhs, b: self.b * rhs }
    }
}

pub impl QM31Impl of QM31Trait {
    #[inline]
    fn from_fixed_array(arr: [M31; QM31_EXTENSION_DEGREE]) -> QM31 {
        let [a, b, c, d] = arr;
        QM31 { a: CM31 { a: a, b: b }, b: CM31 { a: c, b: d } }
    }

    #[inline]
    fn to_fixed_array(self: QM31) -> [M31; QM31_EXTENSION_DEGREE] {
        [self.a.a, self.a.b, self.b.a, self.b.b]
    }

    fn complex_conjugate(self: QM31) -> QM31 {
        QM31 { a: self.a, b: -self.b }
    }

    #[inline]
    fn fused_mul_add(a: QM31, b: QM31, c: QM31) -> QM31 {
        unreduced::fused_mul_add(a, b, c)
    }

    #[inline]
    fn fused_mul_sub(a: QM31, b: QM31, c: QM31) -> QM31 {
        unreduced::fused_mul_sub(a, b, c)
    }

    #[inline]
    fn fused_quotient_denominator(px: QM31, py: QM31, dx: M31, dy: M31) -> CM31 {
        unreduced::fused_quotient_denominator(px, py, dx, dy)
    }

    fn from_partial_evals(evals: [QM31; QM31_EXTENSION_DEGREE]) -> QM31 {
        unreduced::from_partial_evals(evals)
    }
}

pub impl QM31Add of core::traits::Add<QM31> {
    #[inline(never)]
    fn add(lhs: QM31, rhs: QM31) -> QM31 {
        QM31 { a: lhs.a + rhs.a, b: lhs.b + rhs.b }
    }
}

pub impl QM31Sub of core::traits::Sub<QM31> {
    #[inline(never)]
    fn sub(lhs: QM31, rhs: QM31) -> QM31 {
        QM31 { a: lhs.a - rhs.a, b: lhs.b - rhs.b }
    }
}

pub impl QM31Mul of core::traits::Mul<QM31> {
    #[inline(never)]
    fn mul(lhs: QM31, rhs: QM31) -> QM31 {
        unreduced::mul_qm_using_unreduced(lhs, rhs)
    }
}

pub impl QM31Div of core::traits::Div<QM31> {
    #[inline]
    fn div(lhs: QM31, rhs: QM31) -> QM31 {
        lhs * rhs.inverse()
    }
}

pub impl QM31AddAssign of AddAssign<QM31, QM31> {
    #[inline]
    fn add_assign(ref self: QM31, rhs: QM31) {
        self = self + rhs
    }
}

pub impl QM31SubAssign of SubAssign<QM31, QM31> {
    #[inline]
    fn sub_assign(ref self: QM31, rhs: QM31) {
        self = self - rhs
    }
}

pub impl QM31MulAssign of MulAssign<QM31, QM31> {
    #[inline]
    fn mul_assign(ref self: QM31, rhs: QM31) {
        self = self * rhs
    }
}

pub impl QM31Zero of Zero<QM31> {
    #[inline]
    fn zero() -> QM31 {
        QM31 { a: Zero::zero(), b: Zero::zero() }
    }

    fn is_zero(self: @QM31) -> bool {
        (*self).a.is_zero() && (*self).b.is_zero()
    }

    fn is_non_zero(self: @QM31) -> bool {
        !self.is_zero()
    }
}

pub impl QM31One of One<QM31> {
    #[inline]
    fn one() -> QM31 {
        QM31 { a: One::one(), b: Zero::zero() }
    }

    fn is_one(self: @QM31) -> bool {
        (*self).a.is_one() && (*self).b.is_zero()
    }

    fn is_non_one(self: @QM31) -> bool {
        !self.is_one()
    }
}

pub impl M31IntoQM31 of core::traits::Into<M31, QM31> {
    #[inline]
    fn into(self: M31) -> QM31 {
        QM31 { a: self.into(), b: Zero::zero() }
    }
}

pub impl CM31IntoQM31 of core::traits::Into<CM31, QM31> {
    #[inline]
    fn into(self: CM31) -> QM31 {
        QM31 { a: self, b: Zero::zero() }
    }
}

pub impl QM31Neg of Neg<QM31> {
    #[inline]
    fn neg(a: QM31) -> QM31 {
        QM31 { a: -a.a, b: -a.b }
    }
}

pub fn qm31_const<
    const W0: M31InnerT, const W1: M31InnerT, const W2: M31InnerT, const W3: M31InnerT,
>() -> QM31 nopanic {
    QM31 {
        a: CM31 { a: M31 { inner: W0 }, b: M31 { inner: W1 } },
        b: CM31 { a: M31 { inner: W2 }, b: M31 { inner: W3 } },
    }
}
