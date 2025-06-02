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
pub mod call_opcode_rel_imm;
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
    range_check_3_6_6_3, range_check_4_3, range_check_4_4, range_check_4_4_4_4, range_check_5_4,
    range_check_6, range_check_7_2_5, range_check_8, range_check_9_9,
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
