// This file was created by the AIR team.

use crate::components::subroutines::cond_felt_252_as_addr::cond_felt_252_as_addr_evaluate;
use crate::prelude::*;


pub fn handle_opcodes_evaluate(
    input: [QM31; 98],
    partial_limb_msb_col0: QM31,
    partial_limb_msb_col1: QM31,
    common_lookup_elements: @CommonLookupElements,
    ref sum: QM31,
    random_coeff: QM31,
) -> [QM31; 0] {
    let [
        handle_opcodes_input_pc,
        handle_opcodes_input_fp,
        handle_opcodes_input_dst_base_fp,
        handle_opcodes_input_op0_base_fp,
        handle_opcodes_input_op1_base_fp,
        handle_opcodes_input_pc_update_jump,
        handle_opcodes_input_opcode_call,
        handle_opcodes_input_opcode_ret,
        handle_opcodes_input_opcode_assert_eq,
        handle_opcodes_input_res_op1,
        handle_opcodes_input_instruction_size,
        handle_opcodes_input_offset0,
        handle_opcodes_input_offset1,
        handle_opcodes_input_offset2,
        handle_opcodes_input_dst_limb_0,
        handle_opcodes_input_dst_limb_1,
        handle_opcodes_input_dst_limb_2,
        handle_opcodes_input_dst_limb_3,
        handle_opcodes_input_dst_limb_4,
        handle_opcodes_input_dst_limb_5,
        handle_opcodes_input_dst_limb_6,
        handle_opcodes_input_dst_limb_7,
        handle_opcodes_input_dst_limb_8,
        handle_opcodes_input_dst_limb_9,
        handle_opcodes_input_dst_limb_10,
        handle_opcodes_input_dst_limb_11,
        handle_opcodes_input_dst_limb_12,
        handle_opcodes_input_dst_limb_13,
        handle_opcodes_input_dst_limb_14,
        handle_opcodes_input_dst_limb_15,
        handle_opcodes_input_dst_limb_16,
        handle_opcodes_input_dst_limb_17,
        handle_opcodes_input_dst_limb_18,
        handle_opcodes_input_dst_limb_19,
        handle_opcodes_input_dst_limb_20,
        handle_opcodes_input_dst_limb_21,
        handle_opcodes_input_dst_limb_22,
        handle_opcodes_input_dst_limb_23,
        handle_opcodes_input_dst_limb_24,
        handle_opcodes_input_dst_limb_25,
        handle_opcodes_input_dst_limb_26,
        handle_opcodes_input_dst_limb_27,
        handle_opcodes_input_op0_limb_0,
        handle_opcodes_input_op0_limb_1,
        handle_opcodes_input_op0_limb_2,
        handle_opcodes_input_op0_limb_3,
        handle_opcodes_input_op0_limb_4,
        handle_opcodes_input_op0_limb_5,
        handle_opcodes_input_op0_limb_6,
        handle_opcodes_input_op0_limb_7,
        handle_opcodes_input_op0_limb_8,
        handle_opcodes_input_op0_limb_9,
        handle_opcodes_input_op0_limb_10,
        handle_opcodes_input_op0_limb_11,
        handle_opcodes_input_op0_limb_12,
        handle_opcodes_input_op0_limb_13,
        handle_opcodes_input_op0_limb_14,
        handle_opcodes_input_op0_limb_15,
        handle_opcodes_input_op0_limb_16,
        handle_opcodes_input_op0_limb_17,
        handle_opcodes_input_op0_limb_18,
        handle_opcodes_input_op0_limb_19,
        handle_opcodes_input_op0_limb_20,
        handle_opcodes_input_op0_limb_21,
        handle_opcodes_input_op0_limb_22,
        handle_opcodes_input_op0_limb_23,
        handle_opcodes_input_op0_limb_24,
        handle_opcodes_input_op0_limb_25,
        handle_opcodes_input_op0_limb_26,
        handle_opcodes_input_op0_limb_27,
        handle_opcodes_input_res_limb_0,
        handle_opcodes_input_res_limb_1,
        handle_opcodes_input_res_limb_2,
        handle_opcodes_input_res_limb_3,
        handle_opcodes_input_res_limb_4,
        handle_opcodes_input_res_limb_5,
        handle_opcodes_input_res_limb_6,
        handle_opcodes_input_res_limb_7,
        handle_opcodes_input_res_limb_8,
        handle_opcodes_input_res_limb_9,
        handle_opcodes_input_res_limb_10,
        handle_opcodes_input_res_limb_11,
        handle_opcodes_input_res_limb_12,
        handle_opcodes_input_res_limb_13,
        handle_opcodes_input_res_limb_14,
        handle_opcodes_input_res_limb_15,
        handle_opcodes_input_res_limb_16,
        handle_opcodes_input_res_limb_17,
        handle_opcodes_input_res_limb_18,
        handle_opcodes_input_res_limb_19,
        handle_opcodes_input_res_limb_20,
        handle_opcodes_input_res_limb_21,
        handle_opcodes_input_res_limb_22,
        handle_opcodes_input_res_limb_23,
        handle_opcodes_input_res_limb_24,
        handle_opcodes_input_res_limb_25,
        handle_opcodes_input_res_limb_26,
        handle_opcodes_input_res_limb_27,
    ] =
        input;

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_opcode_assert_eq
        * (handle_opcodes_input_res_limb_0 - handle_opcodes_input_dst_limb_0)));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_opcode_assert_eq
        * (handle_opcodes_input_res_limb_1 - handle_opcodes_input_dst_limb_1)));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_opcode_assert_eq
        * (handle_opcodes_input_res_limb_2 - handle_opcodes_input_dst_limb_2)));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_opcode_assert_eq
        * (handle_opcodes_input_res_limb_3 - handle_opcodes_input_dst_limb_3)));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_opcode_assert_eq
        * (handle_opcodes_input_res_limb_4 - handle_opcodes_input_dst_limb_4)));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_opcode_assert_eq
        * (handle_opcodes_input_res_limb_5 - handle_opcodes_input_dst_limb_5)));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_opcode_assert_eq
        * (handle_opcodes_input_res_limb_6 - handle_opcodes_input_dst_limb_6)));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_opcode_assert_eq
        * (handle_opcodes_input_res_limb_7 - handle_opcodes_input_dst_limb_7)));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_opcode_assert_eq
        * (handle_opcodes_input_res_limb_8 - handle_opcodes_input_dst_limb_8)));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_opcode_assert_eq
        * (handle_opcodes_input_res_limb_9 - handle_opcodes_input_dst_limb_9)));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_opcode_assert_eq
        * (handle_opcodes_input_res_limb_10 - handle_opcodes_input_dst_limb_10)));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_opcode_assert_eq
        * (handle_opcodes_input_res_limb_11 - handle_opcodes_input_dst_limb_11)));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_opcode_assert_eq
        * (handle_opcodes_input_res_limb_12 - handle_opcodes_input_dst_limb_12)));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_opcode_assert_eq
        * (handle_opcodes_input_res_limb_13 - handle_opcodes_input_dst_limb_13)));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_opcode_assert_eq
        * (handle_opcodes_input_res_limb_14 - handle_opcodes_input_dst_limb_14)));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_opcode_assert_eq
        * (handle_opcodes_input_res_limb_15 - handle_opcodes_input_dst_limb_15)));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_opcode_assert_eq
        * (handle_opcodes_input_res_limb_16 - handle_opcodes_input_dst_limb_16)));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_opcode_assert_eq
        * (handle_opcodes_input_res_limb_17 - handle_opcodes_input_dst_limb_17)));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_opcode_assert_eq
        * (handle_opcodes_input_res_limb_18 - handle_opcodes_input_dst_limb_18)));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_opcode_assert_eq
        * (handle_opcodes_input_res_limb_19 - handle_opcodes_input_dst_limb_19)));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_opcode_assert_eq
        * (handle_opcodes_input_res_limb_20 - handle_opcodes_input_dst_limb_20)));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_opcode_assert_eq
        * (handle_opcodes_input_res_limb_21 - handle_opcodes_input_dst_limb_21)));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_opcode_assert_eq
        * (handle_opcodes_input_res_limb_22 - handle_opcodes_input_dst_limb_22)));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_opcode_assert_eq
        * (handle_opcodes_input_res_limb_23 - handle_opcodes_input_dst_limb_23)));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_opcode_assert_eq
        * (handle_opcodes_input_res_limb_24 - handle_opcodes_input_dst_limb_24)));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_opcode_assert_eq
        * (handle_opcodes_input_res_limb_25 - handle_opcodes_input_dst_limb_25)));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_opcode_assert_eq
        * (handle_opcodes_input_res_limb_26 - handle_opcodes_input_dst_limb_26)));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_opcode_assert_eq
        * (handle_opcodes_input_res_limb_27 - handle_opcodes_input_dst_limb_27)));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - ret opcode offset0 equals -2
    let constraint_quotient = ((handle_opcodes_input_opcode_ret
        * (handle_opcodes_input_offset0 + qm31_const::<2, 0, 0, 0>())));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - ret opcode offset2 equals -1
    let constraint_quotient = ((handle_opcodes_input_opcode_ret
        * (handle_opcodes_input_offset2 + qm31_const::<1, 0, 0, 0>())));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - ret opcode flags pc_update_jump and dst_base_fp and op1_base_fp_and_res_op1 are
    // on
    let constraint_quotient = ((handle_opcodes_input_opcode_ret
        * ((((qm31_const::<4, 0, 0, 0>() - handle_opcodes_input_pc_update_jump)
            - handle_opcodes_input_dst_base_fp)
            - handle_opcodes_input_op1_base_fp)
            - handle_opcodes_input_res_op1)));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - call opcode offset0 equals 0
    let constraint_quotient = ((handle_opcodes_input_opcode_call * handle_opcodes_input_offset0));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - call opcode offset1 equals 1
    let constraint_quotient = ((handle_opcodes_input_opcode_call
        * (qm31_const::<1, 0, 0, 0>() - handle_opcodes_input_offset1)));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - call opcode flags op0_base_fp and dst_base_fp are off
    let constraint_quotient = ((handle_opcodes_input_opcode_call
        * (handle_opcodes_input_op0_base_fp + handle_opcodes_input_dst_base_fp)));
    sum = sum * random_coeff + constraint_quotient;
    let cond_felt_252_as_addr_output_tmp_aa5c5_2: QM31 = cond_felt_252_as_addr_evaluate(
        [
            handle_opcodes_input_dst_limb_0, handle_opcodes_input_dst_limb_1,
            handle_opcodes_input_dst_limb_2, handle_opcodes_input_dst_limb_3,
            handle_opcodes_input_dst_limb_4, handle_opcodes_input_dst_limb_5,
            handle_opcodes_input_dst_limb_6, handle_opcodes_input_dst_limb_7,
            handle_opcodes_input_dst_limb_8, handle_opcodes_input_dst_limb_9,
            handle_opcodes_input_dst_limb_10, handle_opcodes_input_dst_limb_11,
            handle_opcodes_input_dst_limb_12, handle_opcodes_input_dst_limb_13,
            handle_opcodes_input_dst_limb_14, handle_opcodes_input_dst_limb_15,
            handle_opcodes_input_dst_limb_16, handle_opcodes_input_dst_limb_17,
            handle_opcodes_input_dst_limb_18, handle_opcodes_input_dst_limb_19,
            handle_opcodes_input_dst_limb_20, handle_opcodes_input_dst_limb_21,
            handle_opcodes_input_dst_limb_22, handle_opcodes_input_dst_limb_23,
            handle_opcodes_input_dst_limb_24, handle_opcodes_input_dst_limb_25,
            handle_opcodes_input_dst_limb_26, handle_opcodes_input_dst_limb_27,
            handle_opcodes_input_opcode_call,
        ],
        partial_limb_msb_col0,
        common_lookup_elements,
        ref sum,
        random_coeff,
    );

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_opcode_call
        * (cond_felt_252_as_addr_output_tmp_aa5c5_2 - handle_opcodes_input_fp)));
    sum = sum * random_coeff + constraint_quotient;
    let cond_felt_252_as_addr_output_tmp_aa5c5_5: QM31 = cond_felt_252_as_addr_evaluate(
        [
            handle_opcodes_input_op0_limb_0, handle_opcodes_input_op0_limb_1,
            handle_opcodes_input_op0_limb_2, handle_opcodes_input_op0_limb_3,
            handle_opcodes_input_op0_limb_4, handle_opcodes_input_op0_limb_5,
            handle_opcodes_input_op0_limb_6, handle_opcodes_input_op0_limb_7,
            handle_opcodes_input_op0_limb_8, handle_opcodes_input_op0_limb_9,
            handle_opcodes_input_op0_limb_10, handle_opcodes_input_op0_limb_11,
            handle_opcodes_input_op0_limb_12, handle_opcodes_input_op0_limb_13,
            handle_opcodes_input_op0_limb_14, handle_opcodes_input_op0_limb_15,
            handle_opcodes_input_op0_limb_16, handle_opcodes_input_op0_limb_17,
            handle_opcodes_input_op0_limb_18, handle_opcodes_input_op0_limb_19,
            handle_opcodes_input_op0_limb_20, handle_opcodes_input_op0_limb_21,
            handle_opcodes_input_op0_limb_22, handle_opcodes_input_op0_limb_23,
            handle_opcodes_input_op0_limb_24, handle_opcodes_input_op0_limb_25,
            handle_opcodes_input_op0_limb_26, handle_opcodes_input_op0_limb_27,
            handle_opcodes_input_opcode_call,
        ],
        partial_limb_msb_col1,
        common_lookup_elements,
        ref sum,
        random_coeff,
    );

    // Constraint -
    let constraint_quotient = ((handle_opcodes_input_opcode_call
        * (cond_felt_252_as_addr_output_tmp_aa5c5_5
            - (handle_opcodes_input_pc + handle_opcodes_input_instruction_size))));
    sum = sum * random_coeff + constraint_quotient;

    []
}
