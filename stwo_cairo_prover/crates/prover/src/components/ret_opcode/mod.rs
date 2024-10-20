use num_traits::{One, Zero};
use stwo_prover::core::backend::simd::m31::PackedM31;

use super::memory::id_to_f252::component::N_M31_IN_FELT252;

pub mod component;
pub mod prover;

pub fn packed_felt_252_one() -> [PackedM31; N_M31_IN_FELT252] {
    let mut value = [PackedM31::zero(); N_M31_IN_FELT252];
    value[0] = PackedM31::one();
    value
}
