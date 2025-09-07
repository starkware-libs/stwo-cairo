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

use clap::Parser;
use stwo_cairo_adapter::utils::run_program_and_adapter;
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
    /// Path to a file with arguments for the Cairo program.
    #[arg(long = "program_arguments_file")]
    program_arguments_file: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();
    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
        .init();

    let _span = span!(Level::INFO, "run").entered();

    let prover_input = run_program_and_adapter(
        &args.compiled_program,
        args.cairo1,
        args.program_arguments_file.as_ref(),
    );

    let execution_resources = ExecutionResources::from_prover_input(&prover_input);
    log::info!("Execution resources: {execution_resources:#?}");
}
