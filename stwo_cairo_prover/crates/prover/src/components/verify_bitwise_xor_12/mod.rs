pub mod component;
pub mod prover;

// TODO(Ohad): expand bits.
pub use component::{Claim, Component, Eval, InteractionClaim};
pub use prover::{ClaimGenerator, InteractionClaimGenerator};
