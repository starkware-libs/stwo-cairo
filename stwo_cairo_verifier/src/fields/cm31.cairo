use super::m31::{M31, m31, M31Trait};

#[derive(Copy, Drop, Debug, PartialEq, Eq)]
pub struct CM31 {
    pub a: M31,
    pub b: M31,
}

#[generate_trait]
pub impl CM31Impl of CM31Trait {
    fn inverse(self: CM31) -> CM31 {
        let denom_inverse: M31 = (self.a * self.a + self.b * self.b).inverse();
        CM31 { a: self.a * denom_inverse, b: -self.b * denom_inverse }
    }
}

pub impl CM31Add of core::traits::Add<CM31> {
    fn add(lhs: CM31, rhs: CM31) -> CM31 {
        CM31 { a: lhs.a + rhs.a, b: lhs.b + rhs.b }
    }
}
pub impl CM31Sub of core::traits::Sub<CM31> {
    fn sub(lhs: CM31, rhs: CM31) -> CM31 {
        CM31 { a: lhs.a - rhs.a, b: lhs.b - rhs.b }
    }
}
pub impl CM31Mul of core::traits::Mul<CM31> {
    fn mul(lhs: CM31, rhs: CM31) -> CM31 {
        CM31 { a: lhs.a * rhs.a - lhs.b * rhs.b, b: lhs.a * rhs.b + lhs.b * rhs.a }
    }
}
pub impl CM31Zero of core::num::traits::Zero<CM31> {
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
pub impl CM31One of core::num::traits::One<CM31> {
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
    fn into(self: M31) -> CM31 {
        CM31 { a: self, b: m31(0) }
    }
}
pub impl CM31Neg of Neg<CM31> {
    fn neg(a: CM31) -> CM31 {
        CM31 { a: -a.a, b: -a.b }
    }
}

pub fn cm31(a: u32, b: u32) -> CM31 {
    CM31 { a: m31(a), b: m31(b) }
}


#[cfg(test)]
mod tests {
    use super::{cm31};
    use super::super::m31::{m31, P};

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
