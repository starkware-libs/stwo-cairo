use cairo_vm::cairo_run::{cairo_run_program_with_initial_scope, CairoRunConfig};
use cairo_vm::hint_processor::builtin_hint_processor::builtin_hint_processor_definition::BuiltinHintProcessor;
use cairo_vm::hint_processor::hint_processor_definition::HintProcessor;
use cairo_vm::types::exec_scope::ExecutionScopes;
use cairo_vm::types::layout_name::LayoutName;
use cairo_vm::types::program::Program;

use crate::adapter::adapter;
use crate::ProverInput;

pub fn run_program_and_adapter(
    program: &Program,
    hint_processor: Option<&mut dyn HintProcessor>,
    program_input_contents: Option<String>,
) -> ProverInput {
    let cairo_run_config = CairoRunConfig {
        trace_enabled: true,
        relocate_trace: false,
        layout: LayoutName::all_cairo_stwo,
        proof_mode: true,
        disable_trace_padding: true,
        ..Default::default()
    };

    let mut exec_scopes = ExecutionScopes::new();

    if let Some(program_input_contents) = program_input_contents {
        // Insert the program input into the execution scopes if exists
        exec_scopes.insert_value("program_input", program_input_contents);
    }
    // Insert the program object into the execution scopes
    exec_scopes.insert_value("program_object", program.clone());

    let mut default_hint_processor = BuiltinHintProcessor::new_empty();
    let hint_processor = hint_processor.unwrap_or(&mut default_hint_processor);

    let runner = cairo_run_program_with_initial_scope(
        program,
        &cairo_run_config,
        hint_processor,
        exec_scopes,
    )
    .expect("Failed to run cairo program");
    adapter(&runner)
}
