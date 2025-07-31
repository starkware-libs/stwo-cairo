use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use stwo::core::channel::MerkleChannel;
use stwo::core::compact_binary::CompactBinary;
use stwo::core::vcs::MerkleHasher;
use stwo_cairo_serialize::{CairoDeserialize, CairoSerialize};
use tracing::{span, Level};

use crate::CairoProof;

/// Cairo proof format
#[derive(Debug, Clone, clap::ValueEnum)]
pub enum ProofFormat {
    /// Standard JSON format.
    Json,
    /// Array of field elements serialized as hex strings.
    /// Compatible with `scarb execute`
    CairoSerde,
    /// Compact binary format.
    CompactBinary,
}

/// Serializes Cairo proof given the desired format and writes it to a file.
pub fn serialize_proof_to_file<MC: MerkleChannel>(
    proof: &CairoProof<MC::H>,
    proof_path: PathBuf,
    proof_format: ProofFormat,
) -> Result<(), std::io::Error>
where
    MC::H: Serialize,
    <MC::H as MerkleHasher>::Hash: CairoSerialize + CompactBinary,
{
    let span = span!(Level::INFO, "Serialize proof").entered();

    let mut proof_file = File::create(&proof_path)?;

    match proof_format {
        ProofFormat::Json => {
            fs::write(&proof_path, serde_json::to_string(&proof)?)?;
        }
        ProofFormat::CairoSerde => {
            let mut serialized: Vec<starknet_ff::FieldElement> = Vec::new();
            CairoSerialize::serialize(proof, &mut serialized);
            fs::write(&proof_path, serde_json::to_string(&serialized)?)?;
        }
        ProofFormat::CompactBinary => {
            let mut compact_bytes: Vec<u8> = Vec::new();
            CompactBinary::compact_serialize(proof, &mut compact_bytes);
            proof_file.write_all(&compact_bytes)?;
        }
    }

    span.exit();
    Ok(())
}

/// Deserializes Cairo proof given the desired format from a file.
pub fn deserialize_proof_from_bytes<'a, MC: MerkleChannel>(
    bytes: &'a [u8],
    proof_format: ProofFormat,
) -> Result<CairoProof<MC::H>, std::io::Error>
where
    MC::H: Serialize + Deserialize<'a>,
    <MC::H as MerkleHasher>::Hash: CairoDeserialize + CairoSerialize + CompactBinary,
{
    let cairo_proof = match proof_format {
        ProofFormat::Json => serde_json::from_slice(bytes).unwrap(),
        ProofFormat::CairoSerde => {
            let json: Vec<String> = serde_json::from_slice(bytes).unwrap();

            let felts = json
                .iter()
                .map(|s| {
                    let s = s.strip_prefix("0x").unwrap();
                    starknet_ff::FieldElement::from_hex_be(s)
                        .expect("Failed to parse field element")
                })
                .collect::<Vec<_>>();

            <CairoProof<<MC as MerkleChannel>::H> as stwo_cairo_serialize::CairoDeserialize>::deserialize(&mut felts.iter())
        }
        ProofFormat::CompactBinary => {
            let (_, cairo_proof) = CairoProof::<MC::H>::compact_deserialize(bytes);
            cairo_proof
        }
    };

    Ok(cairo_proof)
}
