use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};

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
