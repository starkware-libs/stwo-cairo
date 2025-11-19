/// Unreduced QM31 implementation.
/// This module is used to implement more efficient fused_mul_add and fused_mul_sub for QM31 when
/// the opcode is not available.
/// `PackedUnreducedQM31` is less efficient for this use case because the saving in the additions
/// are outweighed by the additional packing and unpacking.

use super::super::super::super::cm31::CM31;
use super::super::super::super::m31::{M31, M31Trait};
use super::super::{QM31, QM31Trait};

/// Stores an unreduced [`CM31`] with each coordinate stored as a `felt252`.
#[derive(Copy, Drop, Debug)]
struct UnreducedCM31 {
    a: felt252,
    b: felt252,
}

/// Stores an unreduced [`QM31`] with each coordinate stored as a `UnreducedCM31`.
#[derive(Copy, Drop, Debug)]
struct UnreducedQM31 {
    a: UnreducedCM31,
    b: UnreducedCM31,
}

/// Equals `P * P * 16`.
/// Used to offset computation results involving negative numbers before reduction.
const PP16: felt252 = 0x3fffffff000000010;

#[inline]
fn reduce_m31(value: felt252) -> M31 {
    M31Trait::reduce_u128(value.try_into().unwrap())
}

#[inline]
fn reduce_cm31(value: UnreducedCM31) -> CM31 {
    CM31 { a: reduce_m31(value.a), b: reduce_m31(value.b) }
}

#[inline]
fn reduce_qm31(value: UnreducedQM31) -> QM31 {
    QM31 { a: reduce_cm31(value.a), b: reduce_cm31(value.b) }
}

#[inline]
pub fn mul_cm_unreduced(lhs: CM31, rhs: CM31) -> UnreducedCM31 {
    // `lhs` coordinates.
    let lhs_a: felt252 = lhs.a.into();
    let lhs_b: felt252 = lhs.b.into();

    // `rhs` coordinates.
    let rhs_a: felt252 = rhs.a.into();
    let rhs_b: felt252 = rhs.b.into();

    // lhs * rhs
    let a = lhs_a * rhs_a - lhs_b * rhs_b;
    let b = lhs_a * rhs_b + lhs_b * rhs_a;

    // Note that the `a` element is possibly negative and needs to be offset before reduction.
    UnreducedCM31 { a, b }
}

// TODO(andrew): Consider Karatsuba.
#[inline]
fn mul_qm_unreduced(lhs: QM31, rhs: QM31) -> UnreducedQM31 {
    let aa_t_ba = mul_cm_unreduced(lhs.a, rhs.a);
    let aa_t_bb = mul_cm_unreduced(lhs.a, rhs.b);
    let ab_t_ba = mul_cm_unreduced(lhs.b, rhs.a);
    let ab_t_bb = mul_cm_unreduced(lhs.b, rhs.b);
    // Multiply the u*u block by r = u^2 = i + 2.
    let r_ab_t_bb = {
        let (a, b) = (ab_t_bb.a, ab_t_bb.b);
        let (a, b) = (a + a - b, a + b + b);
        UnreducedCM31 { a, b }
    };

    // All use cases require offsetting the entries prior to the reduction.
    // Limbs aa, ab, ba all involve subtraction in the arithmetic; limb bb does not.
    let res_aa = aa_t_ba.a + r_ab_t_bb.a + PP16;
    let res_ab = aa_t_ba.b + r_ab_t_bb.b + PP16;
    let res_ba = aa_t_bb.a + ab_t_ba.a + PP16;
    let res_bb = aa_t_bb.b + ab_t_ba.b;

    UnreducedQM31 {
        a: UnreducedCM31 { a: res_aa, b: res_ab }, b: UnreducedCM31 { a: res_ba, b: res_bb },
    }
}

#[inline]
pub fn mul_cm_using_unreduced(a: CM31, b: CM31) -> CM31 {
    let mut mul_res = mul_cm_unreduced(a, b);
    mul_res.a += PP16;
    reduce_cm31(mul_res)
}

#[inline]
pub fn mul_qm_using_unreduced(a: QM31, b: QM31) -> QM31 {
    reduce_qm31(mul_qm_unreduced(a, b))
}

pub fn fused_quotient_denominator(px: QM31, py: QM31, dx: M31, dy: M31) -> CM31 {
    let px_aa: felt252 = px.a.a.into() - dx.into();
    let px_ab: felt252 = px.a.b.into();
    let px_ba: felt252 = px.b.a.into();
    let px_bb: felt252 = px.b.b.into();

    let py_aa: felt252 = py.a.a.into() - dy.into();
    let py_ab: felt252 = py.a.b.into();
    let py_ba: felt252 = py.b.a.into();
    let py_bb: felt252 = py.b.b.into();

    // px.a * py.b - px.b * py.a
    let a = (px_aa * py_ba - px_ab * py_bb) - (px_ba * py_aa - px_bb * py_ab) + PP16;
    let b = (px_aa * py_bb + px_ab * py_ba) - (px_ba * py_ab + px_bb * py_aa) + PP16;

    reduce_cm31(UnreducedCM31 { a, b })
}

#[inline]
pub fn fused_mul_add(a: QM31, b: QM31, c: QM31) -> QM31 {
    let mut mul_res = mul_qm_unreduced(a, b);
    mul_res.a.a += c.a.a.into();
    mul_res.a.b += c.a.b.into();
    mul_res.b.a += c.b.a.into();
    mul_res.b.b += c.b.b.into();
    reduce_qm31(mul_res)
}

#[inline]
pub fn fused_mul_sub(a: QM31, b: QM31, c: QM31) -> QM31 {
    let mut mul_res = mul_qm_unreduced(a, b);
    mul_res.a.a -= c.a.a.into();
    mul_res.a.b -= c.a.b.into();
    mul_res.b.a -= c.b.a.into();
    mul_res.b.b -= c.b.b.into();
    mul_res.b.b += PP16;
    reduce_qm31(mul_res)
}

#[inline]
pub fn from_partial_evals(evals: [QM31; 4]) -> QM31 {
    let [e0, e1, e2, e3] = evals;
    let [e00, e01, e02, e03] = e0.to_fixed_array();
    let [e00, e01, e02, e03]: [felt252; 4] = [e00.into(), e01.into(), e02.into(), e03.into()];
    let [e10, e11, e12, e13] = e1.to_fixed_array();
    let [e10, e11, e12, e13]: [felt252; 4] = [e10.into(), e11.into(), e12.into(), e13.into()];
    let [e20, e21, e22, e23] = e2.to_fixed_array();
    let [e20, e21, e22, e23]: [felt252; 4] = [e20.into(), e21.into(), e22.into(), e23.into()];
    let [e30, e31, e32, e33] = e3.to_fixed_array();
    let [e30, e31, e32, e33]: [felt252; 4] = [e30.into(), e31.into(), e32.into(), e33.into()];

    // Multiply the u*u block by r = u^2 = i + 2.
    let (e22r, e23r) = (e22 + e22 - e23, e22 + e23 + e23);
    let (e32r, e33r) = (e32 + e32 - e33, e32 + e33 + e33);

    let res_aa = e00 - e11 + e22r - e33r + PP16;
    let res_ab = e01 + e10 + e23r + e32r + PP16;
    let res_ba = e02 - e13 + e20 - e31 + PP16;
    let res_bb = e03 + e12 + e21 + e30;

    let unreduced = UnreducedQM31 {
        a: UnreducedCM31 { a: res_aa, b: res_ab }, b: UnreducedCM31 { a: res_ba, b: res_bb },
    };
    reduce_qm31(unreduced)
}
