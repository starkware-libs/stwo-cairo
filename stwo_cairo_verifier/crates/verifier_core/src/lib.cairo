pub mod channel;
pub mod circle;
mod circle_mul_table;
pub mod fields;
pub mod fri;
pub mod pcs;
pub mod poly;
mod queries;
pub mod utils;
pub mod vcs;
pub mod verifier;

pub use fields::{BaseField, SecureField};

/// An array in which each element relates (by index) to a column in the trace.
pub type ColumnArray<T> = Array<T>;

/// A span in which each element relates (by index) to a column in the trace.
pub type ColumnSpan<T> = Span<T>;

/// An array in which each element relates (by index) to a commitment tree.
pub type TreeArray<T> = Array<T>;

/// A span in which each element relates (by index) to a commitment tree.
pub type TreeSpan<T> = Span<T>;

#[cfg(not(feature: "poseidon252_verifier"))]
pub type Hash = vcs::blake2s_hasher::Blake2sHash;

#[cfg(feature: "poseidon252_verifier")]
pub type Hash = felt252;

// TODO: Stone uses a different initial state with the key set to 0.
// Consider using this initial state instead.
pub const BLAKE2S_256_INITIAL_STATE: [u32; 8] = [
    0x6B08E647, 0xBB67AE85, 0x3C6EF372, 0xA54FF53A, 0x510E527F, 0x9B05688C, 0x1F83D9AB, 0x5BE0CD19,
];
