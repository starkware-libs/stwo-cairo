//! // ### Example command to run the verifier:
//! ```
//! cargo run --bin execution_resources_from_compiled_program -- --cairo1 --compiled_program  /path/to/executable.json --arguments-file /path/to/proof.json
//! ```
//!
//! To view all available command-line options, run:
//! ```
//! cargo run --bin execution_resources_from_compiled_program -- --help
//! ```
use std::path::PathBuf;

use cairo_lang_executable::executable::Executable;
use clap::Parser;
use dev_utils::utils::{run_cairo1_and_adapter, Error, ProgramArguments};
use stwo_cairo_adapter::test_utils::{read_compiled_cairo_program, run_program_and_adapter};
use stwo_cairo_adapter::ExecutionResources;
use tracing::{span, Level};
use tracing_subscriber::fmt::format::FmtSpan;

/// A tool to get the execution resources from a compiled Cairo program.
#[derive(Parser, Debug)]
struct Args {
    /// Path to the compiled Cairo program.
    #[arg(long = "compiled_program")]
    compiled_program: PathBuf,
    /// Indicates that the input program is compiled from Cairo 1
    #[arg(long = "cairo1")]
    cairo1: bool,
    /// Arguments to pass to the Cairo program, either inline or from file
    #[command(flatten)]
    program_arguments: ProgramArguments,
}

fn main() -> Result<(), Error> {
    let args = Args::parse();
    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
        .init();

    let _span = span!(Level::INFO, "run").entered();

    let prover_input = if args.cairo1 {
        let executable: Executable =
            serde_json::from_reader(std::fs::File::open(&args.compiled_program).unwrap())
                .expect("Failed to read executable");
        run_cairo1_and_adapter(executable, args.program_arguments.read_arguments())
    } else {
        assert!(
            args.program_arguments.arguments.is_empty()
                && args.program_arguments.arguments_file.is_none(),
            "Can't run Cairo0 programs with arguments"
        );
        let program = read_compiled_cairo_program(&args.compiled_program);
        run_program_and_adapter(&program)
    };

    let execution_resources = ExecutionResources::from_prover_input(&prover_input);
    log::info!("Execution resources: {execution_resources:#?}");

    Ok(())
}
