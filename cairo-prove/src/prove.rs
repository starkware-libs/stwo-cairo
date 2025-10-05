use cairo_air::{CairoProof, PreProcessedTraceVariant};
use stwo::core::pcs::PcsConfig;
use stwo::core::vcs::blake2_merkle::{Blake2sMerkleChannel, Blake2sMerkleHasher};
use stwo_cairo_adapter::ProverInput;

/// Deduces the preprocessed trace variant needed for the specific execution, and proves.
pub fn prove(input: ProverInput, pcs_config: PcsConfig) -> CairoProof<Blake2sMerkleHasher> {
    // Currently there are two variants of the preprocessed trace:
    // - Canonical: Pedersen is included in the program.
    // - CanonicalWithoutPedersen: Pedersen is not included in the program.
    // We deduce the variant based on weather the pedersen builtin is included in the program.
    let preprocessed_trace = match input.public_segment_context[1] {
        true => PreProcessedTraceVariant::Canonical,
        false => PreProcessedTraceVariant::CanonicalWithoutPedersen,
    };
    prove_inner(input, preprocessed_trace, pcs_config)
}

fn prove_inner(
    input: ProverInput,
    preprocessed_trace: PreProcessedTraceVariant,
    pcs_config: PcsConfig,
) -> CairoProof<Blake2sMerkleHasher> {
    stwo_cairo_prover::prover::prove_cairo::<Blake2sMerkleChannel>(
        input,
        pcs_config,
        preprocessed_trace,
    )
    .unwrap()
}
