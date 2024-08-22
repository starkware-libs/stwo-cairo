use instructions::Instructions;
use mem::Memory;

mod decode;
pub mod instructions;
pub mod mem;
pub mod plain;
pub mod vm_import;

pub const N_REGISTERS: usize = 3;

// Externally provided inputs.
#[derive(Debug)]
pub struct CairoInput {
    pub instructions: Instructions,
    pub mem: Memory,
    pub public_mem_addresses: Vec<u32>,

    // Builtins.
    pub range_check: SegmentAddrs,
}

#[derive(Clone, Debug)]
pub struct SegmentAddrs {
    pub begin_addr: u32,
    pub end_addr: u32,
}

impl SegmentAddrs {
    pub fn addresses(&self) -> Vec<u32> {
        (self.begin_addr..self.end_addr).collect()
    }
}
