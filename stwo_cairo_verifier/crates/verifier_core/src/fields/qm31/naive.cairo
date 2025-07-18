//! Software only implementation of QM31 field (i.e no QM31 opcode).
use core::num::traits::one::One;
use core::num::traits::zero::Zero;
use core::ops::{AddAssign, MulAssign, SubAssign};
use super::super::Invertible;
use super::super::cm31::{CM31, CM31Trait, PackedUnreducedCM31, PackedUnreducedCM31Trait};
use super::super::m31::{M31, M31InnerT, M31Trait, UnreducedM31};
use super::{PackedUnreducedQM31Trait, QM31Trait, QM31_EXTENSION_DEGREE, UnreducedQM31Trait};


// Represents a + u*b.
#[derive(Copy, Drop, Debug, PartialEq)]
pub struct QM31 {
    a: CM31,
    b: CM31,
}

impl QM31InvertibleImpl of Invertible<QM31> {
    fn inverse(self: QM31) -> QM31 {
        let b2 = self.b * self.b;
        let ib2 = CM31 { a: -b2.b, b: b2.a };
        let denom = self.a * self.a - (b2 + b2 + ib2);
        let denom_inverse = denom.inverse();
        QM31 { a: self.a * denom_inverse, b: -self.b * denom_inverse }
    }
}

pub impl QM31Impl of QM31Trait {
    #[inline]
    fn from_fixed_array(arr: [M31; QM31_EXTENSION_DEGREE]) -> QM31 {
        let [a, b, c, d] = arr;
        QM31 { a: CM31 { a: a, b: b }, b: CM31 { a: c, b: d } }
    }

    #[inline]
    fn to_fixed_array(self: QM31) -> [M31; QM31_EXTENSION_DEGREE] {
        [self.a.a, self.a.b, self.b.a, self.b.b]
    }

    #[inline]
    fn mul_m31(self: QM31, rhs: M31) -> QM31 {
        QM31 { a: CM31Trait::mul_m31(self.a, rhs), b: CM31Trait::mul_m31(self.b, rhs) }
    }

    #[inline]
    fn mul_cm31(self: QM31, rhs: CM31) -> QM31 {
        QM31 { a: self.a * rhs, b: self.b * rhs }
    }

    fn complex_conjugate(self: QM31) -> QM31 {
        QM31 { a: self.a, b: -self.b }
    }

    #[inline]
    fn fused_mul_add(a: QM31, b: QM31, c: QM31) -> QM31 {
        (Self::mul_unreduced(a, b) + c.into()).reduce()
    }

    #[inline]
    fn fused_mul_sub(a: QM31, b: QM31, c: QM31) -> QM31 {
        (Self::mul_unreduced(a, b) - c.into()).reduce()
    }

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

    fn from_partial_evals(evals: [QM31; QM31_EXTENSION_DEGREE]) -> QM31 {
        let [e0, e1, e2, e3] = evals;
        e0
            + e1 * qm31_const::<0, 1, 0, 0>()
            + e2 * qm31_const::<0, 0, 1, 0>()
            + e3 * qm31_const::<0, 0, 0, 1>()
    }
}

pub impl QM31Add of core::traits::Add<QM31> {
    #[inline(never)]
    fn add(lhs: QM31, rhs: QM31) -> QM31 {
        QM31 { a: lhs.a + rhs.a, b: lhs.b + rhs.b }
    }
}

pub impl QM31Sub of core::traits::Sub<QM31> {
    #[inline(never)]
    fn sub(lhs: QM31, rhs: QM31) -> QM31 {
        QM31 { a: lhs.a - rhs.a, b: lhs.b - rhs.b }
    }
}

pub impl QM31Mul of core::traits::Mul<QM31> {
    #[inline(never)]
    fn mul(lhs: QM31, rhs: QM31) -> QM31 {
        // (a + bu) * (c + du) = (ac + rbd) + (ad + bc)u.
        QM31 { a: lhs.a * rhs.a + mul_by_r(lhs.b) * rhs.b, b: lhs.a * rhs.b + lhs.b * rhs.a }
    }
}

pub impl QM31Div of core::traits::Div<QM31> {
    #[inline]
    fn div(lhs: QM31, rhs: QM31) -> QM31 {
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
        !self.is_zero()
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
        !self.is_one()
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

/// Stores an unreduced [`QM31`] with each coordinate stored as a `felt252`.
#[derive(Copy, Drop, Debug)]
pub struct UnreducedQM31 {
    a: felt252,
    b: felt252,
    c: felt252,
    d: felt252,
}

pub impl UnreducedQM31Impl of UnreducedQM31Trait {
    #[inline]
    fn reduce(self: UnreducedQM31) -> QM31 {
        QM31 {
            a: CM31 {
                a: M31Trait::reduce_u128(self.a.try_into().unwrap()).into(),
                b: M31Trait::reduce_u128(self.b.try_into().unwrap()).into(),
            },
            b: CM31 {
                a: M31Trait::reduce_u128(self.c.try_into().unwrap()).into(),
                b: M31Trait::reduce_u128(self.d.try_into().unwrap()).into(),
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

pub impl PackedUnreducedQM31Impl of PackedUnreducedQM31Trait {
    #[inline]
    fn mul_m31(self: PackedUnreducedQM31, rhs: UnreducedM31) -> PackedUnreducedQM31 {
        PackedUnreducedQM31 { a: self.a.mul_m31(rhs), b: self.b.mul_m31(rhs) }
    }

    /// Returns a zero element with each coordinate set to `P*P*P`.
    #[inline]
    fn large_zero() -> PackedUnreducedQM31 {
        PackedUnreducedQM31 {
            a: PackedUnreducedCM31Trait::large_zero(), b: PackedUnreducedCM31Trait::large_zero(),
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

pub fn qm31_const<
    const W0: M31InnerT, const W1: M31InnerT, const W2: M31InnerT, const W3: M31InnerT,
>() -> QM31 nopanic {
    QM31 {
        a: CM31 { a: M31 { inner: W0 }, b: M31 { inner: W1 } },
        b: CM31 { a: M31 { inner: W2 }, b: M31 { inner: W3 } },
    }
}

/// Returns `v * R` where `R = 2 + i = u^2`.
#[inline(always)]
fn mul_by_r(v: CM31) -> CM31 {
    // = v * R
    // = (a + bi) * (2 + i)
    // = (2a - b) + (a + 2b)i
    let CM31 { a, b } = v;
    CM31 { a: a + a - b, b: a + b + b }
}
