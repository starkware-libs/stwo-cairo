#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use cairo_lang_casm::casm;
    use stwo_cairo_prover::cairo_air::{prove_cairo, verify_cairo, ProverConfig};
    use stwo_cairo_prover::input::plain::input_from_plain_casm_with_step_limit;
    use stwo_cairo_serialize::CairoSerialize;
    use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleChannel;
    use stwo_prover::core::vcs::poseidon252_merkle::Poseidon252MerkleChannel;

    pub fn project_root() -> PathBuf {
        std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
    }

    // TODO(Ohad): this is temporary, develop better automation.
    #[ignore = "slow, used to generate a proof"]
    #[test]
    fn generate_jrl0_proof() {
        let instructions = casm! {
            jmp rel 0;
        }
        .instructions;

        let input = input_from_plain_casm_with_step_limit(instructions, 14);
        let proof = prove_cairo::<Blake2sMerkleChannel>(input, ProverConfig::default()).unwrap();

        let path = project_root().join("artifacts/jrl0_proof.json");
        std::fs::write(path, serde_json::to_string(&proof).unwrap()).unwrap();

        verify_cairo::<Blake2sMerkleChannel>(proof).unwrap();
    }

    #[test]
    fn generate_generic_opcode_proof() {
        let instructions = casm! {
            jmp rel 0;
        }
        .instructions;

        let input = input_from_plain_casm_with_step_limit(instructions, 14);
        let proof =
            prove_cairo::<Poseidon252MerkleChannel>(input, ProverConfig::default()).unwrap();

        let path = project_root().join("artifacts/jrl0_proof.json");
        std::fs::write(path, serde_json::to_string(&proof).unwrap()).unwrap();

        let path = project_root().join("artifacts/proof.cairo");
        let mut proof_felts = Vec::new();
        CairoSerialize::serialize(&proof, &mut proof_felts);
        let cairo_proof = proof_felts
            .into_iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(",");
        std::fs::write(path, cairo_proof).unwrap();

        verify_cairo::<Poseidon252MerkleChannel>(proof).unwrap();
    }
}
