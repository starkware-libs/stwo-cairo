use std::path::PathBuf;
use std::process::ExitCode;
use std::time::SystemTime;

use cairo_vm::air_public_input::PublicInputError;
use cairo_vm::cairo_run;
use cairo_vm::cairo_run::EncodeTraceError;
use cairo_vm::hint_processor::builtin_hint_processor::builtin_hint_processor_definition::BuiltinHintProcessor;
use cairo_vm::types::layout_name::LayoutName;
use cairo_vm::vm::errors::cairo_run_errors::CairoRunError;
use cairo_vm::vm::errors::trace_errors::TraceError;
use cairo_vm::vm::errors::vm_errors::VirtualMachineError;
use cairo_vm::vm::runners::cairo_pie::CairoPie;
use cairo_vm::vm::runners::cairo_runner::{CairoRunner, RunResources};
use clap::{Parser, ValueHint};
use stwo_cairo_prover::input::plain::{input_from_finished_runner, print_now};
use stwo_cairo_prover::input::CairoInput;
use thiserror::Error;

// TODO(yg): copied from cairo-vm as it's private. Consider making it public and use directly.
// TODO(yg): Or - remove all that's not needed for our use case.
/// Command line arguments for stwo_vm_runner.
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
        Ok(_) => {
            println!("yg success");
            ExitCode::SUCCESS
        }
        // TODO(yg): log?
        Err(_) => {
            println!("yg failure");
            ExitCode::FAILURE
        }
    }
}

// TODO(yg): revert unwraps to `?`s.
fn run(args: impl Iterator<Item = String>) -> Result<CairoInput, Error> {
    print_now("beginning");
    let args = Args::try_parse_from(args).unwrap();
    let cairo_runner = run_vm(&args)?;
    print_now("middle");
    let cairo_input = adapt(cairo_runner);
    print_now("end");
    // TODO(yg): serialize (here or in an outer function).

    Ok(cairo_input)
}

fn run_vm(args: &Args) -> Result<CairoRunner, Error> {
    // TODO(yg): this is copied-and-modified from cairo-vm-cli/src/main.rs:run in cairo-vm repo.
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

    // TODO(yg): Do we need the else?
    let cairo_runner = match {
        if args.run_from_cairo_pie {
            let pie = CairoPie::read_zip_file(&args.filename).unwrap();
            let mut hint_processor = BuiltinHintProcessor::new(
                Default::default(),
                RunResources::new(pie.execution_resources.n_steps),
            );
            print_now("if - before cairo_run_pie");
            let x = cairo_run::cairo_run_pie(&pie, &cairo_run_config, &mut hint_processor);
            print_now(&format!("if - after cairo_run_pie, {}", x.is_err()));
            x
        } else {
            let program_content = std::fs::read(args.filename.clone())
                .map_err(Error::IO)
                .unwrap();
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

    Ok(cairo_runner)
}

fn adapt(runner: CairoRunner) -> CairoInput {
    input_from_finished_runner(runner)
}
