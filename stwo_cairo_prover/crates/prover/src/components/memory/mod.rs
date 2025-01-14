use std::simd::Simd;

use stwo_prover::core::backend::simd::m31::{PackedM31, N_LANES};
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::backend::Col;
use stwo_prover::core::fields::m31::{BaseField, M31};
use stwo_prover::core::poly::circle::{CanonicCoset, CircleEvaluation};
use stwo_prover::core::poly::BitReversedOrder;

pub mod memory_address_to_id;
pub mod memory_id_to_big;

/// Used for sanity checks and assertions.
pub const LOG_MEMORY_ADDRESS_BOUND: u32 = 27;
pub const MEMORY_ADDRESS_BOUND: usize = 1 << LOG_MEMORY_ADDRESS_BOUND;

const SIMD_ENUMERATION_0: PackedM31 = unsafe {
    PackedM31::from_simd_unchecked(Simd::from_array([
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
    ]))
};

/// A column with the numbers [0..2^log_size-1].
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
        PackedM31::broadcast(M31::from(vec_row * N_LANES)) + SIMD_ENUMERATION_0
    }

    pub fn gen_column_simd(&self) -> CircleEvaluation<SimdBackend, BaseField, BitReversedOrder> {
        let col = Col::<SimdBackend, BaseField>::from_iter(
            (0..(1 << self.log_size)).map(BaseField::from),
        );
        CircleEvaluation::new(CanonicCoset::new(self.log_size).circle_domain(), col)
    }

    pub fn id(&self) -> String {
        format!("preprocessed_is_seq_{}", self.log_size).to_string()
    }
}
