// This file was created by the AIR team.

use crate::prelude::*;


pub fn decode_instruction_d2a10_evaluate(
    input: QM31,
    offset2_col0: QM31,
    op1_imm_col1: QM31,
    op1_base_fp_col2: QM31,
    verify_instruction_lookup_elements: @crate::VerifyInstructionElements,
    ref verify_instruction_sum_0: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 2] {
    let decode_instruction_d2a10_input_pc = input;

    // Constraint - Flag op1_imm is a bit
    let constraint_quotient = ((op1_imm_col1 * (qm31_const::<1, 0, 0, 0>() - op1_imm_col1)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - Flag op1_base_fp is a bit
    let constraint_quotient = ((op1_base_fp_col2 * (qm31_const::<1, 0, 0, 0>() - op1_base_fp_col2)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let op1_base_ap_tmp_d2a10_5: QM31 = ((qm31_const::<1, 0, 0, 0>() - op1_imm_col1)
        - op1_base_fp_col2);

    // Constraint - Flag op1_base_ap is a bit
    let constraint_quotient = ((op1_base_ap_tmp_d2a10_5
        * (qm31_const::<1, 0, 0, 0>() - op1_base_ap_tmp_d2a10_5)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    verify_instruction_sum_0 = verify_instruction_lookup_elements
        .combine_qm31(
            [
                decode_instruction_d2a10_input_pc, qm31_const::<32767, 0, 0, 0>(),
                qm31_const::<32767, 0, 0, 0>(), offset2_col0,
                (((qm31_const::<24, 0, 0, 0>() + (op1_imm_col1 * qm31_const::<32, 0, 0, 0>()))
                    + (op1_base_fp_col2 * qm31_const::<64, 0, 0, 0>()))
                    + (op1_base_ap_tmp_d2a10_5 * qm31_const::<128, 0, 0, 0>())),
                qm31_const::<16, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(),
            ],
        );

    [(offset2_col0 - qm31_const::<32768, 0, 0, 0>()), op1_base_ap_tmp_d2a10_5]
}
