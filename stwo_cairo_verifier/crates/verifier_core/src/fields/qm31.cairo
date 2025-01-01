use core::num::traits::one::One;
use core::num::traits::zero::Zero;
use core::ops::{AddAssign, MulAssign, SubAssign};
use super::Invertible;
use super::cm31::{CM31, cm31};
use super::m31::{M31, M31Impl, UnreducedM31};

/// Equals `(2^31 - 1)^4`.
pub const P4: u128 = 21267647892944572736998860269687930881;

pub const QM31_EXTENSION_DEGREE: usize = 4;

pub const R: CM31 = CM31 { a: M31 { inner: 2 }, b: M31 { inner: 1 } };

#[derive(Copy, Drop, Debug, PartialEq, Serde)]
pub struct QM31 {
    pub a: CM31,
    pub b: CM31,
}

impl QM31InvertibleImpl of Invertible<QM31> {
    fn inverse(self: QM31) -> QM31 {
        assert!(self.is_non_zero());
        let b2 = self.b * self.b;
        let ib2 = CM31 { a: -b2.b, b: b2.a };
        let denom = self.a * self.a - (b2 + b2 + ib2);
        let denom_inverse = denom.inverse();
        QM31 { a: self.a * denom_inverse, b: -self.b * denom_inverse }
    }
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

    #[inline]
    fn mul_m31(self: QM31, multiplier: M31) -> QM31 {
        QM31 {
            a: CM31 { a: self.a.a * multiplier, b: self.a.b * multiplier },
            b: CM31 { a: self.b.a * multiplier, b: self.b.b * multiplier },
        }
    }

    // TODO(andrew): When associated types are supported, support `Mul<QM31, CM31>`.
    #[inline]
    fn mul_cm31(self: QM31, rhs: CM31) -> QM31 {
        QM31 { a: self.a * rhs, b: self.b * rhs }
    }

    fn complex_conjugate(self: QM31) -> QM31 {
        QM31 { a: self.a, b: -self.b }
    }

    /// Returns a fused multiply-subtract i.e. returns `a * b - c`.
    #[inline]
    fn fms(a: QM31, b: QM31, c: QM31) -> QM31 {
        (Self::mul_unreduced(a, b) - c.into()).reduce()
    }

    /// Returns a fused multiply-add i.e. returns `a * b + c`.
    #[inline]
    fn fma(a: QM31, b: QM31, c: QM31) -> QM31 {
        (Self::mul_unreduced(a, b) + c.into()).reduce()
    }

    /// Returns `lhs * rhs` in unreduced form.
    ///
    /// Output coordinates are returned in the range `[P * P, P * P * 13)`.
    // TODO(andrew): May be net worse performance doing unreduced arithmetic due to all felt252
    // multiplications (which are expensive for the M31 prover to simulate). Measure overall
    // prove+verify performance differences with unreduced felt252 vs reduced u32. If prover
    // performance is an issue consider Karatsuba.
    #[inline]
    fn mul_unreduced(lhs: QM31, rhs: QM31) -> UnreducedQM31 {
        /// Equals `P * P * 16`.
        const PP16: felt252 = 0x3fffffff000000010;

        // `lhs` 1st CM31 coordinate.
        let lhs_aa: felt252 = lhs.a.a.into();
        let lhs_ab: felt252 = lhs.a.b.into();

        // `lhs` 2nd CM31 coordinate.
        let lhs_ba: felt252 = lhs.b.a.into();
        let lhs_bb: felt252 = lhs.b.b.into();

        // `rhs` 1st CM31 coordinate.
        let rhs_aa: felt252 = rhs.a.a.into();
        let rhs_ab: felt252 = rhs.a.b.into();

        // `rhs` 2nd CM31 coordinate.
        let rhs_ba: felt252 = rhs.b.a.into();
        let rhs_bb: felt252 = rhs.b.b.into();

        // lhs.a * rhs.a
        let (aa_t_ba_a, aa_t_ba_b) = {
            let res_a = lhs_aa * rhs_aa - lhs_ab * rhs_ab;
            let res_b = lhs_aa * rhs_ab + lhs_ab * rhs_aa;
            (res_a, res_b)
        };

        // R * lhs.b * rhs.b
        let (r_t_ab_t_bb_a, r_t_ab_t_bb_b) = {
            let res_a = lhs_ba * rhs_ba - lhs_bb * rhs_bb;
            let res_b = lhs_ba * rhs_bb + lhs_bb * rhs_ba;
            (res_a + res_a - res_b, res_a + res_b + res_b)
        };

        // lhs.a * rhs.b
        let (aa_t_bb_a, aa_t_bb_b) = {
            let res_a = lhs_aa * rhs_ba - lhs_ab * rhs_bb;
            let res_b = lhs_aa * rhs_bb + lhs_ab * rhs_ba;
            (res_a, res_b)
        };

        // lhs.b * rhs.a
        let (ab_t_ba_a, ab_t_ba_b) = {
            let res_a = lhs_ba * rhs_aa - lhs_bb * rhs_ab;
            let res_b = lhs_ba * rhs_ab + lhs_bb * rhs_aa;
            (res_a, res_b)
        };

        UnreducedQM31 {
            a: PP16 + aa_t_ba_a + r_t_ab_t_bb_a,
            b: PP16 + aa_t_ba_b + r_t_ab_t_bb_b,
            c: PP16 + aa_t_bb_a + ab_t_ba_a,
            d: PP16 + aa_t_bb_b + ab_t_ba_b,
        }
    }

    /// Returns the combined value, given the values of its composing base field polynomials at that
    /// point.
    fn from_partial_evals(evals: [QM31; QM31_EXTENSION_DEGREE]) -> QM31 {
        let [e0, e1, e2, e3] = evals;
        e0 + e1 * qm31(0, 1, 0, 0) + e2 * qm31(0, 0, 1, 0) + e3 * qm31(0, 0, 0, 1)
    }
}

pub impl QM31Add of core::traits::Add<QM31> {
    #[inline]
    fn add(lhs: QM31, rhs: QM31) -> QM31 {
        QM31 { a: lhs.a + rhs.a, b: lhs.b + rhs.b }
    }
}

pub impl QM31Sub of core::traits::Sub<QM31> {
    #[inline]
    fn sub(lhs: QM31, rhs: QM31) -> QM31 {
        QM31 { a: lhs.a - rhs.a, b: lhs.b - rhs.b }
    }
}

pub impl QM31Mul of core::traits::Mul<QM31> {
    #[inline]
    fn mul(lhs: QM31, rhs: QM31) -> QM31 {
        // (a + bu) * (c + du) = (ac + rbd) + (ad + bc)u.
        QM31 { a: lhs.a * rhs.a + R * lhs.b * rhs.b, b: lhs.a * rhs.b + lhs.b * rhs.a }
    }
}

pub impl QM31Div of core::traits::Div<QM31> {
    #[inline]
    fn div(lhs: QM31, rhs: QM31) -> QM31 {
        // (a + bu) * (c + du) = (ac + rbd) + (ad + bc)u.
        lhs * rhs.inverse()
    }
}

pub impl QM31AddAssign of AddAssign<QM31, QM31> {
    #[inline]
    fn add_assign(ref self: QM31, rhs: QM31) {
        self = self + rhs
    }
}

pub impl QM31SubAssign of SubAssign<QM31, QM31> {
    #[inline]
    fn sub_assign(ref self: QM31, rhs: QM31) {
        self = self - rhs
    }
}

pub impl QM31MulAssign of MulAssign<QM31, QM31> {
    #[inline]
    fn mul_assign(ref self: QM31, rhs: QM31) {
        self = self * rhs
    }
}

pub impl QM31Zero of Zero<QM31> {
    #[inline]
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
    #[inline]
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
    #[inline]
    fn into(self: M31) -> QM31 {
        QM31 { a: self.into(), b: Zero::zero() }
    }
}

pub impl CM31IntoQM31 of core::traits::Into<CM31, QM31> {
    #[inline]
    fn into(self: CM31) -> QM31 {
        QM31 { a: self, b: Zero::zero() }
    }
}

pub impl QM31Neg of Neg<QM31> {
    #[inline]
    fn neg(a: QM31) -> QM31 {
        QM31 { a: -a.a, b: -a.b }
    }
}

impl QM31PartialOrd of PartialOrd<QM31> {
    #[inline]
    fn lt(lhs: QM31, rhs: QM31) -> bool {
        lhs.a < rhs.a || (lhs.a == rhs.a && lhs.b < rhs.b)
    }
}

#[inline]
pub fn qm31(a: u32, b: u32, c: u32, d: u32) -> QM31 {
    QM31 { a: cm31(a, b), b: cm31(c, d) }
}

/// Stores an unreduced [`QM31`] with each coordinate stored as a `felt252`.
#[derive(Copy, Drop, Debug)]
struct UnreducedQM31 {
    a: felt252,
    b: felt252,
    c: felt252,
    d: felt252,
}

#[generate_trait]
impl UnreducedQM31Impl of UnreducedQM31Trait {
    #[inline]
    fn reduce(self: UnreducedQM31) -> QM31 {
        QM31 {
            a: CM31 {
                a: M31Impl::reduce_u128(self.a.try_into().unwrap()),
                b: M31Impl::reduce_u128(self.b.try_into().unwrap()),
            },
            b: CM31 {
                a: M31Impl::reduce_u128(self.c.try_into().unwrap()),
                b: M31Impl::reduce_u128(self.d.try_into().unwrap()),
            },
        }
    }
}

impl UnreducedQM31Sub of Sub<UnreducedQM31> {
    #[inline]
    fn sub(lhs: UnreducedQM31, rhs: UnreducedQM31) -> UnreducedQM31 {
        UnreducedQM31 { a: lhs.a - rhs.a, b: lhs.b - rhs.b, c: lhs.c - rhs.c, d: lhs.d - rhs.d }
    }
}

impl UnreducedQM31Add of Add<UnreducedQM31> {
    #[inline]
    fn add(lhs: UnreducedQM31, rhs: UnreducedQM31) -> UnreducedQM31 {
        UnreducedQM31 { a: lhs.a + rhs.a, b: lhs.b + rhs.b, c: lhs.c + rhs.c, d: lhs.d + rhs.d }
    }
}

impl QM31IntoUnreducedQM31 of Into<QM31, UnreducedQM31> {
    #[inline]
    fn into(self: QM31) -> UnreducedQM31 {
        UnreducedQM31 {
            a: self.a.a.inner.into(),
            b: self.a.b.inner.into(),
            c: self.b.a.inner.into(),
            d: self.b.b.inner.into(),
        }
    }
}

/// Stores an unreduced [`QM31`] packed into two `felt252`.
///
/// Is more efficient than [`UnreducedQM31`] since only requires two felt252 operations per addition
/// or M31 multiplication vs 4.
// TODO: Determine if performance difference between UnreducedQM31 and PackedUnreducedQM31 is worth
// keeping around both types.
#[derive(Copy, Drop, Debug)]
pub struct PackedUnreducedQM31 {
    pub a: PackedUnreducedCM31,
    pub b: PackedUnreducedCM31,
}

#[generate_trait]
pub impl PackedUnreducedQM31Impl of PackedUnreducedQM31Trait {
    #[inline]
    fn mul_m31(self: PackedUnreducedQM31, rhs: UnreducedM31) -> PackedUnreducedQM31 {
        PackedUnreducedQM31 { a: self.a.mul_m31(rhs), b: self.b.mul_m31(rhs) }
    }

    /// Returns a zero element with each coordinate set to `P*P*P`.
    #[inline]
    fn large_zero() -> PackedUnreducedQM31 {
        PackedUnreducedQM31 {
            a: PackedUnreducedCM31Impl::large_zero(), b: PackedUnreducedCM31Impl::large_zero(),
        }
    }

    #[inline]
    fn reduce(self: PackedUnreducedQM31) -> QM31 {
        QM31 { a: self.a.reduce(), b: self.b.reduce() }
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
        PackedUnreducedQM31 { a: lhs.a + rhs.a, b: lhs.b + rhs.b }
    }
}

pub impl PackedUnreducedQM31Sub of Sub<PackedUnreducedQM31> {
    #[inline]
    fn sub(lhs: PackedUnreducedQM31, rhs: PackedUnreducedQM31) -> PackedUnreducedQM31 {
        PackedUnreducedQM31 { a: lhs.a - rhs.a, b: lhs.b - rhs.b }
    }
}

pub impl QM31IntoPackedUnreducedQM31 of Into<QM31, PackedUnreducedQM31> {
    #[inline]
    fn into(self: QM31) -> PackedUnreducedQM31 {
        PackedUnreducedQM31 { a: self.a.into(), b: self.b.into() }
    }
}

/// An unreduced [`CM31`] packed into a single `felt252`.
#[derive(Copy, Drop, Debug)]
pub struct PackedUnreducedCM31 {
    /// Stores a 128 bit and 124 bit unreduced M31 packed into a felt252 i.e. `a + (b << 128)`.
    pub inner: felt252,
}

#[generate_trait]
pub impl PackedUnreducedCM31Impl of PackedUnreducedCM31Trait {
    #[inline]
    fn mul_m31(self: PackedUnreducedCM31, rhs: UnreducedM31) -> PackedUnreducedCM31 {
        PackedUnreducedCM31 { inner: self.inner * rhs.inner }
    }

    /// Returns a zero element with each coordinate set to `P*P*P`.
    #[inline]
    fn large_zero() -> PackedUnreducedCM31 {
        // Stores `P*P*P + (P*P*P << 128)`.
        const PPP_PPP: felt252 = 0x1fffffff400000017fffffff000000001fffffff400000017fffffff;
        PackedUnreducedCM31 { inner: PPP_PPP }
    }

    #[inline]
    fn reduce(self: PackedUnreducedCM31) -> CM31 {
        let u256 { low: a, high: b } = self.inner.into();
        CM31 { a: M31Impl::reduce_u128(a), b: M31Impl::reduce_u128(b) }
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
        const POW2_128: felt252 = 0x100000000000000000000000000000000;
        let a_felt: felt252 = self.a.into();
        let b_felt: felt252 = self.b.into();
        PackedUnreducedCM31 { inner: a_felt + b_felt * POW2_128 }
    }
}

impl DisplayQM31 of core::fmt::Display<QM31> {
    fn fmt(self: @QM31, ref f: core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        write!(f, "({}) + ({})u", self.a, self.b)
    }
}

#[cfg(test)]
mod tests {
    use super::super::Invertible;
    use super::super::m31::{P_U32 as P, m31};
    use super::{
        PackedUnreducedQM31Impl, QM31, QM31Impl, QM31IntoPackedUnreducedQM31, QM31Trait, qm31,
    };

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
        assert_eq!(qm1.mul_m31(m), qm1 * m.into());
    }

    #[test]
    fn test_fma() {
        let a = qm31(P - 4, P - 90, 958, P - 1);
        let b = qm31(P - 183, 75, P - 921, P - 6124);
        let c = qm31(2, P - 1, P - 1, P - 8);

        let res = QM31Impl::fma(a, b, c);

        assert_eq!(res, a * b + c);
    }

    #[test]
    fn test_fms() {
        let a = qm31(P - 4, P - 90, 958, P - 1);
        let b = qm31(P - 183, 75, P - 921, P - 6124);
        let c = qm31(2, P - 1, P - 1, P - 8);

        let res = QM31Impl::fms(a, b, c);

        assert_eq!(res, a * b - c);
    }

    #[test]
    fn test_packed_unreduced_qm31() {
        let a = qm31(P - 4, P - 90, 958, P - 1);
        let b = m31(P - 183);

        let res_unreduced = QM31IntoPackedUnreducedQM31::into(a).mul_m31(b.into());
        let res = res_unreduced.reduce();

        assert_eq!(res, a.mul_m31(b));
    }
}
