// TODO(Gali): remove unused.
#![allow(unused)]
pub mod component;
pub mod prover;

pub use component::{Claim, Component, Eval, InteractionClaim};
pub use prover::{ClaimGenerator, InputType, InteractionClaimGenerator, PackedInputType};
