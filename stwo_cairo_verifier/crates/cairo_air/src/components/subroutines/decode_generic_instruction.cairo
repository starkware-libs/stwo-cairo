// This file was created by the AIR team.

use crate::components::subroutines::decode_instruction_df7a6::decode_instruction_df7a6_evaluate;
use crate::prelude::*;


pub fn decode_generic_instruction_evaluate(
    input: QM31,
    offset0_col0: QM31,
    offset1_col1: QM31,
    offset2_col2: QM31,
    dst_base_fp_col3: QM31,
    op0_base_fp_col4: QM31,
    op1_imm_col5: QM31,
    op1_base_fp_col6: QM31,
    op1_base_ap_col7: QM31,
    res_add_col8: QM31,
    res_mul_col9: QM31,
    pc_update_jump_col10: QM31,
    pc_update_jump_rel_col11: QM31,
    pc_update_jnz_col12: QM31,
    ap_update_add_col13: QM31,
    ap_update_add_1_col14: QM31,
    opcode_call_col15: QM31,
    opcode_ret_col16: QM31,
    opcode_assert_eq_col17: QM31,
    verify_instruction_lookup_elements: @crate::VerifyInstructionElements,
    ref verify_instruction_sum_0: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 8] {
    let decode_generic_instruction_input = input;
    let [
        decode_instruction_df7a6_output_tmp_62f3c_20_offset0,
        decode_instruction_df7a6_output_tmp_62f3c_20_offset1,
        decode_instruction_df7a6_output_tmp_62f3c_20_offset2,
    ] =
        decode_instruction_df7a6_evaluate(
        decode_generic_instruction_input,
        offset0_col0,
        offset1_col1,
        offset2_col2,
        dst_base_fp_col3,
        op0_base_fp_col4,
        op1_imm_col5,
        op1_base_fp_col6,
        op1_base_ap_col7,
        res_add_col8,
        res_mul_col9,
        pc_update_jump_col10,
        pc_update_jump_rel_col11,
        pc_update_jnz_col12,
        ap_update_add_col13,
        ap_update_add_1_col14,
        opcode_call_col15,
        opcode_ret_col16,
        opcode_assert_eq_col17,
        verify_instruction_lookup_elements,
        ref verify_instruction_sum_0,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );
    let op1_base_op0_tmp_62f3c_21: QM31 = (((qm31_const::<1, 0, 0, 0>() - op1_imm_col5)
        - op1_base_fp_col6)
        - op1_base_ap_col7);

    // Constraint - op1_src is 0, 1, 2, or 4
    let constraint_quotient = ((op1_base_op0_tmp_62f3c_21
        * (qm31_const::<1, 0, 0, 0>() - op1_base_op0_tmp_62f3c_21)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let res_op1_tmp_62f3c_22: QM31 = (((qm31_const::<1, 0, 0, 0>() - res_add_col8) - res_mul_col9)
        - pc_update_jnz_col12);

    // Constraint - res_logic is 0, 1, or 2
    let constraint_quotient = ((res_op1_tmp_62f3c_22
        * (qm31_const::<1, 0, 0, 0>() - res_op1_tmp_62f3c_22)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let pc_update_regular_tmp_62f3c_23: QM31 = (((qm31_const::<1, 0, 0, 0>() - pc_update_jump_col10)
        - pc_update_jump_rel_col11)
        - pc_update_jnz_col12);

    // Constraint - pc_update is 0, 1, 2, or 4
    let constraint_quotient = ((pc_update_regular_tmp_62f3c_23
        * (qm31_const::<1, 0, 0, 0>() - pc_update_regular_tmp_62f3c_23)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let ap_update_regular_tmp_62f3c_24: QM31 = (((qm31_const::<1, 0, 0, 0>() - ap_update_add_col13)
        - ap_update_add_1_col14)
        - opcode_call_col15);

    // Constraint - ap_update is 0, 1, 2, or 4
    let constraint_quotient = ((ap_update_regular_tmp_62f3c_24
        * (qm31_const::<1, 0, 0, 0>() - ap_update_regular_tmp_62f3c_24)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let fp_update_regular_tmp_62f3c_25: QM31 = ((qm31_const::<1, 0, 0, 0>() - opcode_call_col15)
        - opcode_ret_col16);

    // Constraint - opcode is 0, 1, 2, or 4
    let constraint_quotient = ((fp_update_regular_tmp_62f3c_25
        * (qm31_const::<1, 0, 0, 0>() - fp_update_regular_tmp_62f3c_25)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    [
        op1_base_op0_tmp_62f3c_21, res_op1_tmp_62f3c_22, pc_update_regular_tmp_62f3c_23,
        fp_update_regular_tmp_62f3c_25, (qm31_const::<1, 0, 0, 0>() + op1_imm_col5),
        decode_instruction_df7a6_output_tmp_62f3c_20_offset0,
        decode_instruction_df7a6_output_tmp_62f3c_20_offset1,
        decode_instruction_df7a6_output_tmp_62f3c_20_offset2,
    ]
}
