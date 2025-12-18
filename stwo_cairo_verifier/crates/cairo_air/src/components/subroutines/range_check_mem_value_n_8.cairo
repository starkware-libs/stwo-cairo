// This file was created by the AIR team.

use crate::prelude::*;


pub fn range_check_mem_value_n_8_evaluate(
    input: [QM31; 8],
    common_lookup_elements: @CommonLookupElements,
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

    range_check_9_9_sum_0 = common_lookup_elements
        .combine_qm31(
            [
                qm31_const::<517791011, 0, 0, 0>(), range_check_mem_value_n_8_input_limb_0,
                range_check_mem_value_n_8_input_limb_1,
            ]
                .span(),
        );

    range_check_9_9_b_sum_1 = common_lookup_elements
        .combine_qm31(
            [
                qm31_const::<1897792095, 0, 0, 0>(), range_check_mem_value_n_8_input_limb_2,
                range_check_mem_value_n_8_input_limb_3,
            ]
                .span(),
        );

    range_check_9_9_c_sum_2 = common_lookup_elements
        .combine_qm31(
            [
                qm31_const::<1881014476, 0, 0, 0>(), range_check_mem_value_n_8_input_limb_4,
                range_check_mem_value_n_8_input_limb_5,
            ]
                .span(),
        );

    range_check_9_9_d_sum_3 = common_lookup_elements
        .combine_qm31(
            [
                qm31_const::<1864236857, 0, 0, 0>(), range_check_mem_value_n_8_input_limb_6,
                range_check_mem_value_n_8_input_limb_7,
            ]
                .span(),
        );

    []
}
