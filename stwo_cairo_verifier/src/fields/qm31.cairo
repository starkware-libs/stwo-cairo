use super::m31::{M31, m31};
use super::cm31::{CM31, cm31, CM31Trait};
use core::num::traits::zero::Zero;
use core::num::traits::one::One;

pub const R: CM31 = CM31 { a: M31 { inner: 2 }, b: M31 { inner: 1 } };

#[derive(Copy, Drop, Debug, PartialEq, Eq)]
pub struct QM31 {
    a: CM31,
    b: CM31,
}

#[generate_trait]
pub impl QM31Impl of QM31Trait {
    #[inline]
    fn from_array(arr: [M31; 4]) -> QM31 {
        let [a, b, c, d] = arr;
        QM31 { a: CM31 { a: a, b: b }, b: CM31 { a: c, b: d } }
    }
    #[inline]
    fn to_array(self: QM31) -> [M31; 4] {
        [self.a.a, self.a.b, self.b.a, self.b.b]
    }
    fn inverse(self: QM31) -> QM31 {
        assert_ne!(self, Zero::zero());
        let b2 = self.b * self.b;
        let ib2 = CM31 { a: -b2.b, b: b2.a };
        let denom = self.a * self.a - (b2 + b2 + ib2);
        let denom_inverse = denom.inverse();
        QM31 { a: self.a * denom_inverse, b: -self.b * denom_inverse }
    }

    fn complex_conjugate(self: QM31) -> QM31 {
        QM31 { a: self.a, b: -self.b }
    }
}

pub impl QM31Add of core::traits::Add<QM31> {
    fn add(lhs: QM31, rhs: QM31) -> QM31 {
        QM31 { a: lhs.a + rhs.a, b: lhs.b + rhs.b }
    }
}
pub impl QM31Sub of core::traits::Sub<QM31> {
    fn sub(lhs: QM31, rhs: QM31) -> QM31 {
        QM31 { a: lhs.a - rhs.a, b: lhs.b - rhs.b }
    }
}
pub impl QM31Mul of core::traits::Mul<QM31> {
    fn mul(lhs: QM31, rhs: QM31) -> QM31 {
        // (a + bu) * (c + du) = (ac + rbd) + (ad + bc)u.
        QM31 { a: lhs.a * rhs.a + R * lhs.b * rhs.b, b: lhs.a * rhs.b + lhs.b * rhs.a }
    }
}
pub impl QM31Zero of Zero<QM31> {
    fn zero() -> QM31 {
        QM31 { a: Zero::zero(), b: Zero::zero() }
    }
    fn is_zero(self: @QM31) -> bool {
        (*self).a.is_zero() && (*self).b.is_zero()
    }
    fn is_non_zero(self: @QM31) -> bool {
        (*self).a.is_non_zero() || (*self).b.is_non_zero()
    }
}
pub impl QM31One of One<QM31> {
    fn one() -> QM31 {
        QM31 { a: One::one(), b: Zero::zero() }
    }
    fn is_one(self: @QM31) -> bool {
        (*self).a.is_one() && (*self).b.is_zero()
    }
    fn is_non_one(self: @QM31) -> bool {
        (*self).a.is_non_one() || (*self).b.is_non_zero()
    }
}
pub impl M31IntoQM31 of core::traits::Into<M31, QM31> {
    fn into(self: M31) -> QM31 {
        QM31 { a: self.into(), b: Zero::zero() }
    }
}
pub impl CM31IntoQM31 of core::traits::Into<CM31, QM31> {
    fn into(self: CM31) -> QM31 {
        QM31 { a: self, b: Zero::zero() }
    }
}
pub impl QM31Neg of Neg<QM31> {
    fn neg(a: QM31) -> QM31 {
        QM31 { a: -a.a, b: -a.b }
    }
}

pub fn qm31(a: u32, b: u32, c: u32, d: u32) -> QM31 {
    QM31 { a: cm31(a, b), b: cm31(c, d) }
}


#[cfg(test)]
mod tests {
    use super::{QM31, qm31, QM31Trait};
    use super::super::m31::{m31, P, M31Trait};

    #[test]
    fn test_QM31() {
        let qm0 = qm31(1, 2, 3, 4);
        let qm1 = qm31(4, 5, 6, 7);
        let m = m31(8);
        let qm = Into::<_, QM31>::into(m);
        let qm0_x_qm1 = qm31(P - 71, 93, P - 16, 50);

        assert_eq!(qm0 + qm1, qm31(5, 7, 9, 11));
        assert_eq!(qm1 + m.into(), qm1 + qm);
        assert_eq!(qm0 * qm1, qm0_x_qm1);
        assert_eq!(qm1 * m.into(), qm1 * qm);
        assert_eq!(-qm0, qm31(P - 1, P - 2, P - 3, P - 4));
        assert_eq!(qm0 - qm1, qm31(P - 3, P - 3, P - 3, P - 3));
        assert_eq!(qm1 - m.into(), qm1 - qm);
        assert_eq!(qm0_x_qm1 * qm1.inverse(), qm31(1, 2, 3, 4));
        assert_eq!(qm1 * m.inverse().into(), qm1 * qm.inverse());
    }
}
