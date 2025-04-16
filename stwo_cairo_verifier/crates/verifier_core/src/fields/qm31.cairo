use super::cm31::CM31;
use super::m31::{M31, M31InnerT, UnreducedM31};

mod naive;
use naive::*;

/// Equals `(2^31 - 1)^4`.
pub const P4: u128 = 0xFFFFFFF800000017FFFFFFE00000001;

pub const QM31_EXTENSION_DEGREE: usize = 4;

pub trait QM31Trait {
    fn from_array(arr: [M31InnerT; 4]) -> QM31;

    fn to_array(self: QM31) -> [M31InnerT; 4];

    fn mul_m31(self: QM31, rhs: M31) -> QM31;

    // TODO(andrew): When associated types are supported, support `Mul<QM31, CM31>`.
    fn mul_cm31(self: QM31, rhs: CM31) -> QM31;

    fn complex_conjugate(self: QM31) -> QM31;

    /// Returns a fused multiply-subtract i.e. returns `a * b - c`.
    fn fms(a: QM31, b: QM31, c: QM31) -> QM31;

    /// Returns a fused multiply-add i.e. returns `a * b + c`.
    fn fma(a: QM31, b: QM31, c: QM31) -> QM31;

    /// Returns `lhs * rhs` in unreduced form.
    fn mul_unreduced(lhs: QM31, rhs: QM31) -> UnreducedQM31;

    /// Returns the combined value, given the values of its composing base field polynomials at that
    /// point.
    fn from_partial_evals(evals: [QM31; QM31_EXTENSION_DEGREE]) -> QM31;
}

trait UnreducedQM31Trait {
    fn reduce(self: UnreducedQM31) -> QM31;
}

trait PackedUnreducedCM31Trait {
    fn mul_m31(self: PackedUnreducedCM31, rhs: UnreducedM31) -> PackedUnreducedCM31;

    /// Returns a zero element with each coordinate set to `P*P*P`.
    fn large_zero() -> PackedUnreducedCM31;

    fn reduce(self: PackedUnreducedCM31) -> CM31;
}

pub trait PackedUnreducedQM31Trait {
    fn mul_m31(self: PackedUnreducedQM31, rhs: UnreducedM31) -> PackedUnreducedQM31;

    /// Returns a zero element with each coordinate set to `P*P*P`.
    fn large_zero() -> PackedUnreducedQM31;

    fn reduce(self: PackedUnreducedQM31) -> QM31;
}

#[cfg(test)]
mod tests {
    use super::super::Invertible;
    use super::super::m31::m31;
    use super::{
        PackedUnreducedQM31Impl, QM31, QM31Impl, QM31IntoPackedUnreducedQM31, QM31Trait, qm31_const,
    };

    #[test]
    fn test_QM31() {
        let qm0 = qm31_const::<1, 2, 3, 4>();
        let qm1 = qm31_const::<4, 5, 6, 7>();
        let m = m31(8);
        let qm = Into::<_, QM31>::into(m);
        let qm0_x_qm1 = qm31_const::<2147483576, 93, 2147483631, 50>();

        assert_eq!(qm0 + qm1, qm31_const::<5, 7, 9, 11>());
        assert_eq!(qm1 + m.into(), qm1 + qm);
        assert_eq!(qm0 * qm1, qm0_x_qm1);
        assert_eq!(qm1 * m.into(), qm1 * qm);
        assert_eq!(-qm0, qm31_const::<2147483646, 2147483645, 2147483644, 2147483643>());
        assert_eq!(qm0 - qm1, qm31_const::<2147483644, 2147483644, 2147483644, 2147483644>());
        assert_eq!(qm1 - m.into(), qm1 - qm);
        assert_eq!(qm0_x_qm1 * qm1.inverse(), qm31_const::<1, 2, 3, 4>());
        assert_eq!(qm1 * m.inverse().into(), qm1 * qm.inverse());
        assert_eq!(qm1.mul_m31(m), qm1 * m.into());
    }

    #[test]
    fn test_fma() {
        let a = qm31_const::<2147483643, 2147483557, 958, 2147483646>();
        let b = qm31_const::<2147483464, 75, 2147482726, 2147477523>();
        let c = qm31_const::<2, 2147483646, 2147483646, 2147483639>();

        let res = QM31Trait::fma(a, b, c);

        assert_eq!(res, a * b + c);
    }

    #[test]
    fn test_fms() {
        let a = qm31_const::<2147483643, 2147483557, 958, 2147483646>();
        let b = qm31_const::<2147483464, 75, 2147482726, 2147477523>();
        let c = qm31_const::<2, 2147483646, 2147483646, 2147483639>();

        let res = QM31Trait::fms(a, b, c);

        assert_eq!(res, a * b - c);
    }

    #[test]
    fn test_packed_unreduced_qm31() {
        let a = qm31_const::<2147483643, 2147483557, 958, 2147483646>();
        let b = m31(2147483464);

        let res_unreduced = QM31IntoPackedUnreducedQM31::into(a).mul_m31(b.into());
        let res = res_unreduced.reduce();

        assert_eq!(res, a.mul_m31(b));
    }
}
