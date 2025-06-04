// Constraints version: 9330aaaf

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


pub fn linear_combination_n_1_coefs_2_evaluate(
    input: [QM31; 10],
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
        linear_combination_n_1_coefs_2_input_limb_0,
        linear_combination_n_1_coefs_2_input_limb_1,
        linear_combination_n_1_coefs_2_input_limb_2,
        linear_combination_n_1_coefs_2_input_limb_3,
        linear_combination_n_1_coefs_2_input_limb_4,
        linear_combination_n_1_coefs_2_input_limb_5,
        linear_combination_n_1_coefs_2_input_limb_6,
        linear_combination_n_1_coefs_2_input_limb_7,
        linear_combination_n_1_coefs_2_input_limb_8,
        linear_combination_n_1_coefs_2_input_limb_9,
    ] =
        input;
    let carry_0_tmp_13179_2: QM31 = ((((qm31_const::<2, 0, 0, 0>()
        * linear_combination_n_1_coefs_2_input_limb_0)
        - combination_limb_0_col0)
        - p_coef_col10)
        * qm31_const::<16, 0, 0, 0>());
    let carry_1_tmp_13179_3: QM31 = (((carry_0_tmp_13179_2
        + (qm31_const::<2, 0, 0, 0>() * linear_combination_n_1_coefs_2_input_limb_1))
        - combination_limb_1_col1)
        * qm31_const::<16, 0, 0, 0>());
    let carry_2_tmp_13179_4: QM31 = (((carry_1_tmp_13179_3
        + (qm31_const::<2, 0, 0, 0>() * linear_combination_n_1_coefs_2_input_limb_2))
        - combination_limb_2_col2)
        * qm31_const::<16, 0, 0, 0>());
    let carry_3_tmp_13179_5: QM31 = (((carry_2_tmp_13179_4
        + (qm31_const::<2, 0, 0, 0>() * linear_combination_n_1_coefs_2_input_limb_3))
        - combination_limb_3_col3)
        * qm31_const::<16, 0, 0, 0>());
    let carry_4_tmp_13179_6: QM31 = (((carry_3_tmp_13179_5
        + (qm31_const::<2, 0, 0, 0>() * linear_combination_n_1_coefs_2_input_limb_4))
        - combination_limb_4_col4)
        * qm31_const::<16, 0, 0, 0>());
    let carry_5_tmp_13179_7: QM31 = (((carry_4_tmp_13179_6
        + (qm31_const::<2, 0, 0, 0>() * linear_combination_n_1_coefs_2_input_limb_5))
        - combination_limb_5_col5)
        * qm31_const::<16, 0, 0, 0>());
    let carry_6_tmp_13179_8: QM31 = (((carry_5_tmp_13179_7
        + (qm31_const::<2, 0, 0, 0>() * linear_combination_n_1_coefs_2_input_limb_6))
        - combination_limb_6_col6)
        * qm31_const::<16, 0, 0, 0>());
    let carry_7_tmp_13179_9: QM31 = ((((carry_6_tmp_13179_8
        + (qm31_const::<2, 0, 0, 0>() * linear_combination_n_1_coefs_2_input_limb_7))
        - combination_limb_7_col7)
        - (p_coef_col10 * qm31_const::<136, 0, 0, 0>()))
        * qm31_const::<16, 0, 0, 0>());
    let carry_8_tmp_13179_10: QM31 = (((carry_7_tmp_13179_9
        + (qm31_const::<2, 0, 0, 0>() * linear_combination_n_1_coefs_2_input_limb_8))
        - combination_limb_8_col8)
        * qm31_const::<16, 0, 0, 0>());

    // Constraint - final limb constraint
    let constraint_quotient = ((((carry_8_tmp_13179_10
        + (qm31_const::<2, 0, 0, 0>() * linear_combination_n_1_coefs_2_input_limb_9))
        - combination_limb_9_col9)
        - (p_coef_col10 * qm31_const::<256, 0, 0, 0>())))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let biased_carry_0_tmp_13179_11: QM31 = ((p_coef_col10 + qm31_const::<1, 0, 0, 0>())
        - qm31_const::<1, 0, 0, 0>());

    // Constraint - carry constraint 0
    let constraint_quotient = ((((biased_carry_0_tmp_13179_11 * biased_carry_0_tmp_13179_11)
        * biased_carry_0_tmp_13179_11)
        - biased_carry_0_tmp_13179_11))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let biased_carry_1_tmp_13179_12: QM31 = ((carry_0_tmp_13179_2 + qm31_const::<1, 0, 0, 0>())
        - qm31_const::<1, 0, 0, 0>());

    // Constraint - carry constraint 1
    let constraint_quotient = ((((biased_carry_1_tmp_13179_12 * biased_carry_1_tmp_13179_12)
        * biased_carry_1_tmp_13179_12)
        - biased_carry_1_tmp_13179_12))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let biased_carry_2_tmp_13179_13: QM31 = ((carry_1_tmp_13179_3 + qm31_const::<1, 0, 0, 0>())
        - qm31_const::<1, 0, 0, 0>());

    // Constraint - carry constraint 2
    let constraint_quotient = ((((biased_carry_2_tmp_13179_13 * biased_carry_2_tmp_13179_13)
        * biased_carry_2_tmp_13179_13)
        - biased_carry_2_tmp_13179_13))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let biased_carry_3_tmp_13179_14: QM31 = ((carry_2_tmp_13179_4 + qm31_const::<1, 0, 0, 0>())
        - qm31_const::<1, 0, 0, 0>());

    // Constraint - carry constraint 3
    let constraint_quotient = ((((biased_carry_3_tmp_13179_14 * biased_carry_3_tmp_13179_14)
        * biased_carry_3_tmp_13179_14)
        - biased_carry_3_tmp_13179_14))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let biased_carry_4_tmp_13179_15: QM31 = ((carry_3_tmp_13179_5 + qm31_const::<1, 0, 0, 0>())
        - qm31_const::<1, 0, 0, 0>());

    // Constraint - carry constraint 4
    let constraint_quotient = ((((biased_carry_4_tmp_13179_15 * biased_carry_4_tmp_13179_15)
        * biased_carry_4_tmp_13179_15)
        - biased_carry_4_tmp_13179_15))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let biased_carry_5_tmp_13179_16: QM31 = ((carry_4_tmp_13179_6 + qm31_const::<1, 0, 0, 0>())
        - qm31_const::<1, 0, 0, 0>());

    // Constraint - carry constraint 5
    let constraint_quotient = ((((biased_carry_5_tmp_13179_16 * biased_carry_5_tmp_13179_16)
        * biased_carry_5_tmp_13179_16)
        - biased_carry_5_tmp_13179_16))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let biased_carry_6_tmp_13179_17: QM31 = ((carry_5_tmp_13179_7 + qm31_const::<1, 0, 0, 0>())
        - qm31_const::<1, 0, 0, 0>());

    // Constraint - carry constraint 6
    let constraint_quotient = ((((biased_carry_6_tmp_13179_17 * biased_carry_6_tmp_13179_17)
        * biased_carry_6_tmp_13179_17)
        - biased_carry_6_tmp_13179_17))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let biased_carry_7_tmp_13179_18: QM31 = ((carry_6_tmp_13179_8 + qm31_const::<1, 0, 0, 0>())
        - qm31_const::<1, 0, 0, 0>());

    // Constraint - carry constraint 7
    let constraint_quotient = ((((biased_carry_7_tmp_13179_18 * biased_carry_7_tmp_13179_18)
        * biased_carry_7_tmp_13179_18)
        - biased_carry_7_tmp_13179_18))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let biased_carry_8_tmp_13179_19: QM31 = ((carry_7_tmp_13179_9 + qm31_const::<1, 0, 0, 0>())
        - qm31_const::<1, 0, 0, 0>());

    // Constraint - carry constraint 8
    let constraint_quotient = ((((biased_carry_8_tmp_13179_19 * biased_carry_8_tmp_13179_19)
        * biased_carry_8_tmp_13179_19)
        - biased_carry_8_tmp_13179_19))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let biased_carry_9_tmp_13179_20: QM31 = ((carry_8_tmp_13179_10 + qm31_const::<1, 0, 0, 0>())
        - qm31_const::<1, 0, 0, 0>());

    // Constraint - carry constraint 9
    let constraint_quotient = ((((biased_carry_9_tmp_13179_20 * biased_carry_9_tmp_13179_20)
        * biased_carry_9_tmp_13179_20)
        - biased_carry_9_tmp_13179_20))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    []
}
