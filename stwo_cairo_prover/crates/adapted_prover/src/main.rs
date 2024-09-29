use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;
use stwo_cairo_prover::cairo_air::{prove_cairo, CairoProof};
use stwo_cairo_prover::input::vm_import::{import_from_vm_output, VmImportError};
use stwo_cairo_prover::input::CairoInput;
use stwo_prover::core::prover::ProvingError;
use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleHasher;
use thiserror::Error;

/// Command line arguments for adapted_stwo.
/// Example command line:
///     ```
///     cargo run -r --bin adapted_stwo -- --pub_json absolute/path/to/pub.json
///     --priv_json absolute/path/to/priv.json
///     ```
#[derive(Parser, Debug)]
struct Args {
    #[structopt(long = "pub_json")]
    pub_json: PathBuf,
    #[structopt(long = "priv_json")]
    priv_json: PathBuf,
}

#[derive(Debug, Error)]
enum Error {
    #[error("Invalid arguments: {0}")]
    Cli(#[from] clap::Error),
    #[error("VM import failed: {0}")]
    VmImport(#[from] VmImportError),
    #[error("Proving failed: {0}")]
    Proving(#[from] ProvingError),
}

fn main() -> ExitCode {
    match run(std::env::args()) {
        Ok(_) => {
            println!("Adapted prover succeeded");
            ExitCode::SUCCESS
        }
        Err(error) => {
            println!("Adapted prover failed: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run(args: impl Iterator<Item = String>) -> Result<CairoProof<Blake2sMerkleHasher>, Error> {
    let args = Args::try_parse_from(args)?;

    let vm_output: CairoInput =
        import_from_vm_output(args.pub_json.as_path(), args.priv_json.as_path())?;

    // TODO(yuval): serialize the output to a file.
    Ok(prove_cairo(vm_output)?)
}
