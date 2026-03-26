// This file was created by the AIR team.

use crate::components::subroutines::verify_triple_sum_32::verify_triple_sum_32_evaluate;
use crate::prelude::*;


pub fn triple_sum_32_evaluate(
    input: [QM31; 6],
    triple_sum32_res_limb_0_col0: QM31,
    triple_sum32_res_limb_1_col1: QM31,
    common_lookup_elements: @CommonLookupElements,
    ref sum: QM31,
    random_coeff: QM31,
) -> [QM31; 0] {
    let [
        triple_sum_32_input_a_limb_0,
        triple_sum_32_input_a_limb_1,
        triple_sum_32_input_b_limb_0,
        triple_sum_32_input_b_limb_1,
        triple_sum_32_input_c_limb_0,
        triple_sum_32_input_c_limb_1,
    ] =
        input;
    verify_triple_sum_32_evaluate(
        [
            triple_sum_32_input_a_limb_0, triple_sum_32_input_a_limb_1,
            triple_sum_32_input_b_limb_0, triple_sum_32_input_b_limb_1,
            triple_sum_32_input_c_limb_0, triple_sum_32_input_c_limb_1,
            triple_sum32_res_limb_0_col0, triple_sum32_res_limb_1_col1,
        ],
        common_lookup_elements,
        ref sum,
        random_coeff,
    );

    []
}
