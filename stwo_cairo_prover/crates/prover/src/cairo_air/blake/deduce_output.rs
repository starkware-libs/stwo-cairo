use std::array;
use std::simd::u32x16;

use stwo_cairo_adapter::memory::Memory;
use stwo_cairo_common::preprocessed_consts::blake::N_BLAKE_SIGMA_COLS;
use stwo_cairo_common::prover_types::simd::{PackedUInt32, N_LANES};
use stwo_prover::core::backend::simd::m31::PackedM31;

use super::const_columns::sigma;

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
///
/// [`u32::rotate_right(N)`]: u32::rotate_right
#[inline(always)]
fn rotate<const N: u32>(x: u32x16) -> u32x16 {
    (x >> N) | (x << (u32::BITS - N))
}

#[derive(Debug)]
pub struct BlakeG {}

// TODO(Stav): remove '#[allow(unused)]' when possible.
#[allow(unused)]
impl BlakeG {
    pub fn deduce_output(
        input: [PackedUInt32; NUM_INPUT_WORDS_G],
    ) -> [PackedUInt32; NUM_OUTPUT_WORDS_G] {
        BlakeG::blake_g(input.map(|x| x.simd)).map(|simd| PackedUInt32 { simd })
    }

    fn blake_g(input: [u32x16; 6]) -> [u32x16; 4] {
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
pub struct TripleXor32 {}

// TODO(Stav): remove '#[allow(unused)]' when possible.
#[allow(unused)]
impl TripleXor32 {
    pub fn deduce_output([input0, input1, input2]: [PackedUInt32; 3]) -> PackedUInt32 {
        input0 ^ input1 ^ input2
    }
}

/// Transposes 16 chunks of 16 `u32`s each, to get 16 `u32x16`, each
/// representing 16 packed instances of a word (message or sigma).
fn transpose(mut data: [u32x16; 16]) -> [u32x16; 16] {
    // Index abcd:xyzw, refers to a specific word in data as follows:
    //   abcd - chunk index (in base 2)
    //   xyzw - word offset (in base 2)
    // Transpose by applying 4 times the index permutation:
    //   abcd:xyzw => wabc:dxyz
    // In other words, rotate the index to the right by 1.
    for _ in 0..4 {
        let (d0, d8) = data[0].deinterleave(data[1]);
        let (d1, d9) = data[2].deinterleave(data[3]);
        let (d2, d10) = data[4].deinterleave(data[5]);
        let (d3, d11) = data[6].deinterleave(data[7]);
        let (d4, d12) = data[8].deinterleave(data[9]);
        let (d5, d13) = data[10].deinterleave(data[11]);
        let (d6, d14) = data[12].deinterleave(data[13]);
        let (d7, d15) = data[14].deinterleave(data[15]);
        data = [
            d0, d1, d2, d3, d4, d5, d6, d7, d8, d9, d10, d11, d12, d13, d14, d15,
        ];
    }

    data
}

#[derive(Debug)]
pub struct BlakeRoundSigma {}

// TODO(Stav): remove '#[allow(unused)]' when possible.
#[allow(unused)]
impl BlakeRoundSigma {
    pub fn deduce_output(round: PackedM31) -> [PackedM31; N_BLAKE_SIGMA_COLS] {
        Self::packed_sigma(round.into_simd()).map(|v| unsafe { PackedM31::from_simd_unchecked(v) })
    }

    pub fn packed_sigma(round: u32x16) -> [u32x16; N_BLAKE_SIGMA_COLS] {
        let round_sigmas = round.to_array().map(|x| u32x16::from(sigma(x as usize)));
        transpose(round_sigmas)
    }
}

pub struct BlakeRound<'a> {
    memory: &'a Memory,
}

// TODO(Stav): remove '#[allow(unused)]' when possible.
#[allow(unused)]
impl BlakeRound<'_> {
    fn deduce_output(
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
        let sigma = BlakeRoundSigma::packed_sigma(round);

        let message: [_; N_LANES] = array::from_fn(|i| {
            u32x16::from(array::from_fn(|j| {
                self.memory.get(message_pointer[i] + sigma[i][j]).as_small() as u32
            }))
        });

        let mut state = state;
        for (row_index, &[i0, i1, i2, i3]) in G_STATE_INDICES.iter().enumerate() {
            [state[i0], state[i1], state[i2], state[i3]] = BlakeG::blake_g([
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

#[cfg(test)]
mod tests {
    use stwo_cairo_adapter::memory::{MemoryBuilder, MemoryConfig};
    use stwo_cairo_adapter::vm_import::MemoryEntry;
    use stwo_cairo_common::prover_types::cpu::UInt32;
    use stwo_prover::core::backend::simd::conversion::Unpack;
    use stwo_prover::core::fields::m31::M31;

    use super::*;

    #[test]
    fn test_g() {
        let input = [
            305419896, 4294967295, 2147483647, 123456789, 987654321, 468798,
        ]
        .map(UInt32::from)
        .map(PackedUInt32::broadcast);
        let expected = [2827666065, 4146123195, 3407348176, 3638212488]
            .map(UInt32::from)
            .map(PackedUInt32::broadcast);
        let actual = BlakeG::deduce_output(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_round_sigma() {
        let input = PackedM31::broadcast(M31(7));
        let expected = [13, 11, 7, 14, 12, 1, 3, 9, 5, 0, 15, 4, 8, 6, 2, 10]
            .map(M31::from)
            .map(PackedM31::broadcast);
        let actual = BlakeRoundSigma::deduce_output(input);
        assert_eq!(
            actual.map(PackedM31::to_array),
            expected.map(PackedM31::to_array)
        );
    }

    #[test]
    fn test_blake_round() {
        // Create input
        let state = [
            1589929985, 669959787, 3341104026, 828450965, 1955226293, 542713244, 3587648250,
            2032424797, 3147641385, 3967920621, 2006879305, 2745232376, 2456599919, 130066657,
            1468412498, 325435090,
        ]
        .map(u32x16::splat);
        let message_pointer: u32 = 7687346;

        let messgae = [
            1190313840, 586871615, 3326317950, 2157490798, 2171729911, 4006315130, 3006051123,
            3934250148, 745259603, 1963379556, 3874654107, 2051567115, 2102274589, 1991875188,
            1621381226, 1307057221,
        ]
        .map(u32x16::splat);

        // Create Memory
        let memory_entries: Vec<MemoryEntry> = (0..=15)
            .map(|i| MemoryEntry {
                address: message_pointer as u64 + i as u64,
                value: [messgae[i as usize][0], 0, 0, 0, 0, 0, 0, 0],
            })
            .collect();
        let memory =
            MemoryBuilder::from_iter(MemoryConfig::default(), memory_entries.iter().cloned());

        // Expected output
        let expected_state = [
            1367297060, 2926508617, 107924025, 162546146, 2936957071, 4075222909, 2433518464,
            3059149654, 2047295278, 3825369540, 4231837284, 4024902448, 1952998180, 1326427959,
            3193012524, 1657286418,
        ];
        let expected = (
            M31::from(0),
            M31::from(6),
            (expected_state.map(UInt32::from), M31::from(message_pointer)),
        );

        let blake_round = BlakeRound { memory: &memory };
        let actual = blake_round
            .deduce_output(
                PackedM31::broadcast(M31::from(0)),
                PackedM31::broadcast(M31::from(5)),
                (
                    state.map(|v| PackedUInt32 { simd: v }),
                    PackedM31::broadcast(M31(message_pointer)),
                ),
            )
            .unpack()[0];
        assert_eq!(actual, expected);
    }
}
