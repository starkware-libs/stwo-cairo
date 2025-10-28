use core::box::BoxImpl;
use stwo_constraint_framework::{LookupElementsImpl, PreprocessedMaskValuesImpl};
#[cfg(not(feature: "poseidon252_verifier"))]
pub use super::preprocessed_columns_canonical::*;
#[cfg(feature: "poseidon252_verifier")]
pub use super::preprocessed_columns_without_pedersen::*;
