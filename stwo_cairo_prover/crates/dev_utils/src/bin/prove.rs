use std::fs::File;
use std::path::PathBuf;

use anyhow::Result;
use cairo_air::utils::ProofFormat;
use clap::Parser;
use serde_json::from_reader;
use stwo_cairo_adapter::ProverInput;
use stwo_cairo_prover::prover::create_and_serialize_proof;
use tracing::{span, Level};
use tracing_subscriber::fmt::format::FmtSpan;

// Command line arguments for 'prove_from_compiled_program'.
/// Example command line:
///     ```
///     cargo run --bin prove -- --program
/// absolute/path/to/compiled.json     --proof_path path/to/proof
///     ```
///
/// For optimal performance compile with:
///     ```
///   RUSTFLAGS="-C target-cpu=native -C opt-level=3"
///     ```
#[derive(Parser, Debug)]
struct Args {
    #[structopt(long = "prover_input_path")]
    prover_input_path: PathBuf,
    /// The path to the JSON file containing the prover parameters (optional).
    /// The expected file format is:
    ///     {
    ///         "channel_hash":"blake2s",Sd
    ///         "pcs_config": {
    ///             "pow_bits": 26,
    ///             "fri_config": {
    ///                 "log_last_layer_degree_bound": 0,
    ///                 "log_blowup_factor": 1,
    ///                 "n_queries": 70
    ///             }
    ///         },
    ///         "preprocessed_trace": "canonical_without_pedersen"
    ///     }
    ///
    /// Default parameters are chosen to ensure 96 bits of security.
    #[structopt(long = "params_json")]
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
    let _span = span!(Level::INFO, "prove").entered();

    let prover_input: ProverInput = from_reader(File::open(args.prover_input_path)?)?;
    let result = create_and_serialize_proof(
        prover_input,
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
