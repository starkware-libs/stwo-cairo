use std::collections::HashMap;

use cairo_air::{CairoProof, PreProcessedTraceVariant};
use cairo_vm::vm::runners::cairo_runner::CairoRunner;
use log::{debug, info};
use stwo_cairo_adapter::builtins::MemorySegmentAddresses;
use stwo_cairo_adapter::memory::{MemoryBuilder, MemoryConfig, MemoryEntry};
use stwo_cairo_adapter::vm_import::{RelocatedTraceEntry, adapt_to_stwo_input};
use stwo_cairo_adapter::{ProverInput, PublicSegmentContext};
use stwo_cairo_prover::stwo_prover::core::pcs::PcsConfig;
use stwo_cairo_prover::stwo_prover::core::vcs::blake2_merkle::{
    Blake2sMerkleChannel, Blake2sMerkleHasher,
};

pub fn prover_input_from_runner(runner: &CairoRunner) -> ProverInput {
    let public_input = runner.get_air_public_input().unwrap();
    let addresses = public_input
        .public_memory
        .iter()
        .map(|entry| entry.address as u32)
        .collect::<Vec<_>>();
    let segments = public_input
        .memory_segments
        .iter()
        .map(|(&k, v)| {
            (
                k,
                MemorySegmentAddresses {
                    begin_addr: v.begin_addr,
                    stop_ptr: v.stop_ptr,
                },
            )
        })
        .collect::<HashMap<_, _>>();
    let trace = runner
        .relocated_trace
        .as_ref()
        .unwrap()
        .iter()
        .map(|x| RelocatedTraceEntry {
            ap: x.ap,
            fp: x.fp,
            pc: x.pc,
        })
        .collect::<Vec<_>>();
    let mem = runner
        .relocated_memory
        .iter()
        .enumerate()
        .filter_map(|(i, x)| {
            x.as_ref().map(|value| MemoryEntry {
                address: i as u64,
                value: unsafe { std::mem::transmute::<[u8; 32], [u32; 8]>(value.to_bytes_le()) },
            })
        });
    let mem = MemoryBuilder::from_iter(MemoryConfig::default(), mem);
    let main_args = runner
        .get_program()
        .iter_builtins()
        .copied()
        .collect::<Vec<_>>();
    let public_segment_context = PublicSegmentContext::new(&main_args);

    info!("Generating input for the prover...");
    let input =
        adapt_to_stwo_input(&trace, mem, addresses, &segments, public_segment_context).unwrap();
    info!("Input for the prover generated successfully.");
    debug!(
        "State transitions: {}",
        input.state_transitions.casm_states_by_opcode
    );
    debug!("Builtins: {:#?}", input.builtins_segments.get_counts());
    input
}

pub fn prove(input: ProverInput, pcs_config: PcsConfig) -> CairoProof<Blake2sMerkleHasher> {
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
