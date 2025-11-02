// This file was created by the AIR team.

use crate::components::subroutines::single_karatsuba_n_7::single_karatsuba_n_7_evaluate;
use crate::prelude::*;


pub fn double_karatsuba_1454b_evaluate(
    input: [QM31; 56], ref sum: QM31, domain_vanishing_eval_inv: QM31, random_coeff: QM31,
) -> [QM31; 55] {
    let [
        double_karatsuba_1454b_input_limb_0,
        double_karatsuba_1454b_input_limb_1,
        double_karatsuba_1454b_input_limb_2,
        double_karatsuba_1454b_input_limb_3,
        double_karatsuba_1454b_input_limb_4,
        double_karatsuba_1454b_input_limb_5,
        double_karatsuba_1454b_input_limb_6,
        double_karatsuba_1454b_input_limb_7,
        double_karatsuba_1454b_input_limb_8,
        double_karatsuba_1454b_input_limb_9,
        double_karatsuba_1454b_input_limb_10,
        double_karatsuba_1454b_input_limb_11,
        double_karatsuba_1454b_input_limb_12,
        double_karatsuba_1454b_input_limb_13,
        double_karatsuba_1454b_input_limb_14,
        double_karatsuba_1454b_input_limb_15,
        double_karatsuba_1454b_input_limb_16,
        double_karatsuba_1454b_input_limb_17,
        double_karatsuba_1454b_input_limb_18,
        double_karatsuba_1454b_input_limb_19,
        double_karatsuba_1454b_input_limb_20,
        double_karatsuba_1454b_input_limb_21,
        double_karatsuba_1454b_input_limb_22,
        double_karatsuba_1454b_input_limb_23,
        double_karatsuba_1454b_input_limb_24,
        double_karatsuba_1454b_input_limb_25,
        double_karatsuba_1454b_input_limb_26,
        double_karatsuba_1454b_input_limb_27,
        double_karatsuba_1454b_input_limb_28,
        double_karatsuba_1454b_input_limb_29,
        double_karatsuba_1454b_input_limb_30,
        double_karatsuba_1454b_input_limb_31,
        double_karatsuba_1454b_input_limb_32,
        double_karatsuba_1454b_input_limb_33,
        double_karatsuba_1454b_input_limb_34,
        double_karatsuba_1454b_input_limb_35,
        double_karatsuba_1454b_input_limb_36,
        double_karatsuba_1454b_input_limb_37,
        double_karatsuba_1454b_input_limb_38,
        double_karatsuba_1454b_input_limb_39,
        double_karatsuba_1454b_input_limb_40,
        double_karatsuba_1454b_input_limb_41,
        double_karatsuba_1454b_input_limb_42,
        double_karatsuba_1454b_input_limb_43,
        double_karatsuba_1454b_input_limb_44,
        double_karatsuba_1454b_input_limb_45,
        double_karatsuba_1454b_input_limb_46,
        double_karatsuba_1454b_input_limb_47,
        double_karatsuba_1454b_input_limb_48,
        double_karatsuba_1454b_input_limb_49,
        double_karatsuba_1454b_input_limb_50,
        double_karatsuba_1454b_input_limb_51,
        double_karatsuba_1454b_input_limb_52,
        double_karatsuba_1454b_input_limb_53,
        double_karatsuba_1454b_input_limb_54,
        double_karatsuba_1454b_input_limb_55,
    ] =
        input;

    core::internal::revoke_ap_tracking();

    let [
        single_karatsuba_n_7_output_tmp_1454b_4_limb_0,
        single_karatsuba_n_7_output_tmp_1454b_4_limb_1,
        single_karatsuba_n_7_output_tmp_1454b_4_limb_2,
        single_karatsuba_n_7_output_tmp_1454b_4_limb_3,
        single_karatsuba_n_7_output_tmp_1454b_4_limb_4,
        single_karatsuba_n_7_output_tmp_1454b_4_limb_5,
        single_karatsuba_n_7_output_tmp_1454b_4_limb_6,
        single_karatsuba_n_7_output_tmp_1454b_4_limb_7,
        single_karatsuba_n_7_output_tmp_1454b_4_limb_8,
        single_karatsuba_n_7_output_tmp_1454b_4_limb_9,
        single_karatsuba_n_7_output_tmp_1454b_4_limb_10,
        single_karatsuba_n_7_output_tmp_1454b_4_limb_11,
        single_karatsuba_n_7_output_tmp_1454b_4_limb_12,
        single_karatsuba_n_7_output_tmp_1454b_4_limb_13,
        single_karatsuba_n_7_output_tmp_1454b_4_limb_14,
        single_karatsuba_n_7_output_tmp_1454b_4_limb_15,
        single_karatsuba_n_7_output_tmp_1454b_4_limb_16,
        single_karatsuba_n_7_output_tmp_1454b_4_limb_17,
        single_karatsuba_n_7_output_tmp_1454b_4_limb_18,
        single_karatsuba_n_7_output_tmp_1454b_4_limb_19,
        single_karatsuba_n_7_output_tmp_1454b_4_limb_20,
        single_karatsuba_n_7_output_tmp_1454b_4_limb_21,
        single_karatsuba_n_7_output_tmp_1454b_4_limb_22,
        single_karatsuba_n_7_output_tmp_1454b_4_limb_23,
        single_karatsuba_n_7_output_tmp_1454b_4_limb_24,
        single_karatsuba_n_7_output_tmp_1454b_4_limb_25,
        single_karatsuba_n_7_output_tmp_1454b_4_limb_26,
    ] =
        single_karatsuba_n_7_evaluate(
        [
            double_karatsuba_1454b_input_limb_0, double_karatsuba_1454b_input_limb_1,
            double_karatsuba_1454b_input_limb_2, double_karatsuba_1454b_input_limb_3,
            double_karatsuba_1454b_input_limb_4, double_karatsuba_1454b_input_limb_5,
            double_karatsuba_1454b_input_limb_6, double_karatsuba_1454b_input_limb_7,
            double_karatsuba_1454b_input_limb_8, double_karatsuba_1454b_input_limb_9,
            double_karatsuba_1454b_input_limb_10, double_karatsuba_1454b_input_limb_11,
            double_karatsuba_1454b_input_limb_12, double_karatsuba_1454b_input_limb_13,
            double_karatsuba_1454b_input_limb_28, double_karatsuba_1454b_input_limb_29,
            double_karatsuba_1454b_input_limb_30, double_karatsuba_1454b_input_limb_31,
            double_karatsuba_1454b_input_limb_32, double_karatsuba_1454b_input_limb_33,
            double_karatsuba_1454b_input_limb_34, double_karatsuba_1454b_input_limb_35,
            double_karatsuba_1454b_input_limb_36, double_karatsuba_1454b_input_limb_37,
            double_karatsuba_1454b_input_limb_38, double_karatsuba_1454b_input_limb_39,
            double_karatsuba_1454b_input_limb_40, double_karatsuba_1454b_input_limb_41,
        ],
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );
    let [
        single_karatsuba_n_7_output_tmp_1454b_9_limb_0,
        single_karatsuba_n_7_output_tmp_1454b_9_limb_1,
        single_karatsuba_n_7_output_tmp_1454b_9_limb_2,
        single_karatsuba_n_7_output_tmp_1454b_9_limb_3,
        single_karatsuba_n_7_output_tmp_1454b_9_limb_4,
        single_karatsuba_n_7_output_tmp_1454b_9_limb_5,
        single_karatsuba_n_7_output_tmp_1454b_9_limb_6,
        single_karatsuba_n_7_output_tmp_1454b_9_limb_7,
        single_karatsuba_n_7_output_tmp_1454b_9_limb_8,
        single_karatsuba_n_7_output_tmp_1454b_9_limb_9,
        single_karatsuba_n_7_output_tmp_1454b_9_limb_10,
        single_karatsuba_n_7_output_tmp_1454b_9_limb_11,
        single_karatsuba_n_7_output_tmp_1454b_9_limb_12,
        single_karatsuba_n_7_output_tmp_1454b_9_limb_13,
        single_karatsuba_n_7_output_tmp_1454b_9_limb_14,
        single_karatsuba_n_7_output_tmp_1454b_9_limb_15,
        single_karatsuba_n_7_output_tmp_1454b_9_limb_16,
        single_karatsuba_n_7_output_tmp_1454b_9_limb_17,
        single_karatsuba_n_7_output_tmp_1454b_9_limb_18,
        single_karatsuba_n_7_output_tmp_1454b_9_limb_19,
        single_karatsuba_n_7_output_tmp_1454b_9_limb_20,
        single_karatsuba_n_7_output_tmp_1454b_9_limb_21,
        single_karatsuba_n_7_output_tmp_1454b_9_limb_22,
        single_karatsuba_n_7_output_tmp_1454b_9_limb_23,
        single_karatsuba_n_7_output_tmp_1454b_9_limb_24,
        single_karatsuba_n_7_output_tmp_1454b_9_limb_25,
        single_karatsuba_n_7_output_tmp_1454b_9_limb_26,
    ] =
        single_karatsuba_n_7_evaluate(
        [
            double_karatsuba_1454b_input_limb_14, double_karatsuba_1454b_input_limb_15,
            double_karatsuba_1454b_input_limb_16, double_karatsuba_1454b_input_limb_17,
            double_karatsuba_1454b_input_limb_18, double_karatsuba_1454b_input_limb_19,
            double_karatsuba_1454b_input_limb_20, double_karatsuba_1454b_input_limb_21,
            double_karatsuba_1454b_input_limb_22, double_karatsuba_1454b_input_limb_23,
            double_karatsuba_1454b_input_limb_24, double_karatsuba_1454b_input_limb_25,
            double_karatsuba_1454b_input_limb_26, double_karatsuba_1454b_input_limb_27,
            double_karatsuba_1454b_input_limb_42, double_karatsuba_1454b_input_limb_43,
            double_karatsuba_1454b_input_limb_44, double_karatsuba_1454b_input_limb_45,
            double_karatsuba_1454b_input_limb_46, double_karatsuba_1454b_input_limb_47,
            double_karatsuba_1454b_input_limb_48, double_karatsuba_1454b_input_limb_49,
            double_karatsuba_1454b_input_limb_50, double_karatsuba_1454b_input_limb_51,
            double_karatsuba_1454b_input_limb_52, double_karatsuba_1454b_input_limb_53,
            double_karatsuba_1454b_input_limb_54, double_karatsuba_1454b_input_limb_55,
        ],
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );
    let x_sum_tmp_1454b_10_limb_0: QM31 = (double_karatsuba_1454b_input_limb_0
        + double_karatsuba_1454b_input_limb_14);
    let x_sum_tmp_1454b_10_limb_1: QM31 = (double_karatsuba_1454b_input_limb_1
        + double_karatsuba_1454b_input_limb_15);
    let x_sum_tmp_1454b_10_limb_2: QM31 = (double_karatsuba_1454b_input_limb_2
        + double_karatsuba_1454b_input_limb_16);
    let x_sum_tmp_1454b_10_limb_3: QM31 = (double_karatsuba_1454b_input_limb_3
        + double_karatsuba_1454b_input_limb_17);
    let x_sum_tmp_1454b_10_limb_4: QM31 = (double_karatsuba_1454b_input_limb_4
        + double_karatsuba_1454b_input_limb_18);
    let x_sum_tmp_1454b_10_limb_5: QM31 = (double_karatsuba_1454b_input_limb_5
        + double_karatsuba_1454b_input_limb_19);
    let x_sum_tmp_1454b_10_limb_6: QM31 = (double_karatsuba_1454b_input_limb_6
        + double_karatsuba_1454b_input_limb_20);
    let x_sum_tmp_1454b_10_limb_7: QM31 = (double_karatsuba_1454b_input_limb_7
        + double_karatsuba_1454b_input_limb_21);
    let x_sum_tmp_1454b_10_limb_8: QM31 = (double_karatsuba_1454b_input_limb_8
        + double_karatsuba_1454b_input_limb_22);
    let x_sum_tmp_1454b_10_limb_9: QM31 = (double_karatsuba_1454b_input_limb_9
        + double_karatsuba_1454b_input_limb_23);
    let x_sum_tmp_1454b_10_limb_10: QM31 = (double_karatsuba_1454b_input_limb_10
        + double_karatsuba_1454b_input_limb_24);
    let x_sum_tmp_1454b_10_limb_11: QM31 = (double_karatsuba_1454b_input_limb_11
        + double_karatsuba_1454b_input_limb_25);
    let x_sum_tmp_1454b_10_limb_12: QM31 = (double_karatsuba_1454b_input_limb_12
        + double_karatsuba_1454b_input_limb_26);
    let x_sum_tmp_1454b_10_limb_13: QM31 = (double_karatsuba_1454b_input_limb_13
        + double_karatsuba_1454b_input_limb_27);
    let y_sum_tmp_1454b_11_limb_0: QM31 = (double_karatsuba_1454b_input_limb_28
        + double_karatsuba_1454b_input_limb_42);
    let y_sum_tmp_1454b_11_limb_1: QM31 = (double_karatsuba_1454b_input_limb_29
        + double_karatsuba_1454b_input_limb_43);
    let y_sum_tmp_1454b_11_limb_2: QM31 = (double_karatsuba_1454b_input_limb_30
        + double_karatsuba_1454b_input_limb_44);
    let y_sum_tmp_1454b_11_limb_3: QM31 = (double_karatsuba_1454b_input_limb_31
        + double_karatsuba_1454b_input_limb_45);
    let y_sum_tmp_1454b_11_limb_4: QM31 = (double_karatsuba_1454b_input_limb_32
        + double_karatsuba_1454b_input_limb_46);
    let y_sum_tmp_1454b_11_limb_5: QM31 = (double_karatsuba_1454b_input_limb_33
        + double_karatsuba_1454b_input_limb_47);
    let y_sum_tmp_1454b_11_limb_6: QM31 = (double_karatsuba_1454b_input_limb_34
        + double_karatsuba_1454b_input_limb_48);
    let y_sum_tmp_1454b_11_limb_7: QM31 = (double_karatsuba_1454b_input_limb_35
        + double_karatsuba_1454b_input_limb_49);
    let y_sum_tmp_1454b_11_limb_8: QM31 = (double_karatsuba_1454b_input_limb_36
        + double_karatsuba_1454b_input_limb_50);
    let y_sum_tmp_1454b_11_limb_9: QM31 = (double_karatsuba_1454b_input_limb_37
        + double_karatsuba_1454b_input_limb_51);
    let y_sum_tmp_1454b_11_limb_10: QM31 = (double_karatsuba_1454b_input_limb_38
        + double_karatsuba_1454b_input_limb_52);
    let y_sum_tmp_1454b_11_limb_11: QM31 = (double_karatsuba_1454b_input_limb_39
        + double_karatsuba_1454b_input_limb_53);
    let y_sum_tmp_1454b_11_limb_12: QM31 = (double_karatsuba_1454b_input_limb_40
        + double_karatsuba_1454b_input_limb_54);
    let y_sum_tmp_1454b_11_limb_13: QM31 = (double_karatsuba_1454b_input_limb_41
        + double_karatsuba_1454b_input_limb_55);
    let [
        single_karatsuba_n_7_output_tmp_1454b_16_limb_0,
        single_karatsuba_n_7_output_tmp_1454b_16_limb_1,
        single_karatsuba_n_7_output_tmp_1454b_16_limb_2,
        single_karatsuba_n_7_output_tmp_1454b_16_limb_3,
        single_karatsuba_n_7_output_tmp_1454b_16_limb_4,
        single_karatsuba_n_7_output_tmp_1454b_16_limb_5,
        single_karatsuba_n_7_output_tmp_1454b_16_limb_6,
        single_karatsuba_n_7_output_tmp_1454b_16_limb_7,
        single_karatsuba_n_7_output_tmp_1454b_16_limb_8,
        single_karatsuba_n_7_output_tmp_1454b_16_limb_9,
        single_karatsuba_n_7_output_tmp_1454b_16_limb_10,
        single_karatsuba_n_7_output_tmp_1454b_16_limb_11,
        single_karatsuba_n_7_output_tmp_1454b_16_limb_12,
        single_karatsuba_n_7_output_tmp_1454b_16_limb_13,
        single_karatsuba_n_7_output_tmp_1454b_16_limb_14,
        single_karatsuba_n_7_output_tmp_1454b_16_limb_15,
        single_karatsuba_n_7_output_tmp_1454b_16_limb_16,
        single_karatsuba_n_7_output_tmp_1454b_16_limb_17,
        single_karatsuba_n_7_output_tmp_1454b_16_limb_18,
        single_karatsuba_n_7_output_tmp_1454b_16_limb_19,
        single_karatsuba_n_7_output_tmp_1454b_16_limb_20,
        single_karatsuba_n_7_output_tmp_1454b_16_limb_21,
        single_karatsuba_n_7_output_tmp_1454b_16_limb_22,
        single_karatsuba_n_7_output_tmp_1454b_16_limb_23,
        single_karatsuba_n_7_output_tmp_1454b_16_limb_24,
        single_karatsuba_n_7_output_tmp_1454b_16_limb_25,
        single_karatsuba_n_7_output_tmp_1454b_16_limb_26,
    ] =
        single_karatsuba_n_7_evaluate(
        [
            x_sum_tmp_1454b_10_limb_0, x_sum_tmp_1454b_10_limb_1, x_sum_tmp_1454b_10_limb_2,
            x_sum_tmp_1454b_10_limb_3, x_sum_tmp_1454b_10_limb_4, x_sum_tmp_1454b_10_limb_5,
            x_sum_tmp_1454b_10_limb_6, x_sum_tmp_1454b_10_limb_7, x_sum_tmp_1454b_10_limb_8,
            x_sum_tmp_1454b_10_limb_9, x_sum_tmp_1454b_10_limb_10, x_sum_tmp_1454b_10_limb_11,
            x_sum_tmp_1454b_10_limb_12, x_sum_tmp_1454b_10_limb_13, y_sum_tmp_1454b_11_limb_0,
            y_sum_tmp_1454b_11_limb_1, y_sum_tmp_1454b_11_limb_2, y_sum_tmp_1454b_11_limb_3,
            y_sum_tmp_1454b_11_limb_4, y_sum_tmp_1454b_11_limb_5, y_sum_tmp_1454b_11_limb_6,
            y_sum_tmp_1454b_11_limb_7, y_sum_tmp_1454b_11_limb_8, y_sum_tmp_1454b_11_limb_9,
            y_sum_tmp_1454b_11_limb_10, y_sum_tmp_1454b_11_limb_11, y_sum_tmp_1454b_11_limb_12,
            y_sum_tmp_1454b_11_limb_13,
        ],
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    core::internal::revoke_ap_tracking();

    [
        single_karatsuba_n_7_output_tmp_1454b_4_limb_0,
        single_karatsuba_n_7_output_tmp_1454b_4_limb_1,
        single_karatsuba_n_7_output_tmp_1454b_4_limb_2,
        single_karatsuba_n_7_output_tmp_1454b_4_limb_3,
        single_karatsuba_n_7_output_tmp_1454b_4_limb_4,
        single_karatsuba_n_7_output_tmp_1454b_4_limb_5,
        single_karatsuba_n_7_output_tmp_1454b_4_limb_6,
        single_karatsuba_n_7_output_tmp_1454b_4_limb_7,
        single_karatsuba_n_7_output_tmp_1454b_4_limb_8,
        single_karatsuba_n_7_output_tmp_1454b_4_limb_9,
        single_karatsuba_n_7_output_tmp_1454b_4_limb_10,
        single_karatsuba_n_7_output_tmp_1454b_4_limb_11,
        single_karatsuba_n_7_output_tmp_1454b_4_limb_12,
        single_karatsuba_n_7_output_tmp_1454b_4_limb_13,
        (single_karatsuba_n_7_output_tmp_1454b_4_limb_14
            + ((single_karatsuba_n_7_output_tmp_1454b_16_limb_0
                - single_karatsuba_n_7_output_tmp_1454b_4_limb_0)
                - single_karatsuba_n_7_output_tmp_1454b_9_limb_0)),
        (single_karatsuba_n_7_output_tmp_1454b_4_limb_15
            + ((single_karatsuba_n_7_output_tmp_1454b_16_limb_1
                - single_karatsuba_n_7_output_tmp_1454b_4_limb_1)
                - single_karatsuba_n_7_output_tmp_1454b_9_limb_1)),
        (single_karatsuba_n_7_output_tmp_1454b_4_limb_16
            + ((single_karatsuba_n_7_output_tmp_1454b_16_limb_2
                - single_karatsuba_n_7_output_tmp_1454b_4_limb_2)
                - single_karatsuba_n_7_output_tmp_1454b_9_limb_2)),
        (single_karatsuba_n_7_output_tmp_1454b_4_limb_17
            + ((single_karatsuba_n_7_output_tmp_1454b_16_limb_3
                - single_karatsuba_n_7_output_tmp_1454b_4_limb_3)
                - single_karatsuba_n_7_output_tmp_1454b_9_limb_3)),
        (single_karatsuba_n_7_output_tmp_1454b_4_limb_18
            + ((single_karatsuba_n_7_output_tmp_1454b_16_limb_4
                - single_karatsuba_n_7_output_tmp_1454b_4_limb_4)
                - single_karatsuba_n_7_output_tmp_1454b_9_limb_4)),
        (single_karatsuba_n_7_output_tmp_1454b_4_limb_19
            + ((single_karatsuba_n_7_output_tmp_1454b_16_limb_5
                - single_karatsuba_n_7_output_tmp_1454b_4_limb_5)
                - single_karatsuba_n_7_output_tmp_1454b_9_limb_5)),
        (single_karatsuba_n_7_output_tmp_1454b_4_limb_20
            + ((single_karatsuba_n_7_output_tmp_1454b_16_limb_6
                - single_karatsuba_n_7_output_tmp_1454b_4_limb_6)
                - single_karatsuba_n_7_output_tmp_1454b_9_limb_6)),
        (single_karatsuba_n_7_output_tmp_1454b_4_limb_21
            + ((single_karatsuba_n_7_output_tmp_1454b_16_limb_7
                - single_karatsuba_n_7_output_tmp_1454b_4_limb_7)
                - single_karatsuba_n_7_output_tmp_1454b_9_limb_7)),
        (single_karatsuba_n_7_output_tmp_1454b_4_limb_22
            + ((single_karatsuba_n_7_output_tmp_1454b_16_limb_8
                - single_karatsuba_n_7_output_tmp_1454b_4_limb_8)
                - single_karatsuba_n_7_output_tmp_1454b_9_limb_8)),
        (single_karatsuba_n_7_output_tmp_1454b_4_limb_23
            + ((single_karatsuba_n_7_output_tmp_1454b_16_limb_9
                - single_karatsuba_n_7_output_tmp_1454b_4_limb_9)
                - single_karatsuba_n_7_output_tmp_1454b_9_limb_9)),
        (single_karatsuba_n_7_output_tmp_1454b_4_limb_24
            + ((single_karatsuba_n_7_output_tmp_1454b_16_limb_10
                - single_karatsuba_n_7_output_tmp_1454b_4_limb_10)
                - single_karatsuba_n_7_output_tmp_1454b_9_limb_10)),
        (single_karatsuba_n_7_output_tmp_1454b_4_limb_25
            + ((single_karatsuba_n_7_output_tmp_1454b_16_limb_11
                - single_karatsuba_n_7_output_tmp_1454b_4_limb_11)
                - single_karatsuba_n_7_output_tmp_1454b_9_limb_11)),
        (single_karatsuba_n_7_output_tmp_1454b_4_limb_26
            + ((single_karatsuba_n_7_output_tmp_1454b_16_limb_12
                - single_karatsuba_n_7_output_tmp_1454b_4_limb_12)
                - single_karatsuba_n_7_output_tmp_1454b_9_limb_12)),
        ((single_karatsuba_n_7_output_tmp_1454b_16_limb_13
            - single_karatsuba_n_7_output_tmp_1454b_4_limb_13)
            - single_karatsuba_n_7_output_tmp_1454b_9_limb_13),
        (single_karatsuba_n_7_output_tmp_1454b_9_limb_0
            + ((single_karatsuba_n_7_output_tmp_1454b_16_limb_14
                - single_karatsuba_n_7_output_tmp_1454b_4_limb_14)
                - single_karatsuba_n_7_output_tmp_1454b_9_limb_14)),
        (single_karatsuba_n_7_output_tmp_1454b_9_limb_1
            + ((single_karatsuba_n_7_output_tmp_1454b_16_limb_15
                - single_karatsuba_n_7_output_tmp_1454b_4_limb_15)
                - single_karatsuba_n_7_output_tmp_1454b_9_limb_15)),
        (single_karatsuba_n_7_output_tmp_1454b_9_limb_2
            + ((single_karatsuba_n_7_output_tmp_1454b_16_limb_16
                - single_karatsuba_n_7_output_tmp_1454b_4_limb_16)
                - single_karatsuba_n_7_output_tmp_1454b_9_limb_16)),
        (single_karatsuba_n_7_output_tmp_1454b_9_limb_3
            + ((single_karatsuba_n_7_output_tmp_1454b_16_limb_17
                - single_karatsuba_n_7_output_tmp_1454b_4_limb_17)
                - single_karatsuba_n_7_output_tmp_1454b_9_limb_17)),
        (single_karatsuba_n_7_output_tmp_1454b_9_limb_4
            + ((single_karatsuba_n_7_output_tmp_1454b_16_limb_18
                - single_karatsuba_n_7_output_tmp_1454b_4_limb_18)
                - single_karatsuba_n_7_output_tmp_1454b_9_limb_18)),
        (single_karatsuba_n_7_output_tmp_1454b_9_limb_5
            + ((single_karatsuba_n_7_output_tmp_1454b_16_limb_19
                - single_karatsuba_n_7_output_tmp_1454b_4_limb_19)
                - single_karatsuba_n_7_output_tmp_1454b_9_limb_19)),
        (single_karatsuba_n_7_output_tmp_1454b_9_limb_6
            + ((single_karatsuba_n_7_output_tmp_1454b_16_limb_20
                - single_karatsuba_n_7_output_tmp_1454b_4_limb_20)
                - single_karatsuba_n_7_output_tmp_1454b_9_limb_20)),
        (single_karatsuba_n_7_output_tmp_1454b_9_limb_7
            + ((single_karatsuba_n_7_output_tmp_1454b_16_limb_21
                - single_karatsuba_n_7_output_tmp_1454b_4_limb_21)
                - single_karatsuba_n_7_output_tmp_1454b_9_limb_21)),
        (single_karatsuba_n_7_output_tmp_1454b_9_limb_8
            + ((single_karatsuba_n_7_output_tmp_1454b_16_limb_22
                - single_karatsuba_n_7_output_tmp_1454b_4_limb_22)
                - single_karatsuba_n_7_output_tmp_1454b_9_limb_22)),
        (single_karatsuba_n_7_output_tmp_1454b_9_limb_9
            + ((single_karatsuba_n_7_output_tmp_1454b_16_limb_23
                - single_karatsuba_n_7_output_tmp_1454b_4_limb_23)
                - single_karatsuba_n_7_output_tmp_1454b_9_limb_23)),
        (single_karatsuba_n_7_output_tmp_1454b_9_limb_10
            + ((single_karatsuba_n_7_output_tmp_1454b_16_limb_24
                - single_karatsuba_n_7_output_tmp_1454b_4_limb_24)
                - single_karatsuba_n_7_output_tmp_1454b_9_limb_24)),
        (single_karatsuba_n_7_output_tmp_1454b_9_limb_11
            + ((single_karatsuba_n_7_output_tmp_1454b_16_limb_25
                - single_karatsuba_n_7_output_tmp_1454b_4_limb_25)
                - single_karatsuba_n_7_output_tmp_1454b_9_limb_25)),
        (single_karatsuba_n_7_output_tmp_1454b_9_limb_12
            + ((single_karatsuba_n_7_output_tmp_1454b_16_limb_26
                - single_karatsuba_n_7_output_tmp_1454b_4_limb_26)
                - single_karatsuba_n_7_output_tmp_1454b_9_limb_26)),
        single_karatsuba_n_7_output_tmp_1454b_9_limb_13,
        single_karatsuba_n_7_output_tmp_1454b_9_limb_14,
        single_karatsuba_n_7_output_tmp_1454b_9_limb_15,
        single_karatsuba_n_7_output_tmp_1454b_9_limb_16,
        single_karatsuba_n_7_output_tmp_1454b_9_limb_17,
        single_karatsuba_n_7_output_tmp_1454b_9_limb_18,
        single_karatsuba_n_7_output_tmp_1454b_9_limb_19,
        single_karatsuba_n_7_output_tmp_1454b_9_limb_20,
        single_karatsuba_n_7_output_tmp_1454b_9_limb_21,
        single_karatsuba_n_7_output_tmp_1454b_9_limb_22,
        single_karatsuba_n_7_output_tmp_1454b_9_limb_23,
        single_karatsuba_n_7_output_tmp_1454b_9_limb_24,
        single_karatsuba_n_7_output_tmp_1454b_9_limb_25,
        single_karatsuba_n_7_output_tmp_1454b_9_limb_26,
    ]
}
