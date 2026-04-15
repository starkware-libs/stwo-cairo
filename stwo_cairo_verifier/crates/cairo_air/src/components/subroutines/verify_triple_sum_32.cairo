// This file was created by the AIR team.

use crate::prelude::*;


pub fn verify_triple_sum_32_evaluate(
    input: [QM31; 8],
    common_lookup_elements: @CommonLookupElements,
    ref sum: QM31,
    random_coeff: QM31,
) -> [QM31; 0] {
    let [
        verify_triple_sum_32_input_limb_0,
        verify_triple_sum_32_input_limb_1,
        verify_triple_sum_32_input_limb_2,
        verify_triple_sum_32_input_limb_3,
        verify_triple_sum_32_input_limb_4,
        verify_triple_sum_32_input_limb_5,
        verify_triple_sum_32_input_limb_6,
        verify_triple_sum_32_input_limb_7,
    ] =
        input;
    let carry_low_tmp_24c23_0: QM31 = ((((verify_triple_sum_32_input_limb_0
        + verify_triple_sum_32_input_limb_2)
        + verify_triple_sum_32_input_limb_4)
        - verify_triple_sum_32_input_limb_6)
        * qm31_const::<32768, 0, 0, 0>());

    // Constraint - carry low is 0 or 1 or 2
    let constraint_quotient = (((carry_low_tmp_24c23_0
        * (carry_low_tmp_24c23_0 - qm31_const::<1, 0, 0, 0>()))
        * (carry_low_tmp_24c23_0 - qm31_const::<2, 0, 0, 0>())));
    sum = sum * random_coeff + constraint_quotient;
    let carry_high_tmp_24c23_1: QM31 = (((((verify_triple_sum_32_input_limb_1
        + verify_triple_sum_32_input_limb_3)
        + verify_triple_sum_32_input_limb_5)
        + carry_low_tmp_24c23_0)
        - verify_triple_sum_32_input_limb_7)
        * qm31_const::<32768, 0, 0, 0>());

    // Constraint - carry high is 0 or 1 or 2
    let constraint_quotient = (((carry_high_tmp_24c23_1
        * (carry_high_tmp_24c23_1 - qm31_const::<1, 0, 0, 0>()))
        * (carry_high_tmp_24c23_1 - qm31_const::<2, 0, 0, 0>())));
    sum = sum * random_coeff + constraint_quotient;

    []
}
