use std::path::PathBuf;

use cairo_vm::cairo_run;
use cairo_vm::hint_processor::builtin_hint_processor::builtin_hint_processor_definition::BuiltinHintProcessor;
use cairo_vm::types::layout_name::LayoutName;
use cairo_vm::vm::errors::cairo_run_errors::CairoRunError;
use cairo_vm::vm::runners::cairo_pie::CairoPie;
use cairo_vm::vm::runners::cairo_runner::{CairoRunner, RunResources};
use clap::{Parser, ValueHint};
use thiserror::Error;
use tracing::span;

// This struct is copied-then-modified from cairo-vm repo.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct VmArgs {
    #[clap(value_parser, value_hint=ValueHint::FilePath)]
    pub filename: PathBuf,
    #[clap(long = "trace_file", value_parser)]
    pub trace_file: Option<PathBuf>,
    #[structopt(long = "print_output")]
    pub print_output: bool,
    #[structopt(long = "entrypoint", default_value = "main")]
    pub entrypoint: String,
    #[structopt(long = "memory_file")]
    pub memory_file: Option<PathBuf>,
    #[structopt(long = "proof_mode")]
    pub proof_mode: bool,
    #[structopt(long = "secure_run")]
    pub secure_run: Option<bool>,
    #[clap(long = "air_public_input", requires = "proof_mode")]
    pub air_public_input: Option<String>,
    #[clap(
        long = "air_private_input",
        requires_all = ["proof_mode", "trace_file", "memory_file"]
    )]
    pub air_private_input: Option<String>,
    #[clap(
        long = "cairo_pie_output",
        // We need to add these air_private_input & air_public_input or else
        // passing cairo_pie_output + either of these without proof_mode will not fail
        conflicts_with_all = ["proof_mode", "air_private_input", "air_public_input"]
    )]
    pub cairo_pie_output: Option<String>,
    #[structopt(long = "allow_missing_builtins")]
    pub allow_missing_builtins: Option<bool>,
    #[structopt(
        long = "run_from_cairo_pie",
        // We need to add these air_private_input & air_public_input or else
        // passing run_from_cairo_pie + either of these without proof_mode will not fail
        conflicts_with_all = ["air_private_input", "air_public_input"]
    )]
    pub run_from_cairo_pie: bool,
}

#[derive(Debug, Error)]
pub enum VmError {
    #[error("Failed to interact with the file system")]
    IO(#[from] std::io::Error),
    #[error("The cairo program execution failed")]
    Runner(#[from] CairoRunError),
}

// This function's logic is copied-then-modified from cairo-vm-cli/src/main.rs:run in cairo-vm repo.
/// Runs the Cairo VM according to the given arguments (which are subset of the cairo-vm arguments).
pub fn run_vm(args: &VmArgs, disable_trace_padding: bool) -> Result<CairoRunner, VmError> {
    let _span = span!(tracing::Level::INFO, "run_vm").entered();
    let cairo_run_config = cairo_run::CairoRunConfig {
        entrypoint: &args.entrypoint,
        trace_enabled: true,
        relocate_mem: true,
        layout: LayoutName::all_cairo,
        proof_mode: args.proof_mode,
        secure_run: args.secure_run,
        allow_missing_builtins: args.allow_missing_builtins,
        disable_trace_padding,
        ..Default::default()
    };

    let cairo_runner_result = if args.run_from_cairo_pie {
        let pie = CairoPie::read_zip_file(&args.filename)?;
        let mut hint_processor = BuiltinHintProcessor::new(
            Default::default(),
            RunResources::new(pie.execution_resources.n_steps),
        );
        cairo_run::cairo_run_pie(&pie, &cairo_run_config, &mut hint_processor)
    } else {
        let program_content = std::fs::read(args.filename.clone()).map_err(VmError::IO)?;
        let mut hint_processor = BuiltinHintProcessor::new_empty();
        cairo_run::cairo_run(&program_content, &cairo_run_config, &mut hint_processor)
    };

    let cairo_runner = match cairo_runner_result {
        Ok(runner) => runner,
        Err(error) => {
            eprintln!("{error}");
            return Err(VmError::Runner(error));
        }
    };

    Ok(cairo_runner)
}
