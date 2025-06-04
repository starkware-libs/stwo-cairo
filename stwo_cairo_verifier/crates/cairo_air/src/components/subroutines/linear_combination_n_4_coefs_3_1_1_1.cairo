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


pub fn linear_combination_n_4_coefs_3_1_1_1_evaluate(
    input: [QM31; 40],
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
    range_check_3_3_3_3_3_lookup_elements: @crate::RangeCheck_3_3_3_3_3Elements,
    ref range_check_3_3_3_3_3_sum_0: QM31,
    ref range_check_3_3_3_3_3_sum_1: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 0] {
    let [
        linear_combination_n_4_coefs_3_1_1_1_input_limb_0,
        linear_combination_n_4_coefs_3_1_1_1_input_limb_1,
        linear_combination_n_4_coefs_3_1_1_1_input_limb_2,
        linear_combination_n_4_coefs_3_1_1_1_input_limb_3,
        linear_combination_n_4_coefs_3_1_1_1_input_limb_4,
        linear_combination_n_4_coefs_3_1_1_1_input_limb_5,
        linear_combination_n_4_coefs_3_1_1_1_input_limb_6,
        linear_combination_n_4_coefs_3_1_1_1_input_limb_7,
        linear_combination_n_4_coefs_3_1_1_1_input_limb_8,
        linear_combination_n_4_coefs_3_1_1_1_input_limb_9,
        linear_combination_n_4_coefs_3_1_1_1_input_limb_10,
        linear_combination_n_4_coefs_3_1_1_1_input_limb_11,
        linear_combination_n_4_coefs_3_1_1_1_input_limb_12,
        linear_combination_n_4_coefs_3_1_1_1_input_limb_13,
        linear_combination_n_4_coefs_3_1_1_1_input_limb_14,
        linear_combination_n_4_coefs_3_1_1_1_input_limb_15,
        linear_combination_n_4_coefs_3_1_1_1_input_limb_16,
        linear_combination_n_4_coefs_3_1_1_1_input_limb_17,
        linear_combination_n_4_coefs_3_1_1_1_input_limb_18,
        linear_combination_n_4_coefs_3_1_1_1_input_limb_19,
        linear_combination_n_4_coefs_3_1_1_1_input_limb_20,
        linear_combination_n_4_coefs_3_1_1_1_input_limb_21,
        linear_combination_n_4_coefs_3_1_1_1_input_limb_22,
        linear_combination_n_4_coefs_3_1_1_1_input_limb_23,
        linear_combination_n_4_coefs_3_1_1_1_input_limb_24,
        linear_combination_n_4_coefs_3_1_1_1_input_limb_25,
        linear_combination_n_4_coefs_3_1_1_1_input_limb_26,
        linear_combination_n_4_coefs_3_1_1_1_input_limb_27,
        linear_combination_n_4_coefs_3_1_1_1_input_limb_28,
        linear_combination_n_4_coefs_3_1_1_1_input_limb_29,
        linear_combination_n_4_coefs_3_1_1_1_input_limb_30,
        linear_combination_n_4_coefs_3_1_1_1_input_limb_31,
        linear_combination_n_4_coefs_3_1_1_1_input_limb_32,
        linear_combination_n_4_coefs_3_1_1_1_input_limb_33,
        linear_combination_n_4_coefs_3_1_1_1_input_limb_34,
        linear_combination_n_4_coefs_3_1_1_1_input_limb_35,
        linear_combination_n_4_coefs_3_1_1_1_input_limb_36,
        linear_combination_n_4_coefs_3_1_1_1_input_limb_37,
        linear_combination_n_4_coefs_3_1_1_1_input_limb_38,
        linear_combination_n_4_coefs_3_1_1_1_input_limb_39,
    ] =
        input;
    let carry_0_tmp_f4f81_2: QM31 = (((((((qm31_const::<3, 0, 0, 0>()
        * linear_combination_n_4_coefs_3_1_1_1_input_limb_0)
        + linear_combination_n_4_coefs_3_1_1_1_input_limb_10)
        + linear_combination_n_4_coefs_3_1_1_1_input_limb_20)
        + linear_combination_n_4_coefs_3_1_1_1_input_limb_30)
        - combination_limb_0_col0)
        - p_coef_col10)
        * qm31_const::<16, 0, 0, 0>());
    let carry_1_tmp_f4f81_3: QM31 = ((((((carry_0_tmp_f4f81_2
        + (qm31_const::<3, 0, 0, 0>() * linear_combination_n_4_coefs_3_1_1_1_input_limb_1))
        + linear_combination_n_4_coefs_3_1_1_1_input_limb_11)
        + linear_combination_n_4_coefs_3_1_1_1_input_limb_21)
        + linear_combination_n_4_coefs_3_1_1_1_input_limb_31)
        - combination_limb_1_col1)
        * qm31_const::<16, 0, 0, 0>());
    let carry_2_tmp_f4f81_4: QM31 = ((((((carry_1_tmp_f4f81_3
        + (qm31_const::<3, 0, 0, 0>() * linear_combination_n_4_coefs_3_1_1_1_input_limb_2))
        + linear_combination_n_4_coefs_3_1_1_1_input_limb_12)
        + linear_combination_n_4_coefs_3_1_1_1_input_limb_22)
        + linear_combination_n_4_coefs_3_1_1_1_input_limb_32)
        - combination_limb_2_col2)
        * qm31_const::<16, 0, 0, 0>());
    let carry_3_tmp_f4f81_5: QM31 = ((((((carry_2_tmp_f4f81_4
        + (qm31_const::<3, 0, 0, 0>() * linear_combination_n_4_coefs_3_1_1_1_input_limb_3))
        + linear_combination_n_4_coefs_3_1_1_1_input_limb_13)
        + linear_combination_n_4_coefs_3_1_1_1_input_limb_23)
        + linear_combination_n_4_coefs_3_1_1_1_input_limb_33)
        - combination_limb_3_col3)
        * qm31_const::<16, 0, 0, 0>());
    let carry_4_tmp_f4f81_6: QM31 = ((((((carry_3_tmp_f4f81_5
        + (qm31_const::<3, 0, 0, 0>() * linear_combination_n_4_coefs_3_1_1_1_input_limb_4))
        + linear_combination_n_4_coefs_3_1_1_1_input_limb_14)
        + linear_combination_n_4_coefs_3_1_1_1_input_limb_24)
        + linear_combination_n_4_coefs_3_1_1_1_input_limb_34)
        - combination_limb_4_col4)
        * qm31_const::<16, 0, 0, 0>());
    let carry_5_tmp_f4f81_7: QM31 = ((((((carry_4_tmp_f4f81_6
        + (qm31_const::<3, 0, 0, 0>() * linear_combination_n_4_coefs_3_1_1_1_input_limb_5))
        + linear_combination_n_4_coefs_3_1_1_1_input_limb_15)
        + linear_combination_n_4_coefs_3_1_1_1_input_limb_25)
        + linear_combination_n_4_coefs_3_1_1_1_input_limb_35)
        - combination_limb_5_col5)
        * qm31_const::<16, 0, 0, 0>());
    let carry_6_tmp_f4f81_8: QM31 = ((((((carry_5_tmp_f4f81_7
        + (qm31_const::<3, 0, 0, 0>() * linear_combination_n_4_coefs_3_1_1_1_input_limb_6))
        + linear_combination_n_4_coefs_3_1_1_1_input_limb_16)
        + linear_combination_n_4_coefs_3_1_1_1_input_limb_26)
        + linear_combination_n_4_coefs_3_1_1_1_input_limb_36)
        - combination_limb_6_col6)
        * qm31_const::<16, 0, 0, 0>());
    let carry_7_tmp_f4f81_9: QM31 = (((((((carry_6_tmp_f4f81_8
        + (qm31_const::<3, 0, 0, 0>() * linear_combination_n_4_coefs_3_1_1_1_input_limb_7))
        + linear_combination_n_4_coefs_3_1_1_1_input_limb_17)
        + linear_combination_n_4_coefs_3_1_1_1_input_limb_27)
        + linear_combination_n_4_coefs_3_1_1_1_input_limb_37)
        - combination_limb_7_col7)
        - (p_coef_col10 * qm31_const::<136, 0, 0, 0>()))
        * qm31_const::<16, 0, 0, 0>());
    let carry_8_tmp_f4f81_10: QM31 = ((((((carry_7_tmp_f4f81_9
        + (qm31_const::<3, 0, 0, 0>() * linear_combination_n_4_coefs_3_1_1_1_input_limb_8))
        + linear_combination_n_4_coefs_3_1_1_1_input_limb_18)
        + linear_combination_n_4_coefs_3_1_1_1_input_limb_28)
        + linear_combination_n_4_coefs_3_1_1_1_input_limb_38)
        - combination_limb_8_col8)
        * qm31_const::<16, 0, 0, 0>());

    // Constraint - final limb constraint
    let constraint_quotient = (((((((carry_8_tmp_f4f81_10
        + (qm31_const::<3, 0, 0, 0>() * linear_combination_n_4_coefs_3_1_1_1_input_limb_9))
        + linear_combination_n_4_coefs_3_1_1_1_input_limb_19)
        + linear_combination_n_4_coefs_3_1_1_1_input_limb_29)
        + linear_combination_n_4_coefs_3_1_1_1_input_limb_39)
        - combination_limb_9_col9)
        - (p_coef_col10 * qm31_const::<256, 0, 0, 0>())))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    range_check_3_3_3_3_3_sum_0 = range_check_3_3_3_3_3_lookup_elements
        .combine_qm31(
            [
                (p_coef_col10 + qm31_const::<1, 0, 0, 0>()),
                (carry_0_tmp_f4f81_2 + qm31_const::<1, 0, 0, 0>()),
                (carry_1_tmp_f4f81_3 + qm31_const::<1, 0, 0, 0>()),
                (carry_2_tmp_f4f81_4 + qm31_const::<1, 0, 0, 0>()),
                (carry_3_tmp_f4f81_5 + qm31_const::<1, 0, 0, 0>()),
            ],
        );

    range_check_3_3_3_3_3_sum_1 = range_check_3_3_3_3_3_lookup_elements
        .combine_qm31(
            [
                (carry_4_tmp_f4f81_6 + qm31_const::<1, 0, 0, 0>()),
                (carry_5_tmp_f4f81_7 + qm31_const::<1, 0, 0, 0>()),
                (carry_6_tmp_f4f81_8 + qm31_const::<1, 0, 0, 0>()),
                (carry_7_tmp_f4f81_9 + qm31_const::<1, 0, 0, 0>()),
                (carry_8_tmp_f4f81_10 + qm31_const::<1, 0, 0, 0>()),
            ],
        );

    []
}
