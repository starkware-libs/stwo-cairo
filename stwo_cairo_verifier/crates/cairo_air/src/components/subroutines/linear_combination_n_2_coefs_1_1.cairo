// This file was created by the AIR team.

use crate::prelude::*;


pub fn linear_combination_n_2_coefs_1_1_evaluate(
    input: [QM31; 20],
    combination_limb_0_col0: QM31,
    combination_limb_1_col1: QM31,
    combination_limb_2_col2: QM31,
    combination_limb_3_col3: QM31,
    combination_limb_4_col4: QM31,
    combination_limb_5_col5: QM31,
    combination_limb_6_col6: QM31,
    combination_limb_7_col7: QM31,
    combination_limb_8_col8: QM31,
    combination_limb_9_col9: QM31,
    p_coef_col10: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 0] {
    let [
        linear_combination_n_2_coefs_1_1_input_limb_0,
        linear_combination_n_2_coefs_1_1_input_limb_1,
        linear_combination_n_2_coefs_1_1_input_limb_2,
        linear_combination_n_2_coefs_1_1_input_limb_3,
        linear_combination_n_2_coefs_1_1_input_limb_4,
        linear_combination_n_2_coefs_1_1_input_limb_5,
        linear_combination_n_2_coefs_1_1_input_limb_6,
        linear_combination_n_2_coefs_1_1_input_limb_7,
        linear_combination_n_2_coefs_1_1_input_limb_8,
        linear_combination_n_2_coefs_1_1_input_limb_9,
        linear_combination_n_2_coefs_1_1_input_limb_10,
        linear_combination_n_2_coefs_1_1_input_limb_11,
        linear_combination_n_2_coefs_1_1_input_limb_12,
        linear_combination_n_2_coefs_1_1_input_limb_13,
        linear_combination_n_2_coefs_1_1_input_limb_14,
        linear_combination_n_2_coefs_1_1_input_limb_15,
        linear_combination_n_2_coefs_1_1_input_limb_16,
        linear_combination_n_2_coefs_1_1_input_limb_17,
        linear_combination_n_2_coefs_1_1_input_limb_18,
        linear_combination_n_2_coefs_1_1_input_limb_19,
    ] =
        input;
    let carry_0_tmp_2db6d_2: QM31 = ((((linear_combination_n_2_coefs_1_1_input_limb_0
        + linear_combination_n_2_coefs_1_1_input_limb_10)
        - combination_limb_0_col0)
        - p_coef_col10)
        * qm31_const::<16, 0, 0, 0>());
    let carry_1_tmp_2db6d_3: QM31 = ((((carry_0_tmp_2db6d_2
        + linear_combination_n_2_coefs_1_1_input_limb_1)
        + linear_combination_n_2_coefs_1_1_input_limb_11)
        - combination_limb_1_col1)
        * qm31_const::<16, 0, 0, 0>());
    let carry_2_tmp_2db6d_4: QM31 = ((((carry_1_tmp_2db6d_3
        + linear_combination_n_2_coefs_1_1_input_limb_2)
        + linear_combination_n_2_coefs_1_1_input_limb_12)
        - combination_limb_2_col2)
        * qm31_const::<16, 0, 0, 0>());
    let carry_3_tmp_2db6d_5: QM31 = ((((carry_2_tmp_2db6d_4
        + linear_combination_n_2_coefs_1_1_input_limb_3)
        + linear_combination_n_2_coefs_1_1_input_limb_13)
        - combination_limb_3_col3)
        * qm31_const::<16, 0, 0, 0>());
    let carry_4_tmp_2db6d_6: QM31 = ((((carry_3_tmp_2db6d_5
        + linear_combination_n_2_coefs_1_1_input_limb_4)
        + linear_combination_n_2_coefs_1_1_input_limb_14)
        - combination_limb_4_col4)
        * qm31_const::<16, 0, 0, 0>());
    let carry_5_tmp_2db6d_7: QM31 = ((((carry_4_tmp_2db6d_6
        + linear_combination_n_2_coefs_1_1_input_limb_5)
        + linear_combination_n_2_coefs_1_1_input_limb_15)
        - combination_limb_5_col5)
        * qm31_const::<16, 0, 0, 0>());
    let carry_6_tmp_2db6d_8: QM31 = ((((carry_5_tmp_2db6d_7
        + linear_combination_n_2_coefs_1_1_input_limb_6)
        + linear_combination_n_2_coefs_1_1_input_limb_16)
        - combination_limb_6_col6)
        * qm31_const::<16, 0, 0, 0>());
    let carry_7_tmp_2db6d_9: QM31 = (((((carry_6_tmp_2db6d_8
        + linear_combination_n_2_coefs_1_1_input_limb_7)
        + linear_combination_n_2_coefs_1_1_input_limb_17)
        - combination_limb_7_col7)
        - (p_coef_col10 * qm31_const::<136, 0, 0, 0>()))
        * qm31_const::<16, 0, 0, 0>());
    let carry_8_tmp_2db6d_10: QM31 = ((((carry_7_tmp_2db6d_9
        + linear_combination_n_2_coefs_1_1_input_limb_8)
        + linear_combination_n_2_coefs_1_1_input_limb_18)
        - combination_limb_8_col8)
        * qm31_const::<16, 0, 0, 0>());

    // Constraint - final limb constraint
    let constraint_quotient = (((((carry_8_tmp_2db6d_10
        + linear_combination_n_2_coefs_1_1_input_limb_9)
        + linear_combination_n_2_coefs_1_1_input_limb_19)
        - combination_limb_9_col9)
        - (p_coef_col10 * qm31_const::<256, 0, 0, 0>())))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - carry constraint 0
    let constraint_quotient = ((((p_coef_col10 * p_coef_col10) * p_coef_col10) - p_coef_col10))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let biased_carry_1_tmp_2db6d_11: QM31 = carry_0_tmp_2db6d_2;

    // Constraint - carry constraint 1
    let constraint_quotient = ((((biased_carry_1_tmp_2db6d_11 * biased_carry_1_tmp_2db6d_11)
        * biased_carry_1_tmp_2db6d_11)
        - biased_carry_1_tmp_2db6d_11))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let biased_carry_2_tmp_2db6d_12: QM31 = carry_1_tmp_2db6d_3;

    // Constraint - carry constraint 2
    let constraint_quotient = ((((biased_carry_2_tmp_2db6d_12 * biased_carry_2_tmp_2db6d_12)
        * biased_carry_2_tmp_2db6d_12)
        - biased_carry_2_tmp_2db6d_12))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let biased_carry_3_tmp_2db6d_13: QM31 = carry_2_tmp_2db6d_4;

    // Constraint - carry constraint 3
    let constraint_quotient = ((((biased_carry_3_tmp_2db6d_13 * biased_carry_3_tmp_2db6d_13)
        * biased_carry_3_tmp_2db6d_13)
        - biased_carry_3_tmp_2db6d_13))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let biased_carry_4_tmp_2db6d_14: QM31 = carry_3_tmp_2db6d_5;

    // Constraint - carry constraint 4
    let constraint_quotient = ((((biased_carry_4_tmp_2db6d_14 * biased_carry_4_tmp_2db6d_14)
        * biased_carry_4_tmp_2db6d_14)
        - biased_carry_4_tmp_2db6d_14))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let biased_carry_5_tmp_2db6d_15: QM31 = carry_4_tmp_2db6d_6;

    // Constraint - carry constraint 5
    let constraint_quotient = ((((biased_carry_5_tmp_2db6d_15 * biased_carry_5_tmp_2db6d_15)
        * biased_carry_5_tmp_2db6d_15)
        - biased_carry_5_tmp_2db6d_15))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let biased_carry_6_tmp_2db6d_16: QM31 = carry_5_tmp_2db6d_7;

    // Constraint - carry constraint 6
    let constraint_quotient = ((((biased_carry_6_tmp_2db6d_16 * biased_carry_6_tmp_2db6d_16)
        * biased_carry_6_tmp_2db6d_16)
        - biased_carry_6_tmp_2db6d_16))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let biased_carry_7_tmp_2db6d_17: QM31 = carry_6_tmp_2db6d_8;

    // Constraint - carry constraint 7
    let constraint_quotient = ((((biased_carry_7_tmp_2db6d_17 * biased_carry_7_tmp_2db6d_17)
        * biased_carry_7_tmp_2db6d_17)
        - biased_carry_7_tmp_2db6d_17))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let biased_carry_8_tmp_2db6d_18: QM31 = carry_7_tmp_2db6d_9;

    // Constraint - carry constraint 8
    let constraint_quotient = ((((biased_carry_8_tmp_2db6d_18 * biased_carry_8_tmp_2db6d_18)
        * biased_carry_8_tmp_2db6d_18)
        - biased_carry_8_tmp_2db6d_18))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let biased_carry_9_tmp_2db6d_19: QM31 = carry_8_tmp_2db6d_10;

    // Constraint - carry constraint 9
    let constraint_quotient = ((((biased_carry_9_tmp_2db6d_19 * biased_carry_9_tmp_2db6d_19)
        * biased_carry_9_tmp_2db6d_19)
        - biased_carry_9_tmp_2db6d_19))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    []
}
