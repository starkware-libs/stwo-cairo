//! Implementation of CM31 field using the QM31 opcode.
use core::num::traits::{One, Zero};
use super::super::qm31::QM31;

#[derive(Copy, Drop, Debug, PartialEq)]
pub struct CM31 {
    // Represented using QM31, since QM31 has a dedicated opcode.
    pub inner: super::qm31::QM31,
}

pub impl CM31InvertibleImpl of Invertible<CM31> {
    #[inline(always)]
    fn inverse(self: CM31) -> CM31 {
        CM31 { inner: self.inner.inverse() }
    }
}

pub impl CM31BatchInvertibleImpl of BatchInvertible<CM31> {}

pub impl CM31Impl of CM31Trait {
    #[inline(always)]
    fn mul_m31(self: CM31, rhs: M31) -> CM31 {
        CM31 { inner: self.inner * rhs.inner.into() }
    }

    #[inline(always)]
    fn sub_m31(self: CM31, rhs: M31) -> CM31 {
        CM31 { inner: self.inner - rhs.inner.into() }
    }

    #[inline(always)]
    fn unpack(self: CM31) -> (M31, M31) {
        let [a, b, _, _] = self.inner.to_array();
        (a.into(), b.into())
    }

    #[inline(always)]
    fn pack(a: M31, b: M31) -> CM31 {
        CM31 { inner: a.inner + qm31_const::<0, 1, 0, 0>() * b.inner }
    }
}

pub impl CM31Add of core::traits::Add<CM31> {
    #[inline(always)]
    fn add(lhs: CM31, rhs: CM31) -> CM31 {
        CM31 { inner: lhs.inner + rhs.inner }
    }
}

pub impl CM31Sub of core::traits::Sub<CM31> {
    #[inline(always)]
    fn sub(lhs: CM31, rhs: CM31) -> CM31 {
        CM31 { inner: lhs.inner - rhs.inner }
    }
}

pub impl CM31Mul of core::traits::Mul<CM31> {
    #[inline(always)]
    fn mul(lhs: CM31, rhs: CM31) -> CM31 {
        CM31 { inner: lhs.inner * rhs.inner }
    }
}

pub impl CM31AddAssign of AddAssign<CM31, CM31> {
    #[inline(always)]
    fn add_assign(ref self: CM31, rhs: CM31) {
        self = self + rhs
    }
}

pub impl CM31SubAssign of SubAssign<CM31, CM31> {
    #[inline(always)]
    fn sub_assign(ref self: CM31, rhs: CM31) {
        self = self - rhs
    }
}

pub impl CM31MulAssign of MulAssign<CM31, CM31> {
    #[inline(always)]
    fn mul_assign(ref self: CM31, rhs: CM31) {
        self = self * rhs
    }
}

pub impl CM31Zero of Zero<CM31> {
    #[inline(always)]
    fn zero() -> CM31 {
        CM31 { inner: Zero::zero() }
    }

    #[inline(always)]
    fn is_zero(self: @CM31) -> bool {
        self.inner.is_zero()
    }

    #[inline(always)]
    fn is_non_zero(self: @CM31) -> bool {
        self.inner.is_non_zero()
    }
}

pub impl CM31One of One<CM31> {
    #[inline(always)]
    fn one() -> CM31 {
        CM31 { inner: One::one() }
    }

    #[inline(always)]
    fn is_one(self: @CM31) -> bool {
        self.inner.is_one()
    }

    #[inline(always)]
    fn is_non_one(self: @CM31) -> bool {
        self.inner.is_non_one()
    }
}

pub impl M31IntoCM31 of core::traits::Into<M31, CM31> {
    #[inline(always)]
    fn into(self: M31) -> CM31 {
        CM31 { inner: self.inner.into() }
    }
}

pub impl CM31Neg of Neg<CM31> {
    #[inline(always)]
    fn neg(a: CM31) -> CM31 {
        CM31 { inner: -a.inner }
    }
}

pub impl CM31Serde of Serde<CM31> {
    fn serialize(self: @CM31, ref output: Array<felt252>) {
        let [a, b, _, _] = self.inner.to_array();
        output.append(a.into());
        output.append(b.into());
    }

    fn deserialize(ref serialized: Span<felt252>) -> Option<CM31> {
        let a: M31InnerT = Serde::deserialize(ref serialized)?;
        let b: M31InnerT = Serde::deserialize(ref serialized)?;
        Some(CM31 { inner: QM31Trait::from_array([a, b, 0, 0]) })
    }
}

