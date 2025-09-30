/// Unreduced QM31 implementation.
/// This module is used to implement more efficient fused_mul_add and fused_mul_sub for QM31 when
/// the opcode is not available.
/// `PackedUnreducedQM31` is less efficient for this use case because the saving in the additions
/// are outweighed by the additional packing and unpacking.

use super::super::QM31;
use super::super::super::super::cm31::CM31;
use super::super::super::super::m31::M31Trait;

/// Stores an unreduced [`QM31`] with each coordinate stored as a `felt252`.
#[derive(Copy, Drop, Debug)]
struct UnreducedQM31 {
    a: felt252,
    b: felt252,
    c: felt252,
    d: felt252,
}

fn reduce(value: UnreducedQM31) -> QM31 {
    QM31 {
        a: CM31 {
            a: M31Trait::reduce_u128(value.a.try_into().unwrap()).into(),
            b: M31Trait::reduce_u128(value.b.try_into().unwrap()).into(),
        },
        b: CM31 {
            a: M31Trait::reduce_u128(value.c.try_into().unwrap()).into(),
            b: M31Trait::reduce_u128(value.d.try_into().unwrap()).into(),
        },
    }
}

// TODO(andrew): Consider Karatsuba.
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


#[inline]
pub fn fused_mul_add(a: QM31, b: QM31, c: QM31) -> QM31 {
    let mut mul_res = mul_unreduced(a, b);
    mul_res.a += c.a.a.inner.into();
    mul_res.b += c.a.b.inner.into();
    mul_res.c += c.b.a.inner.into();
    mul_res.d += c.b.b.inner.into();
    reduce(mul_res)
}

#[inline]
pub fn fused_mul_sub(a: QM31, b: QM31, c: QM31) -> QM31 {
    let mut mul_res = mul_unreduced(a, b);
    mul_res.a -= c.a.a.inner.into();
    mul_res.b -= c.a.b.inner.into();
    mul_res.c -= c.b.a.inner.into();
    mul_res.d -= c.b.b.inner.into();
    reduce(mul_res)
}
