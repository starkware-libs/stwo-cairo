use std::array;
use std::cell::UnsafeCell;

use prover_types::simd::N_LANES;
use stwo_prover::core::backend::simd::column::BaseColumn;
use stwo_prover::core::backend::simd::conversion::Pack;
use stwo_prover::core::backend::Column;

pub mod add_ap_opcode_is_imm_f_op1_base_fp_f;
pub mod add_ap_opcode_is_imm_f_op1_base_fp_t;
pub mod add_ap_opcode_is_imm_t_op1_base_fp_f;
pub mod add_opcode_is_small_f_is_imm_f;
pub mod add_opcode_is_small_f_is_imm_t;
pub mod add_opcode_is_small_t_is_imm_f;
pub mod add_opcode_is_small_t_is_imm_t;
pub mod assert_eq_opcode_is_double_deref_f_is_imm_f;
pub mod assert_eq_opcode_is_double_deref_f_is_imm_t;
pub mod assert_eq_opcode_is_double_deref_t_is_imm_f;
pub mod call_opcode_is_rel_f_op1_base_fp_f;
pub mod call_opcode_is_rel_f_op1_base_fp_t;
pub mod call_opcode_is_rel_t_op1_base_fp_f;
pub mod generic_opcode;
pub mod jnz_opcode_is_taken_f_dst_base_fp_f;
pub mod jnz_opcode_is_taken_f_dst_base_fp_t;
pub mod jnz_opcode_is_taken_t_dst_base_fp_f;
pub mod jnz_opcode_is_taken_t_dst_base_fp_t;
pub mod jump_opcode_is_rel_f_is_imm_f_is_double_deref_f;
pub mod jump_opcode_is_rel_f_is_imm_f_is_double_deref_t;
pub mod jump_opcode_is_rel_t_is_imm_f_is_double_deref_f;
pub mod jump_opcode_is_rel_t_is_imm_t_is_double_deref_f;
pub mod memory;
pub mod range_check_vector;
pub mod ret_opcode;
pub mod verify_instruction;

// TODO(Ohad): mul small.
pub mod mul_opcode_is_small_f_is_imm_f;
pub mod mul_opcode_is_small_f_is_imm_t;

pub use memory::{memory_address_to_id, memory_id_to_big};
pub use range_check_vector::{range_check_19, range_check_4_3, range_check_7_2_5, range_check_9_9};

// When padding is needed, the inputs must be arranged in the order defined by the neighbor
// function. This order allows using the partial sum mechanism to sum only the first n_call inputs.
// After getting the `SubComponentInputs` we apply the permutation again to ignore padded values at
// the tail of the vector.
// TODO(Ohad): generalize the padding logic, and move above doc to the relevant function.

pub fn pack_values<T: Pack>(values: &[T]) -> Vec<T::SimdType> {
    values
        .array_chunks::<N_LANES>()
        .map(|c| T::pack(*c))
        .collect()
}

// TODO(Ohad): move to another file.
struct ThreadSafeTrace<const N: usize> {
    data: [UnsafeCell<BaseColumn>; N],
}

unsafe impl<const N: usize> Send for ThreadSafeTrace<N> {}
unsafe impl<const N: usize> Sync for ThreadSafeTrace<N> {}

impl<const N: usize> ThreadSafeTrace<N> {
    fn new(rows: usize) -> Self {
        Self {
            data: array::from_fn(|_| UnsafeCell::new(unsafe { BaseColumn::uninitialized(rows) })),
        }
    }

    pub fn inner(self) -> [BaseColumn; N] {
        self.data.map(|c| c.into_inner())
    }
}
