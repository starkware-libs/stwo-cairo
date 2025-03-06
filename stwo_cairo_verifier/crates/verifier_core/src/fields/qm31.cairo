use bounded_int::upcast;
use core::num::traits::one::One;
use core::num::traits::zero::Zero;
use core::ops::{AddAssign, MulAssign, SubAssign};
use super::cm31::{CM31, CM31Trait, cm31};
use super::m31::{M31, M31InnerT, M31Trait, UnreducedM31};
use super::Invertible;

/// Equals `(2^31 - 1)^4`.
pub const P4: u128 = 0xFFFFFFF800000017FFFFFFE00000001;

pub const QM31_EXTENSION_DEGREE: usize = 4;

#[cfg(feature: "qm31_opcode")]
#[derive(Copy, Drop, PartialEq)]
pub struct QM31 {
    inner: core::qm31::qm31,
}

#[cfg(not(feature: "qm31_opcode"))]
#[derive(Copy, Drop, Debug, PartialEq, Serde)]
pub struct QM31 {
    a: CM31,
    b: CM31,
}

pub trait QM31Trait {
    fn from_array(arr: [M31InnerT; 4]) -> QM31;
    fn to_array(self: QM31) -> [M31InnerT; 4];
    fn mul_m31(self: QM31, rhs: M31) -> QM31;
    fn mul_cm31(self: QM31, rhs: CM31) -> QM31;
    fn complex_conjugate(self: QM31) -> QM31;
    fn fms(a: QM31, b: QM31, c: QM31) -> QM31;
    fn fma(a: QM31, b: QM31, c: QM31) -> QM31;
    fn mul_unreduced(lhs: QM31, rhs: QM31) -> UnreducedQM31;
    fn from_partial_evals(evals: [QM31; QM31_EXTENSION_DEGREE]) -> QM31;
}

#[cfg(feature: "qm31_opcode")]
pub impl QM31Impl of QM31Trait {
    #[inline]
    fn from_array(arr: [M31InnerT; 4]) -> QM31 {
        let [a, b, c, d] = arr;
        QM31 { inner: core::qm31::QM31Trait::new(a, b, c, d) }
    }

    #[inline]
    fn to_array(self: QM31) -> [M31InnerT; 4] {
        let [a, b, c, d] = core::qm31::QM31Trait::unpack(self.inner);
        [a, b, c, d]
    }

    #[inline]
    fn mul_m31(self: QM31, rhs: M31) -> QM31 {
        self * rhs.into()
    }

    #[inline]
    fn mul_cm31(self: QM31, rhs: CM31) -> QM31 {
        self * rhs.into()
    }

    #[inline]
    fn complex_conjugate(self: QM31) -> QM31 {
        let [a, b, c, d] = self.to_array();
        let v0 = Self::from_array([a, b, 0, 0]);
        let v1 = Self::from_array([0, 0, c, d]);
        v0 - v1
    }

    /// Returns a fused multiply-subtract i.e. returns `a * b - c`.
    #[inline]
    fn fms(a: QM31, b: QM31, c: QM31) -> QM31 {
        QM31 { inner: a.inner * b.inner - c.inner }
    }

    /// Returns a fused multiply-add i.e. returns `a * b + c`.
    #[inline]
    fn fma(a: QM31, b: QM31, c: QM31) -> QM31 {
        QM31 { inner: a.inner * b.inner + c.inner }
    }

    #[inline]
    fn mul_unreduced(lhs: QM31, rhs: QM31) -> UnreducedQM31 {
        UnreducedQM31 { inner: lhs * rhs }
    }

    /// Returns the combined value, given the values of its composing base field polynomials at that
    /// point.
    fn from_partial_evals(evals: [QM31; QM31_EXTENSION_DEGREE]) -> QM31 {
        let [e0, e1, e2, e3] = evals;
        e0
            + e1 * qm31_const::<0, 1, 0, 0>()
            + e2 * qm31_const::<0, 0, 1, 0>()
            + e3 * qm31_const::<0, 0, 0, 1>()
    }
}

#[cfg(not(feature: "qm31_opcode"))]
pub impl QM31Impl of QM31Trait {
    #[inline]
    fn from_array(arr: [M31InnerT; 4]) -> QM31 {
        let [a, b, c, d] = arr;
        QM31 {
            a: CM31 { a: M31 { inner: a }, b: M31 { inner: b } },
            b: CM31 { a: M31 { inner: c }, b: M31 { inner: d } },
        }
    }

    #[inline]
    fn to_array(self: QM31) -> [M31InnerT; 4] {
        [self.a.a.inner, self.a.b.inner, self.b.a.inner, self.b.b.inner]
    }

    #[inline]
    fn mul_m31(self: QM31, rhs: M31) -> QM31 {
        QM31 {
            a: CM31 { a: self.a.a * rhs, b: self.a.b * rhs },
            b: CM31 { a: self.b.a * rhs, b: self.b.b * rhs },
        }
    }

    #[inline]
    fn mul_cm31(self: QM31, rhs: CM31) -> QM31 {
        QM31 { a: self.a * rhs, b: self.b * rhs }
    }

    #[inline]
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
        e0
            + e1 * qm31_const::<0, 1, 0, 0>()
            + e2 * qm31_const::<0, 0, 1, 0>()
            + e3 * qm31_const::<0, 0, 0, 1>()
    }
}


pub impl QM31Add of core::traits::Add<QM31> {
    #[inline(always)]
    #[cfg(feature: "qm31_opcode")]
    fn add(lhs: QM31, rhs: QM31) -> QM31 {
        QM31 { inner: lhs.inner + rhs.inner }
    }

    #[inline(never)]
    #[cfg(not(feature: "qm31_opcode"))]
    fn add(lhs: QM31, rhs: QM31) -> QM31 {
        QM31 { a: lhs.a + rhs.a, b: lhs.b + rhs.b }
    }
}

pub impl QM31Sub of core::traits::Sub<QM31> {
    #[inline(always)]
    #[cfg(feature: "qm31_opcode")]
    fn sub(lhs: QM31, rhs: QM31) -> QM31 {
        QM31 { inner: lhs.inner - rhs.inner }
    }

    #[inline(never)]
    #[cfg(not(feature: "qm31_opcode"))]
    fn sub(lhs: QM31, rhs: QM31) -> QM31 {
        QM31 { a: lhs.a - rhs.a, b: lhs.b - rhs.b }
    }
}

pub impl QM31Mul of core::traits::Mul<QM31> {
    #[inline(always)]
    #[cfg(feature: "qm31_opcode")]
    fn mul(lhs: QM31, rhs: QM31) -> QM31 {
        QM31 { inner: lhs.inner * rhs.inner }
    }

    #[inline(never)]
    #[cfg(not(feature: "qm31_opcode"))]
    fn mul(lhs: QM31, rhs: QM31) -> QM31 {
        const R: CM31 = CM31 { a: M31 { inner: 2 }, b: M31 { inner: 1 } };
        // (a + bu) * (c + du) = (ac + rbd) + (ad + bc)u.
        QM31 { a: lhs.a * rhs.a + R * lhs.b * rhs.b, b: lhs.a * rhs.b + lhs.b * rhs.a }
    }
}

pub impl QM31Div of core::traits::Div<QM31> {
    #[inline(always)]
    #[cfg(feature: "qm31_opcode")]
    fn div(lhs: QM31, rhs: QM31) -> QM31 {
        QM31 { inner: lhs.inner / rhs.inner.try_into().unwrap() }
    }

    #[inline]
    #[cfg(not(feature: "qm31_opcode"))]
    fn div(lhs: QM31, rhs: QM31) -> QM31 {
        lhs * rhs.inverse()
    }
}

pub impl QM31AddAssign of AddAssign<QM31, QM31> {
    #[inline(always)]
    fn add_assign(ref self: QM31, rhs: QM31) {
        self = self + rhs
    }
}

pub impl QM31SubAssign of SubAssign<QM31, QM31> {
    #[inline(always)]
    fn sub_assign(ref self: QM31, rhs: QM31) {
        self = self - rhs
    }
}

pub impl QM31MulAssign of MulAssign<QM31, QM31> {
    #[inline(always)]
    fn mul_assign(ref self: QM31, rhs: QM31) {
        self = self * rhs
    }
}

// TODO: Use NonZero<QM31>
impl QM31InvertibleImpl of Invertible<QM31> {
    #[inline(always)]
    #[cfg(feature: "qm31_opcode")]
    fn inverse(self: QM31) -> QM31 {
        qm31_const::<1, 0, 0, 0>() / self
    }

    #[cfg(not(feature: "qm31_opcode"))]
    fn inverse(self: QM31) -> QM31 {
        assert!(self.is_non_zero());
        let b2 = self.b * self.b;
        let ib2 = CM31 { a: -b2.b, b: b2.a };
        let denom = self.a * self.a - (b2 + b2 + ib2);
        let denom_inverse = denom.inverse();
        QM31 { a: self.a * denom_inverse, b: -self.b * denom_inverse }
    }
}

#[cfg(feature: "qm31_opcode")]
pub impl QM31Serde of Serde<QM31> {
    fn serialize(self: @QM31, ref output: Array<felt252>) {
        let [a, b, c, d] = self.to_array();
        output.append(a.into());
        output.append(b.into());
        output.append(c.into());
        output.append(d.into());
    }

    fn deserialize(ref serialized: Span<felt252>) -> Option<QM31> {
        let a: M31InnerT = Serde::deserialize(ref serialized)?;
        let b: M31InnerT = Serde::deserialize(ref serialized)?;
        let c: M31InnerT = Serde::deserialize(ref serialized)?;
        let d: M31InnerT = Serde::deserialize(ref serialized)?;
        Some(QM31Trait::from_array([a, b, c, d]))
    }
}


pub impl QM31Zero of Zero<QM31> {
    #[inline]
    fn zero() -> QM31 {
        qm31_const::<0, 0, 0, 0>()
    }

    #[inline]
    #[cfg(feature: "qm31_opcode")]
    fn is_zero(self: @QM31) -> bool {
        self.inner.is_zero()
    }

    #[cfg(not(feature: "qm31_opcode"))]
    fn is_zero(self: @QM31) -> bool {
        (*self).a.is_zero() && (*self).b.is_zero()
    }

    #[inline]
    #[cfg(feature: "qm31_opcode")]
    fn is_non_zero(self: @QM31) -> bool {
        (*self).inner.is_non_zero()
    }

    #[cfg(not(feature: "qm31_opcode"))]
    fn is_non_zero(self: @QM31) -> bool {
        (*self).a.is_non_zero() || (*self).b.is_non_zero()
    }
}

pub impl QM31One of One<QM31> {
    #[inline]
    fn one() -> QM31 {
        qm31_const::<1, 0, 0, 0>()
    }

    #[cfg(feature: "qm31_opcode")]
    fn is_one(self: @QM31) -> bool {
        @core::qm31::qm31_const::<1, 0, 0, 0>() == self.inner
    }

    #[cfg(not(feature: "qm31_opcode"))]
    fn is_one(self: @QM31) -> bool {
        (*self).a.is_one() && (*self).b.is_zero()
    }

    #[cfg(feature: "qm31_opcode")]
    fn is_non_one(self: @QM31) -> bool {
        @core::qm31::qm31_const::<1, 0, 0, 0>() != self.inner
    }

    #[cfg(not(feature: "qm31_opcode"))]
    fn is_non_one(self: @QM31) -> bool {
        (*self).a.is_non_one() || (*self).b.is_non_zero()
    }
}

pub impl M31IntoQM31 of core::traits::Into<M31, QM31> {
    #[inline]
    #[cfg(feature: "qm31_opcode")]
    fn into(self: M31) -> QM31 {
        QM31 { inner: self.inner.inner }
    }

    #[cfg(not(feature: "qm31_opcode"))]
    fn into(self: M31) -> QM31 {
        QM31 { a: self.into(), b: Zero::zero() }
    }
}

pub impl CM31IntoQM31 of core::traits::Into<CM31, QM31> {
    #[inline]
    #[cfg(feature: "qm31_opcode")]
    fn into(self: CM31) -> QM31 {
        QM31 { inner: self.inner.inner }
    }

    #[cfg(not(feature: "qm31_opcode"))]
    fn into(self: CM31) -> QM31 {
        QM31 { a: self, b: Zero::zero() }
    }
}

pub impl QM31Neg of Neg<QM31> {
    #[inline]
    #[cfg(feature: "qm31_opcode")]
    fn neg(a: QM31) -> QM31 {
        qm31_const::<0, 0, 0, 0>() - a
    }

    #[inline]
    #[cfg(not(feature: "qm31_opcode"))]
    fn neg(a: QM31) -> QM31 {
        QM31 { a: -a.a, b: -a.b }
    }
}

impl QM31PartialOrd of PartialOrd<QM31> {
    #[inline]
    #[cfg(feature: "qm31_opcode")]
    fn lt(lhs: QM31, rhs: QM31) -> bool {
        let [la, lb, lc, ld] = lhs.to_array();
        let [ra, rb, rc, rd] = rhs.to_array();
        let la: u32 = upcast(la);
        let lb: u32 = upcast(lb);
        let lc: u32 = upcast(lc);
        let ld: u32 = upcast(ld);
        let ra: u32 = upcast(ra);
        let rb: u32 = upcast(rb);
        let rc: u32 = upcast(rc);
        let rd: u32 = upcast(rd);
        la < ra
            || (la == ra && lb < rb)
            || (la == ra && lb == rb && lc < rc)
            || (la == ra && lb == rb && lc == rc && ld < rd)
    }

    #[inline]
    #[cfg(not(feature: "qm31_opcode"))]
    fn lt(lhs: QM31, rhs: QM31) -> bool {
        lhs.a < rhs.a || (lhs.a == rhs.a && lhs.b < rhs.b)
    }
}

#[derive(Copy, Drop)]
#[cfg(feature: "qm31_opcode")]
struct UnreducedQM31 {
    inner: QM31,
}

/// Stores an unreduced [`QM31`] with each coordinate stored as a `felt252`.
#[derive(Copy, Drop, Debug)]
#[cfg(not(feature: "qm31_opcode"))]
struct UnreducedQM31 {
    a: felt252,
    b: felt252,
    c: felt252,
    d: felt252,
}

pub trait UnreducedQM31Trait {
    fn reduce(self: UnreducedQM31) -> QM31;
}

#[cfg(feature: "qm31_opcode")]
impl UnreducedQM31Impl of UnreducedQM31Trait {
    #[inline]
    fn reduce(self: UnreducedQM31) -> QM31 {
        self.inner
    }
}

#[cfg(not(feature: "qm31_opcode"))]
impl UnreducedQM31Impl of UnreducedQM31Trait {
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
    #[cfg(feature: "qm31_opcode")]
    fn sub(lhs: UnreducedQM31, rhs: UnreducedQM31) -> UnreducedQM31 {
        UnreducedQM31 { inner: lhs.inner - rhs.inner }
    }

    #[inline]
    #[cfg(not(feature: "qm31_opcode"))]
    fn sub(lhs: UnreducedQM31, rhs: UnreducedQM31) -> UnreducedQM31 {
        UnreducedQM31 { a: lhs.a - rhs.a, b: lhs.b - rhs.b, c: lhs.c - rhs.c, d: lhs.d - rhs.d }
    }
}

impl UnreducedQM31Add of Add<UnreducedQM31> {
    #[inline]
    #[cfg(feature: "qm31_opcode")]
    fn add(lhs: UnreducedQM31, rhs: UnreducedQM31) -> UnreducedQM31 {
        UnreducedQM31 { inner: lhs.inner + rhs.inner }
    }

    #[inline]
    #[cfg(not(feature: "qm31_opcode"))]
    fn add(lhs: UnreducedQM31, rhs: UnreducedQM31) -> UnreducedQM31 {
        UnreducedQM31 { a: lhs.a + rhs.a, b: lhs.b + rhs.b, c: lhs.c + rhs.c, d: lhs.d + rhs.d }
    }
}

impl QM31IntoUnreducedQM31 of Into<QM31, UnreducedQM31> {
    #[inline]
    #[cfg(feature: "qm31_opcode")]
    fn into(self: QM31) -> UnreducedQM31 {
        UnreducedQM31 { inner: self }
    }

    #[inline]
    #[cfg(not(feature: "qm31_opcode"))]
    fn into(self: QM31) -> UnreducedQM31 {
        UnreducedQM31 {
            a: self.a.a.inner.into(),
            b: self.a.b.inner.into(),
            c: self.b.a.inner.into(),
            d: self.b.b.inner.into(),
        }
    }
}

#[derive(Copy, Drop, Debug)]
#[cfg(feature: "qm31_opcode")]
pub struct PackedUnreducedQM31 {
    inner: QM31,
}

/// Stores an unreduced [`QM31`] packed into two `felt252`.
///
/// Is more efficient than [`UnreducedQM31`] since only requires two felt252 operations per addition
/// or M31 multiplication vs 4. These operations happen in the main FRI quotient loop which .
#[derive(Copy, Drop, Debug)]
#[cfg(not(feature: "qm31_opcode"))]
pub struct PackedUnreducedQM31 {
    pub a: PackedUnreducedCM31,
    pub b: PackedUnreducedCM31,
}

pub trait PackedUnreducedQM31Trait {
    fn mul_m31(self: PackedUnreducedQM31, rhs: UnreducedM31) -> PackedUnreducedQM31;
    /// Returns a zero element with each coordinate set to `P*P*P`.
    fn large_zero() -> PackedUnreducedQM31;
    fn reduce(self: PackedUnreducedQM31) -> QM31;
}

#[cfg(feature: "qm31_opcode")]
pub impl PackedUnreducedQM31Impl of PackedUnreducedQM31Trait {
    #[inline]
    fn mul_m31(self: PackedUnreducedQM31, rhs: UnreducedM31) -> PackedUnreducedQM31 {
        PackedUnreducedQM31 { inner: self.inner * rhs.inner.inner }
    }

    /// Returns a zero element with each coordinate set to `P*P*P`.
    #[inline]
    fn large_zero() -> PackedUnreducedQM31 {
        PackedUnreducedQM31 { inner: Zero::zero() }
    }

    #[inline]
    fn reduce(self: PackedUnreducedQM31) -> QM31 {
        self.inner
    }
}

#[cfg(not(feature: "qm31_opcode"))]
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
    #[cfg(feature: "qm31_opcode")]
    fn add(lhs: PackedUnreducedQM31, rhs: PackedUnreducedQM31) -> PackedUnreducedQM31 {
        PackedUnreducedQM31 { inner: lhs.inner + rhs.inner }
    }

    #[inline]
    #[cfg(not(feature: "qm31_opcode"))]
    fn add(lhs: PackedUnreducedQM31, rhs: PackedUnreducedQM31) -> PackedUnreducedQM31 {
        PackedUnreducedQM31 { a: lhs.a + rhs.a, b: lhs.b + rhs.b }
    }
}

pub impl PackedUnreducedQM31Sub of Sub<PackedUnreducedQM31> {
    #[inline]
    #[cfg(feature: "qm31_opcode")]
    fn sub(lhs: PackedUnreducedQM31, rhs: PackedUnreducedQM31) -> PackedUnreducedQM31 {
        PackedUnreducedQM31 { inner: lhs.inner - rhs.inner }
    }

    #[inline]
    #[cfg(not(feature: "qm31_opcode"))]
    fn sub(lhs: PackedUnreducedQM31, rhs: PackedUnreducedQM31) -> PackedUnreducedQM31 {
        PackedUnreducedQM31 { a: lhs.a - rhs.a, b: lhs.b - rhs.b }
    }
}

pub impl QM31IntoPackedUnreducedQM31 of Into<QM31, PackedUnreducedQM31> {
    #[inline]
    #[cfg(feature: "qm31_opcode")]
    fn into(self: QM31) -> PackedUnreducedQM31 {
        PackedUnreducedQM31 { inner: self }
    }

    #[inline]
    #[cfg(not(feature: "qm31_opcode"))]
    fn into(self: QM31) -> PackedUnreducedQM31 {
        PackedUnreducedQM31 { a: self.a.into(), b: self.b.into() }
    }
}

#[derive(Copy, Drop, Debug)]
#[cfg(feature: "qm31_opcode")]
pub struct PackedUnreducedCM31 {
    pub inner: CM31,
}

/// An unreduced [`CM31`] packed into a single `felt252`.
#[derive(Copy, Drop, Debug)]
#[cfg(not(feature: "qm31_opcode"))]
pub struct PackedUnreducedCM31 {
    /// Stores a 128 bit and 124 bit unreduced M31 packed into a felt252 i.e. `a + (b << 128)`.
    pub inner: felt252,
}

pub trait PackedUnreducedCM31Trait {
    fn mul_m31(self: PackedUnreducedCM31, rhs: UnreducedM31) -> PackedUnreducedCM31;
    /// Returns a zero element with each coordinate set to `P*P*P`.
    fn large_zero() -> PackedUnreducedCM31;
    fn reduce(self: PackedUnreducedCM31) -> CM31;
}

#[cfg(feature: "qm31_opcode")]
pub impl PackedUnreducedCM31Impl of PackedUnreducedCM31Trait {
    #[inline]
    fn mul_m31(self: PackedUnreducedCM31, rhs: UnreducedM31) -> PackedUnreducedCM31 {
        PackedUnreducedCM31 { inner: CM31 { inner: self.inner.inner * rhs.inner.inner } }
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

#[cfg(not(feature: "qm31_opcode"))]
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
        CM31 { a: M31Trait::reduce_u128(a).into(), b: M31Trait::reduce_u128(b).into() }
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
    #[cfg(feature: "qm31_opcode")]
    fn into(self: CM31) -> PackedUnreducedCM31 {
        PackedUnreducedCM31 { inner: self }
    }

    #[inline]
    #[cfg(not(feature: "qm31_opcode"))]
    fn into(self: CM31) -> PackedUnreducedCM31 {
        const POW2_128: felt252 = 0x100000000000000000000000000000000;
        let a_felt: felt252 = self.a.into();
        let b_felt: felt252 = self.b.into();
        PackedUnreducedCM31 { inner: a_felt + b_felt * POW2_128 }
    }
}

impl QM31Dispaly of core::fmt::Display<QM31> {
    fn fmt(self: @QM31, ref f: core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        let [a, b, c, d] = (*self).to_array();
        let a: u32 = upcast(a);
        let b: u32 = upcast(b);
        let c: u32 = upcast(c);
        let d: u32 = upcast(d);
        write!(f, "({} + {}i) + ({} + {}i)u", a, b, c, d)
    }
}

#[cfg(feature: "qm31_opcode")]
impl QM31Debug of core::fmt::Debug<QM31> {
    fn fmt(self: @QM31, ref f: core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        QM31Dispaly::fmt(self, ref f)
    }
}

/// Returns a `qm31` given its values as constants.
#[inline(always)]
#[cfg(feature: "qm31_opcode")]
pub fn qm31_const<
    const W0: M31InnerT, const W1: M31InnerT, const W2: M31InnerT, const W3: M31InnerT,
>() -> QM31 nopanic {
    QM31 { inner: core::qm31::qm31_const::<W0, W1, W2, W3>() }
}

#[inline(always)]
#[cfg(not(feature: "qm31_opcode"))]
pub fn qm31_const<
    const W0: M31InnerT, const W1: M31InnerT, const W2: M31InnerT, const W3: M31InnerT,
>() -> QM31 nopanic {
    QM31 {
        a: CM31 { a: M31 { inner: W0 }, b: M31 { inner: W1 } },
        b: CM31 { a: M31 { inner: W2 }, b: M31 { inner: W3 } },
    }
}

#[cfg(test)]
mod tests {
    use super::super::m31::{M31InnerT, M31Trait, P_U32 as P, m31};
    use super::super::Invertible;
    use super::{
        PackedUnreducedQM31Impl, QM31, QM31Impl, QM31IntoPackedUnreducedQM31, QM31Trait, qm31_const,
    };

    const P_NEG_71: M31InnerT = 2147483576;
    const P_NEG_16: M31InnerT = 2147483631;

    #[test]
    fn test_QM31() {
        let qm0 = qm31_const::<1, 2, 3, 4>();
        let qm1 = qm31_const::<4, 5, 6, 7>();
        let m = m31(8);
        let qm = Into::<_, QM31>::into(m);
        let qm0_x_qm1 = qm31(P - 71, 93, P - 16, 50);

        assert_eq!(qm0 + qm1, qm31_const::<5, 7, 9, 11>());
        assert_eq!(qm1 + m.into(), qm1 + qm);
        assert_eq!(qm0 * qm1, qm0_x_qm1);
        assert_eq!(qm1 * m.into(), qm1 * qm);
        assert_eq!(-qm0, qm31(P - 1, P - 2, P - 3, P - 4));
        assert_eq!(qm0 - qm1, qm31(P - 3, P - 3, P - 3, P - 3));
        assert_eq!(qm1 - m.into(), qm1 - qm);
        assert_eq!(qm0_x_qm1 * qm1.inverse(), qm31_const::<1, 2, 3, 4>());
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

    #[cfg(feature: "qm31_opcode")]
    fn qm31(a: u32, b: u32, c: u32, d: u32) -> QM31 {
        let a = M31Trait::reduce_u32(a);
        let b = M31Trait::reduce_u32(b);
        let c = M31Trait::reduce_u32(c);
        let d = M31Trait::reduce_u32(d);
        QM31Impl::from_array([a, b, c, d])
    }

    #[cfg(not(feature: "qm31_opcode"))]
    fn qm31(a: u32, b: u32, c: u32, d: u32) -> QM31 {
        QM31 {
            a: super::CM31 { a: a.into(), b: b.into() },
            b: super::CM31 { a: c.into(), b: d.into() },
        }
    }
}
