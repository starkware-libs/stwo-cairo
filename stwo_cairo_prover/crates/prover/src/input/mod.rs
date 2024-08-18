use instructions::Instructions;
use mem::Memory;

mod decode;
pub mod instructions;
pub mod mem;
pub mod vm_import;

pub const N_REGISTERS: usize = 3;

// Externally provided inputs.
pub struct CairoInput {
    pub instructions: Instructions,
    pub mem: Memory,
    pub public_mem_addresses: Vec<u32>,

    // Builtins.
    pub range_check: RangeCheckInput,
}

pub struct RangeCheckInput {
    pub begin_addr: u32,
    pub end_addr: u32,
}
