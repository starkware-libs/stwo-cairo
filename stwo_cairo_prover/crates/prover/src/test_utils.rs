use std::path::Path;

use cairo_vm::hint_processor::builtin_hint_processor::builtin_hint_processor_definition::BuiltinHintProcessor;
use cairo_vm::types::layout_name::LayoutName;
use cairo_vm::types::program::Program;
use cairo_vm::vm::runners::cairo_runner::CairoRunner;
use stwo_cairo_adapter::plain::adapt_finished_runner;
use stwo_cairo_adapter::ProverInput;

pub fn runner_from_compiled_cairo_program(test_name: &str) -> CairoRunner {
    let file_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../test_data/")
        .join(test_name)
        .join("compiled.json");

    let program = match Program::from_file(Path::new(&file_path), Some("main")) {
        Ok(program) => program,
        Err(e) => panic!("Failed to load program: {:?}", e),
    };

    // TODO(Stav): change to proof_mode = False and dissable trace padding = True after solving the
    // pa issue.
    let mut cairo_runner =
        CairoRunner::new(&program, LayoutName::all_cairo, None, true, true, true).expect("Fail");
    let end = cairo_runner
        .initialize(true)
        .expect("Initialization failed");
    cairo_runner
        .run_until_pc(end, &mut BuiltinHintProcessor::new_empty())
        .expect("Run failed");

    cairo_runner
        .end_run(true, false, &mut BuiltinHintProcessor::new_empty())
        .expect("fail");
    cairo_runner
        .read_return_values(true)
        .expect("Failed to read return values");
    cairo_runner
        .finalize_segments()
        .expect("Failed to finalize segments");

    cairo_runner
}

pub fn prover_input_from_compiled_cairo_program(file_name: &str) -> ProverInput {
    let runner = runner_from_compiled_cairo_program(file_name);
    adapt_finished_runner(runner).expect("Unable to create prover input from finished runner")
}
