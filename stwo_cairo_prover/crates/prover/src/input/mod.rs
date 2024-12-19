use cairo_vm::air_public_input::MemorySegmentAddresses;
use mem::Memory;
use state_transitions::StateTransitions;

mod decode;
pub mod mem;
pub mod plain;
pub mod range_check_unit;
pub mod state_transitions;
pub mod vm_import;

pub const N_REGISTERS: usize = 3;

// Externally provided inputs.
#[derive(Debug)]
pub struct CairoInput {
    pub state_transitions: StateTransitions,
    pub mem: Memory,
    pub public_mem_addresses: Vec<u32>,

    // Builtins.
    pub range_check_builtin: MemorySegmentAddresses,
}
