pub mod add_ap_opcode;
pub mod add_ap_opcode_imm;
pub mod add_ap_opcode_op_1_base_fp;
pub mod add_mod_builtin;
pub mod add_opcode;
pub mod add_opcode_imm;
pub mod add_opcode_small;
pub mod add_opcode_small_imm;
pub mod assert_eq_opcode;
pub mod assert_eq_opcode_double_deref;
pub mod assert_eq_opcode_imm;
pub mod bitwise_builtin;
pub mod call_opcode;
pub mod call_opcode_op_1_base_fp;
pub mod call_opcode_rel;
pub mod cube_252;
pub mod generic_opcode;
pub mod jnz_opcode;
pub mod jnz_opcode_dst_base_fp;
pub mod jnz_opcode_taken;
pub mod jnz_opcode_taken_dst_base_fp;
pub mod jump_opcode;
pub mod jump_opcode_double_deref;
pub mod jump_opcode_rel;
pub mod jump_opcode_rel_imm;
pub mod memory;
pub mod mul_opcode;
pub mod mul_opcode_imm;
pub mod mul_opcode_small;
pub mod mul_opcode_small_imm;
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
pub mod utils;
pub mod verify_bitwise_xor_12;
pub mod verify_bitwise_xor_4;
pub mod verify_bitwise_xor_7;
pub mod verify_bitwise_xor_8;
pub mod verify_bitwise_xor_9;
pub mod verify_instruction;

pub(crate) mod prelude;

pub use memory::{memory_address_to_id, memory_id_to_big};
pub use range_check_vector::{
    range_check_11, range_check_19, range_check_4_3, range_check_4_4_4_4, range_check_6,
    range_check_7_2_5, range_check_9_9,
};
pub mod blake_compress_opcode;
pub mod blake_g;
pub mod blake_round;
pub mod blake_round_sigma;

#[cfg(feature = "slow-tests")]
#[cfg(test)]
mod tests {
    use itertools::chain;
    use stwo_cairo_adapter::vm_import::generate_test_input;

    use crate::cairo_air::debug_tools::assert_constraints::assert_cairo_constraints;
    use crate::cairo_air::preprocessed::PreProcessedTrace;

    #[test]
    fn test_generic_opcode() {
        let mut input = generate_test_input("test_prove_verify_all_opcode_components");
        let casm_state_by_opcode = &mut input.state_transitions.casm_states_by_opcode;
        casm_state_by_opcode.generic_opcode.extend(chain!(
            casm_state_by_opcode.add_ap_opcode.drain(..),
            casm_state_by_opcode.add_ap_opcode_imm.drain(..),
            casm_state_by_opcode.add_ap_opcode_op_1_base_fp.drain(..),
            casm_state_by_opcode.add_opcode_small_imm.drain(..),
            casm_state_by_opcode.add_opcode.drain(..),
            casm_state_by_opcode.add_opcode_small.drain(..),
            casm_state_by_opcode.add_opcode_imm.drain(..),
            casm_state_by_opcode.assert_eq_opcode.drain(..),
            casm_state_by_opcode.assert_eq_opcode_double_deref.drain(..),
            casm_state_by_opcode.assert_eq_opcode_imm.drain(..),
            casm_state_by_opcode.call_opcode.drain(..),
            casm_state_by_opcode.call_opcode_rel.drain(..),
            casm_state_by_opcode.call_opcode_op_1_base_fp.drain(..),
            casm_state_by_opcode.jnz_opcode_taken_dst_base_fp.drain(..),
            casm_state_by_opcode.jnz_opcode.drain(..),
            casm_state_by_opcode.jnz_opcode_taken.drain(..),
            casm_state_by_opcode.jnz_opcode_dst_base_fp.drain(..),
            casm_state_by_opcode.jump_opcode_rel_imm.drain(..),
            casm_state_by_opcode.jump_opcode_rel.drain(..),
            casm_state_by_opcode.jump_opcode_double_deref.drain(..),
            casm_state_by_opcode.jump_opcode.drain(..),
            casm_state_by_opcode.mul_opcode_small_imm.drain(..),
            casm_state_by_opcode.mul_opcode_small.drain(..),
            casm_state_by_opcode.mul_opcode.drain(..),
            casm_state_by_opcode.mul_opcode_imm.drain(..),
            casm_state_by_opcode.ret_opcode.drain(..),
        ));

        let preprocessed_trace = PreProcessedTrace::canonical_without_pedersen();
        assert_cairo_constraints(input, preprocessed_trace);
    }
}
