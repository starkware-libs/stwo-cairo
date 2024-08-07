use std::io::Read;
use std::path::Path;

use bytemuck::{bytes_of_mut, Pod, Zeroable};

use super::instructions::Instructions;
use super::mem::Memory;
use super::CairoInput;

pub fn import_from_vm_output(trace_path: &Path, mem_path: &Path) -> CairoInput {
    let mut mem_file = std::io::BufReader::new(std::fs::File::open(mem_path).unwrap());
    let mem = Memory::from_iter(MemEntryIter(&mut mem_file));
    let mut trace_file = std::io::BufReader::new(std::fs::File::open(trace_path).unwrap());
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

    #[test]
    fn test_read_from_files() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("test_data");
        let _input =
            import_from_vm_output(d.join("trace_file").as_path(), d.join("mem_file").as_path());
    }
}
