// This file was created by the AIR team.

use crate::prelude::*;


pub fn mod_words_to_12_bit_array_evaluate(
    input: [QM31; 22],
    limb1b_0_col0: QM31,
    limb2b_0_col1: QM31,
    limb5b_0_col2: QM31,
    limb6b_0_col3: QM31,
    limb9b_0_col4: QM31,
    limb1b_1_col5: QM31,
    limb2b_1_col6: QM31,
    limb5b_1_col7: QM31,
    limb6b_1_col8: QM31,
    limb9b_1_col9: QM31,
    range_check_3_6_6_3_lookup_elements: @crate::RangeCheck_3_6_6_3Elements,
    ref range_check_3_6_6_3_sum_0: QM31,
    ref range_check_3_6_6_3_sum_1: QM31,
    ref range_check_3_6_6_3_sum_2: QM31,
    ref range_check_3_6_6_3_sum_3: QM31,
    ref range_check_3_6_6_3_sum_4: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 16] {
    let [
        mod_words_to_12_bit_array_input_limb_0,
        mod_words_to_12_bit_array_input_limb_1,
        mod_words_to_12_bit_array_input_limb_2,
        mod_words_to_12_bit_array_input_limb_3,
        mod_words_to_12_bit_array_input_limb_4,
        mod_words_to_12_bit_array_input_limb_5,
        mod_words_to_12_bit_array_input_limb_6,
        mod_words_to_12_bit_array_input_limb_7,
        mod_words_to_12_bit_array_input_limb_8,
        mod_words_to_12_bit_array_input_limb_9,
        mod_words_to_12_bit_array_input_limb_10,
        mod_words_to_12_bit_array_input_limb_28,
        mod_words_to_12_bit_array_input_limb_29,
        mod_words_to_12_bit_array_input_limb_30,
        mod_words_to_12_bit_array_input_limb_31,
        mod_words_to_12_bit_array_input_limb_32,
        mod_words_to_12_bit_array_input_limb_33,
        mod_words_to_12_bit_array_input_limb_34,
        mod_words_to_12_bit_array_input_limb_35,
        mod_words_to_12_bit_array_input_limb_36,
        mod_words_to_12_bit_array_input_limb_37,
        mod_words_to_12_bit_array_input_limb_38,
    ] =
        input;
    let limb1a_0_tmp_f4497_1: QM31 = (mod_words_to_12_bit_array_input_limb_1
        - (limb1b_0_col0 * qm31_const::<8, 0, 0, 0>()));
    let limb2a_0_tmp_f4497_3: QM31 = (mod_words_to_12_bit_array_input_limb_2
        - (limb2b_0_col1 * qm31_const::<64, 0, 0, 0>()));

    range_check_3_6_6_3_sum_0 = range_check_3_6_6_3_lookup_elements
        .combine_qm31([limb1a_0_tmp_f4497_1, limb1b_0_col0, limb2a_0_tmp_f4497_3, limb2b_0_col1]);
    let limb5a_0_tmp_f4497_5: QM31 = (mod_words_to_12_bit_array_input_limb_5
        - (limb5b_0_col2 * qm31_const::<8, 0, 0, 0>()));
    let limb6a_0_tmp_f4497_7: QM31 = (mod_words_to_12_bit_array_input_limb_6
        - (limb6b_0_col3 * qm31_const::<64, 0, 0, 0>()));

    range_check_3_6_6_3_sum_1 = range_check_3_6_6_3_lookup_elements
        .combine_qm31([limb5a_0_tmp_f4497_5, limb5b_0_col2, limb6a_0_tmp_f4497_7, limb6b_0_col3]);
    let limb9a_0_tmp_f4497_9: QM31 = (mod_words_to_12_bit_array_input_limb_9
        - (limb9b_0_col4 * qm31_const::<8, 0, 0, 0>()));
    let limb1a_1_tmp_f4497_11: QM31 = (mod_words_to_12_bit_array_input_limb_29
        - (limb1b_1_col5 * qm31_const::<8, 0, 0, 0>()));
    let limb2a_1_tmp_f4497_13: QM31 = (mod_words_to_12_bit_array_input_limb_30
        - (limb2b_1_col6 * qm31_const::<64, 0, 0, 0>()));

    range_check_3_6_6_3_sum_2 = range_check_3_6_6_3_lookup_elements
        .combine_qm31([limb1a_1_tmp_f4497_11, limb1b_1_col5, limb2a_1_tmp_f4497_13, limb2b_1_col6]);
    let limb5a_1_tmp_f4497_15: QM31 = (mod_words_to_12_bit_array_input_limb_33
        - (limb5b_1_col7 * qm31_const::<8, 0, 0, 0>()));
    let limb6a_1_tmp_f4497_17: QM31 = (mod_words_to_12_bit_array_input_limb_34
        - (limb6b_1_col8 * qm31_const::<64, 0, 0, 0>()));

    range_check_3_6_6_3_sum_3 = range_check_3_6_6_3_lookup_elements
        .combine_qm31([limb5a_1_tmp_f4497_15, limb5b_1_col7, limb6a_1_tmp_f4497_17, limb6b_1_col8]);
    let limb9a_1_tmp_f4497_19: QM31 = (mod_words_to_12_bit_array_input_limb_37
        - (limb9b_1_col9 * qm31_const::<8, 0, 0, 0>()));

    range_check_3_6_6_3_sum_4 = range_check_3_6_6_3_lookup_elements
        .combine_qm31([limb9a_0_tmp_f4497_9, limb9b_0_col4, limb9b_1_col9, limb9a_1_tmp_f4497_19]);

    [
        (mod_words_to_12_bit_array_input_limb_0
            + (qm31_const::<512, 0, 0, 0>() * limb1a_0_tmp_f4497_1)),
        (limb1b_0_col0 + (qm31_const::<64, 0, 0, 0>() * limb2a_0_tmp_f4497_3)),
        (limb2b_0_col1 + (qm31_const::<8, 0, 0, 0>() * mod_words_to_12_bit_array_input_limb_3)),
        (mod_words_to_12_bit_array_input_limb_4
            + (qm31_const::<512, 0, 0, 0>() * limb5a_0_tmp_f4497_5)),
        (limb5b_0_col2 + (qm31_const::<64, 0, 0, 0>() * limb6a_0_tmp_f4497_7)),
        (limb6b_0_col3 + (qm31_const::<8, 0, 0, 0>() * mod_words_to_12_bit_array_input_limb_7)),
        (mod_words_to_12_bit_array_input_limb_8
            + (qm31_const::<512, 0, 0, 0>() * limb9a_0_tmp_f4497_9)),
        (limb9b_0_col4 + (qm31_const::<64, 0, 0, 0>() * mod_words_to_12_bit_array_input_limb_10)),
        (mod_words_to_12_bit_array_input_limb_28
            + (qm31_const::<512, 0, 0, 0>() * limb1a_1_tmp_f4497_11)),
        (limb1b_1_col5 + (qm31_const::<64, 0, 0, 0>() * limb2a_1_tmp_f4497_13)),
        (limb2b_1_col6 + (qm31_const::<8, 0, 0, 0>() * mod_words_to_12_bit_array_input_limb_31)),
        (mod_words_to_12_bit_array_input_limb_32
            + (qm31_const::<512, 0, 0, 0>() * limb5a_1_tmp_f4497_15)),
        (limb5b_1_col7 + (qm31_const::<64, 0, 0, 0>() * limb6a_1_tmp_f4497_17)),
        (limb6b_1_col8 + (qm31_const::<8, 0, 0, 0>() * mod_words_to_12_bit_array_input_limb_35)),
        (mod_words_to_12_bit_array_input_limb_36
            + (qm31_const::<512, 0, 0, 0>() * limb9a_1_tmp_f4497_19)),
        (limb9b_1_col9 + (qm31_const::<64, 0, 0, 0>() * mod_words_to_12_bit_array_input_limb_38)),
    ]
}
