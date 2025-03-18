pub mod component;
pub mod prover;

pub use component::{BigComponent, BigEval, Claim, InteractionClaim, SmallComponent, SmallEval};
pub use prover::{ClaimGenerator, InteractionClaimGenerator};
