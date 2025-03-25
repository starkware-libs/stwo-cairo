pub mod component;
pub mod prover;

// TODO(Ohad): generalize, test.
pub use component::{Claim, Component, Eval, InteractionClaim};
pub use prover::{ClaimGenerator, InteractionClaimGenerator, PackedInputType};

const ELEM_BITS: u32 = 12;
const EXPAND_BITS: u32 = 2;
const LIMB_BITS: u32 = ELEM_BITS - EXPAND_BITS;
const LOG_SIZE: u32 = (ELEM_BITS - EXPAND_BITS) * 2;
const N_MULT_COLUMNS: usize = 1 << (EXPAND_BITS * 2);
const N_TRACE_COLUMNS: usize = N_MULT_COLUMNS;
