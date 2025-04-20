#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use cairo_air::verifier::verify_cairo;
    use cairo_air::PreProcessedTraceVariant;
    use cairo_lang_casm::casm;
    use cairo_vm::hint_processor::builtin_hint_processor::builtin_hint_processor_definition::BuiltinHintProcessor;
    use cairo_vm::types::layout_name::LayoutName;
    use cairo_vm::vm::runners::cairo_runner::CairoRunner;
    use stwo_cairo_adapter::plain::{adapt_finished_runner, program_from_casm};
    use stwo_cairo_adapter::ProverInput;
    use stwo_cairo_prover::prover::prove_cairo;
    use stwo_prover::core::pcs::PcsConfig;
    use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleChannel;

    pub fn project_root() -> PathBuf {
        std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
    }

    // NOTE: the proof will include `step_limit -1` steps.
    fn input_from_plain_casm_with_step_limit(
        casm: Vec<cairo_lang_casm::instructions::Instruction>,
        step_limit: usize,
    ) -> ProverInput {
        let (program, _) = program_from_casm(casm);

        let mut runner =
            CairoRunner::new(&program, LayoutName::all_cairo_stwo, None, true, true, true)
                .expect("Runner creation failed");
        runner.initialize(true).expect("Initialization failed");
        runner
            .run_for_steps(step_limit, &mut BuiltinHintProcessor::new_empty())
            .expect("Run failed");
        runner.relocate(true).unwrap();

        adapt_finished_runner(runner).expect("Failed to adapt finished runner")
    }

    // TODO(Ohad): this is temporary, develop better automation.
    #[ignore = "slow, used to generate a proof"]
    #[test]
    fn generate_jrl0_proof() {
        let instructions = casm! {
            jmp rel 0;
        }
        .instructions;

        let preprocessed_trace = PreProcessedTraceVariant::CanonicalWithoutPedersen;

        let input = input_from_plain_casm_with_step_limit(instructions, 14);
        let proof =
            prove_cairo::<Blake2sMerkleChannel>(input, PcsConfig::default(), preprocessed_trace)
                .unwrap();

        let path = project_root().join("artifacts/jrl0_proof.json");
        std::fs::write(path, serde_json::to_string(&proof).unwrap()).unwrap();

        verify_cairo::<Blake2sMerkleChannel>(proof, PcsConfig::default(), preprocessed_trace)
            .unwrap();
    }
}
