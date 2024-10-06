use std::collections::HashMap;

use cairo_vm::hint_processor::builtin_hint_processor::builtin_hint_processor_definition::BuiltinHintProcessor;
use cairo_vm::types::layout_name::LayoutName;
use cairo_vm::types::relocatable::MaybeRelocatable;
use cairo_vm::vm::runners::cairo_runner::CairoRunner;
use itertools::Itertools;

use super::instructions::Instructions;
use super::mem::{MemConfig, MemoryBuilder};
use super::range_check_unit::RangeCheckUnitInput;
use super::vm_import::MemEntry;
use super::{CairoInput, SegmentAddrs};

pub fn input_from_plain_casm(casm: Vec<cairo_lang_casm::instructions::Instruction>) -> CairoInput {
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
    input_from_finished_runner(runner)
}

// TODO(yuval): consider returning a result instead of panicking...
/// Assumes memory and trace are already relocated. Otherwise panics.
pub fn input_from_finished_runner(runner: CairoRunner) -> CairoInput {
    let program_len = runner.get_program().iter_data().count();
    let mem = runner
        .relocated_memory
        .iter()
        .enumerate()
        .filter_map(|(i, v)| {
            v.map(|v| MemEntry {
                addr: i as u64,
                val: bytemuck::cast_slice(&v.to_bytes_le()).try_into().unwrap(),
            })
        });
    let trace = runner.relocated_trace.unwrap();
    let trace = trace.iter().map(|t| t.clone().into());

    let mut range_check9 = RangeCheckUnitInput::new();
    let mem_config = MemConfig::default();
    let mem = MemoryBuilder::from_iter(mem_config, &mut range_check9, mem);
    let instructions = Instructions::from_iter(trace, &mem);

    // TODO(spapini): Add output builtin to public memory.
    let public_mem_addresses = (0..(program_len as u32)).collect_vec();
    CairoInput {
        instructions,
        mem,
        public_mem_addresses,
        range_check9,
        range_check_builtin: SegmentAddrs {
            begin_addr: 24,
            end_addr: 64,
        },
    }
}
