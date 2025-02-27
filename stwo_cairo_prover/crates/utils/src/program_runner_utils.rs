use std::path::PathBuf;

use cairo_program_runner::cairo_run_program;
use cairo_program_runner::utils::{get_program, get_program_input};
use cairo_vm::cairo_run;
use cairo_vm::types::errors::program_errors::ProgramError;
use cairo_vm::types::layout_name::LayoutName;
use cairo_vm::vm::errors::cairo_run_errors::CairoRunError;
use cairo_vm::vm::runners::cairo_runner::CairoRunner;
use clap::Parser;
use thiserror::Error;
use tracing::span;

// This struct is copied-then-modified from bootloader-hints repo.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct ProgramRunnerArgs {
    #[clap(long = "program", help = "Path to the compiled program")]
    program: PathBuf,
    #[clap(long = "program_input", help = "Path to the program input file.")]
    program_input: Option<PathBuf>,
    #[clap(
        long = "cairo_pie_output",
        help = "Path to the Cairo PIE output file. Cannot be used in proof_mode.",
        conflicts_with_all = &["air_public_input", "air_private_input", "proof_mode"]
    )]
    cairo_pie_output: Option<PathBuf>,
    #[clap(long = "trace_file", help = "Path to the trace output file.")]
    trace_file: Option<PathBuf>,
    #[clap(long = "memory_file", help = "Path to the memory output file.")]
    memory_file: Option<PathBuf>,
}

#[derive(Debug, Error)]
pub enum ProgramRunnerError {
    #[error("Failed parsing the program: {0}")]
    Program(#[from] ProgramError),
    #[error("Failed executing the program: {0}")]
    Runner(#[from] CairoRunError),
    #[error("Failed to interact with the file system: {0}")]
    IO(#[from] std::io::Error),
}

/// Runs the cairo-program-runner according to the given arguments.
pub fn run_cairo_program(args: &ProgramRunnerArgs) -> Result<CairoRunner, ProgramRunnerError> {
    let _span = span!(tracing::Level::INFO, "run_vm").entered();

    let program = get_program(args.program.as_path()).map_err(ProgramRunnerError::Program)?;
    let program_input_contents = get_program_input(&args.program_input)?;
    let cairo_run_config = cairo_run::CairoRunConfig {
        entrypoint: "main",
        trace_enabled: true,
        relocate_mem: true,
        layout: LayoutName::all_cairo,
        proof_mode: true,
        secure_run: None,
        disable_trace_padding: true,
        allow_missing_builtins: None,
        dynamic_layout_params: None,
    };

    cairo_run_program(&program, program_input_contents, cairo_run_config).map_err(|error| {
        eprintln!("{error}");
        ProgramRunnerError::Runner(error)
    })
}
