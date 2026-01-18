use stwo_constraint_framework::preprocessed_columns::PreProcessedColumnId;

use super::poseidon_round_keys::round_keys;
use super::preprocessed_trace::PreProcessedColumn;
use super::preprocessed_utils::pad;
#[cfg(feature = "prover")]
use super::simd_prelude::*;
use crate::prover_types::cpu::{FELT252WIDTH27_N_WORDS, M31};

const LOG_N_ROWS: u32 = (N_ROUNDS as u32).next_power_of_two().ilog2();
#[cfg(feature = "prover")]
const N_PACKED_ROWS: usize = (2_u32.pow(LOG_N_ROWS)) as usize / N_LANES;

pub const N_ROUNDS: usize = 35;
pub const N_FELT252WIDTH27: usize = 3;
pub const N_WORDS: usize = FELT252WIDTH27_N_WORDS * N_FELT252WIDTH27;

pub fn round_keys_m31(round: usize, col: usize) -> M31 {
    assert!(col < N_WORDS);
    assert!(round < N_ROUNDS);

    let felt252_index = col / FELT252WIDTH27_N_WORDS;
    let m31_index = col % FELT252WIDTH27_N_WORDS;
    round_keys(round)[felt252_index].get_m31(m31_index)
}

#[derive(Debug)]
pub struct PoseidonRoundKeys {
    pub keys: Vec<M31>,
    pub col: usize,
}

impl PoseidonRoundKeys {
    pub fn new(col: usize) -> Self {
        let keys = pad(round_keys_m31, N_ROUNDS, col);
        Self { keys, col }
    }
}

impl PreProcessedColumn for PoseidonRoundKeys {
    fn log_size(&self) -> u32 {
        LOG_N_ROWS
    }

    #[cfg(feature = "prover")]
    fn packed_at(&self, vec_row: usize) -> PackedM31 {
        // TODO(AnatG): Now that we store the keys unpacked, we should optimize this function.
        let packed_keys: [PackedM31; N_PACKED_ROWS] = BaseColumn::from_iter(self.keys.clone())
            .data
            .try_into()
            .unwrap();
        packed_keys[vec_row]
    }

    #[cfg(feature = "prover")]
    fn gen_column_simd(&self) -> CircleEvaluation<SimdBackend, BaseField, BitReversedOrder> {
        let packed_keys = BaseColumn::from_iter(self.keys.clone()).data;
        CircleEvaluation::new(
            CanonicCoset::new(LOG_N_ROWS).circle_domain(),
            BaseColumn::from_simd(packed_keys),
        )
    }

    fn id(&self) -> PreProcessedColumnId {
        PreProcessedColumnId {
            id: format!("poseidon_round_keys_{}", self.col),
        }
    }
}

#[cfg(feature = "prover")]
#[cfg(test)]
mod tests {
    use std::array::from_fn;

    use stwo::prover::backend::simd::m31::N_LANES;

    use super::*;
    use crate::prover_types::cpu::Felt252Width27;

    #[test]
    fn test_packed_at_round_keys() {
        for vec_row in 0..N_PACKED_ROWS {
            for i in 0..N_FELT252WIDTH27 {
                let packed: [[M31; N_LANES]; FELT252WIDTH27_N_WORDS] = from_fn(|c| {
                    PoseidonRoundKeys::new((i * FELT252WIDTH27_N_WORDS) + c)
                        .packed_at(vec_row)
                        .to_array()
                });
                for row_in_packed in 0..N_LANES {
                    let felt_limbs: [M31; FELT252WIDTH27_N_WORDS] = packed
                        .iter()
                        .map(|arr| arr[row_in_packed])
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap();
                    let row = (vec_row * N_LANES) + row_in_packed;
                    if row < N_ROUNDS {
                        assert_eq!(Felt252Width27::from_limbs(&felt_limbs), round_keys(row)[i]);
                    } else {
                        assert_eq!(Felt252Width27::from_limbs(&felt_limbs), round_keys(0)[i]);
                    }
                }
            }
        }
    }
}
