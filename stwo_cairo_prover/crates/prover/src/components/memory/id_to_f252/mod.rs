use component::N_ID_TO_VALUE_COLUMNS;
use stwo_prover::constraint_framework::logup::LookupElements;

pub mod component;
pub mod prover;

const N_LOGUP_POWERS: usize = N_ID_TO_VALUE_COLUMNS - 1;
pub type IdToF252LookupElements = LookupElements<N_LOGUP_POWERS>;

pub const N_BITS_PER_FELT: usize = 9;
