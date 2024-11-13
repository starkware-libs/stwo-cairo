pub mod component;
pub mod prover;

pub const N_BITS_PER_FELT: usize = 9;
pub const N_LIMBS_IN_128_BIT_FELT: usize = 15;

pub use component::{
    BigComponent, BigEval, Claim, InteractionClaim, RelationElements, SmallComponent, SmallEval,
};
pub use prover::{ClaimGenerator, InputType, PackedInputType};
