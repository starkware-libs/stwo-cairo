// This file was created by the AIR team.

use crate::components::subroutines::cond_felt_252_as_addr::cond_felt_252_as_addr_evaluate;
use crate::components::subroutines::cond_felt_252_as_rel_imm::cond_felt_252_as_rel_imm_evaluate;
use crate::components::subroutines::range_check_29::range_check_29_evaluate;
use crate::prelude::*;


pub fn update_registers_evaluate(
    input: [QM31; 97],
    partial_limb_msb_col0: QM31,
    msb_col1: QM31,
    mid_limbs_set_col2: QM31,
    partial_limb_msb_col3: QM31,
    dst_sum_squares_inv_col4: QM31,
    dst_sum_inv_col5: QM31,
    op1_as_rel_imm_cond_col6: QM31,
    msb_col7: QM31,
    mid_limbs_set_col8: QM31,
    partial_limb_msb_col9: QM31,
    next_pc_jnz_col10: QM31,
    next_pc_col11: QM31,
    next_ap_col12: QM31,
    range_check_29_bot11bits_col13: QM31,
    next_fp_col14: QM31,
    common_lookup_elements: @CommonLookupElements,
    ref range_check_18_sum_0: QM31,
    ref numerator_0: QM31,
    ref range_check_11_sum_1: QM31,
    ref numerator_1: QM31,
    ref sum: QM31,
    random_coeff: QM31,
) -> [QM31; 0] {
    let [
        update_registers_input_pc,
        update_registers_input_ap,
        update_registers_input_fp,
        update_registers_input_pc_update_jump,
        update_registers_input_pc_update_jump_rel,
        update_registers_input_pc_update_jnz,
        update_registers_input_ap_update_add,
        update_registers_input_ap_update_add_1,
        update_registers_input_opcode_call,
        update_registers_input_opcode_ret,
        update_registers_input_pc_update_regular,
        update_registers_input_fp_update_regular,
        update_registers_input_instruction_size,
        update_registers_input_dst_limb_0,
        update_registers_input_dst_limb_1,
        update_registers_input_dst_limb_2,
        update_registers_input_dst_limb_3,
        update_registers_input_dst_limb_4,
        update_registers_input_dst_limb_5,
        update_registers_input_dst_limb_6,
        update_registers_input_dst_limb_7,
        update_registers_input_dst_limb_8,
        update_registers_input_dst_limb_9,
        update_registers_input_dst_limb_10,
        update_registers_input_dst_limb_11,
        update_registers_input_dst_limb_12,
        update_registers_input_dst_limb_13,
        update_registers_input_dst_limb_14,
        update_registers_input_dst_limb_15,
        update_registers_input_dst_limb_16,
        update_registers_input_dst_limb_17,
        update_registers_input_dst_limb_18,
        update_registers_input_dst_limb_19,
        update_registers_input_dst_limb_20,
        update_registers_input_dst_limb_21,
        update_registers_input_dst_limb_22,
        update_registers_input_dst_limb_23,
        update_registers_input_dst_limb_24,
        update_registers_input_dst_limb_25,
        update_registers_input_dst_limb_26,
        update_registers_input_dst_limb_27,
        update_registers_input_op1_limb_0,
        update_registers_input_op1_limb_1,
        update_registers_input_op1_limb_2,
        update_registers_input_op1_limb_3,
        update_registers_input_op1_limb_4,
        update_registers_input_op1_limb_5,
        update_registers_input_op1_limb_6,
        update_registers_input_op1_limb_7,
        update_registers_input_op1_limb_8,
        update_registers_input_op1_limb_9,
        update_registers_input_op1_limb_10,
        update_registers_input_op1_limb_11,
        update_registers_input_op1_limb_12,
        update_registers_input_op1_limb_13,
        update_registers_input_op1_limb_14,
        update_registers_input_op1_limb_15,
        update_registers_input_op1_limb_16,
        update_registers_input_op1_limb_17,
        update_registers_input_op1_limb_18,
        update_registers_input_op1_limb_19,
        update_registers_input_op1_limb_20,
        update_registers_input_op1_limb_21,
        update_registers_input_op1_limb_22,
        update_registers_input_op1_limb_23,
        update_registers_input_op1_limb_24,
        update_registers_input_op1_limb_25,
        update_registers_input_op1_limb_26,
        update_registers_input_op1_limb_27,
        update_registers_input_res_limb_0,
        update_registers_input_res_limb_1,
        update_registers_input_res_limb_2,
        update_registers_input_res_limb_3,
        update_registers_input_res_limb_4,
        update_registers_input_res_limb_5,
        update_registers_input_res_limb_6,
        update_registers_input_res_limb_7,
        update_registers_input_res_limb_8,
        update_registers_input_res_limb_9,
        update_registers_input_res_limb_10,
        update_registers_input_res_limb_11,
        update_registers_input_res_limb_12,
        update_registers_input_res_limb_13,
        update_registers_input_res_limb_14,
        update_registers_input_res_limb_15,
        update_registers_input_res_limb_16,
        update_registers_input_res_limb_17,
        update_registers_input_res_limb_18,
        update_registers_input_res_limb_19,
        update_registers_input_res_limb_20,
        update_registers_input_res_limb_21,
        update_registers_input_res_limb_22,
        update_registers_input_res_limb_23,
        update_registers_input_res_limb_24,
        update_registers_input_res_limb_25,
        update_registers_input_res_limb_26,
        update_registers_input_res_limb_27,
    ] =
        input;
    let cond_felt_252_as_addr_output_tmp_783d5_2: QM31 = cond_felt_252_as_addr_evaluate(
        [
            update_registers_input_dst_limb_0, update_registers_input_dst_limb_1,
            update_registers_input_dst_limb_2, update_registers_input_dst_limb_3,
            update_registers_input_dst_limb_4, update_registers_input_dst_limb_5,
            update_registers_input_dst_limb_6, update_registers_input_dst_limb_7,
            update_registers_input_dst_limb_8, update_registers_input_dst_limb_9,
            update_registers_input_dst_limb_10, update_registers_input_dst_limb_11,
            update_registers_input_dst_limb_12, update_registers_input_dst_limb_13,
            update_registers_input_dst_limb_14, update_registers_input_dst_limb_15,
            update_registers_input_dst_limb_16, update_registers_input_dst_limb_17,
            update_registers_input_dst_limb_18, update_registers_input_dst_limb_19,
            update_registers_input_dst_limb_20, update_registers_input_dst_limb_21,
            update_registers_input_dst_limb_22, update_registers_input_dst_limb_23,
            update_registers_input_dst_limb_24, update_registers_input_dst_limb_25,
            update_registers_input_dst_limb_26, update_registers_input_dst_limb_27,
            update_registers_input_opcode_ret,
        ],
        partial_limb_msb_col0,
        common_lookup_elements,
        ref sum,
        random_coeff,
    );
    let cond_felt_252_as_rel_imm_output_tmp_783d5_9: QM31 = cond_felt_252_as_rel_imm_evaluate(
        [
            update_registers_input_res_limb_0, update_registers_input_res_limb_1,
            update_registers_input_res_limb_2, update_registers_input_res_limb_3,
            update_registers_input_res_limb_4, update_registers_input_res_limb_5,
            update_registers_input_res_limb_6, update_registers_input_res_limb_7,
            update_registers_input_res_limb_8, update_registers_input_res_limb_9,
            update_registers_input_res_limb_10, update_registers_input_res_limb_11,
            update_registers_input_res_limb_12, update_registers_input_res_limb_13,
            update_registers_input_res_limb_14, update_registers_input_res_limb_15,
            update_registers_input_res_limb_16, update_registers_input_res_limb_17,
            update_registers_input_res_limb_18, update_registers_input_res_limb_19,
            update_registers_input_res_limb_20, update_registers_input_res_limb_21,
            update_registers_input_res_limb_22, update_registers_input_res_limb_23,
            update_registers_input_res_limb_24, update_registers_input_res_limb_25,
            update_registers_input_res_limb_26, update_registers_input_res_limb_27,
            (update_registers_input_pc_update_jump_rel + update_registers_input_ap_update_add),
        ],
        msb_col1,
        mid_limbs_set_col2,
        partial_limb_msb_col3,
        common_lookup_elements,
        ref sum,
        random_coeff,
    );
    let diff_from_p_tmp_783d5_10: QM31 = (update_registers_input_dst_limb_0
        - qm31_const::<1, 0, 0, 0>());
    let diff_from_p_tmp_783d5_11: QM31 = (update_registers_input_dst_limb_21
        - qm31_const::<136, 0, 0, 0>());
    let diff_from_p_tmp_783d5_12: QM31 = (update_registers_input_dst_limb_27
        - qm31_const::<256, 0, 0, 0>());

    // Constraint - dst_not_p
    let constraint_quotient = (((((((((((((((((((((((((((((((diff_from_p_tmp_783d5_10
        * diff_from_p_tmp_783d5_10)
        + update_registers_input_dst_limb_1)
        + update_registers_input_dst_limb_2)
        + update_registers_input_dst_limb_3)
        + update_registers_input_dst_limb_4)
        + update_registers_input_dst_limb_5)
        + update_registers_input_dst_limb_6)
        + update_registers_input_dst_limb_7)
        + update_registers_input_dst_limb_8)
        + update_registers_input_dst_limb_9)
        + update_registers_input_dst_limb_10)
        + update_registers_input_dst_limb_11)
        + update_registers_input_dst_limb_12)
        + update_registers_input_dst_limb_13)
        + update_registers_input_dst_limb_14)
        + update_registers_input_dst_limb_15)
        + update_registers_input_dst_limb_16)
        + update_registers_input_dst_limb_17)
        + update_registers_input_dst_limb_18)
        + update_registers_input_dst_limb_19)
        + update_registers_input_dst_limb_20)
        + (diff_from_p_tmp_783d5_11 * diff_from_p_tmp_783d5_11))
        + update_registers_input_dst_limb_22)
        + update_registers_input_dst_limb_23)
        + update_registers_input_dst_limb_24)
        + update_registers_input_dst_limb_25)
        + update_registers_input_dst_limb_26)
        + (diff_from_p_tmp_783d5_12 * diff_from_p_tmp_783d5_12))
        * dst_sum_squares_inv_col4)
        - qm31_const::<1, 0, 0, 0>()));
    sum = sum * random_coeff + constraint_quotient;
    let dst_sum_tmp_783d5_13: QM31 = (((((((((((((((((((((((((((update_registers_input_dst_limb_0
        + update_registers_input_dst_limb_1)
        + update_registers_input_dst_limb_2)
        + update_registers_input_dst_limb_3)
        + update_registers_input_dst_limb_4)
        + update_registers_input_dst_limb_5)
        + update_registers_input_dst_limb_6)
        + update_registers_input_dst_limb_7)
        + update_registers_input_dst_limb_8)
        + update_registers_input_dst_limb_9)
        + update_registers_input_dst_limb_10)
        + update_registers_input_dst_limb_11)
        + update_registers_input_dst_limb_12)
        + update_registers_input_dst_limb_13)
        + update_registers_input_dst_limb_14)
        + update_registers_input_dst_limb_15)
        + update_registers_input_dst_limb_16)
        + update_registers_input_dst_limb_17)
        + update_registers_input_dst_limb_18)
        + update_registers_input_dst_limb_19)
        + update_registers_input_dst_limb_20)
        + update_registers_input_dst_limb_21)
        + update_registers_input_dst_limb_22)
        + update_registers_input_dst_limb_23)
        + update_registers_input_dst_limb_24)
        + update_registers_input_dst_limb_25)
        + update_registers_input_dst_limb_26)
        + update_registers_input_dst_limb_27);

    // Constraint - op1_as_rel_imm_cond
    let constraint_quotient = ((op1_as_rel_imm_cond_col6
        - (update_registers_input_pc_update_jnz * dst_sum_tmp_783d5_13)));
    sum = sum * random_coeff + constraint_quotient;
    let cond_felt_252_as_rel_imm_output_tmp_783d5_21: QM31 = cond_felt_252_as_rel_imm_evaluate(
        [
            update_registers_input_op1_limb_0, update_registers_input_op1_limb_1,
            update_registers_input_op1_limb_2, update_registers_input_op1_limb_3,
            update_registers_input_op1_limb_4, update_registers_input_op1_limb_5,
            update_registers_input_op1_limb_6, update_registers_input_op1_limb_7,
            update_registers_input_op1_limb_8, update_registers_input_op1_limb_9,
            update_registers_input_op1_limb_10, update_registers_input_op1_limb_11,
            update_registers_input_op1_limb_12, update_registers_input_op1_limb_13,
            update_registers_input_op1_limb_14, update_registers_input_op1_limb_15,
            update_registers_input_op1_limb_16, update_registers_input_op1_limb_17,
            update_registers_input_op1_limb_18, update_registers_input_op1_limb_19,
            update_registers_input_op1_limb_20, update_registers_input_op1_limb_21,
            update_registers_input_op1_limb_22, update_registers_input_op1_limb_23,
            update_registers_input_op1_limb_24, update_registers_input_op1_limb_25,
            update_registers_input_op1_limb_26, update_registers_input_op1_limb_27,
            op1_as_rel_imm_cond_col6,
        ],
        msb_col7,
        mid_limbs_set_col8,
        partial_limb_msb_col9,
        common_lookup_elements,
        ref sum,
        random_coeff,
    );

    // Constraint - Constraint1 for conditional jump
    let constraint_quotient = (((next_pc_jnz_col10
        - (update_registers_input_pc + cond_felt_252_as_rel_imm_output_tmp_783d5_21))
        * dst_sum_tmp_783d5_13));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - Constraint2 for conditional jump
    let constraint_quotient = (((next_pc_jnz_col10
        - (update_registers_input_pc + update_registers_input_instruction_size))
        * ((dst_sum_tmp_783d5_13 * dst_sum_inv_col5) - qm31_const::<1, 0, 0, 0>())));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - next_pc
    let constraint_quotient = ((next_pc_col11
        - ((((update_registers_input_pc_update_regular
            * (update_registers_input_pc + update_registers_input_instruction_size))
            + (update_registers_input_pc_update_jump * cond_felt_252_as_rel_imm_output_tmp_783d5_9))
            + (update_registers_input_pc_update_jump_rel
                * (update_registers_input_pc + cond_felt_252_as_rel_imm_output_tmp_783d5_9)))
            + (update_registers_input_pc_update_jnz * next_pc_jnz_col10))));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - next_ap
    let constraint_quotient = ((next_ap_col12
        - (((update_registers_input_ap
            + (update_registers_input_ap_update_add * cond_felt_252_as_rel_imm_output_tmp_783d5_9))
            + update_registers_input_ap_update_add_1)
            + (update_registers_input_opcode_call * qm31_const::<2, 0, 0, 0>()))));
    sum = sum * random_coeff + constraint_quotient;
    range_check_29_evaluate(
        next_ap_col12,
        range_check_29_bot11bits_col13,
        common_lookup_elements,
        ref range_check_18_sum_0,
        ref numerator_0,
        ref range_check_11_sum_1,
        ref numerator_1,
        ref sum,
        random_coeff,
    );

    // Constraint - next_fp
    let constraint_quotient = ((next_fp_col14
        - (((update_registers_input_fp_update_regular * update_registers_input_fp)
            + (update_registers_input_opcode_ret * cond_felt_252_as_addr_output_tmp_783d5_2))
            + (update_registers_input_opcode_call
                * (update_registers_input_ap + qm31_const::<2, 0, 0, 0>())))));
    sum = sum * random_coeff + constraint_quotient;

    []
}
