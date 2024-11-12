use stwo_prover::constraint_framework::INTERACTION_TRACE_IDX;

pub mod memory;
pub mod range_check_builtin;
pub mod range_check_unit;
pub mod range_check_vector;
pub mod ret_opcode;

// TODO(ShaharS): Move to a common file.
pub const LOOKUP_INTERACTION_PHASE: usize = INTERACTION_TRACE_IDX;
