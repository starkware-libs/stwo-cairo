use prover_types::simd::N_LANES;
use stwo_prover::core::backend::simd::conversion::Pack;

pub mod generic_opcode;
pub mod memory;
pub mod range_check_vector;
pub mod ret_opcode;
pub mod verify_instruction;

pub use memory::{memory_address_to_id, memory_id_to_big};
pub use range_check_vector::{range_check_19, range_check_4_3, range_check_7_2_5, range_check_9_9};

// [`SubComponentInputs::bit_reverse_coset_to_circle_domain_order`] permutes the
// bit-reversed-canonic-coset ordered values to the natural order. Used when padding is done, to
// ignore padded values at the tail of the natural ordered vectored .

pub fn pack_values<T: Pack>(values: &[T]) -> Vec<T::SimdType> {
    values
        .array_chunks::<N_LANES>()
        .map(|c| T::pack(*c))
        .collect()
}
