mod json;

use std::io::Read;

use bytemuck::{bytes_of_mut, Pod, Zeroable};
use cairo_vm::air_public_input::PublicInputError;
use cairo_vm::stdlib::collections::HashMap;
use thiserror::Error;

use super::builtins::BuiltinSegments;
use super::opcodes::StateTransitions;
use super::ProverInput;
use crate::builtins::MemorySegmentAddresses;
use crate::memory::MemoryBuilder;

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

/// Creates Cairo input for Stwo, utilized by:
/// - `adapt_vm_output` in the prover.
/// - `adapt_finished_runner` in the validator. TODO(Stav): delete when
///   'adapt_prover_input_info_vm_output' is used.
pub fn adapt_to_stwo_input(
    trace_iter: impl Iterator<Item = RelocatedTraceEntry>,
    mut memory: MemoryBuilder,
    public_memory_addresses: Vec<u32>,
    memory_segments: &HashMap<&str, MemorySegmentAddresses>,
) -> Result<ProverInput, VmImportError> {
    let state_transitions = StateTransitions::from_iter(trace_iter, &mut memory);
    let mut builtins_segments = BuiltinSegments::from_memory_segments(memory_segments);
    builtins_segments.fill_memory_holes(&mut memory);
    builtins_segments.pad_builtin_segments(&mut memory);
    let (memory, inst_cache) = memory.build();

    Ok(ProverInput {
        state_transitions,
        memory,
        inst_cache,
        public_memory_addresses,
        builtins_segments,
    })
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

pub struct TraceIter<'a, R: Read>(pub &'a mut R);
impl<R: Read> Iterator for TraceIter<'_, R> {
    type Item = RelocatedTraceEntry;

    fn next(&mut self) -> Option<Self::Item> {
        let mut entry = RelocatedTraceEntry::default();
        self.0
            .read_exact(bytes_of_mut(&mut entry))
            .ok()
            .map(|_| entry)
    }
}
