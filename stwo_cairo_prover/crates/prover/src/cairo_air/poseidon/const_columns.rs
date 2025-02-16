use stwo_cairo_common::preprocessed_consts::poseidon::{round_keys, N_ROUNDS, N_WORDS};
use stwo_cairo_common::prover_types::cpu::{FELT252WIDTH27_N_WORDS, M31};
use stwo_cairo_common::prover_types::simd::N_LANES;
use stwo_prover::constraint_framework::preprocessed_columns::PreProcessedColumnId;
use stwo_prover::core::backend::simd::column::BaseColumn;
use stwo_prover::core::backend::simd::m31::PackedM31;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::fields::m31::BaseField;
use stwo_prover::core::poly::circle::{CanonicCoset, CircleEvaluation};
use stwo_prover::core::poly::BitReversedOrder;

use super::super::preprocessed_utils::pad;
use crate::cairo_air::preprocessed::PreProcessedColumn;

const LOG_N_ROWS: u32 = (N_ROUNDS as u32).next_power_of_two().ilog2();
const N_PACKED_ROWS: usize = (2_u32.pow(LOG_N_ROWS)) as usize / N_LANES;

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

    // TODO(Gali): remove unused
    #[allow(unused)]
    pub fn packed_at(&self, vec_row: usize) -> PackedM31 {
        self.packed_keys[vec_row]
    }
}

impl PreProcessedColumn for PoseidonRoundKeys {
    fn log_size(&self) -> u32 {
        LOG_N_ROWS
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

    use stwo_cairo_common::preprocessed_consts::poseidon::N_FELT252WIDTH27;
    use stwo_cairo_common::prover_types::cpu::Felt252Width27;
    use stwo_prover::core::backend::simd::m31::N_LANES;

    use super::*;

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
