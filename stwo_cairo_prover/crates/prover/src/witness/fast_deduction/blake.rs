use std::array::from_fn;
use std::simd::u32x16;

use stwo_cairo_adapter::memory::Memory;
use stwo_cairo_common::preprocessed_columns::blake::{BLAKE_SIGMA, N_BLAKE_SIGMA_COLS};
use stwo_cairo_common::prover_types::simd::{PackedUInt32, N_LANES};

use crate::stwo::prover::backend::simd::m31::PackedM31;

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
                self.memory.get(message_pointer[j] + sigma[i][j]).as_small() as u32
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

#[cfg(test)]
mod tests {
    use stwo::core::fields::m31::M31;
    use stwo_cairo_adapter::memory::{MemoryBuilder, MemoryConfig, MemoryEntry};

    use super::*;

    #[test]
    fn test_g() {
        let input0 = [
            305419896_u32,
            4294967295,
            2147483647,
            123456789,
            987654321,
            468798,
        ];
        let input1 = [
            3694142613, 170668591, 2859583592, 2750542364, 101488500, 3940201164,
        ];
        let input = from_fn(|i| {
            PackedUInt32::from_simd(u32x16::from_slice(&[input0[i], input1[i]].repeat(8)))
        });

        let exp0 = [2827666065, 4146123195, 3407348176, 3638212488];
        let exp1 = [2049993894, 223224271, 100412452, 1063654435];
        let expected =
            from_fn(|i| PackedUInt32::from_simd(u32x16::from_slice(&[exp0[i], exp1[i]].repeat(8))));

        let actual = PackedBlakeG::deduce_output(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_round_sigma() {
        let input =
            unsafe { PackedM31::from_simd_unchecked(u32x16::from_slice(&[7, 2].repeat(8))) };

        let exp0 = [13, 11, 7, 14, 12, 1, 3, 9, 5, 0, 15, 4, 8, 6, 2, 10];
        let exp1 = [11, 8, 12, 0, 5, 2, 15, 13, 10, 14, 3, 6, 7, 1, 9, 4];

        let expected: [PackedM31; 16] = unsafe {
            from_fn(|i| {
                PackedM31::from_simd_unchecked(u32x16::from_slice(&[exp0[i], exp1[i]].repeat(8)))
            })
        };

        let actual = PackedBlakeRoundSigma::deduce_output(input);

        for (expected, actual) in expected.iter().zip(actual.iter()) {
            assert_eq!(expected.to_array(), actual.to_array());
        }
    }

    #[test]
    fn test_blake_round() {
        let state0 = [
            1589929985, 669959787, 3341104026, 828450965, 1955226293, 542713244, 3587648250,
            2032424797, 3147641385, 3967920621, 2006879305, 2745232376, 2456599919, 130066657,
            1468412498, 325435090,
        ];
        let state1 = [
            2981648577, 2100013035, 663841651, 2464560971, 3804981465, 2521887078, 1263129662,
            3279679818, 1291748021, 2308065230, 3957504572, 113619231, 622788508, 1137821987,
            2149537027, 2989138246,
        ];
        let state: [PackedUInt32; 16] = from_fn(|i| {
            PackedUInt32::from_simd(u32x16::from_slice(&[state0[i], state1[i]].repeat(8)))
        });

        let message_pointer = unsafe {
            PackedM31::from_simd_unchecked(u32x16::from_slice(&[7687346, 8876].repeat(8)))
        };

        let message0 = [
            1190313840, 586871615, 3326317950, 2157490798, 2171729911, 4006315130, 3006051123,
            3934250148, 745259603, 1963379556, 3874654107, 2051567115, 2102274589, 1991875188,
            1621381226, 1307057221,
        ];
        let message1 = [
            1883221824, 4159262814, 3806732234, 552650188, 2549022015, 3000021069, 2298537828,
            915357142, 1657285681, 1835346724, 4150146227, 3993296861, 2937251920, 1002511359,
            2142515262, 4138014718,
        ];

        let rounds =
            unsafe { PackedM31::from_simd_unchecked(u32x16::from_slice(&[5, 9].repeat(8))) };
        let chains =
            unsafe { PackedM31::from_simd_unchecked(u32x16::from_slice(&[0, 100].repeat(8))) };

        // Create Memory
        let memory_entries: Vec<MemoryEntry> = (0..=15)
            .flat_map(|i| {
                [
                    MemoryEntry {
                        address: (message_pointer.to_array()[0].0 + i) as u64,
                        value: [message0[i as usize], 0, 0, 0, 0, 0, 0, 0],
                    },
                    MemoryEntry {
                        address: (message_pointer.to_array()[1].0 + i) as u64,
                        value: [message1[i as usize], 0, 0, 0, 0, 0, 0, 0],
                    },
                ]
            })
            .collect();
        let (memory, ..) =
            MemoryBuilder::from_iter(MemoryConfig::default(), memory_entries.iter().cloned())
                .build();

        // Expected output
        let exp0 = [
            1367297060_u32,
            2926508617,
            107924025,
            162546146,
            2936957071,
            4075222909,
            2433518464,
            3059149654,
            2047295278,
            3825369540,
            4231837284,
            4024902448,
            1952998180,
            1326427959,
            3193012524,
            1657286418,
        ];
        let exp1 = [
            538881188, 2460695046, 2867837425, 2135058897, 890208416, 953468451, 1371496227,
            2159536600, 3054417061, 1384474009, 3996057645, 3268429468, 2110965500, 3522211544,
            4011799291, 2106633509,
        ];
        let expected: [PackedUInt32; 16] =
            from_fn(|i| PackedUInt32::from_simd(u32x16::from_slice(&[exp0[i], exp1[i]].repeat(8))));

        let blake_round = BlakeRound { memory };
        let actual = blake_round.deduce_output(chains, rounds, (state, message_pointer));

        let (actual_chain, actual_round, (actual_state, actual_message_pointer)) = actual;
        assert_eq!(actual_chain.to_array(), chains.to_array());
        assert_eq!(
            actual_round.to_array(),
            rounds.to_array().map(|round| M31::from(round.0 + 1))
        );
        assert_eq!(actual_state, expected);
        assert_eq!(
            actual_message_pointer.to_array(),
            message_pointer.to_array()
        );
    }
}
