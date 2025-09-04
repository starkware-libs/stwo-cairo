#[cfg(feature = "slow-tests")]
use {
    crate::utils::{deserialize_proof_from_file, serialize_proof_to_file, ProofFormat},
    dev_utils::utils::get_proof_file_path,
    stwo::core::vcs::blake2_merkle::Blake2sMerkleHasher,
    tempfile::NamedTempFile,
};

use crate::utils::{construct_f252, encode_and_hash_memory_section, encode_felt_in_limbs};

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

#[cfg(feature = "slow-tests")]
#[test]
fn test_serialize_and_deserialize_proof() {
    let proof_path = get_proof_file_path("test_prove_verify_all_opcode_components");
    let mut proof =
        deserialize_proof_from_file::<Blake2sMerkleHasher>(&proof_path, ProofFormat::CairoSerde)
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
    let final_json =
        std::fs::read_to_string(temp_serde_file.path()).expect("Failed to read final proof file");
    let original_json =
        std::fs::read_to_string(&proof_path).expect("Failed to read original proof file");

    assert_eq!(
        final_json, original_json,
        "Final serialized proof should match the original proof"
    );
}
