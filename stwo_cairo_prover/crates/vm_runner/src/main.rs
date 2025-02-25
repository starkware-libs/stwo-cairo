use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;
use stwo_cairo_adapter::plain::adapt_finished_runner;
use stwo_cairo_adapter::vm_import::VmImportError;
use stwo_cairo_adapter::{ExecutionResources, ProverInput};
use stwo_cairo_utils::binary_utils::run_binary;
use stwo_cairo_utils::program_runner_utils::{
    run_cairo_program, ProgramRunnerArgs, ProgramRunnerError,
};
use thiserror::Error;
use tracing::{span, Level};

/// Command line arguments for stwo_vm_runner.
/// Example command line (use absolute paths):
///     ```
///     cargo run -r --bin stwo_vm_runner -- --program path/to/program/file --program_input
///     path/to/input/file --output_er_path path/to/adapted_er/output/file
///     ```
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[command(flatten)]
    program_runner_args: ProgramRunnerArgs,
    /// The file path for the output (the adapted execution resources of the VM run).
    #[structopt(long = "output_er_path")]
    output_er_path: PathBuf,
}

#[derive(Debug, Error)]
enum Error {
    #[error("Invalid arguments: {0}")]
    Cli(#[from] clap::Error),
    #[error("IO failed: {0}")]
    IO(#[from] std::io::Error),
    #[error("program-runner run failed: {0}")]
    ProgramRunner(#[from] ProgramRunnerError),
    #[error("Serialization failed: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("VM import failed: {0}")]
    VmImport(#[from] VmImportError),
}

fn main() -> ExitCode {
    run_binary(run, "stwo_vm_runner")
}

fn run(args: impl Iterator<Item = String>) -> Result<ProverInput, Error> {
    let _span = span!(Level::INFO, "run").entered();
    let args = Args::try_parse_from(args)?;

    let cairo_runner = run_cairo_program(&args.program_runner_args)?;
    let cairo_input = adapt_finished_runner(cairo_runner)?;

    let execution_resources = ExecutionResources::from_prover_input(&cairo_input);
    log::info!("Execution resources: {:#?}", execution_resources);
    std::fs::write(
        args.output_er_path,
        serde_json::to_string(&execution_resources)?,
    )?;

    Ok(cairo_input)
}
