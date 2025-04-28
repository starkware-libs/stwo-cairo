//! Software only implementation of CM31 field (i.e no QM31 opcode).
use core::num::traits::{One, Zero};
use core::ops::{AddAssign, MulAssign, SubAssign};
use super::CM31Trait;
use super::super::m31::{M31, M31InnerT, m31};
use super::super::{BatchInvertible, Invertible};

#[derive(Copy, Drop, Debug, PartialEq, Serde)]
pub struct CM31 {
    pub a: M31,
    pub b: M31,
}

pub impl CM31BatchInvertibleImpl of BatchInvertible<CM31> {}

pub impl CM31InvertibleImpl of Invertible<CM31> {
    fn inverse(self: CM31) -> CM31 {
        assert!(self.is_non_zero());
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
        (*self).a.is_non_zero() || (*self).b.is_non_zero()
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

#[inline]
pub fn cm31_const<const W0: M31InnerT, const W1: M31InnerT>() -> CM31 nopanic {
    CM31 { a: M31 { inner: W0 }, b: M31 { inner: W1 } }
}


#[cfg(test)]
mod tests {
    use super::cm31_const;
    use super::super::m31::m31;

    #[test]
    fn test_cm31() {
        let cm0 = cm31_const::<1, 2>();
        let cm1 = cm31_const::<4, 5>();
        let m = m31(8);
        let cm = cm31_const::<8, 0>();
        let cm0_x_cm1 = cm31_const::<0x7ffffff9, 13>();

        assert_eq!(cm0 + cm1, cm31_const::<5, 7>());
        assert_eq!(cm1 + m.into(), cm1 + cm);
        assert_eq!(cm0 * cm1, cm0_x_cm1);
        assert_eq!(cm1 * m.into(), cm1 * cm);
        assert_eq!(-cm0, cm31_const::<0x7ffffffe, 0x7ffffffd>());
        assert_eq!(cm0 - cm1, cm31_const::<0x7ffffffc, 0x7ffffffc>());
        assert_eq!(cm1 - m.into(), cm1 - cm);
    }
}
