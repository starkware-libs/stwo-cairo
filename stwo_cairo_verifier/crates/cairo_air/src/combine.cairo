use bounded_int::impls::*;
use bounded_int::{
    AddHelper, BoundedInt, DivRemHelper, MulHelper, NZ_U9_SHIFT, add, bounded_int_mul, div_rem,
    upcast,
};
use stwo_verifier_core::fields::m31::{M31InnerT, M31Trait};
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
fn split_u32_to_4_chunks(input: u32, shift: NonZero<u9>) -> (U23_BOUNDED_INT, u9, u9, u9) {
    let (q, lsb) = div_rem::<u32, _, _>(input, shift);
    let (q, r0) = div_rem::<u32, _, _>(upcast(q), NZ_U9_SHIFT);
    let (q, r1) = div_rem::<u32, _, _>(upcast(q), NZ_U9_SHIFT);
    (upcast(q), r1, r0, upcast(lsb))
}

/// Splits input into (msb, 3*u9, lsb) where lsb has log_2(shift) bits.
fn split_u32_to_5_chunks(input: u32, shift: NonZero<u9>) -> (U23_BOUNDED_INT, u9, u9, u9, u9) {
    let (q, lsb) = div_rem::<u32, _, _>(input, shift);
    let (q, r0) = div_rem::<u32, _, _>(upcast(q), NZ_U9_SHIFT);
    let (q, r1) = div_rem::<u32, _, _>(upcast(q), NZ_U9_SHIFT);
    let (q, r2) = div_rem::<u32, _, _>(upcast(q), NZ_U9_SHIFT);
    (upcast(q), r2, r1, r0, upcast(lsb))
}

/// Update sum to sum * alpha + value.
fn horner_step(ref sum: QM31, value: u9, alpha: QM31) {
    sum = sum * alpha + M31Trait::new(upcast(value)).into();
}

/// Same as `horner_step`, but the `value` is given as `msb * shift + lsb`.
fn horner_step_with_split_input(
    ref sum: QM31, msb: u9, lsb: U23_BOUNDED_INT, shift: u9, alpha: QM31,
) {
    sum = sum * alpha
        + M31Trait::new(upcast::<_, M31InnerT>(add(bounded_int_mul(msb, shift), lsb))).into();
}

/// An unrolled implementation for combining a felt252 value as part of the combine_id_to_value
/// flow.
/// This function takes a [u32; 8] value representing a felt252 in little endian, converts it to a
/// [u9; 28] array of coefficients, and uses Horner evaluation to compute the value of the
/// corresponding polynomial at alpha.
///
/// Note that Horner evaluation expects the coefficients in reverse order, which makes the [u32; 8]
/// -> [u9; 28] transformation somewhat unnatural.
pub fn combine_felt252(value: [u32; 8], alpha: QM31) -> QM31 {
    let [v0, v1, v2, v3, v4, v5, v6, v7] = value;

    // Since the value is felt252, we ignore the 4 most significant bits.
    // Take 4 + 27 + 1 bits from v7
    let (_, l27, l26, l25, l24_high) = split_u32_to_5_chunks(v7, 0x2);

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

    // Take 5 + 27 + 0 bits from v0
    let (l3_low, l2, l1, l0, _) = split_u32_to_5_chunks(v0, 1);
    horner_step_with_split_input(ref sum, l3_high, l3_low, 0x20, alpha);

    horner_step(ref sum, l2, alpha);
    horner_step(ref sum, l1, alpha);
    horner_step(ref sum, l0, alpha);

    sum
}
