use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;
use stwo_cairo_prover::cairo_air::air::CairoProof;
use stwo_cairo_prover::cairo_air::{prove_cairo, ConfigBuilder};
use stwo_cairo_prover::input::plain::adapt_finished_runner;
use stwo_cairo_prover::input::vm_import::VmImportError;
use stwo_cairo_utils::binary_utils::run_binary;
use stwo_cairo_utils::vm_utils::{run_vm, VmArgs, VmError};
use stwo_prover::core::prover::ProvingError;
use stwo_prover::core::vcs::blake2_merkle::{Blake2sMerkleChannel, Blake2sMerkleHasher};
use thiserror::Error;
use tracing::{span, Level};

/// Command line arguments for run_and_prove.
/// Example command line (use absolute paths):
///     ```
///     cargo run -r --bin run_and_prove -- --run_from_cairo_pie
///     --proof_path path/to/proof --secure_run=true path/to/cairo/pie
///     ```
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[command(flatten)]
    vm_args: VmArgs,
    /// The output file path for the proof.
    #[structopt(long = "proof_path")]
    proof_path: PathBuf,
    #[structopt(long = "track_relations")]
    track_relations: bool,
    #[structopt(long = "display_components")]
    display_components: bool,
}

#[derive(Debug, Error)]
enum Error {
    #[error("Invalid arguments")]
    Cli(#[from] clap::Error),
    #[error("Failed to interact with the file system")]
    IO(#[from] std::io::Error),
    #[error("VM run failed: {0}")]
    Vm(#[from] VmError),
    #[error("Serialization failed: {0}")]
    Serde(#[from] serde_json::error::Error),
    #[error("VM import failed: {0}")]
    VmImport(#[from] VmImportError),
    #[error("Proving failed: {0}")]
    Proving(#[from] ProvingError),
}

fn main() -> ExitCode {
    run_binary(run)
}

fn run(args: impl Iterator<Item = String>) -> Result<CairoProof<Blake2sMerkleHasher>, Error> {
    let _span = span!(Level::INFO, "run").entered();
    let args = Args::try_parse_from(args)?;
    let cairo_runner = run_vm(&args.vm_args)?;
    let cairo_input = adapt_finished_runner(cairo_runner, false);
    let prover_config = ConfigBuilder::default()
        .track_relations(args.track_relations)
        .display_components(args.display_components)
        .build();

    let casm_states_by_opcode_count = &cairo_input.state_transitions.casm_states_by_opcode.counts();
    log::info!("Casm states by opcode count: {casm_states_by_opcode_count:?}");

    let proof = prove_cairo::<Blake2sMerkleChannel>(cairo_input, prover_config)?;

    std::fs::write(args.proof_path, serde_json::to_string(&proof)?)?;

    Ok(proof)
}
