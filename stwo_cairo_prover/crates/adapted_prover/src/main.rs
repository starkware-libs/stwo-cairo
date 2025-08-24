use std::io::Write;
use std::path::PathBuf;
use std::process::ExitCode;

use cairo_air::verifier::{verify_cairo, CairoVerificationError};
use cairo_air::PreProcessedTraceVariant;
use clap::Parser;
use serde::Serialize;
use stwo::core::channel::MerkleChannel;
use stwo::core::pcs::PcsConfig;
use stwo::core::vcs::blake2_merkle::Blake2sMerkleChannel;
use stwo::core::vcs::poseidon252_merkle::Poseidon252MerkleChannel;
use stwo::core::vcs::MerkleHasher;
use stwo::prover::backend::simd::SimdBackend;
use stwo::prover::backend::BackendForChannel;
use stwo::prover::ProvingError;
use stwo_cairo_adapter::vm_import::{adapt_vm_output, VmImportError};
use stwo_cairo_adapter::{log_prover_input, ProverInput};
use stwo_cairo_prover::prover::{
    default_prod_prover_parameters, prove_cairo, ChannelHash, ProverParameters,
};
use stwo_cairo_serialize::CairoSerialize;
use stwo_cairo_utils::binary_utils::run_binary;
use stwo_cairo_utils::file_utils::{create_file, read_to_string, IoErrorWithPath};
use thiserror::Error;
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
    #[structopt(
        long = "pub_json",
        help = "Absolute path to the JSON file containing the program public input."
    )]
    pub_json: PathBuf,
    #[structopt(
        long = "priv_json",
        help = "Absolute path to the JSON file containing the program private input."
    )]
    priv_json: PathBuf,
    /// Optional absolute path to the JSON file containing the prover parameters.
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
    #[structopt(long = "proof_path", help = "Absolute path to the output proof file.")]
    proof_path: PathBuf,
    /// The format of the proof output.
    /// - json: Standard JSON format (default)
    /// - cairo_serde: Array of field elements serialized as hex strings, ex. `["0x1", "0x2"]`
    #[arg(long, value_enum, default_value_t = ProofFormat::Json)]
    proof_format: ProofFormat,
    #[structopt(long = "verify", help = "Verify the generated proof.")]
    verify: bool,
}

#[derive(Debug, Clone, clap::ValueEnum)]
enum ProofFormat {
    /// Standard JSON format.
    Json,
    /// Array of field elements serialized as hex strings.
    /// Compatible with `scarb execute`
    CairoSerde,
}

#[derive(Debug, Error)]
enum Error {
    #[error("Invalid arguments: {0}")]
    Cli(#[from] clap::Error),
    #[error("IO failed: {0}")]
    IO(#[from] std::io::Error),
    #[error("Proving failed: {0}")]
    Proving(#[from] ProvingError),
    #[error("Serialization failed: {0}")]
    Serializing(#[from] sonic_rs::error::Error),
    #[error("Verification failed: {0}")]
    Verification(#[from] CairoVerificationError),
    #[error("VM import failed: {0}")]
    VmImport(#[from] VmImportError),
    #[error("File IO failed: {0}")]
    File(#[from] IoErrorWithPath),
}

fn main() -> ExitCode {
    run_binary(run, "adapted_stwo")
}

fn run(args: impl Iterator<Item = String>) -> Result<(), Error> {
    let _span = span!(Level::INFO, "run").entered();
    let args = Args::try_parse_from(args)?;

    let vm_output: ProverInput =
        adapt_vm_output(args.pub_json.as_path(), args.priv_json.as_path())?;

    log_prover_input(&vm_output);

    let ProverParameters {
        channel_hash,
        pcs_config,
        preprocessed_trace,
    } = match args.params_json {
        Some(path) => sonic_rs::from_str(&read_to_string(&path)?)?,
        None => default_prod_prover_parameters(),
    };

    let run_inner_fn = match channel_hash {
        ChannelHash::Blake2s => run_inner::<Blake2sMerkleChannel>,
        ChannelHash::Poseidon252 => run_inner::<Poseidon252MerkleChannel>,
    };

    run_inner_fn(
        vm_output,
        pcs_config,
        preprocessed_trace,
        args.verify,
        args.proof_path,
        args.proof_format,
    )?;

    Ok(())
}

/// Generates proof given the Cairo VM output and prover config/parameters.
/// Serializes the proof as JSON and write to the output path.
/// Verifies the proof in case the respective flag is set.
fn run_inner<MC: MerkleChannel>(
    vm_output: ProverInput,
    pcs_config: PcsConfig,
    preprocessed_trace: PreProcessedTraceVariant,
    verify: bool,
    proof_path: PathBuf,
    proof_format: ProofFormat,
) -> Result<(), Error>
where
    SimdBackend: BackendForChannel<MC>,
    MC::H: Serialize,
    <MC::H as MerkleHasher>::Hash: CairoSerialize,
{
    let proof = prove_cairo::<MC>(vm_output, pcs_config, preprocessed_trace)?;
    let mut proof_file = create_file(&proof_path)?;

    let span = span!(Level::INFO, "Serialize proof").entered();
    match proof_format {
        ProofFormat::Json => {
            proof_file.write_all(sonic_rs::to_string_pretty(&proof)?.as_bytes())?;
        }
        ProofFormat::CairoSerde => {
            let mut serialized: Vec<starknet_ff::FieldElement> = Vec::new();
            CairoSerialize::serialize(&proof, &mut serialized);

            let hex_strings: Vec<String> = serialized
                .into_iter()
                .map(|felt| format!("0x{felt:x}"))
                .collect();

            proof_file.write_all(sonic_rs::to_string_pretty(&hex_strings)?.as_bytes())?;
        }
    }
    span.exit();
    if verify {
        verify_cairo::<MC>(proof, preprocessed_trace)?;
        log::info!("Proof verified successfully");
    }

    Ok(())
}
