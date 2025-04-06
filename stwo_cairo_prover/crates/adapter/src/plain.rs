use std::fs::read_to_string;
use std::path::Path;

use cairo_vm::hint_processor::builtin_hint_processor::builtin_hint_processor_definition::BuiltinHintProcessor;
use cairo_vm::stdlib::collections::HashMap;
use cairo_vm::types::layout_name::LayoutName;
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

/// Translates a plain casm into a ProverInput by running the program and extracting the memory and
/// the state transitions.
/// When dev mod is enabled, the opcodes generated from the plain casm will
/// be mapped to the generic component only.
pub fn input_from_plain_casm(casm: Vec<cairo_lang_casm::instructions::Instruction>) -> ProverInput {
    let (program, program_len) = program_from_casm(casm);

    let mut runner = CairoRunner::new(&program, LayoutName::plain, None, true, true, true)
        .expect("Runner creation failed");
    runner.initialize(true).expect("Initialization failed");
    runner
        .run_until_pc(
            (runner.program_base.unwrap() + program_len).unwrap(),
            &mut BuiltinHintProcessor::new_empty(),
        )
        .expect("Run failed");
    runner.relocate(true).unwrap();
    adapt_finished_runner(runner).expect("Failed to adapt finished runner")
}

// NOTE: the proof will include `step_limit -1` steps.
pub fn input_from_plain_casm_with_step_limit(
    casm: Vec<cairo_lang_casm::instructions::Instruction>,
    step_limit: usize,
) -> ProverInput {
    let (program, _) = program_from_casm(casm);

    let mut runner = CairoRunner::new(&program, LayoutName::plain, None, true, true, true)
        .expect("Runner creation failed");
    runner.initialize(true).expect("Initialization failed");
    runner
        .run_for_steps(step_limit, &mut BuiltinHintProcessor::new_empty())
        .expect("Run failed");
    runner.relocate(true).unwrap();

    adapt_finished_runner(runner).expect("Failed to adapt finished runner")
}

fn program_from_casm(
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

    let (state_transitions, instruction_by_pc) = StateTransitions::from_iter(
        relocator
            .relocate_trace(&prover_input_info.relocatable_trace)
            .into_iter(),
        &mut memory,
    );

    let builtins_segments = relocator.get_builtin_segments();

    // TODO(spapini): Add output builtin to public memory.
    Ok(ProverInput {
        state_transitions,
        instruction_by_pc,
        memory: memory.build(),
        public_memory_addresses: relocator
            .relocate_public_addresses(prover_input_info.public_memory_offsets.clone()),
        builtins_segments,
    })
}

pub fn prover_input_from_vm_output(prover_input_info: &Path) -> Result<ProverInput, VmImportError> {
    let _span: span::EnteredSpan =
        span!(Level::INFO, "adapt_prover_input_info_vm_output").entered();
    let prover_input_info_json = read_to_string(prover_input_info).unwrap_or_else(|_| {
        panic!(
            "Unable to read prover input info at path {}",
            prover_input_info.display()
        )
    });
    let mut prover_input_info: ProverInputInfo =
        serde_json::from_str(&prover_input_info_json).unwrap();

    prover_input_info_to_prover_input(&mut prover_input_info)
}
