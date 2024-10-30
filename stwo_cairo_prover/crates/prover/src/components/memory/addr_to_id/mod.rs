pub mod component;
pub mod prover;

pub const N_ADDRESS_FELTS: usize = 1;

pub use component::{Claim, Component, Eval, InteractionClaim, RelationElements};
pub use prover::ClaimGenerator;
