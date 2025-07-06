use std::fs;
use std::path::PathBuf;

use cairo_air::CairoProof;
use clap::Parser;
use regex::Regex;
use serde::Serialize;
use starknet_ff::FieldElement;
use stwo_cairo_serialize::CairoDeserialize;
use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleHasher;
use stwo_prover::core::vcs::poseidon252_merkle::Poseidon252MerkleHasher;
use stwo_prover::core::vcs::MerkleHasher;

#[derive(Parser, Debug)]
#[command(
    name = "convert_proof_format",
    about = "Converts a proof from Cairo-serde format to JSON format"
)]
struct Args {
    /// Input proof file in Cairo-serde format (JSON array of hex strings).
    #[arg(long, short)]
    input: PathBuf,

    /// Output proof file in JSON format.
    #[arg(long, short)]
    output: PathBuf,

    /// Hash function used in the proof (blake2s or poseidon252).
    #[arg(long, default_value = "blake2s")]
    hash: String,
}

fn convert_proof<H: MerkleHasher + Serialize>(
    input_path: &PathBuf,
    output_path: &PathBuf,
) -> Result<(), std::io::Error>
where
    H::Hash: CairoDeserialize,
{
    let values = read_to_felts(input_path)?;
    let proof: CairoProof<H> = CairoDeserialize::deserialize(&mut values.iter());
    let json_content = sonic_rs::to_string_pretty(&proof)?;
    fs::write(output_path, json_content)?;

    println!("Successfully converted proof from Cairo-serde to JSON format");
    Ok(())
}

fn read_to_felts(input_path: &PathBuf) -> Result<Vec<FieldElement>, std::io::Error> {
    let content = fs::read_to_string(input_path)?;
    let re = Regex::new(r#""(0x[0-9A-Fa-f]+)""#).unwrap();
    let values: Vec<FieldElement> = re
        .captures_iter(content.as_str())
        .filter_map(|cap| {
            cap.get(1)
                .map(|m| FieldElement::from_hex_be(m.as_str()).unwrap())
        })
        .collect();
    Ok(values)
}

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();

    println!(
        "Converting proof from {} to {}",
        args.input.display(),
        args.output.display()
    );

    match args.hash.as_str() {
        "blake2s" => convert_proof::<Blake2sMerkleHasher>(&args.input, &args.output)?,
        "poseidon252" => convert_proof::<Poseidon252MerkleHasher>(&args.input, &args.output)?,
        _ => {
            panic!(
                "supported hash functions are blake2s and poseidon252, got {}",
                args.hash
            );
        }
    }

    Ok(())
}
