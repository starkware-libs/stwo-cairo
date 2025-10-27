mod json;

use std::fs::{read_to_string, File};
use std::path::Path;

use bytemuck::{cast_slice, Pod, Zeroable};
use cairo_vm::air_public_input::{PublicInput, PublicInputError};
use cairo_vm::stdlib::collections::HashMap;
use json::PrivateInput;
use memmap2::Mmap;
use serde::{Deserialize, Serialize};
use stwo_cairo_common::memory::MEMORY_ADDRESS_BOUND;
use thiserror::Error;
use tracing::{span, Level};

use super::builtins::BuiltinSegments;
use super::memory::MemoryConfig;
use super::opcodes::StateTransitions;
use super::ProverInput;
use crate::builtins::MemorySegmentAddresses;
use crate::memory::{MemoryBuilder, MemoryEntry};
use crate::PublicSegmentContext;

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
        tracing::info!(
            "Deserializing public input JSON ({} bytes)...",
            public_input_string.len()
        );
        let public_input = sonic_rs::from_str(public_input_string).map_err(|e| {
            tracing::error!(
                "Failed to deserialize public input JSON ({} bytes): {}",
                public_input_string.len(),
                e
            );
            e
        })?;
        tracing::info!("Public input deserialized successfully");

        tracing::info!(
            "Deserializing private input JSON ({} bytes)...",
            private_input_string.len()
        );
        let private_input = sonic_rs::from_str(private_input_string).map_err(|e| {
            tracing::error!(
                "Failed to deserialize private input JSON ({} bytes): {}",
                private_input_string.len(),
                e
            );
            e
        })?;
        tracing::info!("Private input deserialized successfully");

        Ok((public_input, private_input))
    }
    #[cfg(not(feature = "std"))]
    {
        tracing::info!(
            "Deserializing public input JSON ({} bytes)...",
            public_input_string.len()
        );
        let public_input = serde_json::from_str(public_input_string).map_err(|e| {
            tracing::error!(
                "Failed to deserialize public input JSON ({} bytes): {}",
                public_input_string.len(),
                e
            );
            e
        })?;
        tracing::info!("Public input deserialized successfully");

        tracing::info!(
            "Deserializing private input JSON ({} bytes)...",
            private_input_string.len()
        );
        let private_input = serde_json::from_str(private_input_string).map_err(|e| {
            tracing::error!(
                "Failed to deserialize private input JSON ({} bytes): {}",
                private_input_string.len(),
                e
            );
            e
        })?;
        tracing::info!("Private input deserialized successfully");

        Ok((public_input, private_input))
    }
}

/// Adapts the VM's output files to the Cairo input of the prover.
/// TODO(Stav): delete when 'adapt_prover_input_info_vm_output' is used.
///
/// # Assumptions
/// - The arguments are the artifacts of a bootloader execution, using every builtin.
pub fn adapt_vm_output(
    public_input_json: &Path,
    private_input_json: &Path,
) -> Result<ProverInput, VmImportError> {
    let _span = span!(Level::INFO, "adapt_vm_output").entered();

    tracing::info!(
        "Reading JSON files: public_input={}, private_input={}",
        public_input_json.display(),
        private_input_json.display()
    );

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

    tracing::info!(
        "JSON files loaded - public_input size: {} bytes ({:.2} MB), private_input size: {} bytes",
        public_input_string.len(),
        public_input_string.len() as f64 / 1_048_576.0,
        private_input_string.len()
    );

    tracing::info!("Starting JSON deserialization...");
    let (public_input, private_input) =
        deserialize_inputs(&public_input_string, &private_input_string)?;
    tracing::info!("JSON deserialization completed successfully");

    tracing::info!(
        "Public input contains {} memory segments, {} public memory entries",
        public_input.memory_segments.len(),
        public_input.public_memory.len()
    );

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

    tracing::info!(
        "Loading binary files: memory={}, trace={}",
        memory_path.display(),
        trace_path.display()
    );

    let memory = MmappedFile::<MemoryEntry>::new(memory_path.as_path());
    let trace = MmappedFile::<RelocatedTraceEntry>::new(trace_path.as_path());

    tracing::info!(
        "Binary files loaded - memory: {} entries ({:.2} MB), trace: {} entries ({:.2} MB)",
        memory.as_slice().len(),
        (memory.as_slice().len() * std::mem::size_of::<MemoryEntry>()) as f64 / 1_048_576.0,
        trace.as_slice().len(),
        (trace.as_slice().len() * std::mem::size_of::<RelocatedTraceEntry>()) as f64 / 1_048_576.0
    );

    let public_memory_addresses = public_input
        .public_memory
        .iter()
        .map(|entry| entry.address as u32)
        .collect();

    let public_segment_context = PublicSegmentContext::bootloader_context();
    let res = adapt_to_stwo_input(
        trace.as_slice(),
        MemoryBuilder::from_iter(MemoryConfig::default(), memory.as_slice().iter().copied()),
        public_memory_addresses,
        &public_input
            .memory_segments
            .into_iter()
            .map(|(k, v)| (k, v.into()))
            .collect(),
        public_segment_context,
    );
    res
}

/// Creates Cairo input for Stwo, utilized by `adapt_vm_output` in the prover.
/// TODO(Stav): delete when 'adapt_prover_input_info_vm_output' is used.
pub fn adapt_to_stwo_input(
    trace: &[RelocatedTraceEntry],
    mut memory: MemoryBuilder,
    public_memory_addresses: Vec<u32>,
    memory_segments: &HashMap<&str, MemorySegmentAddresses>,
    public_segment_context: PublicSegmentContext,
) -> Result<ProverInput, VmImportError> {
    tracing::info!(
        "Building prover input - trace entries: {}, public memory addresses: {}",
        trace.len(),
        public_memory_addresses.len()
    );

    let state_transitions = StateTransitions::from_slice_parallel(trace, &memory);
    let mut builtin_segments = BuiltinSegments::from_memory_segments(memory_segments);
    builtin_segments.fill_memory_holes(&mut memory);
    builtin_segments.pad_builtin_segments(&mut memory);
    let (memory, inst_cache) = memory.build();

    tracing::info!(
        "Prover input built - instruction cache size: {}",
        inst_cache.len()
    );

    Ok(ProverInput {
        state_transitions,
        memory,
        inst_cache,
        public_memory_addresses,
        builtin_segments,
        public_segment_context,
        #[cfg(feature = "extract-mem-trace")]
        relocated_mem: Vec::new(),
        #[cfg(feature = "extract-mem-trace")]
        relocated_trace: Vec::new(),
    })
}

struct MmappedFile<T: Pod> {
    mmap: Mmap,
    _marker: std::marker::PhantomData<T>,
}
impl<T: Pod> MmappedFile<T> {
    fn new(path: &Path) -> Self {
        let file = File::open(path)
            .unwrap_or_else(|_| panic!("Unable to open file at path {}", path.display()));
        let mmap = unsafe { memmap2::Mmap::map(&file).unwrap() };
        assert!(
            mmap.len().is_multiple_of(std::mem::size_of::<T>()),
            "File size is not a multiple of the type size"
        );
        tracing::debug!(
            "Memory-mapped file {} - size: {} bytes ({:.2} MB), entries: {}",
            path.display(),
            mmap.len(),
            mmap.len() as f64 / 1_048_576.0,
            mmap.len() / std::mem::size_of::<T>()
        );
        Self {
            mmap,
            _marker: std::marker::PhantomData,
        }
    }

    fn as_slice(&self) -> &[T] {
        cast_slice(&self.mmap)
    }
}

/// A single entry from the trace file.
#[repr(C)]
#[derive(Copy, Clone, Default, Pod, Zeroable, Debug, PartialEq, Serialize, Deserialize)]
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
