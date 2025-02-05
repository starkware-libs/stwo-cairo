#[cfg(test)]
use std::array::from_fn;

use num_traits::Zero;
use prover_types::cpu::M31;
use stwo_prover::constraint_framework::preprocessed_columns::PreProcessedColumnId;
use stwo_prover::core::backend::simd::column::BaseColumn;
use stwo_prover::core::backend::simd::m31::{PackedM31, LOG_N_LANES, N_LANES};
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::fields::m31::BaseField;
use stwo_prover::core::poly::circle::{CanonicCoset, CircleEvaluation};
use stwo_prover::core::poly::BitReversedOrder;

pub const POSEIDON_ROUND_NUMBER_COLUMN: &str = "poseidon_round_number";

#[derive(Debug)]
pub struct PoseidonRoundNumber {}

impl PoseidonRoundNumber {
    pub const fn log_size(&self) -> u32 {
        4
    }

    pub fn packed_at(&self, vec_row: usize) -> PackedM31 {
        let nums: [M31; N_LANES] = (vec_row * N_LANES..(vec_row + 1) * N_LANES)
            .map(|i| {
                if i < 35 {
                    M31::from(i as u32)
                } else {
                    M31::zero()
                }
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        PackedM31::from_array(nums)
    }

    pub fn gen_column_simd(&self) -> CircleEvaluation<SimdBackend, BaseField, BitReversedOrder> {
        CircleEvaluation::new(
            CanonicCoset::new(self.log_size()).circle_domain(),
            BaseColumn::from_simd(
                (0..(1 << (self.log_size() - LOG_N_LANES)))
                    .map(|i| self.packed_at(i))
                    .collect(),
            ),
        )
    }

    pub fn id(&self) -> PreProcessedColumnId {
        PreProcessedColumnId {
            id: POSEIDON_ROUND_NUMBER_COLUMN.to_string(),
        }
    }
}

#[test]
fn test_packed_at_round_number() {
    let nums = PoseidonRoundNumber {};
    let expected: [M31; 64] = from_fn(|i| {
        if i < 35 {
            M31::from(i as u32)
        } else {
            M31::zero()
        }
    });
    let packed: [[M31; 16]; 4] = from_fn(|i| nums.packed_at(i).to_array());
    let packed_flat: [M31; 64] = packed
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    assert_eq!(packed_flat, expected);
}
