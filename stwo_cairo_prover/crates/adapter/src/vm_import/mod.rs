mod json;

use std::io::Read;
use std::path::Path;

use bytemuck::{bytes_of_mut, Pod, Zeroable};
use cairo_vm::air_public_input::PublicInput;
use cairo_vm::stdlib::collections::HashMap;
use cairo_vm::types::builtin_name::BuiltinName;
use cairo_vm::vm::trace::trace_entry::RelocatedTraceEntry;
use itertools::Itertools;
use json::PrivateInput;
use serde::{Deserialize, Serialize};
use stwo_cairo_common::memory::MEMORY_ADDRESS_BOUND;
use stwo_cairo_serialize::CairoSerialize;
use stwo_cairo_utils::file_utils::{open_file, read_to_string, IoErrorWithPath};
use thiserror::Error;
use tracing::{span, Level};

use super::builtins::BuiltinSegments;
use super::memory::MemoryConfig;
use super::opcodes::StateTransitions;
use super::ProverInput;
use crate::builtins::MemorySegmentAddresses;
use crate::memory::{Memory, MemoryBuilder};

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

    #[error("Trace not relocated")]
    TraceNotRelocated,

    #[error("Cannot get public input from runner: {0}")]
    PublicInput(#[from] cairo_vm::air_public_input::PublicInputError),
}

// (id, value)
#[derive(Clone, Debug, Serialize, Deserialize, Copy, CairoSerialize)]
pub struct MemorySmallValue {
    pub id: u32,
    pub value: u32,
}

pub type PubMemoryValue = (u32, [u32; 8]);
// pub type MemorySmallValue = (u32, u32);
pub type PubMemoryEntry = (u32, u32, [u32; 8]);

#[derive(Clone, Debug, Serialize, Deserialize, Copy, CairoSerialize)]
pub struct SegmentRange {
    pub start_ptr: MemorySmallValue,
    pub stop_ptr: MemorySmallValue,
}
#[derive(Clone, Debug, Serialize, Deserialize, Copy, CairoSerialize, Default)]
pub struct PublicSegmentRanges {
    pub add_mod: Option<SegmentRange>,
    pub bitwise: Option<SegmentRange>,
    pub output: Option<SegmentRange>,
    pub range_check_128: Option<SegmentRange>,
    pub range_check_96: Option<SegmentRange>,
}

impl PublicSegmentRanges {
    pub fn new_from_memory(
        memory_segments: &HashMap<&str, MemorySegmentAddresses>,
        memory: &Memory,
        initial_pc: u32, 
        initial_ap: u32,
        final_ap: u32,
    ) -> Self {
        let mut res = PublicSegmentRanges::default();
        for (name, value) in memory_segments.iter() {
            if let Some(builtin_name) = BuiltinName::from_str(name) {
                // Filter empty segments.
                let segment = if value.begin_addr == value.stop_ptr {
                    None
                } else {
                    let start_ptr = value.begin_addr as u32;
                    let stop_ptr = value.stop_ptr as u32;
                    assert!(
                        start_ptr < stop_ptr,
                        "Invalid segment addresses: '{}'-'{}' for builtin '{name}'",
                        start_ptr,
                        stop_ptr,
                    );
                    let [start_ptr, stop_ptr] = [start_ptr, stop_ptr].map(|ptr| MemorySmallValue {
                        id:5,// Alonbalon
                        value: ptr,
                    });
                    Some(SegmentRange{start_ptr, stop_ptr})
                };
                match builtin_name {
                    BuiltinName::add_mod => res.add_mod = segment,
                    BuiltinName::bitwise => res.bitwise = segment,
                    BuiltinName::output => res.output = segment,
                    BuiltinName::range_check => res.range_check_128 = segment,
                    BuiltinName::range_check96 => res.range_check_96 = segment,
                    // Not supported
                    | BuiltinName::segment_arena
                    | BuiltinName::pedersen
                    | BuiltinName::ecdsa
                    | BuiltinName::keccak
                    | BuiltinName::ec_op
                    | BuiltinName::poseidon
                    | BuiltinName::mul_mod => {}
                }
            };
        }
        res
    }
    pub fn memory_entries(
        &self,
        initial_ap: u32,
        final_ap: u32,
    ) -> impl Iterator<Item = PubMemoryEntry> {
        let segments = [
            self.output,
            self.range_check_128,
            self.bitwise,
            self.range_check_96,
            self.add_mod,
        ]
        .into_iter()
        .flatten()
        .collect_vec();
        println!(
            "Public segment ranges: {:#?}",
            segments
        );
        let n_segments = segments.len() as u32;

        segments
            .into_iter()
            .enumerate()
            .map(
                move |(
                    i,
                    SegmentRange {
                        start_ptr,
                        stop_ptr,
                    },
                )| {
                    let start_address = initial_ap + i as u32;
                    let stop_address = final_ap - n_segments.clone() as u32 + i as u32;
                    [
                        (start_address, start_ptr.id, start_ptr.value),
                        (stop_address, stop_ptr.id, stop_ptr.value),
                    ]
                },
            )
            .flatten()
            .map(|(addr, id, value)| (addr, id, [value, 0, 0, 0, 0, 0, 0, 0]))
    }
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
    // let memory_segments = public_input.memory_segments
    let public_memory_addresses = public_input
        .public_memory
        .iter()
        .map(|entry| entry.address as u32)
        .collect();
    adapt_to_stwo_input(
        TraceIter(&mut trace_file),
        MemoryBuilder::from_iter(MemoryConfig::default(), MemoryEntryIter(&mut memory_file)),
        public_memory_addresses,
        &public_input
            .memory_segments
            .into_iter()
            .map(|(k, v)| (k, v.into()))
            .collect(),
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
    let memory = memory.build();

    Ok(ProverInput {
        state_transitions,
        instruction_by_pc,
        memory,
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

/// Creates a prover input from `pub.json`, `priv.json`, `mem`, and `trace` files.
///
/// # Expects
/// - These files must be stored in the `test_data/test_name` directory and contain valid Cairo
///   program data.
/// - They can be downloaded from Google Storage using `./scripts/download_test_data.sh`.   See
///   `SLOW_TESTS_README.md` for details.
///
/// # Panics
/// - If it fails to convert the files into a prover input.
pub fn generate_test_input(test_name: &str) -> ProverInput {
    let mut d = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("../../test_data/");
    d.push(test_name);

    adapt_vm_output(d.join("pub.json").as_path(), d.join("priv.json").as_path()).expect(
        "
        Failed to read test files. Checkout input/README.md.",
    )
}

#[cfg(test)]
#[cfg(feature = "slow-tests")]
pub mod slow_tests {
    use super::generate_test_input;

    #[test]
    fn test_read_from_large_files() {
        let input = generate_test_input("test_read_from_large_files");

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
        let input = generate_test_input("test_read_from_small_files");

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
