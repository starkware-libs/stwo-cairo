mod json;

use std::io::Read;
use std::path::Path;

use bytemuck::{bytes_of_mut, Pod, Zeroable};
use cairo_vm::vm::trace::trace_entry::RelocatedTraceEntry;
#[cfg(test)]
use indexmap::IndexMap;
use json::{PrivateInput, PublicInput};
use thiserror::Error;
use tracing::{span, Level};

use super::instructions::Instructions;
use super::mem::MemConfig;
use super::CairoInput;
use crate::input::mem::MemoryBuilder;
use crate::input::SegmentAddrs;

#[derive(Debug, Error)]
pub enum VmImportError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] sonic_rs::Error),
    #[error("No memory segments")]
    NoMemorySegments,
}

pub fn import_from_vm_output(
    pub_json: &Path,
    priv_json: &Path,
) -> Result<CairoInput, VmImportError> {
    let _span = span!(Level::INFO, "import_from_vm_output").entered();
    let pub_data: PublicInput = sonic_rs::from_str(&std::fs::read_to_string(pub_json)?)?;
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
    let instructions = Instructions::from_iter(TraceIter(&mut trace_file), &mut mem);

    let public_mem_addresses = pub_data
        .public_memory
        .iter()
        .map(|entry| entry.address as u32)
        .collect();

    Ok(CairoInput {
        instructions,
        mem: mem.build(),
        public_mem_addresses,
        range_check_builtin: SegmentAddrs {
            begin_addr: pub_data.memory_segments["range_check"].begin_addr as u32,
            end_addr: pub_data.memory_segments["range_check"].stop_ptr as u32,
        },
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
impl<'a, R: Read> Iterator for TraceIter<'a, R> {
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
impl<'a, R: Read> Iterator for MemEntryIter<'a, R> {
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

        import_from_vm_output(d.join("pub.json").as_path(), d.join("priv.json").as_path()).expect(
            "
            Failed to read test files. Maybe git-lfs is not installed? Checkout README.md.",
        )
    }

    pub fn small_cairo_input() -> CairoInput {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("test_data/small_cairo_input");
        import_from_vm_output(d.join("pub.json").as_path(), d.join("priv.json").as_path()).expect(
            "
            Failed to read test files. Maybe git-lfs is not installed? Checkout README.md.",
        )
    }

    #[ignore]
    #[test]
    fn test_read_from_large_files() {
        let input = large_cairo_input();
        let expected_counts: IndexMap<&str, usize> = IndexMap::from_iter(vec![
            ("generic_opcode", 0),
            ("add_ap_opcode_is_imm_f_op1_base_fp_f", 0),
            ("add_ap_opcode_is_imm_t_op1_base_fp_f", 36895), // add ap
            ("add_ap_opcode_is_imm_f_op1_base_fp_t", 33),    // generic
            ("add_opcode_is_small_t_is_imm_t", 94680),       // generic
            ("add_opcode_is_small_f_is_imm_f", 181481),      // generic
            ("add_opcode_is_small_t_is_imm_f", 44567),       // generic
            ("add_opcode_is_small_f_is_imm_t", 12141),       // generic
            ("assert_eq_opcode_is_double_deref_f_is_imm_f", 233432), // mov mem
            ("assert_eq_opcode_is_double_deref_t_is_imm_f", 811061), // deref
            ("assert_eq_opcode_is_double_deref_f_is_imm_t", 43184), // push imm
            ("call_opcode_is_rel_f_op1_base_fp_f", 0),
            ("call_opcode_is_rel_t_op1_base_fp_f", 49439), // call rel imm
            ("call_opcode_is_rel_f_op1_base_fp_t", 33),    // cal abs
            ("jnz_opcode_is_taken_t_dst_base_fp_t", 11235), // jnz
            ("jnz_opcode_is_taken_f_dst_base_fp_f", 27032), // jnz
            ("jnz_opcode_is_taken_t_dst_base_fp_f", 51060), // jnz
            ("jnz_opcode_is_taken_f_dst_base_fp_t", 5100), // jnz
            ("jump_opcode_is_rel_t_is_imm_t_is_double_deref_f", 31873866), // jump rel imm
            ("jump_opcode_is_rel_t_is_imm_f_is_double_deref_f", 500), // generic
            ("jump_opcode_is_rel_f_is_imm_f_is_double_deref_t", 32), // generic
            ("jump_opcode_is_rel_f_is_imm_f_is_double_deref_f", 0),
            ("mul_opcode_is_small_t_is_imm_t", 14653), // generic
            ("mul_opcode_is_small_t_is_imm_f", 8574),  // generic
            ("mul_opcode_is_small_f_is_imm_f", 2572),  // generic
            ("mul_opcode_is_small_f_is_imm_t", 3390),  // generic
            ("ret_opcode", 49472),                     // ret
        ]);
        assert_eq!(input.instructions.counts(), expected_counts);
        println!("Instruction counts: {:#?}", input.instructions.counts());
    }

    #[ignore]
    #[test]
    fn test_read_from_small_files() {
        let input = small_cairo_input();
        let expected_counts: IndexMap<&str, usize> = IndexMap::from_iter(vec![
            ("generic_opcode", 0),
            ("add_ap_opcode_is_imm_f_op1_base_fp_f", 0),
            ("add_ap_opcode_is_imm_t_op1_base_fp_f", 2), // add ap
            ("add_ap_opcode_is_imm_f_op1_base_fp_t", 1), // generic
            ("add_opcode_is_small_t_is_imm_t", 750),     // generic
            ("add_opcode_is_small_f_is_imm_f", 0),       // generic
            ("add_opcode_is_small_t_is_imm_f", 0),       // generic
            ("add_opcode_is_small_f_is_imm_t", 200),     // generic
            ("assert_eq_opcode_is_double_deref_f_is_imm_f", 55), // mov mem
            ("assert_eq_opcode_is_double_deref_t_is_imm_f", 2100), // deref
            ("assert_eq_opcode_is_double_deref_f_is_imm_t", 1952), // push imm
            ("call_opcode_is_rel_f_op1_base_fp_f", 0),
            ("call_opcode_is_rel_t_op1_base_fp_f", 462), // call rel imm
            ("call_opcode_is_rel_f_op1_base_fp_t", 0),   // cal abs
            ("jnz_opcode_is_taken_t_dst_base_fp_t", 450), // jnz
            ("jnz_opcode_is_taken_f_dst_base_fp_f", 0),  // jnz
            ("jnz_opcode_is_taken_t_dst_base_fp_f", 0),  // jnz
            ("jnz_opcode_is_taken_f_dst_base_fp_t", 11), // jnz
            ("jump_opcode_is_rel_t_is_imm_t_is_double_deref_f", 124627), // jump rel imm
            ("jump_opcode_is_rel_t_is_imm_f_is_double_deref_f", 0), // generic
            ("jump_opcode_is_rel_f_is_imm_f_is_double_deref_t", 0), // generic
            ("jump_opcode_is_rel_f_is_imm_f_is_double_deref_f", 0),
            ("mul_opcode_is_small_t_is_imm_t", 0), // generic
            ("mul_opcode_is_small_t_is_imm_f", 0), // generic
            ("mul_opcode_is_small_f_is_imm_f", 0), // generic
            ("mul_opcode_is_small_f_is_imm_t", 0), // generic
            ("ret_opcode", 462),                   // ret
        ]);
        assert_eq!(input.instructions.counts(), expected_counts);
        println!("Instruction counts: {:#?}", input.instructions.counts());
    }
}
