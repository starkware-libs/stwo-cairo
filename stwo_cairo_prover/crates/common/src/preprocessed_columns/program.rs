use std::sync::{LazyLock, RwLock};

use stwo::core::fields::m31::M31;
use stwo_constraint_framework::preprocessed_columns::PreProcessedColumnId;

use super::preprocessed_trace::PreProcessedColumn;
#[cfg(feature = "prover")]
use super::simd_prelude::*;
use crate::prover_types::cpu::{Felt252, FELT252_N_WORDS};

pub const PROGRAM_N_COLUMNS: usize = FELT252_N_WORDS;

pub const MAX_PROGRAM_LOG_LEN: usize = 12;

static PROGRAM_TABLE: LazyLock<RwLock<Vec<[M31; FELT252_N_WORDS]>>> =
    LazyLock::new(|| RwLock::new(vec![[M31(0); FELT252_N_WORDS]; 1 << MAX_PROGRAM_LOG_LEN]));

/// Sets the global PROGRAM_TABLE. Pre-computes 9-bit limbs from the raw `[u32; 8]` values.
/// Called automatically when creating a preprocessed trace with program data.
pub fn set_program_table(program: &[(u32, [u32; 8])]) {
    assert!(program.len() <= 1 << MAX_PROGRAM_LOG_LEN);
    let mut values: Vec<[M31; FELT252_N_WORDS]> = program
        .iter()
        .map(|(_, v)| {
            let limbs = [
                v[0] as u64 | ((v[1] as u64) << 32),
                v[2] as u64 | ((v[3] as u64) << 32),
                v[4] as u64 | ((v[5] as u64) << 32),
                v[6] as u64 | ((v[7] as u64) << 32),
            ];
            Felt252::from(limbs).get_limbs()
        })
        .collect();
    // Pad to the fixed program table size so ProgramColumn has the correct log_size.
    values.resize(1 << MAX_PROGRAM_LOG_LEN, [M31(0); FELT252_N_WORDS]);
    *PROGRAM_TABLE.write().unwrap() = values;
}

#[derive(Debug)]
pub struct ProgramColumn {
    col_index: usize,
    column_data: Vec<M31>,
}
impl ProgramColumn {
    pub fn new(col_index: usize) -> Self {
        let data = PROGRAM_TABLE.read().unwrap();
        let padded_len = data.len().next_power_of_two();
        let column_data = (0..padded_len)
            .map(|i| {
                if i < data.len() {
                    data[i][col_index]
                } else {
                    M31(0)
                }
            })
            .collect();
        Self {
            col_index,
            column_data,
        }
    }

    pub fn get_data(&self) -> &[M31] {
        &self.column_data
    }
}

impl PreProcessedColumn for ProgramColumn {
    fn log_size(&self) -> u32 {
        self.column_data.len().ilog2()
    }

    fn id(&self) -> PreProcessedColumnId {
        PreProcessedColumnId {
            id: format!("curr_program_{}", self.col_index),
        }
    }

    #[cfg(feature = "prover")]
    fn packed_at(&self, vec_row: usize) -> PackedM31 {
        let array = self.get_data()[(vec_row * N_LANES)..((vec_row + 1) * N_LANES)]
            .try_into()
            .unwrap();
        PackedM31::from_array(array)
    }

    #[cfg(feature = "prover")]
    fn gen_column_simd(&self) -> CircleEvaluation<SimdBackend, BaseField, BitReversedOrder> {
        CircleEvaluation::new(
            CanonicCoset::new(self.log_size()).circle_domain(),
            BaseColumn::from_cpu(self.get_data()),
        )
    }
}
