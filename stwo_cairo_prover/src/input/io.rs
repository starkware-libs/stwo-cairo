use std::io::Read;
use std::path::Path;

use bytemuck::{bytes_of_mut, Pod, Zeroable};

use super::instructions::Instructions;
use super::mem::Memory;
use super::CairoInput;

pub fn cairo_input_from_files(trace_path: &Path, mem_path: &Path) -> CairoInput {
    let mut mem_file = std::io::BufReader::new(std::fs::File::open(mem_path).unwrap());
    let mem = Memory::from_iter(MemEntryIter(&mut mem_file));
    let mut trace_file = std::io::BufReader::new(std::fs::File::open(trace_path).unwrap());
    let instructions = Instructions::from_iter(TraceIter(&mut trace_file), &mem);

    CairoInput { instructions, mem }
}

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

    use super::*;

    #[test]
    fn test_read_from_files() {
        let _input = cairo_input_from_files(
            Path::new("/home/spapini/Downloads/trace_file_sn"),
            Path::new("/home/spapini/Downloads/mem_file_sn"),
        );
    }
}
