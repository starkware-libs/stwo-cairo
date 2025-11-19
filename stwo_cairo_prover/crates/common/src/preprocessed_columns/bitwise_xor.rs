use std::simd::{u32x16, Simd};

use stwo::core::fields::m31::BaseField;
use stwo::core::poly::circle::CanonicCoset;
use stwo::prover::backend::simd::column::BaseColumn;
use stwo::prover::backend::simd::m31::PackedM31;
use stwo::prover::backend::simd::SimdBackend;
use stwo::prover::poly::circle::CircleEvaluation;
use stwo::prover::poly::BitReversedOrder;
use stwo_constraint_framework::preprocessed_columns::PreProcessedColumnId;

use super::preprocessed_trace::PreProcessedColumn;
use super::preprocessed_utils::SIMD_ENUMERATION_0;
use crate::prover_types::simd::{LOG_N_LANES, N_LANES};

/// A table of a,b,c, where a,b,c are integers and a ^ b = c.
///
/// # Attributes
///
/// - `n_bits`: The number of bits in each integer.
/// - `col_index`: The column index in the preprocessed table.
#[derive(Debug)]
pub struct BitwiseXor {
    n_bits: u32,
    col_index: usize,
}
impl BitwiseXor {
    pub const fn new(n_bits: u32, col_index: usize) -> Self {
        assert!(col_index < 3, "col_index must be in range 0..=2");
        Self { n_bits, col_index }
    }
}

impl PreProcessedColumn for BitwiseXor {
    fn log_size(&self) -> u32 {
        2 * self.n_bits
    }

    fn packed_at(&self, vec_row: usize) -> PackedM31 {
        let lhs = || -> u32x16 {
            (SIMD_ENUMERATION_0 + Simd::splat((vec_row * N_LANES) as u32)) >> self.n_bits
        };
        let rhs = || -> u32x16 {
            (SIMD_ENUMERATION_0 + Simd::splat((vec_row * N_LANES) as u32))
                & Simd::splat((1 << self.n_bits) - 1)
        };
        let simd = match self.col_index {
            0 => lhs(),
            1 => rhs(),
            2 => lhs() ^ rhs(),
            _ => unreachable!(),
        };
        unsafe { PackedM31::from_simd_unchecked(simd) }
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
            id: format!("bitwise_xor_{}_{}", self.n_bits, self.col_index),
        }
    }
}

#[cfg(test)]
pub mod tests {
    const LOG_SIZE: u32 = 8;
    use crate::preprocessed_columns::bitwise_xor::BitwiseXor;
    use crate::preprocessed_columns::preprocessed_trace::PreProcessedColumn;
    use crate::prover_types::simd::N_LANES;

    #[test]
    fn test_packed_at_bitwise_xor() {
        let bitwise_a = BitwiseXor::new(LOG_SIZE, 0);
        let bitwise_b = BitwiseXor::new(LOG_SIZE, 1);
        let bitwise_xor = BitwiseXor::new(LOG_SIZE, 2);
        let index: usize = 1000;
        let a = index / (1 << LOG_SIZE);
        let b = index % (1 << LOG_SIZE);
        let expected_xor = a ^ b;

        let res_a = bitwise_a.packed_at(index / N_LANES).to_array()[index % N_LANES];
        let res_b = bitwise_b.packed_at(index / N_LANES).to_array()[index % N_LANES];
        let res_xor = bitwise_xor.packed_at(index / N_LANES).to_array()[index % N_LANES];

        assert_eq!(res_a.0, a as u32);
        assert_eq!(res_b.0, b as u32);
        assert_eq!(res_xor.0, expected_xor as u32);
    }
}
