use std::collections::HashMap;

use cairo_lang_casm::hints::Hint;
use cairo_lang_executable::executable::{EntryPointKind, Executable};
use cairo_lang_runner::{Arg, CairoHintProcessor, build_hints_dict};
use cairo_vm::Felt252;
use cairo_vm::cairo_run::{CairoRunConfig, cairo_run_program};
use cairo_vm::types::layout_name::LayoutName;
use cairo_vm::types::program::Program;
use cairo_vm::types::relocatable::MaybeRelocatable;
use cairo_vm::vm::runners::cairo_runner::CairoRunner;
use log::info;

pub fn execute(target_path: &str, args: Vec<Arg>) -> CairoRunner {
    let executable: Executable =
        serde_json::from_reader(std::fs::File::open(target_path).unwrap()).unwrap();

    let (program, string_to_hint) = program_and_hints_from_executable(&executable);

    let mut hint_processor = CairoHintProcessor {
        runner: None,
        user_args: vec![vec![Arg::Array(args)]],
        string_to_hint,
        starknet_state: Default::default(),
        run_resources: Default::default(),
        syscalls_used_resources: Default::default(),
        no_temporary_segments: false,
        markers: Default::default(),
        panic_traceback: Default::default(),
    };

    let cairo_run_config = CairoRunConfig {
        trace_enabled: true,
        relocate_mem: true,
        layout: LayoutName::all_cairo_stwo,
        secure_run: None,
        allow_missing_builtins: None,
        dynamic_layout_params: None,
        disable_trace_padding: true,
        proof_mode: true,
        ..Default::default()
    };

    info!("Executing program...");
    let runner = cairo_run_program(&program, &cairo_run_config, &mut hint_processor).unwrap();
    info!("Program executed successfully.");
    runner
}

// TODO(Ohad): use cairo_lang_utils::program_and_hints_from_executable after updating `Cairo`.
fn program_and_hints_from_executable(executable: &Executable) -> (Program, HashMap<String, Hint>) {
    let data: Vec<MaybeRelocatable> = executable
        .program
        .bytecode
        .iter()
        .map(Felt252::from)
        .map(MaybeRelocatable::from)
        .collect();
    let (hints, string_to_hint) = build_hints_dict(&executable.program.hints);
    let entrypoint = executable
        .entrypoints
        .iter()
        .find(|e| matches!(e.kind, EntryPointKind::Standalone))
        .unwrap();
    let program = Program::new_for_proof(
        entrypoint.builtins.clone(),
        data,
        entrypoint.offset,
        entrypoint.offset + 4,
        hints,
        Default::default(),
        Default::default(),
        vec![],
        None,
    )
    .unwrap();
    (program, string_to_hint)
}
