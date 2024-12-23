mod json;

use std::collections::HashMap;
use std::io::Read;
use std::path::Path;

use bytemuck::{bytes_of_mut, Pod, Zeroable};
use cairo_vm::air_public_input::{MemorySegmentAddresses, PublicInput};
use cairo_vm::vm::trace::trace_entry::RelocatedTraceEntry;
use json::PrivateInput;
use thiserror::Error;
use tracing::{span, Level};

use super::mem::MemConfig;
use super::state_transitions::StateTransitions;
use super::CairoInput;
use crate::input::mem::MemoryBuilder;

#[derive(Debug, Error)]
pub enum VmImportError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] sonic_rs::Error),
    #[error("No memory segments")]
    NoMemorySegments,
}

// TODO(Ohad): remove dev_mode after adding the rest of the instructions.
/// Adapts the VM's output files to the Cairo input of the prover.
pub fn adapt_vm_output(
    pub_json: &Path,
    priv_json: &Path,
    dev_mod: bool,
) -> Result<CairoInput, VmImportError> {
    let _span = span!(Level::INFO, "import_from_vm_output").entered();
    let pub_data_string = std::fs::read_to_string(pub_json)?;
    let pub_data: PublicInput<'_> = sonic_rs::from_str(&pub_data_string)?;
    let priv_data: PrivateInput = sonic_rs::from_str(&std::fs::read_to_string(priv_json)?)?;

    let end_addr = pub_data
        .memory_segments
        .values()
        .map(|v| v.stop_ptr)
        .max()
        .ok_or(VmImportError::NoMemorySegments)?;
    assert!(end_addr < (1 << 32));

    let mem_path = priv_json.parent().unwrap().join(&priv_data.memory_path);
    let trace_path = priv_json.parent().unwrap().join(&priv_data.trace_path);

    let mut trace_file = std::io::BufReader::new(std::fs::File::open(trace_path)?);
    let mut mem_file = std::io::BufReader::new(std::fs::File::open(mem_path)?);

    let public_mem_addresses = pub_data
        .public_memory
        .iter()
        .map(|entry| entry.address as u32)
        .collect();
    adapter(
        TraceIter(&mut trace_file),
        MemoryBuilder::from_iter(MemConfig::default(), MemEntryIter(&mut mem_file)),
        public_mem_addresses,
        &pub_data.memory_segments,
        dev_mod,
    )
}

// TODO(Ohad): remove dev_mode after adding the rest of the opcodes.
/// Creates Cairo input for Stwo, utilized by:
/// - `adapt_vm_output` in the prover.
/// - `adapt_finished_runner` in the validator.
pub fn adapter(
    trace_iter: impl Iterator<Item = TraceEntry>,
    mut mem: MemoryBuilder,
    public_mem_addresses: Vec<u32>,
    memory_segments: &HashMap<&str, MemorySegmentAddresses>,
    dev_mode: bool,
) -> Result<CairoInput, VmImportError> {
    Ok(CairoInput {
        state_transitions: StateTransitions::from_iter(trace_iter, &mut mem, dev_mode),
        mem: mem.build(),
        public_mem_addresses,
        builtins_segments: (memory_segments).into(),
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
pub struct MemEntry {
    pub addr: u64,
    pub val: [u32; 8],
}

pub struct MemEntryIter<'a, R: Read>(pub &'a mut R);
impl<R: Read> Iterator for MemEntryIter<'_, R> {
    type Item = MemEntry;

    fn next(&mut self) -> Option<Self::Item> {
        let mut entry = MemEntry::default();
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

    pub fn large_cairo_input() -> CairoInput {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("test_data/large_cairo_input");

        adapt_vm_output(
            d.join("pub.json").as_path(),
            d.join("priv.json").as_path(),
            false,
        )
        .expect(
            "
            Failed to read test files. Maybe git-lfs is not installed? Checkout README.md.",
        )
    }

    pub fn small_cairo_input() -> CairoInput {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("test_data/small_cairo_input");
        adapt_vm_output(
            d.join("pub.json").as_path(),
            d.join("priv.json").as_path(),
            false,
        )
        .expect(
            "
            Failed to read test files. Maybe git-lfs is not installed? Checkout README.md.",
        )
    }

    // TODO (Stav): Once all the components are in, verify the proof to ensure the sort was correct.
    #[ignore]
    #[test]
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
        assert_eq!(components.add_opcode_is_small_t_is_imm_t.len(), 83399);
        assert_eq!(components.add_opcode_is_small_f_is_imm_f.len(), 189425);
        assert_eq!(components.add_opcode_is_small_t_is_imm_f.len(), 36623);
        assert_eq!(components.add_opcode_is_small_f_is_imm_t.len(), 23422);
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
        assert_eq!(components.mul_opcode_is_small_t_is_imm_t.len(), 8996);
        assert_eq!(components.mul_opcode_is_small_t_is_imm_f.len(), 6563);
        assert_eq!(components.mul_opcode_is_small_f_is_imm_f.len(), 4583);
        assert_eq!(components.mul_opcode_is_small_f_is_imm_t.len(), 9047);
        assert_eq!(components.ret_opcode.len(), 49472);

        // Test builtins.
        let builtins_segments = input.builtins_segments;
        assert_eq!(
            builtins_segments.range_check_bits_128_builtin,
            Some((1715768, 1757348).into())
        );
        assert_eq!(
            builtins_segments.range_check_bits_96_builtin,
            Some((17706552, 17706552).into())
        );
        assert_eq!(
            builtins_segments.bitwise_builtin,
            Some((5942840, 5942840).into())
        );
        assert_eq!(
            builtins_segments.add_mod_builtin,
            Some((21900856, 21900856).into())
        );
        assert_eq!(
            builtins_segments.ec_op_builtin,
            Some((16428600, 16428747).into())
        );
        assert_eq!(
            builtins_segments.ecdsa_builtin,
            Some((5910072, 5910072).into())
        );
        assert_eq!(
            builtins_segments.keccak_builtin,
            Some((16657976, 16657976).into())
        );
        assert_eq!(
            builtins_segments.mul_mod_builtin,
            Some((23735864, 23735864).into())
        );
        assert_eq!(
            builtins_segments.pedersen_builtin,
            Some((1322552, 1337489).into())
        );
        assert_eq!(
            builtins_segments.poseidon_builtin,
            Some((16920120, 17444532).into())
        );
    }

    #[ignore]
    #[test]
    fn test_read_from_small_files() {
        let input = small_cairo_input();

        // Test opcode components.
        let components = input.state_transitions.casm_states_by_opcode;
        assert_eq!(components.generic_opcode.len(), 0);
        assert_eq!(components.add_ap_opcode_is_imm_f_op_1_base_fp_f.len(), 0);
        assert_eq!(components.add_ap_opcode_is_imm_t_op_1_base_fp_f.len(), 2);
        assert_eq!(components.add_ap_opcode_is_imm_f_op_1_base_fp_t.len(), 1);
        assert_eq!(components.add_opcode_is_small_t_is_imm_t.len(), 950);
        assert_eq!(components.add_opcode_is_small_f_is_imm_f.len(), 0);
        assert_eq!(components.add_opcode_is_small_t_is_imm_f.len(), 0);
        assert_eq!(components.add_opcode_is_small_f_is_imm_t.len(), 0);
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
        assert_eq!(
            builtins_segments.range_check_bits_128_builtin,
            Some((6000, 6050).into())
        );
        assert_eq!(
            builtins_segments.range_check_bits_96_builtin,
            Some((68464, 68514).into())
        );
        assert_eq!(
            builtins_segments.bitwise_builtin,
            Some((22512, 22762).into())
        );
        assert_eq!(
            builtins_segments.add_mod_builtin,
            Some((84848, 84848).into())
        );
        assert_eq!(builtins_segments.ec_op_builtin, Some((63472, 63822).into()));
        assert_eq!(builtins_segments.ecdsa_builtin, Some((22384, 22484).into()));
        assert_eq!(
            builtins_segments.keccak_builtin,
            Some((64368, 65168).into())
        );
        assert_eq!(
            builtins_segments.mul_mod_builtin,
            Some((92016, 92016).into())
        );
        assert_eq!(
            builtins_segments.pedersen_builtin,
            Some((4464, 4614).into())
        );
        assert_eq!(
            builtins_segments.poseidon_builtin,
            Some((65392, 65692).into())
        );
    }
}
