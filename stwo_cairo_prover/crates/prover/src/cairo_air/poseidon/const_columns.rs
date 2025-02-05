use std::array::from_fn;
use std::rc::Rc;

use num_traits::Zero;
use prover_types::cpu::{Felt252Packed27, FELT252PACKED27_N_WORDS, M31};
use stwo_prover::constraint_framework::preprocessed_columns::PreProcessedColumnId;
use stwo_prover::core::backend::simd::column::BaseColumn;
#[cfg(test)]
use stwo_prover::core::backend::simd::m31::N_LANES;
use stwo_prover::core::backend::simd::m31::{PackedM31, LOG_N_LANES};
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::fields::m31::BaseField;
use stwo_prover::core::poly::circle::{CanonicCoset, CircleEvaluation};
use stwo_prover::core::poly::BitReversedOrder;

use super::super::preprocessed::table_id_to_col_id;
use super::consts::POSEIDON_ROUND_KEYS;
use crate::cairo_air::preprocessed::PreProcessedColumn;

pub const POSEIDON_ROUND_KEYS_TABLE: &str = "poseidon_round_keys";

pub fn round_keys(round: M31) -> [Felt252Packed27; 3] {
    POSEIDON_ROUND_KEYS[round.0 as usize].map(|k| Felt252Packed27 { limbs: k })
}

pub fn round_keys_m31s(round: M31) -> [[M31; FELT252PACKED27_N_WORDS]; 3] {
    round_keys(round).map(|felt| from_fn(|i| felt.get_m31(i)))
}

#[derive(Debug)]
pub struct PoseidonRoundKeysPackedM31(pub [[PackedM31; FELT252PACKED27_N_WORDS * 3]; 4]);

impl PoseidonRoundKeysPackedM31 {
    pub fn new() -> Self {
        // Add the first row until we have 64 rows
        let first_row = round_keys_m31s(M31::zero());
        let keys: [[[M31; FELT252PACKED27_N_WORDS]; 3]; 64] = from_fn(|i| match i {
            1..35 => round_keys_m31s(M31::from(i)),
            _ => first_row,
        });

        // Pack every 16 rows into PackedM31
        let mut packed = [[[PackedM31::broadcast(M31::zero()); FELT252PACKED27_N_WORDS]; 3]; 4];
        for j in 0..3 {
            for k in 0..FELT252PACKED27_N_WORDS {
                for (i, r) in [0..16, 16..32, 32..48, 48..64].into_iter().enumerate() {
                    packed[i][j][k] = PackedM31::from_array(
                        keys.get(r)
                            .unwrap()
                            .iter()
                            .map(|arr| arr[j][k])
                            .collect::<Vec<_>>()
                            .try_into()
                            .unwrap(),
                    );
                }
            }
        }

        // Flatten the array
        let flat: [[PackedM31; FELT252PACKED27_N_WORDS * 3]; 4] = packed
            .into_iter()
            .map(|arr| {
                arr.into_iter()
                    .flatten()
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        Self(flat)
    }
}

impl Default for PoseidonRoundKeysPackedM31 {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct PoseidonRoundKeysColumn {
    pub keys: Rc<PoseidonRoundKeysPackedM31>,
    pub index: usize,
}

impl PoseidonRoundKeysColumn {
    pub const fn new(keys: Rc<PoseidonRoundKeysPackedM31>, index: usize) -> Self {
        Self { keys, index }
    }

    pub fn packed_at(&self, vec_row: usize) -> PackedM31 {
        self.keys.0[vec_row][self.index]
    }
}

impl PreProcessedColumn for PoseidonRoundKeysColumn {
    fn log_size(&self) -> u32 {
        4
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
            id: table_id_to_col_id(POSEIDON_ROUND_KEYS_TABLE, self.index),
        }
    }
}

#[test]
fn test_packed_at_round_keys() {
    let keys_packed = Rc::new(PoseidonRoundKeysPackedM31::new());
    for vec_row in 0..4 {
        for i in 0..3 {
            let packed: [[M31; N_LANES]; FELT252PACKED27_N_WORDS] = from_fn(|c| {
                PoseidonRoundKeysColumn::new(keys_packed.clone(), (i * FELT252PACKED27_N_WORDS) + c)
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
                    assert_eq!(
                        Felt252Packed27::from_limbs(&felt_limbs),
                        round_keys(M31::from(row))[i]
                    );
                } else {
                    assert_eq!(
                        Felt252Packed27::from_limbs(&felt_limbs),
                        round_keys(M31::zero())[i]
                    );
                }
            }
        }
    }
}
