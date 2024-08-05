use component::N_MEMORY_COLUMNS;
use stwo_prover::constraint_framework::logup::LookupElements;


pub mod component;
pub mod simd_trace;

pub type MemoryLookupElements = LookupElements<N_MEMORY_COLUMNS>;

pub const N_ADDRESS_FELTS: usize = 1;
pub const N_BITS_PER_FELT: usize = 9;