// This file was created by the AIR team.

use crate::prelude::*;


pub fn range_check_mem_value_n_28_evaluate(
    input: [QM31; 28],
    common_lookup_elements: @CommonLookupElements,
    ref range_check_9_9_sum_0: QM31,
    ref numerator_0: QM31,
    ref range_check_9_9_b_sum_1: QM31,
    ref numerator_1: QM31,
    ref range_check_9_9_c_sum_2: QM31,
    ref numerator_2: QM31,
    ref range_check_9_9_d_sum_3: QM31,
    ref numerator_3: QM31,
    ref range_check_9_9_e_sum_4: QM31,
    ref numerator_4: QM31,
    ref range_check_9_9_f_sum_5: QM31,
    ref numerator_5: QM31,
    ref range_check_9_9_g_sum_6: QM31,
    ref numerator_6: QM31,
    ref range_check_9_9_h_sum_7: QM31,
    ref numerator_7: QM31,
    ref range_check_9_9_sum_8: QM31,
    ref numerator_8: QM31,
    ref range_check_9_9_b_sum_9: QM31,
    ref numerator_9: QM31,
    ref range_check_9_9_c_sum_10: QM31,
    ref numerator_10: QM31,
    ref range_check_9_9_d_sum_11: QM31,
    ref numerator_11: QM31,
    ref range_check_9_9_e_sum_12: QM31,
    ref numerator_12: QM31,
    ref range_check_9_9_f_sum_13: QM31,
    ref numerator_13: QM31,
    ref sum: QM31,
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

    range_check_9_9_sum_0 = common_lookup_elements
        .combine_qm31(
            [
                qm31_const::<517791011, 0, 0, 0>(), range_check_mem_value_n_28_input_limb_0,
                range_check_mem_value_n_28_input_limb_1,
            ]
                .span(),
        );
    numerator_0 = qm31_const::<1, 0, 0, 0>();

    range_check_9_9_b_sum_1 = common_lookup_elements
        .combine_qm31(
            [
                qm31_const::<1897792095, 0, 0, 0>(), range_check_mem_value_n_28_input_limb_2,
                range_check_mem_value_n_28_input_limb_3,
            ]
                .span(),
        );
    numerator_1 = qm31_const::<1, 0, 0, 0>();

    range_check_9_9_c_sum_2 = common_lookup_elements
        .combine_qm31(
            [
                qm31_const::<1881014476, 0, 0, 0>(), range_check_mem_value_n_28_input_limb_4,
                range_check_mem_value_n_28_input_limb_5,
            ]
                .span(),
        );
    numerator_2 = qm31_const::<1, 0, 0, 0>();

    range_check_9_9_d_sum_3 = common_lookup_elements
        .combine_qm31(
            [
                qm31_const::<1864236857, 0, 0, 0>(), range_check_mem_value_n_28_input_limb_6,
                range_check_mem_value_n_28_input_limb_7,
            ]
                .span(),
        );
    numerator_3 = qm31_const::<1, 0, 0, 0>();

    range_check_9_9_e_sum_4 = common_lookup_elements
        .combine_qm31(
            [
                qm31_const::<1847459238, 0, 0, 0>(), range_check_mem_value_n_28_input_limb_8,
                range_check_mem_value_n_28_input_limb_9,
            ]
                .span(),
        );
    numerator_4 = qm31_const::<1, 0, 0, 0>();

    range_check_9_9_f_sum_5 = common_lookup_elements
        .combine_qm31(
            [
                qm31_const::<1830681619, 0, 0, 0>(), range_check_mem_value_n_28_input_limb_10,
                range_check_mem_value_n_28_input_limb_11,
            ]
                .span(),
        );
    numerator_5 = qm31_const::<1, 0, 0, 0>();

    range_check_9_9_g_sum_6 = common_lookup_elements
        .combine_qm31(
            [
                qm31_const::<1813904000, 0, 0, 0>(), range_check_mem_value_n_28_input_limb_12,
                range_check_mem_value_n_28_input_limb_13,
            ]
                .span(),
        );
    numerator_6 = qm31_const::<1, 0, 0, 0>();

    range_check_9_9_h_sum_7 = common_lookup_elements
        .combine_qm31(
            [
                qm31_const::<2065568285, 0, 0, 0>(), range_check_mem_value_n_28_input_limb_14,
                range_check_mem_value_n_28_input_limb_15,
            ]
                .span(),
        );
    numerator_7 = qm31_const::<1, 0, 0, 0>();

    range_check_9_9_sum_8 = common_lookup_elements
        .combine_qm31(
            [
                qm31_const::<517791011, 0, 0, 0>(), range_check_mem_value_n_28_input_limb_16,
                range_check_mem_value_n_28_input_limb_17,
            ]
                .span(),
        );
    numerator_8 = qm31_const::<1, 0, 0, 0>();

    range_check_9_9_b_sum_9 = common_lookup_elements
        .combine_qm31(
            [
                qm31_const::<1897792095, 0, 0, 0>(), range_check_mem_value_n_28_input_limb_18,
                range_check_mem_value_n_28_input_limb_19,
            ]
                .span(),
        );
    numerator_9 = qm31_const::<1, 0, 0, 0>();

    range_check_9_9_c_sum_10 = common_lookup_elements
        .combine_qm31(
            [
                qm31_const::<1881014476, 0, 0, 0>(), range_check_mem_value_n_28_input_limb_20,
                range_check_mem_value_n_28_input_limb_21,
            ]
                .span(),
        );
    numerator_10 = qm31_const::<1, 0, 0, 0>();

    range_check_9_9_d_sum_11 = common_lookup_elements
        .combine_qm31(
            [
                qm31_const::<1864236857, 0, 0, 0>(), range_check_mem_value_n_28_input_limb_22,
                range_check_mem_value_n_28_input_limb_23,
            ]
                .span(),
        );
    numerator_11 = qm31_const::<1, 0, 0, 0>();

    range_check_9_9_e_sum_12 = common_lookup_elements
        .combine_qm31(
            [
                qm31_const::<1847459238, 0, 0, 0>(), range_check_mem_value_n_28_input_limb_24,
                range_check_mem_value_n_28_input_limb_25,
            ]
                .span(),
        );
    numerator_12 = qm31_const::<1, 0, 0, 0>();

    range_check_9_9_f_sum_13 = common_lookup_elements
        .combine_qm31(
            [
                qm31_const::<1830681619, 0, 0, 0>(), range_check_mem_value_n_28_input_limb_26,
                range_check_mem_value_n_28_input_limb_27,
            ]
                .span(),
        );
    numerator_13 = qm31_const::<1, 0, 0, 0>();

    []
}
