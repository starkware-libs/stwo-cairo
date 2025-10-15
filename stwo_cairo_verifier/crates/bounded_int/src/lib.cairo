//! Utility crate that exports internal corelib function.
//! This crate is compiled with an older edition that does not enforce visibity rules.

use core::integer::upcast;
#[feature("bounded-int-utils")]
use core::internal::bounded_int::{
    AddHelper, BoundedInt, ConstrainHelper, DivRemHelper, MulHelper, SubHelper, add,
    bounded_int_mul, constrain, div_rem, sub,
};

type ConstValue<const VALUE: felt252> = BoundedInt<VALUE, VALUE>;

const U8_SHIFT: felt252 = 0x100; // 2**8
const U9_SHIFT: felt252 = 0x200; // 2**9
const U32_SHIFT: felt252 = 0x100000000; // 2**32

const NZ_U8_SHIFT: NonZero<ConstValue<U8_SHIFT>> = 0x100;
const NZ_U9_SHIFT: NonZero<ConstValue<U9_SHIFT>> = 0x200;
const NZ_U32_SHIFT: NonZero<ConstValue<U32_SHIFT>> = 0x100000000;

const M31_SHIFT_NZ_U256: NonZero<u256> = 0x80000000; // 2**31

pub mod impls {
    use super::{BoundedInt, ConstValue, DivRemHelper, U32_SHIFT, U8_SHIFT, U9_SHIFT};

    type U8_BOUNDED_INT = BoundedInt<0, { U8_SHIFT - 1 }>;
    type U9_BOUNDED_INT = BoundedInt<0, { U9_SHIFT - 1 }>;
    type U9_PLUS_1_BOUNDED_INT = BoundedInt<0, { U9_SHIFT }>;
    type U16_BOUNDED_INT = BoundedInt<0, { 0x10000 - 1 }>; // 2**16 - 1
    type U23_BOUNDED_INT = BoundedInt<0, { 0x800000 - 1 }>; // 2**23 - 1
    type U24_BOUNDED_INT = BoundedInt<0, { 0x1000000 - 1 }>; // 2**24 - 1
    type U32_BOUNDED_INT = BoundedInt<0, { 0x100000000 - 1 }>; // 2**32 - 1
    type U40_BOUNDED_INT = BoundedInt<0, { 0x10000000000 - 1 }>; // 2**40 - 1
    type U48_BOUNDED_INT = BoundedInt<0, { 0x1000000000000 - 1 }>; // 2**48 - 1
    type U56_BOUNDED_INT = BoundedInt<0, { 0x100000000000000 - 1 }>; // 2**56 - 1
    type U64_BOUNDED_INT = BoundedInt<0, { 0x10000000000000000 - 1 }>; // 2**64 - 1
    type U72_BOUNDED_INT = BoundedInt<0, { 0x1000000000000000000 - 1 }>; // 2**72 - 1
    type U80_BOUNDED_INT = BoundedInt<0, { 0x100000000000000000000 - 1 }>; // 2**80 - 1
    type U88_BOUNDED_INT = BoundedInt<0, { 0x10000000000000000000000 - 1 }>; // 2**88 - 1
    type U96_BOUNDED_INT = BoundedInt<0, { 0x1000000000000000000000000 - 1 }>; // 2**96 - 1
    type U104_BOUNDED_INT = BoundedInt<0, { 0x100000000000000000000000000 - 1 }>; // 2**104 - 1
    type U112_BOUNDED_INT = BoundedInt<0, { 0x10000000000000000000000000000 - 1 }>; // 2**112 - 1
    type U120_BOUNDED_INT = BoundedInt<0, { 0x1000000000000000000000000000000 - 1 }>; // 2**120 - 1
    type U128_BOUNDED_INT =
        BoundedInt<0, { 0x100000000000000000000000000000000 - 1 }>; // 2**128 - 1


    pub impl DivRemU128ByU8Shift of DivRemHelper<u128, ConstValue<U8_SHIFT>> {
        type DivT = U120_BOUNDED_INT;
        type RemT = U8_BOUNDED_INT;
    }

    pub impl DivRemU128ByU32Shift of DivRemHelper<u128, ConstValue<U32_SHIFT>> {
        type DivT = U96_BOUNDED_INT;
        type RemT = U32_BOUNDED_INT;
    }

    pub impl DivRemBoundedU96ByU32Shift of DivRemHelper<U96_BOUNDED_INT, ConstValue<U32_SHIFT>> {
        type DivT = U64_BOUNDED_INT;
        type RemT = U32_BOUNDED_INT;
    }

    pub impl DivRemBoundedU64ByU32Shift of DivRemHelper<U64_BOUNDED_INT, ConstValue<U32_SHIFT>> {
        type DivT = U32_BOUNDED_INT;
        type RemT = U32_BOUNDED_INT;
    }

    pub impl DivRemU64ByU32Shift of DivRemHelper<u64, ConstValue<U32_SHIFT>> {
        type DivT = BoundedInt<0, { U32_SHIFT - 1 }>;
        type RemT = BoundedInt<0, { U32_SHIFT - 1 }>;
    }


    pub impl DivRemU32ByU8Shift of DivRemHelper<u32, ConstValue<U8_SHIFT>> {
        type DivT = U24_BOUNDED_INT;
        type RemT = U8_BOUNDED_INT;
    }

    // All DivRemBoundedU*ByU8Shift are pub implemented here (up to U120).
    pub impl DivRemBoundedU16ByU8Shift of DivRemHelper<U16_BOUNDED_INT, ConstValue<U8_SHIFT>> {
        type DivT = U8_BOUNDED_INT;
        type RemT = U8_BOUNDED_INT;
    }

    pub impl DivRemBoundedU24ByU8Shift of DivRemHelper<U24_BOUNDED_INT, ConstValue<U8_SHIFT>> {
        type DivT = U16_BOUNDED_INT;
        type RemT = U8_BOUNDED_INT;
    }

    pub impl DivRemBoundedU32ByU8Shift of DivRemHelper<U32_BOUNDED_INT, ConstValue<U8_SHIFT>> {
        type DivT = U24_BOUNDED_INT;
        type RemT = U8_BOUNDED_INT;
    }

    pub impl DivRemBoundedU40ByU8Shift of DivRemHelper<U40_BOUNDED_INT, ConstValue<U8_SHIFT>> {
        type DivT = U32_BOUNDED_INT;
        type RemT = U8_BOUNDED_INT;
    }

    pub impl DivRemBoundedU48ByU8Shift of DivRemHelper<U48_BOUNDED_INT, ConstValue<U8_SHIFT>> {
        type DivT = U40_BOUNDED_INT;
        type RemT = U8_BOUNDED_INT;
    }

    pub impl DivRemBoundedU56ByU8Shift of DivRemHelper<U56_BOUNDED_INT, ConstValue<U8_SHIFT>> {
        type DivT = U48_BOUNDED_INT;
        type RemT = U8_BOUNDED_INT;
    }

    pub impl DivRemBoundedU64ByU8Shift of DivRemHelper<U64_BOUNDED_INT, ConstValue<U8_SHIFT>> {
        type DivT = U56_BOUNDED_INT;
        type RemT = U8_BOUNDED_INT;
    }

    pub impl DivRemBoundedU72ByU8Shift of DivRemHelper<U72_BOUNDED_INT, ConstValue<U8_SHIFT>> {
        type DivT = U64_BOUNDED_INT;
        type RemT = U8_BOUNDED_INT;
    }

    pub impl DivRemBoundedU80ByU8Shift of DivRemHelper<U80_BOUNDED_INT, ConstValue<U8_SHIFT>> {
        type DivT = U72_BOUNDED_INT;
        type RemT = U8_BOUNDED_INT;
    }

    pub impl DivRemBoundedU88ByU8Shift of DivRemHelper<U88_BOUNDED_INT, ConstValue<U8_SHIFT>> {
        type DivT = U80_BOUNDED_INT;
        type RemT = U8_BOUNDED_INT;
    }

    pub impl DivRemBoundedU96ByU8Shift of DivRemHelper<U96_BOUNDED_INT, ConstValue<U8_SHIFT>> {
        type DivT = U88_BOUNDED_INT;
        type RemT = U8_BOUNDED_INT;
    }

    pub impl DivRemBoundedU104ByU8Shift of DivRemHelper<U104_BOUNDED_INT, ConstValue<U8_SHIFT>> {
        type DivT = U96_BOUNDED_INT;
        type RemT = U8_BOUNDED_INT;
    }

    pub impl DivRemBoundedU112ByU8Shift of DivRemHelper<U112_BOUNDED_INT, ConstValue<U8_SHIFT>> {
        type DivT = U104_BOUNDED_INT;
        type RemT = U8_BOUNDED_INT;
    }

    pub impl DivRemBoundedU120ByU8Shift of DivRemHelper<U120_BOUNDED_INT, ConstValue<U8_SHIFT>> {
        type DivT = U112_BOUNDED_INT;
        type RemT = U8_BOUNDED_INT;
    }


    pub impl DivRemU32ByU9Shift of DivRemHelper<u32, ConstValue<U9_SHIFT>> {
        type DivT = U23_BOUNDED_INT;
        type RemT = BoundedInt<0, { U9_SHIFT - 1 }>;
    }

    pub impl DivRemU32ByU9Plus1 of DivRemHelper<u32, U9_PLUS_1_BOUNDED_INT> {
        type DivT = U32_BOUNDED_INT;
        type RemT = U9_BOUNDED_INT;
    }
}
