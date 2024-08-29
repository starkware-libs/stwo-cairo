mod json;

use std::io::Read;
use std::path::Path;

use bytemuck::{bytes_of_mut, Pod, Zeroable};
use json::{PrivateInput, PublicInput};
use thiserror::Error;

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
    let pub_data: PublicInput = sonic_rs::from_str(&std::fs::read_to_string(pub_json)?)?;
    let priv_data: PrivateInput = sonic_rs::from_str(&std::fs::read_to_string(priv_json)?)?;

    let end_addr = 1 << 26; // TODO: Find a better wat to get maximum address.
    let mem_config = MemConfig::new((1 << 20) - 1, end_addr as u32);

    let mem_path = priv_json.parent().unwrap().join(&priv_data.memory_path);
    let trace_path = priv_json.parent().unwrap().join(&priv_data.trace_path);

    let mut trace_file = std::io::BufReader::new(std::fs::File::open(trace_path)?);
    let mut mem_file = std::io::BufReader::new(std::fs::File::open(mem_path)?);
    let mut mem = MemoryBuilder::from_iter(mem_config, MemEntryIter(&mut mem_file));
    let (instructions, aux) = Instructions::from_iter(TraceIter(&mut trace_file), &mut mem);

    let public_mem_addresses = pub_data
        .public_memory
        .iter()
        .map(|entry| entry.address as u32)
        .collect();

    Ok(CairoInput {
        aux,
        instructions,
        mem: mem.build(),
        public_mem_addresses,
        range_check: SegmentAddrs {
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
mod tests {

    use std::path::PathBuf;

    use super::*;
    use crate::input::instructions::InstructionUsage;

    // Slow test. Run only in release.
    #[test]
    fn test_read_from_files() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("test_data/sample0");
        let input =
            import_from_vm_output(d.join("pub.json").as_path(), d.join("priv.json").as_path())
                .expect(
                    "
            Failed to read test files. Maybe git-lfs is not installed? Checkout README.md.",
                );
        assert_eq!(
            input.instructions.usage(),
            InstructionUsage {
                ret: 49472,
                add_ap: 36895,
                jmp_rel_imm: [31873866, 0],
                jmp_abs: [0, 0, 0, 0],
                call_rel_imm: 49439,
                call_abs: [0, 33],
                jnz_imm: [20957, 5100, 30113, 11235, 6075, 0, 20947, 0],
                mov_mem: 233432,
                deref: 811061,
                push_imm: 43184,
                generic: 362623
            }
        );
        println!("Usage: {:#?}", input.instructions.usage());
    }
}
