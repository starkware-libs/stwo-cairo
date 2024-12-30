use std::env::Args;
use std::fmt::Display;
use std::process::ExitCode;

use crate::logging_utils::init_logging;

pub fn run_binary<T, E: Display>(run_function: fn(Args) -> Result<T, E>) -> ExitCode {
    // TODO(yuval): allow control on log levels through args.
    init_logging(log::LevelFilter::Info);
    match run_function(std::env::args()) {
        Ok(_) => {
            log::info!("run_and_prove succeeded");
            ExitCode::SUCCESS
        }
        Err(error) => {
            log::info!("run_and_prove failed: {error}");
            ExitCode::FAILURE
        }
    }
}