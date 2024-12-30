use builtin_segments::BuiltinSegments;
use memory::Memory;
use state_transitions::StateTransitions;

pub mod builtin_segments;
mod decode;
pub mod memory;
pub mod plain;
pub mod range_check_unit;
pub mod state_transitions;
pub mod vm_import;

pub const N_REGISTERS: usize = 3;

/// Externally provided inputs for the Stwo prover.
#[derive(Debug)]
pub struct ProverInput {
    pub state_transitions: StateTransitions,
    pub max_pc: u32,
    pub memory: Memory,
    pub public_memory_addresses: Vec<u32>,
    pub builtins_segments: BuiltinSegments,
}
