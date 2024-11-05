pub mod channel;
pub mod circle;
pub mod fields;
pub mod fri;
pub mod pcs;
pub mod poly;
mod queries;
pub mod utils;
pub mod vcs;
pub mod verifier;

pub use fields::{BaseField, SecureField};

fn main() {}

/// An array in which each element relates (by index) to a column in the trace.
pub type ColumnArray<T> = Array<T>;

/// An array in which each element relates (by index) to a commitment tree.
pub type TreeArray<T> = Array<T>;
