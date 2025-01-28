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
    pub opcodes_instance_counter: HashMap<String, usize>,
    pub builtin_instance_counter: HashMap<BuiltinName, usize>,
    pub memory_tables_sizes: HashMap<String, usize>,
    pub verify_instructions_counter: usize,
}
impl ExecutionResources {
    pub fn new(
        opcodes_instance_counter: HashMap<String, usize>,
        builtin_instance_counter: HashMap<BuiltinName, usize>,
        memory_tables_sizes: HashMap<String, usize>,
        verify_instructions_counter: usize,
    ) -> Self {
        ExecutionResources {
            opcodes_instance_counter,
            builtin_instance_counter,
            memory_tables_sizes,
            verify_instructions_counter,
        }
    }
    pub fn from_prover_input(input: &ProverInput) -> Self {
        ExecutionResources {
            opcodes_instance_counter: input
                .state_transitions
                .casm_states_by_opcode
                .counts()
                .into_iter()
                .map(|(s, count)| (s.to_string(), count))
                .collect(),
            builtin_instance_counter: input.builtins_segments.get_builtin_usage_counter(),
            memory_tables_sizes: {
                let mut map = HashMap::new();
                map.insert(
                    "memory_address_to_id_size".to_string(),
                    input.memory.address_to_id.len(),
                );
                map.insert(
                    "memory_id_to_big_size".to_string(),
                    input.memory.f252_values.len(),
                );
                map.insert(
                    "memory_id_to_small_size".to_string(),
                    input.memory.small_values.len(),
                );
                map
            },
            verify_instructions_counter: input.instruction_by_pc.len(),
        }
    }
}
