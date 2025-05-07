pub mod add_ap_opcode;
pub mod add_mod_builtin;
pub mod add_opcode;
pub mod add_opcode_small;
pub mod assert_eq_opcode;
pub mod assert_eq_opcode_double_deref;
pub mod assert_eq_opcode_imm;
pub mod bitwise_builtin;
pub mod blake_compress_opcode;
pub mod blake_g;
pub mod blake_round;
pub mod blake_round_sigma;
pub mod call_opcode;
pub mod call_opcode_op_1_base_fp;
pub mod call_opcode_rel;
pub mod cube_252;
pub mod generic_opcode;
pub mod jnz_opcode;
pub mod jnz_opcode_taken;
pub mod jump_opcode;
pub mod jump_opcode_double_deref;
pub mod jump_opcode_rel;
pub mod jump_opcode_rel_imm;
pub mod memory_address_to_id;
pub mod memory_id_to_big;
pub mod mul_mod_builtin;
pub mod mul_opcode;
pub mod mul_opcode_small;
pub mod partial_ec_mul;
pub mod pedersen_builtin;
pub mod pedersen_points_table;
pub mod poseidon_3_partial_rounds_chain;
pub mod poseidon_builtin;
pub mod poseidon_full_round_chain;
pub mod poseidon_round_keys;
pub mod qm_31_add_mul_opcode;
pub mod range_check_builtin_bits_128;
pub mod range_check_builtin_bits_96;
pub mod range_check_felt_252_width_27;
pub mod range_check_vector;
pub mod ret_opcode;
pub mod triple_xor_32;
pub mod verify_bitwise_xor_12;
pub mod verify_bitwise_xor_4;
pub mod verify_bitwise_xor_7;
pub mod verify_bitwise_xor_8;
pub mod verify_bitwise_xor_9;
pub mod verify_instruction;

pub(crate) mod prelude;
pub(crate) mod subroutines;

use itertools::Itertools;
pub use range_check_vector::{
    range_check_11, range_check_12, range_check_18, range_check_19, range_check_3_3_3_3_3,
    range_check_3_6, range_check_3_6_6_3, range_check_4_3, range_check_4_4, range_check_4_4_4_4,
    range_check_5_4, range_check_6, range_check_7_2_5, range_check_8, range_check_9_9,
};
use stwo_prover::constraint_framework::{FrameworkComponent, FrameworkEval};

#[cfg(test)]
pub mod constraints_regression_test_values;

// TODO(Ohad): move somewhere else.
pub(crate) fn indented_component_display<E: FrameworkEval>(
    component: &FrameworkComponent<E>,
) -> String {
    let component_display = &format!("\n{}", component);
    component_display
        .lines()
        .map(|line| format!("\t{}", line))
        .join("\n")
}

pub(crate) fn display_components<E: FrameworkEval>(components: &[FrameworkComponent<E>]) -> String {
    components
        .iter()
        .map(|component| indented_component_display(component))
        .join("\n")
}
pub mod read_positive_num_bits_96;
pub mod linear_combination_n_1_coefs_2;
pub mod split_16_low_part_size_8;
pub mod poseidon_partial_round;
pub mod single_karatsuba_n_8;
pub mod linear_combination_n_4_coefs_1_1_m2_1;
pub mod single_karatsuba_n_7;
pub mod decode_instruction_cb32b;
pub mod split_16_low_part_size_12;
pub mod bitwise_xor_num_bits_4;
pub mod read_positive_num_bits_27;
pub mod decode_instruction_64420;
pub mod decode_instruction_ea769;
pub mod linear_combination_n_4_coefs_4_2_m2_1;
pub mod range_check_last_limb_bits_in_ms_limb_2;
pub mod mul_252;
pub mod decode_instruction_3b105;
pub mod read_positive_num_bits_144;
pub mod qm_31_read_reduced;
pub mod decode_instruction_2a7a2;
pub mod read_blake_word;
pub mod decode_instruction_161c9;
pub mod linear_combination_n_4_coefs_1_m1_1_1;
pub mod cond_decode_small_sign;
pub mod add_252;
pub mod read_split;
pub mod read_positive_num_bits_72;
pub mod range_check_mem_value_n_28;
pub mod cond_felt_252_as_addr;
pub mod split_16_low_part_size_7;
pub mod xor_rot_32_r_8;
pub mod ec_add;
pub mod linear_combination_n_4_coefs_4_2_1_1;
pub mod xor_rot_32_r_12;
pub mod xor_rot_32_r_7;
pub mod div_252;
pub mod decode_instruction_bc3cd;
pub mod felt_252_unpack_from_27_range_check_output;
pub mod decode_instruction_fe864;
pub mod verify_mul_252;
pub mod verify_mul_small;
pub mod decode_instruction_7ebc4;
pub mod verify_add_252;
pub mod xor_rot_32_r_16;
pub mod double_karatsuba_n_8_limb_max_bound_4095;
pub mod read_small;
pub mod bitwise_xor_num_bits_9;
pub mod verify_blake_word;
pub mod decode_instruction_fdb6e;
pub mod linear_combination_n_2_coefs_1_1;
pub mod bitwise_xor_num_bits_7;
pub mod poseidon_hades_permutation;
pub mod decode_instruction_df7a6;
pub mod eval_operands;
pub mod create_blake_output;
pub mod mod_utils;
pub mod mem_verify;
pub mod triple_sum_32;
pub mod read_positive_num_bits_252;
pub mod mod_words_to_12_bit_array;
pub mod cond_felt_252_as_rel_imm;
pub mod decode_generic_instruction;
pub mod read_positive_num_bits_128;
pub mod decode_blake_opcode;
pub mod decode_instruction_15a61;
pub mod bitwise_xor_num_bits_8;
pub mod felt_252_unpack_from_27;
pub mod encode_offsets;
pub mod decode_instruction_43e1c;
pub mod read_positive_num_bits_36;
pub mod decode_instruction_3802d;
pub mod decode_instruction_4b8cf;
pub mod handle_opcodes;
pub mod linear_combination_n_4_coefs_3_1_1_1;
pub mod decode_instruction_de75a;
pub mod decode_instruction_d2a10;
pub mod create_blake_round_input;
pub mod bitwise_xor_num_bits_12;
pub mod decode_instruction_9bd86;
pub mod mem_verify_equal;
pub mod double_karatsuba_n_7_limb_max_bound_511;
pub mod range_check_last_limb_bits_in_ms_limb_6;
pub mod mem_cond_verify_equal_known_id;
pub mod linear_combination_n_6_coefs_4_2_3_1_m1_1;
pub mod verify_reduced_252;
pub mod sub_252;
pub mod update_registers;
pub mod read_positive_num_bits_99;
pub mod range_check_19;
pub mod range_check_5_4;
pub mod range_check_11;
pub mod range_check_12;
pub mod range_check_3_3_3_3_3;
pub mod range_check_18;
pub mod range_check_6;
pub mod range_check_4_4;
pub mod range_check_8;
pub mod range_check_9_9;
pub mod range_check_4_4_4_4;
pub mod range_check_3_6_6_3;
pub mod range_check_4_3;
pub mod range_check_7_2_5;
