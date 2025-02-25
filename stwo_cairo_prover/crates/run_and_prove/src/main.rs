use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;
use stwo_cairo_adapter::plain::adapt_finished_runner;
use stwo_cairo_adapter::vm_import::VmImportError;
use stwo_cairo_adapter::ExecutionResources;
use stwo_cairo_prover::cairo_air::{
    default_prod_prover_parameters, prove_cairo, verify_cairo, CairoVerificationError,
    ConfigBuilder, ProverParameters,
};
use stwo_cairo_utils::binary_utils::run_binary;
use stwo_cairo_utils::program_runner_utils::{
    run_cairo_program, ProgramRunnerArgs, ProgramRunnerError,
};
use stwo_prover::core::prover::ProvingError;
use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleChannel;
use thiserror::Error;
use tracing::{span, Level};

// TODO(yuval): unite this and adapted_prover to a single binary, or at least share more code.
/// Command line arguments for run_and_prove.
/// Example command line (use absolute paths):
///     ```
///     cargo run -r --bin run_and_prove -- --program path/to/program/file --program_input
///     path/to/input/file --output_er_path path/to/adapted_er/output/file --proof_path
/// 	path/to/proof/output/file     ```
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[command(flatten)]
    program_runner_args: ProgramRunnerArgs,
    /// The output file path for the proof.
    #[structopt(long = "proof_path")]
    proof_path: PathBuf,
    /// The file path for the output (the adapted execution resources of the VM run).
    #[structopt(long = "output_er_path")]
    output_er_path: Option<PathBuf>,
    #[structopt(long = "track_relations")]
    track_relations: bool,
    #[structopt(long = "display_components")]
    display_components: bool,
    /// Verify the generated proof.
    #[structopt(long = "verify")]
    verify: bool,
}

#[derive(Debug, Error)]
enum Error {
    #[error("Invalid arguments: {0}")]
    Cli(#[from] clap::Error),
    #[error("IO failed: {0}")]
    IO(#[from] std::io::Error),
    #[error("Proving failed: {0}")]
    Proving(#[from] ProvingError),
    #[error("Serialization failed: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Verification failed: {0}")]
    Verification(#[from] CairoVerificationError),
    #[error("program-runner run failed: {0}")]
    ProgramRunner(#[from] ProgramRunnerError),
    #[error("VM import failed: {0}")]
    VmImport(#[from] VmImportError),
}

fn main() -> ExitCode {
    run_binary(run, "run_and_prove")
}

fn run(args: impl Iterator<Item = String>) -> Result<(), Error> {
    let _span = span!(Level::INFO, "run").entered();
    let args = Args::try_parse_from(args)?;

    let cairo_runner = run_cairo_program(&args.program_runner_args)?;
    let cairo_input = adapt_finished_runner(cairo_runner)?;

    if let Some(output_er_path) = args.output_er_path {
        let execution_resources = ExecutionResources::from_prover_input(&cairo_input);
        std::fs::write(output_er_path, serde_json::to_string(&execution_resources)?)?;
    }

    let prover_config = ConfigBuilder::default()
        .track_relations(args.track_relations)
        .display_components(args.display_components)
        .build();

    log::info!(
        "Casm states by opcode:\n{}",
        cairo_input.state_transitions.casm_states_by_opcode
    );

    let ProverParameters { pcs_config } = default_prod_prover_parameters();

    // TODO(Ohad): Propagate hash from CLI args.
    let proof = prove_cairo::<Blake2sMerkleChannel>(cairo_input, prover_config, pcs_config)?;

    std::fs::write(args.proof_path, serde_json::to_string(&proof)?)?;

    if args.verify {
        verify_cairo::<Blake2sMerkleChannel>(proof, pcs_config)?;
        log::info!("Proof verified successfully");
    }

    Ok(())
}
