use prover_types::simd::N_LANES;
use stwo_prover::core::backend::simd::conversion::Pack;

pub mod memory;
pub mod opcodes;
pub mod range_check_unit;
pub mod range_check_vector;
pub mod verifyinstruction;

pub fn pack_values<T: Pack>(values: &[T]) -> Vec<T::SimdType> {
    values
        .array_chunks::<N_LANES>()
        .map(|c| T::pack(*c))
        .collect()
}
