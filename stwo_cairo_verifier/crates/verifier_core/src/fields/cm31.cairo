use core::num::traits::{One, Zero};
use core::ops::{AddAssign, MulAssign, SubAssign};
use super::m31::{M31, M31Impl, m31};
use super::{BatchInvertible, Invertible};

#[derive(Copy, Drop, Debug, PartialEq, Serde)]
pub struct CM31 {
    pub a: M31,
    pub b: M31,
}

pub impl CM31InvertibleImpl of Invertible<CM31> {
    fn inverse(self: CM31) -> CM31 {
        assert!(self.is_non_zero());
        let denom_inverse: M31 = (self.a * self.a + self.b * self.b).inverse();
        CM31 { a: self.a * denom_inverse, b: -self.b * denom_inverse }
    }
}

pub impl CM31BatchInvertibleImpl of BatchInvertible<CM31> {}

#[generate_trait]
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
        cm31(0, 0)
    }

    fn is_zero(self: @CM31) -> bool {
        (*self).a.is_zero() && (*self).b.is_zero()
    }

    fn is_non_zero(self: @CM31) -> bool {
        (*self).a.is_non_zero() || (*self).b.is_non_zero()
    }
}

pub impl CM31One of One<CM31> {
    fn one() -> CM31 {
        cm31(1, 0)
    }
    fn is_one(self: @CM31) -> bool {
        (*self).a.is_one() && (*self).b.is_zero()
    }
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
    use super::cm31;
    use super::super::m31::{P_U32 as P, m31};

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
