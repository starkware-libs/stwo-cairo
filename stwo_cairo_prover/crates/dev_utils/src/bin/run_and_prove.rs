use std::fs::read_to_string;
use std::path::PathBuf;

use anyhow::Result;
use cairo_air::utils::{serialize_proof_to_file, ProofFormat};
use cairo_air::verifier::verify_cairo;
use cairo_air::PreProcessedTraceVariant;
use clap::Parser;
use stwo::core::fri::FriConfig;
use stwo::core::pcs::PcsConfig;
use stwo::core::vcs_lifted::blake2_merkle::Blake2sMerkleChannel;
use stwo_cairo_prover::prover::{
    precompute_trace_data, prove_cairo_with_precomputed, ChannelHash, ProverParameters,
};
use stwo_cairo_utils::vm_utils::{run_and_adapt, ProgramType};
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

mod json {
    #[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
    pub use serde_json::from_str;
    #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
    pub use sonic_rs::from_str;
}

fn main() -> Result<()> {
    let args = Args::parse();
    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
        .init();
    let _span = span!(Level::INFO, "run_and_prove").entered();

    // Parse prover params early so we can start precomputation in parallel.
    let proof_params = if let Some(ref proof_params_json) = args.proof_params_json {
        json::from_str(&read_to_string(proof_params_json)?)?
    } else {
        // The default prover parameters for prod use (96 bits of security).
        ProverParameters {
            channel_hash: ChannelHash::Blake2s,
            channel_salt: 0,
            pcs_config: PcsConfig {
                pow_bits: 26,
                fri_config: FriConfig {
                    log_last_layer_degree_bound: 0,
                    log_blowup_factor: 1,
                    n_queries: 70,
                },
            },
            preprocessed_trace: PreProcessedTraceVariant::Canonical,
            store_polynomials_coefficients: false,
        }
    };

    // Run VM+adapt and precompute trace data in parallel.
    let (prover_input_result, precomputed) = rayon::join(
        || {
            run_and_adapt(
                &args.program,
                args.program_type.clone(),
                args.program_arguments_file.as_ref(),
            )
        },
        || precompute_trace_data(&proof_params),
    );

    let prover_input = prover_input_result?;

    // Prove with precomputed data.
    let result: Result<()> = match proof_params.channel_hash {
        ChannelHash::Blake2s => {
            let cairo_proof = prove_cairo_with_precomputed::<Blake2sMerkleChannel>(
                prover_input,
                proof_params,
                precomputed,
            )?;
            if args.verify {
                verify_cairo::<Blake2sMerkleChannel>(cairo_proof.clone().into())?;
            }
            serialize_proof_to_file(&cairo_proof, &args.proof_path, args.proof_format)?;
            Ok(())
        }
        #[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
        ChannelHash::Poseidon252 => {
            unimplemented!("Poseidon252 is not supported for wasm targets");
        }
        #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
        ChannelHash::Poseidon252 => {
            use stwo::core::vcs_lifted::poseidon252_merkle::Poseidon252MerkleChannel;
            let cairo_proof = prove_cairo_with_precomputed::<Poseidon252MerkleChannel>(
                prover_input,
                proof_params,
                precomputed,
            )?;
            if args.verify {
                verify_cairo::<Poseidon252MerkleChannel>(cairo_proof.clone().into())?;
            }
            serialize_proof_to_file(&cairo_proof, &args.proof_path, args.proof_format)?;
            Ok(())
        }
    };

    match result {
        Ok(_) => log::info!("✅ Proved successfully!"),
        Err(ref e) => log::error!("❌ Proving failed: {e:?}"),
    }

    result
}
