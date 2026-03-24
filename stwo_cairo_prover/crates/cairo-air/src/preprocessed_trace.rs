use serde::{Deserialize, Serialize};
use stwo_cairo_common::preprocessed_columns::preprocessed_trace::{
    PreProcessedTrace, CANONICAL_SIZE, CANONICAL_SMALL, CANONICAL_WITHOUT_PEDERSEN_SIZE,
};

/// The preprocessed trace used for the prover.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreProcessedTraceVariant {
    Canonical,
    CanonicalWithoutPedersen,
    CanonicalSmall,
}
impl PreProcessedTraceVariant {
    pub fn n_trace_cells(&self) -> u32 {
        match self {
            PreProcessedTraceVariant::Canonical => CANONICAL_SIZE,
            PreProcessedTraceVariant::CanonicalWithoutPedersen => CANONICAL_WITHOUT_PEDERSEN_SIZE,
            PreProcessedTraceVariant::CanonicalSmall => CANONICAL_SMALL,
        }
    }

    pub fn to_preprocessed_trace(&self) -> PreProcessedTrace {
        match self {
            PreProcessedTraceVariant::Canonical => PreProcessedTrace::canonical(),
            PreProcessedTraceVariant::CanonicalWithoutPedersen => {
                PreProcessedTrace::canonical_without_pedersen()
            }
            PreProcessedTraceVariant::CanonicalSmall => PreProcessedTrace::canonical_small(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_n_trace_cells() {
        for variant in [
            PreProcessedTraceVariant::Canonical,
            PreProcessedTraceVariant::CanonicalWithoutPedersen,
            PreProcessedTraceVariant::CanonicalSmall,
        ] {
            let trace = variant.to_preprocessed_trace();
            let actual_size: u32 = trace.columns.iter().map(|col| 1 << col.log_size()).sum();
            assert_eq!(actual_size, variant.n_trace_cells());
        }
    }
}
