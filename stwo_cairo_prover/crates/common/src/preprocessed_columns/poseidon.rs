use stwo::core::fields::m31::BaseField;
use stwo::core::poly::circle::CanonicCoset;
use stwo::prover::backend::simd::column::BaseColumn;
use stwo::prover::backend::simd::m31::PackedM31;
use stwo::prover::backend::simd::SimdBackend;
use stwo::prover::poly::circle::CircleEvaluation;
use stwo::prover::poly::BitReversedOrder;
use stwo_constraint_framework::preprocessed_columns::PreProcessedColumnId;

use super::preprocessed_trace::PreProcessedColumn;
use super::preprocessed_utils::pad;
use crate::preprocessed_columns::poseidon_round_keys::round_keys;
use crate::prover_types::cpu::{FELT252WIDTH27_N_WORDS, M31};
use crate::prover_types::simd::N_LANES;

const LOG_N_ROWS: u32 = (N_ROUNDS as u32).next_power_of_two().ilog2();
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
    pub packed_keys: [PackedM31; N_PACKED_ROWS],
    pub col: usize,
}

impl PoseidonRoundKeys {
    pub fn new(col: usize) -> Self {
        let packed_keys = BaseColumn::from_iter(pad(round_keys_m31, N_ROUNDS, col)).data;
        Self {
            packed_keys: packed_keys.try_into().unwrap(),
            col,
        }
    }
}

impl PreProcessedColumn for PoseidonRoundKeys {
    fn log_size(&self) -> u32 {
        LOG_N_ROWS
    }

    fn packed_at(&self, vec_row: usize) -> PackedM31 {
        self.packed_keys[vec_row]
    }

    fn gen_column_simd(&self) -> CircleEvaluation<SimdBackend, BaseField, BitReversedOrder> {
        CircleEvaluation::new(
            CanonicCoset::new(LOG_N_ROWS).circle_domain(),
            BaseColumn::from_simd(self.packed_keys.to_vec()),
        )
    }

    fn id(&self) -> PreProcessedColumnId {
        PreProcessedColumnId {
            id: format!("poseidon_round_keys_{}", self.col),
        }
    }
}

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
