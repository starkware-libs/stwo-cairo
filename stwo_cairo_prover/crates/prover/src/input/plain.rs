use std::collections::HashMap;

use cairo_vm::hint_processor::builtin_hint_processor::builtin_hint_processor_definition::BuiltinHintProcessor;
use cairo_vm::types::layout_name::LayoutName;
use cairo_vm::types::relocatable::MaybeRelocatable;
use cairo_vm::vm::runners::cairo_runner::CairoRunner;
use itertools::Itertools;

use super::mem::{MemConfig, MemoryBuilder};
use super::state_transitions::StateTransitions;
use super::vm_import::MemEntry;
use super::{BuiltinSegments, CairoInput};

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
    input_from_finished_runner(runner, dev_mode)
}

// TODO(yuval): consider returning a result instead of panicking.
// TODO(Ohad): remove dev_mode after adding the rest of the opcodes.
/// Assumes memory and trace are already relocated. Otherwise panics.
/// When dev mod is enabled, the opcodes generated from the plain casm will be mapped to the generic
/// component only.
pub fn input_from_finished_runner(runner: CairoRunner, dev_mode: bool) -> CairoInput {
    let program_len = runner.get_program().iter_data().count();
    let mem = runner
        .relocated_memory
        .iter()
        .enumerate()
        .filter_map(|(i, v)| {
            v.map(|v| MemEntry {
                addr: i as u64,
                val: bytemuck::cast(v.to_bytes_le()),
            })
        });

    let memory_segments = &runner
        .get_air_public_input()
        .expect("Unable to get public input from the runner")
        .memory_segments;
    let builtins_segments = BuiltinSegments::from_memory_segments(memory_segments);

    let trace = runner.relocated_trace.unwrap();
    let trace = trace.iter().map(|t| t.clone().into());

    let mem_config = MemConfig::default();
    let mut mem = MemoryBuilder::from_iter(mem_config, mem);
    let state_transitions = StateTransitions::from_iter(trace, &mut mem, dev_mode);

    // TODO(spapini): Add output builtin to public memory.
    let public_mem_addresses = (0..(program_len as u32)).collect_vec();
    CairoInput {
        state_transitions,
        mem: mem.build(),
        public_mem_addresses,
        builtins_segments,
    }
}
