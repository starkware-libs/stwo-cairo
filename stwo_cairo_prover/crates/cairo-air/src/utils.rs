use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use bzip2::read::BzDecoder;
use bzip2::write::BzEncoder;
use bzip2::Compression;
use clap::ValueEnum;
use itertools::Itertools;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use stwo::core::fields::m31::BaseField;
use stwo::core::fields::qm31::SECURE_EXTENSION_DEGREE;
use stwo::core::pcs::TreeVec;
use stwo::core::vcs::blake2_hash::Blake2sHasher;
use stwo::core::vcs_lifted::MerkleHasherLifted;
use stwo::core::verifier::COMPOSITION_LOG_SPLIT;
use stwo_cairo_serialize::{CairoDeserialize, CairoSerialize};
use stwo_constraint_framework::{INTERACTION_TRACE_IDX, PREPROCESSED_TRACE_IDX};
use tracing::{span, Level};

use crate::air::{CairoProofSorted, MemorySection, PublicMemory};

mod json {
    #[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
    pub use serde_json::{from_str, to_string_pretty};
    #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
    pub use sonic_rs::{from_str, to_string_pretty};
}

/// 2^31, used for encoding small felt252 values.
const MSB_U32: u32 = 0x80000000;

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

/// Serializes Cairo proof given the desired format and writes it to a file.
pub fn serialize_proof_to_file<H: MerkleHasherLifted + Serialize>(
    proof: &CairoProofSorted<H>,
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
            proof_file.write_all(json::to_string_pretty(proof)?.as_bytes())?;
        }
        ProofFormat::CairoSerde => {
            let mut serialized: Vec<starknet_ff::FieldElement> = Vec::new();
            CairoSerialize::serialize(proof, &mut serialized);

            let hex_strings: Vec<String> = serialized
                .into_iter()
                .map(|felt| format!("0x{felt:x}"))
                .collect();

            proof_file.write_all(json::to_string_pretty(&hex_strings)?.as_bytes())?;
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
pub fn deserialize_proof_from_file<H: MerkleHasherLifted + DeserializeOwned>(
    proof_path: &Path,
    proof_format: ProofFormat,
) -> Result<CairoProofSorted<H>, std::io::Error>
where
    H::Hash: CairoDeserialize,
{
    match proof_format {
        ProofFormat::Json => {
            let proof_str = std::fs::read_to_string(proof_path)?;
            json::from_str(&proof_str).map_err(std::io::Error::other)
        }
        ProofFormat::CairoSerde => {
            let proof_str = std::fs::read_to_string(proof_path)?;
            let felts: Vec<starknet_ff::FieldElement> =
                json::from_str(&proof_str).map_err(std::io::Error::other)?;
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

/// A utility function which transforms the order and layout of the queried values of a stwo proof
/// according to the format expected by the Cairo verifier.
pub fn sort_and_transpose_queried_values(
    queried_values: &TreeVec<Vec<Vec<BaseField>>>,
    preprocessed_trace_log_sizes: &[u32],
    trace_and_interaction_trace_log_sizes: Vec<&[u32]>,
) -> TreeVec<Vec<BaseField>> {
    debug_assert!(trace_and_interaction_trace_log_sizes[PREPROCESSED_TRACE_IDX].is_empty());
    debug_assert!(trace_and_interaction_trace_log_sizes.len() == 3);
    // `all_log_sizes` contains the column log sizes for the preprocessed trace, base trace and
    // interaction trace.
    let mut all_log_sizes = trace_and_interaction_trace_log_sizes;
    all_log_sizes[0] = preprocessed_trace_log_sizes;

    let mut new_queried_values_per_tree = vec![];
    let n_queries = queried_values[0][0].len();
    // Sort and transpose the queried values for interaction phases 0, 1, 2, i.e. preprocessed, base
    // trace, interaction trace.
    for (queried_values, col_sizes) in queried_values[..=INTERACTION_TRACE_IDX]
        .iter()
        .zip_eq(all_log_sizes.iter())
    {
        let mut new_queried_values = vec![];
        let mut sorted_queries: Vec<_> = queried_values
            .iter()
            .zip_eq(col_sizes.iter())
            .sorted_by_key(|(_, col_size)| *col_size)
            .map(|(vals, _)| vals.iter())
            .collect();
        for _ in 0..n_queries {
            new_queried_values.extend(
                sorted_queries
                    .iter_mut()
                    .map(|col_iter| *col_iter.next().unwrap()),
            );
        }
        new_queried_values_per_tree.push(new_queried_values)
    }

    // Sort and transpose the queried values of the composition polynomial commitment. All columns
    // in the composition commitment are of the same length so there is no need to sort.
    let composition_queried_values = &queried_values.last().unwrap();
    let mut new_queried_values: Vec<BaseField> = vec![];
    for row_idx in 0..n_queries {
        new_queried_values.extend(composition_queried_values.iter().map(|vals| vals[row_idx]));
    }
    new_queried_values_per_tree.push(new_queried_values);
    TreeVec(new_queried_values_per_tree)
}

/// A utility function which transforms the order and layout of the queried values from the format
/// expected by the Cairo verifier to the format expected by the stwo verifier. This transformation
/// is the inverse transformation of [`sort_and_transpose_queried_values`].
pub fn unsort_and_transpose_queried_values(
    queried_values_per_tree: &TreeVec<Vec<BaseField>>,
    preprocessed_trace_log_sizes: &[u32],
    trace_and_interaction_trace_log_sizes: Vec<&[u32]>,
) -> TreeVec<Vec<Vec<BaseField>>> {
    debug_assert!(trace_and_interaction_trace_log_sizes[PREPROCESSED_TRACE_IDX].is_empty());
    debug_assert!(trace_and_interaction_trace_log_sizes.len() == 3);
    // `all_log_sizes` contains the column log sizes for the preprocessed trace, base trace and
    // interaction trace.
    let mut all_log_sizes = trace_and_interaction_trace_log_sizes;
    all_log_sizes[0] = preprocessed_trace_log_sizes;

    // For each tree of until (and including) the interaction trace tree, change the order of the
    // of the queried values: from ascending by column size to column index.
    let mut new_queried_values_per_tree = vec![];
    for (queried_values, col_sizes) in queried_values_per_tree[..=INTERACTION_TRACE_IDX]
        .iter()
        .zip_eq(all_log_sizes.iter())
    {
        let n_cols = col_sizes.len();
        let mut new_queried_values = vec![];

        for i in 0..n_cols {
            new_queried_values.push(
                queried_values
                    .iter()
                    .skip(i)
                    .step_by(n_cols)
                    .copied()
                    .collect(),
            )
        }
        let permutation: Vec<usize> = col_sizes
            .iter()
            .enumerate()
            .sorted_by_key(|(_, log_size)| *log_size)
            .map(|(i, _)| i)
            .collect();
        new_queried_values_per_tree.push(
            new_queried_values
                .into_iter()
                .enumerate()
                .sorted_by_key(|(i, _)| permutation[*i])
                .map(|(_, vals)| vals)
                .collect(),
        );
    }

    // For the composition polynomial commitment, all the columns have the same size so we don't
    // need to compute the sorting permutation. The number of columns in this tree is
    // 2^{COMPOSITION_LOG_SPLIT} * 4.
    let queried_values = queried_values_per_tree.last().unwrap();
    let n_cols = (1 << COMPOSITION_LOG_SPLIT) * SECURE_EXTENSION_DEGREE;
    let mut new_queried_values = vec![];
    for i in 0..n_cols {
        new_queried_values.push(
            queried_values
                .iter()
                .skip(i)
                .step_by(n_cols)
                .copied()
                .collect(),
        )
    }
    new_queried_values_per_tree.push(new_queried_values);

    TreeVec(new_queried_values_per_tree)
}

#[cfg(test)]
mod tests {
    use rand::rngs::SmallRng;
    use rand::{Rng, SeedableRng};
    use stwo::core::fields::m31::M31;

    use super::*;

    #[test]
    fn test_encode_felt_in_limbs() {
        let felt0 = [0x12345678, 0x70000000, 0, 0, 0, 0, 0, 0];
        let felt1 = [
            0x12345678, 0x90abcdef, 0xabcdef12, 0x34567890, 0x01234567, 0x89abcdef, 0x01234567, 0,
        ];
        let limbs0 = encode_felt_in_limbs(felt0);
        let limbs1 = encode_felt_in_limbs(felt1);
        assert_eq!(limbs0, vec![1879048192, 305419896]);
        assert_eq!(
            limbs1,
            vec![
                2147483648, 19088743, 2309737967, 19088743, 878082192, 2882400018, 2427178479,
                305419896
            ]
        );
    }

    #[test]
    fn test_encode_and_hash_memory_section() {
        let memory_section = vec![
            (0, [0x12345678, 0x90abcdef, 0, 0, 0, 0, 0, 0]),
            (1, [0xabcdef12, 0x34567890, 0, 0, 0, 0, 0, 0]),
        ];
        let hash = encode_and_hash_memory_section(&memory_section);
        let expected = [
            2421522214_u32,
            635981307,
            2862863578,
            1664236125,
            1878536921,
            1607560013,
            4274188691,
            2957079540,
        ];
        assert_eq!(hash, expected);
    }

    #[test]
    fn test_construct_f252() {
        let limbs = [
            2421522214_u32,
            635981307,
            2862863578,
            1664236125,
            1878536921,
            1607560013,
            4274188691,
            2957079540,
        ];
        let expected = starknet_ff::FieldElement::from_dec_str(
            "115645365096977585374207223166120623839439046970571781411593222716768222992",
        )
        .unwrap();
        assert_eq!(construct_f252(&limbs), expected);
    }
    #[test]
    fn test_sort_unsort_are_inverses() {
        const N_COLS_PER_TREE: usize = 10;
        const N_QUERIES: usize = 7;
        let mut rng = SmallRng::seed_from_u64(0);

        let preprocessed_columns_log_sizes: Vec<_> =
            (0..N_COLS_PER_TREE).map(|_| rng.gen_range(1..20)).collect();
        let trace_and_interaction_trace_log_sizes = [
            vec![],
            (0..N_COLS_PER_TREE).map(|_| rng.gen_range(1..20)).collect(),
            (0..N_COLS_PER_TREE).map(|_| rng.gen_range(1..20)).collect(),
        ];
        let trace_and_interaction_trace_log_sizes: Vec<&[u32]> =
            trace_and_interaction_trace_log_sizes
                .iter()
                .map(|v| v.as_slice())
                .collect();
        let mut queried_vals = (0..3)
            .map(|_| {
                (0..N_COLS_PER_TREE * N_QUERIES)
                    .map(BaseField::from)
                    .collect()
            })
            .collect::<Vec<_>>();
        queried_vals.push((0..(8 * N_QUERIES)).map(BaseField::from).collect());
        let queried_vals = TreeVec(queried_vals);
        let sort_unsort_queried_vals = sort_and_transpose_queried_values(
            &unsort_and_transpose_queried_values(
                &queried_vals,
                &preprocessed_columns_log_sizes,
                trace_and_interaction_trace_log_sizes.clone(),
            ),
            &preprocessed_columns_log_sizes,
            trace_and_interaction_trace_log_sizes.clone(),
        );
        // Check that x = sort(unsort(x)).
        assert_eq!(queried_vals.0, sort_unsort_queried_vals.0);

        let queried_vals = unsort_and_transpose_queried_values(
            &queried_vals,
            &preprocessed_columns_log_sizes,
            trace_and_interaction_trace_log_sizes.clone(),
        );
        let unsort_sort_queried_vals = unsort_and_transpose_queried_values(
            &sort_and_transpose_queried_values(
                &queried_vals,
                &preprocessed_columns_log_sizes,
                trace_and_interaction_trace_log_sizes.clone(),
            ),
            &preprocessed_columns_log_sizes,
            trace_and_interaction_trace_log_sizes,
        );
        // Check that x = unsort(sort(x)).
        assert_eq!(queried_vals.0, unsort_sort_queried_vals.0);
    }

    #[test]
    fn test_sort_unsort() {
        let preprocessed_trace_log_sizes = vec![6, 5, 7];
        let trace_and_interaction_trace_log_sizes = [vec![], vec![4, 3, 2, 1], vec![4, 1, 3, 2]];
        let trace_and_interaction_trace_log_sizes: Vec<&[u32]> =
            trace_and_interaction_trace_log_sizes
                .iter()
                .map(|v| v.as_slice())
                .collect();
        let unsorted_queried_values = TreeVec(vec![
            vec![
                vec![M31::from(1), M31::from(2)],
                vec![M31::from(3), M31::from(4)],
                vec![M31::from(5), M31::from(6)],
            ],
            vec![
                vec![M31::from(1), M31::from(2)],
                vec![M31::from(3), M31::from(4)],
                vec![M31::from(5), M31::from(6)],
                vec![M31::from(7), M31::from(8)],
            ],
            vec![
                vec![M31::from(1), M31::from(2)],
                vec![M31::from(3), M31::from(4)],
                vec![M31::from(5), M31::from(6)],
                vec![M31::from(7), M31::from(8)],
            ],
            vec![vec![M31::from(1), M31::from(2)]; 8],
        ]);
        let sorted_queried_values = TreeVec(vec![
            vec![
                M31::from(3),
                M31::from(1),
                M31::from(5),
                M31::from(4),
                M31::from(2),
                M31::from(6),
            ],
            vec![
                M31::from(7),
                M31::from(5),
                M31::from(3),
                M31::from(1),
                M31::from(8),
                M31::from(6),
                M31::from(4),
                M31::from(2),
            ],
            vec![
                M31::from(3),
                M31::from(7),
                M31::from(5),
                M31::from(1),
                M31::from(4),
                M31::from(8),
                M31::from(6),
                M31::from(2),
            ],
            [[M31::from(1); 8], [M31::from(2); 8]].concat(),
        ]);

        assert_eq!(
            unsorted_queried_values.0,
            unsort_and_transpose_queried_values(
                &sorted_queried_values,
                &preprocessed_trace_log_sizes,
                trace_and_interaction_trace_log_sizes.clone()
            )
            .0
        );
        assert_eq!(
            sorted_queried_values.0,
            sort_and_transpose_queried_values(
                &unsorted_queried_values,
                &preprocessed_trace_log_sizes,
                trace_and_interaction_trace_log_sizes
            )
            .0
        );
    }
}

#[cfg(test)]
#[cfg(feature = "slow-tests")]
mod slow_tests {
    use dev_utils::utils::get_proof_file_path;
    use stwo::core::vcs_lifted::blake2_merkle::Blake2sMerkleHasher;
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
