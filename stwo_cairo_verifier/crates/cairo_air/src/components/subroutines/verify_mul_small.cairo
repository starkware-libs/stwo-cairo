// This file was created by the AIR team.

use crate::prelude::*;


pub fn verify_mul_small_evaluate(
    input: [QM31; 16],
    carry_1_col0: QM31,
    carry_3_col1: QM31,
    carry_5_col2: QM31,
    common_lookup_elements: @CommonLookupElements,
    ref range_check_11_sum_0: QM31,
    ref numerator_0: QM31,
    ref range_check_11_sum_1: QM31,
    ref numerator_1: QM31,
    ref range_check_11_sum_2: QM31,
    ref numerator_2: QM31,
    ref sum: QM31,
    random_coeff: QM31,
) -> [QM31; 0] {
    let [
        verify_mul_small_input_a_limb_0,
        verify_mul_small_input_a_limb_1,
        verify_mul_small_input_a_limb_2,
        verify_mul_small_input_a_limb_3,
        verify_mul_small_input_b_limb_0,
        verify_mul_small_input_b_limb_1,
        verify_mul_small_input_b_limb_2,
        verify_mul_small_input_b_limb_3,
        verify_mul_small_input_c_limb_0,
        verify_mul_small_input_c_limb_1,
        verify_mul_small_input_c_limb_2,
        verify_mul_small_input_c_limb_3,
        verify_mul_small_input_c_limb_4,
        verify_mul_small_input_c_limb_5,
        verify_mul_small_input_c_limb_6,
        verify_mul_small_input_c_limb_7,
    ] =
        input;

    range_check_11_sum_0 = common_lookup_elements
        .combine_qm31([qm31_const::<991608089, 0, 0, 0>(), carry_1_col0].span());
    numerator_0 = qm31_const::<1, 0, 0, 0>();

    // Constraint - carry 1 definition
    let constraint_quotient = (((carry_1_col0 * qm31_const::<262144, 0, 0, 0>())
        - (((verify_mul_small_input_a_limb_0 * verify_mul_small_input_b_limb_0)
            - verify_mul_small_input_c_limb_0)
            + ((((verify_mul_small_input_a_limb_0 * verify_mul_small_input_b_limb_1)
                + (verify_mul_small_input_a_limb_1 * verify_mul_small_input_b_limb_0))
                - verify_mul_small_input_c_limb_1)
                * qm31_const::<512, 0, 0, 0>()))));
    sum = sum * random_coeff + constraint_quotient;

    range_check_11_sum_1 = common_lookup_elements
        .combine_qm31([qm31_const::<991608089, 0, 0, 0>(), carry_3_col1].span());
    numerator_1 = qm31_const::<1, 0, 0, 0>();

    // Constraint - carry 3 definition
    let constraint_quotient = (((carry_3_col1 * qm31_const::<262144, 0, 0, 0>())
        - ((carry_1_col0
            + ((((verify_mul_small_input_a_limb_0 * verify_mul_small_input_b_limb_2)
                + (verify_mul_small_input_a_limb_1 * verify_mul_small_input_b_limb_1))
                + (verify_mul_small_input_a_limb_2 * verify_mul_small_input_b_limb_0))
                - verify_mul_small_input_c_limb_2))
            + ((((((verify_mul_small_input_a_limb_0 * verify_mul_small_input_b_limb_3)
                + (verify_mul_small_input_a_limb_1 * verify_mul_small_input_b_limb_2))
                + (verify_mul_small_input_a_limb_2 * verify_mul_small_input_b_limb_1))
                + (verify_mul_small_input_a_limb_3 * verify_mul_small_input_b_limb_0))
                - verify_mul_small_input_c_limb_3)
                * qm31_const::<512, 0, 0, 0>()))));
    sum = sum * random_coeff + constraint_quotient;

    range_check_11_sum_2 = common_lookup_elements
        .combine_qm31([qm31_const::<991608089, 0, 0, 0>(), carry_5_col2].span());
    numerator_2 = qm31_const::<1, 0, 0, 0>();

    // Constraint - carry 5 definition
    let constraint_quotient = (((carry_5_col2 * qm31_const::<262144, 0, 0, 0>())
        - ((carry_3_col1
            + ((((verify_mul_small_input_a_limb_1 * verify_mul_small_input_b_limb_3)
                + (verify_mul_small_input_a_limb_2 * verify_mul_small_input_b_limb_2))
                + (verify_mul_small_input_a_limb_3 * verify_mul_small_input_b_limb_1))
                - verify_mul_small_input_c_limb_4))
            + ((((verify_mul_small_input_a_limb_2 * verify_mul_small_input_b_limb_3)
                + (verify_mul_small_input_a_limb_3 * verify_mul_small_input_b_limb_2))
                - verify_mul_small_input_c_limb_5)
                * qm31_const::<512, 0, 0, 0>()))));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - final limb constraint
    let constraint_quotient = ((((carry_5_col2
        + (verify_mul_small_input_a_limb_3 * verify_mul_small_input_b_limb_3))
        - (verify_mul_small_input_c_limb_7 * qm31_const::<512, 0, 0, 0>()))
        - verify_mul_small_input_c_limb_6));
    sum = sum * random_coeff + constraint_quotient;

    []
}
