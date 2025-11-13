use core::box::BoxImpl;
use stwo_constraint_framework::{INVALID_COLUMN_IDX, LookupElementsImpl, PreprocessedMaskValuesImpl};
#[cfg(not(feature: "poseidon252_verifier"))]
pub use super::preprocessed_columns_canonical::*;
#[cfg(feature: "poseidon252_verifier")]
pub use super::preprocessed_columns_without_pedersen::*;

// Make sure INVALID_COLUMN_IDX is not the ID of any column
const INVALID_IDX_CHECK: () = if NUM_PREPROCESSED_COLUMNS >= INVALID_COLUMN_IDX {
    core::panic_with_felt252('invalid idx too small')
};
