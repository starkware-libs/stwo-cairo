use std::fs::read_to_string;
use std::path::PathBuf;

use anyhow::Result;
use cairo_air::utils::ProofFormat;
use cairo_air::PreProcessedTraceVariant;
use clap::Parser;
use stwo_cairo_prover::prover::{create_and_serialize_proof, ProverParameters};
use stwo_cairo_utils::vm_utils::{run_and_adapt_full, ProgramType};
use tracing::{span, Level};
use tracing_subscriber::fmt::format::FmtSpan;

// Command line arguments for 'prove_from_compiled_program'.
/// Example command line:
///     ```
///     cargo run --bin run_and_prove -- --program
/// absolute/path/to/compiled.json     --proof_path path/to/proof
///     ```
///
/// For optimal performance compile with:
///     ```
///   RUSTFLAGS="-C target-cpu=native -C opt-level=3"
///     ```
#[derive(Parser, Debug)]
struct Args {
    #[structopt(long = "program")]
    program: PathBuf,
    /// Indicates if program is an executable or json.
    #[arg(long = "program_type", default_value = "json")]
    program_type: ProgramType,
    /// Path to a file with arguments for the Cairo program.
    #[arg(long = "program_arguments_file")]
    program_arguments_file: Option<PathBuf>,
    #[structopt(long = "params_json")]
    /// The path to the JSON file containing the prover parameters (optional).
    /// The expected file format is:
    ///     {
    ///         "channel_hash":"blake2s",
    ///         "channel_salt": 0
    ///         "pcs_config": {
    ///             "pow_bits": 26,
    ///             "fri_config": {
    ///                 "log_last_layer_degree_bound": 0,
    ///                 "log_blowup_factor": 1,
    ///                 "n_queries": 70
    ///             }
    ///         },
    ///         "preprocessed_trace": "canonical_without_pedersen",
    ///     }
    ///
    /// Default parameters are chosen to ensure 96 bits of security.
    proof_params_json: Option<PathBuf>,
    /// The output file path for the proof.
    #[structopt(long = "proof_path")]
    proof_path: PathBuf,
    /// The format of the proof output.
    /// - json: Standard JSON format (default)
    /// - cairo_serde: Array of field elements serialized as hex strings, ex. `["0x1", "0x2"]`
    /// - binary: Binary format, compressed
    #[arg(long, value_enum, default_value_t = ProofFormat::Json)]
    proof_format: ProofFormat,
    /// Verify the generated proof.
    #[structopt(long = "verify")]
    verify: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
        .init();
    let _span = span!(Level::INFO, "run_and_prove").entered();

    // Load proof params early to get preprocessed trace variant.
    let preprocessed_trace_variant = if let Some(ref proof_params_json) = args.proof_params_json {
        let params: ProverParameters = sonic_rs::from_str(&read_to_string(proof_params_json)?)?;
        params.preprocessed_trace
    } else {
        // Default to Canonical if no params provided.
        PreProcessedTraceVariant::Canonical
    };

    let adapted_input = run_and_adapt_full(
        &args.program,
        args.program_type,
        args.program_arguments_file.as_ref(),
        preprocessed_trace_variant,
    )?;

    let result = create_and_serialize_proof(
        adapted_input,
        args.verify,
        args.proof_path,
        args.proof_format,
        args.proof_params_json,
    );
    match result {
        Ok(_) => log::info!("✅ Proved successfully!"),
        Err(ref e) => log::error!("❌ Proving failed: {e:?}"),
    }

    result
}
