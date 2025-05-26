use std::path::PathBuf;

use clap::Parser;
use stwo_cairo_adapter::test_utils::{read_compiled_cairo_program, run_program_and_adapter};
use stwo_cairo_utils::logging_utils::init_logging;
use tracing::{span, Level};
use vm_adapter_prover::utils::{create_and_serialize_proof, Error, ProofFormat};

// Command line arguments for adapted_stwo.
/// Example command line:
///     ```
///     cargo run --bin vm_adapter_prover -- --compiled_program absolute/path/to/compiled.json
/// --proof_path path/to/proof``` For optimal performance compile with:
///     ```
///   RUSTFLAGS="-C target-cpu=native -C opt-level=3"
///     ```
///
/// Standard library features is off by default, to enable it, further improving performance, use:
///     ```
///     --features=std
///     ```
#[derive(Parser, Debug)]
struct Args {
    #[structopt(long = "compiled_program")]
    compiled_program: PathBuf,
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
    params_json: Option<PathBuf>,
    /// The output file path for the proof.
    #[structopt(long = "proof_path")]
    proof_path: PathBuf,
    /// The format of the proof output.
    /// - json: Standard JSON format (default)
    /// - cairo_serde: Array of field elements serialized as hex strings, ex. `["0x1", "0x2"]`
    #[arg(long, value_enum, default_value_t = ProofFormat::Json)]
    proof_format: ProofFormat,
    /// Verify the generated proof.
    #[structopt(long = "verify")]
    verify: bool,
}

fn main() -> Result<(), Error> {
    init_logging(log::LevelFilter::Info);
    let _span = span!(Level::INFO, "run").entered();
    let args = Args::try_parse_from(std::env::args())?;

    let compiled_program = read_compiled_cairo_program(&args.compiled_program);
    let input = run_program_and_adapter(&compiled_program);

    create_and_serialize_proof(
        input,
        args.verify,
        args.proof_path,
        args.proof_format,
        args.params_json,
    )
}
