// This file was created by the AIR team.

use crate::prelude::*;


pub fn range_check_mem_value_n_28_evaluate(
    input: [QM31; 28],
    range_check_9_9_lookup_elements: @crate::RangeCheck_9_9Elements,
    range_check_9_9_b_lookup_elements: @crate::RangeCheck_9_9_BElements,
    range_check_9_9_c_lookup_elements: @crate::RangeCheck_9_9_CElements,
    range_check_9_9_d_lookup_elements: @crate::RangeCheck_9_9_DElements,
    range_check_9_9_e_lookup_elements: @crate::RangeCheck_9_9_EElements,
    range_check_9_9_f_lookup_elements: @crate::RangeCheck_9_9_FElements,
    range_check_9_9_g_lookup_elements: @crate::RangeCheck_9_9_GElements,
    range_check_9_9_h_lookup_elements: @crate::RangeCheck_9_9_HElements,
    ref range_check_9_9_sum_0: QM31,
    ref range_check_9_9_b_sum_1: QM31,
    ref range_check_9_9_c_sum_2: QM31,
    ref range_check_9_9_d_sum_3: QM31,
    ref range_check_9_9_e_sum_4: QM31,
    ref range_check_9_9_f_sum_5: QM31,
    ref range_check_9_9_g_sum_6: QM31,
    ref range_check_9_9_h_sum_7: QM31,
    ref range_check_9_9_sum_8: QM31,
    ref range_check_9_9_b_sum_9: QM31,
    ref range_check_9_9_c_sum_10: QM31,
    ref range_check_9_9_d_sum_11: QM31,
    ref range_check_9_9_e_sum_12: QM31,
    ref range_check_9_9_f_sum_13: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 0] {
    let [
        range_check_mem_value_n_28_input_limb_0,
        range_check_mem_value_n_28_input_limb_1,
        range_check_mem_value_n_28_input_limb_2,
        range_check_mem_value_n_28_input_limb_3,
        range_check_mem_value_n_28_input_limb_4,
        range_check_mem_value_n_28_input_limb_5,
        range_check_mem_value_n_28_input_limb_6,
        range_check_mem_value_n_28_input_limb_7,
        range_check_mem_value_n_28_input_limb_8,
        range_check_mem_value_n_28_input_limb_9,
        range_check_mem_value_n_28_input_limb_10,
        range_check_mem_value_n_28_input_limb_11,
        range_check_mem_value_n_28_input_limb_12,
        range_check_mem_value_n_28_input_limb_13,
        range_check_mem_value_n_28_input_limb_14,
        range_check_mem_value_n_28_input_limb_15,
        range_check_mem_value_n_28_input_limb_16,
        range_check_mem_value_n_28_input_limb_17,
        range_check_mem_value_n_28_input_limb_18,
        range_check_mem_value_n_28_input_limb_19,
        range_check_mem_value_n_28_input_limb_20,
        range_check_mem_value_n_28_input_limb_21,
        range_check_mem_value_n_28_input_limb_22,
        range_check_mem_value_n_28_input_limb_23,
        range_check_mem_value_n_28_input_limb_24,
        range_check_mem_value_n_28_input_limb_25,
        range_check_mem_value_n_28_input_limb_26,
        range_check_mem_value_n_28_input_limb_27,
    ] =
        input;

    range_check_9_9_sum_0 = range_check_9_9_lookup_elements
        .combine_qm31(
            [range_check_mem_value_n_28_input_limb_0, range_check_mem_value_n_28_input_limb_1],
        );

    range_check_9_9_b_sum_1 = range_check_9_9_b_lookup_elements
        .combine_qm31(
            [range_check_mem_value_n_28_input_limb_2, range_check_mem_value_n_28_input_limb_3],
        );

    range_check_9_9_c_sum_2 = range_check_9_9_c_lookup_elements
        .combine_qm31(
            [range_check_mem_value_n_28_input_limb_4, range_check_mem_value_n_28_input_limb_5],
        );

    range_check_9_9_d_sum_3 = range_check_9_9_d_lookup_elements
        .combine_qm31(
            [range_check_mem_value_n_28_input_limb_6, range_check_mem_value_n_28_input_limb_7],
        );

    range_check_9_9_e_sum_4 = range_check_9_9_e_lookup_elements
        .combine_qm31(
            [range_check_mem_value_n_28_input_limb_8, range_check_mem_value_n_28_input_limb_9],
        );

    range_check_9_9_f_sum_5 = range_check_9_9_f_lookup_elements
        .combine_qm31(
            [range_check_mem_value_n_28_input_limb_10, range_check_mem_value_n_28_input_limb_11],
        );

    range_check_9_9_g_sum_6 = range_check_9_9_g_lookup_elements
        .combine_qm31(
            [range_check_mem_value_n_28_input_limb_12, range_check_mem_value_n_28_input_limb_13],
        );

    range_check_9_9_h_sum_7 = range_check_9_9_h_lookup_elements
        .combine_qm31(
            [range_check_mem_value_n_28_input_limb_14, range_check_mem_value_n_28_input_limb_15],
        );

    range_check_9_9_sum_8 = range_check_9_9_lookup_elements
        .combine_qm31(
            [range_check_mem_value_n_28_input_limb_16, range_check_mem_value_n_28_input_limb_17],
        );

    range_check_9_9_b_sum_9 = range_check_9_9_b_lookup_elements
        .combine_qm31(
            [range_check_mem_value_n_28_input_limb_18, range_check_mem_value_n_28_input_limb_19],
        );

    range_check_9_9_c_sum_10 = range_check_9_9_c_lookup_elements
        .combine_qm31(
            [range_check_mem_value_n_28_input_limb_20, range_check_mem_value_n_28_input_limb_21],
        );

    range_check_9_9_d_sum_11 = range_check_9_9_d_lookup_elements
        .combine_qm31(
            [range_check_mem_value_n_28_input_limb_22, range_check_mem_value_n_28_input_limb_23],
        );

    range_check_9_9_e_sum_12 = range_check_9_9_e_lookup_elements
        .combine_qm31(
            [range_check_mem_value_n_28_input_limb_24, range_check_mem_value_n_28_input_limb_25],
        );

    range_check_9_9_f_sum_13 = range_check_9_9_f_lookup_elements
        .combine_qm31(
            [range_check_mem_value_n_28_input_limb_26, range_check_mem_value_n_28_input_limb_27],
        );

    []
}
