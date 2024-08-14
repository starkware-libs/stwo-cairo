use component::{N_M31_IN_FELT252, N_MEMORY_COLUMNS};
use stwo_prover::constraint_framework::logup::LookupElements;
use stwo_prover::core::backend::simd::m31::PackedM31;

pub mod component;
pub mod prover;

const N_LOGUP_POWERS: usize = N_MEMORY_COLUMNS - 1;
pub type MemoryLookupElements = LookupElements<N_LOGUP_POWERS>;

pub const N_ADDRESS_FELTS: usize = 1;
pub const N_BITS_PER_FELT: usize = 9;

pub struct MemoryValues {
    pub values: Vec<[PackedM31; N_M31_IN_FELT252]>,
}
