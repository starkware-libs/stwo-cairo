use stwo_prover::core::backend::simd::m31::{LOG_N_LANES, N_LANES};

use super::mem::Memory;
use crate::felt::split_f252;
use crate::prover_types::PackedUInt32;

#[derive(Debug)]
pub struct RangeCheckUnitInput {
    pub values: Vec<u32>,
}

impl RangeCheckUnitInput {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }

    pub fn collect_from_memory(&mut self, memory: &Memory) {
        for value in memory.iter_values() {
            let value = split_f252(value.as_u256());
            for limb in value {
                self.values.push(limb.0);
            }
        }
    }

    // Returns a 2D vector of SIMD vectors, where the first dimension is the repetition index and
    // the second dimension is the SIMD vector of multiplicities.
    pub fn to_2d_simd_vec<const LOG_HEIGHT: u32, const N_REPETITIONS: usize>(
        &self,
    ) -> [Vec<PackedUInt32>; N_REPETITIONS] {
        let mut multiplicities: [Vec<PackedUInt32>; N_REPETITIONS] = std::array::from_fn(|_| {
            vec![PackedUInt32::broadcast(0); 1 << (LOG_HEIGHT - LOG_N_LANES) as usize]
        });
        for value in self.values.iter() {
            let (rep, num) = (value >> LOG_HEIGHT, value % (1 << LOG_HEIGHT));
            multiplicities[rep as usize][(num as usize) / N_LANES].simd
                [(num as usize) % N_LANES] += 1;
        }
        multiplicities
    }
}

impl Default for RangeCheckUnitInput {
    fn default() -> Self {
        Self::new()
    }
}
