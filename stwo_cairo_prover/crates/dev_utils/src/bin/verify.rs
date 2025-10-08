use std::path::PathBuf;

use anyhow::Result;
use cairo_air::verifier::verify_cairo;
use cairo_air::{CairoProof, PreProcessedTraceVariant};
use clap::Parser;
use stwo::core::vcs::blake2_merkle::{Blake2sMerkleChannel, Blake2sMerkleHasher};
use stwo::core::vcs::poseidon252_merkle::{Poseidon252MerkleChannel, Poseidon252MerkleHasher};
use stwo_cairo_prover::prover::ChannelHash;
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

fn parse_channel_hash(hash_str: &str) -> Result<ChannelHash> {
    match hash_str.to_lowercase().as_str() {
        "blake2s" => Ok(ChannelHash::Blake2s),
        "poseidon252" => Ok(ChannelHash::Poseidon252),
        _ => anyhow::bail!("Invalid channel hash: {hash_str}. Must be 'blake2s' or 'poseidon252'"),
    }
}

fn parse_preprocessed_trace(preprocessed_trace: &str) -> PreProcessedTraceVariant {
    match preprocessed_trace {
        "canonical" => PreProcessedTraceVariant::Canonical,
        "no_pedersen" => PreProcessedTraceVariant::CanonicalWithoutPedersen,
        _ => panic!(
            "Invalid preprocessed trace: {preprocessed_trace}, must be 'canonical' or 'no_pedersen'"
        ),
    }
}

fn verify_blake2s_proof(proof: String, preprocessed_trace: PreProcessedTraceVariant) -> Result<()> {
    let proof: CairoProof<Blake2sMerkleHasher> = sonic_rs::from_str(&proof)?;
    verify_cairo::<Blake2sMerkleChannel>(proof, preprocessed_trace)?;
    Ok(())
}

fn verify_poseidon252_proof(
    proof: String,
    preprocessed_trace: PreProcessedTraceVariant,
) -> Result<()> {
    let proof: CairoProof<Poseidon252MerkleHasher> = sonic_rs::from_str(&proof)?;
    verify_cairo::<Poseidon252MerkleChannel>(proof, preprocessed_trace)?;
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();
    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
        .init();
    let _span = span!(Level::INFO, "verify").entered();

    log::info!("Verifying a {:?} proof", args.channel_hash);
    log::info!("Proof path: {}", args.proof_path.display());

    let proof = std::fs::read_to_string(&args.proof_path)?;
    let channel = parse_channel_hash(&args.channel_hash)?;
    let preprocessed_trace = parse_preprocessed_trace(&args.pp_trace);

    let result = match channel {
        ChannelHash::Blake2s => verify_blake2s_proof(proof, preprocessed_trace),
        ChannelHash::Poseidon252 => verify_poseidon252_proof(proof, preprocessed_trace),
    };
    match result {
        Ok(_) => log::info!("✅ Proof verified successfully!"),
        Err(ref e) => log::error!("❌ Proof verification failed: {e:?}"),
    }

    result
}
