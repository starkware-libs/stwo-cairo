#[cfg(feature = "slow-tests")]
mod slow_tests {
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
