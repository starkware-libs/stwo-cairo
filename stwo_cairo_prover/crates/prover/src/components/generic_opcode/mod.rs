#![allow(clippy::too_many_arguments)]
pub mod component;
pub mod prover;

pub use component::{Claim, Component, Eval, InteractionClaim};
pub use prover::{ClaimGenerator, InputType, InteractionClaimGenerator};