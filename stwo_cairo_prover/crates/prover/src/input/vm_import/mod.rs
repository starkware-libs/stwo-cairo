mod json;

use std::io::Read;
use std::path::Path;

use bytemuck::{bytes_of_mut, Pod, Zeroable};
use json::{PrivateInput, PublicInput};

use super::instructions::Instructions;
use super::mem::{MemConfig, Memory};
use super::CairoInput;

pub fn import_from_vm_output(pub_json: &Path, priv_json: &Path) -> CairoInput {
    let pub_data: PublicInput =
        sonic_rs::from_str(&std::fs::read_to_string(pub_json).unwrap()).unwrap();
    let priv_data: PrivateInput =
        sonic_rs::from_str(&std::fs::read_to_string(priv_json).unwrap()).unwrap();

    let end_addr = pub_data
        .memory_segments
        .values()
        .map(|v| v.stop_ptr)
        .max()
        .unwrap();
    assert!(end_addr < (1 << 32));
    let mem_config = MemConfig::new((1 << 20) - 1, end_addr as u32);

    let mem_path = priv_json.parent().unwrap().join(&priv_data.memory_path);
    let trace_path = priv_json.parent().unwrap().join(&priv_data.trace_path);

    let mut trace_file = std::io::BufReader::new(std::fs::File::open(trace_path).unwrap());
    let mut mem_file = std::io::BufReader::new(std::fs::File::open(mem_path).unwrap());
    let mem = Memory::from_iter(mem_config, MemEntryIter(&mut mem_file));
    let instructions = Instructions::from_iter(TraceIter(&mut trace_file), &mem);

    CairoInput { instructions, mem }
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

    #[test]
    fn test_read_from_files() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("test_data/sample0");
        let input =
            import_from_vm_output(d.join("pub.json").as_path(), d.join("priv.json").as_path());
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

    #[ignore]
    #[test]
    fn test_print_usage() {
        let input = import_from_vm_output(
            Path::new(&std::env::var("TRACE_FILE").unwrap()),
            Path::new(&std::env::var("MEM_FILE").unwrap()),
        );
        println!("Usage: {:#?}", input.instructions.usage());
    }
}
