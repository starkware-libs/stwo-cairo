pub mod component;
pub mod prover;

pub const N_BITS_PER_FELT: usize = 9;

pub use component::{BigComponent, BigEval, Claim, InteractionClaim, SmallComponent, SmallEval};
pub use prover::{ClaimGenerator, InputType, InteractionClaimGenerator};
