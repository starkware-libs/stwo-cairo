pub mod component;
pub mod prover;

pub const N_BITS_PER_FELT: usize = 9;

pub use component::{Claim, Component, Eval, InteractionClaim, RelationElements};
pub use prover::ClaimGenerator;
