use component::{MEMORY_ID_SIZE, N_M31_IN_FELT252};
use stwo_prover::constraint_framework::logup::LookupElements;

pub mod component;
pub mod prover;

const N_LOGUP_POWERS: usize = MEMORY_ID_SIZE + N_M31_IN_FELT252;
pub type IdToF252LookupElements = LookupElements<N_LOGUP_POWERS>;

pub const N_ADDRESS_FELTS: usize = 1;
pub const N_BITS_PER_FELT: usize = 9;
