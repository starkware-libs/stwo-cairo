// This file was created by the AIR team.

use crate::components::subroutines::cond_range_check_2::cond_range_check_2_evaluate;
use crate::components::subroutines::decode_small_sign::decode_small_sign_evaluate;
use crate::prelude::*;


pub fn cond_felt_252_as_rel_imm_evaluate(
    input: [QM31; 29],
    msb_col0: QM31,
    mid_limbs_set_col1: QM31,
    partial_limb_msb_col2: QM31,
    common_lookup_elements: @crate::CommonElements,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> QM31 {
    let [
        cond_felt_252_as_rel_imm_input_limb_0,
        cond_felt_252_as_rel_imm_input_limb_1,
        cond_felt_252_as_rel_imm_input_limb_2,
        cond_felt_252_as_rel_imm_input_limb_3,
        cond_felt_252_as_rel_imm_input_limb_4,
        cond_felt_252_as_rel_imm_input_limb_5,
        cond_felt_252_as_rel_imm_input_limb_6,
        cond_felt_252_as_rel_imm_input_limb_7,
        cond_felt_252_as_rel_imm_input_limb_8,
        cond_felt_252_as_rel_imm_input_limb_9,
        cond_felt_252_as_rel_imm_input_limb_10,
        cond_felt_252_as_rel_imm_input_limb_11,
        cond_felt_252_as_rel_imm_input_limb_12,
        cond_felt_252_as_rel_imm_input_limb_13,
        cond_felt_252_as_rel_imm_input_limb_14,
        cond_felt_252_as_rel_imm_input_limb_15,
        cond_felt_252_as_rel_imm_input_limb_16,
        cond_felt_252_as_rel_imm_input_limb_17,
        cond_felt_252_as_rel_imm_input_limb_18,
        cond_felt_252_as_rel_imm_input_limb_19,
        cond_felt_252_as_rel_imm_input_limb_20,
        cond_felt_252_as_rel_imm_input_limb_21,
        cond_felt_252_as_rel_imm_input_limb_22,
        cond_felt_252_as_rel_imm_input_limb_23,
        cond_felt_252_as_rel_imm_input_limb_24,
        cond_felt_252_as_rel_imm_input_limb_25,
        cond_felt_252_as_rel_imm_input_limb_26,
        cond_felt_252_as_rel_imm_input_limb_27,
        cond_felt_252_as_rel_imm_input_limb_28,
    ] =
        input;
    let [
        decode_small_sign_output_tmp_1e9bf_2_limb3_7_high_bits,
        decode_small_sign_output_tmp_1e9bf_2_limbs4_to_20,
        decode_small_sign_output_tmp_1e9bf_2_limb21,
        decode_small_sign_output_tmp_1e9bf_2_limb27,
    ] =
        decode_small_sign_evaluate(
        [],
        msb_col0,
        mid_limbs_set_col1,
        common_lookup_elements,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );
    let remainder_bits_tmp_1e9bf_3: QM31 = (cond_felt_252_as_rel_imm_input_limb_3
        - decode_small_sign_output_tmp_1e9bf_2_limb3_7_high_bits);
    cond_range_check_2_evaluate(
        [remainder_bits_tmp_1e9bf_3, cond_felt_252_as_rel_imm_input_limb_28],
        partial_limb_msb_col2,
        common_lookup_elements,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    // Constraint - When the condition holds, limbs 4-20 must be zero or 0x1ff
    let constraint_quotient = ((cond_felt_252_as_rel_imm_input_limb_28
        * (((((((((((((((((cond_felt_252_as_rel_imm_input_limb_4
            + cond_felt_252_as_rel_imm_input_limb_5)
            + cond_felt_252_as_rel_imm_input_limb_6)
            + cond_felt_252_as_rel_imm_input_limb_7)
            + cond_felt_252_as_rel_imm_input_limb_8)
            + cond_felt_252_as_rel_imm_input_limb_9)
            + cond_felt_252_as_rel_imm_input_limb_10)
            + cond_felt_252_as_rel_imm_input_limb_11)
            + cond_felt_252_as_rel_imm_input_limb_12)
            + cond_felt_252_as_rel_imm_input_limb_13)
            + cond_felt_252_as_rel_imm_input_limb_14)
            + cond_felt_252_as_rel_imm_input_limb_15)
            + cond_felt_252_as_rel_imm_input_limb_16)
            + cond_felt_252_as_rel_imm_input_limb_17)
            + cond_felt_252_as_rel_imm_input_limb_18)
            + cond_felt_252_as_rel_imm_input_limb_19)
            + cond_felt_252_as_rel_imm_input_limb_20)
            - (decode_small_sign_output_tmp_1e9bf_2_limbs4_to_20 * qm31_const::<17, 0, 0, 0>()))))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - When the condition holds, limb 21 must be 0x0, 0x88 or 0x87
    let constraint_quotient = ((cond_felt_252_as_rel_imm_input_limb_28
        * (cond_felt_252_as_rel_imm_input_limb_21 - decode_small_sign_output_tmp_1e9bf_2_limb21)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - When the condition holds, limbs 22-26 must be zero
    let constraint_quotient = ((cond_felt_252_as_rel_imm_input_limb_28
        * ((((cond_felt_252_as_rel_imm_input_limb_22 + cond_felt_252_as_rel_imm_input_limb_23)
            + cond_felt_252_as_rel_imm_input_limb_24)
            + cond_felt_252_as_rel_imm_input_limb_25)
            + cond_felt_252_as_rel_imm_input_limb_26)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - When the condition holds, limb 27 must be 0x0 or 0x100
    let constraint_quotient = ((cond_felt_252_as_rel_imm_input_limb_28
        * (cond_felt_252_as_rel_imm_input_limb_27 - decode_small_sign_output_tmp_1e9bf_2_limb27)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    (((((cond_felt_252_as_rel_imm_input_limb_0
        + (cond_felt_252_as_rel_imm_input_limb_1 * qm31_const::<512, 0, 0, 0>()))
        + (cond_felt_252_as_rel_imm_input_limb_2 * qm31_const::<262144, 0, 0, 0>()))
        + (remainder_bits_tmp_1e9bf_3 * qm31_const::<134217728, 0, 0, 0>()))
        - msb_col0)
        - (qm31_const::<536870912, 0, 0, 0>() * mid_limbs_set_col1))
}
