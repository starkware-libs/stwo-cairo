use prover_types::cpu::{Felt252Packed27, FELT252PACKED27_N_WORDS, M31};
use stwo_prover::constraint_framework::preprocessed_columns::PreProcessedColumnId;
use stwo_prover::core::backend::simd::column::BaseColumn;
use stwo_prover::core::backend::simd::m31::{PackedM31, LOG_N_LANES};
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::fields::m31::BaseField;
use stwo_prover::core::poly::circle::{CanonicCoset, CircleEvaluation};
use stwo_prover::core::poly::BitReversedOrder;

use super::super::preprocessed_utils::{pack, table_id_to_col_id};
use super::consts::POSEIDON_ROUND_KEYS;
use crate::cairo_air::preprocessed::PreProcessedColumn;

pub const POSEIDON_ROUND_KEYS_TABLE: &str = "poseidon_round_keys";

pub fn round_keys(round: usize) -> [Felt252Packed27; 3] {
    POSEIDON_ROUND_KEYS[round].map(|k| Felt252Packed27 { limbs: k })
}

pub fn round_keys_m31(round: usize, col: usize) -> M31 {
    assert!(col < FELT252PACKED27_N_WORDS * 3);
    assert!(round < 35);

    let felt252_index = col / FELT252PACKED27_N_WORDS;
    let felt_index = col % FELT252PACKED27_N_WORDS;
    round_keys(round)[felt252_index].get_m31(felt_index)
}

#[derive(Debug)]
pub struct PoseidonRoundKeysColumn {
    pub col: usize,
}

impl PoseidonRoundKeysColumn {
    pub fn packed_at(&self, vec_row: usize) -> PackedM31 {
        pack::<35, _>(round_keys_m31, vec_row, self.col)
    }
}

impl PreProcessedColumn for PoseidonRoundKeysColumn {
    fn log_size(&self) -> u32 {
        6
    }

    fn gen_column_simd(&self) -> CircleEvaluation<SimdBackend, BaseField, BitReversedOrder> {
        CircleEvaluation::new(
            CanonicCoset::new(self.log_size()).circle_domain(),
            BaseColumn::from_simd(
                (0..(1 << (self.log_size() - LOG_N_LANES)))
                    .map(|i| self.packed_at(i))
                    .collect(),
            ),
        )
    }

    fn id(&self) -> PreProcessedColumnId {
        PreProcessedColumnId {
            id: table_id_to_col_id(POSEIDON_ROUND_KEYS_TABLE, self.col),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::array::from_fn;

    use stwo_prover::core::backend::simd::m31::N_LANES;

    use super::*;

    #[test]
    fn test_packed_at_round_keys() {
        for vec_row in 0..4 {
            for i in 0..3 {
                let packed: [[M31; N_LANES]; FELT252PACKED27_N_WORDS] = from_fn(|c| {
                    PoseidonRoundKeysColumn {
                        col: (i * FELT252PACKED27_N_WORDS) + c,
                    }
                    .packed_at(vec_row)
                    .to_array()
                });
                for row_in_packed in 0..N_LANES {
                    let felt_limbs: [M31; FELT252PACKED27_N_WORDS] = packed
                        .iter()
                        .map(|arr| arr[row_in_packed])
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap();
                    let row = (vec_row * N_LANES) + row_in_packed;
                    if row < 35 {
                        assert_eq!(Felt252Packed27::from_limbs(&felt_limbs), round_keys(row)[i]);
                    } else {
                        assert_eq!(Felt252Packed27::from_limbs(&felt_limbs), round_keys(0)[i]);
                    }
                }
            }
        }
    }
}
