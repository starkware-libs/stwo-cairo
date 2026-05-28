// This file was created by the AIR team.

use crate::components::*;

// Component index constants (memory_id_to_big: base only; the rest are at base + 0..N).
pub const ADD_OPCODE_COMPONENT_IDX: usize = 0;
pub const ADD_OPCODE_SMALL_COMPONENT_IDX: usize = 1;
pub const ADD_AP_OPCODE_COMPONENT_IDX: usize = 2;
pub const ASSERT_EQ_OPCODE_COMPONENT_IDX: usize = 3;
pub const ASSERT_EQ_OPCODE_IMM_COMPONENT_IDX: usize = 4;
pub const ASSERT_EQ_OPCODE_DOUBLE_DEREF_COMPONENT_IDX: usize = 5;
pub const BLAKE_COMPRESS_OPCODE_COMPONENT_IDX: usize = 6;
pub const CALL_OPCODE_ABS_COMPONENT_IDX: usize = 7;
pub const CALL_OPCODE_REL_IMM_COMPONENT_IDX: usize = 8;
pub const GENERIC_OPCODE_COMPONENT_IDX: usize = 9;
pub const JNZ_OPCODE_NON_TAKEN_COMPONENT_IDX: usize = 10;
pub const JNZ_OPCODE_TAKEN_COMPONENT_IDX: usize = 11;
pub const JUMP_OPCODE_ABS_COMPONENT_IDX: usize = 12;
pub const JUMP_OPCODE_DOUBLE_DEREF_COMPONENT_IDX: usize = 13;
pub const JUMP_OPCODE_REL_COMPONENT_IDX: usize = 14;
pub const JUMP_OPCODE_REL_IMM_COMPONENT_IDX: usize = 15;
pub const MUL_OPCODE_COMPONENT_IDX: usize = 16;
pub const MUL_OPCODE_SMALL_COMPONENT_IDX: usize = 17;
pub const QM_31_ADD_MUL_OPCODE_COMPONENT_IDX: usize = 18;
pub const RET_OPCODE_COMPONENT_IDX: usize = 19;
pub const VERIFY_INSTRUCTION_COMPONENT_IDX: usize = 20;
pub const BLAKE_ROUND_COMPONENT_IDX: usize = 21;
pub const BLAKE_G_COMPONENT_IDX: usize = 22;
pub const BLAKE_ROUND_SIGMA_COMPONENT_IDX: usize = 23;
pub const TRIPLE_XOR_32_COMPONENT_IDX: usize = 24;
pub const VERIFY_BITWISE_XOR_12_COMPONENT_IDX: usize = 25;
pub const ADD_MOD_BUILTIN_COMPONENT_IDX: usize = 26;
pub const BITWISE_BUILTIN_COMPONENT_IDX: usize = 27;
pub const MUL_MOD_BUILTIN_COMPONENT_IDX: usize = 28;
pub const PEDERSEN_BUILTIN_COMPONENT_IDX: usize = 29;
pub const PEDERSEN_BUILTIN_NARROW_WINDOWS_COMPONENT_IDX: usize = 30;
pub const POSEIDON_BUILTIN_COMPONENT_IDX: usize = 31;
pub const RANGE_CHECK96_BUILTIN_COMPONENT_IDX: usize = 32;
pub const RANGE_CHECK_BUILTIN_COMPONENT_IDX: usize = 33;
pub const EC_OP_BUILTIN_COMPONENT_IDX: usize = 34;
pub const PARTIAL_EC_MUL_GENERIC_COMPONENT_IDX: usize = 35;
pub const PEDERSEN_AGGREGATOR_WINDOW_BITS_18_COMPONENT_IDX: usize = 36;
pub const PARTIAL_EC_MUL_WINDOW_BITS_18_COMPONENT_IDX: usize = 37;
pub const PEDERSEN_POINTS_TABLE_WINDOW_BITS_18_COMPONENT_IDX: usize = 38;
pub const PEDERSEN_AGGREGATOR_WINDOW_BITS_9_COMPONENT_IDX: usize = 39;
pub const PARTIAL_EC_MUL_WINDOW_BITS_9_COMPONENT_IDX: usize = 40;
pub const PEDERSEN_POINTS_TABLE_WINDOW_BITS_9_COMPONENT_IDX: usize = 41;
pub const POSEIDON_AGGREGATOR_COMPONENT_IDX: usize = 42;
pub const POSEIDON_3_PARTIAL_ROUNDS_CHAIN_COMPONENT_IDX: usize = 43;
pub const POSEIDON_FULL_ROUND_CHAIN_COMPONENT_IDX: usize = 44;
pub const CUBE_252_COMPONENT_IDX: usize = 45;
pub const POSEIDON_ROUND_KEYS_COMPONENT_IDX: usize = 46;
pub const RANGE_CHECK_252_WIDTH_27_COMPONENT_IDX: usize = 47;
pub const MEMORY_ADDRESS_TO_ID_COMPONENT_IDX: usize = 48;
pub const MEMORY_ID_TO_BIG_BASE_COMPONENT_IDX: usize = 49;
pub const MEMORY_ID_TO_SMALL_COMPONENT_IDX: usize = 65;
pub const RANGE_CHECK_6_COMPONENT_IDX: usize = 66;
pub const RANGE_CHECK_8_COMPONENT_IDX: usize = 67;
pub const RANGE_CHECK_11_COMPONENT_IDX: usize = 68;
pub const RANGE_CHECK_12_COMPONENT_IDX: usize = 69;
pub const RANGE_CHECK_18_COMPONENT_IDX: usize = 70;
pub const RANGE_CHECK_20_COMPONENT_IDX: usize = 71;
pub const RANGE_CHECK_4_3_COMPONENT_IDX: usize = 72;
pub const RANGE_CHECK_4_4_COMPONENT_IDX: usize = 73;
pub const RANGE_CHECK_9_9_COMPONENT_IDX: usize = 74;
pub const RANGE_CHECK_7_2_5_COMPONENT_IDX: usize = 75;
pub const RANGE_CHECK_3_6_6_3_COMPONENT_IDX: usize = 76;
pub const RANGE_CHECK_4_4_4_4_COMPONENT_IDX: usize = 77;
pub const RANGE_CHECK_3_3_3_3_3_COMPONENT_IDX: usize = 78;
pub const VERIFY_BITWISE_XOR_4_COMPONENT_IDX: usize = 79;
pub const VERIFY_BITWISE_XOR_7_COMPONENT_IDX: usize = 80;
pub const VERIFY_BITWISE_XOR_8_COMPONENT_IDX: usize = 81;
pub const VERIFY_BITWISE_XOR_9_COMPONENT_IDX: usize = 82;

pub const N_COMPONENTS: usize = 83;

/// Number of trace columns per component, indexed by COMPONENT_IDX.
pub const N_TRACE_COLUMNS_PER_COMPONENT_IDX: [usize; N_COMPONENTS] = [
    add_opcode::N_TRACE_COLUMNS,
    add_opcode_small::N_TRACE_COLUMNS,
    add_ap_opcode::N_TRACE_COLUMNS,
    assert_eq_opcode::N_TRACE_COLUMNS,
    assert_eq_opcode_imm::N_TRACE_COLUMNS,
    assert_eq_opcode_double_deref::N_TRACE_COLUMNS,
    blake_compress_opcode::N_TRACE_COLUMNS,
    call_opcode_abs::N_TRACE_COLUMNS,
    call_opcode_rel_imm::N_TRACE_COLUMNS,
    generic_opcode::N_TRACE_COLUMNS,
    jnz_opcode_non_taken::N_TRACE_COLUMNS,
    jnz_opcode_taken::N_TRACE_COLUMNS,
    jump_opcode_abs::N_TRACE_COLUMNS,
    jump_opcode_double_deref::N_TRACE_COLUMNS,
    jump_opcode_rel::N_TRACE_COLUMNS,
    jump_opcode_rel_imm::N_TRACE_COLUMNS,
    mul_opcode::N_TRACE_COLUMNS,
    mul_opcode_small::N_TRACE_COLUMNS,
    qm_31_add_mul_opcode::N_TRACE_COLUMNS,
    ret_opcode::N_TRACE_COLUMNS,
    verify_instruction::N_TRACE_COLUMNS,
    blake_round::N_TRACE_COLUMNS,
    blake_g::N_TRACE_COLUMNS,
    blake_round_sigma::N_TRACE_COLUMNS,
    triple_xor_32::N_TRACE_COLUMNS,
    verify_bitwise_xor_12::N_TRACE_COLUMNS,
    add_mod_builtin::N_TRACE_COLUMNS,
    bitwise_builtin::N_TRACE_COLUMNS,
    mul_mod_builtin::N_TRACE_COLUMNS,
    pedersen_builtin::N_TRACE_COLUMNS,
    pedersen_builtin_narrow_windows::N_TRACE_COLUMNS,
    poseidon_builtin::N_TRACE_COLUMNS,
    range_check96_builtin::N_TRACE_COLUMNS,
    range_check_builtin::N_TRACE_COLUMNS,
    ec_op_builtin::N_TRACE_COLUMNS,
    partial_ec_mul_generic::N_TRACE_COLUMNS,
    pedersen_aggregator_window_bits_18::N_TRACE_COLUMNS,
    partial_ec_mul_window_bits_18::N_TRACE_COLUMNS,
    pedersen_points_table_window_bits_18::N_TRACE_COLUMNS,
    pedersen_aggregator_window_bits_9::N_TRACE_COLUMNS,
    partial_ec_mul_window_bits_9::N_TRACE_COLUMNS,
    pedersen_points_table_window_bits_9::N_TRACE_COLUMNS,
    poseidon_aggregator::N_TRACE_COLUMNS,
    poseidon_3_partial_rounds_chain::N_TRACE_COLUMNS,
    poseidon_full_round_chain::N_TRACE_COLUMNS,
    cube_252::N_TRACE_COLUMNS,
    poseidon_round_keys::N_TRACE_COLUMNS,
    range_check_252_width_27::N_TRACE_COLUMNS,
    memory_address_to_id::N_TRACE_COLUMNS,
    // 16 slots for memory_id_to_big (BIG_N_COLUMNS each).
    memory_id_to_big::BIG_N_COLUMNS,
    memory_id_to_big::BIG_N_COLUMNS,
    memory_id_to_big::BIG_N_COLUMNS,
    memory_id_to_big::BIG_N_COLUMNS,
    memory_id_to_big::BIG_N_COLUMNS,
    memory_id_to_big::BIG_N_COLUMNS,
    memory_id_to_big::BIG_N_COLUMNS,
    memory_id_to_big::BIG_N_COLUMNS,
    memory_id_to_big::BIG_N_COLUMNS,
    memory_id_to_big::BIG_N_COLUMNS,
    memory_id_to_big::BIG_N_COLUMNS,
    memory_id_to_big::BIG_N_COLUMNS,
    memory_id_to_big::BIG_N_COLUMNS,
    memory_id_to_big::BIG_N_COLUMNS,
    memory_id_to_big::BIG_N_COLUMNS,
    memory_id_to_big::BIG_N_COLUMNS,
    memory_id_to_small::N_TRACE_COLUMNS,
    range_check_6::N_TRACE_COLUMNS,
    range_check_8::N_TRACE_COLUMNS,
    range_check_11::N_TRACE_COLUMNS,
    range_check_12::N_TRACE_COLUMNS,
    range_check_18::N_TRACE_COLUMNS,
    range_check_20::N_TRACE_COLUMNS,
    range_check_4_3::N_TRACE_COLUMNS,
    range_check_4_4::N_TRACE_COLUMNS,
    range_check_9_9::N_TRACE_COLUMNS,
    range_check_7_2_5::N_TRACE_COLUMNS,
    range_check_3_6_6_3::N_TRACE_COLUMNS,
    range_check_4_4_4_4::N_TRACE_COLUMNS,
    range_check_3_3_3_3_3::N_TRACE_COLUMNS,
    verify_bitwise_xor_4::N_TRACE_COLUMNS,
    verify_bitwise_xor_7::N_TRACE_COLUMNS,
    verify_bitwise_xor_8::N_TRACE_COLUMNS,
    verify_bitwise_xor_9::N_TRACE_COLUMNS,
];

/// Number of interaction columns per component, indexed by COMPONENT_IDX.
pub const N_INTERACTION_COLUMNS_PER_COMPONENT_IDX: [usize; N_COMPONENTS] = [
    add_opcode::N_INTERACTION_COLUMNS,
    add_opcode_small::N_INTERACTION_COLUMNS,
    add_ap_opcode::N_INTERACTION_COLUMNS,
    assert_eq_opcode::N_INTERACTION_COLUMNS,
    assert_eq_opcode_imm::N_INTERACTION_COLUMNS,
    assert_eq_opcode_double_deref::N_INTERACTION_COLUMNS,
    blake_compress_opcode::N_INTERACTION_COLUMNS,
    call_opcode_abs::N_INTERACTION_COLUMNS,
    call_opcode_rel_imm::N_INTERACTION_COLUMNS,
    generic_opcode::N_INTERACTION_COLUMNS,
    jnz_opcode_non_taken::N_INTERACTION_COLUMNS,
    jnz_opcode_taken::N_INTERACTION_COLUMNS,
    jump_opcode_abs::N_INTERACTION_COLUMNS,
    jump_opcode_double_deref::N_INTERACTION_COLUMNS,
    jump_opcode_rel::N_INTERACTION_COLUMNS,
    jump_opcode_rel_imm::N_INTERACTION_COLUMNS,
    mul_opcode::N_INTERACTION_COLUMNS,
    mul_opcode_small::N_INTERACTION_COLUMNS,
    qm_31_add_mul_opcode::N_INTERACTION_COLUMNS,
    ret_opcode::N_INTERACTION_COLUMNS,
    verify_instruction::N_INTERACTION_COLUMNS,
    blake_round::N_INTERACTION_COLUMNS,
    blake_g::N_INTERACTION_COLUMNS,
    blake_round_sigma::N_INTERACTION_COLUMNS,
    triple_xor_32::N_INTERACTION_COLUMNS,
    verify_bitwise_xor_12::N_INTERACTION_COLUMNS,
    add_mod_builtin::N_INTERACTION_COLUMNS,
    bitwise_builtin::N_INTERACTION_COLUMNS,
    mul_mod_builtin::N_INTERACTION_COLUMNS,
    pedersen_builtin::N_INTERACTION_COLUMNS,
    pedersen_builtin_narrow_windows::N_INTERACTION_COLUMNS,
    poseidon_builtin::N_INTERACTION_COLUMNS,
    range_check96_builtin::N_INTERACTION_COLUMNS,
    range_check_builtin::N_INTERACTION_COLUMNS,
    ec_op_builtin::N_INTERACTION_COLUMNS,
    partial_ec_mul_generic::N_INTERACTION_COLUMNS,
    pedersen_aggregator_window_bits_18::N_INTERACTION_COLUMNS,
    partial_ec_mul_window_bits_18::N_INTERACTION_COLUMNS,
    pedersen_points_table_window_bits_18::N_INTERACTION_COLUMNS,
    pedersen_aggregator_window_bits_9::N_INTERACTION_COLUMNS,
    partial_ec_mul_window_bits_9::N_INTERACTION_COLUMNS,
    pedersen_points_table_window_bits_9::N_INTERACTION_COLUMNS,
    poseidon_aggregator::N_INTERACTION_COLUMNS,
    poseidon_3_partial_rounds_chain::N_INTERACTION_COLUMNS,
    poseidon_full_round_chain::N_INTERACTION_COLUMNS,
    cube_252::N_INTERACTION_COLUMNS,
    poseidon_round_keys::N_INTERACTION_COLUMNS,
    range_check_252_width_27::N_INTERACTION_COLUMNS,
    memory_address_to_id::N_INTERACTION_COLUMNS,
    // 16 slots for memory_id_to_big (BIG_N_INTERACTION_COLUMNS each).
    memory_id_to_big::BIG_N_INTERACTION_COLUMNS,
    memory_id_to_big::BIG_N_INTERACTION_COLUMNS,
    memory_id_to_big::BIG_N_INTERACTION_COLUMNS,
    memory_id_to_big::BIG_N_INTERACTION_COLUMNS,
    memory_id_to_big::BIG_N_INTERACTION_COLUMNS,
    memory_id_to_big::BIG_N_INTERACTION_COLUMNS,
    memory_id_to_big::BIG_N_INTERACTION_COLUMNS,
    memory_id_to_big::BIG_N_INTERACTION_COLUMNS,
    memory_id_to_big::BIG_N_INTERACTION_COLUMNS,
    memory_id_to_big::BIG_N_INTERACTION_COLUMNS,
    memory_id_to_big::BIG_N_INTERACTION_COLUMNS,
    memory_id_to_big::BIG_N_INTERACTION_COLUMNS,
    memory_id_to_big::BIG_N_INTERACTION_COLUMNS,
    memory_id_to_big::BIG_N_INTERACTION_COLUMNS,
    memory_id_to_big::BIG_N_INTERACTION_COLUMNS,
    memory_id_to_big::BIG_N_INTERACTION_COLUMNS,
    memory_id_to_small::N_INTERACTION_COLUMNS,
    range_check_6::N_INTERACTION_COLUMNS,
    range_check_8::N_INTERACTION_COLUMNS,
    range_check_11::N_INTERACTION_COLUMNS,
    range_check_12::N_INTERACTION_COLUMNS,
    range_check_18::N_INTERACTION_COLUMNS,
    range_check_20::N_INTERACTION_COLUMNS,
    range_check_4_3::N_INTERACTION_COLUMNS,
    range_check_4_4::N_INTERACTION_COLUMNS,
    range_check_9_9::N_INTERACTION_COLUMNS,
    range_check_7_2_5::N_INTERACTION_COLUMNS,
    range_check_3_6_6_3::N_INTERACTION_COLUMNS,
    range_check_4_4_4_4::N_INTERACTION_COLUMNS,
    range_check_3_3_3_3_3::N_INTERACTION_COLUMNS,
    verify_bitwise_xor_4::N_INTERACTION_COLUMNS,
    verify_bitwise_xor_7::N_INTERACTION_COLUMNS,
    verify_bitwise_xor_8::N_INTERACTION_COLUMNS,
    verify_bitwise_xor_9::N_INTERACTION_COLUMNS,
];
