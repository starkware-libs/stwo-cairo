use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;
use stwo_cairo_adapter::plain::adapt_finished_runner;
use stwo_cairo_adapter::vm_import::VmImportError;
use stwo_cairo_adapter::{ExecutionResources, ProverInput};
use stwo_cairo_utils::binary_utils::run_binary;
use stwo_cairo_utils::vm_utils::{run_vm, VmArgs, VmError};
use thiserror::Error;
use tracing::{span, Level};

/// Command line arguments for stwo_vm_runner.
/// Example command line (use absolute paths):
///     ```
///     cargo run -r --bin stwo_vm_runner -- --run_from_cairo_pie
///     --output_path path/to/output --secure_run=true path/to/cairo/pie
///     ```
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[command(flatten)]
    vm_args: VmArgs,
    /// The file path for the output (the adapted execution resources of the VM run).
    #[structopt(long = "output_path")]
    output_path: PathBuf,
}

#[derive(Debug, Error)]
enum Error {
    #[error("Invalid arguments")]
    Cli(#[from] clap::Error),
    #[error("Failed to interact with the file system")]
    IO(#[from] std::io::Error),
    #[error("VM run failed: {0}")]
    Vm(#[from] VmError),
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

    // Usually vm_runner runs the VM in non-proof-mode, in which case we don't need
    // `disable_trace_padding`. If it runs the VM in proof-mode, it should also disable trace
    // padding as this is the padding relevant for Stwo.
    let disable_trace_padding = args.vm_args.proof_mode;
    let cairo_runner = run_vm(&args.vm_args, disable_trace_padding)?;
    let cairo_input = adapt_finished_runner(cairo_runner)?;

    let execution_resources = ExecutionResources::from_prover_input(&cairo_input);
    log::info!("Execution resources: {:#?}", execution_resources);
    std::fs::write(
        args.output_path,
        serde_json::to_string(&execution_resources)?,
    )?;

    Ok(cairo_input)
}
