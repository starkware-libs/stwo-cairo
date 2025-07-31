//! Implementation of CM31 field using the QM31 opcode.
use core::num::traits::{One, Zero};
use core::ops::{AddAssign, MulAssign, SubAssign};
use super::super::m31::{M31, M31Zero, UnreducedM31};
use super::super::qm31::{M31IntoQM31, QM31Trait};
use super::super::{BatchInvertible, Invertible};
use super::{CM31Trait, PackedUnreducedCM31Trait};

#[derive(Copy, Drop, Debug, PartialEq)]
pub struct CM31 {
    // Represented using QM31, since QM31 has a dedicated opcode.
    pub inner: super::super::qm31::QM31,
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
        CM31 { inner: self.inner * rhs.into() }
    }

    #[inline(always)]
    fn sub_m31(self: CM31, rhs: M31) -> CM31 {
        CM31 { inner: self.inner - rhs.into() }
    }

    #[inline(always)]
    fn unpack(self: CM31) -> (M31, M31) {
        let [a, b, _, _] = self.inner.to_fixed_array();
        (a, b)
    }

    #[inline(always)]
    fn pack(a: M31, b: M31) -> CM31 {
        CM31 { inner: QM31Trait::from_fixed_array([a, b, M31Zero::zero(), M31Zero::zero()]) }
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
        CM31 { inner: self.into() }
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
        let [a, b, _, _] = self.inner.to_fixed_array();
        output.append(a.into());
        output.append(b.into());
    }

    fn deserialize(ref serialized: Span<felt252>) -> Option<CM31> {
        let a: M31 = Serde::deserialize(ref serialized)?;
        let b: M31 = Serde::deserialize(ref serialized)?;
        Some(CM31 { inner: QM31Trait::from_fixed_array([a, b, M31Zero::zero(), M31Zero::zero()]) })
    }
}


#[derive(Copy, Drop, Debug)]
pub struct PackedUnreducedCM31 {
    // Using CM31 directly is efficient thanks to the QM31 opcode.
    pub inner: CM31,
}

pub impl PackedUnreducedCM31Impl of PackedUnreducedCM31Trait {
    #[inline]
    fn mul_m31(self: PackedUnreducedCM31, rhs: M31) -> PackedUnreducedCM31 {
        PackedUnreducedCM31 { inner: CM31 { inner: self.inner.inner * rhs.into() } }
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

