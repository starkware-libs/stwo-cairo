// This file was created by the AIR team.

use crate::prelude::*;


pub fn verify_add_252_evaluate(
    input: [QM31; 84],
    sub_p_bit_col0: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 0] {
    let [
        verify_add_252_input_a_limb_0,
        verify_add_252_input_a_limb_1,
        verify_add_252_input_a_limb_2,
        verify_add_252_input_a_limb_3,
        verify_add_252_input_a_limb_4,
        verify_add_252_input_a_limb_5,
        verify_add_252_input_a_limb_6,
        verify_add_252_input_a_limb_7,
        verify_add_252_input_a_limb_8,
        verify_add_252_input_a_limb_9,
        verify_add_252_input_a_limb_10,
        verify_add_252_input_a_limb_11,
        verify_add_252_input_a_limb_12,
        verify_add_252_input_a_limb_13,
        verify_add_252_input_a_limb_14,
        verify_add_252_input_a_limb_15,
        verify_add_252_input_a_limb_16,
        verify_add_252_input_a_limb_17,
        verify_add_252_input_a_limb_18,
        verify_add_252_input_a_limb_19,
        verify_add_252_input_a_limb_20,
        verify_add_252_input_a_limb_21,
        verify_add_252_input_a_limb_22,
        verify_add_252_input_a_limb_23,
        verify_add_252_input_a_limb_24,
        verify_add_252_input_a_limb_25,
        verify_add_252_input_a_limb_26,
        verify_add_252_input_a_limb_27,
        verify_add_252_input_b_limb_0,
        verify_add_252_input_b_limb_1,
        verify_add_252_input_b_limb_2,
        verify_add_252_input_b_limb_3,
        verify_add_252_input_b_limb_4,
        verify_add_252_input_b_limb_5,
        verify_add_252_input_b_limb_6,
        verify_add_252_input_b_limb_7,
        verify_add_252_input_b_limb_8,
        verify_add_252_input_b_limb_9,
        verify_add_252_input_b_limb_10,
        verify_add_252_input_b_limb_11,
        verify_add_252_input_b_limb_12,
        verify_add_252_input_b_limb_13,
        verify_add_252_input_b_limb_14,
        verify_add_252_input_b_limb_15,
        verify_add_252_input_b_limb_16,
        verify_add_252_input_b_limb_17,
        verify_add_252_input_b_limb_18,
        verify_add_252_input_b_limb_19,
        verify_add_252_input_b_limb_20,
        verify_add_252_input_b_limb_21,
        verify_add_252_input_b_limb_22,
        verify_add_252_input_b_limb_23,
        verify_add_252_input_b_limb_24,
        verify_add_252_input_b_limb_25,
        verify_add_252_input_b_limb_26,
        verify_add_252_input_b_limb_27,
        verify_add_252_input_c_limb_0,
        verify_add_252_input_c_limb_1,
        verify_add_252_input_c_limb_2,
        verify_add_252_input_c_limb_3,
        verify_add_252_input_c_limb_4,
        verify_add_252_input_c_limb_5,
        verify_add_252_input_c_limb_6,
        verify_add_252_input_c_limb_7,
        verify_add_252_input_c_limb_8,
        verify_add_252_input_c_limb_9,
        verify_add_252_input_c_limb_10,
        verify_add_252_input_c_limb_11,
        verify_add_252_input_c_limb_12,
        verify_add_252_input_c_limb_13,
        verify_add_252_input_c_limb_14,
        verify_add_252_input_c_limb_15,
        verify_add_252_input_c_limb_16,
        verify_add_252_input_c_limb_17,
        verify_add_252_input_c_limb_18,
        verify_add_252_input_c_limb_19,
        verify_add_252_input_c_limb_20,
        verify_add_252_input_c_limb_21,
        verify_add_252_input_c_limb_22,
        verify_add_252_input_c_limb_23,
        verify_add_252_input_c_limb_24,
        verify_add_252_input_c_limb_25,
        verify_add_252_input_c_limb_26,
        verify_add_252_input_c_limb_27,
    ] =
        input;

    core::internal::revoke_ap_tracking();

    // Constraint - sub_p_bit is a bit
    let constraint_quotient = ((sub_p_bit_col0 * (sub_p_bit_col0 - qm31_const::<1, 0, 0, 0>())))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let carry_tmp_4afb1_1: QM31 = ((((verify_add_252_input_a_limb_2 + verify_add_252_input_b_limb_2)
        + ((((verify_add_252_input_a_limb_1 + verify_add_252_input_b_limb_1)
            + ((((verify_add_252_input_a_limb_0 + verify_add_252_input_b_limb_0)
                - verify_add_252_input_c_limb_0)
                - sub_p_bit_col0)
                * qm31_const::<4194304, 0, 0, 0>()))
            - verify_add_252_input_c_limb_1)
            * qm31_const::<4194304, 0, 0, 0>()))
        - verify_add_252_input_c_limb_2)
        * qm31_const::<4194304, 0, 0, 0>());

    // Constraint -
    let constraint_quotient = ((carry_tmp_4afb1_1
        * ((carry_tmp_4afb1_1 * carry_tmp_4afb1_1) - qm31_const::<1, 0, 0, 0>())))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let carry_tmp_4afb1_2: QM31 = ((((verify_add_252_input_a_limb_5 + verify_add_252_input_b_limb_5)
        + ((((verify_add_252_input_a_limb_4 + verify_add_252_input_b_limb_4)
            + ((((verify_add_252_input_a_limb_3 + verify_add_252_input_b_limb_3)
                + carry_tmp_4afb1_1)
                - verify_add_252_input_c_limb_3)
                * qm31_const::<4194304, 0, 0, 0>()))
            - verify_add_252_input_c_limb_4)
            * qm31_const::<4194304, 0, 0, 0>()))
        - verify_add_252_input_c_limb_5)
        * qm31_const::<4194304, 0, 0, 0>());

    // Constraint -
    let constraint_quotient = ((carry_tmp_4afb1_2
        * ((carry_tmp_4afb1_2 * carry_tmp_4afb1_2) - qm31_const::<1, 0, 0, 0>())))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let carry_tmp_4afb1_3: QM31 = ((((verify_add_252_input_a_limb_8 + verify_add_252_input_b_limb_8)
        + ((((verify_add_252_input_a_limb_7 + verify_add_252_input_b_limb_7)
            + ((((verify_add_252_input_a_limb_6 + verify_add_252_input_b_limb_6)
                + carry_tmp_4afb1_2)
                - verify_add_252_input_c_limb_6)
                * qm31_const::<4194304, 0, 0, 0>()))
            - verify_add_252_input_c_limb_7)
            * qm31_const::<4194304, 0, 0, 0>()))
        - verify_add_252_input_c_limb_8)
        * qm31_const::<4194304, 0, 0, 0>());

    // Constraint -
    let constraint_quotient = ((carry_tmp_4afb1_3
        * ((carry_tmp_4afb1_3 * carry_tmp_4afb1_3) - qm31_const::<1, 0, 0, 0>())))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let carry_tmp_4afb1_4: QM31 = ((((verify_add_252_input_a_limb_11
        + verify_add_252_input_b_limb_11)
        + ((((verify_add_252_input_a_limb_10 + verify_add_252_input_b_limb_10)
            + ((((verify_add_252_input_a_limb_9 + verify_add_252_input_b_limb_9)
                + carry_tmp_4afb1_3)
                - verify_add_252_input_c_limb_9)
                * qm31_const::<4194304, 0, 0, 0>()))
            - verify_add_252_input_c_limb_10)
            * qm31_const::<4194304, 0, 0, 0>()))
        - verify_add_252_input_c_limb_11)
        * qm31_const::<4194304, 0, 0, 0>());

    // Constraint -
    let constraint_quotient = ((carry_tmp_4afb1_4
        * ((carry_tmp_4afb1_4 * carry_tmp_4afb1_4) - qm31_const::<1, 0, 0, 0>())))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let carry_tmp_4afb1_5: QM31 = ((((verify_add_252_input_a_limb_14
        + verify_add_252_input_b_limb_14)
        + ((((verify_add_252_input_a_limb_13 + verify_add_252_input_b_limb_13)
            + ((((verify_add_252_input_a_limb_12 + verify_add_252_input_b_limb_12)
                + carry_tmp_4afb1_4)
                - verify_add_252_input_c_limb_12)
                * qm31_const::<4194304, 0, 0, 0>()))
            - verify_add_252_input_c_limb_13)
            * qm31_const::<4194304, 0, 0, 0>()))
        - verify_add_252_input_c_limb_14)
        * qm31_const::<4194304, 0, 0, 0>());

    // Constraint -
    let constraint_quotient = ((carry_tmp_4afb1_5
        * ((carry_tmp_4afb1_5 * carry_tmp_4afb1_5) - qm31_const::<1, 0, 0, 0>())))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let carry_tmp_4afb1_6: QM31 = ((((verify_add_252_input_a_limb_17
        + verify_add_252_input_b_limb_17)
        + ((((verify_add_252_input_a_limb_16 + verify_add_252_input_b_limb_16)
            + ((((verify_add_252_input_a_limb_15 + verify_add_252_input_b_limb_15)
                + carry_tmp_4afb1_5)
                - verify_add_252_input_c_limb_15)
                * qm31_const::<4194304, 0, 0, 0>()))
            - verify_add_252_input_c_limb_16)
            * qm31_const::<4194304, 0, 0, 0>()))
        - verify_add_252_input_c_limb_17)
        * qm31_const::<4194304, 0, 0, 0>());

    // Constraint -
    let constraint_quotient = ((carry_tmp_4afb1_6
        * ((carry_tmp_4afb1_6 * carry_tmp_4afb1_6) - qm31_const::<1, 0, 0, 0>())))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let carry_tmp_4afb1_7: QM31 = ((((verify_add_252_input_a_limb_20
        + verify_add_252_input_b_limb_20)
        + ((((verify_add_252_input_a_limb_19 + verify_add_252_input_b_limb_19)
            + ((((verify_add_252_input_a_limb_18 + verify_add_252_input_b_limb_18)
                + carry_tmp_4afb1_6)
                - verify_add_252_input_c_limb_18)
                * qm31_const::<4194304, 0, 0, 0>()))
            - verify_add_252_input_c_limb_19)
            * qm31_const::<4194304, 0, 0, 0>()))
        - verify_add_252_input_c_limb_20)
        * qm31_const::<4194304, 0, 0, 0>());

    // Constraint -
    let constraint_quotient = ((carry_tmp_4afb1_7
        * ((carry_tmp_4afb1_7 * carry_tmp_4afb1_7) - qm31_const::<1, 0, 0, 0>())))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let carry_tmp_4afb1_8: QM31 = ((((verify_add_252_input_a_limb_23
        + verify_add_252_input_b_limb_23)
        + ((((verify_add_252_input_a_limb_22 + verify_add_252_input_b_limb_22)
            + (((((verify_add_252_input_a_limb_21 + verify_add_252_input_b_limb_21)
                + carry_tmp_4afb1_7)
                - verify_add_252_input_c_limb_21)
                - (qm31_const::<136, 0, 0, 0>() * sub_p_bit_col0))
                * qm31_const::<4194304, 0, 0, 0>()))
            - verify_add_252_input_c_limb_22)
            * qm31_const::<4194304, 0, 0, 0>()))
        - verify_add_252_input_c_limb_23)
        * qm31_const::<4194304, 0, 0, 0>());

    // Constraint -
    let constraint_quotient = ((carry_tmp_4afb1_8
        * ((carry_tmp_4afb1_8 * carry_tmp_4afb1_8) - qm31_const::<1, 0, 0, 0>())))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let carry_tmp_4afb1_9: QM31 = ((((verify_add_252_input_a_limb_26
        + verify_add_252_input_b_limb_26)
        + ((((verify_add_252_input_a_limb_25 + verify_add_252_input_b_limb_25)
            + ((((verify_add_252_input_a_limb_24 + verify_add_252_input_b_limb_24)
                + carry_tmp_4afb1_8)
                - verify_add_252_input_c_limb_24)
                * qm31_const::<4194304, 0, 0, 0>()))
            - verify_add_252_input_c_limb_25)
            * qm31_const::<4194304, 0, 0, 0>()))
        - verify_add_252_input_c_limb_26)
        * qm31_const::<4194304, 0, 0, 0>());

    // Constraint -
    let constraint_quotient = ((carry_tmp_4afb1_9
        * ((carry_tmp_4afb1_9 * carry_tmp_4afb1_9) - qm31_const::<1, 0, 0, 0>())))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint -
    let constraint_quotient = (((((verify_add_252_input_a_limb_27 + verify_add_252_input_b_limb_27)
        + carry_tmp_4afb1_9)
        - verify_add_252_input_c_limb_27)
        - (qm31_const::<256, 0, 0, 0>() * sub_p_bit_col0)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    core::internal::revoke_ap_tracking();

    []
}
