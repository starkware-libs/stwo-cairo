use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;
use stwo_cairo_adapter::vm_import::adapt_vm_output;
use stwo_cairo_adapter::ProverInput;
use stwo_cairo_prover::debug_tools::serialize_proof::{
    create_and_serialize_proof, Error, ProofFormat,
};
use stwo_cairo_utils::binary_utils::run_binary;
use tracing::{span, Level};

/// Command line arguments for adapted_stwo.
/// Example command line:
///     ```
///     cargo run -r --bin adapted_stwo -- --pub_json absolute/path/to/pub.json
///     --priv_json absolute/path/to/priv.json --proof_path path/to/proof
///     ```
///
/// For optimal performance compile with:
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
    #[structopt(long = "pub_json")]
    pub_json: PathBuf,
    #[structopt(long = "priv_json")]
    priv_json: PathBuf,
    /// The path to the JSON file containing the prover parameters (optional).
    /// The expected file format is:
    ///     {
    ///         "channel_hash":"blake2s",
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
    #[structopt(long = "track_relations")]
    track_relations: bool,
    /// Verify the generated proof.
    #[structopt(long = "verify")]
    verify: bool,
}

fn main() -> ExitCode {
    run_binary(run, "adapted_stwo")
}

fn run(args: impl Iterator<Item = String>) -> Result<(), Error> {
    let _span = span!(Level::INFO, "run").entered();
    let args = Args::try_parse_from(args)?;

    let input: ProverInput = adapt_vm_output(args.pub_json.as_path(), args.priv_json.as_path())?;

    log::info!(
        "Casm states by opcode:\n{}",
        input.state_transitions.casm_states_by_opcode
    );

    create_and_serialize_proof(
        input,
        args.verify,
        args.proof_path,
        args.proof_format,
        args.params_json,
    )
}
