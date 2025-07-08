use bounded_int::impls::*;
use bounded_int::{BoundedInt, DivRemHelper, NZ_U9_SHIFT, div_rem, upcast};
use stwo_verifier_core::fields::m31::M31Trait;
use stwo_verifier_core::fields::qm31::QM31;

// Use a short name in this file as it is used in many places.
type u9 = U9_BOUNDED_INT;

/// Splits input into (msb, 2*u9, lsb) where lsb has log_2(shift) bits.
fn take18(input: u32, shift: NonZero<u9>) -> (u32, u9, u9, u9) {
    let (q, lsb) = div_rem::<u32, _, _>(input, shift);
    let (q, r0) = div_rem::<u32, _, _>(upcast(q), NZ_U9_SHIFT);
    let (q, r1) = div_rem::<u32, _, _>(upcast(q), NZ_U9_SHIFT);
    (upcast(q), r1, r0, upcast(lsb))
}

/// Splits input into (msb, 3*u9, lsb) where lsb has log_2(shift) bits.
fn take27(input: u32, shift: NonZero<u9>) -> (u32, u9, u9, u9, u9) {
    let (q, lsb) = div_rem::<u32, _, _>(input, shift);
    let (q, r0) = div_rem::<u32, _, _>(upcast(q), NZ_U9_SHIFT);
    let (q, r1) = div_rem::<u32, _, _>(upcast(q), NZ_U9_SHIFT);
    let (q, r2) = div_rem::<u32, _, _>(upcast(q), NZ_U9_SHIFT);
    (upcast(q), r2, r1, r0, upcast(lsb))
}

/// Update sum to sum * alpha + val.
fn add_to_sum(ref sum: QM31, val: u9, alpha: QM31) {
    sum = sum * alpha + M31Trait::new(upcast(val)).into();
}

/// Ignoring types, this is the same as `add_to_sum(ref sum, msb * shift + lsb, alpha)`.
fn add_to_sum_with_shift(ref sum: QM31, msb: u9, lsb: u32, shift: u9, alpha: QM31) {
    let msb: u32 = upcast(msb);
    let shift = upcast(shift);
    // TODO(ilya): Avoid using reduce_u32.
    sum = sum * alpha + M31Trait::reduce_u32(msb * shift + lsb).into();
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
    let (_, l27, l26, l25, l24_high) = take27(v7, 0x2);

    let mut sum: QM31 = M31Trait::new(upcast(l27)).into();
    add_to_sum(ref sum, l26, alpha);
    add_to_sum(ref sum, l25, alpha);

    // Take 8 + 18 + 6 bits from v6
    let (l24_low, l23, l22, l21_high) = take18(v6, 0x40);
    add_to_sum_with_shift(ref sum, l24_high, l24_low, 0x100, alpha);

    add_to_sum(ref sum, l23, alpha);
    add_to_sum(ref sum, l22, alpha);

    // Take 3 + 27 + 2 bits from v5
    let (l21_low, l20, l19, l18, l17_high) = take27(v5, 0x4);
    add_to_sum_with_shift(ref sum, l21_high, l21_low, 0x8, alpha);

    add_to_sum(ref sum, l20, alpha);
    add_to_sum(ref sum, l19, alpha);
    add_to_sum(ref sum, l18, alpha);

    // Take 7 + 18 + 7 bits from v4
    let (l17_low, l16, l15, l14_high) = take18(v4, 0x80);
    add_to_sum_with_shift(ref sum, l17_high, l17_low, 0x80, alpha);

    add_to_sum(ref sum, l16, alpha);
    add_to_sum(ref sum, l15, alpha);

    // Take 2 + 27 + 3 bits from v3
    let (l14_low, l13, l12, l11, l10_high) = take27(v3, 0x8);
    add_to_sum_with_shift(ref sum, l14_high, l14_low, 0x4, alpha);

    add_to_sum(ref sum, l13, alpha);
    add_to_sum(ref sum, l12, alpha);
    add_to_sum(ref sum, l11, alpha);

    // Take 6 + 18 + 8 bits from v2
    let (l10_low, l9, l8, l17_high) = take18(v2, 0x100);
    add_to_sum_with_shift(ref sum, l10_high, l10_low, 0x40, alpha);

    add_to_sum(ref sum, l9, alpha);
    add_to_sum(ref sum, l8, alpha);

    // Take 1 + 27 + 4 bits from v1
    let (l17_low, l6, l5, l4, l3_high) = take27(v1, 0x10);
    add_to_sum_with_shift(ref sum, l17_high, l17_low, 0x2, alpha);

    add_to_sum(ref sum, l6, alpha);
    add_to_sum(ref sum, l5, alpha);
    add_to_sum(ref sum, l4, alpha);

    // Take 5 + 27 + 0 bits from v0
    let (l3_low, l2, l1, l0, _) = take27(v0, 1);
    add_to_sum_with_shift(ref sum, l3_high, l3_low, 0x20, alpha);

    add_to_sum(ref sum, l2, alpha);
    add_to_sum(ref sum, l1, alpha);
    add_to_sum(ref sum, l0, alpha);

    sum
}
