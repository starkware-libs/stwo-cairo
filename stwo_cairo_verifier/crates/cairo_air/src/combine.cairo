use bounded_int::impls::*;
use bounded_int::{
    AddHelper, BoundedInt, MulHelper, NZ_U9_SHIFT, add, bounded_int_mul, div_rem, upcast,
};
use stwo_verifier_core::fields::m31::M31Trait;
#[cfg(not(feature: "qm31_opcode"))]
use stwo_verifier_core::fields::m31::MulByM31Trait;
#[cfg(not(feature: "qm31_opcode"))]
use stwo_verifier_core::fields::qm31::PackedUnreducedQM31;
use stwo_verifier_core::fields::qm31::QM31;


// Use a short name in this file as it is used in many places.
type u9 = U9_BOUNDED_INT;

// (U9_SHIFT - 1)**2 = 0x3fc01.
type U9_BB_U9_BOUNDED = BoundedInt<0, 0x3fc01>;

pub impl MulU9ByU9 of MulHelper<u9, u9> {
    type Result = U9_BB_U9_BOUNDED;
}

pub impl ADD_U9_BB_U9_BOUNDED_TO_U23 of AddHelper<U9_BB_U9_BOUNDED, U23_BOUNDED_INT> {
    type Result = BoundedInt<0, 0x83fc00>;
}

/// Splits input into (msb, 2*u9, lsb) where lsb has log_2(shift) bits.
fn split_u32_to_4_chunks(
    input: u32, shift: NonZero<U9_PLUS_1_BOUNDED_INT>,
) -> (U23_BOUNDED_INT, u9, u9, u9) {
    let (q, lsb) = div_rem::<u32, _, _>(input, shift);
    let (q, r0) = div_rem::<u32, _, _>(upcast(q), NZ_U9_SHIFT);
    let (q, r1) = div_rem::<u32, _, _>(upcast(q), NZ_U9_SHIFT);
    (upcast(q), r1, r0, lsb)
}

/// Splits input into (msb, 3*u9, lsb) where lsb has log_2(shift) bits.
fn split_u32_to_5_chunks(
    input: u32, shift: NonZero<U9_PLUS_1_BOUNDED_INT>,
) -> (U23_BOUNDED_INT, u9, u9, u9, u9) {
    let (q, lsb) = div_rem::<u32, _, _>(input, shift);
    let (q, r0) = div_rem::<u32, _, _>(upcast(q), NZ_U9_SHIFT);
    let (q, r1) = div_rem::<u32, _, _>(upcast(q), NZ_U9_SHIFT);
    let (q, r2) = div_rem::<u32, _, _>(upcast(q), NZ_U9_SHIFT);
    (upcast(q), r2, r1, r0, lsb)
}

/// Update sum to sum * alpha + value.
fn horner_step(ref sum: QM31, value: u9, alpha: QM31) {
    sum = sum * alpha + M31Trait::new(upcast(value)).into();
}

/// Same as `horner_step`, but the `value` is given as `msb * lsb_bound + lsb`.
fn horner_step_with_split_input(
    ref sum: QM31, msb: u9, lsb: U23_BOUNDED_INT, lsb_bound: u9, alpha: QM31,
) {
    let value = add(bounded_int_mul(msb, lsb_bound), lsb);
    sum = sum * alpha + M31Trait::new(upcast(value)).into();
}

/// An unrolled implementation for combining a `felt252` value within the
/// `combine_id_to_value` flow.
///
/// Computes:
///     ∑ values[i] * α^(i+1)
///
/// The function:
/// - Interprets the input `[u32; 8]` as a `felt252` in little-endian form.
/// - transforms it into a `[u9; 28]` array of polynomial coefficients.
/// - Applies Horner’s method to evaluate the polynomial at `α`.
/// - Multiplying the final Horner evaluation result by `α`.
///
/// Note:
/// Horner’s method requires coefficients in reverse order, which makes the
/// `[u32; 8]` → `[u9; 28]` transformation somewhat unintuitive.
#[cfg(feature: "qm31_opcode")]
pub fn combine_felt252(value: [u32; 8], alpha: QM31) -> QM31 {
    let [v0, v1, v2, v3, v4, v5, v6, v7] = value;

    // Since the value is felt252, we ignore the 4 most significant bits.
    // Take 4 + 27 + 1 bits from v7
    let (l28, l27, l26, l25, l24_high) = split_u32_to_5_chunks(v7, 0x2);

    // `value` is intended to represent a felt252, so its upper 4 bits must be zero.
    // This ensures uniqueness modulo the felt252 prime.
    // Note, however, that the 252-bit [u32; 8] representation is not unique;
    // for example, 0 can be represented both as 0 and as the felt252 prime.
    assert!(l28 == 0);

    let mut sum: QM31 = M31Trait::new(upcast(l27)).into();
    horner_step(ref sum, l26, alpha);
    horner_step(ref sum, l25, alpha);

    // Take 8 + 18 + 6 bits from v6
    let (l24_low, l23, l22, l21_high) = split_u32_to_4_chunks(v6, 0x40);
    horner_step_with_split_input(ref sum, l24_high, l24_low, 0x100, alpha);

    horner_step(ref sum, l23, alpha);
    horner_step(ref sum, l22, alpha);

    // Take 3 + 27 + 2 bits from v5
    let (l21_low, l20, l19, l18, l17_high) = split_u32_to_5_chunks(v5, 0x4);
    horner_step_with_split_input(ref sum, l21_high, l21_low, 0x8, alpha);

    horner_step(ref sum, l20, alpha);
    horner_step(ref sum, l19, alpha);
    horner_step(ref sum, l18, alpha);

    // Take 7 + 18 + 7 bits from v4
    let (l17_low, l16, l15, l14_high) = split_u32_to_4_chunks(v4, 0x80);
    horner_step_with_split_input(ref sum, l17_high, l17_low, 0x80, alpha);

    horner_step(ref sum, l16, alpha);
    horner_step(ref sum, l15, alpha);

    // Take 2 + 27 + 3 bits from v3
    let (l14_low, l13, l12, l11, l10_high) = split_u32_to_5_chunks(v3, 0x8);
    horner_step_with_split_input(ref sum, l14_high, l14_low, 0x4, alpha);

    horner_step(ref sum, l13, alpha);
    horner_step(ref sum, l12, alpha);
    horner_step(ref sum, l11, alpha);

    // Take 6 + 18 + 8 bits from v2
    let (l10_low, l9, l8, l17_high) = split_u32_to_4_chunks(v2, 0x100);
    horner_step_with_split_input(ref sum, l10_high, l10_low, 0x40, alpha);

    horner_step(ref sum, l9, alpha);
    horner_step(ref sum, l8, alpha);

    // Take 1 + 27 + 4 bits from v1
    let (l17_low, l6, l5, l4, l3_high) = split_u32_to_5_chunks(v1, 0x10);
    horner_step_with_split_input(ref sum, l17_high, l17_low, 0x2, alpha);

    horner_step(ref sum, l6, alpha);
    horner_step(ref sum, l5, alpha);
    horner_step(ref sum, l4, alpha);

    // Take 5 + 18 + 9 bits from v0
    let (l3_low, l2, l1, l0) = split_u32_to_4_chunks(v0, 0x200);
    horner_step_with_split_input(ref sum, l3_high, l3_low, 0x20, alpha);

    horner_step(ref sum, l2, alpha);
    horner_step(ref sum, l1, alpha);
    horner_step(ref sum, l0, alpha);

    // Multiply by alpha to be consistent with the no-opcode version.
    sum * alpha
}

/// The no-opcode version of `combine_felt252`.
///
/// This function splits the 252-bit value into chunks and combines them using the provided
/// array of precomputed `PackedUnreducedQM31` alpha powers, efficiently evaluating the
/// polynomial representation of the value.
///
/// The function works backwards in order to be similar to the implementation with qm31_opcode.
#[cfg(not(feature: "qm31_opcode"))]
pub fn combine_felt252(
    value: [u32; 8], alphas: Box<[PackedUnreducedQM31; 28]>,
) -> PackedUnreducedQM31 {
    let [
        a1,
        a2,
        a3,
        a4,
        a5,
        a6,
        a7,
        a8,
        a9,
        a10,
        a11,
        a12,
        a13,
        a14,
        a15,
        a16,
        a17,
        a18,
        a19,
        a20,
        a21,
        a22,
        a23,
        a24,
        a25,
        a26,
        a27,
        a28,
    ]: [PackedUnreducedQM31; 28] =
        alphas
        .unbox();
    let [v0, v1, v2, v3, v4, v5, v6, v7] = value;

    // Take 4 + 27 + 1 bits from v7
    let (l28, l27, l26, l25, l24_high) = split_u32_to_5_chunks(v7, 0x2);

    // `value` is intended to represent a felt252, so its upper 4 bits must be zero.
    // This ensures uniqueness modulo the felt252 prime.
    // Note, however, that the 252-bit [u32; 8] representation is not unique;
    // for example, 0 can be represented both as 0 and as the felt252 prime.
    assert!(l28 == 0);

    let mut sum = a28.mul_m31(M31Trait::new(upcast(l27)));
    sum += a27.mul_m31(M31Trait::new(upcast(l26)));
    sum += a26.mul_m31(M31Trait::new(upcast(l25)));

    // Take 8 + 18 + 6 bits from v6
    let (l24_low, l23, l22, l21_high) = split_u32_to_4_chunks(v6, 0x40);
    let l24 = add(bounded_int_mul(l24_high, 0x100), l24_low);

    sum += a25.mul_m31(M31Trait::new(upcast(l24)));
    sum += a24.mul_m31(M31Trait::new(upcast(l23)));
    sum += a23.mul_m31(M31Trait::new(upcast(l22)));

    // Take 3 + 27 + 2 bits from v5
    let (l21_low, l20, l19, l18, l17_high) = split_u32_to_5_chunks(v5, 0x4);
    let l21 = add(bounded_int_mul(l21_high, 0x8), l21_low);
    sum += a22.mul_m31(M31Trait::new(upcast(l21)));
    sum += a21.mul_m31(M31Trait::new(upcast(l20)));
    sum += a20.mul_m31(M31Trait::new(upcast(l19)));
    sum += a19.mul_m31(M31Trait::new(upcast(l18)));

    // Take 7 + 18 + 7 bits from v4
    let (l17_low, l16, l15, l14_high) = split_u32_to_4_chunks(v4, 0x80);
    let l17 = add(bounded_int_mul(l17_high, 0x80), l17_low);
    sum += a18.mul_m31(M31Trait::new(upcast(l17)));
    sum += a17.mul_m31(M31Trait::new(upcast(l16)));
    sum += a16.mul_m31(M31Trait::new(upcast(l15)));

    // Take 2 + 27 + 3 bits from v3
    let (l14_low, l13, l12, l11, l10_high) = split_u32_to_5_chunks(v3, 0x8);
    let l14 = add(bounded_int_mul(l14_high, 0x4), l14_low);
    sum += a15.mul_m31(M31Trait::new(upcast(l14)));
    sum += a14.mul_m31(M31Trait::new(upcast(l13)));
    sum += a13.mul_m31(M31Trait::new(upcast(l12)));
    sum += a12.mul_m31(M31Trait::new(upcast(l11)));

    // Take 6 + 18 + 8 bits from v2
    let (l10_low, l9, l8, l7_high) = split_u32_to_4_chunks(v2, 0x100);
    let l10 = add(bounded_int_mul(l10_high, 0x40), l10_low);
    sum += a11.mul_m31(M31Trait::new(upcast(l10)));
    sum += a10.mul_m31(M31Trait::new(upcast(l9)));
    sum += a9.mul_m31(M31Trait::new(upcast(l8)));

    // Take 1 + 27 + 4 bits from v1
    let (l7_low, l6, l5, l4, l3_high) = split_u32_to_5_chunks(v1, 0x10);
    let l7 = add(bounded_int_mul(l7_high, 0x2), l7_low);
    sum += a8.mul_m31(M31Trait::new(upcast(l7)));
    sum += a7.mul_m31(M31Trait::new(upcast(l6)));
    sum += a6.mul_m31(M31Trait::new(upcast(l5)));
    sum += a5.mul_m31(M31Trait::new(upcast(l4)));

    // Take 5 + 18 + 9 bits from v0
    let (l3_low, l2, l1, l0) = split_u32_to_4_chunks(v0, 0x200);
    let l3 = add(bounded_int_mul(l3_high, 0x20), l3_low);
    sum += a4.mul_m31(M31Trait::new(upcast(l3)));
    sum += a3.mul_m31(M31Trait::new(upcast(l2)));
    sum += a2.mul_m31(M31Trait::new(upcast(l1)));
    sum += a1.mul_m31(M31Trait::new(upcast(l0)));

    sum
}
