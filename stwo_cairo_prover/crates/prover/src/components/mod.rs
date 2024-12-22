use std::mem::transmute;
use std::simd::Simd;

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
/// An aligned vector of `u32` that is used to store the multiplicities of the columns, for logup
/// arguments.
pub struct MultiplicityColumn {
    data: Vec<Simd<u32, N_LANES>>,
}
impl MultiplicityColumn {
    /// Creates a new `MultiplicityColumn` with the given size. The elements are initialized to 0.
    pub fn new(size: usize) -> Self {
        let vec_size = size.next_multiple_of(N_LANES) / N_LANES;
        Self {
            data: vec![unsafe { std::mem::zeroed() }; vec_size],
        }
    }

    pub fn increase_at(&mut self, address: usize) {
        self.data[address / N_LANES][address % N_LANES] += 1;
    }

    /// Returns the internal data as a Vec<PackedM31>.
    pub fn into_vec(self) -> Vec<PackedM31> {
        // Safe because the data is aligned to the size of PackedM31 and the size of the data is a
        // multiple of N_LANES.
        unsafe { transmute(self.data) }
    }
}

#[cfg(test)]
mod tests {
    use num_traits::{One, Zero};
    use rand::rngs::SmallRng;
    use rand::{Rng, SeedableRng};
    use stwo_prover::core::backend::simd::m31::N_LANES;
    use stwo_prover::core::fields::m31::M31;

    #[test]
    fn test_multiplicities_column() {
        let mut multiplicity_column = super::MultiplicityColumn::new(6 * N_LANES - 2);
        let mut vec = vec![M31::zero(); 6 * N_LANES];
        let mut rng = SmallRng::seed_from_u64(0u64);

        (0..10 * N_LANES).for_each(|_| {
            let addr = rng.gen_range(0..N_LANES * 6);
            multiplicity_column.increase_at(addr);
            vec[addr] += M31::one();
        });

        assert!(multiplicity_column.data.len() == 6);
        // The vector should be aligned to the size of PackedM31.
        assert!(
            multiplicity_column.data.as_ptr() as usize % std::mem::size_of::<super::PackedM31>()
                == 0
        );
        for (packed_chunk, vec_chunk) in multiplicity_column
            .into_vec()
            .iter()
            .zip(vec.chunks(N_LANES))
        {
            assert!(packed_chunk.to_array() == vec_chunk);
        }
    }
}
