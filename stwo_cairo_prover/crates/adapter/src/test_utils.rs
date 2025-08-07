use std::fs::{read, read_to_string, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use cairo_vm::stdlib::collections::HashMap;
use cairo_vm::types::relocatable::MaybeRelocatable;
use cairo_vm::vm::runners::cairo_runner::ProverInputInfo;
use itertools::Itertools;
use serde_json::{to_string_pretty, Value};
use tracing::{span, Level};

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

pub fn read_prover_input_info_file(prover_input_info_path: &Path) -> ProverInputInfo {
    let _span: span::EnteredSpan = span!(Level::INFO, "read_prover_input_info_file").entered();

    let bytes = read(prover_input_info_path).unwrap_or_else(|_| {
        panic!(
            "Unable to read prover input info at path {}",
            prover_input_info_path.display()
        )
    });
    let (prover_input_info, _): (ProverInputInfo, usize) =
        bincode::serde::decode_from_slice(&bytes, bincode::config::standard())
            .expect("Unable to decode prover input info");

    prover_input_info
}

pub fn get_test_program(test_name: &str) -> Vec<u8> {
    let program_path = get_compiled_cairo_program_path(test_name);
    read_compiled_cairo_program(&program_path)
}

pub fn read_compiled_cairo_program(program_path: &PathBuf) -> Vec<u8> {
    match std::fs::read(program_path) {
        Ok(program) => program,
        Err(e) => panic!("Failed to read program: {:?}", e),
    }
}

fn get_compiled_cairo_program_path(test_name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../test_data/")
        .join(test_name)
        .join("compiled.json")
}

pub fn get_prover_input_path(test_name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../test_data/")
        .join(test_name)
        .join("prover_input.json")
}

pub fn get_prover_input_info_path(test_name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../test_data/")
        .join(test_name)
        .join("prover_input_info")
}

pub fn read_json(file_path: &PathBuf) -> Value {
    let json_file = read_to_string(file_path).unwrap();
    serde_json::from_str(&json_file).expect("Invalid JSON file")
}

pub fn write_json(file_path: &PathBuf, value: &Value) {
    let mut file = File::create(file_path).unwrap();
    write!(file, "{}", &to_string_pretty(&value).unwrap()).unwrap();
}
