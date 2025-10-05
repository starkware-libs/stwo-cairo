use std::fs::{read_to_string, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use anyhow::Result;
use bzip2::read::BzDecoder;
use bzip2::write::BzEncoder;
use bzip2::Compression;
use cairo_air::verifier::verify_cairo;
use cairo_air::{CairoProof, PreProcessedTraceVariant};
use clap::ValueEnum;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use sonic_rs::from_str;
use stwo::core::fri::FriConfig;
use stwo::core::pcs::PcsConfig;
use stwo::core::vcs::blake2_merkle::Blake2sMerkleChannel;
use stwo::core::vcs::poseidon252_merkle::Poseidon252MerkleChannel;
use stwo::core::vcs::MerkleHasher;
use stwo_cairo_adapter::ProverInput;
use stwo_cairo_serialize::{CairoDeserialize, CairoSerialize};
use tracing::{span, Level};

use crate::prover::prove_cairo;

/// Cairo proof format
#[derive(Debug, Clone, ValueEnum)]
pub enum ProofFormat {
    /// Standard JSON format.
    Json,
    /// Array of field elements serialized as hex strings.
    /// Compatible with `scarb execute`
    CairoSerde,
    /// Binary format.
    /// Additionally compressed to minimize the proof size.
    Binary,
}

/// Concrete parameters of the proving system.
/// Used both for producing and verifying proofs.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ProverParameters {
    /// Channel hash function.
    pub channel_hash: ChannelHash,
    /// Parameters of the commitment scheme.
    pub pcs_config: PcsConfig,
    /// Preprocessed trace.
    pub preprocessed_trace: PreProcessedTraceVariant,
}

/// The hash function used for commitments, for the prover-verifier channel,
/// and for PoW grinding.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChannelHash {
    /// Default variant, the fastest option.
    Blake2s,
    /// A variant for recursive proof verification.
    /// Note that using `Poseidon252` results in a significant decrease in proving speed compared
    /// to `Blake2s` (because of the large field emulation)
    Poseidon252,
}

/// Generates proof given the Cairo VM output and prover config/parameters.
/// Serializes the proof as JSON and write to the output path.
/// Verifies the proof in case the respective flag is set.
pub fn create_and_serialize_proof(
    input: ProverInput,
    verify: bool,
    proof_path: PathBuf,
    proof_format: ProofFormat,
    proof_params_json: Option<PathBuf>,
) -> Result<()> {
    let proof_params = if let Some(proof_params_json) = proof_params_json {
        from_str(&read_to_string(&proof_params_json)?)?
    } else {
        // The default prover parameters for prod use (96 bits of security).
        // The formula is `security_bits = pow_bits + log_blowup_factor * n_queries`.
        ProverParameters {
            channel_hash: ChannelHash::Blake2s,
            pcs_config: PcsConfig {
                // Stay within 500ms on M3.
                pow_bits: 26,
                fri_config: FriConfig {
                    log_last_layer_degree_bound: 0,
                    // Blowup factor > 1 significantly degrades proving speed.
                    // Can be in range [1, 16].
                    log_blowup_factor: 1,
                    // The more FRI queries, the larger the proof.
                    // Proving time is not affected much by increasing this value.
                    n_queries: 70,
                },
            },
            preprocessed_trace: PreProcessedTraceVariant::Canonical,
        }
    };

    match proof_params.channel_hash {
        ChannelHash::Blake2s => {
            let proof = prove_cairo::<Blake2sMerkleChannel>(
                input,
                proof_params.pcs_config,
                proof_params.preprocessed_trace,
            )?;
            serialize_proof_to_file(&proof, &proof_path, proof_format)?;
            if verify {
                verify_cairo::<Blake2sMerkleChannel>(proof, proof_params.preprocessed_trace)?;
            }
        }
        ChannelHash::Poseidon252 => {
            let proof = prove_cairo::<Poseidon252MerkleChannel>(
                input,
                proof_params.pcs_config,
                proof_params.preprocessed_trace,
            )?;
            serialize_proof_to_file(&proof, &proof_path, proof_format)?;
            if verify {
                verify_cairo::<Poseidon252MerkleChannel>(proof, proof_params.preprocessed_trace)?;
            }
        }
    };

    Ok(())
}

/// Serializes Cairo proof given the desired format and writes it to a file.
pub fn serialize_proof_to_file<H: MerkleHasher + Serialize>(
    proof: &CairoProof<H>,
    proof_path: &Path,
    proof_format: ProofFormat,
) -> Result<(), std::io::Error>
where
    H::Hash: CairoSerialize,
{
    let span = span!(Level::INFO, "Serialize proof").entered();

    let mut proof_file = File::create(proof_path)?;

    match proof_format {
        ProofFormat::Json => {
            proof_file.write_all(sonic_rs::to_string_pretty(proof)?.as_bytes())?;
        }
        ProofFormat::CairoSerde => {
            let mut serialized: Vec<starknet_ff::FieldElement> = Vec::new();
            CairoSerialize::serialize(proof, &mut serialized);

            let hex_strings: Vec<String> = serialized
                .into_iter()
                .map(|felt| format!("0x{felt:x}"))
                .collect();

            proof_file.write_all(sonic_rs::to_string_pretty(&hex_strings)?.as_bytes())?;
        }
        ProofFormat::Binary => {
            let serialized_bytes = bincode::serialize(proof).map_err(std::io::Error::other)?;

            let mut bz_encoder = BzEncoder::new(proof_file, Compression::best());
            bz_encoder.write_all(&serialized_bytes)?;
            bz_encoder.finish()?;
        }
    }

    span.exit();
    Ok(())
}

/// Deserializes Cairo proof from a file given the desired format.
pub fn deserialize_proof_from_file<H: MerkleHasher + DeserializeOwned>(
    proof_path: &Path,
    proof_format: ProofFormat,
) -> Result<CairoProof<H>, std::io::Error>
where
    H::Hash: CairoDeserialize,
{
    match proof_format {
        ProofFormat::Json => {
            let proof_str = std::fs::read_to_string(proof_path)?;
            sonic_rs::from_str(&proof_str).map_err(std::io::Error::other)
        }
        ProofFormat::CairoSerde => {
            let proof_str = std::fs::read_to_string(proof_path)?;
            let felts: Vec<starknet_ff::FieldElement> =
                sonic_rs::from_str(&proof_str).map_err(std::io::Error::other)?;
            Ok(CairoDeserialize::deserialize(&mut felts.iter()))
        }
        ProofFormat::Binary => {
            let proof_file = File::open(proof_path)?;
            let mut proof_bytes = Vec::new();
            let mut bz_decoder = BzDecoder::new(proof_file);
            bz_decoder.read_to_end(&mut proof_bytes)?;
            bincode::deserialize(&proof_bytes).map_err(std::io::Error::other)
        }
    }
}

#[cfg(test)]
#[cfg(feature = "slow-tests")]
mod tests {
    use dev_utils::utils::get_proof_file_path;
    use stwo::core::vcs::blake2_merkle::Blake2sMerkleHasher;
    use tempfile::NamedTempFile;

    use super::*;

    #[test]
    fn test_serialize_and_deserialize_proof() {
        let proof_path = get_proof_file_path("test_prove_verify_all_opcode_components");
        let mut proof = deserialize_proof_from_file::<Blake2sMerkleHasher>(
            &proof_path,
            ProofFormat::CairoSerde,
        )
        .expect("Failed to deserialize proof (CairoSerde)");

        let temp_json_file = NamedTempFile::new().expect("Failed to create temp file");
        serialize_proof_to_file::<Blake2sMerkleHasher>(
            &proof,
            temp_json_file.path(),
            ProofFormat::Json,
        )
        .expect("Failed to serialize proof (Json)");

        proof = deserialize_proof_from_file::<Blake2sMerkleHasher>(
            temp_json_file.path(),
            ProofFormat::Json,
        )
        .expect("Failed to deserialize proof (Json)");

        let temp_binary_file = NamedTempFile::new().expect("Failed to create temp file");
        serialize_proof_to_file::<Blake2sMerkleHasher>(
            &proof,
            temp_binary_file.path(),
            ProofFormat::Binary,
        )
        .expect("Failed to serialize proof (Binary)");

        proof = deserialize_proof_from_file::<Blake2sMerkleHasher>(
            temp_binary_file.path(),
            ProofFormat::Binary,
        )
        .expect("Failed to deserialize proof (Binary)");

        let temp_serde_file = NamedTempFile::new().expect("Failed to create temp file");
        serialize_proof_to_file::<Blake2sMerkleHasher>(
            &proof,
            temp_serde_file.path(),
            ProofFormat::CairoSerde,
        )
        .expect("Failed to serialize proof (CairoSerde)");

        // Verify the final serialized proof matches the original by comparing JSON strings
        let final_json = std::fs::read_to_string(temp_serde_file.path())
            .expect("Failed to read final proof file");
        let original_json =
            std::fs::read_to_string(&proof_path).expect("Failed to read original proof file");

        assert_eq!(
            final_json, original_json,
            "Final serialized proof should match the original proof"
        );
    }
}
