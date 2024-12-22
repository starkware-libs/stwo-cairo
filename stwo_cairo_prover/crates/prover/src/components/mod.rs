use std::{alloc, ptr};

use stwo_prover::core::backend::simd::conversion::Pack;
use stwo_prover::core::backend::simd::m31::{PackedM31, N_LANES};

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

// TODO(Gali): Move to somewhere else.
/// An aligned vector of `u32` that is used to store the multiplicities of the columns, for a direct
/// transmuting to Vec<PackedM31>.
pub struct MultiplicityColumn {
    data: Vec<u32>,
}
impl MultiplicityColumn {
    /// Creates a new `MultiplicityColumn` with the given size. The elements are initialized to 0.
    pub fn new(size: usize) -> Self {
        let layout = alloc::Layout::from_size_align(
            size * std::mem::size_of::<u32>(),
            std::mem::size_of::<PackedM31>(),
        )
        .unwrap();
        let vec_ptr = unsafe { alloc::alloc(layout) as *mut u32 };

        unsafe {
            ptr::write_bytes(vec_ptr, 0, size);
        }
        Self {
            data: unsafe { Vec::from_raw_parts(vec_ptr, size, size) },
        }
    }

    /// Transmutes the internal data to a Vec<PackedM31>.
    /// The size of the internal data must be a multiple of N_LANES.
    /// The internal data is forgotten.
    pub fn as_packed_m31(self) -> Vec<PackedM31> {
        assert!(self.data.len() % N_LANES == 0);

        let packed_vec = unsafe {
            Vec::from_raw_parts(
                self.data.as_ptr() as *mut PackedM31,
                self.data.len() / N_LANES,
                self.data.capacity() / N_LANES,
            )
        };
        std::mem::forget(self.data);
        packed_vec
    }
}

#[cfg(test)]
mod tests {
    use num_traits::Zero;
    use stwo_prover::core::backend::simd::m31::N_LANES;
    use stwo_prover::core::fields::m31::M31;

    #[test]
    fn test_multiplicities_column_new() {
        let col_mults = super::MultiplicityColumn::new(N_LANES);

        assert!(col_mults.data.len() == N_LANES);
        assert!(col_mults.data.iter().sum::<u32>() == 0);
        assert!(col_mults.data.as_ptr() as usize % std::mem::size_of::<super::PackedM31>() == 0);
    }

    #[test]
    fn test_multiplicities_column_as_packed_m31() {
        let col_mults = super::MultiplicityColumn::new(N_LANES);
        let packed_vec = col_mults.as_packed_m31();

        assert!(packed_vec.len() == 1);
        assert!(packed_vec[0].to_array().iter().all(|&x| x == M31::zero()));
    }
}
