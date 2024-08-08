use instructions::Instructions;
use mem::Memory;

pub mod instructions;
pub mod mem;
pub mod vm_import;

// Externally provided inputs.
pub struct CairoInput {
    // Opcodes.
    pub instructions: Instructions,
    // Builtins.

    // Memory.
    pub mem: Memory,
    // Tables. (rc, xor, ...)
}
