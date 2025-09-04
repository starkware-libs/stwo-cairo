//! ### Example command to extract memory trace:
//! ```
//! cargo run --features extract-mem-trace --bin extract_mem_trace -- --compiled_program /path/to/compiled.json --mem /path/to/mem_file --trace /path/to/trace_file
//! ```
//!
//! To view all available command-line options, run:
//! ```
//! cargo run --features extract-mem-trace --bin extract_mem_trace -- --help
//! ```

use std::fs::write;
use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use dev_utils::utils::runner_from_compiled_program;
use sonic_rs::to_string_pretty;
use stwo_cairo_adapter::adapter::adapter;
use tracing::{span, Level};
use tracing_subscriber::fmt::format::FmtSpan;

#[derive(Clone, Debug, Default, ValueEnum)]
enum OutputFormat {
    /// Output in JSON format (default)
    #[default]
    Json,
    /// Output in binary format using bincode
    Binary,
}

/// A tool to extract memory trace from a compiled Cairo program.
#[derive(Parser, Debug)]
struct Args {
    /// Path to the compiled Cairo program.
    #[arg(long = "compiled_program")]
    compiled_program: PathBuf,
    /// Output file path for the memory.
    #[arg(long = "mem_file")]
    mem: Option<PathBuf>,
    /// Output file path for the trace.
    #[arg(long = "trace_file")]
    trace: Option<PathBuf>,
    #[arg(long = "cairo1")]
    /// Indicates that the input program is compiled from Cairo 1
    cairo1: bool,
    /// Path to a file with arguments for the Cairo program.
    #[arg(long = "program_arguments_file")]
    program_arguments_file: Option<PathBuf>,
    /// Output format for the files.
    #[arg(long = "format", default_value = "json")]
    format: OutputFormat,
}

fn main() {
    let args = Args::parse();

    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
        .init();

    let _span = span!(Level::INFO, "extract_mem_trace").entered();

    let runner = runner_from_compiled_program(
        &args.compiled_program,
        args.cairo1,
        args.program_arguments_file.as_ref(),
    );
    let prover_input = adapter(&runner);

    if let Some(mem_file) = args.mem {
        match args.format {
            OutputFormat::Binary => {
                let data = bincode::serialize(&prover_input.relocated_mem)
                    .expect("Failed to serialize memory data to binary");
                write(mem_file, data).expect("Failed to write memory data to file");
            }
            OutputFormat::Json => {
                let data = to_string_pretty(&prover_input.relocated_mem)
                    .expect("Failed to serialize memory data to JSON");
                write(mem_file, data).expect("Failed to write memory data to file");
            }
        }
    }
    if let Some(trace_file) = args.trace {
        match args.format {
            OutputFormat::Binary => {
                let data = bincode::serialize(&prover_input.relocated_trace)
                    .expect("Failed to serialize trace data to binary");
                write(trace_file, data).expect("Failed to write trace data to file");
            }
            OutputFormat::Json => {
                let data = to_string_pretty(&prover_input.relocated_trace)
                    .expect("Failed to serialize trace data to JSON");
                write(trace_file, data).expect("Failed to write trace data to file");
            }
        }
    }
}
