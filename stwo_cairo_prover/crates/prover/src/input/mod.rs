use cairo_vm::air_public_input::MemorySegmentAddresses;
use memory::Memory;
use serde::{Deserialize, Serialize};
use state_transitions::StateTransitions;

mod decode;
pub mod memory;
pub mod plain;
pub mod range_check_unit;
pub mod state_transitions;
pub mod vm_import;

pub const N_REGISTERS: usize = 3;

// Externally provided inputs.
#[derive(Debug, Serialize, Deserialize)]
pub struct CairoInput {
    pub state_transitions: StateTransitions,
    pub memory: Memory,
    pub public_memory_addresses: Vec<u32>,

    // Builtins.
    pub range_check_builtin: MemorySegmentAddresses,
}
