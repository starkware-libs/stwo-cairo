// This file was created by the AIR team.

use crate::components::subroutines::double_karatsuba_1454b::double_karatsuba_1454b_evaluate;
use crate::prelude::*;


pub fn verify_mul_252_evaluate(
    input: [QM31; 84],
    k_col0: QM31,
    carry_0_col1: QM31,
    carry_1_col2: QM31,
    carry_2_col3: QM31,
    carry_3_col4: QM31,
    carry_4_col5: QM31,
    carry_5_col6: QM31,
    carry_6_col7: QM31,
    carry_7_col8: QM31,
    carry_8_col9: QM31,
    carry_9_col10: QM31,
    carry_10_col11: QM31,
    carry_11_col12: QM31,
    carry_12_col13: QM31,
    carry_13_col14: QM31,
    carry_14_col15: QM31,
    carry_15_col16: QM31,
    carry_16_col17: QM31,
    carry_17_col18: QM31,
    carry_18_col19: QM31,
    carry_19_col20: QM31,
    carry_20_col21: QM31,
    carry_21_col22: QM31,
    carry_22_col23: QM31,
    carry_23_col24: QM31,
    carry_24_col25: QM31,
    carry_25_col26: QM31,
    carry_26_col27: QM31,
    range_check_20_lookup_elements: @crate::RangeCheck_20Elements,
    range_check_20_b_lookup_elements: @crate::RangeCheck_20_BElements,
    range_check_20_c_lookup_elements: @crate::RangeCheck_20_CElements,
    range_check_20_d_lookup_elements: @crate::RangeCheck_20_DElements,
    range_check_20_e_lookup_elements: @crate::RangeCheck_20_EElements,
    range_check_20_f_lookup_elements: @crate::RangeCheck_20_FElements,
    range_check_20_g_lookup_elements: @crate::RangeCheck_20_GElements,
    range_check_20_h_lookup_elements: @crate::RangeCheck_20_HElements,
    ref range_check_20_sum_0: QM31,
    ref range_check_20_b_sum_1: QM31,
    ref range_check_20_c_sum_2: QM31,
    ref range_check_20_d_sum_3: QM31,
    ref range_check_20_e_sum_4: QM31,
    ref range_check_20_f_sum_5: QM31,
    ref range_check_20_g_sum_6: QM31,
    ref range_check_20_h_sum_7: QM31,
    ref range_check_20_sum_8: QM31,
    ref range_check_20_b_sum_9: QM31,
    ref range_check_20_c_sum_10: QM31,
    ref range_check_20_d_sum_11: QM31,
    ref range_check_20_e_sum_12: QM31,
    ref range_check_20_f_sum_13: QM31,
    ref range_check_20_g_sum_14: QM31,
    ref range_check_20_h_sum_15: QM31,
    ref range_check_20_sum_16: QM31,
    ref range_check_20_b_sum_17: QM31,
    ref range_check_20_c_sum_18: QM31,
    ref range_check_20_d_sum_19: QM31,
    ref range_check_20_e_sum_20: QM31,
    ref range_check_20_f_sum_21: QM31,
    ref range_check_20_g_sum_22: QM31,
    ref range_check_20_h_sum_23: QM31,
    ref range_check_20_sum_24: QM31,
    ref range_check_20_b_sum_25: QM31,
    ref range_check_20_c_sum_26: QM31,
    ref range_check_20_d_sum_27: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 0] {
    let [
        verify_mul_252_input_a_limb_0,
        verify_mul_252_input_a_limb_1,
        verify_mul_252_input_a_limb_2,
        verify_mul_252_input_a_limb_3,
        verify_mul_252_input_a_limb_4,
        verify_mul_252_input_a_limb_5,
        verify_mul_252_input_a_limb_6,
        verify_mul_252_input_a_limb_7,
        verify_mul_252_input_a_limb_8,
        verify_mul_252_input_a_limb_9,
        verify_mul_252_input_a_limb_10,
        verify_mul_252_input_a_limb_11,
        verify_mul_252_input_a_limb_12,
        verify_mul_252_input_a_limb_13,
        verify_mul_252_input_a_limb_14,
        verify_mul_252_input_a_limb_15,
        verify_mul_252_input_a_limb_16,
        verify_mul_252_input_a_limb_17,
        verify_mul_252_input_a_limb_18,
        verify_mul_252_input_a_limb_19,
        verify_mul_252_input_a_limb_20,
        verify_mul_252_input_a_limb_21,
        verify_mul_252_input_a_limb_22,
        verify_mul_252_input_a_limb_23,
        verify_mul_252_input_a_limb_24,
        verify_mul_252_input_a_limb_25,
        verify_mul_252_input_a_limb_26,
        verify_mul_252_input_a_limb_27,
        verify_mul_252_input_b_limb_0,
        verify_mul_252_input_b_limb_1,
        verify_mul_252_input_b_limb_2,
        verify_mul_252_input_b_limb_3,
        verify_mul_252_input_b_limb_4,
        verify_mul_252_input_b_limb_5,
        verify_mul_252_input_b_limb_6,
        verify_mul_252_input_b_limb_7,
        verify_mul_252_input_b_limb_8,
        verify_mul_252_input_b_limb_9,
        verify_mul_252_input_b_limb_10,
        verify_mul_252_input_b_limb_11,
        verify_mul_252_input_b_limb_12,
        verify_mul_252_input_b_limb_13,
        verify_mul_252_input_b_limb_14,
        verify_mul_252_input_b_limb_15,
        verify_mul_252_input_b_limb_16,
        verify_mul_252_input_b_limb_17,
        verify_mul_252_input_b_limb_18,
        verify_mul_252_input_b_limb_19,
        verify_mul_252_input_b_limb_20,
        verify_mul_252_input_b_limb_21,
        verify_mul_252_input_b_limb_22,
        verify_mul_252_input_b_limb_23,
        verify_mul_252_input_b_limb_24,
        verify_mul_252_input_b_limb_25,
        verify_mul_252_input_b_limb_26,
        verify_mul_252_input_b_limb_27,
        verify_mul_252_input_c_limb_0,
        verify_mul_252_input_c_limb_1,
        verify_mul_252_input_c_limb_2,
        verify_mul_252_input_c_limb_3,
        verify_mul_252_input_c_limb_4,
        verify_mul_252_input_c_limb_5,
        verify_mul_252_input_c_limb_6,
        verify_mul_252_input_c_limb_7,
        verify_mul_252_input_c_limb_8,
        verify_mul_252_input_c_limb_9,
        verify_mul_252_input_c_limb_10,
        verify_mul_252_input_c_limb_11,
        verify_mul_252_input_c_limb_12,
        verify_mul_252_input_c_limb_13,
        verify_mul_252_input_c_limb_14,
        verify_mul_252_input_c_limb_15,
        verify_mul_252_input_c_limb_16,
        verify_mul_252_input_c_limb_17,
        verify_mul_252_input_c_limb_18,
        verify_mul_252_input_c_limb_19,
        verify_mul_252_input_c_limb_20,
        verify_mul_252_input_c_limb_21,
        verify_mul_252_input_c_limb_22,
        verify_mul_252_input_c_limb_23,
        verify_mul_252_input_c_limb_24,
        verify_mul_252_input_c_limb_25,
        verify_mul_252_input_c_limb_26,
        verify_mul_252_input_c_limb_27,
    ] =
        input;
    let [
        double_karatsuba_1454b_output_tmp_9a554_17_limb_0,
        double_karatsuba_1454b_output_tmp_9a554_17_limb_1,
        double_karatsuba_1454b_output_tmp_9a554_17_limb_2,
        double_karatsuba_1454b_output_tmp_9a554_17_limb_3,
        double_karatsuba_1454b_output_tmp_9a554_17_limb_4,
        double_karatsuba_1454b_output_tmp_9a554_17_limb_5,
        double_karatsuba_1454b_output_tmp_9a554_17_limb_6,
        double_karatsuba_1454b_output_tmp_9a554_17_limb_7,
        double_karatsuba_1454b_output_tmp_9a554_17_limb_8,
        double_karatsuba_1454b_output_tmp_9a554_17_limb_9,
        double_karatsuba_1454b_output_tmp_9a554_17_limb_10,
        double_karatsuba_1454b_output_tmp_9a554_17_limb_11,
        double_karatsuba_1454b_output_tmp_9a554_17_limb_12,
        double_karatsuba_1454b_output_tmp_9a554_17_limb_13,
        double_karatsuba_1454b_output_tmp_9a554_17_limb_14,
        double_karatsuba_1454b_output_tmp_9a554_17_limb_15,
        double_karatsuba_1454b_output_tmp_9a554_17_limb_16,
        double_karatsuba_1454b_output_tmp_9a554_17_limb_17,
        double_karatsuba_1454b_output_tmp_9a554_17_limb_18,
        double_karatsuba_1454b_output_tmp_9a554_17_limb_19,
        double_karatsuba_1454b_output_tmp_9a554_17_limb_20,
        double_karatsuba_1454b_output_tmp_9a554_17_limb_21,
        double_karatsuba_1454b_output_tmp_9a554_17_limb_22,
        double_karatsuba_1454b_output_tmp_9a554_17_limb_23,
        double_karatsuba_1454b_output_tmp_9a554_17_limb_24,
        double_karatsuba_1454b_output_tmp_9a554_17_limb_25,
        double_karatsuba_1454b_output_tmp_9a554_17_limb_26,
        double_karatsuba_1454b_output_tmp_9a554_17_limb_27,
        double_karatsuba_1454b_output_tmp_9a554_17_limb_28,
        double_karatsuba_1454b_output_tmp_9a554_17_limb_29,
        double_karatsuba_1454b_output_tmp_9a554_17_limb_30,
        double_karatsuba_1454b_output_tmp_9a554_17_limb_31,
        double_karatsuba_1454b_output_tmp_9a554_17_limb_32,
        double_karatsuba_1454b_output_tmp_9a554_17_limb_33,
        double_karatsuba_1454b_output_tmp_9a554_17_limb_34,
        double_karatsuba_1454b_output_tmp_9a554_17_limb_35,
        double_karatsuba_1454b_output_tmp_9a554_17_limb_36,
        double_karatsuba_1454b_output_tmp_9a554_17_limb_37,
        double_karatsuba_1454b_output_tmp_9a554_17_limb_38,
        double_karatsuba_1454b_output_tmp_9a554_17_limb_39,
        double_karatsuba_1454b_output_tmp_9a554_17_limb_40,
        double_karatsuba_1454b_output_tmp_9a554_17_limb_41,
        double_karatsuba_1454b_output_tmp_9a554_17_limb_42,
        double_karatsuba_1454b_output_tmp_9a554_17_limb_43,
        double_karatsuba_1454b_output_tmp_9a554_17_limb_44,
        double_karatsuba_1454b_output_tmp_9a554_17_limb_45,
        double_karatsuba_1454b_output_tmp_9a554_17_limb_46,
        double_karatsuba_1454b_output_tmp_9a554_17_limb_47,
        double_karatsuba_1454b_output_tmp_9a554_17_limb_48,
        double_karatsuba_1454b_output_tmp_9a554_17_limb_49,
        double_karatsuba_1454b_output_tmp_9a554_17_limb_50,
        double_karatsuba_1454b_output_tmp_9a554_17_limb_51,
        double_karatsuba_1454b_output_tmp_9a554_17_limb_52,
        double_karatsuba_1454b_output_tmp_9a554_17_limb_53,
        double_karatsuba_1454b_output_tmp_9a554_17_limb_54,
    ] =
        double_karatsuba_1454b_evaluate(
        [
            verify_mul_252_input_a_limb_0, verify_mul_252_input_a_limb_1,
            verify_mul_252_input_a_limb_2, verify_mul_252_input_a_limb_3,
            verify_mul_252_input_a_limb_4, verify_mul_252_input_a_limb_5,
            verify_mul_252_input_a_limb_6, verify_mul_252_input_a_limb_7,
            verify_mul_252_input_a_limb_8, verify_mul_252_input_a_limb_9,
            verify_mul_252_input_a_limb_10, verify_mul_252_input_a_limb_11,
            verify_mul_252_input_a_limb_12, verify_mul_252_input_a_limb_13,
            verify_mul_252_input_a_limb_14, verify_mul_252_input_a_limb_15,
            verify_mul_252_input_a_limb_16, verify_mul_252_input_a_limb_17,
            verify_mul_252_input_a_limb_18, verify_mul_252_input_a_limb_19,
            verify_mul_252_input_a_limb_20, verify_mul_252_input_a_limb_21,
            verify_mul_252_input_a_limb_22, verify_mul_252_input_a_limb_23,
            verify_mul_252_input_a_limb_24, verify_mul_252_input_a_limb_25,
            verify_mul_252_input_a_limb_26, verify_mul_252_input_a_limb_27,
            verify_mul_252_input_b_limb_0, verify_mul_252_input_b_limb_1,
            verify_mul_252_input_b_limb_2, verify_mul_252_input_b_limb_3,
            verify_mul_252_input_b_limb_4, verify_mul_252_input_b_limb_5,
            verify_mul_252_input_b_limb_6, verify_mul_252_input_b_limb_7,
            verify_mul_252_input_b_limb_8, verify_mul_252_input_b_limb_9,
            verify_mul_252_input_b_limb_10, verify_mul_252_input_b_limb_11,
            verify_mul_252_input_b_limb_12, verify_mul_252_input_b_limb_13,
            verify_mul_252_input_b_limb_14, verify_mul_252_input_b_limb_15,
            verify_mul_252_input_b_limb_16, verify_mul_252_input_b_limb_17,
            verify_mul_252_input_b_limb_18, verify_mul_252_input_b_limb_19,
            verify_mul_252_input_b_limb_20, verify_mul_252_input_b_limb_21,
            verify_mul_252_input_b_limb_22, verify_mul_252_input_b_limb_23,
            verify_mul_252_input_b_limb_24, verify_mul_252_input_b_limb_25,
            verify_mul_252_input_b_limb_26, verify_mul_252_input_b_limb_27,
        ],
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );
    let conv_tmp_9a554_18_limb_0: QM31 = (double_karatsuba_1454b_output_tmp_9a554_17_limb_0
        - verify_mul_252_input_c_limb_0);
    let conv_tmp_9a554_18_limb_1: QM31 = (double_karatsuba_1454b_output_tmp_9a554_17_limb_1
        - verify_mul_252_input_c_limb_1);
    let conv_tmp_9a554_18_limb_2: QM31 = (double_karatsuba_1454b_output_tmp_9a554_17_limb_2
        - verify_mul_252_input_c_limb_2);
    let conv_tmp_9a554_18_limb_3: QM31 = (double_karatsuba_1454b_output_tmp_9a554_17_limb_3
        - verify_mul_252_input_c_limb_3);
    let conv_tmp_9a554_18_limb_4: QM31 = (double_karatsuba_1454b_output_tmp_9a554_17_limb_4
        - verify_mul_252_input_c_limb_4);
    let conv_tmp_9a554_18_limb_5: QM31 = (double_karatsuba_1454b_output_tmp_9a554_17_limb_5
        - verify_mul_252_input_c_limb_5);
    let conv_tmp_9a554_18_limb_6: QM31 = (double_karatsuba_1454b_output_tmp_9a554_17_limb_6
        - verify_mul_252_input_c_limb_6);
    let conv_tmp_9a554_18_limb_7: QM31 = (double_karatsuba_1454b_output_tmp_9a554_17_limb_7
        - verify_mul_252_input_c_limb_7);
    let conv_tmp_9a554_18_limb_8: QM31 = (double_karatsuba_1454b_output_tmp_9a554_17_limb_8
        - verify_mul_252_input_c_limb_8);
    let conv_tmp_9a554_18_limb_9: QM31 = (double_karatsuba_1454b_output_tmp_9a554_17_limb_9
        - verify_mul_252_input_c_limb_9);
    let conv_tmp_9a554_18_limb_10: QM31 = (double_karatsuba_1454b_output_tmp_9a554_17_limb_10
        - verify_mul_252_input_c_limb_10);
    let conv_tmp_9a554_18_limb_11: QM31 = (double_karatsuba_1454b_output_tmp_9a554_17_limb_11
        - verify_mul_252_input_c_limb_11);
    let conv_tmp_9a554_18_limb_12: QM31 = (double_karatsuba_1454b_output_tmp_9a554_17_limb_12
        - verify_mul_252_input_c_limb_12);
    let conv_tmp_9a554_18_limb_13: QM31 = (double_karatsuba_1454b_output_tmp_9a554_17_limb_13
        - verify_mul_252_input_c_limb_13);
    let conv_tmp_9a554_18_limb_14: QM31 = (double_karatsuba_1454b_output_tmp_9a554_17_limb_14
        - verify_mul_252_input_c_limb_14);
    let conv_tmp_9a554_18_limb_15: QM31 = (double_karatsuba_1454b_output_tmp_9a554_17_limb_15
        - verify_mul_252_input_c_limb_15);
    let conv_tmp_9a554_18_limb_16: QM31 = (double_karatsuba_1454b_output_tmp_9a554_17_limb_16
        - verify_mul_252_input_c_limb_16);
    let conv_tmp_9a554_18_limb_17: QM31 = (double_karatsuba_1454b_output_tmp_9a554_17_limb_17
        - verify_mul_252_input_c_limb_17);
    let conv_tmp_9a554_18_limb_18: QM31 = (double_karatsuba_1454b_output_tmp_9a554_17_limb_18
        - verify_mul_252_input_c_limb_18);
    let conv_tmp_9a554_18_limb_19: QM31 = (double_karatsuba_1454b_output_tmp_9a554_17_limb_19
        - verify_mul_252_input_c_limb_19);
    let conv_tmp_9a554_18_limb_20: QM31 = (double_karatsuba_1454b_output_tmp_9a554_17_limb_20
        - verify_mul_252_input_c_limb_20);
    let conv_tmp_9a554_18_limb_21: QM31 = (double_karatsuba_1454b_output_tmp_9a554_17_limb_21
        - verify_mul_252_input_c_limb_21);
    let conv_tmp_9a554_18_limb_22: QM31 = (double_karatsuba_1454b_output_tmp_9a554_17_limb_22
        - verify_mul_252_input_c_limb_22);
    let conv_tmp_9a554_18_limb_23: QM31 = (double_karatsuba_1454b_output_tmp_9a554_17_limb_23
        - verify_mul_252_input_c_limb_23);
    let conv_tmp_9a554_18_limb_24: QM31 = (double_karatsuba_1454b_output_tmp_9a554_17_limb_24
        - verify_mul_252_input_c_limb_24);
    let conv_tmp_9a554_18_limb_25: QM31 = (double_karatsuba_1454b_output_tmp_9a554_17_limb_25
        - verify_mul_252_input_c_limb_25);
    let conv_tmp_9a554_18_limb_26: QM31 = (double_karatsuba_1454b_output_tmp_9a554_17_limb_26
        - verify_mul_252_input_c_limb_26);
    let conv_tmp_9a554_18_limb_27: QM31 = (double_karatsuba_1454b_output_tmp_9a554_17_limb_27
        - verify_mul_252_input_c_limb_27);
    let conv_tmp_9a554_18_limb_28: QM31 = double_karatsuba_1454b_output_tmp_9a554_17_limb_28;
    let conv_tmp_9a554_18_limb_29: QM31 = double_karatsuba_1454b_output_tmp_9a554_17_limb_29;
    let conv_tmp_9a554_18_limb_30: QM31 = double_karatsuba_1454b_output_tmp_9a554_17_limb_30;
    let conv_tmp_9a554_18_limb_31: QM31 = double_karatsuba_1454b_output_tmp_9a554_17_limb_31;
    let conv_tmp_9a554_18_limb_32: QM31 = double_karatsuba_1454b_output_tmp_9a554_17_limb_32;
    let conv_tmp_9a554_18_limb_33: QM31 = double_karatsuba_1454b_output_tmp_9a554_17_limb_33;
    let conv_tmp_9a554_18_limb_34: QM31 = double_karatsuba_1454b_output_tmp_9a554_17_limb_34;
    let conv_tmp_9a554_18_limb_35: QM31 = double_karatsuba_1454b_output_tmp_9a554_17_limb_35;
    let conv_tmp_9a554_18_limb_36: QM31 = double_karatsuba_1454b_output_tmp_9a554_17_limb_36;
    let conv_tmp_9a554_18_limb_37: QM31 = double_karatsuba_1454b_output_tmp_9a554_17_limb_37;
    let conv_tmp_9a554_18_limb_38: QM31 = double_karatsuba_1454b_output_tmp_9a554_17_limb_38;
    let conv_tmp_9a554_18_limb_39: QM31 = double_karatsuba_1454b_output_tmp_9a554_17_limb_39;
    let conv_tmp_9a554_18_limb_40: QM31 = double_karatsuba_1454b_output_tmp_9a554_17_limb_40;
    let conv_tmp_9a554_18_limb_41: QM31 = double_karatsuba_1454b_output_tmp_9a554_17_limb_41;
    let conv_tmp_9a554_18_limb_42: QM31 = double_karatsuba_1454b_output_tmp_9a554_17_limb_42;
    let conv_tmp_9a554_18_limb_43: QM31 = double_karatsuba_1454b_output_tmp_9a554_17_limb_43;
    let conv_tmp_9a554_18_limb_44: QM31 = double_karatsuba_1454b_output_tmp_9a554_17_limb_44;
    let conv_tmp_9a554_18_limb_45: QM31 = double_karatsuba_1454b_output_tmp_9a554_17_limb_45;
    let conv_tmp_9a554_18_limb_46: QM31 = double_karatsuba_1454b_output_tmp_9a554_17_limb_46;
    let conv_tmp_9a554_18_limb_47: QM31 = double_karatsuba_1454b_output_tmp_9a554_17_limb_47;
    let conv_tmp_9a554_18_limb_48: QM31 = double_karatsuba_1454b_output_tmp_9a554_17_limb_48;
    let conv_tmp_9a554_18_limb_49: QM31 = double_karatsuba_1454b_output_tmp_9a554_17_limb_49;
    let conv_tmp_9a554_18_limb_50: QM31 = double_karatsuba_1454b_output_tmp_9a554_17_limb_50;
    let conv_tmp_9a554_18_limb_51: QM31 = double_karatsuba_1454b_output_tmp_9a554_17_limb_51;
    let conv_tmp_9a554_18_limb_52: QM31 = double_karatsuba_1454b_output_tmp_9a554_17_limb_52;
    let conv_tmp_9a554_18_limb_53: QM31 = double_karatsuba_1454b_output_tmp_9a554_17_limb_53;
    let conv_tmp_9a554_18_limb_54: QM31 = double_karatsuba_1454b_output_tmp_9a554_17_limb_54;
    let conv_mod_tmp_9a554_19_limb_0: QM31 = (((qm31_const::<32, 0, 0, 0>()
        * conv_tmp_9a554_18_limb_0)
        - (qm31_const::<4, 0, 0, 0>() * conv_tmp_9a554_18_limb_21))
        + (qm31_const::<8, 0, 0, 0>() * conv_tmp_9a554_18_limb_49));
    let conv_mod_tmp_9a554_19_limb_1: QM31 = (((conv_tmp_9a554_18_limb_0
        + (qm31_const::<32, 0, 0, 0>() * conv_tmp_9a554_18_limb_1))
        - (qm31_const::<4, 0, 0, 0>() * conv_tmp_9a554_18_limb_22))
        + (qm31_const::<8, 0, 0, 0>() * conv_tmp_9a554_18_limb_50));
    let conv_mod_tmp_9a554_19_limb_2: QM31 = (((conv_tmp_9a554_18_limb_1
        + (qm31_const::<32, 0, 0, 0>() * conv_tmp_9a554_18_limb_2))
        - (qm31_const::<4, 0, 0, 0>() * conv_tmp_9a554_18_limb_23))
        + (qm31_const::<8, 0, 0, 0>() * conv_tmp_9a554_18_limb_51));
    let conv_mod_tmp_9a554_19_limb_3: QM31 = (((conv_tmp_9a554_18_limb_2
        + (qm31_const::<32, 0, 0, 0>() * conv_tmp_9a554_18_limb_3))
        - (qm31_const::<4, 0, 0, 0>() * conv_tmp_9a554_18_limb_24))
        + (qm31_const::<8, 0, 0, 0>() * conv_tmp_9a554_18_limb_52));
    let conv_mod_tmp_9a554_19_limb_4: QM31 = (((conv_tmp_9a554_18_limb_3
        + (qm31_const::<32, 0, 0, 0>() * conv_tmp_9a554_18_limb_4))
        - (qm31_const::<4, 0, 0, 0>() * conv_tmp_9a554_18_limb_25))
        + (qm31_const::<8, 0, 0, 0>() * conv_tmp_9a554_18_limb_53));
    let conv_mod_tmp_9a554_19_limb_5: QM31 = (((conv_tmp_9a554_18_limb_4
        + (qm31_const::<32, 0, 0, 0>() * conv_tmp_9a554_18_limb_5))
        - (qm31_const::<4, 0, 0, 0>() * conv_tmp_9a554_18_limb_26))
        + (qm31_const::<8, 0, 0, 0>() * conv_tmp_9a554_18_limb_54));
    let conv_mod_tmp_9a554_19_limb_6: QM31 = ((conv_tmp_9a554_18_limb_5
        + (qm31_const::<32, 0, 0, 0>() * conv_tmp_9a554_18_limb_6))
        - (qm31_const::<4, 0, 0, 0>() * conv_tmp_9a554_18_limb_27));
    let conv_mod_tmp_9a554_19_limb_7: QM31 = ((((qm31_const::<2, 0, 0, 0>()
        * conv_tmp_9a554_18_limb_0)
        + conv_tmp_9a554_18_limb_6)
        + (qm31_const::<32, 0, 0, 0>() * conv_tmp_9a554_18_limb_7))
        - (qm31_const::<4, 0, 0, 0>() * conv_tmp_9a554_18_limb_28));
    let conv_mod_tmp_9a554_19_limb_8: QM31 = ((((qm31_const::<2, 0, 0, 0>()
        * conv_tmp_9a554_18_limb_1)
        + conv_tmp_9a554_18_limb_7)
        + (qm31_const::<32, 0, 0, 0>() * conv_tmp_9a554_18_limb_8))
        - (qm31_const::<4, 0, 0, 0>() * conv_tmp_9a554_18_limb_29));
    let conv_mod_tmp_9a554_19_limb_9: QM31 = ((((qm31_const::<2, 0, 0, 0>()
        * conv_tmp_9a554_18_limb_2)
        + conv_tmp_9a554_18_limb_8)
        + (qm31_const::<32, 0, 0, 0>() * conv_tmp_9a554_18_limb_9))
        - (qm31_const::<4, 0, 0, 0>() * conv_tmp_9a554_18_limb_30));
    let conv_mod_tmp_9a554_19_limb_10: QM31 = ((((qm31_const::<2, 0, 0, 0>()
        * conv_tmp_9a554_18_limb_3)
        + conv_tmp_9a554_18_limb_9)
        + (qm31_const::<32, 0, 0, 0>() * conv_tmp_9a554_18_limb_10))
        - (qm31_const::<4, 0, 0, 0>() * conv_tmp_9a554_18_limb_31));
    let conv_mod_tmp_9a554_19_limb_11: QM31 = ((((qm31_const::<2, 0, 0, 0>()
        * conv_tmp_9a554_18_limb_4)
        + conv_tmp_9a554_18_limb_10)
        + (qm31_const::<32, 0, 0, 0>() * conv_tmp_9a554_18_limb_11))
        - (qm31_const::<4, 0, 0, 0>() * conv_tmp_9a554_18_limb_32));
    let conv_mod_tmp_9a554_19_limb_12: QM31 = ((((qm31_const::<2, 0, 0, 0>()
        * conv_tmp_9a554_18_limb_5)
        + conv_tmp_9a554_18_limb_11)
        + (qm31_const::<32, 0, 0, 0>() * conv_tmp_9a554_18_limb_12))
        - (qm31_const::<4, 0, 0, 0>() * conv_tmp_9a554_18_limb_33));
    let conv_mod_tmp_9a554_19_limb_13: QM31 = ((((qm31_const::<2, 0, 0, 0>()
        * conv_tmp_9a554_18_limb_6)
        + conv_tmp_9a554_18_limb_12)
        + (qm31_const::<32, 0, 0, 0>() * conv_tmp_9a554_18_limb_13))
        - (qm31_const::<4, 0, 0, 0>() * conv_tmp_9a554_18_limb_34));
    let conv_mod_tmp_9a554_19_limb_14: QM31 = ((((qm31_const::<2, 0, 0, 0>()
        * conv_tmp_9a554_18_limb_7)
        + conv_tmp_9a554_18_limb_13)
        + (qm31_const::<32, 0, 0, 0>() * conv_tmp_9a554_18_limb_14))
        - (qm31_const::<4, 0, 0, 0>() * conv_tmp_9a554_18_limb_35));
    let conv_mod_tmp_9a554_19_limb_15: QM31 = ((((qm31_const::<2, 0, 0, 0>()
        * conv_tmp_9a554_18_limb_8)
        + conv_tmp_9a554_18_limb_14)
        + (qm31_const::<32, 0, 0, 0>() * conv_tmp_9a554_18_limb_15))
        - (qm31_const::<4, 0, 0, 0>() * conv_tmp_9a554_18_limb_36));
    let conv_mod_tmp_9a554_19_limb_16: QM31 = ((((qm31_const::<2, 0, 0, 0>()
        * conv_tmp_9a554_18_limb_9)
        + conv_tmp_9a554_18_limb_15)
        + (qm31_const::<32, 0, 0, 0>() * conv_tmp_9a554_18_limb_16))
        - (qm31_const::<4, 0, 0, 0>() * conv_tmp_9a554_18_limb_37));
    let conv_mod_tmp_9a554_19_limb_17: QM31 = ((((qm31_const::<2, 0, 0, 0>()
        * conv_tmp_9a554_18_limb_10)
        + conv_tmp_9a554_18_limb_16)
        + (qm31_const::<32, 0, 0, 0>() * conv_tmp_9a554_18_limb_17))
        - (qm31_const::<4, 0, 0, 0>() * conv_tmp_9a554_18_limb_38));
    let conv_mod_tmp_9a554_19_limb_18: QM31 = ((((qm31_const::<2, 0, 0, 0>()
        * conv_tmp_9a554_18_limb_11)
        + conv_tmp_9a554_18_limb_17)
        + (qm31_const::<32, 0, 0, 0>() * conv_tmp_9a554_18_limb_18))
        - (qm31_const::<4, 0, 0, 0>() * conv_tmp_9a554_18_limb_39));
    let conv_mod_tmp_9a554_19_limb_19: QM31 = ((((qm31_const::<2, 0, 0, 0>()
        * conv_tmp_9a554_18_limb_12)
        + conv_tmp_9a554_18_limb_18)
        + (qm31_const::<32, 0, 0, 0>() * conv_tmp_9a554_18_limb_19))
        - (qm31_const::<4, 0, 0, 0>() * conv_tmp_9a554_18_limb_40));
    let conv_mod_tmp_9a554_19_limb_20: QM31 = ((((qm31_const::<2, 0, 0, 0>()
        * conv_tmp_9a554_18_limb_13)
        + conv_tmp_9a554_18_limb_19)
        + (qm31_const::<32, 0, 0, 0>() * conv_tmp_9a554_18_limb_20))
        - (qm31_const::<4, 0, 0, 0>() * conv_tmp_9a554_18_limb_41));
    let conv_mod_tmp_9a554_19_limb_21: QM31 = ((((qm31_const::<2, 0, 0, 0>()
        * conv_tmp_9a554_18_limb_14)
        + conv_tmp_9a554_18_limb_20)
        - (qm31_const::<4, 0, 0, 0>() * conv_tmp_9a554_18_limb_42))
        + (qm31_const::<64, 0, 0, 0>() * conv_tmp_9a554_18_limb_49));
    let conv_mod_tmp_9a554_19_limb_22: QM31 = ((((qm31_const::<2, 0, 0, 0>()
        * conv_tmp_9a554_18_limb_15)
        - (qm31_const::<4, 0, 0, 0>() * conv_tmp_9a554_18_limb_43))
        + (qm31_const::<2, 0, 0, 0>() * conv_tmp_9a554_18_limb_49))
        + (qm31_const::<64, 0, 0, 0>() * conv_tmp_9a554_18_limb_50));
    let conv_mod_tmp_9a554_19_limb_23: QM31 = ((((qm31_const::<2, 0, 0, 0>()
        * conv_tmp_9a554_18_limb_16)
        - (qm31_const::<4, 0, 0, 0>() * conv_tmp_9a554_18_limb_44))
        + (qm31_const::<2, 0, 0, 0>() * conv_tmp_9a554_18_limb_50))
        + (qm31_const::<64, 0, 0, 0>() * conv_tmp_9a554_18_limb_51));
    let conv_mod_tmp_9a554_19_limb_24: QM31 = ((((qm31_const::<2, 0, 0, 0>()
        * conv_tmp_9a554_18_limb_17)
        - (qm31_const::<4, 0, 0, 0>() * conv_tmp_9a554_18_limb_45))
        + (qm31_const::<2, 0, 0, 0>() * conv_tmp_9a554_18_limb_51))
        + (qm31_const::<64, 0, 0, 0>() * conv_tmp_9a554_18_limb_52));
    let conv_mod_tmp_9a554_19_limb_25: QM31 = ((((qm31_const::<2, 0, 0, 0>()
        * conv_tmp_9a554_18_limb_18)
        - (qm31_const::<4, 0, 0, 0>() * conv_tmp_9a554_18_limb_46))
        + (qm31_const::<2, 0, 0, 0>() * conv_tmp_9a554_18_limb_52))
        + (qm31_const::<64, 0, 0, 0>() * conv_tmp_9a554_18_limb_53));
    let conv_mod_tmp_9a554_19_limb_26: QM31 = ((((qm31_const::<2, 0, 0, 0>()
        * conv_tmp_9a554_18_limb_19)
        - (qm31_const::<4, 0, 0, 0>() * conv_tmp_9a554_18_limb_47))
        + (qm31_const::<2, 0, 0, 0>() * conv_tmp_9a554_18_limb_53))
        + (qm31_const::<64, 0, 0, 0>() * conv_tmp_9a554_18_limb_54));
    let conv_mod_tmp_9a554_19_limb_27: QM31 = (((qm31_const::<2, 0, 0, 0>()
        * conv_tmp_9a554_18_limb_20)
        - (qm31_const::<4, 0, 0, 0>() * conv_tmp_9a554_18_limb_48))
        + (qm31_const::<2, 0, 0, 0>() * conv_tmp_9a554_18_limb_54));

    range_check_20_sum_0 = range_check_20_lookup_elements
        .combine_qm31([(k_col0 + qm31_const::<524288, 0, 0, 0>())]);

    // Constraint -
    let constraint_quotient = (((carry_0_col1 * qm31_const::<512, 0, 0, 0>())
        - (conv_mod_tmp_9a554_19_limb_0 - k_col0)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    range_check_20_b_sum_1 = range_check_20_b_lookup_elements
        .combine_qm31([(carry_0_col1 + qm31_const::<524288, 0, 0, 0>())]);

    // Constraint -
    let constraint_quotient = (((carry_1_col2 * qm31_const::<512, 0, 0, 0>())
        - (conv_mod_tmp_9a554_19_limb_1 + carry_0_col1)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    range_check_20_c_sum_2 = range_check_20_c_lookup_elements
        .combine_qm31([(carry_1_col2 + qm31_const::<524288, 0, 0, 0>())]);

    // Constraint -
    let constraint_quotient = (((carry_2_col3 * qm31_const::<512, 0, 0, 0>())
        - (conv_mod_tmp_9a554_19_limb_2 + carry_1_col2)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    range_check_20_d_sum_3 = range_check_20_d_lookup_elements
        .combine_qm31([(carry_2_col3 + qm31_const::<524288, 0, 0, 0>())]);

    // Constraint -
    let constraint_quotient = (((carry_3_col4 * qm31_const::<512, 0, 0, 0>())
        - (conv_mod_tmp_9a554_19_limb_3 + carry_2_col3)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    range_check_20_e_sum_4 = range_check_20_e_lookup_elements
        .combine_qm31([(carry_3_col4 + qm31_const::<524288, 0, 0, 0>())]);

    // Constraint -
    let constraint_quotient = (((carry_4_col5 * qm31_const::<512, 0, 0, 0>())
        - (conv_mod_tmp_9a554_19_limb_4 + carry_3_col4)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    range_check_20_f_sum_5 = range_check_20_f_lookup_elements
        .combine_qm31([(carry_4_col5 + qm31_const::<524288, 0, 0, 0>())]);

    // Constraint -
    let constraint_quotient = (((carry_5_col6 * qm31_const::<512, 0, 0, 0>())
        - (conv_mod_tmp_9a554_19_limb_5 + carry_4_col5)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    range_check_20_g_sum_6 = range_check_20_g_lookup_elements
        .combine_qm31([(carry_5_col6 + qm31_const::<524288, 0, 0, 0>())]);

    // Constraint -
    let constraint_quotient = (((carry_6_col7 * qm31_const::<512, 0, 0, 0>())
        - (conv_mod_tmp_9a554_19_limb_6 + carry_5_col6)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    range_check_20_h_sum_7 = range_check_20_h_lookup_elements
        .combine_qm31([(carry_6_col7 + qm31_const::<524288, 0, 0, 0>())]);

    // Constraint -
    let constraint_quotient = (((carry_7_col8 * qm31_const::<512, 0, 0, 0>())
        - (conv_mod_tmp_9a554_19_limb_7 + carry_6_col7)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    range_check_20_sum_8 = range_check_20_lookup_elements
        .combine_qm31([(carry_7_col8 + qm31_const::<524288, 0, 0, 0>())]);

    // Constraint -
    let constraint_quotient = (((carry_8_col9 * qm31_const::<512, 0, 0, 0>())
        - (conv_mod_tmp_9a554_19_limb_8 + carry_7_col8)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    range_check_20_b_sum_9 = range_check_20_b_lookup_elements
        .combine_qm31([(carry_8_col9 + qm31_const::<524288, 0, 0, 0>())]);

    // Constraint -
    let constraint_quotient = (((carry_9_col10 * qm31_const::<512, 0, 0, 0>())
        - (conv_mod_tmp_9a554_19_limb_9 + carry_8_col9)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    range_check_20_c_sum_10 = range_check_20_c_lookup_elements
        .combine_qm31([(carry_9_col10 + qm31_const::<524288, 0, 0, 0>())]);

    // Constraint -
    let constraint_quotient = (((carry_10_col11 * qm31_const::<512, 0, 0, 0>())
        - (conv_mod_tmp_9a554_19_limb_10 + carry_9_col10)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    range_check_20_d_sum_11 = range_check_20_d_lookup_elements
        .combine_qm31([(carry_10_col11 + qm31_const::<524288, 0, 0, 0>())]);

    // Constraint -
    let constraint_quotient = (((carry_11_col12 * qm31_const::<512, 0, 0, 0>())
        - (conv_mod_tmp_9a554_19_limb_11 + carry_10_col11)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    range_check_20_e_sum_12 = range_check_20_e_lookup_elements
        .combine_qm31([(carry_11_col12 + qm31_const::<524288, 0, 0, 0>())]);

    // Constraint -
    let constraint_quotient = (((carry_12_col13 * qm31_const::<512, 0, 0, 0>())
        - (conv_mod_tmp_9a554_19_limb_12 + carry_11_col12)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    range_check_20_f_sum_13 = range_check_20_f_lookup_elements
        .combine_qm31([(carry_12_col13 + qm31_const::<524288, 0, 0, 0>())]);

    // Constraint -
    let constraint_quotient = (((carry_13_col14 * qm31_const::<512, 0, 0, 0>())
        - (conv_mod_tmp_9a554_19_limb_13 + carry_12_col13)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    range_check_20_g_sum_14 = range_check_20_g_lookup_elements
        .combine_qm31([(carry_13_col14 + qm31_const::<524288, 0, 0, 0>())]);

    // Constraint -
    let constraint_quotient = (((carry_14_col15 * qm31_const::<512, 0, 0, 0>())
        - (conv_mod_tmp_9a554_19_limb_14 + carry_13_col14)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    range_check_20_h_sum_15 = range_check_20_h_lookup_elements
        .combine_qm31([(carry_14_col15 + qm31_const::<524288, 0, 0, 0>())]);

    // Constraint -
    let constraint_quotient = (((carry_15_col16 * qm31_const::<512, 0, 0, 0>())
        - (conv_mod_tmp_9a554_19_limb_15 + carry_14_col15)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    range_check_20_sum_16 = range_check_20_lookup_elements
        .combine_qm31([(carry_15_col16 + qm31_const::<524288, 0, 0, 0>())]);

    // Constraint -
    let constraint_quotient = (((carry_16_col17 * qm31_const::<512, 0, 0, 0>())
        - (conv_mod_tmp_9a554_19_limb_16 + carry_15_col16)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    range_check_20_b_sum_17 = range_check_20_b_lookup_elements
        .combine_qm31([(carry_16_col17 + qm31_const::<524288, 0, 0, 0>())]);

    // Constraint -
    let constraint_quotient = (((carry_17_col18 * qm31_const::<512, 0, 0, 0>())
        - (conv_mod_tmp_9a554_19_limb_17 + carry_16_col17)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    range_check_20_c_sum_18 = range_check_20_c_lookup_elements
        .combine_qm31([(carry_17_col18 + qm31_const::<524288, 0, 0, 0>())]);

    // Constraint -
    let constraint_quotient = (((carry_18_col19 * qm31_const::<512, 0, 0, 0>())
        - (conv_mod_tmp_9a554_19_limb_18 + carry_17_col18)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    range_check_20_d_sum_19 = range_check_20_d_lookup_elements
        .combine_qm31([(carry_18_col19 + qm31_const::<524288, 0, 0, 0>())]);

    // Constraint -
    let constraint_quotient = (((carry_19_col20 * qm31_const::<512, 0, 0, 0>())
        - (conv_mod_tmp_9a554_19_limb_19 + carry_18_col19)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    range_check_20_e_sum_20 = range_check_20_e_lookup_elements
        .combine_qm31([(carry_19_col20 + qm31_const::<524288, 0, 0, 0>())]);

    // Constraint -
    let constraint_quotient = (((carry_20_col21 * qm31_const::<512, 0, 0, 0>())
        - (conv_mod_tmp_9a554_19_limb_20 + carry_19_col20)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    range_check_20_f_sum_21 = range_check_20_f_lookup_elements
        .combine_qm31([(carry_20_col21 + qm31_const::<524288, 0, 0, 0>())]);

    // Constraint -
    let constraint_quotient = (((carry_21_col22 * qm31_const::<512, 0, 0, 0>())
        - ((conv_mod_tmp_9a554_19_limb_21 - (qm31_const::<136, 0, 0, 0>() * k_col0))
            + carry_20_col21)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    range_check_20_g_sum_22 = range_check_20_g_lookup_elements
        .combine_qm31([(carry_21_col22 + qm31_const::<524288, 0, 0, 0>())]);

    // Constraint -
    let constraint_quotient = (((carry_22_col23 * qm31_const::<512, 0, 0, 0>())
        - (conv_mod_tmp_9a554_19_limb_22 + carry_21_col22)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    range_check_20_h_sum_23 = range_check_20_h_lookup_elements
        .combine_qm31([(carry_22_col23 + qm31_const::<524288, 0, 0, 0>())]);

    // Constraint -
    let constraint_quotient = (((carry_23_col24 * qm31_const::<512, 0, 0, 0>())
        - (conv_mod_tmp_9a554_19_limb_23 + carry_22_col23)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    range_check_20_sum_24 = range_check_20_lookup_elements
        .combine_qm31([(carry_23_col24 + qm31_const::<524288, 0, 0, 0>())]);

    // Constraint -
    let constraint_quotient = (((carry_24_col25 * qm31_const::<512, 0, 0, 0>())
        - (conv_mod_tmp_9a554_19_limb_24 + carry_23_col24)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    range_check_20_b_sum_25 = range_check_20_b_lookup_elements
        .combine_qm31([(carry_24_col25 + qm31_const::<524288, 0, 0, 0>())]);

    // Constraint -
    let constraint_quotient = (((carry_25_col26 * qm31_const::<512, 0, 0, 0>())
        - (conv_mod_tmp_9a554_19_limb_25 + carry_24_col25)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    range_check_20_c_sum_26 = range_check_20_c_lookup_elements
        .combine_qm31([(carry_25_col26 + qm31_const::<524288, 0, 0, 0>())]);

    // Constraint -
    let constraint_quotient = (((carry_26_col27 * qm31_const::<512, 0, 0, 0>())
        - (conv_mod_tmp_9a554_19_limb_26 + carry_25_col26)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    range_check_20_d_sum_27 = range_check_20_d_lookup_elements
        .combine_qm31([(carry_26_col27 + qm31_const::<524288, 0, 0, 0>())]);

    // Constraint -
    let constraint_quotient = (((conv_mod_tmp_9a554_19_limb_27
        - (qm31_const::<256, 0, 0, 0>() * k_col0))
        + carry_26_col27))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    []
}
