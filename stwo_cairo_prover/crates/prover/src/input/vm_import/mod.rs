mod json;

use std::io::Read;
use std::path::Path;

use bytemuck::{bytes_of_mut, Pod, Zeroable};
use cairo_vm::air_public_input::{MemorySegmentAddresses, PublicInput};
use cairo_vm::stdlib::collections::HashMap;
use cairo_vm::vm::trace::trace_entry::RelocatedTraceEntry;
use json::PrivateInput;
use thiserror::Error;
use tracing::{span, Level};

use super::builtin_segments::BuiltinSegments;
use super::memory::MemoryConfig;
use super::state_transitions::StateTransitions;
use super::ProverInput;
use crate::components::memory::MEMORY_ADDRESS_BOUND;
use crate::input::memory::MemoryBuilder;

#[derive(Debug, Error)]
pub enum VmImportError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("No memory segments")]
    NoMemorySegments,
}

// TODO(Ohad): remove dev_mode after adding the rest of the instructions.
/// Adapts the VM's output files to the Cairo input of the prover.
pub fn adapt_vm_output(
    public_input_json: &Path,
    private_input_json: &Path,
    dev_mode: bool,
) -> Result<ProverInput, VmImportError> {
    let _span = span!(Level::INFO, "adapt_vm_output").entered();
    let public_input_string = std::fs::read_to_string(public_input_json)?;
    let public_input: PublicInput<'_> = serde_json::from_str(&public_input_string)?;
    let private_input: PrivateInput =
        serde_json::from_str(&std::fs::read_to_string(private_input_json)?)?;

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

    let mut memory_file = std::io::BufReader::new(std::fs::File::open(memory_path)?);
    let mut trace_file = std::io::BufReader::new(std::fs::File::open(trace_path)?);

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
        dev_mode,
    )
}

// TODO(Ohad): remove dev_mode after adding the rest of the opcodes.
/// Creates Cairo input for Stwo, utilized by:
/// - `adapt_vm_output` in the prover.
/// - `adapt_finished_runner` in the validator.
pub fn adapt_to_stwo_input(
    trace_iter: impl Iterator<Item = TraceEntry>,
    mut memory: MemoryBuilder,
    public_memory_addresses: Vec<u32>,
    memory_segments: &HashMap<&str, MemorySegmentAddresses>,
    dev_mode: bool,
) -> Result<ProverInput, VmImportError> {
    let (state_transitions, instruction_by_pc) =
        StateTransitions::from_iter(trace_iter, &mut memory, dev_mode);
    Ok(ProverInput {
        state_transitions,
        instruction_by_pc,
        memory: memory.build(),
        public_memory_addresses,
        builtins_segments: BuiltinSegments::from_memory_segments(memory_segments),
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
pub mod tests {
    use std::path::PathBuf;

    use super::*;

    pub fn large_cairo_input() -> ProverInput {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("test_data/test_read_from_large_files");

        adapt_vm_output(
            d.join("pub.json").as_path(),
            d.join("priv.json").as_path(),
            false,
        )
        .expect(
            "
            Failed to read test files. Checkout input/README.md.",
        )
    }

    pub fn small_cairo_input() -> ProverInput {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("test_data/test_read_from_small_files");
        adapt_vm_output(
            d.join("pub.json").as_path(),
            d.join("priv.json").as_path(),
            false,
        )
        .expect(
            "
            Failed to read test files. Checkout input/README.md.",
        )
    }

    #[test]
    #[cfg(feature = "slow-tests")]
    fn test_read_from_large_files() {
        let input = large_cairo_input();

        // Test opcode components.
        let components = input.state_transitions.casm_states_by_opcode;
        assert_eq!(components.generic_opcode.len(), 0);
        assert_eq!(components.add_ap_opcode_is_imm_f_op_1_base_fp_f.len(), 0);
        assert_eq!(
            components.add_ap_opcode_is_imm_t_op_1_base_fp_f.len(),
            36895
        );
        assert_eq!(components.add_ap_opcode_is_imm_f_op_1_base_fp_t.len(), 33);
        assert_eq!(components.add_opcode_is_small_t_is_imm_t.len(), 84732);
        assert_eq!(components.add_opcode_is_small_f_is_imm_f.len(), 189425);
        assert_eq!(components.add_opcode_is_small_t_is_imm_f.len(), 36623);
        assert_eq!(components.add_opcode_is_small_f_is_imm_t.len(), 22089);
        assert_eq!(
            components.assert_eq_opcode_is_double_deref_f_is_imm_f.len(),
            233432
        );
        assert_eq!(
            components.assert_eq_opcode_is_double_deref_t_is_imm_f.len(),
            811061
        );
        assert_eq!(
            components.assert_eq_opcode_is_double_deref_f_is_imm_t.len(),
            43184
        );
        assert_eq!(components.call_opcode_is_rel_f_op_1_base_fp_f.len(), 0);
        assert_eq!(components.call_opcode_is_rel_t_op_1_base_fp_f.len(), 49439);
        assert_eq!(components.call_opcode_is_rel_f_op_1_base_fp_t.len(), 33);
        assert_eq!(components.jnz_opcode_is_taken_t_dst_base_fp_t.len(), 11235);
        assert_eq!(components.jnz_opcode_is_taken_f_dst_base_fp_f.len(), 27032);
        assert_eq!(components.jnz_opcode_is_taken_t_dst_base_fp_f.len(), 51060);
        assert_eq!(components.jnz_opcode_is_taken_f_dst_base_fp_t.len(), 5100);
        assert_eq!(
            components
                .jump_opcode_is_rel_t_is_imm_t_is_double_deref_f
                .len(),
            31873865
        );
        assert_eq!(
            components
                .jump_opcode_is_rel_t_is_imm_f_is_double_deref_f
                .len(),
            500
        );
        assert_eq!(
            components
                .jump_opcode_is_rel_f_is_imm_f_is_double_deref_t
                .len(),
            32
        );
        assert_eq!(
            components
                .jump_opcode_is_rel_f_is_imm_f_is_double_deref_f
                .len(),
            0
        );
        assert_eq!(components.mul_opcode_is_small_t_is_imm_t.len(), 7234);
        assert_eq!(components.mul_opcode_is_small_t_is_imm_f.len(), 7203);
        assert_eq!(components.mul_opcode_is_small_f_is_imm_f.len(), 3943);
        assert_eq!(components.mul_opcode_is_small_f_is_imm_t.len(), 10809);
        assert_eq!(components.ret_opcode.len(), 49472);

        // Test builtins.
        let builtins_segments = input.builtins_segments;
        assert_eq!(builtins_segments.add_mod, None);
        assert_eq!(builtins_segments.bitwise, None);
        assert_eq!(builtins_segments.ec_op, Some((16428600, 16428747).into()));
        assert_eq!(builtins_segments.ecdsa, None);
        assert_eq!(builtins_segments.keccak, None);
        assert_eq!(builtins_segments.mul_mod, None);
        assert_eq!(builtins_segments.pedersen, Some((1322552, 1337489).into()));
        assert_eq!(
            builtins_segments.poseidon,
            Some((16920120, 17444532).into())
        );
        assert_eq!(builtins_segments.range_check_bits_96, None);
        assert_eq!(
            builtins_segments.range_check_bits_128,
            Some((1715768, 1757348).into())
        );
    }

    #[cfg(feature = "slow-tests")]
    #[test]
    fn test_read_from_small_files() {
        let input = small_cairo_input();

        // Test opcode components.
        let components = input.state_transitions.casm_states_by_opcode;
        assert_eq!(components.generic_opcode.len(), 0);
        assert_eq!(components.add_ap_opcode_is_imm_f_op_1_base_fp_f.len(), 0);
        assert_eq!(components.add_ap_opcode_is_imm_t_op_1_base_fp_f.len(), 2);
        assert_eq!(components.add_ap_opcode_is_imm_f_op_1_base_fp_t.len(), 1);
        assert_eq!(components.add_opcode_is_small_t_is_imm_t.len(), 500);
        assert_eq!(components.add_opcode_is_small_f_is_imm_f.len(), 0);
        assert_eq!(components.add_opcode_is_small_t_is_imm_f.len(), 0);
        assert_eq!(components.add_opcode_is_small_f_is_imm_t.len(), 450);
        assert_eq!(
            components.assert_eq_opcode_is_double_deref_f_is_imm_f.len(),
            55
        );
        assert_eq!(
            components.assert_eq_opcode_is_double_deref_t_is_imm_f.len(),
            2100
        );
        assert_eq!(
            components.assert_eq_opcode_is_double_deref_f_is_imm_t.len(),
            1952
        );
        assert_eq!(components.call_opcode_is_rel_f_op_1_base_fp_f.len(), 0);
        assert_eq!(components.call_opcode_is_rel_t_op_1_base_fp_f.len(), 462);
        assert_eq!(components.call_opcode_is_rel_f_op_1_base_fp_t.len(), 0);
        assert_eq!(components.jnz_opcode_is_taken_t_dst_base_fp_t.len(), 450);
        assert_eq!(components.jnz_opcode_is_taken_f_dst_base_fp_f.len(), 0);
        assert_eq!(components.jnz_opcode_is_taken_t_dst_base_fp_f.len(), 0);
        assert_eq!(components.jnz_opcode_is_taken_f_dst_base_fp_t.len(), 11);
        assert_eq!(
            components
                .jump_opcode_is_rel_t_is_imm_t_is_double_deref_f
                .len(),
            124626
        );
        assert_eq!(
            components
                .jump_opcode_is_rel_t_is_imm_f_is_double_deref_f
                .len(),
            0
        );
        assert_eq!(
            components
                .jump_opcode_is_rel_f_is_imm_f_is_double_deref_t
                .len(),
            0
        );
        assert_eq!(
            components
                .jump_opcode_is_rel_f_is_imm_f_is_double_deref_f
                .len(),
            0
        );
        assert_eq!(components.mul_opcode_is_small_t_is_imm_t.len(), 0);
        assert_eq!(components.mul_opcode_is_small_t_is_imm_f.len(), 0);
        assert_eq!(components.mul_opcode_is_small_f_is_imm_f.len(), 0);
        assert_eq!(components.mul_opcode_is_small_f_is_imm_t.len(), 0);
        assert_eq!(components.ret_opcode.len(), 462);

        // Test builtins.
        let builtins_segments = input.builtins_segments;
        assert_eq!(builtins_segments.add_mod, None);
        assert_eq!(builtins_segments.bitwise, Some((22512, 22762).into()));
        assert_eq!(builtins_segments.ec_op, Some((63472, 63822).into()));
        assert_eq!(builtins_segments.ecdsa, Some((22384, 22484).into()));
        assert_eq!(builtins_segments.keccak, Some((64368, 65168).into()));
        assert_eq!(builtins_segments.mul_mod, None);
        assert_eq!(builtins_segments.pedersen, Some((4464, 4614).into()));
        assert_eq!(builtins_segments.poseidon, Some((65392, 65692).into()));
        assert_eq!(
            builtins_segments.range_check_bits_96,
            Some((68464, 68514).into())
        );
        assert_eq!(
            builtins_segments.range_check_bits_128,
            Some((6000, 6050).into())
        );
    }
}
