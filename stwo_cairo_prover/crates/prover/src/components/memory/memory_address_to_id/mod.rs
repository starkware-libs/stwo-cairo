pub mod component;
pub mod prover;

pub const N_ADDRESS_FELTS: usize = 1;

pub use component::{Claim, Component, Eval, InteractionClaim};
pub use prover::{ClaimGenerator, InputType, InteractionClaimGenerator, PackedInputType};
