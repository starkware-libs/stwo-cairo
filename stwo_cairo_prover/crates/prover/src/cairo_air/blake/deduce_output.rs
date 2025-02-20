use stwo_cairo_common::preprocessed_consts::blake::N_BLAKE_SIGMA_COLS;
use stwo_cairo_common::prover_types::cpu::*;

use super::const_columns::sigma;
#[cfg(test)]
use crate::components::{memory_address_to_id, memory_id_to_big};

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
impl TripleXor32 {
    pub fn deduce_output(input: [UInt32; 3]) -> UInt32 {
        UInt32::from(input[0].value ^ input[1].value ^ input[2].value)
    }
}

#[derive(Debug)]
pub struct BlakeRoundSigma {}
impl BlakeRoundSigma {
    pub fn deduce_output(round: M31) -> [M31; N_BLAKE_SIGMA_COLS] {
        sigma(round.0 as usize)
    }
}

#[cfg(test)]
pub struct BlakeRound<'a> {
    memory_address_to_id_state: &'a memory_address_to_id::ClaimGenerator,
    memory_id_to_big_state: &'a memory_id_to_big::ClaimGenerator,
}

#[cfg(test)]
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
            let id = self.memory_address_to_id_state.get_id(message_pointer + *i);
            message.push(self.memory_id_to_big_state.get_small_value(&id) as u32);
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
        let expected_output = [2827666065, 4146123195, 3407348176, 3638212488];
        assert_eq!(
            BlakeG::deduce_output(input.map(UInt32::from)),
            expected_output.map(UInt32::from)
        );
    }

    #[test]
    fn test_round_sigma() {
        let input = 7;
        let expected_output = [13, 11, 7, 14, 12, 1, 3, 9, 5, 0, 15, 4, 8, 6, 2, 10];
        assert_eq!(
            BlakeRoundSigma::deduce_output(M31::from(input)),
            expected_output.map(M31::from)
        );
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
        let memory_addr_to_id = memory_address_to_id::ClaimGenerator::new(&memory);
        let memory_id_to_value = memory_id_to_big::ClaimGenerator::new(&memory);

        // Expected output
        let expected_state = [
            1367297060, 2926508617, 107924025, 162546146, 2936957071, 4075222909, 2433518464,
            3059149654, 2047295278, 3825369540, 4231837284, 4024902448, 1952998180, 1326427959,
            3193012524, 1657286418,
        ];
        let expected_output = (
            M31::from(0),
            M31::from(6),
            (expected_state.map(UInt32::from), M31::from(message_pointer)),
        );

        let blake_round = BlakeRound {
            memory_address_to_id_state: &memory_addr_to_id,
            memory_id_to_big_state: &memory_id_to_value,
        };
        assert_eq!(
            blake_round.deduce_output(
                M31::from(0),
                M31::from(5),
                (state.map(UInt32::from), M31::from(message_pointer)),
            ),
            expected_output
        );
    }
}
