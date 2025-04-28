// Constraints version: 252b9d8a

use core::num::traits::Zero;
use stwo_constraint_framework::{
    PreprocessedColumn, PreprocessedColumnSet, PreprocessedColumnSetImpl, PreprocessedMaskValues,
    PreprocessedMaskValuesImpl,
};
use stwo_verifier_core::channel::{Channel, ChannelTrait};
use stwo_verifier_core::circle::{
    CirclePoint, CirclePointIndexTrait, CirclePointQM31AddCirclePointM31Trait,
};
use stwo_verifier_core::fields::Invertible;
use stwo_verifier_core::fields::m31::{M31, m31};
use stwo_verifier_core::fields::qm31::{
    QM31, QM31Impl, QM31Serde, QM31Zero, QM31_EXTENSION_DEGREE, qm31_const,
};
use stwo_verifier_core::poly::circle::CanonicCosetImpl;
use stwo_verifier_core::utils::{ArrayImpl, pow2};
use stwo_verifier_core::{ColumnArray, ColumnSpan, TreeArray};
use crate::components::memory_address_to_id::{
    MEMORY_ADDRESS_TO_ID_RELATION_SIZE, memory_address_to_id_sum,
};
use crate::components::memory_id_to_big::{MEMORY_ID_TO_BIG_RELATION_SIZE, memory_id_to_big_sum};
use crate::components::range_check_7_2_5::{RANGE_CHECK_7_2_5_RELATION_SIZE, range_check_7_2_5_sum};
use crate::components::subroutines::decode_instruction_64420::decode_instruction_64420_evaluate;
use crate::components::subroutines::read_blake_word::read_blake_word_evaluate;
use crate::components::subroutines::read_positive_num_bits_27::read_positive_num_bits_27_evaluate;
use crate::components::verify_instruction::{
    VERIFY_INSTRUCTION_RELATION_SIZE, verify_instruction_sum,
};
use crate::components::{CairoComponent, OPCODES_RELATION_SIZE, opcodes_sum};
use crate::utils::U32Impl;


pub const N_TRACE_COLUMNS: usize = 30;


pub fn decode_blake_opcode_evaluate(
    input: [QM31; 3],
    offset0_col0: QM31,
    offset1_col1: QM31,
    offset2_col2: QM31,
    dst_base_fp_col3: QM31,
    op0_base_fp_col4: QM31,
    op1_base_fp_col5: QM31,
    op1_base_ap_col6: QM31,
    ap_update_add_1_col7: QM31,
    opcode_extension_col8: QM31,
    mem0_base_col9: QM31,
    op0_id_col10: QM31,
    op0_limb_0_col11: QM31,
    op0_limb_1_col12: QM31,
    op0_limb_2_col13: QM31,
    mem1_base_col14: QM31,
    op1_id_col15: QM31,
    op1_limb_0_col16: QM31,
    op1_limb_1_col17: QM31,
    op1_limb_2_col18: QM31,
    ap_id_col19: QM31,
    ap_limb_0_col20: QM31,
    ap_limb_1_col21: QM31,
    ap_limb_2_col22: QM31,
    mem_dst_base_col23: QM31,
    low_16_bits_col24: QM31,
    high_16_bits_col25: QM31,
    low_7_ms_bits_col26: QM31,
    high_14_ms_bits_col27: QM31,
    high_5_ms_bits_col28: QM31,
    dst_id_col29: QM31,
    verify_instruction_alphas: Span<QM31>,
    verify_instruction_z: QM31,
    memory_address_to_id_alphas: Span<QM31>,
    memory_address_to_id_z: QM31,
    memory_id_to_big_alphas: Span<QM31>,
    memory_id_to_big_z: QM31,
    range_check_7_2_5_alphas: Span<QM31>,
    range_check_7_2_5_z: QM31,
    ref verify_instruction_sum_0: QM31,
    ref memory_address_to_id_sum_1: QM31,
    ref memory_id_to_big_sum_2: QM31,
    ref memory_address_to_id_sum_3: QM31,
    ref memory_id_to_big_sum_4: QM31,
    ref memory_address_to_id_sum_5: QM31,
    ref memory_id_to_big_sum_6: QM31,
    ref range_check_7_2_5_sum_7: QM31,
    ref memory_address_to_id_sum_8: QM31,
    ref memory_id_to_big_sum_9: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 7] {
    let [decode_blake_opcode_input_pc, decode_blake_opcode_input_ap, decode_blake_opcode_input_fp] =
        input;

    let output: [QM31; 19] = decode_instruction_64420_evaluate(
        decode_blake_opcode_input_pc,
        offset0_col0,
        offset1_col1,
        offset2_col2,
        dst_base_fp_col3,
        op0_base_fp_col4,
        op1_base_fp_col5,
        op1_base_ap_col6,
        ap_update_add_1_col7,
        opcode_extension_col8,
        verify_instruction_alphas,
        verify_instruction_z,
        ref verify_instruction_sum_0,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );
    let [
        decode_instruction_64420_output_tmp_47e62_10_offset0,
        decode_instruction_64420_output_tmp_47e62_10_offset1,
        decode_instruction_64420_output_tmp_47e62_10_offset2,
        decode_instruction_64420_output_tmp_47e62_10_dst_base_fp,
        decode_instruction_64420_output_tmp_47e62_10_op0_base_fp,
        decode_instruction_64420_output_tmp_47e62_10_op1_imm,
        decode_instruction_64420_output_tmp_47e62_10_op1_base_fp,
        decode_instruction_64420_output_tmp_47e62_10_op1_base_ap,
        decode_instruction_64420_output_tmp_47e62_10_res_add,
        decode_instruction_64420_output_tmp_47e62_10_res_mul,
        decode_instruction_64420_output_tmp_47e62_10_pc_update_jump,
        decode_instruction_64420_output_tmp_47e62_10_pc_update_jump_rel,
        decode_instruction_64420_output_tmp_47e62_10_pc_update_jnz,
        decode_instruction_64420_output_tmp_47e62_10_ap_update_add,
        decode_instruction_64420_output_tmp_47e62_10_ap_update_add_1,
        decode_instruction_64420_output_tmp_47e62_10_opcode_call,
        decode_instruction_64420_output_tmp_47e62_10_opcode_ret,
        decode_instruction_64420_output_tmp_47e62_10_opcode_assert_eq,
        decode_instruction_64420_output_tmp_47e62_10_opcode_extension,
    ] =
        output;

    // Constraint - Exactly one of op1_base_fp and op1_base_ap is 1
    let constraint_quotient = (((op1_base_fp_col5 + op1_base_ap_col6) - qm31_const::<1, 0, 0, 0>()))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - OpcodeExtension is either Blake or BlakeFinalize
    let constraint_quotient = (((opcode_extension_col8 - qm31_const::<1, 0, 0, 0>())
        * (opcode_extension_col8 - qm31_const::<2, 0, 0, 0>())))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - mem0_base
    let constraint_quotient = ((mem0_base_col9
        - ((op0_base_fp_col4 * decode_blake_opcode_input_fp)
            + ((qm31_const::<1, 0, 0, 0>() - op0_base_fp_col4) * decode_blake_opcode_input_ap))))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let output: [QM31; 29] = read_positive_num_bits_27_evaluate(
        (mem0_base_col9 + decode_instruction_64420_output_tmp_47e62_10_offset1),
        op0_id_col10,
        op0_limb_0_col11,
        op0_limb_1_col12,
        op0_limb_2_col13,
        memory_address_to_id_alphas,
        memory_address_to_id_z,
        memory_id_to_big_alphas,
        memory_id_to_big_z,
        ref memory_address_to_id_sum_1,
        ref memory_id_to_big_sum_2,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );
    let [
        read_positive_num_bits_27_output_tmp_47e62_13_limb_0,
        read_positive_num_bits_27_output_tmp_47e62_13_limb_1,
        read_positive_num_bits_27_output_tmp_47e62_13_limb_2,
        read_positive_num_bits_27_output_tmp_47e62_13_limb_3,
        read_positive_num_bits_27_output_tmp_47e62_13_limb_4,
        read_positive_num_bits_27_output_tmp_47e62_13_limb_5,
        read_positive_num_bits_27_output_tmp_47e62_13_limb_6,
        read_positive_num_bits_27_output_tmp_47e62_13_limb_7,
        read_positive_num_bits_27_output_tmp_47e62_13_limb_8,
        read_positive_num_bits_27_output_tmp_47e62_13_limb_9,
        read_positive_num_bits_27_output_tmp_47e62_13_limb_10,
        read_positive_num_bits_27_output_tmp_47e62_13_limb_11,
        read_positive_num_bits_27_output_tmp_47e62_13_limb_12,
        read_positive_num_bits_27_output_tmp_47e62_13_limb_13,
        read_positive_num_bits_27_output_tmp_47e62_13_limb_14,
        read_positive_num_bits_27_output_tmp_47e62_13_limb_15,
        read_positive_num_bits_27_output_tmp_47e62_13_limb_16,
        read_positive_num_bits_27_output_tmp_47e62_13_limb_17,
        read_positive_num_bits_27_output_tmp_47e62_13_limb_18,
        read_positive_num_bits_27_output_tmp_47e62_13_limb_19,
        read_positive_num_bits_27_output_tmp_47e62_13_limb_20,
        read_positive_num_bits_27_output_tmp_47e62_13_limb_21,
        read_positive_num_bits_27_output_tmp_47e62_13_limb_22,
        read_positive_num_bits_27_output_tmp_47e62_13_limb_23,
        read_positive_num_bits_27_output_tmp_47e62_13_limb_24,
        read_positive_num_bits_27_output_tmp_47e62_13_limb_25,
        read_positive_num_bits_27_output_tmp_47e62_13_limb_26,
        read_positive_num_bits_27_output_tmp_47e62_13_limb_27,
        read_positive_num_bits_27_output_tmp_47e62_13_limb_28,
    ] =
        output;

    // Constraint - mem1_base
    let constraint_quotient = ((mem1_base_col14
        - ((op1_base_fp_col5 * decode_blake_opcode_input_fp)
            + (op1_base_ap_col6 * decode_blake_opcode_input_ap))))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let output: [QM31; 29] = read_positive_num_bits_27_evaluate(
        (mem1_base_col14 + decode_instruction_64420_output_tmp_47e62_10_offset2),
        op1_id_col15,
        op1_limb_0_col16,
        op1_limb_1_col17,
        op1_limb_2_col18,
        memory_address_to_id_alphas,
        memory_address_to_id_z,
        memory_id_to_big_alphas,
        memory_id_to_big_z,
        ref memory_address_to_id_sum_3,
        ref memory_id_to_big_sum_4,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );
    let [
        read_positive_num_bits_27_output_tmp_47e62_16_limb_0,
        read_positive_num_bits_27_output_tmp_47e62_16_limb_1,
        read_positive_num_bits_27_output_tmp_47e62_16_limb_2,
        read_positive_num_bits_27_output_tmp_47e62_16_limb_3,
        read_positive_num_bits_27_output_tmp_47e62_16_limb_4,
        read_positive_num_bits_27_output_tmp_47e62_16_limb_5,
        read_positive_num_bits_27_output_tmp_47e62_16_limb_6,
        read_positive_num_bits_27_output_tmp_47e62_16_limb_7,
        read_positive_num_bits_27_output_tmp_47e62_16_limb_8,
        read_positive_num_bits_27_output_tmp_47e62_16_limb_9,
        read_positive_num_bits_27_output_tmp_47e62_16_limb_10,
        read_positive_num_bits_27_output_tmp_47e62_16_limb_11,
        read_positive_num_bits_27_output_tmp_47e62_16_limb_12,
        read_positive_num_bits_27_output_tmp_47e62_16_limb_13,
        read_positive_num_bits_27_output_tmp_47e62_16_limb_14,
        read_positive_num_bits_27_output_tmp_47e62_16_limb_15,
        read_positive_num_bits_27_output_tmp_47e62_16_limb_16,
        read_positive_num_bits_27_output_tmp_47e62_16_limb_17,
        read_positive_num_bits_27_output_tmp_47e62_16_limb_18,
        read_positive_num_bits_27_output_tmp_47e62_16_limb_19,
        read_positive_num_bits_27_output_tmp_47e62_16_limb_20,
        read_positive_num_bits_27_output_tmp_47e62_16_limb_21,
        read_positive_num_bits_27_output_tmp_47e62_16_limb_22,
        read_positive_num_bits_27_output_tmp_47e62_16_limb_23,
        read_positive_num_bits_27_output_tmp_47e62_16_limb_24,
        read_positive_num_bits_27_output_tmp_47e62_16_limb_25,
        read_positive_num_bits_27_output_tmp_47e62_16_limb_26,
        read_positive_num_bits_27_output_tmp_47e62_16_limb_27,
        read_positive_num_bits_27_output_tmp_47e62_16_limb_28,
    ] =
        output;

    let output: [QM31; 29] = read_positive_num_bits_27_evaluate(
        decode_blake_opcode_input_ap,
        ap_id_col19,
        ap_limb_0_col20,
        ap_limb_1_col21,
        ap_limb_2_col22,
        memory_address_to_id_alphas,
        memory_address_to_id_z,
        memory_id_to_big_alphas,
        memory_id_to_big_z,
        ref memory_address_to_id_sum_5,
        ref memory_id_to_big_sum_6,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );
    let [
        read_positive_num_bits_27_output_tmp_47e62_19_limb_0,
        read_positive_num_bits_27_output_tmp_47e62_19_limb_1,
        read_positive_num_bits_27_output_tmp_47e62_19_limb_2,
        read_positive_num_bits_27_output_tmp_47e62_19_limb_3,
        read_positive_num_bits_27_output_tmp_47e62_19_limb_4,
        read_positive_num_bits_27_output_tmp_47e62_19_limb_5,
        read_positive_num_bits_27_output_tmp_47e62_19_limb_6,
        read_positive_num_bits_27_output_tmp_47e62_19_limb_7,
        read_positive_num_bits_27_output_tmp_47e62_19_limb_8,
        read_positive_num_bits_27_output_tmp_47e62_19_limb_9,
        read_positive_num_bits_27_output_tmp_47e62_19_limb_10,
        read_positive_num_bits_27_output_tmp_47e62_19_limb_11,
        read_positive_num_bits_27_output_tmp_47e62_19_limb_12,
        read_positive_num_bits_27_output_tmp_47e62_19_limb_13,
        read_positive_num_bits_27_output_tmp_47e62_19_limb_14,
        read_positive_num_bits_27_output_tmp_47e62_19_limb_15,
        read_positive_num_bits_27_output_tmp_47e62_19_limb_16,
        read_positive_num_bits_27_output_tmp_47e62_19_limb_17,
        read_positive_num_bits_27_output_tmp_47e62_19_limb_18,
        read_positive_num_bits_27_output_tmp_47e62_19_limb_19,
        read_positive_num_bits_27_output_tmp_47e62_19_limb_20,
        read_positive_num_bits_27_output_tmp_47e62_19_limb_21,
        read_positive_num_bits_27_output_tmp_47e62_19_limb_22,
        read_positive_num_bits_27_output_tmp_47e62_19_limb_23,
        read_positive_num_bits_27_output_tmp_47e62_19_limb_24,
        read_positive_num_bits_27_output_tmp_47e62_19_limb_25,
        read_positive_num_bits_27_output_tmp_47e62_19_limb_26,
        read_positive_num_bits_27_output_tmp_47e62_19_limb_27,
        read_positive_num_bits_27_output_tmp_47e62_19_limb_28,
    ] =
        output;

    // Constraint - mem_dst_base
    let constraint_quotient = ((mem_dst_base_col23
        - ((dst_base_fp_col3 * decode_blake_opcode_input_fp)
            + ((qm31_const::<1, 0, 0, 0>() - dst_base_fp_col3) * decode_blake_opcode_input_ap))))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let output: [QM31; 2] = read_blake_word_evaluate(
        (mem_dst_base_col23 + decode_instruction_64420_output_tmp_47e62_10_offset0),
        low_16_bits_col24,
        high_16_bits_col25,
        low_7_ms_bits_col26,
        high_14_ms_bits_col27,
        high_5_ms_bits_col28,
        dst_id_col29,
        range_check_7_2_5_alphas,
        range_check_7_2_5_z,
        memory_address_to_id_alphas,
        memory_address_to_id_z,
        memory_id_to_big_alphas,
        memory_id_to_big_z,
        ref range_check_7_2_5_sum_7,
        ref memory_address_to_id_sum_8,
        ref memory_id_to_big_sum_9,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );
    let [read_blake_word_output_tmp_47e62_28_limb_0, read_blake_word_output_tmp_47e62_28_limb_1] =
        output;

    [
        ((op0_limb_0_col11 + (op0_limb_1_col12 * qm31_const::<512, 0, 0, 0>()))
            + (op0_limb_2_col13 * qm31_const::<262144, 0, 0, 0>())),
        ((op1_limb_0_col16 + (op1_limb_1_col17 * qm31_const::<512, 0, 0, 0>()))
            + (op1_limb_2_col18 * qm31_const::<262144, 0, 0, 0>())),
        ((ap_limb_0_col20 + (ap_limb_1_col21 * qm31_const::<512, 0, 0, 0>()))
            + (ap_limb_2_col22 * qm31_const::<262144, 0, 0, 0>())),
        low_16_bits_col24, high_16_bits_col25, ap_update_add_1_col7,
        (opcode_extension_col8 - qm31_const::<1, 0, 0, 0>()),
    ]
}
