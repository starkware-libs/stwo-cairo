// This file was created by the AIR team.

use crate::components::subroutines::range_check_mem_value_n_28::range_check_mem_value_n_28_evaluate;
use crate::prelude::*;


pub fn felt_252_unpack_from_27_range_check_output_evaluate(
    input: [QM31; 10],
    unpacked_limb_0_col0: QM31,
    unpacked_limb_1_col1: QM31,
    unpacked_limb_3_col2: QM31,
    unpacked_limb_4_col3: QM31,
    unpacked_limb_6_col4: QM31,
    unpacked_limb_7_col5: QM31,
    unpacked_limb_9_col6: QM31,
    unpacked_limb_10_col7: QM31,
    unpacked_limb_12_col8: QM31,
    unpacked_limb_13_col9: QM31,
    unpacked_limb_15_col10: QM31,
    unpacked_limb_16_col11: QM31,
    unpacked_limb_18_col12: QM31,
    unpacked_limb_19_col13: QM31,
    unpacked_limb_21_col14: QM31,
    unpacked_limb_22_col15: QM31,
    unpacked_limb_24_col16: QM31,
    unpacked_limb_25_col17: QM31,
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
) -> [QM31; 10] {
    let [
        felt_252_unpack_from_27_range_check_output_input_limb_0,
        felt_252_unpack_from_27_range_check_output_input_limb_1,
        felt_252_unpack_from_27_range_check_output_input_limb_2,
        felt_252_unpack_from_27_range_check_output_input_limb_3,
        felt_252_unpack_from_27_range_check_output_input_limb_4,
        felt_252_unpack_from_27_range_check_output_input_limb_5,
        felt_252_unpack_from_27_range_check_output_input_limb_6,
        felt_252_unpack_from_27_range_check_output_input_limb_7,
        felt_252_unpack_from_27_range_check_output_input_limb_8,
        felt_252_unpack_from_27_range_check_output_input_limb_9,
    ] =
        input;
    let unpacked_tmp_4f7f8_1_limb_2: QM31 =
        (((felt_252_unpack_from_27_range_check_output_input_limb_0
        - unpacked_limb_0_col0)
        - (unpacked_limb_1_col1 * qm31_const::<512, 0, 0, 0>()))
        * qm31_const::<8192, 0, 0, 0>());
    let unpacked_tmp_4f7f8_1_limb_5: QM31 =
        (((felt_252_unpack_from_27_range_check_output_input_limb_1
        - unpacked_limb_3_col2)
        - (unpacked_limb_4_col3 * qm31_const::<512, 0, 0, 0>()))
        * qm31_const::<8192, 0, 0, 0>());
    let unpacked_tmp_4f7f8_1_limb_8: QM31 =
        (((felt_252_unpack_from_27_range_check_output_input_limb_2
        - unpacked_limb_6_col4)
        - (unpacked_limb_7_col5 * qm31_const::<512, 0, 0, 0>()))
        * qm31_const::<8192, 0, 0, 0>());
    let unpacked_tmp_4f7f8_1_limb_11: QM31 =
        (((felt_252_unpack_from_27_range_check_output_input_limb_3
        - unpacked_limb_9_col6)
        - (unpacked_limb_10_col7 * qm31_const::<512, 0, 0, 0>()))
        * qm31_const::<8192, 0, 0, 0>());
    let unpacked_tmp_4f7f8_1_limb_14: QM31 =
        (((felt_252_unpack_from_27_range_check_output_input_limb_4
        - unpacked_limb_12_col8)
        - (unpacked_limb_13_col9 * qm31_const::<512, 0, 0, 0>()))
        * qm31_const::<8192, 0, 0, 0>());
    let unpacked_tmp_4f7f8_1_limb_17: QM31 =
        (((felt_252_unpack_from_27_range_check_output_input_limb_5
        - unpacked_limb_15_col10)
        - (unpacked_limb_16_col11 * qm31_const::<512, 0, 0, 0>()))
        * qm31_const::<8192, 0, 0, 0>());
    let unpacked_tmp_4f7f8_1_limb_20: QM31 =
        (((felt_252_unpack_from_27_range_check_output_input_limb_6
        - unpacked_limb_18_col12)
        - (unpacked_limb_19_col13 * qm31_const::<512, 0, 0, 0>()))
        * qm31_const::<8192, 0, 0, 0>());
    let unpacked_tmp_4f7f8_1_limb_23: QM31 =
        (((felt_252_unpack_from_27_range_check_output_input_limb_7
        - unpacked_limb_21_col14)
        - (unpacked_limb_22_col15 * qm31_const::<512, 0, 0, 0>()))
        * qm31_const::<8192, 0, 0, 0>());
    let unpacked_tmp_4f7f8_1_limb_26: QM31 =
        (((felt_252_unpack_from_27_range_check_output_input_limb_8
        - unpacked_limb_24_col16)
        - (unpacked_limb_25_col17 * qm31_const::<512, 0, 0, 0>()))
        * qm31_const::<8192, 0, 0, 0>());
    let unpacked_tmp_4f7f8_1_limb_27: QM31 =
        felt_252_unpack_from_27_range_check_output_input_limb_9;
    range_check_mem_value_n_28_evaluate(
        [
            unpacked_limb_0_col0, unpacked_limb_1_col1, unpacked_tmp_4f7f8_1_limb_2,
            unpacked_limb_3_col2, unpacked_limb_4_col3, unpacked_tmp_4f7f8_1_limb_5,
            unpacked_limb_6_col4, unpacked_limb_7_col5, unpacked_tmp_4f7f8_1_limb_8,
            unpacked_limb_9_col6, unpacked_limb_10_col7, unpacked_tmp_4f7f8_1_limb_11,
            unpacked_limb_12_col8, unpacked_limb_13_col9, unpacked_tmp_4f7f8_1_limb_14,
            unpacked_limb_15_col10, unpacked_limb_16_col11, unpacked_tmp_4f7f8_1_limb_17,
            unpacked_limb_18_col12, unpacked_limb_19_col13, unpacked_tmp_4f7f8_1_limb_20,
            unpacked_limb_21_col14, unpacked_limb_22_col15, unpacked_tmp_4f7f8_1_limb_23,
            unpacked_limb_24_col16, unpacked_limb_25_col17, unpacked_tmp_4f7f8_1_limb_26,
            unpacked_tmp_4f7f8_1_limb_27,
        ],
        range_check_9_9_lookup_elements,
        range_check_9_9_b_lookup_elements,
        range_check_9_9_c_lookup_elements,
        range_check_9_9_d_lookup_elements,
        range_check_9_9_e_lookup_elements,
        range_check_9_9_f_lookup_elements,
        range_check_9_9_g_lookup_elements,
        range_check_9_9_h_lookup_elements,
        ref range_check_9_9_sum_0,
        ref range_check_9_9_b_sum_1,
        ref range_check_9_9_c_sum_2,
        ref range_check_9_9_d_sum_3,
        ref range_check_9_9_e_sum_4,
        ref range_check_9_9_f_sum_5,
        ref range_check_9_9_g_sum_6,
        ref range_check_9_9_h_sum_7,
        ref range_check_9_9_sum_8,
        ref range_check_9_9_b_sum_9,
        ref range_check_9_9_c_sum_10,
        ref range_check_9_9_d_sum_11,
        ref range_check_9_9_e_sum_12,
        ref range_check_9_9_f_sum_13,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    [
        unpacked_tmp_4f7f8_1_limb_2, unpacked_tmp_4f7f8_1_limb_5, unpacked_tmp_4f7f8_1_limb_8,
        unpacked_tmp_4f7f8_1_limb_11, unpacked_tmp_4f7f8_1_limb_14, unpacked_tmp_4f7f8_1_limb_17,
        unpacked_tmp_4f7f8_1_limb_20, unpacked_tmp_4f7f8_1_limb_23, unpacked_tmp_4f7f8_1_limb_26,
        unpacked_tmp_4f7f8_1_limb_27,
    ]
}
