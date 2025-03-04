#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use cairo_lang_casm::casm;
    use stwo_cairo_adapter::plain::input_from_plain_casm_with_step_limit;
    use stwo_cairo_prover::cairo_air::prover::{default_prod_prover_parameters, CairoProver};
    use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleChannel;

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
        let prover = CairoProver::new(default_prod_prover_parameters());
        let proof = prover.prove::<Blake2sMerkleChannel>(input).unwrap();

        let path = project_root().join("artifacts/jrl0_proof.json");
        std::fs::write(path, serde_json::to_string(&proof).unwrap()).unwrap();
    }
}
