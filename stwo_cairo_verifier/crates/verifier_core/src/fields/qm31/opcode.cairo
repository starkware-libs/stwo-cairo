//! Implementation of QM31 field using the QM31 opcode.
use core::num::traits::{One, Zero};
use core::ops::{AddAssign, MulAssign, SubAssign};
use super::super::Invertible;
use super::super::cm31::{CM31, CM31Trait, MulByCM31Trait};
use super::super::m31::{M31, M31InnerT, M31Trait, MulByM31Trait};
// TODO(Gali): Remove.
#[allow(unused_imports)]
use super::{PackedUnreducedQM31Trait, QM31Display, QM31Trait, QM31_EXTENSION_DEGREE};

#[derive(Copy, Drop, PartialEq)]
pub struct QM31 {
    inner: core::qm31::qm31,
}

impl QM31MulByM31Impl of MulByM31Trait<QM31> {
    #[inline]
    fn mul_m31(self: QM31, rhs: M31) -> QM31 {
        self * rhs.into()
    }
}

impl QM31MulByCM31Impl of MulByCM31Trait<QM31> {
    #[inline]
    fn mul_cm31(self: QM31, rhs: CM31) -> QM31 {
        self * rhs.into()
    }
}

pub impl QM31Impl of QM31Trait {
    #[inline]
    fn from_fixed_array(arr: [M31; QM31_EXTENSION_DEGREE]) -> QM31 {
        let [a, b, c, d] = arr;
        QM31 { inner: core::qm31::QM31Trait::new(a.inner, b.inner, c.inner, d.inner) }
    }

    #[inline]
    fn to_fixed_array(self: QM31) -> [M31; QM31_EXTENSION_DEGREE] {
        let [a, b, c, d] = core::qm31::QM31Trait::unpack(self.inner);
        [M31Trait::new(a), M31Trait::new(b), M31Trait::new(c), M31Trait::new(d)]
    }

    #[inline]
    fn complex_conjugate(self: QM31) -> QM31 {
        let [a, b, c, d] = self.to_fixed_array();
        Self::from_fixed_array([a, b, -c, -d])
    }

    #[inline]
    fn fused_mul_add(a: QM31, b: QM31, c: QM31) -> QM31 {
        QM31 { inner: a.inner * b.inner + c.inner }
    }

    #[inline]
    fn fused_mul_sub(a: QM31, b: QM31, c: QM31) -> QM31 {
        QM31 { inner: a.inner * b.inner - c.inner }
    }

    #[inline]
    fn fused_quotient_denominator(px: QM31, py: QM31, dx: M31, dy: M31) -> CM31 {
        let [_, _, c, d] = ((px - dx.into()).complex_conjugate() * (py - dy.into()))
            .to_fixed_array();
        CM31Trait::pack(c, d)
    }

    fn from_partial_evals(evals: [QM31; QM31_EXTENSION_DEGREE]) -> QM31 {
        let [e0, e1, e2, e3] = evals;
        e0
            + e1 * qm31_const::<0, 1, 0, 0>()
            + e2 * qm31_const::<0, 0, 1, 0>()
            + e3 * qm31_const::<0, 0, 0, 1>()
    }
}

pub impl QM31Add of core::traits::Add<QM31> {
    #[inline(always)]
    fn add(lhs: QM31, rhs: QM31) -> QM31 {
        QM31 { inner: lhs.inner + rhs.inner }
    }
}

pub impl QM31Sub of core::traits::Sub<QM31> {
    #[inline(always)]
    fn sub(lhs: QM31, rhs: QM31) -> QM31 {
        QM31 { inner: lhs.inner - rhs.inner }
    }
}

pub impl QM31Mul of core::traits::Mul<QM31> {
    #[inline(always)]
    fn mul(lhs: QM31, rhs: QM31) -> QM31 {
        QM31 { inner: lhs.inner * rhs.inner }
    }
}

pub impl QM31Div of core::traits::Div<QM31> {
    #[inline(always)]
    fn div(lhs: QM31, rhs: QM31) -> QM31 {
        QM31 { inner: lhs.inner / rhs.inner }
    }
}

pub impl QM31AddAssign of AddAssign<QM31, QM31> {
    #[inline(always)]
    fn add_assign(ref self: QM31, rhs: QM31) {
        self = self + rhs
    }
}

pub impl QM31SubAssign of SubAssign<QM31, QM31> {
    #[inline(always)]
    fn sub_assign(ref self: QM31, rhs: QM31) {
        self = self - rhs
    }
}

pub impl QM31MulAssign of MulAssign<QM31, QM31> {
    #[inline(always)]
    fn mul_assign(ref self: QM31, rhs: QM31) {
        self = self * rhs
    }
}

// TODO: Use NonZero<QM31>
impl QM31InvertibleImpl of Invertible<QM31> {
    #[inline(always)]
    fn inverse(self: QM31) -> QM31 {
        qm31_const::<1, 0, 0, 0>() / self
    }
}

pub impl QM31Zero of Zero<QM31> {
    #[inline]
    fn zero() -> QM31 {
        qm31_const::<0, 0, 0, 0>()
    }

    #[inline]
    fn is_zero(self: @QM31) -> bool {
        self.inner.is_zero()
    }

    #[inline]
    fn is_non_zero(self: @QM31) -> bool {
        self.inner.is_non_zero()
    }
}

pub impl QM31One of One<QM31> {
    #[inline]
    fn one() -> QM31 {
        qm31_const::<1, 0, 0, 0>()
    }

    fn is_one(self: @QM31) -> bool {
        @core::qm31::qm31_const::<1, 0, 0, 0>() == self.inner
    }

    fn is_non_one(self: @QM31) -> bool {
        @core::qm31::qm31_const::<1, 0, 0, 0>() != self.inner
    }
}
pub impl M31IntoQM31 of core::traits::Into<M31, QM31> {
    #[inline]
    fn into(self: M31) -> QM31 {
        QM31 { inner: self.inner.into() }
    }
}

pub impl QM31Neg of Neg<QM31> {
    #[inline]
    fn neg(a: QM31) -> QM31 {
        qm31_const::<0, 0, 0, 0>() - a
    }
}


// Alias PackedUnreducedQM31 to QM31, since QM31 provides the most efficient implementation
// when QM31 opcodes are available.
pub type PackedUnreducedQM31 = QM31;

pub impl PackedUnreducedQM31Impl of PackedUnreducedQM31Trait {
    #[inline]
    fn large_zero() -> PackedUnreducedQM31 {
        Zero::zero()
    }

    #[inline]
    fn reduce(self: PackedUnreducedQM31) -> QM31 {
        self
    }

    #[inline]
    fn packed_fused_mul_add(a: PackedUnreducedQM31, b: QM31, c: PackedUnreducedQM31) -> QM31 {
        QM31Trait::fused_mul_add(a, b, c)
    }
}

impl QM31Debug of core::fmt::Debug<QM31> {
    fn fmt(self: @QM31, ref f: core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        QM31Display::fmt(self, ref f)
    }
}

#[inline(always)]
pub fn qm31_const<
    const W0: M31InnerT, const W1: M31InnerT, const W2: M31InnerT, const W3: M31InnerT,
>() -> QM31 nopanic {
    QM31 { inner: core::qm31::qm31_const::<W0, W1, W2, W3>() }
}
