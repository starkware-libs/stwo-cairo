use std::array::from_fn;
use std::simd::u32x16;

use stwo_cairo_adapter::memory::Memory;
use stwo_cairo_common::preprocessed_consts::blake::{BLAKE_SIGMA, N_BLAKE_SIGMA_COLS};
use stwo_cairo_common::prover_types::cpu::Relocatable;
use stwo_cairo_common::prover_types::simd::{PackedUInt32, N_LANES};

use crate::stwo_prover::core::backend::simd::m31::PackedM31;

const NUM_INPUT_WORDS_G: usize = 6;
const NUM_OUTPUT_WORDS_G: usize = 4;
pub const G_STATE_INDICES: [[usize; 4]; 8] = [
    [0, 4, 8, 12],
    [1, 5, 9, 13],
    [2, 6, 10, 14],
    [3, 7, 11, 15],
    [0, 5, 10, 15],
    [1, 6, 11, 12],
    [2, 7, 8, 13],
    [3, 4, 9, 14],
];

/// Applies [`u32::rotate_right(N)`] to each element of the vector
#[inline(always)]
fn rotate<const N: u32>(x: u32x16) -> u32x16 {
    (x >> N) | (x << (u32::BITS - N))
}

#[derive(Debug)]
pub struct PackedBlakeG {}

impl PackedBlakeG {
    pub fn deduce_output(
        input: [PackedUInt32; NUM_INPUT_WORDS_G],
    ) -> [PackedUInt32; NUM_OUTPUT_WORDS_G] {
        PackedBlakeG::blake_g(input.map(|x| x.simd)).map(|simd| PackedUInt32 { simd })
    }

    fn blake_g(input: [u32x16; NUM_INPUT_WORDS_G]) -> [u32x16; NUM_OUTPUT_WORDS_G] {
        let [mut a, mut b, mut c, mut d, m0, m1] = input;

        a = a + b + m0;
        d ^= a;
        d = rotate::<16>(d);

        c += d;
        b ^= c;
        b = rotate::<12>(b);

        a = a + b + m1;
        d ^= a;
        d = rotate::<8>(d);

        c += d;
        b ^= c;
        b = rotate::<7>(b);

        [a, b, c, d]
    }
}

#[derive(Debug)]
pub struct PackedTripleXor32 {}

impl PackedTripleXor32 {
    pub fn deduce_output([a, b, c]: [PackedUInt32; 3]) -> PackedUInt32 {
        a ^ b ^ c
    }
}

#[derive(Debug)]
pub struct PackedBlakeRoundSigma {}

impl PackedBlakeRoundSigma {
    pub fn deduce_output(round: PackedM31) -> [PackedM31; N_BLAKE_SIGMA_COLS] {
        Self::packed_sigma(round.into_simd()).map(|v| unsafe { PackedM31::from_simd_unchecked(v) })
    }

    pub fn packed_sigma(round: u32x16) -> [u32x16; N_BLAKE_SIGMA_COLS] {
        from_fn(|i| u32x16::from(round.to_array().map(|x| BLAKE_SIGMA[x as usize][i])))
    }
}

pub struct BlakeRound {
    memory: Memory,
}

impl BlakeRound {
    pub fn new(memory: Memory) -> Self {
        Self { memory }
    }
    pub fn deduce_output(
        &self,
        chain: PackedM31,
        round: PackedM31,
        (state, message_pointer): ([PackedUInt32; 16], PackedM31),
    ) -> (PackedM31, PackedM31, ([PackedUInt32; 16], PackedM31)) {
        let (chain, round, (state, message_pointer)) = self.blake_round(
            chain.into_simd(),
            round.into_simd(),
            (state.map(|x| x.simd), message_pointer.into_simd()),
        );

        unsafe {
            (
                PackedM31::from_simd_unchecked(chain),
                PackedM31::from_simd_unchecked(round),
                (
                    state.map(|simd| PackedUInt32 { simd }),
                    PackedM31::from_simd_unchecked(message_pointer),
                ),
            )
        }
    }
    fn blake_round(
        &self,
        chain: u32x16,
        round: u32x16,
        (state, message_pointer): ([u32x16; 16], u32x16),
    ) -> (u32x16, u32x16, ([u32x16; 16], u32x16)) {
        let sigma = PackedBlakeRoundSigma::packed_sigma(round);

        let message: [_; N_LANES] = from_fn(|i| {
            u32x16::from(from_fn(|j| {
                self.memory
                    .get(Relocatable {
                        segment_index: 1,
                        offset: message_pointer[j] + sigma[i][j],
                    })
                    .as_small() as u32
            }))
        });

        let mut state = state;
        for (row_index, &[i0, i1, i2, i3]) in G_STATE_INDICES.iter().enumerate() {
            [state[i0], state[i1], state[i2], state[i3]] = PackedBlakeG::blake_g([
                state[i0],
                state[i1],
                state[i2],
                state[i3],
                message[row_index * 2],
                message[row_index * 2 + 1],
            ]);
        }

        (chain, round + u32x16::splat(1), (state, message_pointer))
    }
}
