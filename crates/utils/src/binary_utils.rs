use std::fmt::Display;
use std::process::ExitCode;

use crate::logging_utils::init_logging;

pub fn run_binary<T, E: Display>(
    run_function: fn() -> Result<T, E>,
    binary_name: &str,
) -> ExitCode {
    // TODO(yuval): allow control on log levels through args.
    init_logging(log::LevelFilter::Info);
    match run_function() {
        Ok(_) => {
            log::info!("{binary_name} succeeded");
            ExitCode::SUCCESS
        }
        Err(error) => {
            log::info!("{binary_name} failed: {error}");
            ExitCode::FAILURE
        }
    }
}
