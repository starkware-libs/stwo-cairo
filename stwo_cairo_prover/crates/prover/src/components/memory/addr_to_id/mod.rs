use stwo_prover::constraint_framework::logup::LookupElements;

pub mod component;
pub mod prover;

pub const N_ADDRESS_FELTS: usize = 1;
pub type AddrToIdLookupElements = LookupElements<2>;
