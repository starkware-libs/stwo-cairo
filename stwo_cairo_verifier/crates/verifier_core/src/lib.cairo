pub mod channel;

pub mod circle;
mod circle_mul_table;
#[cfg(test)]
mod circle_mul_table_test;
#[cfg(test)]
mod circle_test;

pub mod fields;
#[cfg(test)]
mod fields_test;

pub mod fri;
#[cfg(test)]
mod fri_test;

pub mod pcs;
#[cfg(test)]
mod pcs_test;

pub mod poly;

mod queries;
#[cfg(test)]
mod queries_test;

pub mod utils;
#[cfg(test)]
mod utils_test;

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
