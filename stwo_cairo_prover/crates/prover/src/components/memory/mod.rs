use component::N_MEMORY_COLUMNS;
use stwo_prover::constraint_framework::logup::LookupElements;

pub mod component;
pub mod prover;

const N_LOGUP_POWERS: usize = N_MEMORY_COLUMNS - 1;
pub type MemoryElements = LookupElements<N_LOGUP_POWERS>;

pub const N_ADDRESS_FELTS: usize = 1;
pub const N_BITS_PER_FELT: usize = 9;
