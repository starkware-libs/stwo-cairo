mod json;

use std::io::Read;
use std::path::Path;

use bytemuck::{bytes_of_mut, Pod, Zeroable};
#[cfg(test)]
use cairo_vm::air_public_input::MemorySegmentAddresses;
use cairo_vm::air_public_input::PublicInput;
use cairo_vm::vm::trace::trace_entry::RelocatedTraceEntry;
use json::PrivateInput;
use thiserror::Error;
use tracing::{span, Level};

use super::mem::MemConfig;
use super::state_transitions::StateTransitions;
use super::CairoInput;
use crate::input::mem::MemoryBuilder;
use crate::input::BuiltinsSegments;

#[derive(Debug, Error)]
pub enum VmImportError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] sonic_rs::Error),
    #[error("No memory segments")]
    NoMemorySegments,
}

// TODO(Ohad): remove dev_mode after adding the rest of the opcodes.
pub fn import_from_vm_output(
    pub_json: &Path,
    priv_json: &Path,
    dev_mode: bool,
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
    let mem_config = MemConfig::default();

    let mem_path = priv_json.parent().unwrap().join(&priv_data.memory_path);
    let trace_path = priv_json.parent().unwrap().join(&priv_data.trace_path);

    let mut trace_file = std::io::BufReader::new(std::fs::File::open(trace_path)?);
    let mut mem_file = std::io::BufReader::new(std::fs::File::open(mem_path)?);
    let mut mem = MemoryBuilder::from_iter(mem_config, MemEntryIter(&mut mem_file));
    let state_transitions =
        StateTransitions::from_iter(TraceIter(&mut trace_file), &mut mem, dev_mode);

    let public_mem_addresses = pub_data
        .public_memory
        .iter()
        .map(|entry| entry.address as u32)
        .collect();

    let builtins_segments = BuiltinsSegments::from_memory_segments(&pub_data.memory_segments);

    Ok(CairoInput {
        state_transitions,
        mem: mem.build(),
        public_mem_addresses,
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

        import_from_vm_output(
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
        import_from_vm_output(
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
            Some(MemorySegmentAddresses {
                begin_addr: 1715768,
                stop_ptr: 1757348
            })
        );
        assert_eq!(
            builtins_segments.range_check_bits_96_builtin,
            Some(MemorySegmentAddresses {
                begin_addr: 17706552,
                stop_ptr: 17706552
            })
        );
        assert_eq!(
            builtins_segments.bitwise_builtin,
            Some(MemorySegmentAddresses {
                begin_addr: 5942840,
                stop_ptr: 5942840
            })
        );
        assert_eq!(
            builtins_segments.add_mod_builtin,
            Some(MemorySegmentAddresses {
                begin_addr: 21900856,
                stop_ptr: 21900856
            })
        );
        assert_eq!(
            builtins_segments.ec_op_builtin,
            Some(MemorySegmentAddresses {
                begin_addr: 16428600,
                stop_ptr: 16428747
            })
        );
        assert_eq!(
            builtins_segments.ecdsa_builtin,
            Some(MemorySegmentAddresses {
                begin_addr: 5910072,
                stop_ptr: 5910072
            })
        );
        assert_eq!(
            builtins_segments.keccak_builtin,
            Some(MemorySegmentAddresses {
                begin_addr: 16657976,
                stop_ptr: 16657976
            })
        );
        assert_eq!(
            builtins_segments.mul_mod_builtin,
            Some(MemorySegmentAddresses {
                begin_addr: 23735864,
                stop_ptr: 23735864
            })
        );
        assert_eq!(
            builtins_segments.pedersen_builtin,
            Some(MemorySegmentAddresses {
                begin_addr: 1322552,
                stop_ptr: 1337489
            })
        );
        assert_eq!(
            builtins_segments.poseidon_builtin,
            Some(MemorySegmentAddresses {
                begin_addr: 16920120,
                stop_ptr: 17444532
            })
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
            Some(MemorySegmentAddresses {
                begin_addr: 6000,
                stop_ptr: 6050
            })
        );
        assert_eq!(
            builtins_segments.range_check_bits_96_builtin,
            Some(MemorySegmentAddresses {
                begin_addr: 68464,
                stop_ptr: 68514
            })
        );
        assert_eq!(
            builtins_segments.bitwise_builtin,
            Some(MemorySegmentAddresses {
                begin_addr: 22512,
                stop_ptr: 22762
            })
        );
        assert_eq!(
            builtins_segments.add_mod_builtin,
            Some(MemorySegmentAddresses {
                begin_addr: 84848,
                stop_ptr: 84848
            })
        );
        assert_eq!(
            builtins_segments.ec_op_builtin,
            Some(MemorySegmentAddresses {
                begin_addr: 63472,
                stop_ptr: 63822
            })
        );
        assert_eq!(
            builtins_segments.ecdsa_builtin,
            Some(MemorySegmentAddresses {
                begin_addr: 22384,
                stop_ptr: 22484
            })
        );
        assert_eq!(
            builtins_segments.keccak_builtin,
            Some(MemorySegmentAddresses {
                begin_addr: 64368,
                stop_ptr: 65168
            })
        );
        assert_eq!(
            builtins_segments.mul_mod_builtin,
            Some(MemorySegmentAddresses {
                begin_addr: 92016,
                stop_ptr: 92016
            })
        );
        assert_eq!(
            builtins_segments.pedersen_builtin,
            Some(MemorySegmentAddresses {
                begin_addr: 4464,
                stop_ptr: 4614
            })
        );
        assert_eq!(
            builtins_segments.poseidon_builtin,
            Some(MemorySegmentAddresses {
                begin_addr: 65392,
                stop_ptr: 65692
            })
        );
    }
}
