// AIR version aca38612
use core::num::traits::Zero;
use stwo_constraint_framework::{
    LookupElementsImpl, PreprocessedColumn, PreprocessedColumnSet, PreprocessedColumnSetImpl,
    PreprocessedMaskValues, PreprocessedMaskValuesImpl,
};
use stwo_verifier_core::channel::{Channel, ChannelTrait};
use stwo_verifier_core::circle::{
    CirclePoint, CirclePointIndexTrait, CirclePointQM31AddCirclePointM31Trait,
};
use stwo_verifier_core::fields::Invertible;
use stwo_verifier_core::fields::m31::{M31, m31};
use stwo_verifier_core::fields::qm31::{QM31, QM31Impl, QM31Serde, QM31Zero, qm31_const};
use stwo_verifier_core::poly::circle::CanonicCosetImpl;
use stwo_verifier_core::utils::{ArrayImpl, pow2};
use stwo_verifier_core::{ColumnArray, ColumnSpan, TreeArray};
use crate::PreprocessedColumnTrait;
use crate::cairo_component::CairoComponent;
use crate::components::subroutines::cond_felt_252_as_addr::cond_felt_252_as_addr_evaluate;
use crate::components::subroutines::cond_felt_252_as_rel_imm::cond_felt_252_as_rel_imm_evaluate;
use crate::components::subroutines::range_check_ap::range_check_ap_evaluate;


pub fn update_registers_evaluate(
    input: [QM31; 97],
    msb_col0: QM31,
    mid_limbs_set_col1: QM31,
    dst_sum_squares_inv_col2: QM31,
    dst_sum_inv_col3: QM31,
    op1_as_rel_imm_cond_col4: QM31,
    msb_col5: QM31,
    mid_limbs_set_col6: QM31,
    next_pc_jnz_col7: QM31,
    next_pc_col8: QM31,
    next_ap_col9: QM31,
    range_check_ap_bot8bits_col10: QM31,
    next_fp_col11: QM31,
    range_check_19_lookup_elements: @crate::RangeCheck_19Elements,
    range_check_8_lookup_elements: @crate::RangeCheck_8Elements,
    ref range_check_19_sum_0: QM31,
    ref range_check_8_sum_1: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
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

    let output: [QM31; 1] = cond_felt_252_as_addr_evaluate(
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
            update_registers_input_pc_update_jump,
        ],
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );
    let [cond_felt_252_as_addr_output_tmp_783d5_0] = output;

    let output: [QM31; 1] = cond_felt_252_as_addr_evaluate(
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
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );
    let [cond_felt_252_as_addr_output_tmp_783d5_1] = output;

    core::internal::revoke_ap_tracking();

    let output: [QM31; 1] = cond_felt_252_as_rel_imm_evaluate(
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
        msb_col0,
        mid_limbs_set_col1,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );
    let [cond_felt_252_as_rel_imm_output_tmp_783d5_5] = output;
    let diff_from_p_tmp_783d5_6: QM31 = (update_registers_input_dst_limb_0
        - qm31_const::<1, 0, 0, 0>());
    let diff_from_p_tmp_783d5_7: QM31 = (update_registers_input_dst_limb_21
        - qm31_const::<136, 0, 0, 0>());
    let diff_from_p_tmp_783d5_8: QM31 = (update_registers_input_dst_limb_27
        - qm31_const::<256, 0, 0, 0>());

    // Constraint - dst_not_p
    let constraint_quotient = (((((((((((((((((((((((((((((((diff_from_p_tmp_783d5_6
        * diff_from_p_tmp_783d5_6)
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
        + (diff_from_p_tmp_783d5_7 * diff_from_p_tmp_783d5_7))
        + update_registers_input_dst_limb_22)
        + update_registers_input_dst_limb_23)
        + update_registers_input_dst_limb_24)
        + update_registers_input_dst_limb_25)
        + update_registers_input_dst_limb_26)
        + (diff_from_p_tmp_783d5_8 * diff_from_p_tmp_783d5_8))
        * dst_sum_squares_inv_col2)
        - qm31_const::<1, 0, 0, 0>()))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let dst_sum_tmp_783d5_9: QM31 = (((((((((((((((((((((((((((update_registers_input_dst_limb_0
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
    let constraint_quotient = ((op1_as_rel_imm_cond_col4
        - (update_registers_input_pc_update_jnz * dst_sum_tmp_783d5_9)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let output: [QM31; 1] = cond_felt_252_as_rel_imm_evaluate(
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
            op1_as_rel_imm_cond_col4,
        ],
        msb_col5,
        mid_limbs_set_col6,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );
    let [cond_felt_252_as_rel_imm_output_tmp_783d5_14] = output;

    // Constraint - Constraint1 for conditional jump
    let constraint_quotient = (((next_pc_jnz_col7
        - (update_registers_input_pc + cond_felt_252_as_rel_imm_output_tmp_783d5_14))
        * dst_sum_tmp_783d5_9))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - Constraint2 for conditional jump
    let constraint_quotient = (((next_pc_jnz_col7
        - (update_registers_input_pc + update_registers_input_instruction_size))
        * ((dst_sum_tmp_783d5_9 * dst_sum_inv_col3) - qm31_const::<1, 0, 0, 0>())))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    core::internal::revoke_ap_tracking();

    // Constraint - next_pc
    let constraint_quotient = ((next_pc_col8
        - ((((update_registers_input_pc_update_regular
            * (update_registers_input_pc + update_registers_input_instruction_size))
            + (update_registers_input_pc_update_jump * cond_felt_252_as_addr_output_tmp_783d5_0))
            + (update_registers_input_pc_update_jump_rel
                * (update_registers_input_pc + cond_felt_252_as_rel_imm_output_tmp_783d5_5)))
            + (update_registers_input_pc_update_jnz * next_pc_jnz_col7))))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - next_ap
    let constraint_quotient = ((next_ap_col9
        - (((update_registers_input_ap
            + (update_registers_input_ap_update_add * cond_felt_252_as_rel_imm_output_tmp_783d5_5))
            + update_registers_input_ap_update_add_1)
            + (update_registers_input_opcode_call * qm31_const::<2, 0, 0, 0>()))))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    range_check_ap_evaluate(
        [next_ap_col9],
        range_check_ap_bot8bits_col10,
        range_check_19_lookup_elements,
        range_check_8_lookup_elements,
        ref range_check_19_sum_0,
        ref range_check_8_sum_1,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    // Constraint - next_fp
    let constraint_quotient = ((next_fp_col11
        - (((update_registers_input_fp_update_regular * update_registers_input_fp)
            + (update_registers_input_opcode_ret * cond_felt_252_as_addr_output_tmp_783d5_1))
            + (update_registers_input_opcode_call
                * (update_registers_input_ap + qm31_const::<2, 0, 0, 0>())))))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    []
}
