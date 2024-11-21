use std::path::PathBuf;
use std::process::ExitCode;

use cairo_vm::cairo_run;
use cairo_vm::hint_processor::builtin_hint_processor::builtin_hint_processor_definition::BuiltinHintProcessor;
use cairo_vm::types::layout_name::LayoutName;
use cairo_vm::vm::errors::cairo_run_errors::CairoRunError;
use cairo_vm::vm::runners::cairo_pie::CairoPie;
use cairo_vm::vm::runners::cairo_runner::{CairoRunner, RunResources};
use clap::{Parser, ValueHint};
use stwo_cairo_prover::input::plain::input_from_finished_runner;
use stwo_cairo_prover::input::CairoInput;
use stwo_cairo_utils::logging_utils::init_logging;
use thiserror::Error;
use tracing::{span, Level};

// This struct is copied-then-modified from cairo-vm repo.
/// Command line arguments for stwo_vm_runner.
/// Example command line (use absolute paths):
///     ```
///     cargo run -r --bin stwo_vm_runner -- --run_from_cairo_pie
///     --output_path path/to/output --secure_run=true path/to/cairo/pie
///     ```
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(value_parser, value_hint=ValueHint::FilePath)]
    filename: PathBuf,
    #[clap(long = "trace_file", value_parser)]
    trace_file: Option<PathBuf>,
    #[structopt(long = "print_output")]
    print_output: bool,
    #[structopt(long = "entrypoint", default_value = "main")]
    entrypoint: String,
    #[structopt(long = "memory_file")]
    memory_file: Option<PathBuf>,
    #[structopt(long = "proof_mode")]
    proof_mode: bool,
    #[structopt(long = "secure_run")]
    secure_run: Option<bool>,
    #[clap(long = "air_public_input", requires = "proof_mode")]
    air_public_input: Option<String>,
    #[clap(
        long = "air_private_input",
        requires_all = ["proof_mode", "trace_file", "memory_file"]
    )]
    air_private_input: Option<String>,
    #[clap(
        long = "cairo_pie_output",
        // We need to add these air_private_input & air_public_input or else
        // passing cairo_pie_output + either of these without proof_mode will not fail
        conflicts_with_all = ["proof_mode", "air_private_input", "air_public_input"]
    )]
    cairo_pie_output: Option<String>,
    #[structopt(long = "allow_missing_builtins")]
    allow_missing_builtins: Option<bool>,
    #[structopt(
        long = "run_from_cairo_pie",
        // We need to add these air_private_input & air_public_input or else
        // passing run_from_cairo_pie + either of these without proof_mode will not fail
        conflicts_with_all = ["proof_mode", "air_private_input", "air_public_input"]
    )]
    run_from_cairo_pie: bool,
    #[structopt(long = "output_path")]
    output_path: PathBuf,
}

#[derive(Debug, Error)]
enum Error {
    #[error("Invalid arguments")]
    Cli(#[from] clap::Error),
    #[error("Failed to interact with the file system")]
    IO(#[from] std::io::Error),
    #[error("The cairo program execution failed")]
    Runner(#[from] CairoRunError),
    #[error("Serialization failed: {0}")]
    Serde(#[from] serde_json::error::Error),
}

fn main() -> ExitCode {
    // TODO(yuval): allow control on log levels through args.
    init_logging(log::LevelFilter::Info);
    match run(std::env::args()) {
        Ok(_) => {
            log::info!("VM runner succeeded");
            ExitCode::SUCCESS
        }
        Err(error) => {
            log::info!("VM runner failed: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run(args: impl Iterator<Item = String>) -> Result<CairoInput, Error> {
    let _span = span!(Level::INFO, "run").entered();
    let args = Args::try_parse_from(args)?;
    let cairo_runner = run_vm(&args)?;
    let cairo_input = adapt_vm_output_to_stwo(cairo_runner);

    let execution_resources = cairo_input.state_transitions.opcode_components.clone();
    std::fs::write(
        args.output_path,
        serde_json::to_string(&execution_resources)?,
    )?;

    Ok(cairo_input)
}

// This function's logic is copied-then-modified from cairo-vm-cli/src/main.rs:run in cairo-vm repo.
/// Runs the Cairo VM according to the given arguments (which are subset of the cairo-vm arguments).
fn run_vm(args: &Args) -> Result<CairoRunner, Error> {
    let _span = span!(tracing::Level::INFO, "run_vm").entered();
    let cairo_run_config = cairo_run::CairoRunConfig {
        entrypoint: &args.entrypoint,
        trace_enabled: true,
        relocate_mem: true,
        layout: LayoutName::all_cairo,
        proof_mode: args.proof_mode,
        secure_run: args.secure_run,
        allow_missing_builtins: args.allow_missing_builtins,
        ..Default::default()
    };

    let cairo_runner_result = if args.run_from_cairo_pie {
        let pie = CairoPie::read_zip_file(&args.filename)?;
        let mut hint_processor = BuiltinHintProcessor::new(
            Default::default(),
            RunResources::new(pie.execution_resources.n_steps),
        );
        cairo_run::cairo_run_pie(&pie, &cairo_run_config, &mut hint_processor)
    } else {
        let program_content = std::fs::read(args.filename.clone()).map_err(Error::IO)?;
        let mut hint_processor = BuiltinHintProcessor::new_empty();
        cairo_run::cairo_run(&program_content, &cairo_run_config, &mut hint_processor)
    };

    let cairo_runner = match cairo_runner_result {
        Ok(runner) => runner,
        Err(error) => {
            eprintln!("{error}");
            return Err(Error::Runner(error));
        }
    };

    Ok(cairo_runner)
}

/// Adapts the Cairo VM output to the input of Stwo.
/// Assumes memory and trace are already relocated. Otherwise panics.
fn adapt_vm_output_to_stwo(runner: CairoRunner) -> CairoInput {
    let _span = tracing::info_span!("adapt_vm_output_to_stwo").entered();
    input_from_finished_runner(runner, false)
}
