mod channel;
mod circle;
mod fields;
mod fri;
mod pcs;
mod poly;
mod queries;
mod utils;
mod vcs;
mod verifier;

pub use fields::{BaseField, SecureField};

fn main() {}

/// An array in which each element relates (by index) to a column in the trace.
pub type ColumnArray<T> = Array<T>;

/// An array in which each element relates (by index) to a commitment tree.
pub type TreeArray<T> = Array<T>;
