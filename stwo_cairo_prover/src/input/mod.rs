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
}
