use bounded_int::{
    AddHelper, BoundedInt, DivRemHelper, MulHelper, add, bounded_int_mul, div_rem, upcast,
};

type ConstValue<const VALUE: felt252> = BoundedInt<VALUE, VALUE>;
const U9_SHIFT: felt252 = 0x200; // 2**9
use stwo_verifier_core::fields::m31::{M31InnerT, M31Trait};
use stwo_verifier_core::fields::qm31::QM31;


const NZ_U9_SHIFT: NonZero<ConstValue<U9_SHIFT>> = 0x200;

type u23 = BoundedInt<0, { 0x800000 - 1 }>;

type u9 = BoundedInt<0, { U9_SHIFT - 1 }>;

pub impl DivRemU32ByU9Shift of DivRemHelper<u32, ConstValue<U9_SHIFT>> {
    type DivT = u23;
    type RemT = BoundedInt<0, { U9_SHIFT - 1 }>;
}


type U32_BOUNDED = BoundedInt<0, 0xffffffff>;


pub impl DivRemU32ByU9 of DivRemHelper<U32_BOUNDED, u9> {
    type DivT = U32_BOUNDED;
    type RemT = BoundedInt<0, { U9_SHIFT - 2 }>;
}

// (U9_SHIFT - 1)**2 = 0x3fc01.
type U9_BB_U9_BOUNDED = BoundedInt<0, 0x3fc01>;

pub impl MulU9ByU9 of MulHelper<u9, u9> {
    type Result = U9_BB_U9_BOUNDED;
}


pub impl ADD_U9_BB_U9_BOUNDED_TO_U23 of AddHelper<U9_BB_U9_BOUNDED, u23> {
    type Result = BoundedInt<0, 0x83fc00>;
}

/// Splits input into (msb, 2*u9, lsb) where lsb has log_2(shift) bits.
fn take18(input: u32, shift: NonZero<u9>) -> (u23, u9, u9, u9) {
    let (q, lsb) = div_rem::<U32_BOUNDED, _, _>(upcast(input), shift);
    let (q, r0) = div_rem::<u32, _, _>(upcast(q), NZ_U9_SHIFT);
    let (q, r1) = div_rem::<u32, _, _>(upcast(q), NZ_U9_SHIFT);
    (upcast(q), r1, r0, upcast(lsb))
}

/// Splits input into (msb, 3*u9, lsb) where lsb has log_2(shift) bits.
fn take27(input: u32, shift: NonZero<u9>) -> (u23, u9, u9, u9, u9) {
    let (q, lsb) = div_rem::<U32_BOUNDED, _, _>(upcast(input), shift);
    let (q, r0) = div_rem::<u32, _, _>(upcast(q), NZ_U9_SHIFT);
    let (q, r1) = div_rem::<u32, _, _>(upcast(q), NZ_U9_SHIFT);
    let (q, r2) = div_rem::<u32, _, _>(upcast(q), NZ_U9_SHIFT);
    (upcast(q), r2, r1, r0, upcast(lsb))
}

/// Update sum to sum * alpha + val.
fn add_to_sum(ref sum: QM31, val: u9, alpha: QM31) {
    sum = sum * alpha + M31Trait::new(upcast(val)).into();
}

/// Ignore types, this is the same as add_to_sum(ref sum, msb * shift + lsb, alpha).
fn add_to_sum_with_shift(ref sum: QM31, msb: u9, lsb: u23, shift: u9, alpha: QM31) {
    sum = sum * alpha
        + M31Trait::new(upcast::<_, M31InnerT>(add(bounded_int_mul(msb, shift), lsb))).into();
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
    add_to_sum_with_shift(ref sum, l7, m6, 0x100, alpha);

    add_to_sum(ref sum, r1, alpha);
    add_to_sum(ref sum, r0, alpha);

    // Take 3 + 27 + 2 bits from v5
    let (m5, r2, r1, r0, l5) = take27(v5, 0x4);
    add_to_sum_with_shift(ref sum, l6, m5, 0x8, alpha);

    add_to_sum(ref sum, r2, alpha);
    add_to_sum(ref sum, r1, alpha);
    add_to_sum(ref sum, r0, alpha);

    // Take 7 + 18 + 7 bits from v4
    let (m4, r1, r0, l4) = take18(v4, 0x80);
    add_to_sum_with_shift(ref sum, l5, m4, 0x80, alpha);

    add_to_sum(ref sum, r1, alpha);
    add_to_sum(ref sum, r0, alpha);

    // Take 2 + 27 + 3 bits from v3
    let (m3, r2, r1, r0, l3) = take27(v3, 0x8);
    add_to_sum_with_shift(ref sum, l4, m3, 0x4, alpha);

    add_to_sum(ref sum, r2, alpha);
    add_to_sum(ref sum, r1, alpha);
    add_to_sum(ref sum, r0, alpha);

    // Take 6 + 18 + 8 bits from v2
    let (m2, r1, r0, l2) = take18(v2, 0x100);
    add_to_sum_with_shift(ref sum, l3, m2, 0x40, alpha);

    add_to_sum(ref sum, r1, alpha);
    add_to_sum(ref sum, r0, alpha);

    // Take 1 + 27 + 4 bits from v1
    let (m1, r2, r1, r0, l1) = take27(v1, 0x10);
    add_to_sum_with_shift(ref sum, l2, m1, 0x2, alpha);

    add_to_sum(ref sum, r2, alpha);
    add_to_sum(ref sum, r1, alpha);
    add_to_sum(ref sum, r0, alpha);

    // Take 5 + 27 + 0 bits from v0
    let (m0, r2, r1, r0, _) = take27(v0, 1);
    add_to_sum_with_shift(ref sum, l1, m0, 0x20, alpha);

    add_to_sum(ref sum, r2, alpha);
    add_to_sum(ref sum, r1, alpha);
    add_to_sum(ref sum, r0, alpha);

    sum
}
