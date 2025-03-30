pub mod component;

// TODO(Ohad): generalize, test.
pub use component::{Claim, Component, Eval, InteractionClaim};

pub const ELEM_BITS: u32 = 12;
pub const EXPAND_BITS: u32 = 2;
pub const LIMB_BITS: u32 = ELEM_BITS - EXPAND_BITS;
pub const LOG_SIZE: u32 = (ELEM_BITS - EXPAND_BITS) * 2;
pub const N_MULT_COLUMNS: usize = 1 << (EXPAND_BITS * 2);
pub const N_TRACE_COLUMNS: usize = N_MULT_COLUMNS;
