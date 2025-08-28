use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use bzip2::read::BzDecoder;
use bzip2::write::BzEncoder;
use bzip2::Compression;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use stwo::core::vcs::blake2_hash::Blake2sHasher;
use stwo::core::vcs::MerkleHasher;
use stwo_cairo_serialize::{CairoDeserialize, CairoSerialize};
use tracing::{span, Level};

use crate::air::{MemorySection, PublicMemory};
use crate::CairoProof;

#[cfg(test)]
mod tests;

/// 2^31, used for encoding small felt252 values.
const MSB_U32: u32 = 0x80000000;

/// Cairo proof format
#[derive(Debug, Clone, clap::ValueEnum)]
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
            let serialized_bytes = bincode::serialize(proof)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

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
            sonic_rs::from_str(&proof_str)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
        }
        ProofFormat::CairoSerde => {
            let proof_str = std::fs::read_to_string(proof_path)?;
            let felts: Vec<starknet_ff::FieldElement> = sonic_rs::from_str(&proof_str)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
            Ok(CairoDeserialize::deserialize(&mut felts.iter()))
        }
        ProofFormat::Binary => {
            let proof_file = File::open(proof_path)?;
            let mut proof_bytes = Vec::new();
            let mut bz_decoder = BzDecoder::new(proof_file);
            bz_decoder.read_to_end(&mut proof_bytes)?;
            bincode::deserialize(&proof_bytes)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
        }
    }
}

/// The data associated with the Cairo proof.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationOutput {
    /// Program hash.
    pub program_hash: starknet_ff::FieldElement,
    /// Public output.
    pub output: Vec<starknet_ff::FieldElement>,
}

/// Extract program hash (blake2s) and public output from the public memory.
pub fn get_verification_output(public_memory: &PublicMemory) -> VerificationOutput {
    let program_hash = construct_f252(&encode_and_hash_memory_section(&public_memory.program));
    let output = public_memory
        .output
        .iter()
        .map(|(_, entry)| construct_f252(entry))
        .collect();
    VerificationOutput {
        program_hash,
        output,
    }
}

/// Encodes a memory section and hashes it using Cairo blake.
pub fn encode_and_hash_memory_section(section: &MemorySection) -> [u32; 8] {
    let mut hasher = Blake2sHasher::new();
    for entry in section {
        let (_, val) = *entry;
        let limbs = encode_felt_in_limbs(val);
        for limb in limbs {
            // Cairo blake uses little-endian byte order for the input.
            hasher.update(&limb.to_le_bytes());
        }
    }
    let digest_bytes = hasher.finalize().0.to_vec();

    // Cairo blake uses little-endian byte order for the output as well, so we need to reverse each
    // 4-byte limb.
    let limbs: Vec<u32> = digest_bytes
        .chunks_exact(4)
        .map(|l| u32::from_le_bytes(l.try_into().unwrap()))
        .collect();

    limbs.try_into().unwrap()
}

/// Convert digest to a field element, adding every limb to the result (shifted) to reduce modulo
/// stark prime.
pub fn construct_f252(limbs: &[u32; 8]) -> starknet_ff::FieldElement {
    let mut result = starknet_ff::FieldElement::ZERO;
    let offset = starknet_ff::FieldElement::from(0x100000000_u64);
    for limb in limbs.iter().rev() {
        result = result * offset + (*limb).into();
    }
    result
}

/// Encodes a felt, represented by 8 u32 limbs in little-endian order and returns the encoded result
/// in big-endian order.
///
/// The encoding is done in the following way:
/// * If the felt is less than 2^63, it's encoded as the 2 least significant limbs.
/// * Otherwise, it's encoded as the 8 limbs, where the most significant limb has its MSB set (Note
///   that the prime is less than 2^255 so the MSB could not be set prior to this intervention).
pub fn encode_felt_in_limbs(felt: [u32; 8]) -> Vec<u32> {
    let [v0, v1, v2, v3, v4, v5, v6, v7] = felt;
    if v2 == 0 && v3 == 0 && v4 == 0 && v5 == 0 && v6 == 0 && v7 == 0 && v1 < MSB_U32 {
        vec![v1, v0]
    } else {
        vec![v7 + MSB_U32, v6, v5, v4, v3, v2, v1, v0]
    }
}
