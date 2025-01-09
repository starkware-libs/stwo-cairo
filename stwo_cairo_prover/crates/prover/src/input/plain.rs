use cairo_vm::hint_processor::builtin_hint_processor::builtin_hint_processor_definition::BuiltinHintProcessor;
use cairo_vm::stdlib::collections::HashMap;
use cairo_vm::types::layout_name::LayoutName;
use cairo_vm::types::relocatable::MaybeRelocatable;
use cairo_vm::vm::runners::cairo_runner::CairoRunner;
use itertools::Itertools;

use super::memory::{MemoryBuilder, MemoryConfig};
use super::vm_import::{adapt_to_stwo_input, MemoryEntry};
use super::ProverInput;

// TODO(Ohad): remove dev_mode after adding the rest of the opcodes.
/// Translates a plain casm into a ProverInput by running the program and extracting the memory and
/// the state transitions.
/// When dev mod is enabled, the opcodes generated from the plain casm will
/// be mapped to the generic component only.
pub fn input_from_plain_casm(
    casm: Vec<cairo_lang_casm::instructions::Instruction>,
    dev_mode: bool,
) -> ProverInput {
    let (program, program_len) = program_from_casm(casm);

    let mut runner = CairoRunner::new(&program, LayoutName::plain, None, true, true)
        .expect("Runner creation failed");
    runner.initialize(true).expect("Initialization failed");
    runner
        .run_until_pc(
            (runner.program_base.unwrap() + program_len).unwrap(),
            &mut BuiltinHintProcessor::new_empty(),
        )
        .expect("Run failed");
    runner.relocate(true).unwrap();
    adapt_finished_runner(runner, dev_mode)
}

// NOTE: the proof will include `step_limit -1` steps.
pub fn input_from_plain_casm_with_step_limit(
    casm: Vec<cairo_lang_casm::instructions::Instruction>,
    step_limit: usize,
) -> ProverInput {
    let (program, _) = program_from_casm(casm);

    let mut runner = CairoRunner::new(&program, LayoutName::plain, None, true, true)
        .expect("Runner creation failed");
    runner.initialize(true).expect("Initialization failed");
    runner
        .run_for_steps(step_limit, &mut BuiltinHintProcessor::new_empty())
        .expect("Run failed");
    runner.relocate(true).unwrap();

    adapt_finished_runner(runner, true)
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

// TODO(yuval): consider returning a result instead of panicking.
// TODO(Ohad): remove dev_mode after adding the rest of the opcodes.
/// Translates a CairoRunner that finished its run into a ProverInput by extracting the relevant
/// input to the adapter.
/// When dev mod is enabled, the opcodes generated from the plain casm will be mapped to the generic
/// component only.
/// # Panics
/// - if the memory or the trace are not relocated.
pub fn adapt_finished_runner(runner: CairoRunner, dev_mode: bool) -> ProverInput {
    let _span = tracing::info_span!("adapt_finished_runner").entered();
    let memory_iter = runner
        .relocated_memory
        .iter()
        .enumerate()
        .filter_map(|(i, v)| {
            v.map(|v| MemoryEntry {
                address: i as u64,
                value: bytemuck::cast(v.to_bytes_le()),
            })
        });

    let public_input = runner
        .get_air_public_input()
        .expect("Unable to get public input from the runner");

    let trace_iter = match runner.relocated_trace {
        Some(ref trace) => trace.iter().map(|t| t.clone().into()),
        None => panic!("Trace is not relocated"),
    };

    let memory_segments = &public_input.memory_segments;

    let public_memory_addresses = public_input
        .public_memory
        .iter()
        .map(|s| s.address as u32)
        .collect_vec();

    // TODO(spapini): Add output builtin to public memory.
    adapt_to_stwo_input(
        trace_iter,
        MemoryBuilder::from_iter(MemoryConfig::default(), memory_iter),
        public_memory_addresses,
        memory_segments,
        dev_mode,
    )
    .unwrap()
}
