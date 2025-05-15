//! Implementation of QM31 field using the QM31 opcode.
use core::num::traits::{One, Zero};
use core::ops::{AddAssign, MulAssign, SubAssign};
use super::super::Invertible;
use super::super::cm31::CM31;
use super::super::m31::{M31, M31InnerT, M31InnerTIntoM31, M31Trait, UnreducedM31};
use super::{
    PackedUnreducedCM31Trait, PackedUnreducedQM31Trait, QM31Dispaly, QM31Trait,
    QM31_EXTENSION_DEGREE, UnreducedQM31Trait,
};

#[derive(Copy, Drop, PartialEq)]
pub struct QM31 {
    inner: core::qm31::qm31,
}

pub impl QM31Impl of QM31Trait {
    #[inline]
    fn from_array(arr: [M31; QM31_EXTENSION_DEGREE]) -> QM31 {
        let [a, b, c, d] = arr;
        QM31 { inner: core::qm31::QM31Trait::new(a.inner, b.inner, c.inner, d.inner) }
    }

    #[inline]
    fn to_array(self: QM31) -> [M31; QM31_EXTENSION_DEGREE] {
        let [a, b, c, d] = core::qm31::QM31Trait::unpack(self.inner);
        [M31Trait::new(a), M31Trait::new(b), M31Trait::new(c), M31Trait::new(d)]
    }

    #[inline]
    fn mul_m31(self: QM31, rhs: M31) -> QM31 {
        self * rhs.into()
    }

    #[inline]
    fn mul_cm31(self: QM31, rhs: CM31) -> QM31 {
        self * rhs.into()
    }

    #[inline]
    fn complex_conjugate(self: QM31) -> QM31 {
        let [a, b, c, d] = self.to_array();
        let neg_c = (-TryInto::<_, M31>::try_into(c).unwrap());
        let neg_d = (-TryInto::<_, M31>::try_into(d).unwrap());
        Self::from_array([a, b, neg_c, neg_d])
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
    fn mul_unreduced(lhs: QM31, rhs: QM31) -> UnreducedQM31 {
        UnreducedQM31 { inner: lhs * rhs }
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

pub impl CM31IntoQM31 of core::traits::Into<CM31, QM31> {
    #[inline]
    fn into(self: CM31) -> QM31 {
        self.inner
    }
}

pub impl QM31Neg of Neg<QM31> {
    #[inline]
    fn neg(a: QM31) -> QM31 {
        qm31_const::<0, 0, 0, 0>() - a
    }
}

#[derive(Copy, Drop)]
pub struct UnreducedQM31 {
    // Using QM31 directly is efficient thanks to the QM31 opcode.
    inner: QM31,
}

impl UnreducedQM31Impl of UnreducedQM31Trait {
    #[inline]
    fn reduce(self: UnreducedQM31) -> QM31 {
        self.inner
    }
}

impl UnreducedQM31Sub of Sub<UnreducedQM31> {
    #[inline]
    fn sub(lhs: UnreducedQM31, rhs: UnreducedQM31) -> UnreducedQM31 {
        UnreducedQM31 { inner: lhs.inner - rhs.inner }
    }
}

impl UnreducedQM31Add of Add<UnreducedQM31> {
    #[inline]
    fn add(lhs: UnreducedQM31, rhs: UnreducedQM31) -> UnreducedQM31 {
        UnreducedQM31 { inner: lhs.inner + rhs.inner }
    }
}

impl QM31IntoUnreducedQM31 of Into<QM31, UnreducedQM31> {
    #[inline]
    fn into(self: QM31) -> UnreducedQM31 {
        UnreducedQM31 { inner: self }
    }
}

#[derive(Copy, Drop, Debug)]
pub struct PackedUnreducedQM31 {
    // Using QM31 directly is efficient thanks to the QM31 opcode.
    inner: QM31,
}

pub impl PackedUnreducedQM31Impl of PackedUnreducedQM31Trait {
    #[inline]
    fn mul_m31(self: PackedUnreducedQM31, rhs: UnreducedM31) -> PackedUnreducedQM31 {
        PackedUnreducedQM31 { inner: self.inner * rhs.inner.into() }
    }

    /// Returns a zero element with each coordinate set to `P*P*P`.
    #[inline]
    fn large_zero() -> PackedUnreducedQM31 {
        PackedUnreducedQM31 { inner: Zero::zero() }
    }

    #[inline]
    fn reduce(self: PackedUnreducedQM31) -> QM31 {
        self.inner
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
        PackedUnreducedQM31 { inner: lhs.inner + rhs.inner }
    }
}

pub impl PackedUnreducedQM31Sub of Sub<PackedUnreducedQM31> {
    #[inline]
    fn sub(lhs: PackedUnreducedQM31, rhs: PackedUnreducedQM31) -> PackedUnreducedQM31 {
        PackedUnreducedQM31 { inner: lhs.inner - rhs.inner }
    }
}

pub impl QM31IntoPackedUnreducedQM31 of Into<QM31, PackedUnreducedQM31> {
    #[inline]
    fn into(self: QM31) -> PackedUnreducedQM31 {
        PackedUnreducedQM31 { inner: self }
    }
}

#[derive(Copy, Drop, Debug)]
pub struct PackedUnreducedCM31 {
    // Using CM31 directly is efficient thanks to the QM31 opcode.
    pub inner: CM31,
}

pub impl PackedUnreducedCM31Impl of PackedUnreducedCM31Trait {
    #[inline]
    fn mul_m31(self: PackedUnreducedCM31, rhs: UnreducedM31) -> PackedUnreducedCM31 {
        PackedUnreducedCM31 { inner: CM31 { inner: self.inner.inner * rhs.inner.into() } }
    }

    /// Returns a zero element with each coordinate set to `P*P*P`.
    #[inline]
    fn large_zero() -> PackedUnreducedCM31 {
        PackedUnreducedCM31 { inner: Zero::zero() }
    }

    #[inline]
    fn reduce(self: PackedUnreducedCM31) -> CM31 {
        self.inner
    }
}

pub impl PackedUnreducedCM31Add of Add<PackedUnreducedCM31> {
    #[inline]
    fn add(lhs: PackedUnreducedCM31, rhs: PackedUnreducedCM31) -> PackedUnreducedCM31 {
        PackedUnreducedCM31 { inner: lhs.inner + rhs.inner }
    }
}

pub impl PackedUnreducedCM31Sub of Sub<PackedUnreducedCM31> {
    #[inline]
    fn sub(lhs: PackedUnreducedCM31, rhs: PackedUnreducedCM31) -> PackedUnreducedCM31 {
        PackedUnreducedCM31 { inner: lhs.inner - rhs.inner }
    }
}

pub impl CM31IntoPackedUnreducedCM31 of Into<CM31, PackedUnreducedCM31> {
    #[inline]
    fn into(self: CM31) -> PackedUnreducedCM31 {
        PackedUnreducedCM31 { inner: self }
    }
}

impl QM31Debug of core::fmt::Debug<QM31> {
    fn fmt(self: @QM31, ref f: core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        QM31Dispaly::fmt(self, ref f)
    }
}

#[inline(always)]
pub fn qm31_const<
    const W0: M31InnerT, const W1: M31InnerT, const W2: M31InnerT, const W3: M31InnerT,
>() -> QM31 nopanic {
    QM31 { inner: core::qm31::qm31_const::<W0, W1, W2, W3>() }
}
