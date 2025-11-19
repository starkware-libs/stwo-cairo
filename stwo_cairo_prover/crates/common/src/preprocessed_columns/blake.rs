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
use crate::prover_types::cpu::M31;

pub const N_BLAKE_ROUNDS: usize = 10;
pub const N_BLAKE_SIGMA_COLS: usize = 16;

pub const BLAKE_SIGMA: [[u32; N_BLAKE_SIGMA_COLS]; N_BLAKE_ROUNDS] = [
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    [14, 10, 4, 8, 9, 15, 13, 6, 1, 12, 0, 2, 11, 7, 5, 3],
    [11, 8, 12, 0, 5, 2, 15, 13, 10, 14, 3, 6, 7, 1, 9, 4],
    [7, 9, 3, 1, 13, 12, 11, 14, 2, 6, 5, 10, 4, 0, 15, 8],
    [9, 0, 5, 7, 2, 4, 10, 15, 14, 1, 11, 12, 6, 8, 3, 13],
    [2, 12, 6, 10, 0, 11, 8, 3, 4, 13, 7, 5, 15, 14, 1, 9],
    [12, 5, 1, 15, 14, 13, 4, 10, 0, 7, 6, 3, 9, 2, 8, 11],
    [13, 11, 7, 14, 12, 1, 3, 9, 5, 0, 15, 4, 8, 6, 2, 10],
    [6, 15, 14, 9, 11, 3, 0, 8, 12, 2, 13, 7, 1, 4, 10, 5],
    [10, 2, 8, 4, 7, 6, 1, 5, 15, 11, 9, 14, 3, 12, 13, 0],
];

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

    fn packed_at(&self, vec_row: usize) -> PackedM31 {
        assert!(
            vec_row == 0,
            "Accessing BlakeSigma out of bounds row {vec_row}"
        );
        PackedM31::from_array(pad(sigma_m31, N_BLAKE_ROUNDS, self.col).try_into().unwrap())
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
