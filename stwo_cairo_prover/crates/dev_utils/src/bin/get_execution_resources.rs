//! // ### Example command to run the verifier:
//! ```
//! cargo run --bin get_execution_resources -- --program  /path/to/verifier.executable --program_type executable --arguments-file /path/to/proof.json
//! ```
//!
//! To view all available command-line options, run:
//! ```
//! cargo run --bin get_execution_resources -- --help
//! ```
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use stwo_cairo_adapter::utils::{run_program_and_adapter, ProgramType};
use stwo_cairo_adapter::ExecutionResources;
use tracing::{span, Level};
use tracing_subscriber::fmt::format::FmtSpan;

/// A tool to get the execution resources from a compiled Cairo program.
#[derive(Parser, Debug)]
struct Args {
    /// Path to the compiled Cairo program.
    #[arg(long = "program")]
    program: PathBuf,
    /// Indicates if program is an executable or json.
    #[arg(long = "program_type", default_value = "json")]
    program_type: ProgramType,
    /// Path to a file with arguments for the Cairo program.
    #[arg(long = "program_arguments_file")]
    program_arguments_file: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
        .init();

    let _span = span!(Level::INFO, "run").entered();

    let prover_input = run_program_and_adapter(
        &args.program,
        args.program_type,
        args.program_arguments_file.as_ref(),
    )?;

    let execution_resources = ExecutionResources::from_prover_input(&prover_input);
    log::info!("Execution resources: {execution_resources:#?}");

    Ok(())
}
