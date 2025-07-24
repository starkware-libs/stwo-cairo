use std::path::Path;
use std::time::Instant;

use cairo_air::utils::{ProofFormat, serialize_proof_to_file};
use cairo_air::verifier::verify_cairo;
use cairo_air::{CairoProof, PreProcessedTraceVariant};
use cairo_lang_runner::Arg;
use cairo_prove::args::{Cli, Commands, ProgramArguments};
use cairo_prove::execute::execute;
use cairo_prove::prove::{prove, prover_input_from_runner};
use clap::Parser;
use log::{error, info};
use stwo_cairo_prover::stwo::core::fri::FriConfig;
use stwo_cairo_prover::stwo::core::pcs::PcsConfig;
use stwo_cairo_prover::stwo::core::vcs::blake2_merkle::{
    Blake2sMerkleChannel, Blake2sMerkleHasher,
};

fn execute_and_prove(
    target_path: &str,
    args: Vec<Arg>,
    pcs_config: PcsConfig,
) -> CairoProof<Blake2sMerkleHasher> {
    // Execute.
    let executable = serde_json::from_reader(std::fs::File::open(target_path).unwrap())
        .expect("Failed to read executable");
    let runner = execute(executable, args);

    // Prove.
    let prover_input = prover_input_from_runner(&runner);
    prove(prover_input, pcs_config)
}

fn secure_pcs_config() -> PcsConfig {
    PcsConfig {
        pow_bits: 26,
        fri_config: FriConfig {
            log_last_layer_degree_bound: 0,
            log_blowup_factor: 1,
            n_queries: 70,
        },
    }
}

fn handle_prove(target: &Path, proof: &Path, proof_format: ProofFormat, args: ProgramArguments) {
    info!("Generating proof for target: {:?}", target);
    let start = Instant::now();
    let cairo_proof = execute_and_prove(
        target.to_str().unwrap(),
        args.read_arguments(),
        secure_pcs_config(),
    );
    let elapsed = start.elapsed();

    serialize_proof_to_file::<Blake2sMerkleChannel>(&cairo_proof, proof.into(), proof_format)
        .expect("Failed to serialize proof");

    info!("Proof saved to: {:?}", proof);
    info!("Proof generation completed in {:.2?}", elapsed);
}

fn handle_verify(proof: &Path, with_pedersen: bool) {
    info!("Verifying proof from: {:?}", proof);
    let cairo_proof =
        serde_json::from_reader(std::fs::File::open(proof.to_str().unwrap()).unwrap()).unwrap();
    let preprocessed_trace = match with_pedersen {
        true => PreProcessedTraceVariant::Canonical,
        false => PreProcessedTraceVariant::CanonicalWithoutPedersen,
    };
    let result = verify_cairo::<Blake2sMerkleChannel>(cairo_proof, preprocessed_trace);
    match result {
        Ok(_) => info!("Verification successful"),
        Err(e) => error!("Verification failed: {:?}", e),
    }
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Prove {
            target,
            proof,
            proof_format,
            program_arguments,
        } => {
            handle_prove(&target, &proof, proof_format, program_arguments);
        }
        Commands::Verify {
            proof,
            with_pedersen,
        } => {
            handle_verify(&proof, with_pedersen);
        }
    }
}

#[cfg(test)]
mod tests {
    use cairo_vm::Felt252;
    use num_bigint::BigInt;

    use super::*;

    #[test]
    fn test_e2e() {
        let target_path = "./example/target/release/example.executable.json";
        let args = vec![Arg::Value(Felt252::from(BigInt::from(100)))];
        let proof = execute_and_prove(target_path, args, PcsConfig::default());
        let preprocessed_trace = PreProcessedTraceVariant::CanonicalWithoutPedersen;
        let result = verify_cairo::<Blake2sMerkleChannel>(proof, preprocessed_trace);
        assert!(result.is_ok());
    }
}
