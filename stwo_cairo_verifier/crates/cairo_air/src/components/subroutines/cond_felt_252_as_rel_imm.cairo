// AIR version 9acd5104
use crate::components::subroutines::cond_decode_small_sign::cond_decode_small_sign_evaluate;
use crate::components::subroutines::cond_range_check_2::cond_range_check_2_evaluate;
use crate::prelude::*;


pub fn cond_felt_252_as_rel_imm_evaluate(
    input: [QM31; 29],
    msb_col0: QM31,
    mid_limbs_set_col1: QM31,
    partial_limb_msb_col2: QM31,
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
    cond_decode_small_sign_evaluate(
        cond_felt_252_as_rel_imm_input_limb_28,
        msb_col0,
        mid_limbs_set_col1,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );
    let remainder_bits_tmp_1e9bf_3: QM31 = (cond_felt_252_as_rel_imm_input_limb_3
        - (mid_limbs_set_col1 * qm31_const::<508, 0, 0, 0>()));
    cond_range_check_2_evaluate(
        [remainder_bits_tmp_1e9bf_3, cond_felt_252_as_rel_imm_input_limb_28],
        partial_limb_msb_col2,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    // Constraint - rel_imm limb 4 is fixed
    let constraint_quotient = ((cond_felt_252_as_rel_imm_input_limb_28
        * (cond_felt_252_as_rel_imm_input_limb_4
            - (mid_limbs_set_col1 * qm31_const::<511, 0, 0, 0>()))))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - rel_imm limb 5 is fixed
    let constraint_quotient = ((cond_felt_252_as_rel_imm_input_limb_28
        * (cond_felt_252_as_rel_imm_input_limb_5
            - (mid_limbs_set_col1 * qm31_const::<511, 0, 0, 0>()))))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - rel_imm limb 6 is fixed
    let constraint_quotient = ((cond_felt_252_as_rel_imm_input_limb_28
        * (cond_felt_252_as_rel_imm_input_limb_6
            - (mid_limbs_set_col1 * qm31_const::<511, 0, 0, 0>()))))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - rel_imm limb 7 is fixed
    let constraint_quotient = ((cond_felt_252_as_rel_imm_input_limb_28
        * (cond_felt_252_as_rel_imm_input_limb_7
            - (mid_limbs_set_col1 * qm31_const::<511, 0, 0, 0>()))))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - rel_imm limb 8 is fixed
    let constraint_quotient = ((cond_felt_252_as_rel_imm_input_limb_28
        * (cond_felt_252_as_rel_imm_input_limb_8
            - (mid_limbs_set_col1 * qm31_const::<511, 0, 0, 0>()))))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - rel_imm limb 9 is fixed
    let constraint_quotient = ((cond_felt_252_as_rel_imm_input_limb_28
        * (cond_felt_252_as_rel_imm_input_limb_9
            - (mid_limbs_set_col1 * qm31_const::<511, 0, 0, 0>()))))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - rel_imm limb 10 is fixed
    let constraint_quotient = ((cond_felt_252_as_rel_imm_input_limb_28
        * (cond_felt_252_as_rel_imm_input_limb_10
            - (mid_limbs_set_col1 * qm31_const::<511, 0, 0, 0>()))))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - rel_imm limb 11 is fixed
    let constraint_quotient = ((cond_felt_252_as_rel_imm_input_limb_28
        * (cond_felt_252_as_rel_imm_input_limb_11
            - (mid_limbs_set_col1 * qm31_const::<511, 0, 0, 0>()))))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - rel_imm limb 12 is fixed
    let constraint_quotient = ((cond_felt_252_as_rel_imm_input_limb_28
        * (cond_felt_252_as_rel_imm_input_limb_12
            - (mid_limbs_set_col1 * qm31_const::<511, 0, 0, 0>()))))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - rel_imm limb 13 is fixed
    let constraint_quotient = ((cond_felt_252_as_rel_imm_input_limb_28
        * (cond_felt_252_as_rel_imm_input_limb_13
            - (mid_limbs_set_col1 * qm31_const::<511, 0, 0, 0>()))))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - rel_imm limb 14 is fixed
    let constraint_quotient = ((cond_felt_252_as_rel_imm_input_limb_28
        * (cond_felt_252_as_rel_imm_input_limb_14
            - (mid_limbs_set_col1 * qm31_const::<511, 0, 0, 0>()))))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - rel_imm limb 15 is fixed
    let constraint_quotient = ((cond_felt_252_as_rel_imm_input_limb_28
        * (cond_felt_252_as_rel_imm_input_limb_15
            - (mid_limbs_set_col1 * qm31_const::<511, 0, 0, 0>()))))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - rel_imm limb 16 is fixed
    let constraint_quotient = ((cond_felt_252_as_rel_imm_input_limb_28
        * (cond_felt_252_as_rel_imm_input_limb_16
            - (mid_limbs_set_col1 * qm31_const::<511, 0, 0, 0>()))))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - rel_imm limb 17 is fixed
    let constraint_quotient = ((cond_felt_252_as_rel_imm_input_limb_28
        * (cond_felt_252_as_rel_imm_input_limb_17
            - (mid_limbs_set_col1 * qm31_const::<511, 0, 0, 0>()))))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - rel_imm limb 18 is fixed
    let constraint_quotient = ((cond_felt_252_as_rel_imm_input_limb_28
        * (cond_felt_252_as_rel_imm_input_limb_18
            - (mid_limbs_set_col1 * qm31_const::<511, 0, 0, 0>()))))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - rel_imm limb 19 is fixed
    let constraint_quotient = ((cond_felt_252_as_rel_imm_input_limb_28
        * (cond_felt_252_as_rel_imm_input_limb_19
            - (mid_limbs_set_col1 * qm31_const::<511, 0, 0, 0>()))))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - rel_imm limb 20 is fixed
    let constraint_quotient = ((cond_felt_252_as_rel_imm_input_limb_28
        * (cond_felt_252_as_rel_imm_input_limb_20
            - (mid_limbs_set_col1 * qm31_const::<511, 0, 0, 0>()))))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - rel_imm limb 21 is fixed
    let constraint_quotient = ((cond_felt_252_as_rel_imm_input_limb_28
        * (cond_felt_252_as_rel_imm_input_limb_21
            - ((qm31_const::<136, 0, 0, 0>() * msb_col0) - mid_limbs_set_col1))))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - rel_imm limb 22 is fixed
    let constraint_quotient = ((cond_felt_252_as_rel_imm_input_limb_28
        * cond_felt_252_as_rel_imm_input_limb_22))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - rel_imm limb 23 is fixed
    let constraint_quotient = ((cond_felt_252_as_rel_imm_input_limb_28
        * cond_felt_252_as_rel_imm_input_limb_23))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - rel_imm limb 24 is fixed
    let constraint_quotient = ((cond_felt_252_as_rel_imm_input_limb_28
        * cond_felt_252_as_rel_imm_input_limb_24))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - rel_imm limb 25 is fixed
    let constraint_quotient = ((cond_felt_252_as_rel_imm_input_limb_28
        * cond_felt_252_as_rel_imm_input_limb_25))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - rel_imm limb 26 is fixed
    let constraint_quotient = ((cond_felt_252_as_rel_imm_input_limb_28
        * cond_felt_252_as_rel_imm_input_limb_26))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - rel_imm limb 27 is fixed
    let constraint_quotient = ((cond_felt_252_as_rel_imm_input_limb_28
        * (cond_felt_252_as_rel_imm_input_limb_27 - (msb_col0 * qm31_const::<256, 0, 0, 0>()))))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    (((((cond_felt_252_as_rel_imm_input_limb_0
        + (cond_felt_252_as_rel_imm_input_limb_1 * qm31_const::<512, 0, 0, 0>()))
        + (cond_felt_252_as_rel_imm_input_limb_2 * qm31_const::<262144, 0, 0, 0>()))
        + (remainder_bits_tmp_1e9bf_3 * qm31_const::<134217728, 0, 0, 0>()))
        - msb_col0)
        - (qm31_const::<536870912, 0, 0, 0>() * mid_limbs_set_col1))
}
