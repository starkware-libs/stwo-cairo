use itertools::{chain, Itertools};
use prover_types::simd::LOG_N_LANES;
use stwo_prover::constraint_framework::preprocessed_columns::{IsFirst, PreProcessedColumnId};
use stwo_prover::core::backend::simd::m31::{PackedM31, N_LANES};
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::backend::Col;
use stwo_prover::core::fields::m31::{BaseField, M31};
use stwo_prover::core::poly::circle::{CanonicCoset, CircleEvaluation};
use stwo_prover::core::poly::BitReversedOrder;

use super::LOG_MAX_ROWS;
use crate::components::range_check_vector::SIMD_ENUMERATION_0;

const N_PREPROCESSED_COLUMN_SIZES: usize = (LOG_MAX_ROWS - LOG_N_LANES) as usize + 1;

// List of sizes to initialize the preprocessed trace with for `PreprocessedColumn::IsFirst`.
const IS_FIRST_LOG_SIZES: [u32; N_PREPROCESSED_COLUMN_SIZES] = preprocessed_log_sizes();

// List of sizes to initialize the preprocessed trace with for `PreprocessedColumn::Seq`.
const SEQ_LOG_SIZES: [u32; N_PREPROCESSED_COLUMN_SIZES] = preprocessed_log_sizes();

/// [LOG_MAX_ROWS, LOG_MAX_ROWS - 1, ..., LOG_N_LANES]
const fn preprocessed_log_sizes() -> [u32; N_PREPROCESSED_COLUMN_SIZES] {
    let mut arr = [0; N_PREPROCESSED_COLUMN_SIZES];
    let mut i = 0;
    while i < N_PREPROCESSED_COLUMN_SIZES {
        arr[i] = LOG_MAX_ROWS - i as u32;
        i += 1;
    }
    arr
}

pub enum PreProcessedColumn {
    IsFirst(IsFirst),
    Seq(Seq),
}
impl PreProcessedColumn {
    pub fn log_size(&self) -> u32 {
        match self {
            PreProcessedColumn::IsFirst(column) => column.log_size,
            PreProcessedColumn::Seq(column) => column.log_size,
        }
    }
    pub fn id(&self) -> PreProcessedColumnId {
        match self {
            PreProcessedColumn::IsFirst(column) => column.id(),
            PreProcessedColumn::Seq(column) => column.id(),
        }
    }
    pub fn gen_column_simd(&self) -> CircleEvaluation<SimdBackend, BaseField, BitReversedOrder> {
        match self {
            PreProcessedColumn::IsFirst(column) => column.gen_column_simd(),
            PreProcessedColumn::Seq(column) => column.gen_column_simd(),
        }
    }

    pub fn packed_at(&self, vec_row: usize) -> PackedM31 {
        match self {
            PreProcessedColumn::IsFirst(column) => column.packed_at(vec_row),
            PreProcessedColumn::Seq(column) => column.packed_at(vec_row),
        }
    }
}

/// Returns column info for the preprocessed trace.
pub fn preprocessed_trace_columns() -> Vec<PreProcessedColumn> {
    let is_first_columns =
        IS_FIRST_LOG_SIZES.map(|log_size| PreProcessedColumn::IsFirst(IsFirst::new(log_size)));
    let seq_columns = SEQ_LOG_SIZES.map(|log_size| PreProcessedColumn::Seq(Seq::new(log_size)));
    chain![is_first_columns, seq_columns]
        .sorted_by_key(|column| std::cmp::Reverse(column.log_size()))
        .collect_vec()
}

/// A column with the numbers [0..(2^log_size)-1].
#[derive(Debug, Clone)]
pub struct Seq {
    pub log_size: u32,
}
impl Seq {
    pub const fn new(log_size: u32) -> Self {
        Self { log_size }
    }

    pub fn packed_at(&self, vec_row: usize) -> PackedM31 {
        assert!(vec_row < (1 << self.log_size) / N_LANES);
        PackedM31::broadcast(M31::from(vec_row * N_LANES))
            + unsafe { PackedM31::from_simd_unchecked(SIMD_ENUMERATION_0) }
    }

    pub fn gen_column_simd(&self) -> CircleEvaluation<SimdBackend, BaseField, BitReversedOrder> {
        let col = Col::<SimdBackend, BaseField>::from_iter(
            (0..(1 << self.log_size)).map(BaseField::from),
        );
        CircleEvaluation::new(CanonicCoset::new(self.log_size).circle_domain(), col)
    }

    pub fn id(&self) -> PreProcessedColumnId {
        PreProcessedColumnId {
            id: format!("preprocessed_seq_{}", self.log_size).to_string(),
        }
    }
}

/// A preprocessed table for the xor operation of 2 n_bits numbers.
/// The index_in_table is the column index in the preprocessed table.
#[derive(Debug)]
pub struct BitwiseXor {
    pub n_bits: u32,
    pub col_index: usize,
}
impl BitwiseXor {
    pub const fn new(n_bits: u32, col_index: usize) -> Self {
        assert!(col_index < 3, "col_index must be in range 0..=2");
        Self { n_bits, col_index }
    }

    pub fn id(&self) -> PreProcessedColumnId {
        PreProcessedColumnId {
            id: format!(
                "preprocessed_bitwise_xor_{}_{}",
                self.n_bits, self.col_index
            ),
        }
    }

    pub const fn log_size(&self) -> u32 {
        2 * self.n_bits
    }

    fn packed_at_lhs(&self, vec_row: usize) -> PackedM31 {
        let at_row: [BaseField; N_LANES] = (vec_row * N_LANES..(vec_row + 1) * N_LANES)
            .map(|i| BaseField::from_u32_unchecked((i >> self.n_bits) as u32))
            .collect_vec()
            .try_into()
            .unwrap();
        PackedM31::from_array(at_row)
    }

    fn packed_at_rhs(&self, vec_row: usize) -> PackedM31 {
        let at_row: [BaseField; N_LANES] = (vec_row * N_LANES..(vec_row + 1) * N_LANES)
            .map(|i| BaseField::from_u32_unchecked((i & ((1 << self.n_bits) - 1)) as u32))
            .collect_vec()
            .try_into()
            .unwrap();
        PackedM31::from_array(at_row)
    }

    pub fn packed_at(&self, vec_row: usize) -> PackedM31 {
        match self.col_index {
            0 => self.packed_at_lhs(vec_row),
            1 => self.packed_at_rhs(vec_row),
            2 => {
                let lhs_array = self.packed_at_lhs(vec_row).to_array();
                let rhs_array = self.packed_at_rhs(vec_row).to_array();
                let at_row: [BaseField; N_LANES] = std::array::from_fn(|i| {
                    BaseField::from_u32_unchecked(lhs_array[i].0 ^ rhs_array[i].0)
                });
                PackedM31::from_array(at_row)
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const LOG_SIZE: u32 = 8;
    use stwo_prover::core::backend::Column;

    #[test]
    fn test_columns_are_in_decending_order() {
        let columns = preprocessed_trace_columns();

        assert!(columns
            .windows(2)
            .all(|w| w[0].log_size() >= w[1].log_size()));
    }

    #[test]
    fn test_gen_seq() {
        let seq = Seq::new(LOG_SIZE).gen_column_simd();
        for i in 0..(1 << LOG_SIZE) {
            assert_eq!(seq.at(i), BaseField::from_u32_unchecked(i as u32));
        }
    }

    #[test]
    fn test_packed_at_seq() {
        let seq = Seq::new(LOG_SIZE);
        let expected_seq: [_; 1 << LOG_SIZE] = std::array::from_fn(|i| M31::from(i as u32));
        let packed_seq = std::array::from_fn::<_, { (1 << LOG_SIZE) / N_LANES }, _>(|i| {
            seq.packed_at(i).to_array()
        })
        .concat();
        assert_eq!(packed_seq, expected_seq);
    }

    #[test]
    fn test_packed_at_bitwise_xor() {
        let bitwise_xor = BitwiseXor::new(LOG_SIZE, 2);
        let expected_bitwise_xor: [_; 1 << (2 * LOG_SIZE)] = std::array::from_fn(|i| {
            let a = i >> LOG_SIZE;
            let b = i & ((1 << LOG_SIZE) - 1);
            M31::from((a ^ b) as u32)
        });

        let packed_bitwise_xor =
            std::array::from_fn::<_, { (1 << (2 * LOG_SIZE)) / N_LANES }, _>(|i| {
                bitwise_xor.packed_at(i).to_array()
            })
            .concat();

        assert_eq!(packed_bitwise_xor, expected_bitwise_xor);
    }
}
