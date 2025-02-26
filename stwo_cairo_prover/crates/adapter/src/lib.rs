use std::collections::HashMap;

use builtins::BuiltinSegments;
use cairo_vm::types::builtin_name::BuiltinName;
use memory::Memory;
use opcodes::StateTransitions;
use serde::{Deserialize, Serialize};
use stwo_cairo_common::prover_types::cpu::M31;
use vm_import::PublicSegmentRanges;
pub mod builtins;
pub mod decode;
pub mod memory;
pub mod opcodes;
pub mod plain;
pub mod vm_import;

pub const N_REGISTERS: usize = 3;

/// Externally provided inputs for the Stwo prover.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProverInput {
    pub state_transitions: StateTransitions,
    pub instruction_by_pc: HashMap<M31, u128>,
    pub memory: Memory,
    pub public_memory_addresses: Vec<u32>,
    pub builtins_segments: BuiltinSegments,
}

/// Sizes of memory address to ID and ID to value tables.
#[derive(Debug, Serialize, Deserialize)]
pub struct MemoryTablesSizes {
    /// Size of memory address to ID table.
    pub address_to_id: usize,
    /// Size of memory ID to big value table.
    pub id_to_big: usize,
    /// Size of memory ID to small value table.
    pub id_to_small: usize,
}

/// Execution resources required to compute trace size.
#[derive(Debug, Serialize, Deserialize)]
pub struct ExecutionResources {
    /// Map opcode to the number of invocations.
    pub opcode_instance_counts: HashMap<String, usize>,
    /// Map builtin to the number of invocations.
    pub builtin_instance_counts: HashMap<BuiltinName, usize>,
    /// Sizes of memory tables.
    pub memory_tables_sizes: MemoryTablesSizes,
    /// Number of verify instructions, corresponds to the number of unique pc values.
    pub verify_instructions_count: usize,
}

impl ExecutionResources {
    /// Create execution resources from prover input.
    pub fn from_prover_input(input: &ProverInput) -> Self {
        ExecutionResources {
            opcode_instance_counts: input
                .state_transitions
                .casm_states_by_opcode
                .counts()
                .into_iter()
                .collect(),
            builtin_instance_counts: input.builtins_segments.get_counts(),
            memory_tables_sizes: MemoryTablesSizes {
                address_to_id: input.memory.address_to_id.len(),
                id_to_big: input.memory.f252_values.len(),
                id_to_small: input.memory.small_values.len(),
            },
            verify_instructions_count: input.instruction_by_pc.len(),
        }
    }
}
