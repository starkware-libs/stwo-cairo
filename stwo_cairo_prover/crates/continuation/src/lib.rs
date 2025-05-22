#[cfg(test)]
mod tests{
    use stwo_cairo_adapter::test_utils::{get_compiled_cairo_program,runner_from_compiled_cairo_program};
    use stwo_cairo_adapter::ProverInput;
    use stwo_cairo_adapter::adapter::adapt_finished_runner;
    // use cairo_vm::types::relocatable::Relocatable;
    // use cairo_vm::vm::errors::memory_errors::MemoryError;
    use stwo_cairo_prover::prover::prove_cairo;
    use stwo_prover::core::pcs::PcsConfig;
    use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleChannel;
    use cairo_air::PreProcessedTraceVariant;

    fn test_fibonacci() -> ProverInput{
        let compiled_cairo_program = get_compiled_cairo_program("fibonacci");
        let runner = runner_from_compiled_cairo_program(&compiled_cairo_program);

        adapt_finished_runner(runner).expect("Failed to adapt finished runner")
    }

    #[test]
    fn run_fibonacci_test() {

        let result = test_fibonacci();
        println!("{:#?}", result);
        let preprocessed_trace = PreProcessedTraceVariant::CanonicalWithoutPedersen;
        let _proof =
        prove_cairo::<Blake2sMerkleChannel>(result, PcsConfig::default(), preprocessed_trace);
        // let path = "../../test_data/fibonacci/fibonacci_proof.json";
        // std::fs::write(path, serde_json::to_string(&proof).unwrap()).unwrap();
    }
}
