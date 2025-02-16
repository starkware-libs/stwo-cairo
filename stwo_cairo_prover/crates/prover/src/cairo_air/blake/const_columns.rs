use stwo_cairo_common::preprocessed_consts::blake::{
    BLAKE_SIGMA, N_BLAKE_ROUNDS, N_BLAKE_SIGMA_COLS,
};
use stwo_cairo_common::prover_types::cpu::M31;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::fields::m31::BaseField;
use stwo_prover::core::poly::circle::{CanonicCoset, CircleEvaluation};
use stwo_prover::core::poly::BitReversedOrder;

use crate::cairo_air::preprocessed::PreProcessedColumn;
use crate::cairo_air::preprocessed_utils::pad;
use crate::stwo_prover::constraint_framework::preprocessed_columns::PreProcessedColumnId;
use crate::stwo_prover::core::backend::simd::column::BaseColumn;
use crate::stwo_prover::core::backend::simd::m31::PackedM31;

pub const BLAKE_SIGMA_TABLE: &str = "blake_sigma";
const LOG_N_ROWS: u32 = (N_BLAKE_ROUNDS as u32).next_power_of_two().ilog2();

pub fn sigma(round: usize) -> [M31; N_BLAKE_SIGMA_COLS] {
    BLAKE_SIGMA[round].map(M31::from)
}

pub fn sigma_m31(round: usize, col: usize) -> M31 {
    assert!(col < N_BLAKE_SIGMA_COLS);
    assert!(round < N_BLAKE_ROUNDS);
    sigma(round)[col]
}

#[derive(Debug)]
pub struct BlakeSigmaColumn {
    pub values: PackedM31,
    pub col: usize,
}

impl BlakeSigmaColumn {
    pub fn new(col: usize) -> Self {
        Self {
            values: PackedM31::from_array(
                pad(sigma_m31, N_BLAKE_ROUNDS, col)
                    .try_into()
                    .expect("Expected BLAKE sigma column padding to have 16 rows"),
            ),
            col,
        }
    }
}

impl PreProcessedColumn for BlakeSigmaColumn {
    fn log_size(&self) -> u32 {
        LOG_N_ROWS
    }

    fn gen_column_simd(&self) -> CircleEvaluation<SimdBackend, BaseField, BitReversedOrder> {
        CircleEvaluation::new(
            CanonicCoset::new(LOG_N_ROWS).circle_domain(),
            BaseColumn::from_simd(vec![self.values]),
        )
    }

    fn id(&self) -> PreProcessedColumnId {
        PreProcessedColumnId {
            id: format!("{}_{}", BLAKE_SIGMA_TABLE, self.col),
        }
    }
}

#[cfg(test)]
mod tests {
    use stwo_prover::core::backend::simd::m31::N_LANES;

    use super::*;

    #[test]
    fn test_blake_sigma() {
        for i in 0..N_BLAKE_SIGMA_COLS {
            let sigma_col: [M31; N_LANES] = BlakeSigmaColumn::new(i).values.to_array();

            for (row, value) in sigma_col.iter().enumerate() {
                if row < N_BLAKE_ROUNDS {
                    assert_eq!(*value, sigma(row)[i]);
                } else {
                    assert_eq!(*value, sigma(0)[i]);
                }
            }
        }
    }
}
