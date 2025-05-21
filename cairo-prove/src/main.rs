use std::time::Instant;

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
    let executable = serde_json::from_reader(std::fs::File::open(target_path).unwrap())
        .expect("Failed to read executable");
    let runner = execute(executable, args);
    let input = prover_input_from_runner(&runner);
    prove(input, pcs_config)
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

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Prove {
            target,
            proof,
            arguments,
            arguments_file,
        } => {
            info!("Generating proof for target: {:?}", target);
            let start = Instant::now();
            let args = ProgramArguments {
                arguments: arguments.unwrap_or_default(),
                arguments_file,
            };
            let cairo_proof = execute_and_prove(
                target.to_str().unwrap(),
                args.read_arguments(),
                secure_pcs_config(),
            );
            let elapsed = start.elapsed();

            // Serialize proof to file.
            let proof_json = serde_json::to_string(&cairo_proof).unwrap();
            std::fs::write(proof.to_str().unwrap(), proof_json).unwrap();
            info!("Proof saved to: {:?}", proof);
            info!("Proof generation completed in {:.2?}", elapsed);
        }
        Commands::Verify {
            proof,
            with_pedersen,
        } => {
            info!("Verifying proof from: {:?}", proof);
            let cairo_proof =
                serde_json::from_reader(std::fs::File::open(proof.to_str().unwrap()).unwrap())
                    .unwrap();
            let preprocessed_trace = match with_pedersen {
                true => PreProcessedTraceVariant::Canonical,
                false => PreProcessedTraceVariant::CanonicalWithoutPedersen,
            };
            let result = verify_cairo::<Blake2sMerkleChannel>(
                cairo_proof,
                secure_pcs_config(),
                preprocessed_trace,
            );
            match result {
                Ok(_) => info!("Verification successful"),
                Err(e) => error!("Verification failed: {:?}", e),
            }
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
        let pcs_config = PcsConfig::default();
        let preprocessed_trace = PreProcessedTraceVariant::CanonicalWithoutPedersen;
        let result = verify_cairo::<Blake2sMerkleChannel>(proof, pcs_config, preprocessed_trace);
        assert!(result.is_ok());
    }
}
