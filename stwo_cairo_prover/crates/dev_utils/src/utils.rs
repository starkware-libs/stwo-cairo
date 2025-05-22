use std::io::Write;
use std::path::PathBuf;

use cairo_air::verifier::{verify_cairo, CairoVerificationError};
use cairo_air::PreProcessedTraceVariant;
use serde::Serialize;
use stwo_cairo_adapter::vm_import::VmImportError;
use stwo_cairo_adapter::ProverInput;
use stwo_cairo_prover::prover::{
    default_prod_prover_parameters, prove_cairo, ChannelHash, ProverParameters,
};
use stwo_cairo_serialize::CairoSerialize;
use stwo_cairo_utils::file_utils::{create_file, read_to_string, IoErrorWithPath};
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::backend::BackendForChannel;
use stwo_prover::core::channel::MerkleChannel;
use stwo_prover::core::pcs::PcsConfig;
use stwo_prover::core::prover::ProvingError;
use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleChannel;
use stwo_prover::core::vcs::ops::MerkleHasher;
use stwo_prover::core::vcs::poseidon252_merkle::Poseidon252MerkleChannel;
use thiserror::Error;
use tracing::{span, Level};

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum ProofFormat {
    /// Standard JSON format.
    Json,
    /// Array of field elements serialized as hex strings.
    /// Compatible with `scarb execute`
    CairoSerde,
}

#[derive(Debug, Error)]
pub enum Error {
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

/// Generates proof given the Cairo VM output and prover config/parameters.
/// Serializes the proof as JSON and write to the output path.
/// Verifies the proof in case the respective flag is set.
pub fn create_and_serialize_generic_proof<MC: MerkleChannel>(
    input: ProverInput,
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
    let proof = prove_cairo::<MC>(input, pcs_config, preprocessed_trace)?;
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
                .map(|felt| format!("0x{:x}", felt))
                .collect();

            proof_file.write_all(sonic_rs::to_string_pretty(&hex_strings)?.as_bytes())?;
        }
    }
    span.exit();
    if verify {
        verify_cairo::<MC>(proof, pcs_config, preprocessed_trace)?;
        log::info!("Proof verified successfully");
    }

    Ok(())
}

pub fn create_and_serialize_proof(
    input: ProverInput,
    verify: bool,
    proof_path: PathBuf,
    proof_format: ProofFormat,
    proof_params_json: Option<PathBuf>,
) -> Result<(), Error> {
    let ProverParameters {
        channel_hash,
        pcs_config,
        preprocessed_trace,
    } = match proof_params_json {
        Some(path) => sonic_rs::from_str(&read_to_string(&path)?)?,
        None => default_prod_prover_parameters(),
    };

    let create_and_serialize_generic_proof: fn(
        ProverInput,
        PcsConfig,
        PreProcessedTraceVariant,
        bool,
        PathBuf,
        ProofFormat,
    ) -> Result<(), Error> = match channel_hash {
        ChannelHash::Blake2s => create_and_serialize_generic_proof::<Blake2sMerkleChannel>,
        ChannelHash::Poseidon252 => create_and_serialize_generic_proof::<Poseidon252MerkleChannel>,
    };

    create_and_serialize_generic_proof(
        input,
        pcs_config,
        preprocessed_trace,
        verify,
        proof_path,
        proof_format,
    )?;

    Ok(())
}
