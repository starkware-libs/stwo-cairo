use std::collections::{BTreeMap, HashMap};

use cairo_vm::air_public_input::MemorySegmentAddresses;
use cairo_vm::hint_processor::builtin_hint_processor::builtin_hint_processor_definition::BuiltinHintProcessor;
use cairo_vm::types::layout_name::LayoutName;
use cairo_vm::types::relocatable::MaybeRelocatable;
use cairo_vm::vm::runners::cairo_runner::CairoRunner;
use itertools::Itertools;

use super::vm_import::{adapter, MemEntry};
use super::CairoInput;
use crate::input::mem::{MemConfig, MemoryBuilder};

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
/// When dev mod is enabled, the opcodes generated from the plain casm will be mapped to the generic
/// component only.
/// Translates a CairoRunner into a CairoInput by extracting the relevant input to the adapter.
/// Assumes memory and trace are already relocated. Otherwise panics.
pub fn adapt_finished_runner(runner: CairoRunner, dev_mode: bool) -> CairoInput {
    let mem_iter = runner
        .relocated_memory
        .iter()
        .enumerate()
        .filter_map(|(i, v)| {
            v.map(|v| MemEntry {
                addr: i as u64,
                val: bytemuck::cast_slice(&v.to_bytes_le()).try_into().unwrap(),
            })
        });

    let trace = runner.relocated_trace.clone().unwrap();
    let trace_iter = trace.iter().map(|t| t.clone().into());

    let public_input = runner
        .get_air_public_input()
        .expect("Unable to get public input from the runner");
    let public_mem_addresses = public_input
        .public_memory
        .into_iter()
        .map(|s| s.address as u32)
        .collect::<Vec<u32>>();

    let memory_segments = public_input
        .memory_segments
        .into_iter()
        .map(|(s, m)| (s.to_string(), m))
        .collect::<BTreeMap<String, MemorySegmentAddresses>>();

    let cairo_input = adapter(
        trace_iter,
        MemoryBuilder::from_iter(MemConfig::default(), mem_iter),
        public_mem_addresses,
        &memory_segments,
        dev_mode,
    );
    match cairo_input {
        Ok(cairo_input) => cairo_input,
        Err(e) => panic!("Error: {e}"),
    }
}
