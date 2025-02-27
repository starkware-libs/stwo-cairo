use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;
use serde::Serialize;
use stwo_cairo_adapter::vm_import::{adapt_vm_output, VmImportError};
use stwo_cairo_adapter::ProverInput;
use stwo_cairo_prover::cairo_air::{
    default_prod_prover_parameters, prove_cairo, verify_cairo, CairoVerificationError, ChannelHash,
    ConfigBuilder, ProverConfig, ProverParameters,
};
use stwo_cairo_utils::binary_utils::run_binary;
use stwo_cairo_utils::file_utils::{read_to_string, IoErrorWithPath};
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::backend::BackendForChannel;
use stwo_prover::core::channel::MerkleChannel;
use stwo_prover::core::pcs::PcsConfig;
use stwo_prover::core::prover::ProvingError;
use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleChannel;
use stwo_prover::core::vcs::poseidon252_merkle::Poseidon252MerkleChannel;
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
    #[structopt(long = "pub_json")]
    pub_json: PathBuf,
    #[structopt(long = "priv_json")]
    priv_json: PathBuf,
    /// The path to the JSON file containing the prover parameters (optional).
    /// The expected file format is:
    ///     {
    ///         "pcs_config": {
    ///             "pow_bits": 26,
    ///             "fri_config": {
    ///                 "log_last_layer_degree_bound": 0,
    ///                 "log_blowup_factor": 1,
    ///                 "n_queries": 70
    ///             }
    ///         }
    ///     }
    ///
    /// Default parameters are chosen to ensure 96 bits of security.
    #[structopt(long = "params_json")]
    params_json: Option<PathBuf>,
    /// The output file path for the proof.
    #[structopt(long = "proof_path")]
    proof_path: PathBuf,
    #[structopt(long = "track_relations")]
    track_relations: bool,
    #[structopt(long = "display_components")]
    display_components: bool,
    /// Verify the generated proof.
    #[structopt(long = "verify")]
    verify: bool,
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
    Serde(#[from] serde_json::error::Error),
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
    let prover_config = ConfigBuilder::default()
        .track_relations(args.track_relations)
        .display_components(args.display_components)
        .build();

    log::info!(
        "Casm states by opcode:\n{}",
        vm_output.state_transitions.casm_states_by_opcode
    );

    let ProverParameters {
        channel_hash,
        pcs_config,
    } = match args.params_json {
        Some(path) => serde_json::from_str(&read_to_string(&path)?)?,
        None => default_prod_prover_parameters(),
    };

    let run_inner_fn = match channel_hash {
        ChannelHash::Blake2s => run_inner::<Blake2sMerkleChannel>,
        ChannelHash::Poseidon252 => run_inner::<Poseidon252MerkleChannel>,
    };

    run_inner_fn(
        vm_output,
        prover_config,
        pcs_config,
        args.verify,
        args.proof_path,
    )?;

    Ok(())
}

/// Generates proof given the Cairo VM output and prover config/parameters.
/// Serializes the proof as JSON and write to the output path.
/// Verifies the proof in case the respective flag is set.
fn run_inner<MC: MerkleChannel>(
    vm_output: ProverInput,
    prover_config: ProverConfig,
    pcs_config: PcsConfig,
    verify: bool,
    proof_path: PathBuf,
) -> Result<(), Error>
where
    SimdBackend: BackendForChannel<MC>,
    MC::H: Serialize,
{
    let proof = prove_cairo::<MC>(vm_output, prover_config, pcs_config)?;
    std::fs::write(&proof_path, serde_json::to_string(&proof)?)?;

    if verify {
        verify_cairo::<MC>(proof, pcs_config)?;
        log::info!("Proof verified successfully");
    }
    Ok(())
}
