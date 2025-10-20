// This file was created by the AIR team.

use crate::prelude::*;


pub fn decode_instruction_df7a6_evaluate(
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
) -> [QM31; 3] {
    let decode_instruction_df7a6_input_pc = input;

    // Constraint - Flag dst_base_fp is a bit
    let constraint_quotient = ((dst_base_fp_col3 * (qm31_const::<1, 0, 0, 0>() - dst_base_fp_col3)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - Flag op0_base_fp is a bit
    let constraint_quotient = ((op0_base_fp_col4 * (qm31_const::<1, 0, 0, 0>() - op0_base_fp_col4)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - Flag op1_imm is a bit
    let constraint_quotient = ((op1_imm_col5 * (qm31_const::<1, 0, 0, 0>() - op1_imm_col5)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - Flag op1_base_fp is a bit
    let constraint_quotient = ((op1_base_fp_col6 * (qm31_const::<1, 0, 0, 0>() - op1_base_fp_col6)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - Flag op1_base_ap is a bit
    let constraint_quotient = ((op1_base_ap_col7 * (qm31_const::<1, 0, 0, 0>() - op1_base_ap_col7)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - Flag res_add is a bit
    let constraint_quotient = ((res_add_col8 * (qm31_const::<1, 0, 0, 0>() - res_add_col8)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - Flag res_mul is a bit
    let constraint_quotient = ((res_mul_col9 * (qm31_const::<1, 0, 0, 0>() - res_mul_col9)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - Flag pc_update_jump is a bit
    let constraint_quotient = ((pc_update_jump_col10
        * (qm31_const::<1, 0, 0, 0>() - pc_update_jump_col10)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - Flag pc_update_jump_rel is a bit
    let constraint_quotient = ((pc_update_jump_rel_col11
        * (qm31_const::<1, 0, 0, 0>() - pc_update_jump_rel_col11)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - Flag pc_update_jnz is a bit
    let constraint_quotient = ((pc_update_jnz_col12
        * (qm31_const::<1, 0, 0, 0>() - pc_update_jnz_col12)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - Flag ap_update_add is a bit
    let constraint_quotient = ((ap_update_add_col13
        * (qm31_const::<1, 0, 0, 0>() - ap_update_add_col13)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - Flag ap_update_add_1 is a bit
    let constraint_quotient = ((ap_update_add_1_col14
        * (qm31_const::<1, 0, 0, 0>() - ap_update_add_1_col14)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - Flag opcode_call is a bit
    let constraint_quotient = ((opcode_call_col15
        * (qm31_const::<1, 0, 0, 0>() - opcode_call_col15)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - Flag opcode_ret is a bit
    let constraint_quotient = ((opcode_ret_col16 * (qm31_const::<1, 0, 0, 0>() - opcode_ret_col16)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - Flag opcode_assert_eq is a bit
    let constraint_quotient = ((opcode_assert_eq_col17
        * (qm31_const::<1, 0, 0, 0>() - opcode_assert_eq_col17)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    verify_instruction_sum_0 = verify_instruction_lookup_elements
        .combine_qm31(
            [
                decode_instruction_df7a6_input_pc, offset0_col0, offset1_col1, offset2_col2,
                ((((((dst_base_fp_col3 * qm31_const::<8, 0, 0, 0>())
                    + (op0_base_fp_col4 * qm31_const::<16, 0, 0, 0>()))
                    + (op1_imm_col5 * qm31_const::<32, 0, 0, 0>()))
                    + (op1_base_fp_col6 * qm31_const::<64, 0, 0, 0>()))
                    + (op1_base_ap_col7 * qm31_const::<128, 0, 0, 0>()))
                    + (res_add_col8 * qm31_const::<256, 0, 0, 0>())),
                ((((((((res_mul_col9 + (pc_update_jump_col10 * qm31_const::<2, 0, 0, 0>()))
                    + (pc_update_jump_rel_col11 * qm31_const::<4, 0, 0, 0>()))
                    + (pc_update_jnz_col12 * qm31_const::<8, 0, 0, 0>()))
                    + (ap_update_add_col13 * qm31_const::<16, 0, 0, 0>()))
                    + (ap_update_add_1_col14 * qm31_const::<32, 0, 0, 0>()))
                    + (opcode_call_col15 * qm31_const::<64, 0, 0, 0>()))
                    + (opcode_ret_col16 * qm31_const::<128, 0, 0, 0>()))
                    + (opcode_assert_eq_col17 * qm31_const::<256, 0, 0, 0>())),
                qm31_const::<0, 0, 0, 0>(),
            ],
        );

    [
        (offset0_col0 - qm31_const::<32768, 0, 0, 0>()),
        (offset1_col1 - qm31_const::<32768, 0, 0, 0>()),
        (offset2_col2 - qm31_const::<32768, 0, 0, 0>()),
    ]
}
