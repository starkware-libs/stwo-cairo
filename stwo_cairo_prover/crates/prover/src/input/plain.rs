use std::collections::HashMap;

use cairo_vm::hint_processor::builtin_hint_processor::builtin_hint_processor_definition::BuiltinHintProcessor;
use cairo_vm::types::layout_name::LayoutName;
use cairo_vm::types::relocatable::MaybeRelocatable;
use cairo_vm::vm::runners::cairo_runner::CairoRunner;
use itertools::Itertools;

use super::memory::{MemoryBuilder, MemoryConfig};
use super::vm_import::{adapt_to_stwo_input, MemoryEntry};
use super::CairoInput;

// TODO(Ohad): remove dev_mode after adding the rest of the opcodes.
/// Translates a plain casm into a CairoInput by running the program and extracting the memory and
/// the state transitions.
/// When dev mod is enabled, the opcodes generated from the plain casm will
/// be mapped to the generic component only.
pub fn input_from_plain_casm(
    casm: Vec<cairo_lang_casm::instructions::Instruction>,
    dev_mode: bool,
) -> CairoInput {
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

// TODO(yuval): consider returning a result instead of panicking.
// TODO(Ohad): remove dev_mode after adding the rest of the opcodes.
/// Translates a CairoRunner that finished its run into a CairoInput by extracting the relevant
/// input to the adapter. Assumes memory and trace are already relocated. Otherwise panics.
/// When dev mod is enabled, the opcodes generated from the plain casm will be mapped to the generic
/// component only.
pub fn adapt_finished_runner(runner: CairoRunner, dev_mode: bool) -> CairoInput {
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

    let trace = runner.relocated_trace.clone().unwrap();
    let trace_iter = trace.iter().map(|t| t.clone().into());

    let public_input = runner
        .get_air_public_input()
        .expect("Unable to get public input from the runner");

    let memory_segments = &public_input.memory_segments;

    let public_memory_addresses = public_input
        .public_memory
        .into_iter()
        .map(|s| s.address as u32)
        .collect::<Vec<u32>>();

    // TODO(spapini): Add output builtin to public memory.
    let cairo_input = adapt_to_stwo_input(
        trace_iter,
        MemoryBuilder::from_iter(MemoryConfig::default(), memory_iter),
        public_memory_addresses,
        memory_segments,
        dev_mode,
    );
    cairo_input.unwrap()
}
