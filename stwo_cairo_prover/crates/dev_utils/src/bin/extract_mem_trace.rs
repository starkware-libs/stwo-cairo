//! // ### Example command to run the verifier:
//! ```
//! cargo run --bin extract_mem_trace -- --compiled_program  /path/to/compiled.json --mem /path/to/mem_file --trace /path/to/trace_file
//! ```
//!
//! To view all available command-line options, run:
//! ```
//! cargo run --bin extract_mem_trace -- --help
//! ```

use std::fs::{write, File};
use std::path::PathBuf;

use cairo_lang_executable::executable::Executable;
use clap::Parser;
use dev_utils::utils::{
    read_cairo_arguments_from_file, read_compiled_cairo_program, run_cairo1_program, run_program,
};
use serde_json::from_reader;
use sonic_rs::to_string_pretty;
use stwo_cairo_adapter::adapter::extract_relocated_mem_trace;
use tracing::{span, Level};
use tracing_subscriber::fmt::format::FmtSpan;

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
}

fn main() {
    let args = Args::parse();

    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
        .init();

    let _span = span!(Level::INFO, "extract_mem_trace").entered();

    let runner = if args.cairo1 {
        let executable: Executable =
            from_reader(File::open(&args.compiled_program).expect("Unable to open executable"))
                .expect("Failed to read executable");
        let args = args
            .program_arguments_file
            .map(|path| read_cairo_arguments_from_file(&path))
            .unwrap_or_default();
        run_cairo1_program(executable, args)
    } else {
        assert!(
            args.program_arguments_file.is_none(),
            "Can't run Cairo0 programs with arguments"
        );
        let program = read_compiled_cairo_program(&args.compiled_program);
        run_program(&program, None)
    };

    let (relocated_memory, relocated_trace) = extract_relocated_mem_trace(&runner);
    if let Some(mem_file) = args.mem {
        let json_data =
            to_string_pretty(&relocated_memory).expect("Failed to serialize data to JSON");
        write(mem_file, json_data).expect("Failed to write data to file");
    }
    if let Some(trace_file) = args.trace {
        let json_data =
            to_string_pretty(&relocated_trace).expect("Failed to serialize data to JSON");
        write(trace_file, json_data).expect("Failed to write data to file")
    }
}
