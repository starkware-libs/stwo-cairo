use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use cairo_vm::cairo_run::{cairo_run, CairoRunConfig};
use cairo_vm::hint_processor::builtin_hint_processor::builtin_hint_processor_definition::*;
use cairo_vm::types::layout_name::LayoutName;
use cairo_vm::vm::runners::cairo_runner::CairoRunner;
use serde_json::{to_string_pretty, Value};
use stwo_cairo_adapter::plain::adapt_finished_runner;
use stwo_cairo_adapter::ProverInput;

pub fn runner_from_compiled_cairo_program(test_name: &str) -> CairoRunner {
    let file_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../test_data/")
        .join(test_name)
        .join("compiled.json");

    let cairo_run_config = CairoRunConfig {
        entrypoint: "main",
        trace_enabled: true,
        relocate_mem: false,
        layout: LayoutName::all_cairo_stwo,
        proof_mode: true,
        secure_run: None,
        allow_missing_builtins: None,
        dynamic_layout_params: None,
        disable_trace_padding: true,
    };

    let program = match std::fs::read(file_path) {
        Ok(program) => program,
        Err(e) => panic!("Failed to read program: {:?}", e),
    };
    cairo_run(
        &program,
        &cairo_run_config,
        &mut BuiltinHintProcessor::new_empty(),
    )
    .expect("Failed to run cairo program")
}

pub fn prover_input_from_compiled_cairo_program(file_name: &str) -> ProverInput {
    let runner = runner_from_compiled_cairo_program(file_name);
    adapt_finished_runner(runner).expect("Unable to create prover input from finished runner")
}

pub fn get_prover_input_path(test_name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../test_data/")
        .join(test_name)
        .join("prover_input.json")
}

pub fn read_json(file_path: &PathBuf) -> Value {
    let json_file = read_to_string(file_path).unwrap();
    serde_json::from_str(&json_file).expect("Invalid JSON file")
}

pub fn write_json(file_path: &PathBuf, value: &Value) {
    let mut file = File::create(file_path).unwrap();
    write!(file, "{}", &to_string_pretty(&value).unwrap()).unwrap();
}
