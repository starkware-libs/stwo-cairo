use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;
use stwo_cairo_prover::input::plain::input_from_finished_runner;
use stwo_cairo_prover::input::CairoInput;
use stwo_cairo_utils::binary_utils::run_binary;
use stwo_cairo_utils::vm_utils::{run_vm, VmArgs, VmError};
use thiserror_no_std::Error;
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
    Serde(#[from] serde_json::error::Error),
}

fn main() -> ExitCode {
    run_binary(run)
}

fn run(args: impl Iterator<Item = String>) -> Result<CairoInput, Error> {
    let _span = span!(Level::INFO, "run").entered();
    let args = Args::try_parse_from(args)?;
    let cairo_runner = run_vm(&args.vm_args)?;
    let cairo_input = input_from_finished_runner(cairo_runner, false);

    let execution_resources = &cairo_input.state_transitions.casm_states_by_opcode.counts();
    std::fs::write(
        args.output_path,
        serde_json::to_string(execution_resources)?,
    )?;

    Ok(cairo_input)
}
