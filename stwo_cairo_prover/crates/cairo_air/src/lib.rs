#![feature(portable_simd, trait_upcasting)]
// TODO(Ohad): remove.
#![allow(clippy::too_many_arguments)]

mod air;
mod builtins_air;
pub(crate) mod debug_tools;
pub mod opcodes_air;
pub mod prover;
pub mod range_checks_air;
// TODO(Ohad): verifier crate.
mod blake_air;
mod poseidon_air;
pub mod verifier;

pub use air::CairoProof;
