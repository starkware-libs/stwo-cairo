use std::fs::read_to_string;
use std::path::Path;

use cairo_vm::air_private_input::PrivateInputPair;
use cairo_vm::stdlib::collections::HashMap;
use cairo_vm::types::relocatable::MaybeRelocatable;
use cairo_vm::vm::runners::cairo_runner::{CairoRunner, ProverInputInfo};
use itertools::Itertools;
use tracing::{span, Level};

use super::memory::{MemoryBuilder, MemoryConfig};
use super::vm_import::VmImportError;
use super::ProverInput;
use crate::builtins::BuiltinSegments;
use crate::relocator::Relocator;
use crate::StateTransitions;

pub fn program_from_casm(
    casm: Vec<cairo_lang_casm::instructions::Instruction>,
) -> (cairo_vm::types::program::Program, usize) {
    let felt_code = casm
        .into_iter()
        .flat_map(|instruction| instruction.assemble().encode())
        .map(|felt| MaybeRelocatable::Int(felt.into()))
        .collect_vec();
    let program_len = felt_code.len();
    let program = cairo_vm::types::program::Program::new_for_proof(
        vec![],
        felt_code,
        0,
        1,
        HashMap::default(),
        Default::default(),
        Default::default(),
        Default::default(),
        Default::default(),
    )
    .expect("Program creation failed");
    (program, program_len)
}

/// Translates a CairoRunner that finished its run into a ProverInput by extracting the relevant
/// input to the adapter.
/// When dev mod is enabled, the opcodes generated from the plain casm will be mapped to the generic
/// component only.
pub fn adapt_finished_runner(runner: CairoRunner) -> Result<ProverInput, VmImportError> {
    let _span = tracing::info_span!("adapt_finished_runner").entered();

    let mut prover_input_info = runner
        .get_prover_input_info()
        .expect("Unable to get prover input info");

    prover_input_info_to_prover_input(&mut prover_input_info)
}

pub fn prover_input_info_to_prover_input(
    prover_input_info: &mut ProverInputInfo,
) -> Result<ProverInput, VmImportError> {
    BuiltinSegments::pad_relocatble_builtin_segments(
        &mut prover_input_info.relocatable_memory,
        prover_input_info.builtins_segments.clone(),
    );
    let relocator = Relocator::new(
        prover_input_info.relocatable_memory.clone(),
        prover_input_info.builtins_segments.clone(),
    );
    let mut memory =
        MemoryBuilder::from_iter(MemoryConfig::default(), relocator.get_relocated_memory());

    let state_transitions = StateTransitions::from_iter(
        relocator
            .relocate_trace(&prover_input_info.relocatable_trace)
            .into_iter(),
        &mut memory,
    );

    let builtins_segments = relocator.get_builtin_segments();

    // TODO(spapini): Add output builtin to public memory.
    let (memory, inst_cache) = memory.build();
    Ok(ProverInput {
        state_transitions,
        memory,
        inst_cache,
        public_memory_addresses: relocator
            .relocate_public_addresses(prover_input_info.public_memory_offsets.clone()),
        builtins_segments,
    })
}

use cairo_vm::vm::trace::trace_entry::TraceEntry;
use cairo_vm::types::relocatable::Relocatable;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

fn read_prover_input_info_file(prover_input_info_path: &Path) -> ProverInputInfo {
    let _span: span::EnteredSpan = span!(Level::INFO, "read_prover_input_info_file").entered();

    let file = File::open(prover_input_info_path).expect("Unable to open file");
    let mut reader = BufReader::new(file); 

    let mut buffer = [0u8; 8];
    let mut entries = vec![];

    while reader.read_exact(&mut buffer).is_ok() {
        reader.read_exact(&mut buffer).expect("");
        let ap = u64::from_le_bytes(buffer) as usize;

        reader.read_exact(&mut buffer).expect("");
        let fp = u64::from_le_bytes(buffer) as usize;

        reader.read_exact(&mut buffer).expect("");
        let segment_index = u64::from_le_bytes(buffer) as isize;

        reader.read_exact(&mut buffer).expect("");
        let offset = u64::from_le_bytes(buffer) as usize;

        entries.push(TraceEntry {
            ap,
            fp,
            pc: Relocatable {
                segment_index,
                offset,
            },
        });
    }

    let mut res = ProverInputInfo::Default();

    res.trace

    let prover_input_info_json = read_to_string(prover_input_info_path).unwrap_or_else(|_| {
        panic!(
            "Unable to read prover input info at path {}",
            prover_input_info_path.display()
        )
    });

    serde_json::from_str(&prover_input_info_json).unwrap()
}
pub fn prover_input_from_vm_output(
    prover_input_info_path: &Path,
) -> Result<ProverInput, VmImportError> {
    let _span: span::EnteredSpan = span!(Level::INFO, "adapter").entered();

    prover_input_info_to_prover_input(&mut read_prover_input_info_file(prover_input_info_path))
}
