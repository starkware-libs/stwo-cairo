mod json;
use bytemuck::{Pod, Zeroable};
use cairo_vm::air_public_input::PublicInputError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum VmImportError {
    #[cfg(not(feature = "std"))]
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[cfg(feature = "std")]
    #[error("JSON error: {0}")]
    Json(#[from] sonic_rs::Error),
    #[error("No memory segments")]
    NoMemorySegments,

    #[error("Trace not relocated")]
    TraceNotRelocated,

    #[error("Cannot get public input from runner: {0}")]
    PublicInput(#[from] PublicInputError),
}

/// A single entry from the trace file.
#[repr(C)]
#[derive(Copy, Clone, Default, Pod, Zeroable, Debug, PartialEq)]
pub struct RelocatedTraceEntry {
    pub ap: usize,
    pub fp: usize,
    pub pc: usize,
}

impl From<cairo_vm::vm::trace::trace_entry::RelocatedTraceEntry> for RelocatedTraceEntry {
    fn from(entry: cairo_vm::vm::trace::trace_entry::RelocatedTraceEntry) -> Self {
        RelocatedTraceEntry {
            ap: entry.ap,
            fp: entry.fp,
            pc: entry.pc,
        }
    }
}
