use serde::{Deserialize, Serialize};

use crate::cairo_air::components::prelude::PreProcessedTrace;

pub(crate) mod air;
pub(crate) mod blake;
pub(crate) mod builtins_air;
pub mod components;
pub(crate) mod debug_tools;
pub(crate) mod opcodes_air;
pub(crate) mod pedersen;
pub(crate) mod poseidon;
pub(crate) mod preprocessed;
pub mod preprocessed_utils;
pub mod range_checks_air;
pub(crate) mod relations;

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
