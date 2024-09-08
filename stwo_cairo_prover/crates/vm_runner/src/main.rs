use std::path::PathBuf;
use std::process::ExitCode;

use cairo_vm::air_public_input::PublicInputError;
use cairo_vm::cairo_run;
use cairo_vm::cairo_run::EncodeTraceError;
use cairo_vm::hint_processor::builtin_hint_processor::builtin_hint_processor_definition::BuiltinHintProcessor;
use cairo_vm::types::layout_name::LayoutName;
use cairo_vm::vm::errors::cairo_run_errors::CairoRunError;
use cairo_vm::vm::errors::trace_errors::TraceError;
use cairo_vm::vm::errors::vm_errors::VirtualMachineError;
use cairo_vm::vm::runners::cairo_pie::CairoPie;
use cairo_vm::vm::runners::cairo_runner::RunResources;
use clap::{Parser, ValueHint};
use stwo_cairo_prover::input::plain::input_from_finished_runner;
use stwo_cairo_prover::input::CairoInput;
use thiserror::Error;

// TODO(yg): copied from cairo-vm as it's private. Make it public and use directly?
// TODO(yg): if not, remove all that's not needed for our use case.
/// Command line arguments for stwo_vm_runner
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
    // TODO(yg): deleted layout here - make sure it isn't needed.
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
    #[structopt(long = "tracer")]
    #[cfg(feature = "with_tracer")]
    tracer: bool,
    #[structopt(
        long = "run_from_cairo_pie",
        // We need to add these air_private_input & air_public_input or else
        // passing run_from_cairo_pie + either of these without proof_mode will not fail
        conflicts_with_all = ["proof_mode", "air_private_input", "air_public_input"]
    )]
    run_from_cairo_pie: bool,
}

// TODO(yg): copied from cairo-vm repo, cairo-vm-cli/src/main.rs. either make it public and use, or
// remove all that's irrelevant.
#[derive(Debug, Error)]
enum Error {
    #[error("Invalid arguments")]
    Cli(#[from] clap::Error),
    #[error("Failed to interact with the file system")]
    IO(#[from] std::io::Error),
    #[error("The cairo program execution failed")]
    Runner(#[from] CairoRunError),
    #[error(transparent)]
    EncodeTrace(#[from] EncodeTraceError),
    #[error(transparent)]
    VirtualMachine(#[from] VirtualMachineError),
    #[error(transparent)]
    Trace(#[from] TraceError),
    #[error(transparent)]
    PublicInput(#[from] PublicInputError),
    // TODO(yg): do we need the `with_tracer`?
    #[error(transparent)]
    #[cfg(feature = "with_tracer")]
    TraceDataError(#[from] TraceDataError),
}

fn main() -> ExitCode {
    match run(std::env::args()) {
        Ok(_) => ExitCode::SUCCESS,
        // TODO(yg): log?
        Err(_) => ExitCode::FAILURE,
    }
}

fn run(args: impl Iterator<Item = String>) -> Result<CairoInput, Error> {
    // TODO(yg): this is copied from cairo-vm-cli/src/main.rs:run in cairo-vm repo. consider using
    // directly (currently private)
    let args = Args::try_parse_from(args)?;

    let trace_enabled = args.trace_file.is_some() || args.air_public_input.is_some();

    let cairo_run_config = cairo_run::CairoRunConfig {
        entrypoint: &args.entrypoint,
        trace_enabled,
        relocate_mem: args.memory_file.is_some() || args.air_public_input.is_some(),
        layout: LayoutName::all_cairo,
        proof_mode: args.proof_mode,
        secure_run: args.secure_run,
        allow_missing_builtins: args.allow_missing_builtins,
        ..Default::default()
    };

    // TODO(yg): Do we need the else?
    let cairo_runner = match {
        if args.run_from_cairo_pie {
            let pie = CairoPie::read_zip_file(&args.filename)?;
            let mut hint_processor = BuiltinHintProcessor::new(
                Default::default(),
                RunResources::new(pie.execution_resources.n_steps),
            );
            cairo_run::cairo_run_pie(&pie, &cairo_run_config, &mut hint_processor)
        } else {
            let program_content = std::fs::read(args.filename).map_err(Error::IO)?;
            let mut hint_processor = BuiltinHintProcessor::new_empty();
            cairo_run::cairo_run(&program_content, &cairo_run_config, &mut hint_processor)
        }
    } {
        Ok(runner) => runner,
        Err(error) => {
            eprintln!("{error}");
            return Err(Error::Runner(error));
        }
    };

    // TODO(yg): split here to 2 parts - one is running the cairo-vm and getting cairo_runner.
    // Second is adapter.

    let cairo_input = input_from_finished_runner(cairo_runner);
    // TODO(yg): serialize (here or in an outer function.

    Ok(cairo_input)
}
