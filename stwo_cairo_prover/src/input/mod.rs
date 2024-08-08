use instructions::Instructions;
use mem::Memory;

pub mod instructions;
pub mod mem;
pub mod vm_import;

// Externally provided inputs.
pub struct CairoInput {
    pub instructions: Instructions,
    pub mem: Memory,
}
