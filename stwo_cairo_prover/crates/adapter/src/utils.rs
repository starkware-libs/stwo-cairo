use std::fs::{read, read_to_string, File};
use std::path::PathBuf;

use anyhow::{Context, Result};
use cairo_lang_executable::executable::{EntryPointKind, Executable};
use cairo_lang_runner::{build_hints_dict, Arg, CairoHintProcessor};
use cairo_lang_utils::bigint::BigUintAsHex;
use cairo_vm::cairo_run::{cairo_run_program_with_initial_scope, CairoRunConfig};
use cairo_vm::hint_processor::builtin_hint_processor::builtin_hint_processor_definition::BuiltinHintProcessor;
use cairo_vm::hint_processor::hint_processor_definition::HintProcessor;
use cairo_vm::types::exec_scope::ExecutionScopes;
use cairo_vm::types::layout_name::LayoutName;
use cairo_vm::types::program::Program;
use cairo_vm::types::relocatable::MaybeRelocatable;
use cairo_vm::vm::runners::cairo_runner::CairoRunner;
use cairo_vm::Felt252;
use clap::ValueEnum;
use serde_json::from_reader;

use crate::adapter::adapter;
use crate::ProverInput;

#[derive(Clone, Debug, ValueEnum)]
pub enum ProgramType {
    Executable,
    Json,
}

pub fn run_program_and_adapter(
    program_path: &PathBuf,
    program_type: ProgramType,
    args: Option<&PathBuf>,
) -> Result<ProverInput> {
    let runner = run_program(program_path, program_type, args)?;
    adapter(&runner)
}

fn run_program(
    program_path: &PathBuf,
    program_type: ProgramType,
    args_file: Option<&PathBuf>,
) -> Result<CairoRunner> {
    let cairo_run_config = CairoRunConfig {
        trace_enabled: true,
        relocate_trace: false,
        layout: LayoutName::all_cairo_stwo,
        proof_mode: true,
        disable_trace_padding: true,
        ..Default::default()
    };

    let (program, mut hints, exec_scopes) = match program_type {
        ProgramType::Executable => {
            let executable: Executable = from_reader(File::open(program_path)?)?;

            let args = if let Some(args_file) = args_file {
                let args_biguint: Vec<BigUintAsHex> = from_reader(File::open(args_file)?)?;
                args_biguint
                    .into_iter()
                    .map(|i| Arg::Value(i.value.into()))
                    .collect()
            } else {
                vec![]
            };

            let (program, hints) = get_program_and_hints_from_executable(&executable, args)?;

            (program, hints, ExecutionScopes::new())
        }
        ProgramType::Json => {
            let bytes = read(program_path)?;
            let program = Program::from_bytes(&bytes, Some("main"))?;

            let mut exec_scopes = ExecutionScopes::new();
            if let Some(args) = args_file {
                // Insert the program input into the execution scopes if exists
                exec_scopes.insert_value("program_input", read_to_string(args)?);
            }
            // Insert the program object into the execution scopes
            exec_scopes.insert_value("program_object", program.clone());

            (
                program,
                Box::new(BuiltinHintProcessor::new_empty()) as Box<dyn HintProcessor>,
                exec_scopes,
            )
        }
    };

    Ok(cairo_run_program_with_initial_scope(
        &program,
        &cairo_run_config,
        hints.as_mut(),
        exec_scopes,
    )?)
}

fn get_program_and_hints_from_executable(
    executable: &Executable,
    args: Vec<cairo_lang_runner::Arg>,
) -> Result<(Program, Box<dyn HintProcessor>)> {
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
        .with_context(|| "Failed to find entrypoint")?;
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
    )?;

    let hint_processor = CairoHintProcessor {
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

    Ok((program, Box::new(hint_processor)))
}
