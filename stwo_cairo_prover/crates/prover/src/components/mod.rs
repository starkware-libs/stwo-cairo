use prover_types::simd::N_LANES;
use stwo_prover::core::backend::simd::conversion::Pack;

pub mod addapopcode_is_imm_t_op1_base_fp_f;
pub mod callopcode_is_rel_t_op1_base_fp_f;
pub mod genericopcode;
pub mod memory;
pub mod range_check_vector;
pub mod ret_opcode;
pub mod verifyinstruction;

pub fn pack_values<T: Pack>(values: &[T]) -> Vec<T::SimdType> {
    values
        .array_chunks::<N_LANES>()
        .map(|c| T::pack(*c))
        .collect()
}
