use std::path::{Path, PathBuf};

use anyhow::Result;
use cairo_air::utils::{deserialize_proof_from_file, serialize_proof_to_file, ProofFormat};
use cairo_air::CairoProof;
use clap::Parser;
use serde::de::DeserializeOwned;
use serde::Serialize;
use stwo::core::vcs::blake2_merkle::Blake2sMerkleHasher;
use stwo::core::vcs::poseidon252_merkle::Poseidon252MerkleHasher;
use stwo::core::vcs::MerkleHasher;
use stwo_cairo_serialize::{CairoDeserialize, CairoSerialize};

#[derive(Parser, Debug)]
#[command(
    name = "convert_proof_format",
    about = "Converts a proof between the different possible formats, Cairo-serde, Json and Binary."
)]
struct Args {
    /// Path to the input proof file.
    #[arg(long, short)]
    input: PathBuf,

    /// Input proof format: `cairo-serde` (default), `json`, or `binary`.
    #[arg(long, default_value = "cairo_serde")]
    input_format: ProofFormat,

    /// Path to the output proof file.
    #[arg(long, short)]
    output: PathBuf,

    /// Output proof format: `cairo-serde`, `json` (default), or `binary`.
    #[arg(long, default_value = "json")]
    output_format: ProofFormat,

    /// Hash function used in the proof: `blake2s` (default) or `poseidon252`.
    #[arg(long, default_value = "blake2s")]
    hash: String,
}

fn convert_proof<H: MerkleHasher + Serialize + DeserializeOwned>(
    input_path: &Path,
    output_path: &Path,
    input_format: ProofFormat,
    output_format: ProofFormat,
) -> Result<(), std::io::Error>
where
    H::Hash: CairoSerialize + CairoDeserialize,
{
    let proof: CairoProof<H> = deserialize_proof_from_file(input_path, input_format)?;
    serialize_proof_to_file::<H>(&proof, output_path, output_format)?;

    println!("Successfully converted proof");
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!(
        "Converting proof from {} ({:?}) to {} ({:?})",
        args.input.display(),
        args.input_format,
        args.output.display(),
        args.output_format,
    );

    match args.hash.as_str() {
        "blake2s" => convert_proof::<Blake2sMerkleHasher>(
            &args.input,
            &args.output,
            args.input_format,
            args.output_format,
        )?,
        "poseidon252" => convert_proof::<Poseidon252MerkleHasher>(
            &args.input,
            &args.output,
            args.input_format,
            args.output_format,
        )?,
        _ => {
            anyhow::bail!(
                "supported hash functions are blake2s and poseidon252, got {}",
                args.hash
            )
        }
    }
    Ok(())
}
