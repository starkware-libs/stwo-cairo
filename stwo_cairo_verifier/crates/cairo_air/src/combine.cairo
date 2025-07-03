use bounded_int::{BoundedInt, DivRemHelper, div_rem, upcast};

type ConstValue<const VALUE: felt252> = BoundedInt<VALUE, VALUE>;
const U9_SHIFT: felt252 = 0x200; // 2**9
use stwo_verifier_core::fields::m31::M31Trait;
use stwo_verifier_core::fields::qm31::QM31;


const NZ_U9_SHIFT: NonZero<ConstValue<U9_SHIFT>> = 0x200;

type U23_BOUNDED_INT = BoundedInt<0, { 0x800000 - 1 }>;

type U9_BOUNDED_INT = BoundedInt<0, { U9_SHIFT - 1 }>;

type u9 = BoundedInt<0, { U9_SHIFT - 1 }>;

pub impl DivRemU32ByU9Shift of DivRemHelper<u32, ConstValue<U9_SHIFT>> {
    type DivT = U23_BOUNDED_INT;
    type RemT = BoundedInt<0, { U9_SHIFT - 1 }>;
}

/// Splits input into (msb, 2*u9, lsb) where lsb has log_2(shift) bits.
fn take18(input: u32, shift: NonZero<u32>) -> (u32, u9, u9, u32) {
    let (q, lsb) = DivRem::div_rem(input, shift);
    let (q, r0) = div_rem::<u32, _, _>(upcast(q), NZ_U9_SHIFT);
    let (q, r1) = div_rem::<u32, _, _>(upcast(q), NZ_U9_SHIFT);
    (upcast(q), r1, r0, lsb)
}

/// Splits input into (msb, 3*u9, lsb) where lsb has log_2(shift) bits.
fn take27(input: u32, shift: NonZero<u32>) -> (u32, u9, u9, u9, u32) {
    let (q, lsb) = DivRem::div_rem(input, shift);
    let (q, r0) = div_rem::<u32, _, _>(upcast(q), NZ_U9_SHIFT);
    let (q, r1) = div_rem::<u32, _, _>(upcast(q), NZ_U9_SHIFT);
    let (q, r2) = div_rem::<u32, _, _>(upcast(q), NZ_U9_SHIFT);
    (upcast(q), r2, r1, r0, lsb)
}

fn add_to_sum(ref sum: QM31, val: u9, alpha: QM31) {
    sum = sum * alpha + M31Trait::new(upcast(val)).into();
}

/// This is a handwritten implementation for combining a felt252 as part of combine_id_to_value.
/// It does + split_f252 + horner evaluation at alpha. Since horner evaluation is done in reverse,
/// the split here is a bit contrived.
pub fn combine_felt252(value: [u32; 8], alpha: QM31) -> QM31 {
    let [v0, v1, v2, v3, v4, v5, v6, v7] = value;

    // Since the value is felt252, we ignore the 4 most significant bits.
    // Take 4 + 27 + 1 bits from v7
    let (_, r2, r1, r0, l7) = take27(v7, 0x2);

    let mut sum: QM31 = M31Trait::new(upcast(r2)).into();
    add_to_sum(ref sum, r1, alpha);
    add_to_sum(ref sum, r0, alpha);

    // Take 8 + 18 + 6 bits from v6
    let (m6, r1, r0, l6) = take18(v6, 0x40);
    sum = sum * alpha + M31Trait::reduce_u32(l7 * 0x100 + m6).into();

    add_to_sum(ref sum, r1, alpha);
    add_to_sum(ref sum, r0, alpha);

    // Take 3 + 27 + 2 bits from v5
    let (m5, r2, r1, r0, l5) = take27(v5, 0x4);
    sum = sum * alpha + M31Trait::reduce_u32(l6 * 0x8 + m5).into();
    add_to_sum(ref sum, r2, alpha);
    add_to_sum(ref sum, r1, alpha);
    add_to_sum(ref sum, r0, alpha);

    // Take 7 + 18 + 7 bits from v4
    let (m4, r1, r0, l4) = take18(v4, 0x80);
    sum = sum * alpha + M31Trait::reduce_u32(l5 * 0x80 + m4).into();

    add_to_sum(ref sum, r1, alpha);
    add_to_sum(ref sum, r0, alpha);

    // Take 2 + 27 + 3 bits from v3
    let (m3, r2, r1, r0, l3) = take27(v3, 0x8);
    sum = sum * alpha + M31Trait::reduce_u32(l4 * 0x4 + m3).into();

    add_to_sum(ref sum, r2, alpha);
    add_to_sum(ref sum, r1, alpha);
    add_to_sum(ref sum, r0, alpha);

    // Take 6 + 18 + 8 bits from v2
    let (m2, r1, r0, l2) = take18(v2, 0x100);
    sum = sum * alpha + M31Trait::reduce_u32(l3 * 0x40 + m2).into();

    add_to_sum(ref sum, r1, alpha);
    add_to_sum(ref sum, r0, alpha);

    // Take 1 + 27 + 4 bits from v1
    let (m1, r2, r1, r0, l1) = take27(v1, 0x10);

    sum = sum * alpha + M31Trait::reduce_u32(l2 * 0x2 + m1).into();

    add_to_sum(ref sum, r2, alpha);
    add_to_sum(ref sum, r1, alpha);
    add_to_sum(ref sum, r0, alpha);

    // Take 5 + 27 + 0 bits from v0
    let (m0, r2, r1, r0, _) = take27(v0, 1);

    sum = sum * alpha + M31Trait::reduce_u32(l1 * 0x20 + m0).into();

    add_to_sum(ref sum, r2, alpha);
    add_to_sum(ref sum, r1, alpha);
    add_to_sum(ref sum, r0, alpha);

    sum
}
