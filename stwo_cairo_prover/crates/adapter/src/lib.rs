use std::ops::Deref;

use builtins::BuiltinSegments;
pub use cairo_vm::stdlib::collections::HashMap;
use cairo_vm::types::builtin_name::BuiltinName;
use memory::Memory;
use opcodes::StateTransitions;
use serde::{Deserialize, Serialize};

pub mod adapter;
pub mod builtins;
pub mod decode;
pub mod memory;
pub mod opcodes;
pub mod relocator;
pub mod test_utils;
pub mod vm_import;

pub const N_REGISTERS: usize = 3;

/// Externally provided inputs for the Stwo prover.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProverInput {
    pub state_transitions: StateTransitions,
    pub memory: Memory,
    pub inst_cache: Vec<(u32, u128)>,
    pub public_memory_addresses: Vec<u32>,
    pub builtins_segments: BuiltinSegments,
    pub public_segment_context: PublicSegmentContext,
}

const N_PUBLIC_SEGMENTS: usize = 11;

/// Represents the pointer arguments of the `main` function.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PublicSegmentContext {
    present: [bool; N_PUBLIC_SEGMENTS],
}
impl PublicSegmentContext {
    pub fn new(vm_format: &[BuiltinName]) -> Self {
        let mut present = [false; N_PUBLIC_SEGMENTS];
        for builtin in vm_format {
            match builtin {
                BuiltinName::output => present[0] = true,
                BuiltinName::pedersen => present[1] = true,
                BuiltinName::range_check => present[2] = true,
                BuiltinName::ecdsa => present[3] = true,
                BuiltinName::bitwise => present[4] = true,
                BuiltinName::ec_op => present[5] = true,
                BuiltinName::keccak => present[6] = true,
                BuiltinName::poseidon => present[7] = true,
                BuiltinName::range_check96 => present[8] = true,
                BuiltinName::add_mod => present[9] = true,
                BuiltinName::mul_mod => present[10] = true,
                BuiltinName::segment_arena => {
                    // Do nothing.
                }
            }
        }
        Self { present }
    }

    pub const fn bootloader_context() -> Self {
        // Bootloader always uses every builtin.
        Self {
            present: [true; N_PUBLIC_SEGMENTS],
        }
    }
}
impl Deref for PublicSegmentContext {
    type Target = [bool; N_PUBLIC_SEGMENTS];

    fn deref(&self) -> &Self::Target {
        &self.present
    }
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
            verify_instructions_count: input.inst_cache.len(),
        }
    }
}

#[cfg(test)]
#[macro_export]
macro_rules! casm_state {
    ($val1 : expr, $val2 : expr, $val3: expr) => {
        CasmState {
            pc: M31($val1),
            ap: M31($val2),
            fp: M31($val3),
        }
    };
}

#[cfg(test)]
#[macro_export]
macro_rules! relocated_trace_entry {
    ($val1 : expr, $val2 : expr, $val3: expr) => {
        RelocatedTraceEntry {
            ap: $val1,
            fp: $val2,
            pc: $val3,
        }
    };
}

pub fn log_prover_input(
    ProverInput {
        state_transitions,
        memory,
        inst_cache,
        public_memory_addresses,
        builtins_segments,
        public_segment_context,
    }: &ProverInput,
) {
    log_memory(memory);
    log::info!(
        "Casm states by opcode:\n{}",
        state_transitions.casm_states_by_opcode
    );

    let _ = public_memory_addresses;
    let _ = inst_cache;
    let _ = public_segment_context;
    let _ = builtins_segments;
}

fn log_memory(memory: &Memory) {
    let mut msg = String::new();
    let n_address_to_id = memory.address_to_id.len();
    let n_big_values = memory.f252_values.len();
    let n_id_to_value = n_big_values + memory.small_values.len();
    msg.push_str(&format!(
        "Address to ID: {:?}, 2 ** {:.2?}",
        n_address_to_id,
        (memory.address_to_id.len() as f64).log2()
    ));
    msg.push('\n');
    msg.push_str(&format!(
        "ID to VALUE, big values: {:?}, 2 ** {:.2?}",
        n_big_values,
        (n_big_values as f64).log2()
    ));
    msg.push('\n');
    msg.push_str(&format!(
        "ID to VALUE, small values: {:?}, 2 ** {:.2?}",
        n_id_to_value - n_big_values,
        ((n_id_to_value - n_big_values) as f64).log2()
    ));
    msg.push('\n');
    msg.push_str(&format!(
        "ID to VALUE (big + small): {:?}, 2 ** {:.2?}",
        n_id_to_value,
        (n_id_to_value as f64).log2()
    ));
    log::info!("Memory resources:\n{}", msg);
}
