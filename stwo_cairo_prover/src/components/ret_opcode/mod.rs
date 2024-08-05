use num_traits::{One, Zero};
use stwo_prover::core::backend::simd::m31::PackedM31;

use super::memory::component::N_M31_IN_FELT252;

pub mod component;
pub mod simd_trace;

pub fn packed_felt_252_one() -> [PackedM31; N_M31_IN_FELT252] {
    let mut value = [PackedM31::zero(); N_M31_IN_FELT252];
    value[0] = PackedM31::one();
    value
}
