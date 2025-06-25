use std::path::PathBuf;

use cairo_lang_executable::executable::Executable;
use cairo_prove::args::ProgramArguments;
use cairo_prove::execute::execute;
use clap::Parser;
use dev_utils::utils::Error;
use stwo_cairo_adapter::adapter::adapter;
use stwo_cairo_adapter::test_utils::{read_compiled_cairo_program, run_program_and_adapter};
use stwo_cairo_adapter::ProverInput;
use tracing::{info, span, Level};
use tracing_subscriber::fmt::format::FmtSpan;

/// Command-line usage for `run_program_and_adapter`.
///
/// ### Example command to run:
/// ```
/// cargo run --bin run_program_and_adapter -- --compiled_program /path/to/compiled.json
/// ```
/// Use `--cairo1` to indicate that the program is.
///
/// ### Passing arguments:
/// - To pass inline arguments: ``` --arguments <args> ```
/// - To load arguments from a file: ``` --arguments-file ./args.json ```
///
/// // ### Example command to run the verifier:
/// ```
/// cargo run --bin run_program_and_adapter -- --cairo1 --compiled_program  /path/to/executable.json --arguments-file /path/to/proof.json
/// ```
#[derive(Parser, Debug)]
struct Args {
    #[structopt(long = "compiled_program")]
    compiled_program: PathBuf,
    /// Is the program a Cairo 1 program?
    #[structopt(long = "cairo1")]
    cairo1: bool,
    #[command(flatten)]
    program_arguments: ProgramArguments,
}

fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
        .init();

    let _span = span!(Level::INFO, "run").entered();
    let args = Args::try_parse_from(std::env::args())?;

    let prover_input = if args.cairo1 {
        let executable: Executable =
            serde_json::from_reader(std::fs::File::open(&args.compiled_program).unwrap())
                .expect("Failed to read executable");
        run_cairo1_and_adapter(executable, args.program_arguments.read_arguments())
    } else {
        assert!(
            args.program_arguments.arguments.is_empty()
                && args.program_arguments.arguments_file.is_none(),
            "Can't run Cairo0 programs with arguments"
        );
        let program = read_compiled_cairo_program(&args.compiled_program);
        run_program_and_adapter(&program)
    };

    // TODO(Stav): delete when we have this print in the adapter.
    info!(
        "Opcode's count: {:?}",
        prover_input
            .state_transitions
            .casm_states_by_opcode
            .counts()
    );

    Ok(())
}

fn run_cairo1_and_adapter(program: Executable, args: Vec<cairo_lang_runner::Arg>) -> ProverInput {
    let runner = execute(program, args);
    let mut prover_input_info = runner.get_prover_input_info().expect("");
    adapter(&mut prover_input_info).expect("Failed to run adapter")
}
