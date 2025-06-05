#![feature(portable_simd)]
#![feature(array_chunks)]
use serde::{Deserialize, Serialize};

use crate::preprocessed::PreProcessedTrace;

pub mod air;
pub mod blake;
pub mod builtins_air;
pub mod components;
pub mod opcodes_air;
pub mod pedersen;
pub mod poseidon;
pub mod preprocessed;
pub mod preprocessed_utils;
pub mod range_checks_air;
pub mod relations;
pub mod utils;

// TODO(Ohad): verifier crate.
pub mod verifier;

pub use air::CairoProof;

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
