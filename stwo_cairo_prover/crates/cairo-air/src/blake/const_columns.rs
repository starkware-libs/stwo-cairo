use stwo_cairo_common::preprocessed_consts::blake::{
    BLAKE_SIGMA, N_BLAKE_ROUNDS, N_BLAKE_SIGMA_COLS,
};
use stwo_cairo_common::prover_types::cpu::M31;
use stwo_constraint_framework::preprocessed_columns::PreProcessedColumnId;
use stwo_prover::core::fields::m31::BaseField;
use stwo_prover::core::poly::circle::CanonicCoset;
use stwo_prover::prover::backend::simd::column::BaseColumn;
use stwo_prover::prover::backend::simd::SimdBackend;
use stwo_prover::prover::poly::circle::CircleEvaluation;
use stwo_prover::prover::poly::BitReversedOrder;

use crate::preprocessed::PreProcessedColumn;
use crate::preprocessed_utils::pad;

pub const BLAKE_SIGMA_TABLE: &str = "blake_sigma";
const LOG_N_ROWS: u32 = (N_BLAKE_ROUNDS as u32).next_power_of_two().ilog2();

pub fn sigma(round: usize) -> [u32; N_BLAKE_SIGMA_COLS] {
    BLAKE_SIGMA[round]
}

pub fn sigma_m31(round: usize, col: usize) -> M31 {
    assert!(col < N_BLAKE_SIGMA_COLS);
    assert!(round < N_BLAKE_ROUNDS);
    (sigma(round)[col]).into()
}

#[derive(Debug)]
pub struct BlakeSigma {
    pub col: usize,
}

impl BlakeSigma {
    pub fn new(col: usize) -> Self {
        Self { col }
    }
}

impl PreProcessedColumn for BlakeSigma {
    fn log_size(&self) -> u32 {
        LOG_N_ROWS
    }

    fn gen_column_simd(&self) -> CircleEvaluation<SimdBackend, BaseField, BitReversedOrder> {
        CircleEvaluation::new(
            CanonicCoset::new(LOG_N_ROWS).circle_domain(),
            BaseColumn::from_iter(pad(sigma_m31, N_BLAKE_ROUNDS, self.col)),
        )
    }

    fn id(&self) -> PreProcessedColumnId {
        PreProcessedColumnId {
            id: format!("{}_{}", BLAKE_SIGMA_TABLE, self.col),
        }
    }
}
