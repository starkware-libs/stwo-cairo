use std::io;
use std::path::{Path, PathBuf};

/// An extension error to the io:Error, additionally specifying the not-found path.
#[derive(std::fmt::Debug)]
pub struct IoErrorWithPath {
    source: io::Error,
    path: PathBuf,
}
impl serde::de::StdError for IoErrorWithPath {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}
impl std::fmt::Display for IoErrorWithPath {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "{}: {}", self.source, self.path.display())
    }
}

/// A wrapper to `std::fs::File::open`, which, in case of failure, also logs the not-found path.
pub fn open_file(path: PathBuf) -> Result<std::fs::File, IoErrorWithPath> {
    std::fs::File::open(path.clone()).map_err(|e| IoErrorWithPath { source: e, path })
}

/// A wrapper to `std::fs::read_to_string`, which, in case of failure, also logs the not-found path.
pub fn read_to_string(path: &Path) -> Result<String, IoErrorWithPath> {
    std::fs::read_to_string(path).map_err(|e| IoErrorWithPath {
        source: e,
        path: path.to_path_buf(),
    })
}
