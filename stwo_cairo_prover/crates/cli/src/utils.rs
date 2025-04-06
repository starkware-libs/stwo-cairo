use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};

use tracing_subscriber::fmt::format::FmtSpan;

/// Initializes env_logger.
/// The format is:
/// `<level>  /path/to/file:<line_number>  <time>  <log_message>`
pub fn init_logging(log_level: log::LevelFilter) {
    env_logger::Builder::new().filter_level(log_level).init();

    let subscriber = tracing_subscriber::fmt()
        .with_ansi(false)
        .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Setting tracing default failed")
}

/// An extension error to the io:Error, additionally specifying the not-found path.
#[derive(std::fmt::Debug)]
pub struct IoErrorWithPath {
    source: io::Error,
    path: PathBuf,
}
impl std::error::Error for IoErrorWithPath {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}
impl std::fmt::Display for IoErrorWithPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}: {}", self.source, self.path.display())
    }
}

/// A wrapper to `File::open`, which, in case of failure, also logs the not-found path.
pub fn open_file(path: &Path) -> Result<File, IoErrorWithPath> {
    File::open(path).map_err(|e| IoErrorWithPath {
        source: e,
        path: path.to_path_buf(),
    })
}

/// A wrapper to `File::create`, which, in case of failure, also logs the file path.
pub fn create_file(path: &Path) -> Result<File, IoErrorWithPath> {
    File::create(path).map_err(|e| IoErrorWithPath {
        source: e,
        path: path.to_path_buf(),
    })
}

/// A wrapper to `std::fs::read_to_string`, which, in case of failure, also logs the not-found path.
pub fn read_to_string(path: &Path) -> Result<String, IoErrorWithPath> {
    std::fs::read_to_string(path).map_err(|e| IoErrorWithPath {
        source: e,
        path: path.to_path_buf(),
    })
}

use std::env::Args;
use std::fmt::Display;
use std::process::ExitCode;

pub fn run_binary<T, E: Display>(
    run_function: fn(Args) -> Result<T, E>,
    binary_name: &str,
) -> ExitCode {
    // TODO(yuval): allow control on log levels through args.
    init_logging(log::LevelFilter::Info);
    match run_function(std::env::args()) {
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
