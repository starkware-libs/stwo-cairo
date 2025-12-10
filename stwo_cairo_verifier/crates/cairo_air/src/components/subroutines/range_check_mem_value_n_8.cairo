// This file was created by the AIR team.

use crate::prelude::*;


pub fn range_check_mem_value_n_8_evaluate(
    input: [QM31; 8],
    range_check_9_9_lookup_elements: @crate::RangeCheck_9_9Elements,
    range_check_9_9_b_lookup_elements: @crate::RangeCheck_9_9_BElements,
    range_check_9_9_c_lookup_elements: @crate::RangeCheck_9_9_CElements,
    range_check_9_9_d_lookup_elements: @crate::RangeCheck_9_9_DElements,
    ref range_check_9_9_sum_0: QM31,
    ref range_check_9_9_b_sum_1: QM31,
    ref range_check_9_9_c_sum_2: QM31,
    ref range_check_9_9_d_sum_3: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 0] {
    let [
        range_check_mem_value_n_8_input_limb_0,
        range_check_mem_value_n_8_input_limb_1,
        range_check_mem_value_n_8_input_limb_2,
        range_check_mem_value_n_8_input_limb_3,
        range_check_mem_value_n_8_input_limb_4,
        range_check_mem_value_n_8_input_limb_5,
        range_check_mem_value_n_8_input_limb_6,
        range_check_mem_value_n_8_input_limb_7,
    ] =
        input;

    range_check_9_9_sum_0 = range_check_9_9_lookup_elements
        .combine_qm31(
            [range_check_mem_value_n_8_input_limb_0, range_check_mem_value_n_8_input_limb_1],
        );

    range_check_9_9_b_sum_1 = range_check_9_9_b_lookup_elements
        .combine_qm31(
            [range_check_mem_value_n_8_input_limb_2, range_check_mem_value_n_8_input_limb_3],
        );

    range_check_9_9_c_sum_2 = range_check_9_9_c_lookup_elements
        .combine_qm31(
            [range_check_mem_value_n_8_input_limb_4, range_check_mem_value_n_8_input_limb_5],
        );

    range_check_9_9_d_sum_3 = range_check_9_9_d_lookup_elements
        .combine_qm31(
            [range_check_mem_value_n_8_input_limb_6, range_check_mem_value_n_8_input_limb_7],
        );

    []
}
