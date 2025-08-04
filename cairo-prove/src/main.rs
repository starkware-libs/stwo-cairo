use std::path::Path;
use std::time::Instant;

use cairo_air::utils::{ProofFormat, deserialize_proof_from_bytes, serialize_proof_to_file};
use cairo_air::verifier::verify_cairo;
use cairo_air::{CairoProof, PreProcessedTraceVariant};
use cairo_lang_runner::Arg;
use cairo_prove::args::{Cli, Commands, ProgramArguments};
use cairo_prove::execute::execute;
use cairo_prove::prove::{prove, prover_input_from_runner};
use clap::Parser;
use log::{error, info};
use stwo_cairo_prover::stwo_prover::core::fri::FriConfig;
use stwo_cairo_prover::stwo_prover::core::pcs::PcsConfig;
use stwo_cairo_prover::stwo_prover::core::vcs::blake2_merkle::{
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
            n_queries: 700,
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

fn handle_verify(proof: &Path, proof_format: ProofFormat, with_pedersen: bool) {
    info!("Verifying proof from: {:?}", proof);

    let bytes = std::fs::read(proof).expect("Failed to read proof file");
    let cairo_proof = deserialize_proof_from_bytes::<Blake2sMerkleChannel>(&bytes, proof_format)
        .expect("Failed to deserialize proof");

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
            proof_format,
            with_pedersen,
        } => {
            handle_verify(&proof, proof_format, with_pedersen);
        }
        Commands::Transpile {
            in_proof,
            in_proof_format,
            out_proof,
            out_proof_format,
        } => {
            let bytes = std::fs::read(&in_proof).expect("Failed to read input proof file");
            let cairo_proof =
                deserialize_proof_from_bytes::<Blake2sMerkleChannel>(&bytes, in_proof_format)
                    .expect("Failed to deserialize input proof");

            serialize_proof_to_file::<Blake2sMerkleChannel>(
                &cairo_proof,
                out_proof,
                out_proof_format,
            )
            .expect("Failed to serialize output proof");
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
