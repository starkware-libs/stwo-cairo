use std::collections::HashMap;

use builtin_segments::BuiltinSegments;
use memory::Memory;
use prover_types::cpu::M31;
use serde::{Deserialize, Serialize};
use state_transitions::StateTransitions;

pub mod builtin_segments;
pub mod decode;
pub mod memory;
pub mod plain;
pub mod range_check_unit;
pub mod state_transitions;
pub mod vm_import;

pub const N_REGISTERS: usize = 3;

/// Externally provided inputs for the Stwo prover.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProverInput {
    pub state_transitions: StateTransitions,
    pub instruction_by_pc: HashMap<M31, u64>,
    pub memory: Memory,
    pub public_memory_addresses: Vec<u32>,
    pub builtins_segments: BuiltinSegments,
}
