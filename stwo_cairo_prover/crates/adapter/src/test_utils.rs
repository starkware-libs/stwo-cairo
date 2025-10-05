use std::path::PathBuf;

use cairo_vm::stdlib::collections::HashMap;
use cairo_vm::types::program::Program;
use cairo_vm::types::relocatable::MaybeRelocatable;
use itertools::Itertools;

pub fn program_from_casm(
    casm: Vec<cairo_lang_casm::instructions::Instruction>,
) -> (cairo_vm::types::program::Program, usize) {
    let felt_code = casm
        .into_iter()
        .flat_map(|instruction| instruction.assemble().encode())
        .map(|felt| MaybeRelocatable::Int(felt.into()))
        .collect_vec();
    let program_len = felt_code.len();
    let program = Program::new_for_proof(
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

pub fn get_prover_input_path(test_name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../test_data/")
        .join(test_name)
        .join("prover_input.json")
}

#[macro_export]
macro_rules! casm_state {
    ($val1 : expr, $val2 : expr, $val3: expr) => {
        CasmState {
            pc: M31($val1),
            ap: M31($val2),
            fp: M31($val3),
        }
    };
}

#[macro_export]
macro_rules! relocated_trace_entry {
    ($val1 : expr, $val2 : expr, $val3: expr) => {
        RelocatedTraceEntry {
            ap: $val1,
            fp: $val2,
            pc: $val3,
        }
    };
}
