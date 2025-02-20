use stwo_cairo_adapter::memory::Memory;
use stwo_cairo_common::preprocessed_consts::blake::N_BLAKE_SIGMA_COLS;
use stwo_cairo_common::prover_types::cpu::*;

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

#[derive(Debug)]
pub struct BlakeG {}

// TODO(Stav): remove '#[allow(unused)]' when possible.
#[allow(unused)]
impl BlakeG {
    pub fn deduce_output(input: [UInt32; NUM_INPUT_WORDS_G]) -> [UInt32; NUM_OUTPUT_WORDS_G] {
        BlakeG::blake_g(input.map(|x| x.value)).map(UInt32::from)
    }

    fn blake_g(input: [u32; 6]) -> [u32; 4] {
        let [mut a, mut b, mut c, mut d, m0, m1] = input;

        a = a.wrapping_add(b).wrapping_add(m0);
        d ^= a;
        d = d.rotate_right(16);

        c = c.wrapping_add(d);
        b ^= c;
        b = b.rotate_right(12);

        a = a.wrapping_add(b).wrapping_add(m1);
        d ^= a;
        d = d.rotate_right(8);

        c = c.wrapping_add(d);
        b ^= c;
        b = b.rotate_right(7);

        [a, b, c, d]
    }
}

#[derive(Debug)]
pub struct TripleXor32 {}

// TODO(Stav): remove '#[allow(unused)]' when possible.
#[allow(unused)]
impl TripleXor32 {
    pub fn deduce_output(input: [UInt32; 3]) -> UInt32 {
        UInt32::from(input[0].value ^ input[1].value ^ input[2].value)
    }
}

#[derive(Debug)]
pub struct BlakeRoundSigma {}

// TODO(Stav): remove '#[allow(unused)]' when possible.
#[allow(unused)]
impl BlakeRoundSigma {
    pub fn deduce_output(round: M31) -> [M31; N_BLAKE_SIGMA_COLS] {
        sigma(round.0 as usize)
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
        chain: M31,
        round: M31,
        (state, message_pointer): ([UInt32; 16], M31),
    ) -> (M31, M31, ([UInt32; 16], M31)) {
        let sigma = sigma(round.0 as usize);
        let mut message = vec![];
        for i in sigma.iter() {
            message.push(self.memory.get(message_pointer.0 + i.0).as_small() as u32);
        }

        let mut state = state.map(|x| x.value);
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

        (
            chain,
            round + M31::from(1),
            (state.map(UInt32::from), message_pointer),
        )
    }
}

#[cfg(test)]
mod tests {
    use stwo_cairo_adapter::memory::{MemoryBuilder, MemoryConfig};
    use stwo_cairo_adapter::vm_import::MemoryEntry;

    use super::*;

    #[test]
    fn test_g() {
        let input = [
            305419896, 4294967295, 2147483647, 123456789, 987654321, 468798,
        ];
        let expected = [2827666065, 4146123195, 3407348176, 3638212488].map(UInt32::from);
        let actual = BlakeG::deduce_output(input.map(UInt32::from));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_round_sigma() {
        let input = 7;
        let expected = [13, 11, 7, 14, 12, 1, 3, 9, 5, 0, 15, 4, 8, 6, 2, 10].map(M31::from);
        let actual = BlakeRoundSigma::deduce_output(M31::from(input));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_blake_round() {
        // Create input
        let state = [
            1589929985, 669959787, 3341104026, 828450965, 1955226293, 542713244, 3587648250,
            2032424797, 3147641385, 3967920621, 2006879305, 2745232376, 2456599919, 130066657,
            1468412498, 325435090,
        ];
        let message_pointer: u32 = 7687346;

        let messgae: [u32; 16] = [
            1190313840, 586871615, 3326317950, 2157490798, 2171729911, 4006315130, 3006051123,
            3934250148, 745259603, 1963379556, 3874654107, 2051567115, 2102274589, 1991875188,
            1621381226, 1307057221,
        ];

        // Create Memory
        let memory_entries: Vec<MemoryEntry> = (0..=15)
            .map(|i| MemoryEntry {
                address: message_pointer as u64 + i as u64,
                value: [messgae[i as usize], 0, 0, 0, 0, 0, 0, 0],
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
        let actual = blake_round.deduce_output(
            M31::from(0),
            M31::from(5),
            (state.map(UInt32::from), M31::from(message_pointer)),
        );
        assert_eq!(actual, expected);
    }
}
