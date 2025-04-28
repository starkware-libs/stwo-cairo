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


pub fn verify_reduced_252_evaluate(
    input: [QM31; 28],
    ms_limb_is_max_col0: QM31,
    ms_and_mid_limbs_are_max_col1: QM31,
    rc_input_col2: QM31,
    range_check_8_lookup_elements: @crate::RangeCheck_8Elements,
    ref range_check_8_sum_0: QM31,
    ref range_check_8_sum_1: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 0] {
    let [
        verify_reduced_252_input_limb_0,
        verify_reduced_252_input_limb_1,
        verify_reduced_252_input_limb_2,
        verify_reduced_252_input_limb_3,
        verify_reduced_252_input_limb_4,
        verify_reduced_252_input_limb_5,
        verify_reduced_252_input_limb_6,
        verify_reduced_252_input_limb_7,
        verify_reduced_252_input_limb_8,
        verify_reduced_252_input_limb_9,
        verify_reduced_252_input_limb_10,
        verify_reduced_252_input_limb_11,
        verify_reduced_252_input_limb_12,
        verify_reduced_252_input_limb_13,
        verify_reduced_252_input_limb_14,
        verify_reduced_252_input_limb_15,
        verify_reduced_252_input_limb_16,
        verify_reduced_252_input_limb_17,
        verify_reduced_252_input_limb_18,
        verify_reduced_252_input_limb_19,
        verify_reduced_252_input_limb_20,
        verify_reduced_252_input_limb_21,
        verify_reduced_252_input_limb_22,
        verify_reduced_252_input_limb_23,
        verify_reduced_252_input_limb_24,
        verify_reduced_252_input_limb_25,
        verify_reduced_252_input_limb_26,
        verify_reduced_252_input_limb_27,
    ] =
        input;

    // Constraint - ms_max is bit
    let constraint_quotient = ((ms_limb_is_max_col0
        * (qm31_const::<1, 0, 0, 0>() - ms_limb_is_max_col0)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - both_max is bit
    let constraint_quotient = ((ms_and_mid_limbs_are_max_col1
        * (qm31_const::<1, 0, 0, 0>() - ms_and_mid_limbs_are_max_col1)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    range_check_8_sum_0 = range_check_8_lookup_elements
        .combine_qm31([(verify_reduced_252_input_limb_27 - ms_limb_is_max_col0)]);

    // Constraint - If the MS limb is max, high limbs should be 0
    let constraint_quotient = ((ms_limb_is_max_col0 * verify_reduced_252_input_limb_22))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - If the MS limb is max, high limbs should be 0
    let constraint_quotient = ((ms_limb_is_max_col0 * verify_reduced_252_input_limb_23))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - If the MS limb is max, high limbs should be 0
    let constraint_quotient = ((ms_limb_is_max_col0 * verify_reduced_252_input_limb_24))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - If the MS limb is max, high limbs should be 0
    let constraint_quotient = ((ms_limb_is_max_col0 * verify_reduced_252_input_limb_25))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - If the MS limb is max, high limbs should be 0
    let constraint_quotient = ((ms_limb_is_max_col0 * verify_reduced_252_input_limb_26))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - rc_input
    let constraint_quotient = ((rc_input_col2
        - (ms_limb_is_max_col0
            * ((qm31_const::<120, 0, 0, 0>() + verify_reduced_252_input_limb_21)
                - ms_and_mid_limbs_are_max_col1))))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    range_check_8_sum_1 = range_check_8_lookup_elements.combine_qm31([rc_input_col2]);

    // Constraint - If the MS and mid limbs are max, low limbs should be 0
    let constraint_quotient = ((ms_and_mid_limbs_are_max_col1 * verify_reduced_252_input_limb_0))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - If the MS and mid limbs are max, low limbs should be 0
    let constraint_quotient = ((ms_and_mid_limbs_are_max_col1 * verify_reduced_252_input_limb_1))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - If the MS and mid limbs are max, low limbs should be 0
    let constraint_quotient = ((ms_and_mid_limbs_are_max_col1 * verify_reduced_252_input_limb_2))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - If the MS and mid limbs are max, low limbs should be 0
    let constraint_quotient = ((ms_and_mid_limbs_are_max_col1 * verify_reduced_252_input_limb_3))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - If the MS and mid limbs are max, low limbs should be 0
    let constraint_quotient = ((ms_and_mid_limbs_are_max_col1 * verify_reduced_252_input_limb_4))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - If the MS and mid limbs are max, low limbs should be 0
    let constraint_quotient = ((ms_and_mid_limbs_are_max_col1 * verify_reduced_252_input_limb_5))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - If the MS and mid limbs are max, low limbs should be 0
    let constraint_quotient = ((ms_and_mid_limbs_are_max_col1 * verify_reduced_252_input_limb_6))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - If the MS and mid limbs are max, low limbs should be 0
    let constraint_quotient = ((ms_and_mid_limbs_are_max_col1 * verify_reduced_252_input_limb_7))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - If the MS and mid limbs are max, low limbs should be 0
    let constraint_quotient = ((ms_and_mid_limbs_are_max_col1 * verify_reduced_252_input_limb_8))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - If the MS and mid limbs are max, low limbs should be 0
    let constraint_quotient = ((ms_and_mid_limbs_are_max_col1 * verify_reduced_252_input_limb_9))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - If the MS and mid limbs are max, low limbs should be 0
    let constraint_quotient = ((ms_and_mid_limbs_are_max_col1 * verify_reduced_252_input_limb_10))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - If the MS and mid limbs are max, low limbs should be 0
    let constraint_quotient = ((ms_and_mid_limbs_are_max_col1 * verify_reduced_252_input_limb_11))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - If the MS and mid limbs are max, low limbs should be 0
    let constraint_quotient = ((ms_and_mid_limbs_are_max_col1 * verify_reduced_252_input_limb_12))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - If the MS and mid limbs are max, low limbs should be 0
    let constraint_quotient = ((ms_and_mid_limbs_are_max_col1 * verify_reduced_252_input_limb_13))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - If the MS and mid limbs are max, low limbs should be 0
    let constraint_quotient = ((ms_and_mid_limbs_are_max_col1 * verify_reduced_252_input_limb_14))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - If the MS and mid limbs are max, low limbs should be 0
    let constraint_quotient = ((ms_and_mid_limbs_are_max_col1 * verify_reduced_252_input_limb_15))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - If the MS and mid limbs are max, low limbs should be 0
    let constraint_quotient = ((ms_and_mid_limbs_are_max_col1 * verify_reduced_252_input_limb_16))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - If the MS and mid limbs are max, low limbs should be 0
    let constraint_quotient = ((ms_and_mid_limbs_are_max_col1 * verify_reduced_252_input_limb_17))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - If the MS and mid limbs are max, low limbs should be 0
    let constraint_quotient = ((ms_and_mid_limbs_are_max_col1 * verify_reduced_252_input_limb_18))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - If the MS and mid limbs are max, low limbs should be 0
    let constraint_quotient = ((ms_and_mid_limbs_are_max_col1 * verify_reduced_252_input_limb_19))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - If the MS and mid limbs are max, low limbs should be 0
    let constraint_quotient = ((ms_and_mid_limbs_are_max_col1 * verify_reduced_252_input_limb_20))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    []
}
