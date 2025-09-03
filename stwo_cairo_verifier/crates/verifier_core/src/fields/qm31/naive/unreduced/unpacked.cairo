/// Unreduced QM31 implementation.
/// This module is used to implement more efficient fused_mul_add and fused_mul_sub for QM31 when
/// the opcode is not available.
/// `PackedUnreducedQM31` is less efficient for this use case because the saving in the additions
/// are outweighed by the additional packing and unpacking.

use super::super::QM31;
use super::super::super::super::cm31::CM31;
use super::super::super::super::m31::{M31, M31Trait};

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
    /// Equals `P * P * 16`.
    const PP16: felt252 = 0x3fffffff000000010;

    let aa_t_ba = mul_cm_unreduced(lhs.a, rhs.a);
    let aa_t_bb = mul_cm_unreduced(lhs.a, rhs.b);
    let ab_t_ba = mul_cm_unreduced(lhs.b, rhs.a);
    let ab_t_bb = mul_cm_unreduced(lhs.b, rhs.b);
    let r_ab_t_bb = {
        let (a, b) = (ab_t_bb.a, ab_t_bb.b);
        let (a, b) = (a + a - b, a + b + b);
        UnreducedCM31 { a, b }
    };

    // All use cases require offsetting the entries prior to the reduction.
    UnreducedQM31 {
        a: UnreducedCM31 { a: PP16 + aa_t_ba.a + r_ab_t_bb.a, b: PP16 + aa_t_ba.b + r_ab_t_bb.b },
        b: UnreducedCM31 { a: PP16 + aa_t_bb.a + ab_t_ba.a, b: PP16 + aa_t_bb.b + ab_t_ba.b },
    }
}

#[inline]
pub fn mul_cm_using_unreduced(a: CM31, b: CM31) -> CM31 {
    /// Equals `P * P * 16`.
    const PP16: felt252 = 0x3fffffff000000010;

    let mut mul_res = mul_cm_unreduced(a, b);
    mul_res.a += PP16;
    reduce_cm31(mul_res)
}

#[inline]
pub fn mul_qm_using_unreduced(a: QM31, b: QM31) -> QM31 {
    reduce_qm31(mul_qm_unreduced(a, b))
}

pub fn fused_quotient_denominator(px: QM31, py: QM31, dx: M31, dy: M31) -> CM31 {
    /// Equals `P * P * 16`.
    const PP16: felt252 = 0x3fffffff000000010;

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
    mul_res.a.a += c.a.a.inner.into();
    mul_res.a.b += c.a.b.inner.into();
    mul_res.b.a += c.b.a.inner.into();
    mul_res.b.b += c.b.b.inner.into();
    reduce_qm31(mul_res)
}

#[inline]
pub fn fused_mul_sub(a: QM31, b: QM31, c: QM31) -> QM31 {
    let mut mul_res = mul_qm_unreduced(a, b);
    mul_res.a.a -= c.a.a.inner.into();
    mul_res.a.b -= c.a.b.inner.into();
    mul_res.b.a -= c.b.a.inner.into();
    mul_res.b.b -= c.b.b.inner.into();
    reduce_qm31(mul_res)
}
