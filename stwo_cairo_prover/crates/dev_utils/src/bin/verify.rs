use std::path::{Path, PathBuf};

use cairo_air::verifier::verify_cairo;
use cairo_air::{CairoProof, PreProcessedTraceVariant};
use clap::Parser;
use dev_utils::utils::Error;
use stwo_cairo_prover::prover::ChannelHash;
use stwo_prover::core::vcs::blake2_merkle::{Blake2sMerkleChannel, Blake2sMerkleHasher};
use stwo_prover::core::vcs::poseidon252_merkle::{
    Poseidon252MerkleChannel, Poseidon252MerkleHasher,
};
use tracing::{span, Level};
use tracing_subscriber::fmt::format::FmtSpan;

/// CLI tool to verify Cairo proofs
///
/// Example usage:
///     ```
///     cargo run --bin verify -- --proof_path path/to/proof.json \
///     --channel_hash blake2s --pp_trace no_pedersen
///     ```
#[derive(Parser, Debug)]
struct Args {
    /// Path to the JSON file containing the proof to verify
    #[arg(long = "proof_path")]
    proof_path: PathBuf,

    /// Hash variant to use for verification (blake2s or poseidon252)
    #[arg(long = "channel_hash", default_value = "blake2s")]
    channel_hash: String,

    /// Preprocessed trace variant (canonical or no_pedersen)
    #[arg(long = "pp_trace", default_value = "canonical")]
    pp_trace: String,
}

fn parse_channel_hash(hash_str: &str) -> Result<ChannelHash, Error> {
    match hash_str.to_lowercase().as_str() {
        "blake2s" => Ok(ChannelHash::Blake2s),
        "poseidon252" => Ok(ChannelHash::Poseidon252),
        _ => Err(Error::Cli(clap::Error::raw(
            clap::error::ErrorKind::InvalidValue,
            format!(
                "Invalid channel hash: {}. Must be 'blake2s' or 'poseidon252'",
                hash_str
            ),
        ))),
    }
}

fn parse_preprocessed_trace(preprocessed_trace: &str) -> PreProcessedTraceVariant {
    match preprocessed_trace {
        "canonical" => PreProcessedTraceVariant::Canonical,
        "no_pedersen" => PreProcessedTraceVariant::CanonicalWithoutPedersen,
        _ => panic!(
            "Invalid preprocessed trace: {}, must be 'canonical' or 'no_pedersen'",
            preprocessed_trace
        ),
    }
}

fn verify_proof(
    proof_path: &Path,
    channel: ChannelHash,
    preprocessed_trace: String,
) -> Result<(), Error> {
    let proof = std::fs::read_to_string(proof_path)?;
    let preprocessed_trace = parse_preprocessed_trace(&preprocessed_trace);
    match channel {
        ChannelHash::Blake2s => verify_blake2s_proof(proof, preprocessed_trace),
        ChannelHash::Poseidon252 => verify_poseidon252_proof(proof, preprocessed_trace),
    }
}

fn verify_blake2s_proof(
    proof: String,
    preprocessed_trace: PreProcessedTraceVariant,
) -> Result<(), Error> {
    let proof: CairoProof<Blake2sMerkleHasher> = sonic_rs::from_str(&proof)?;
    let pcs_config = proof.stark_proof.config;
    verify_cairo::<Blake2sMerkleChannel>(proof, pcs_config, preprocessed_trace)?;
    Ok(())
}

fn verify_poseidon252_proof(
    proof: String,
    preprocessed_trace: PreProcessedTraceVariant,
) -> Result<(), Error> {
    let proof: CairoProof<Poseidon252MerkleHasher> = sonic_rs::from_str(&proof)?;
    let pcs_config = proof.stark_proof.config;
    verify_cairo::<Poseidon252MerkleChannel>(proof, pcs_config, preprocessed_trace)?;
    Ok(())
}

fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
        .init();

    let _span = span!(Level::INFO, "verify").entered();
    let args = Args::try_parse_from(std::env::args())?;

    log::info!("Verifying a {:?} proof", args.channel_hash);
    log::info!("Proof path: {}", args.proof_path.display());

    verify_proof(
        &args.proof_path,
        parse_channel_hash(&args.channel_hash)?,
        args.pp_trace,
    )?;

    log::info!("✅ Proof verified successfully!");
    Ok(())
}
