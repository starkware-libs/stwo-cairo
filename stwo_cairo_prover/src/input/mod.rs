use instructions::Instructions;
use mem::Memory;

pub mod instructions;
pub mod io;
pub mod mem;

// Externally provided inputs.
pub struct CairoInput {
    // Opcodes.
    pub instructions: Instructions,
    // Builtins.

    // Memory.
    pub mem: Memory,
    // Tables. (rc, xor, ...)
}
