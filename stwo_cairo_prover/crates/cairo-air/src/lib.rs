#![feature(portable_simd)]
#![feature(array_chunks)]
use serde::{Deserialize, Serialize};

pub mod air;
pub mod blake;
pub mod builtins_air;
pub mod components;
pub mod opcodes_air;
pub mod pedersen;
pub mod poseidon;
pub mod range_checks_air;
pub mod relations;
pub mod utils;

// TODO(Ohad): verifier crate.
pub mod verifier;

pub use air::CairoProof;
use stwo_cairo_common::preprocessed_columns::preprocessed_trace::PreProcessedTrace;

/// The preprocessed trace used for the prover.
// TODO(Ohad): move somewhere else.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreProcessedTraceVariant {
    Canonical,
    CanonicalWithoutPedersen,
}
impl PreProcessedTraceVariant {
    pub fn to_preprocessed_trace(&self) -> PreProcessedTrace {
        match self {
            PreProcessedTraceVariant::Canonical => PreProcessedTrace::canonical(),
            PreProcessedTraceVariant::CanonicalWithoutPedersen => {
                PreProcessedTrace::canonical_without_pedersen()
            }
        }
    }
}
