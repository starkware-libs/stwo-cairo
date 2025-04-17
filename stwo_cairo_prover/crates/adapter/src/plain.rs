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
    adapt_finished_runner(runner, true).expect("Failed to adapt finished runner")
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

    adapt_finished_runner(runner, true).expect("Failed to adapt finished runner")
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
/// Dissable soundness checks for testing purposes.
pub fn adapt_finished_runner(
    runner: CairoRunner,
    dissable_verifier_soundness_checks: bool,
) -> Result<ProverInput, VmImportError> {
    let _span = tracing::info_span!("adapt_finished_runner").entered();

    let mut prover_input_info = runner
        .get_prover_input_info()
        .expect("Unable to get prover input info");

    prover_input_info_to_prover_input(&mut prover_input_info, dissable_verifier_soundness_checks)
}

// Dissable soundness checks for testing purposes.
pub fn prover_input_info_to_prover_input(
    prover_input_info: &mut ProverInputInfo,
    dissable_verifier_soundness_checks: bool,
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

    if !dissable_verifier_soundness_checks {
        // Soundness checks that will be verified by the verifier.
        assert_eq!(state_transitions.initial_state.pc.0, 1);
        assert!(
            state_transitions.initial_state.pc.0 < state_transitions.initial_state.ap.0 - 2,
            "Initial pc must be less than initial ap - 2, but got initial_pc: {}, initial_ap: {}",
            state_transitions.initial_state.pc.0,
            state_transitions.initial_state.ap.0 - 2
        );
        assert_eq!(
            state_transitions.initial_state.fp.0,
            state_transitions.final_state.fp.0
        );
        assert_eq!(
            state_transitions.initial_state.fp.0,
            state_transitions.initial_state.ap.0
        );
        assert_eq!(state_transitions.final_state.pc.0, 5);
        assert!(state_transitions.initial_state.ap.0 <= state_transitions.final_state.ap.0);
        assert!(
            state_transitions.final_state.ap.0 < 1 << 31,
            "final_ap must be less than 2^31, but got {}",
            state_transitions.final_state.ap.0
        );
    }

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

pub fn prover_input_from_vm_output(
    prover_input_info_path: &Path,
) -> Result<ProverInput, VmImportError> {
    let _span: span::EnteredSpan =
        span!(Level::INFO, "adapt_prover_input_info_vm_output").entered();
    let prover_input_info_json = read_to_string(prover_input_info_path).unwrap_or_else(|_| {
        panic!(
            "Unable to read prover input info at path {}",
            prover_input_info_path.display()
        )
    });
    let mut prover_input_info: ProverInputInfo =
        serde_json::from_str(&prover_input_info_json).unwrap();

    prover_input_info_to_prover_input(&mut prover_input_info, false)
}
