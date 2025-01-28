use std::collections::HashMap;

use builtin_segments::BuiltinSegments;
use cairo_vm::types::builtin_name::BuiltinName;
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

/// Execution resources required to compute trace size.
#[derive(Debug, Serialize, Deserialize)]
pub struct ExecutionResources {
    /// Map opcode to the number of invocations.
    pub opcodes_instance_counts: HashMap<String, usize>,
    /// Map builtin to the number of invocations.
    pub builtin_instance_counts: HashMap<BuiltinName, usize>,
    /// Sizes of memory address to id and it to value tables.
    pub memory_tables_sizes: MemoryTablesSizes,
    /// Number of verify instructions, corresponds to the number of unique pc values.
    pub verify_instructions_counts: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemoryTablesSizes {
    pub memory_address_to_id_size: usize,
    pub memory_id_to_big_size: usize,
    pub memory_id_to_small_size: usize,
}

impl ExecutionResources {
    /// Create execution resources from prover input.
    pub fn from_prover_input(input: &ProverInput) -> Self {
        ExecutionResources {
            opcodes_instance_counts: input
                .state_transitions
                .casm_states_by_opcode
                .counts()
                .into_iter()
                .collect(),
            builtin_instance_counts: input.builtins_segments.get_builtin_counts(),
            memory_tables_sizes: MemoryTablesSizes {
                memory_address_to_id_size: input.memory.address_to_id.len(),
                memory_id_to_big_size: input.memory.f252_values.len(),
                memory_id_to_small_size: input.memory.small_values.len(),
            },
            verify_instructions_counts: input.instruction_by_pc.len(),
        }
    }
}
