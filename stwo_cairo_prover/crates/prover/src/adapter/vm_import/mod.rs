mod json;

use std::io::Read;
use std::path::Path;

use bytemuck::{bytes_of_mut, Pod, Zeroable};
use cairo_vm::air_public_input::{MemorySegmentAddresses, PublicInput};
use cairo_vm::stdlib::collections::HashMap;
use cairo_vm::vm::trace::trace_entry::RelocatedTraceEntry;
use json::PrivateInput;
use stwo_cairo_utils::file_utils::{open_file, read_to_string, IoErrorWithPath};
use thiserror::Error;
use tracing::{span, Level};

use super::builtin_segments::BuiltinSegments;
use super::memory::MemoryConfig;
use super::state_transitions::StateTransitions;
use super::ProverInput;
use crate::adapter::memory::MemoryBuilder;
use crate::components::memory::MEMORY_ADDRESS_BOUND;

#[derive(Debug, Error)]
pub enum VmImportError {
    #[error("IO error: {0}")]
    Io(#[from] IoErrorWithPath),

    #[cfg(not(feature = "std"))]
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[cfg(feature = "std")]
    #[error("JSON error: {0}")]
    Json(#[from] sonic_rs::Error),
    #[error("No memory segments")]
    NoMemorySegments,
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
pub fn adapt_vm_output(
    public_input_json: &Path,
    private_input_json: &Path,
) -> Result<ProverInput, VmImportError> {
    let _span = span!(Level::INFO, "adapt_vm_output").entered();

    let (public_input_string, private_input_string) = (
        read_to_string(public_input_json)?,
        read_to_string(private_input_json)?,
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

    let mut memory_file = std::io::BufReader::new(open_file(memory_path.as_path())?);
    let mut trace_file = std::io::BufReader::new(open_file(trace_path.as_path())?);

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
/// - `adapt_finished_runner` in the validator.
pub fn adapt_to_stwo_input(
    trace_iter: impl Iterator<Item = TraceEntry>,
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
/// Note: This struct must be kept in sync with the Cairo VM's trace output file.
#[repr(C)]
#[derive(Copy, Clone, Default, Pod, Zeroable)]
pub struct TraceEntry {
    pub ap: u64,
    pub fp: u64,
    pub pc: u64,
}

impl From<RelocatedTraceEntry> for TraceEntry {
    fn from(value: RelocatedTraceEntry) -> Self {
        Self {
            ap: value.ap as u64,
            fp: value.fp as u64,
            pc: value.pc as u64,
        }
    }
}

pub struct TraceIter<'a, R: Read>(pub &'a mut R);
impl<R: Read> Iterator for TraceIter<'_, R> {
    type Item = TraceEntry;

    fn next(&mut self) -> Option<Self::Item> {
        let mut entry = TraceEntry::default();
        self.0
            .read_exact(bytes_of_mut(&mut entry))
            .ok()
            .map(|_| entry)
    }
}

/// A single entry from the memory file.
/// Note: This struct must be kept in sync with the Cairo VM's memory output file.
#[repr(C)]
#[derive(Copy, Clone, Default, Pod, Zeroable)]
pub struct MemoryEntry {
    pub address: u64,
    pub value: [u32; 8],
}

pub struct MemoryEntryIter<'a, R: Read>(pub &'a mut R);
impl<R: Read> Iterator for MemoryEntryIter<'_, R> {
    type Item = MemoryEntry;

    fn next(&mut self) -> Option<Self::Item> {
        let mut entry = MemoryEntry::default();
        self.0
            .read_exact(bytes_of_mut(&mut entry))
            .ok()
            .map(|_| entry)
    }
}

#[cfg(test)]
#[cfg(feature = "slow-tests")]
pub mod slow_tests {
    use crate::cairo_air::tests::test_input;

    #[test]
    fn test_read_from_large_files() {
        let input = test_input("test_read_from_large_files");

        // Test opcode components.
        let components = input.state_transitions.casm_states_by_opcode;
        assert_eq!(components.generic_opcode.len(), 0);
        assert_eq!(components.add_ap_opcode.len(), 0);
        assert_eq!(components.add_ap_opcode_imm.len(), 36895);
        assert_eq!(components.add_ap_opcode_op_1_base_fp.len(), 33);
        assert_eq!(components.add_opcode_small_imm.len(), 84732);
        assert_eq!(components.add_opcode.len(), 189425);
        assert_eq!(components.add_opcode_small.len(), 36623);
        assert_eq!(components.add_opcode_imm.len(), 22089);
        assert_eq!(components.assert_eq_opcode.len(), 233432);
        assert_eq!(components.assert_eq_opcode_double_deref.len(), 811061);
        assert_eq!(components.assert_eq_opcode_imm.len(), 43184);
        assert_eq!(components.call_opcode.len(), 0);
        assert_eq!(components.call_opcode_rel.len(), 49439);
        assert_eq!(components.call_opcode_op_1_base_fp.len(), 33);
        assert_eq!(components.jnz_opcode_taken_dst_base_fp.len(), 11235);
        assert_eq!(components.jnz_opcode.len(), 27032);
        assert_eq!(components.jnz_opcode_taken.len(), 51060);
        assert_eq!(components.jnz_opcode_dst_base_fp.len(), 5100);
        assert_eq!(components.jump_opcode_rel_imm.len(), 31873865);
        assert_eq!(components.jump_opcode_rel.len(), 500);
        assert_eq!(components.jump_opcode_double_deref.len(), 32);
        assert_eq!(components.jump_opcode.len(), 0);
        assert_eq!(components.mul_opcode_small_imm.len(), 7234);
        assert_eq!(components.mul_opcode_small.len(), 7203);
        assert_eq!(components.mul_opcode.len(), 3943);
        assert_eq!(components.mul_opcode_imm.len(), 10809);
        assert_eq!(components.ret_opcode.len(), 49472);

        // Test builtins.
        let builtins_segments = input.builtins_segments;
        assert_eq!(builtins_segments.add_mod, None);
        assert_eq!(builtins_segments.bitwise, None);
        assert_eq!(builtins_segments.ec_op, Some((16428600, 16428824).into()));
        assert_eq!(builtins_segments.ecdsa, None);
        assert_eq!(builtins_segments.keccak, None);
        assert_eq!(builtins_segments.mul_mod, None);
        assert_eq!(builtins_segments.pedersen, Some((1322552, 1347128).into()));
        assert_eq!(
            builtins_segments.poseidon,
            Some((16920120, 17706552).into())
        );
        assert_eq!(builtins_segments.range_check_bits_96, None);
        assert_eq!(
            builtins_segments.range_check_bits_128,
            Some((1715768, 1781304).into())
        );
    }

    #[test]
    fn test_read_from_small_files() {
        let input = test_input("test_read_from_small_files");

        // Test opcode components.
        let components = input.state_transitions.casm_states_by_opcode;
        assert_eq!(components.generic_opcode.len(), 0);
        assert_eq!(components.add_ap_opcode.len(), 0);
        assert_eq!(components.add_ap_opcode_imm.len(), 2);
        assert_eq!(components.add_ap_opcode_op_1_base_fp.len(), 1);
        assert_eq!(components.add_opcode_small_imm.len(), 500);
        assert_eq!(components.add_opcode.len(), 0);
        assert_eq!(components.add_opcode_small.len(), 0);
        assert_eq!(components.add_opcode_imm.len(), 450);
        assert_eq!(components.assert_eq_opcode.len(), 55);
        assert_eq!(components.assert_eq_opcode_double_deref.len(), 2100);
        assert_eq!(components.assert_eq_opcode_imm.len(), 1952);
        assert_eq!(components.call_opcode.len(), 0);
        assert_eq!(components.call_opcode_rel.len(), 462);
        assert_eq!(components.call_opcode_op_1_base_fp.len(), 0);
        assert_eq!(components.jnz_opcode_taken_dst_base_fp.len(), 450);
        assert_eq!(components.jnz_opcode.len(), 0);
        assert_eq!(components.jnz_opcode_taken.len(), 0);
        assert_eq!(components.jnz_opcode_dst_base_fp.len(), 11);
        assert_eq!(components.jump_opcode_rel_imm.len(), 124626);
        assert_eq!(components.jump_opcode_rel.len(), 0);
        assert_eq!(components.jump_opcode_double_deref.len(), 0);
        assert_eq!(components.jump_opcode.len(), 0);
        assert_eq!(components.mul_opcode_small_imm.len(), 0);
        assert_eq!(components.mul_opcode_small.len(), 0);
        assert_eq!(components.mul_opcode.len(), 0);
        assert_eq!(components.mul_opcode_imm.len(), 0);
        assert_eq!(components.ret_opcode.len(), 462);

        // Test builtins.
        let builtins_segments = input.builtins_segments;
        assert_eq!(builtins_segments.add_mod, None);
        assert_eq!(builtins_segments.bitwise, Some((22512, 22832).into()));
        assert_eq!(builtins_segments.ec_op, Some((63472, 63920).into()));
        assert_eq!(builtins_segments.ecdsa, Some((22384, 22512).into()));
        assert_eq!(builtins_segments.keccak, Some((64368, 65392).into()));
        assert_eq!(builtins_segments.mul_mod, None);
        assert_eq!(builtins_segments.pedersen, Some((4464, 4656).into()));
        assert_eq!(builtins_segments.poseidon, Some((65392, 65776).into()));
        assert_eq!(
            builtins_segments.range_check_bits_96,
            Some((68464, 68528).into())
        );
        assert_eq!(
            builtins_segments.range_check_bits_128,
            Some((6000, 6064).into())
        );
    }
}
