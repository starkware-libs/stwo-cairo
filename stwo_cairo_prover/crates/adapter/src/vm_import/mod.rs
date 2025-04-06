mod json;

use std::fs::{read_to_string, File};
use std::io::Read;
use std::path::Path;

use bytemuck::{bytes_of_mut, Pod, Zeroable};
use cairo_vm::air_public_input::{MemorySegmentAddresses, PublicInput, PublicInputError};
use cairo_vm::stdlib::collections::HashMap;
use json::PrivateInput;
use stwo_cairo_common::memory::MEMORY_ADDRESS_BOUND;
use thiserror::Error;
use tracing::{span, Level};

use super::builtins::BuiltinSegments;
use super::memory::MemoryConfig;
use super::opcodes::StateTransitions;
use super::ProverInput;
use crate::memory::{MemoryBuilder, MemoryEntryIter};

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

fn deserialize_inputs<'a>(
    public_input_string: &'a str,
    private_input_string: &'a str,
) -> Result<(PublicInput<'a>, PrivateInput), VmImportError> {
    #[cfg(feature = "std")]
    {
        Ok((
            sonic_rs::from_str(public_input_string)?,
            sonic_rs::from_str(private_input_string)?,
        ))
    }
    #[cfg(not(feature = "std"))]
    {
        Ok((
            serde_json::from_str(public_input_string)?,
            serde_json::from_str(private_input_string)?,
        ))
    }
}

/// Adapts the VM's output files to the Cairo input of the prover.
/// TODO(Stav): delete when 'adapt_prover_input_info_vm_output' is used.
pub fn adapt_vm_output(
    public_input_json: &Path,
    private_input_json: &Path,
) -> Result<ProverInput, VmImportError> {
    let _span = span!(Level::INFO, "adapt_vm_output").entered();

    let (public_input_string, private_input_string) = (
        read_to_string(public_input_json).unwrap_or_else(|_| {
            panic!(
                "Unable to read public input file at path {}",
                public_input_json.display()
            )
        }),
        read_to_string(private_input_json).unwrap_or_else(|_| {
            panic!(
                "Unable to read private input file at path {}",
                private_input_json.display()
            )
        }),
    );
    let (public_input, private_input) =
        deserialize_inputs(&public_input_string, &private_input_string)?;

    let end_addr = public_input
        .memory_segments
        .values()
        .map(|v| v.stop_ptr)
        .max()
        .ok_or(VmImportError::NoMemorySegments)?;
    assert!(end_addr < MEMORY_ADDRESS_BOUND);

    let memory_path = private_input_json
        .parent()
        .unwrap()
        .join(&private_input.memory_path);
    let trace_path = private_input_json
        .parent()
        .unwrap()
        .join(&private_input.trace_path);

    let mut memory_file =
        std::io::BufReader::new(File::open(memory_path.as_path()).unwrap_or_else(|_| {
            panic!(
                "Unable to open memory file at path {}",
                memory_path.display()
            )
        }));
    let mut trace_file =
        std::io::BufReader::new(File::open(trace_path.as_path()).unwrap_or_else(|_| {
            panic!("Unable to open trace file at path {}", trace_path.display())
        }));

    let public_memory_addresses = public_input
        .public_memory
        .iter()
        .map(|entry| entry.address as u32)
        .collect();
    adapt_to_stwo_input(
        TraceIter(&mut trace_file),
        MemoryBuilder::from_iter(MemoryConfig::default(), MemoryEntryIter(&mut memory_file)),
        public_memory_addresses,
        &public_input.memory_segments,
    )
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
    let (state_transitions, instruction_by_pc) =
        StateTransitions::from_iter(trace_iter, &mut memory);
    let mut builtins_segments = BuiltinSegments::from_memory_segments(memory_segments);
    builtins_segments.fill_memory_holes(&mut memory);
    builtins_segments.pad_builtin_segments(&mut memory);

    Ok(ProverInput {
        state_transitions,
        instruction_by_pc,
        memory: memory.build(),
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
