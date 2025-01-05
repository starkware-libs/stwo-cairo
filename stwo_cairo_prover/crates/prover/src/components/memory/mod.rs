pub mod memory_address_to_id;
pub mod memory_id_to_big;

/// Used for sanity checks and assertions.
pub const LOG_MEMORY_ADDRESS_BOUND: u32 = 27;
pub const MEMORY_ADDRESS_BOUND: usize = 1 << LOG_MEMORY_ADDRESS_BOUND;
