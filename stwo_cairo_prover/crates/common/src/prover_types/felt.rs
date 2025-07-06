use std::ops::{BitAnd, BitOrAssign, Shl, ShrAssign};
use std::simd::u32x16;

use stwo::core::fields::m31::M31;
use stwo::prover::backend::simd::m31::{PackedM31, N_LANES};

use crate::memory::{N_BITS_PER_FELT, N_M31_IN_FELT252};

/// Splits a 32N bit dense representation into felts, each with N_BITS_PER_FELT bits.
///
/// Parameters:
/// - `N`: the number of 32-bit words in the input.
/// - `M`: the number of felts in the output.
/// - `TU32`: the type of the input and output words.
/// - `x`: the input dense representation.
/// - `mask`: (1 << N_BITS_PER_FELT) - 1.
fn split<const N: usize, const M: usize, TU32>(x: [TU32; N], mask: TU32) -> [TU32; M]
where
    TU32: BitAnd<Output = TU32>
        + BitOrAssign
        + Copy
        + ShrAssign<u32>
        + Shl<u32, Output = TU32>
        + Default,
{
    let mut res = [TU32::default(); M];
    let mut n_bits_in_word = 32;
    let mut word_i = 0;
    let mut word = x[word_i];
    for e in res.iter_mut() {
        // If current word has more bits than needed, chop it.
        if n_bits_in_word > N_BITS_PER_FELT {
            *e = word & mask;
            word >>= N_BITS_PER_FELT as u32;
            n_bits_in_word -= N_BITS_PER_FELT;
            continue;
        }

        *e = word;
        // Fetch next word.
        word_i += 1;
        word = x.get(word_i).copied().unwrap_or_default();

        // If we need more bits to fill, take from next word.
        if n_bits_in_word < N_BITS_PER_FELT {
            *e |= (word << n_bits_in_word as u32) & mask;
            word >>= (N_BITS_PER_FELT - n_bits_in_word) as u32;
        }

        n_bits_in_word += 32 - N_BITS_PER_FELT;
    }
    res
}

/// Splits a 252 bit dense representation into felts, each with N_BITS_PER_FELT bits.
pub fn split_f252_simd(x: [u32x16; 8]) -> [PackedM31; N_M31_IN_FELT252] {
    split(x, u32x16::from_array([(1 << N_BITS_PER_FELT) - 1; N_LANES]))
        .map(|x| PackedM31::from(x.to_array().map(M31::from_u32_unchecked)))
}

/// Splits a 252 bit dense representation into felts, each with N_BITS_PER_FELT bits.
pub fn split_f252(x: [u32; 8]) -> [M31; N_M31_IN_FELT252] {
    split(x, (1 << N_BITS_PER_FELT) - 1).map(M31::from_u32_unchecked)
}
