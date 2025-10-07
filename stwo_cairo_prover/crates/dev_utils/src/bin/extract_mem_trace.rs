//! ### Example command to extract memory trace:
//! ```
//! cargo run --features extract-mem-trace --bin extract_mem_trace -- --program /path/to/compiled.json --mem /path/to/mem_file --trace /path/to/trace_file
//! ```
//!
//! To view all available command-line options, run:
//! ```
//! cargo run --features extract-mem-trace --bin extract_mem_trace -- --help
//! ```

use std::fs::write;
use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, ValueEnum};
use serde::Serialize;
use sonic_rs::to_string_pretty;
use stwo_cairo_utils::vm_utils::{run_and_adapt, ProgramType};
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
    #[arg(long = "program")]
    program: PathBuf,
    /// Indicates if program is an executable or json.
    #[arg(long = "program_type", default_value = "json")]
    program_type: ProgramType,
    /// Output file path for the memory.
    #[arg(long = "mem")]
    mem: Option<PathBuf>,
    /// Output file path for the trace.
    #[arg(long = "trace")]
    trace: Option<PathBuf>,
    /// Path to a file with arguments for the Cairo program.
    #[arg(long = "program_arguments_file")]
    program_arguments_file: Option<PathBuf>,
    /// Output format for the files.
    #[arg(long = "format", default_value = "json")]
    format: OutputFormat,
}

fn main() -> Result<()> {
    let args = Args::parse();

    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
        .init();
    let _span = span!(Level::INFO, "extract_mem_trace").entered();

    let prover_input = run_and_adapt(
        &args.program,
        args.program_type,
        args.program_arguments_file.as_ref(),
    )?;

    if let Some(mem_file) = args.mem.as_ref() {
        write_output(mem_file, &args.format, &prover_input.relocated_mem);
    }

    if let Some(trace_file) = args.trace.as_ref() {
        write_output(trace_file, &args.format, &prover_input.relocated_trace);
    }

    Ok(())
}

fn write_output<T: Serialize>(path: &PathBuf, format: &OutputFormat, data: &T) {
    match format {
        OutputFormat::Binary => {
            let bytes = bincode::serialize(data).expect("Failed to serialize data to binary");
            write(path, bytes).expect("Failed to write data to file");
        }
        OutputFormat::Json => {
            let data = to_string_pretty(data).expect("Failed to serialize data to JSON");
            write(path, data).expect("Failed to write data to file");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs::read;
    use std::process::Command;

    use bincode::deserialize;
    use cairo_vm::vm::trace::trace_entry::RelocatedTraceEntry;
    use dev_utils::utils::get_compiled_cairo_program_path;
    use stwo_cairo_adapter::memory::MemoryEntry;
    use stwo_cairo_utils::vm_utils::{run_and_adapt, ProgramType};
    use tempfile::NamedTempFile;

    #[test]
    fn test_serialize_deserialize_mem_trace() {
        // Use an existing test program
        let compiled_program_path =
            get_compiled_cairo_program_path("test_prove_verify_all_opcode_components");

        let prover_input = run_and_adapt(&compiled_program_path, ProgramType::Json, None).unwrap();

        // Test JSON format first
        let temp_mem_file = NamedTempFile::new().expect("Failed to create temp file");
        let temp_trace_file = NamedTempFile::new().expect("Failed to create temp file");

        let binary_output = Command::new("cargo")
            .args([
                "run",
                "--features",
                "stwo-cairo-adapter/extract-mem-trace",
                "--bin",
                "extract_mem_trace",
                "--",
                "--program",
                &compiled_program_path.to_string_lossy(),
                "--mem",
                &temp_mem_file.path().to_string_lossy(),
                "--trace",
                &temp_trace_file.path().to_string_lossy(),
                "--format",
                "binary",
            ])
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .output()
            .expect("Failed to run extract_mem_trace binary for binary format");

        assert!(
            binary_output.status.success(),
            "extract_mem_trace failed for binary format"
        );

        let binary_mem_content =
            read(temp_mem_file.path()).expect("Failed to read binary memory file");
        let binary_trace_content =
            read(temp_trace_file.path()).expect("Failed to read binary trace file");

        let relocated_mem: Vec<MemoryEntry> =
            deserialize(&binary_mem_content).expect("Failed to deserialize relocated memory");
        let relocated_trace: Vec<RelocatedTraceEntry> =
            deserialize(&binary_trace_content).expect("Failed to deserialize trace");

        assert_eq!(relocated_mem, prover_input.relocated_mem);
        assert_eq!(relocated_trace, prover_input.relocated_trace);
    }
}
