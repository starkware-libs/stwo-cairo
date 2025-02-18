use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;
use stwo_cairo_prover::adapter::plain::adapt_finished_runner;
use stwo_cairo_prover::adapter::vm_import::VmImportError;
use stwo_cairo_prover::cairo_air::{
    prove_cairo, verify_cairo, CairoVerificationError, ConfigBuilder,
};
use stwo_cairo_utils::binary_utils::run_binary;
use stwo_cairo_utils::vm_utils::{run_vm, VmArgs, VmError};
use stwo_prover::core::prover::ProvingError;
use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleChannel;
use thiserror::Error;
use tracing::{span, Level};

// TODO(yuval): unite this and adapted_prover to a single binary, or at least share more code.
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
    Serde(#[from] serde_json::error::Error),
    #[error("Verification failed: {0}")]
    Verification(#[from] CairoVerificationError),
    #[error("VM run failed: {0}")]
    Vm(#[from] VmError),
    #[error("VM import failed: {0}")]
    VmImport(#[from] VmImportError),
}

fn main() -> ExitCode {
    run_binary(run, "run_and_prove")
}

fn run(args: impl Iterator<Item = String>) -> Result<(), Error> {
    let _span = span!(Level::INFO, "run").entered();
    let args = Args::try_parse_from(args)?;

    // `disable_trace_padding` is set to true as run_and_prove runs the VM in proof mode, and
    // should disable trace padding (this is the mode Stwo uses).
    let cairo_runner = run_vm(&args.vm_args, true)?;
    let cairo_input = adapt_finished_runner(cairo_runner)?;
    let prover_config = ConfigBuilder::default()
        .track_relations(args.track_relations)
        .display_components(args.display_components)
        .build();

    log::info!(
        "Casm states by opcode:\n{}",
        cairo_input.state_transitions.casm_states_by_opcode
    );

    // TODO(Ohad): Propagate hash from CLI args.
    let proof = prove_cairo::<Blake2sMerkleChannel>(cairo_input, prover_config)?;

    std::fs::write(args.proof_path, serde_json::to_string(&proof)?)?;

    if args.verify {
        verify_cairo::<Blake2sMerkleChannel>(proof)?;
        log::info!("Proof verified successfully");
    }

    Ok(())
}
