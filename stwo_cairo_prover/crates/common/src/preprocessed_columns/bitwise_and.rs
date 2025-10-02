/// A table of a,b,c, where a,b,c are integers and a & b = c.
///
/// # Attributes
///
/// - `n_bits`: The number of bits in each integer.
/// - `col_index`: The column index in the preprocessed table.
#[derive(Debug)]
pub struct BitwiseAnd {
    n_bits: u32,
    col_index: usize,
}
impl BitwiseAnd {
    pub const fn new(n_bits: u32, col_index: usize) -> Self {
        assert!(col_index < 3, "col_index must be in range 0..=2");
        Self { n_bits, col_index }
    }

    pub fn packed_at(&self, vec_row: usize) -> PackedM31 {
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
            2 => lhs() & rhs(),
            _ => unreachable!(),
        };
        unsafe { PackedM31::from_simd_unchecked(simd) }
    }
}
impl PreProcessedColumn for BitwiseAnd {
    fn log_size(&self) -> u32 {
        2 * self.n_bits
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
            id: format!("bitwise_and_{}_{}", self.n_bits, self.col_index),
        }
    }
}
