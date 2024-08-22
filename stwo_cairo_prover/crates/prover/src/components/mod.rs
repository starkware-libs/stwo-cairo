pub mod memory;
pub mod range_check_builtin;
pub mod range_check_unit;
pub mod ret_opcode;

// TODO(ShaharS): Move to a common file.
pub const LOOKUP_INTERACTION_PHASE: usize = 1;
// TODO(AlonH): Fallback to CPU if smaller.
pub const MIN_SIMD_TRACE_LENGTH: usize = 64;
