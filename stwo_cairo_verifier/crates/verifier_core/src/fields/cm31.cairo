use core::num::traits::{One, Zero};
use core::ops::{AddAssign, MulAssign, SubAssign};
use super::m31::{M31, M31Trait, m31};
use super::{BatchInvertible, Invertible};

#[derive(Copy, Drop, Debug, PartialEq, Serde)]
#[cfg(feature: "qm31_opcode")]
pub struct CM31 {
    pub inner: super::qm31::QM31,
}

#[derive(Copy, Drop, Debug, PartialEq, Serde)]
#[cfg(not(feature: "qm31_opcode"))]
pub struct CM31 {
    pub a: M31,
    pub b: M31,
}

pub impl CM31InvertibleImpl of Invertible<CM31> {
    #[cfg(feature: "qm31_opcode")]
    fn inverse(self: CM31) -> CM31 {
        CM31 { inner: self.inner.inverse() }
    }

    #[cfg(not(feature: "qm31_opcode"))]
    fn inverse(self: CM31) -> CM31 {
        assert!(self.is_non_zero());
        let denom_inverse: M31 = (self.a * self.a + self.b * self.b).inverse();
        CM31 { a: self.a * denom_inverse, b: -self.b * denom_inverse }
    }
}

pub impl CM31BatchInvertibleImpl of BatchInvertible<CM31> {}

pub trait CM31Trait {
    // TODO(andrew): When associated types are supported, support `Mul<CM31, M31>`.
    fn mul_m31(self: CM31, rhs: M31) -> CM31;

    // TODO(andrew): When associated types are supported, support `Sub<CM31, M31>`.
    fn sub_m31(self: CM31, rhs: M31) -> CM31;

    fn unpack(self: CM31) -> (M31, M31);

    fn pack(a: M31, b: M31) -> CM31;
}

#[cfg(feature: "qm31_opcode")]
pub impl CM31Impl of CM31Trait {
    #[inline]
    fn mul_m31(self: CM31, rhs: M31) -> CM31 {
        CM31 { inner: self.inner.mul_m31(rhs) }
    }

    #[inline]
    fn sub_m31(self: CM31, rhs: M31) -> CM31 {
        CM31 { inner: self.inner.sub_m31(rhs) }
    }

    fn unpack(self: CM31) -> (M31, M31) {
        let [a, b, _, _] = self.inner.to_array();
        (a, b)
    }

    fn pack(a: M31, b: M31) -> CM31 {
        CM31 { inner: super::qm31::QM31::from_array([a, b, m31(0), m31(0)]) }
    }
}

#[cfg(not(feature: "qm31_opcode"))]
pub impl CM31Impl of CM31Trait {
    #[inline]
    fn mul_m31(self: CM31, rhs: M31) -> CM31 {
        CM31 { a: self.a * rhs, b: self.b * rhs }
    }

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
    #[cfg(feature: "qm31_opcode")]
    fn add(lhs: CM31, rhs: CM31) -> CM31 {
        CM31 { inner: lhs.inner + rhs.inner }
    }

    #[inline]
    #[cfg(not(feature: "qm31_opcode"))]
    fn add(lhs: CM31, rhs: CM31) -> CM31 {
        CM31 { a: lhs.a + rhs.a, b: lhs.b + rhs.b }
    }
}

pub impl CM31Sub of core::traits::Sub<CM31> {
    #[inline]
    #[cfg(feature: "qm31_opcode")]
    fn sub(lhs: CM31, rhs: CM31) -> CM31 {
        CM31 { inner: lhs.inner - rhs.inner }
    }

    #[inline]
    #[cfg(not(feature: "qm31_opcode"))]
    fn sub(lhs: CM31, rhs: CM31) -> CM31 {
        CM31 { a: lhs.a - rhs.a, b: lhs.b - rhs.b }
    }
}

pub impl CM31Mul of core::traits::Mul<CM31> {
    #[inline]
    #[cfg(feature: "qm31_opcode")]
    fn mul(lhs: CM31, rhs: CM31) -> CM31 {
        CM31 { inner: lhs.inner * rhs.inner }
    }

    #[inline]
    #[cfg(not(feature: "qm31_opcode"))]
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
    #[cfg(feature: "qm31_opcode")]
    fn zero() -> CM31 {
        CM31 { inner: Zero::zero() }
    }

    #[cfg(not(feature: "qm31_opcode"))]
    fn zero() -> CM31 {
        cm31(0, 0)
    }

    #[cfg(feature: "qm31_opcode")]
    fn is_zero(self: @CM31) -> bool {
        self.inner.is_zero()
    }

    #[cfg(not(feature: "qm31_opcode"))]
    fn is_zero(self: @CM31) -> bool {
        (*self).a.is_zero() && (*self).b.is_zero()
    }

    #[cfg(feature: "qm31_opcode")]
    fn is_non_zero(self: @CM31) -> bool {
        self.inner.is_non_zero()
    }

    #[cfg(not(feature: "qm31_opcode"))]
    fn is_non_zero(self: @CM31) -> bool {
        (*self).a.is_non_zero() || (*self).b.is_non_zero()
    }
}

pub impl CM31One of One<CM31> {
    #[cfg(feature: "qm31_opcode")]
    fn one() -> CM31 {
        CM31 { inner: One::one() }
    }

    #[cfg(not(feature: "qm31_opcode"))]
    fn one() -> CM31 {
        cm31(1, 0)
    }

    #[cfg(feature: "qm31_opcode")]
    fn is_one(self: @CM31) -> bool {
        self.inner.is_one()
    }

    #[cfg(not(feature: "qm31_opcode"))]
    fn is_one(self: @CM31) -> bool {
        (*self).a.is_one() && (*self).b.is_zero()
    }

    #[cfg(feature: "qm31_opcode")]
    fn is_non_one(self: @CM31) -> bool {
        self.inner.is_non_one()
    }

    #[cfg(not(feature: "qm31_opcode"))]
    fn is_non_one(self: @CM31) -> bool {
        (*self).a.is_non_one() || (*self).b.is_non_zero()
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

impl CM31PartialOrd of PartialOrd<CM31> {
    fn lt(lhs: CM31, rhs: CM31) -> bool {
        lhs.a < rhs.a || (lhs.a == rhs.a && lhs.b < rhs.b)
    }
}

impl DisplayCM31 of core::fmt::Display<CM31> {
    fn fmt(self: @CM31, ref f: core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        write!(f, "{} + {}i", self.a, self.b)
    }
}

#[inline]
pub fn cm31(a: u32, b: u32) -> CM31 {
    CM31 { a: m31(a), b: m31(b) }
}


#[cfg(test)]
mod tests {
    use super::super::m31::{P_U32 as P, m31};
    use super::cm31;

    #[test]
    fn test_cm31() {
        let cm0 = cm31(1, 2);
        let cm1 = cm31(4, 5);
        let m = m31(8);
        let cm = cm31(8, 0);
        let cm0_x_cm1 = cm31(P - 6, 13);

        assert_eq!(cm0 + cm1, cm31(5, 7));
        assert_eq!(cm1 + m.into(), cm1 + cm);
        assert_eq!(cm0 * cm1, cm0_x_cm1);
        assert_eq!(cm1 * m.into(), cm1 * cm);
        assert_eq!(-cm0, cm31(P - 1, P - 2));
        assert_eq!(cm0 - cm1, cm31(P - 3, P - 3));
        assert_eq!(cm1 - m.into(), cm1 - cm);
    }
}
