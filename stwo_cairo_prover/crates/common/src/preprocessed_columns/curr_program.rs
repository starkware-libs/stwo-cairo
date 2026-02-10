use stwo::core::fields::m31::M31;
use stwo_constraint_framework::preprocessed_columns::PreProcessedColumnId;

use super::preprocessed_trace::PreProcessedColumn;
#[cfg(feature = "prover")]
use super::simd_prelude::*;

/// Number of 9-bit limbs needed to represent a Cairo instruction (63 bits).
pub const PROGRAM_N_COLUMNS: usize = 65 / 9;

/// Extracts a 9-bit limb from a 256-bit value stored as `[u32; 8]` little-endian limbs.
fn extract_9bit_limb(value: &[u32; 8], limb_index: usize) -> u32 {
    let bit_offset = limb_index * 9;
    let word_index = bit_offset / 32;
    let bit_shift = bit_offset % 32;
    let mut result = value[word_index] >> bit_shift;
    // Handle the case where the 9-bit limb spans two u32 words.
    if bit_shift + 9 > 32 {
        result |= value[word_index + 1] << (32 - bit_shift);
    }
    result & 0x1FF
}

#[derive(Debug)]
pub struct CurrProgramColumn {
    col_index: usize,
    column_data: Vec<M31>,
}
impl CurrProgramColumn {
    pub fn new(col_index: usize, program: &[(u32, [u32; 8])]) -> Self {
        let padded_len = program.len().next_power_of_two();
        let column_data = (0..padded_len)
            .map(|i| {
                if i < program.len() {
                    M31::from_u32_unchecked(extract_9bit_limb(&program[i].1, col_index))
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

    pub fn get_data(&self) -> &Vec<M31> {
        &self.column_data
    }
}

impl PreProcessedColumn for CurrProgramColumn {
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

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_extract_9bit_limb() {
        // Value: 0b_000_111_010_110_001_011_101 = 0x1D6B5 (in 7 limbs of 9 bits)
        // limb 0: 101 = 0b_000_101_101 ... let's use a concrete example.
        // value[0] = 0x01FF_03FE = bits: limb0=0x1FE(510), limb1=0x1FF(511), ...
        // Actually let's just test specific values.

        // All zeros.
        let value = [0u32; 8];
        for i in 0..7 {
            assert_eq!(extract_9bit_limb(&value, i), 0);
        }

        // limb 0 = lower 9 bits of value[0].
        let value = [0x1AB, 0, 0, 0, 0, 0, 0, 0]; // 0x1AB = 0b110101011 = 427
        assert_eq!(extract_9bit_limb(&value, 0), 0x1AB);

        // limb 3 spans value[0] bits 27-31 and value[1] bits 0-3.
        // bit_offset = 27, word_index = 0, bit_shift = 27.
        // Place 0b_101010111 at bits 27-35:
        // value[0] bits 27-31 = lower 5 bits of limb = 0b10111 => value[0] |= 0b10111 << 27
        // value[1] bits 0-3 = upper 4 bits of limb = 0b1010 => value[1] |= 0b1010
        let value = [0b10111 << 27, 0b1010, 0, 0, 0, 0, 0, 0];
        assert_eq!(extract_9bit_limb(&value, 3), 0b101010111);
    }

    #[test]
    fn test_program_column_new() {
        let program = vec![
            (0u32, [0x1FFu32, 0, 0, 0, 0, 0, 0, 0]), // limb 0 = 0x1FF (511)
            (1, [0, 0, 0, 0, 0, 0, 0, 0]),           // all zeros
        ];

        let col0 = CurrProgramColumn::new(0, &program);
        // Padded to next power of 2 = 2.
        assert_eq!(col0.column_data.len(), 2);
        assert_eq!(col0.column_data[0], M31(511));
        assert_eq!(col0.column_data[1], M31(0));

        let col1 = CurrProgramColumn::new(1, &program);
        // 0x1FF in bits 0-8, limb 1 = bits 9-17 = 0.
        assert_eq!(col1.column_data[0], M31(0));
    }

    #[test]
    fn test_program_column_pads_to_power_of_two() {
        let program = vec![
            (0u32, [1u32, 0, 0, 0, 0, 0, 0, 0]),
            (1, [2, 0, 0, 0, 0, 0, 0, 0]),
            (2, [3, 0, 0, 0, 0, 0, 0, 0]),
        ];

        let col = CurrProgramColumn::new(0, &program);
        // 3 entries padded to 4.
        assert_eq!(col.column_data.len(), 4);
        assert_eq!(col.column_data[0], M31(1));
        assert_eq!(col.column_data[1], M31(2));
        assert_eq!(col.column_data[2], M31(3));
        assert_eq!(col.column_data[3], M31(0));
    }

    #[test]
    fn test_program_column_log_size() {
        let program = vec![(0u32, [0u32; 8]), (1, [0; 8]), (2, [0; 8])];
        let col = CurrProgramColumn::new(0, &program);
        // 3 entries padded to 4 = 2^2.
        assert_eq!(col.log_size(), 2);
    }

    #[test]
    fn test_program_column_id() {
        let program = vec![(0u32, [0u32; 8])];
        let col = CurrProgramColumn::new(3, &program);
        assert_eq!(col.id().id, "curr_program_3");
    }

    #[cfg(feature = "prover")]
    #[test]
    fn test_packed_at() {
        // Create a program with N_LANES entries so packed_at(0) covers them all.
        let program: Vec<(u32, [u32; 8])> = (0..N_LANES)
            .map(|i| (i as u32, [i as u32, 0, 0, 0, 0, 0, 0, 0]))
            .collect();

        let col = CurrProgramColumn::new(0, &program);
        let packed = col.packed_at(0);
        let array = packed.to_array();

        for (i, val) in array.iter().enumerate() {
            assert_eq!(*val, M31(i as u32));
        }
    }
}
