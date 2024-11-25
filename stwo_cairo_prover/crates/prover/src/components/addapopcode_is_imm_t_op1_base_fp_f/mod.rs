pub mod component;
pub mod prover;

pub use component::{Claim, InteractionClaim, RelationElements, Component, Eval};
pub use prover::{ClaimGenerator, InputType, InteractionClaimGenerator};
