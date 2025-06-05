use std::path::Path;

use cairo_vm::vm::runners::cairo_runner::{CairoRunner, ProverInputInfo};
use tracing::{span, Level};

use super::memory::{MemoryBuilder, MemoryConfig};
use super::vm_import::VmImportError;
use super::ProverInput;
use crate::builtins::BuiltinSegments;
use crate::test_utils::read_prover_input_info_file;
use crate::StateTransitions;

/// Translates a CairoRunner that finished its run into a ProverInput by extracting the relevant
/// input to the adapter.
/// When dev mod is enabled, the opcodes generated from the plain casm will be mapped to the generic
/// component only.
pub fn adapt_finished_runner(runner: CairoRunner) -> Result<ProverInput, VmImportError> {
    let _span = tracing::info_span!("adapt_finished_runner").entered();

    let mut prover_input_info = runner
        .get_prover_input_info()
        .expect("Unable to get prover input info");

    adapter(&mut prover_input_info)
}

pub fn adapter(prover_input_info: &mut ProverInputInfo) -> Result<ProverInput, VmImportError> {
    BuiltinSegments::pad_relocatable_builtin_segments(
        &mut prover_input_info.relocatable_memory,
        prover_input_info.builtins_segments.clone(),
    );

    let memory = MemoryBuilder::from_relocatable_memory(
        MemoryConfig::default(),
        &prover_input_info.relocatable_memory.clone(),
    );
    let state_transitions =
        StateTransitions::from_relocatables(&prover_input_info.relocatable_trace, &memory);

    let builtins_segments = BuiltinSegments::get_builtin_segments(
        &prover_input_info.builtins_segments,
        &prover_input_info.relocatable_memory,
    );

    let public_memory_addresses = prover_input_info
        .public_memory_offsets
        .iter()
        .flat_map(|(segment_idx, offsets_in_segment)| {
            offsets_in_segment.iter().map(move |offset_val| {
                stwo_cairo_common::prover_types::cpu::Relocatable {
                    segment_index: *segment_idx,
                    offset: *offset_val as u32,
                }
            })
        })
        .collect();

    // TODO(spapini): Add output builtin to public memory.
    let (memory, inst_cache) = memory.build();
    Ok(ProverInput {
        state_transitions,
        memory,
        inst_cache,
        public_memory_addresses,
        builtins_segments,
    })
}

pub fn read_and_adapt_prover_input_info_file(
    prover_input_info_path: &Path,
) -> Result<ProverInput, VmImportError> {
    let _span: span::EnteredSpan = span!(Level::INFO, "adapter").entered();

    adapter(&mut read_prover_input_info_file(prover_input_info_path))
}
