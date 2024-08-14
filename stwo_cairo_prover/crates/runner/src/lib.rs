use std::collections::HashMap;

use cairo_vm::hint_processor::builtin_hint_processor::builtin_hint_processor_definition::BuiltinHintProcessor;
use cairo_vm::types::layout_name::LayoutName;
use cairo_vm::types::relocatable::MaybeRelocatable;
use cairo_vm::vm::runners::cairo_runner::CairoRunner;
use itertools::Itertools;
use stwo_cairo_prover::input::instructions::Instructions;
use stwo_cairo_prover::input::mem::{MemConfig, Memory};
use stwo_cairo_prover::input::vm_import::{MemEntry, TraceEntry};
use stwo_cairo_prover::input::CairoInput;

pub fn run_casm(casm: Vec<cairo_lang_casm::instructions::Instruction>) -> CairoInput {
    let felt_code = casm
        .into_iter()
        .flat_map(|i| i.assemble().encode())
        .map(|i| MaybeRelocatable::Int(i.into()))
        .collect_vec();

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

    let mut runner =
        CairoRunner::new(&program, LayoutName::plain, true, true).expect("Runner creation failed");
    runner.initialize(true).expect("Initialization failed");
    runner
        .run_for_steps(1 << 8, &mut BuiltinHintProcessor::new_empty())
        .expect("Run failed");
    runner.relocate(true).expect("Relocation failed");

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
    let trace = trace.iter().map(|t| TraceEntry {
        pc: t.pc as u64,
        ap: t.ap as u64,
        fp: t.fp as u64,
    });

    let mem_config = MemConfig::default();
    let mem = Memory::from_iter(mem_config, mem);
    let instructions = Instructions::from_iter(trace, &mem);
    CairoInput { instructions, mem }
}
