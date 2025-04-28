// Constraints version: cab09e9c

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
use crate::components::CairoComponent;
use crate::utils::U32Impl;


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

    // Constraint - sub_p_bit is a bit
    let constraint_quotient = ((sub_p_bit_col0 * (sub_p_bit_col0 - qm31_const::<1, 0, 0, 0>())))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let carry_tmp_4afb1_1: QM31 = ((((verify_add_252_input_a_limb_0 + verify_add_252_input_b_limb_0)
        - verify_add_252_input_c_limb_0)
        - sub_p_bit_col0)
        * qm31_const::<4194304, 0, 0, 0>());

    // Constraint -
    let constraint_quotient = ((carry_tmp_4afb1_1
        * ((carry_tmp_4afb1_1 * carry_tmp_4afb1_1) - qm31_const::<1, 0, 0, 0>())))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let carry_tmp_4afb1_2: QM31 = ((((verify_add_252_input_a_limb_1 + verify_add_252_input_b_limb_1)
        + carry_tmp_4afb1_1)
        - verify_add_252_input_c_limb_1)
        * qm31_const::<4194304, 0, 0, 0>());

    // Constraint -
    let constraint_quotient = ((carry_tmp_4afb1_2
        * ((carry_tmp_4afb1_2 * carry_tmp_4afb1_2) - qm31_const::<1, 0, 0, 0>())))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let carry_tmp_4afb1_3: QM31 = ((((verify_add_252_input_a_limb_2 + verify_add_252_input_b_limb_2)
        + carry_tmp_4afb1_2)
        - verify_add_252_input_c_limb_2)
        * qm31_const::<4194304, 0, 0, 0>());

    // Constraint -
    let constraint_quotient = ((carry_tmp_4afb1_3
        * ((carry_tmp_4afb1_3 * carry_tmp_4afb1_3) - qm31_const::<1, 0, 0, 0>())))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let carry_tmp_4afb1_4: QM31 = ((((verify_add_252_input_a_limb_3 + verify_add_252_input_b_limb_3)
        + carry_tmp_4afb1_3)
        - verify_add_252_input_c_limb_3)
        * qm31_const::<4194304, 0, 0, 0>());

    // Constraint -
    let constraint_quotient = ((carry_tmp_4afb1_4
        * ((carry_tmp_4afb1_4 * carry_tmp_4afb1_4) - qm31_const::<1, 0, 0, 0>())))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let carry_tmp_4afb1_5: QM31 = ((((verify_add_252_input_a_limb_4 + verify_add_252_input_b_limb_4)
        + carry_tmp_4afb1_4)
        - verify_add_252_input_c_limb_4)
        * qm31_const::<4194304, 0, 0, 0>());

    // Constraint -
    let constraint_quotient = ((carry_tmp_4afb1_5
        * ((carry_tmp_4afb1_5 * carry_tmp_4afb1_5) - qm31_const::<1, 0, 0, 0>())))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let carry_tmp_4afb1_6: QM31 = ((((verify_add_252_input_a_limb_5 + verify_add_252_input_b_limb_5)
        + carry_tmp_4afb1_5)
        - verify_add_252_input_c_limb_5)
        * qm31_const::<4194304, 0, 0, 0>());

    // Constraint -
    let constraint_quotient = ((carry_tmp_4afb1_6
        * ((carry_tmp_4afb1_6 * carry_tmp_4afb1_6) - qm31_const::<1, 0, 0, 0>())))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let carry_tmp_4afb1_7: QM31 = ((((verify_add_252_input_a_limb_6 + verify_add_252_input_b_limb_6)
        + carry_tmp_4afb1_6)
        - verify_add_252_input_c_limb_6)
        * qm31_const::<4194304, 0, 0, 0>());

    // Constraint -
    let constraint_quotient = ((carry_tmp_4afb1_7
        * ((carry_tmp_4afb1_7 * carry_tmp_4afb1_7) - qm31_const::<1, 0, 0, 0>())))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let carry_tmp_4afb1_8: QM31 = ((((verify_add_252_input_a_limb_7 + verify_add_252_input_b_limb_7)
        + carry_tmp_4afb1_7)
        - verify_add_252_input_c_limb_7)
        * qm31_const::<4194304, 0, 0, 0>());

    // Constraint -
    let constraint_quotient = ((carry_tmp_4afb1_8
        * ((carry_tmp_4afb1_8 * carry_tmp_4afb1_8) - qm31_const::<1, 0, 0, 0>())))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let carry_tmp_4afb1_9: QM31 = ((((verify_add_252_input_a_limb_8 + verify_add_252_input_b_limb_8)
        + carry_tmp_4afb1_8)
        - verify_add_252_input_c_limb_8)
        * qm31_const::<4194304, 0, 0, 0>());

    // Constraint -
    let constraint_quotient = ((carry_tmp_4afb1_9
        * ((carry_tmp_4afb1_9 * carry_tmp_4afb1_9) - qm31_const::<1, 0, 0, 0>())))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let carry_tmp_4afb1_10: QM31 = ((((verify_add_252_input_a_limb_9
        + verify_add_252_input_b_limb_9)
        + carry_tmp_4afb1_9)
        - verify_add_252_input_c_limb_9)
        * qm31_const::<4194304, 0, 0, 0>());

    // Constraint -
    let constraint_quotient = ((carry_tmp_4afb1_10
        * ((carry_tmp_4afb1_10 * carry_tmp_4afb1_10) - qm31_const::<1, 0, 0, 0>())))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let carry_tmp_4afb1_11: QM31 = ((((verify_add_252_input_a_limb_10
        + verify_add_252_input_b_limb_10)
        + carry_tmp_4afb1_10)
        - verify_add_252_input_c_limb_10)
        * qm31_const::<4194304, 0, 0, 0>());

    // Constraint -
    let constraint_quotient = ((carry_tmp_4afb1_11
        * ((carry_tmp_4afb1_11 * carry_tmp_4afb1_11) - qm31_const::<1, 0, 0, 0>())))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let carry_tmp_4afb1_12: QM31 = ((((verify_add_252_input_a_limb_11
        + verify_add_252_input_b_limb_11)
        + carry_tmp_4afb1_11)
        - verify_add_252_input_c_limb_11)
        * qm31_const::<4194304, 0, 0, 0>());

    // Constraint -
    let constraint_quotient = ((carry_tmp_4afb1_12
        * ((carry_tmp_4afb1_12 * carry_tmp_4afb1_12) - qm31_const::<1, 0, 0, 0>())))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let carry_tmp_4afb1_13: QM31 = ((((verify_add_252_input_a_limb_12
        + verify_add_252_input_b_limb_12)
        + carry_tmp_4afb1_12)
        - verify_add_252_input_c_limb_12)
        * qm31_const::<4194304, 0, 0, 0>());

    // Constraint -
    let constraint_quotient = ((carry_tmp_4afb1_13
        * ((carry_tmp_4afb1_13 * carry_tmp_4afb1_13) - qm31_const::<1, 0, 0, 0>())))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let carry_tmp_4afb1_14: QM31 = ((((verify_add_252_input_a_limb_13
        + verify_add_252_input_b_limb_13)
        + carry_tmp_4afb1_13)
        - verify_add_252_input_c_limb_13)
        * qm31_const::<4194304, 0, 0, 0>());

    // Constraint -
    let constraint_quotient = ((carry_tmp_4afb1_14
        * ((carry_tmp_4afb1_14 * carry_tmp_4afb1_14) - qm31_const::<1, 0, 0, 0>())))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let carry_tmp_4afb1_15: QM31 = ((((verify_add_252_input_a_limb_14
        + verify_add_252_input_b_limb_14)
        + carry_tmp_4afb1_14)
        - verify_add_252_input_c_limb_14)
        * qm31_const::<4194304, 0, 0, 0>());

    // Constraint -
    let constraint_quotient = ((carry_tmp_4afb1_15
        * ((carry_tmp_4afb1_15 * carry_tmp_4afb1_15) - qm31_const::<1, 0, 0, 0>())))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let carry_tmp_4afb1_16: QM31 = ((((verify_add_252_input_a_limb_15
        + verify_add_252_input_b_limb_15)
        + carry_tmp_4afb1_15)
        - verify_add_252_input_c_limb_15)
        * qm31_const::<4194304, 0, 0, 0>());

    // Constraint -
    let constraint_quotient = ((carry_tmp_4afb1_16
        * ((carry_tmp_4afb1_16 * carry_tmp_4afb1_16) - qm31_const::<1, 0, 0, 0>())))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let carry_tmp_4afb1_17: QM31 = ((((verify_add_252_input_a_limb_16
        + verify_add_252_input_b_limb_16)
        + carry_tmp_4afb1_16)
        - verify_add_252_input_c_limb_16)
        * qm31_const::<4194304, 0, 0, 0>());

    // Constraint -
    let constraint_quotient = ((carry_tmp_4afb1_17
        * ((carry_tmp_4afb1_17 * carry_tmp_4afb1_17) - qm31_const::<1, 0, 0, 0>())))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let carry_tmp_4afb1_18: QM31 = ((((verify_add_252_input_a_limb_17
        + verify_add_252_input_b_limb_17)
        + carry_tmp_4afb1_17)
        - verify_add_252_input_c_limb_17)
        * qm31_const::<4194304, 0, 0, 0>());

    // Constraint -
    let constraint_quotient = ((carry_tmp_4afb1_18
        * ((carry_tmp_4afb1_18 * carry_tmp_4afb1_18) - qm31_const::<1, 0, 0, 0>())))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let carry_tmp_4afb1_19: QM31 = ((((verify_add_252_input_a_limb_18
        + verify_add_252_input_b_limb_18)
        + carry_tmp_4afb1_18)
        - verify_add_252_input_c_limb_18)
        * qm31_const::<4194304, 0, 0, 0>());

    // Constraint -
    let constraint_quotient = ((carry_tmp_4afb1_19
        * ((carry_tmp_4afb1_19 * carry_tmp_4afb1_19) - qm31_const::<1, 0, 0, 0>())))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let carry_tmp_4afb1_20: QM31 = ((((verify_add_252_input_a_limb_19
        + verify_add_252_input_b_limb_19)
        + carry_tmp_4afb1_19)
        - verify_add_252_input_c_limb_19)
        * qm31_const::<4194304, 0, 0, 0>());

    // Constraint -
    let constraint_quotient = ((carry_tmp_4afb1_20
        * ((carry_tmp_4afb1_20 * carry_tmp_4afb1_20) - qm31_const::<1, 0, 0, 0>())))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let carry_tmp_4afb1_21: QM31 = ((((verify_add_252_input_a_limb_20
        + verify_add_252_input_b_limb_20)
        + carry_tmp_4afb1_20)
        - verify_add_252_input_c_limb_20)
        * qm31_const::<4194304, 0, 0, 0>());

    // Constraint -
    let constraint_quotient = ((carry_tmp_4afb1_21
        * ((carry_tmp_4afb1_21 * carry_tmp_4afb1_21) - qm31_const::<1, 0, 0, 0>())))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let carry_tmp_4afb1_22: QM31 = (((((verify_add_252_input_a_limb_21
        + verify_add_252_input_b_limb_21)
        + carry_tmp_4afb1_21)
        - verify_add_252_input_c_limb_21)
        - (qm31_const::<136, 0, 0, 0>() * sub_p_bit_col0))
        * qm31_const::<4194304, 0, 0, 0>());

    // Constraint -
    let constraint_quotient = ((carry_tmp_4afb1_22
        * ((carry_tmp_4afb1_22 * carry_tmp_4afb1_22) - qm31_const::<1, 0, 0, 0>())))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let carry_tmp_4afb1_23: QM31 = ((((verify_add_252_input_a_limb_22
        + verify_add_252_input_b_limb_22)
        + carry_tmp_4afb1_22)
        - verify_add_252_input_c_limb_22)
        * qm31_const::<4194304, 0, 0, 0>());

    // Constraint -
    let constraint_quotient = ((carry_tmp_4afb1_23
        * ((carry_tmp_4afb1_23 * carry_tmp_4afb1_23) - qm31_const::<1, 0, 0, 0>())))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let carry_tmp_4afb1_24: QM31 = ((((verify_add_252_input_a_limb_23
        + verify_add_252_input_b_limb_23)
        + carry_tmp_4afb1_23)
        - verify_add_252_input_c_limb_23)
        * qm31_const::<4194304, 0, 0, 0>());

    // Constraint -
    let constraint_quotient = ((carry_tmp_4afb1_24
        * ((carry_tmp_4afb1_24 * carry_tmp_4afb1_24) - qm31_const::<1, 0, 0, 0>())))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let carry_tmp_4afb1_25: QM31 = ((((verify_add_252_input_a_limb_24
        + verify_add_252_input_b_limb_24)
        + carry_tmp_4afb1_24)
        - verify_add_252_input_c_limb_24)
        * qm31_const::<4194304, 0, 0, 0>());

    // Constraint -
    let constraint_quotient = ((carry_tmp_4afb1_25
        * ((carry_tmp_4afb1_25 * carry_tmp_4afb1_25) - qm31_const::<1, 0, 0, 0>())))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let carry_tmp_4afb1_26: QM31 = ((((verify_add_252_input_a_limb_25
        + verify_add_252_input_b_limb_25)
        + carry_tmp_4afb1_25)
        - verify_add_252_input_c_limb_25)
        * qm31_const::<4194304, 0, 0, 0>());

    // Constraint -
    let constraint_quotient = ((carry_tmp_4afb1_26
        * ((carry_tmp_4afb1_26 * carry_tmp_4afb1_26) - qm31_const::<1, 0, 0, 0>())))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let carry_tmp_4afb1_27: QM31 = ((((verify_add_252_input_a_limb_26
        + verify_add_252_input_b_limb_26)
        + carry_tmp_4afb1_26)
        - verify_add_252_input_c_limb_26)
        * qm31_const::<4194304, 0, 0, 0>());

    // Constraint -
    let constraint_quotient = ((carry_tmp_4afb1_27
        * ((carry_tmp_4afb1_27 * carry_tmp_4afb1_27) - qm31_const::<1, 0, 0, 0>())))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint -
    let constraint_quotient = (((((verify_add_252_input_a_limb_27 + verify_add_252_input_b_limb_27)
        + carry_tmp_4afb1_27)
        - verify_add_252_input_c_limb_27)
        - (qm31_const::<256, 0, 0, 0>() * sub_p_bit_col0)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    []
}
