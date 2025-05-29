#[cfg(test)]
mod tests{
    const PROOF_FILE: &str = "fibonacci";

    use stwo_cairo_adapter::test_utils::{get_compiled_cairo_program,runner_from_compiled_cairo_program};
    use stwo_cairo_adapter::ProverInput;
    use stwo_cairo_adapter::adapter::adapt_finished_runner;
    use stwo_cairo_prover::prover::prove_cairo;
    use stwo_prover::core::pcs::PcsConfig;
    use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleChannel;
    use cairo_air::PreProcessedTraceVariant;

    fn test_fibonacci() -> ProverInput{
        let compiled_cairo_program = get_compiled_cairo_program(PROOF_FILE);
        let runner = runner_from_compiled_cairo_program(&compiled_cairo_program);

        adapt_finished_runner(runner).expect("Failed to adapt finished runner")
    }

    #[test]
    fn run_fibonacci_test() {

        let result = test_fibonacci();
        let preprocessed_trace = PreProcessedTraceVariant::CanonicalWithoutPedersen;
        let proof =
        prove_cairo::<Blake2sMerkleChannel>(result, PcsConfig::default(), preprocessed_trace);
        let path = format!("../../test_data/{}/proof_2D.json", PROOF_FILE);
        match proof {
            Ok(proof) => {
                std::fs::write(path, serde_json::to_string(&proof).unwrap()).unwrap();
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }
}
