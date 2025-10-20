// This file was created by the AIR team.

use crate::prelude::*;


pub fn single_karatsuba_n_7_evaluate(
    input: [QM31; 28], ref sum: QM31, domain_vanishing_eval_inv: QM31, random_coeff: QM31,
) -> [QM31; 27] {
    let [
        single_karatsuba_n_7_input_limb_0,
        single_karatsuba_n_7_input_limb_1,
        single_karatsuba_n_7_input_limb_2,
        single_karatsuba_n_7_input_limb_3,
        single_karatsuba_n_7_input_limb_4,
        single_karatsuba_n_7_input_limb_5,
        single_karatsuba_n_7_input_limb_6,
        single_karatsuba_n_7_input_limb_7,
        single_karatsuba_n_7_input_limb_8,
        single_karatsuba_n_7_input_limb_9,
        single_karatsuba_n_7_input_limb_10,
        single_karatsuba_n_7_input_limb_11,
        single_karatsuba_n_7_input_limb_12,
        single_karatsuba_n_7_input_limb_13,
        single_karatsuba_n_7_input_limb_14,
        single_karatsuba_n_7_input_limb_15,
        single_karatsuba_n_7_input_limb_16,
        single_karatsuba_n_7_input_limb_17,
        single_karatsuba_n_7_input_limb_18,
        single_karatsuba_n_7_input_limb_19,
        single_karatsuba_n_7_input_limb_20,
        single_karatsuba_n_7_input_limb_21,
        single_karatsuba_n_7_input_limb_22,
        single_karatsuba_n_7_input_limb_23,
        single_karatsuba_n_7_input_limb_24,
        single_karatsuba_n_7_input_limb_25,
        single_karatsuba_n_7_input_limb_26,
        single_karatsuba_n_7_input_limb_27,
    ] =
        input;

    core::internal::revoke_ap_tracking();

    let z0_tmp_90dd9_0_limb_0: QM31 = (single_karatsuba_n_7_input_limb_0
        * single_karatsuba_n_7_input_limb_14);
    let z0_tmp_90dd9_0_limb_1: QM31 = ((single_karatsuba_n_7_input_limb_0
        * single_karatsuba_n_7_input_limb_15)
        + (single_karatsuba_n_7_input_limb_1 * single_karatsuba_n_7_input_limb_14));
    let z0_tmp_90dd9_0_limb_2: QM31 = (((single_karatsuba_n_7_input_limb_0
        * single_karatsuba_n_7_input_limb_16)
        + (single_karatsuba_n_7_input_limb_1 * single_karatsuba_n_7_input_limb_15))
        + (single_karatsuba_n_7_input_limb_2 * single_karatsuba_n_7_input_limb_14));
    let z0_tmp_90dd9_0_limb_3: QM31 = ((((single_karatsuba_n_7_input_limb_0
        * single_karatsuba_n_7_input_limb_17)
        + (single_karatsuba_n_7_input_limb_1 * single_karatsuba_n_7_input_limb_16))
        + (single_karatsuba_n_7_input_limb_2 * single_karatsuba_n_7_input_limb_15))
        + (single_karatsuba_n_7_input_limb_3 * single_karatsuba_n_7_input_limb_14));
    let z0_tmp_90dd9_0_limb_4: QM31 = (((((single_karatsuba_n_7_input_limb_0
        * single_karatsuba_n_7_input_limb_18)
        + (single_karatsuba_n_7_input_limb_1 * single_karatsuba_n_7_input_limb_17))
        + (single_karatsuba_n_7_input_limb_2 * single_karatsuba_n_7_input_limb_16))
        + (single_karatsuba_n_7_input_limb_3 * single_karatsuba_n_7_input_limb_15))
        + (single_karatsuba_n_7_input_limb_4 * single_karatsuba_n_7_input_limb_14));
    let z0_tmp_90dd9_0_limb_5: QM31 = ((((((single_karatsuba_n_7_input_limb_0
        * single_karatsuba_n_7_input_limb_19)
        + (single_karatsuba_n_7_input_limb_1 * single_karatsuba_n_7_input_limb_18))
        + (single_karatsuba_n_7_input_limb_2 * single_karatsuba_n_7_input_limb_17))
        + (single_karatsuba_n_7_input_limb_3 * single_karatsuba_n_7_input_limb_16))
        + (single_karatsuba_n_7_input_limb_4 * single_karatsuba_n_7_input_limb_15))
        + (single_karatsuba_n_7_input_limb_5 * single_karatsuba_n_7_input_limb_14));
    let z0_tmp_90dd9_0_limb_6: QM31 = (((((((single_karatsuba_n_7_input_limb_0
        * single_karatsuba_n_7_input_limb_20)
        + (single_karatsuba_n_7_input_limb_1 * single_karatsuba_n_7_input_limb_19))
        + (single_karatsuba_n_7_input_limb_2 * single_karatsuba_n_7_input_limb_18))
        + (single_karatsuba_n_7_input_limb_3 * single_karatsuba_n_7_input_limb_17))
        + (single_karatsuba_n_7_input_limb_4 * single_karatsuba_n_7_input_limb_16))
        + (single_karatsuba_n_7_input_limb_5 * single_karatsuba_n_7_input_limb_15))
        + (single_karatsuba_n_7_input_limb_6 * single_karatsuba_n_7_input_limb_14));
    let z0_tmp_90dd9_0_limb_7: QM31 = ((((((single_karatsuba_n_7_input_limb_1
        * single_karatsuba_n_7_input_limb_20)
        + (single_karatsuba_n_7_input_limb_2 * single_karatsuba_n_7_input_limb_19))
        + (single_karatsuba_n_7_input_limb_3 * single_karatsuba_n_7_input_limb_18))
        + (single_karatsuba_n_7_input_limb_4 * single_karatsuba_n_7_input_limb_17))
        + (single_karatsuba_n_7_input_limb_5 * single_karatsuba_n_7_input_limb_16))
        + (single_karatsuba_n_7_input_limb_6 * single_karatsuba_n_7_input_limb_15));
    let z0_tmp_90dd9_0_limb_8: QM31 = (((((single_karatsuba_n_7_input_limb_2
        * single_karatsuba_n_7_input_limb_20)
        + (single_karatsuba_n_7_input_limb_3 * single_karatsuba_n_7_input_limb_19))
        + (single_karatsuba_n_7_input_limb_4 * single_karatsuba_n_7_input_limb_18))
        + (single_karatsuba_n_7_input_limb_5 * single_karatsuba_n_7_input_limb_17))
        + (single_karatsuba_n_7_input_limb_6 * single_karatsuba_n_7_input_limb_16));
    let z0_tmp_90dd9_0_limb_9: QM31 = ((((single_karatsuba_n_7_input_limb_3
        * single_karatsuba_n_7_input_limb_20)
        + (single_karatsuba_n_7_input_limb_4 * single_karatsuba_n_7_input_limb_19))
        + (single_karatsuba_n_7_input_limb_5 * single_karatsuba_n_7_input_limb_18))
        + (single_karatsuba_n_7_input_limb_6 * single_karatsuba_n_7_input_limb_17));
    let z0_tmp_90dd9_0_limb_10: QM31 = (((single_karatsuba_n_7_input_limb_4
        * single_karatsuba_n_7_input_limb_20)
        + (single_karatsuba_n_7_input_limb_5 * single_karatsuba_n_7_input_limb_19))
        + (single_karatsuba_n_7_input_limb_6 * single_karatsuba_n_7_input_limb_18));
    let z0_tmp_90dd9_0_limb_11: QM31 = ((single_karatsuba_n_7_input_limb_5
        * single_karatsuba_n_7_input_limb_20)
        + (single_karatsuba_n_7_input_limb_6 * single_karatsuba_n_7_input_limb_19));
    let z0_tmp_90dd9_0_limb_12: QM31 = (single_karatsuba_n_7_input_limb_6
        * single_karatsuba_n_7_input_limb_20);
    let z2_tmp_90dd9_1_limb_0: QM31 = (single_karatsuba_n_7_input_limb_7
        * single_karatsuba_n_7_input_limb_21);
    let z2_tmp_90dd9_1_limb_1: QM31 = ((single_karatsuba_n_7_input_limb_7
        * single_karatsuba_n_7_input_limb_22)
        + (single_karatsuba_n_7_input_limb_8 * single_karatsuba_n_7_input_limb_21));
    let z2_tmp_90dd9_1_limb_2: QM31 = (((single_karatsuba_n_7_input_limb_7
        * single_karatsuba_n_7_input_limb_23)
        + (single_karatsuba_n_7_input_limb_8 * single_karatsuba_n_7_input_limb_22))
        + (single_karatsuba_n_7_input_limb_9 * single_karatsuba_n_7_input_limb_21));
    let z2_tmp_90dd9_1_limb_3: QM31 = ((((single_karatsuba_n_7_input_limb_7
        * single_karatsuba_n_7_input_limb_24)
        + (single_karatsuba_n_7_input_limb_8 * single_karatsuba_n_7_input_limb_23))
        + (single_karatsuba_n_7_input_limb_9 * single_karatsuba_n_7_input_limb_22))
        + (single_karatsuba_n_7_input_limb_10 * single_karatsuba_n_7_input_limb_21));
    let z2_tmp_90dd9_1_limb_4: QM31 = (((((single_karatsuba_n_7_input_limb_7
        * single_karatsuba_n_7_input_limb_25)
        + (single_karatsuba_n_7_input_limb_8 * single_karatsuba_n_7_input_limb_24))
        + (single_karatsuba_n_7_input_limb_9 * single_karatsuba_n_7_input_limb_23))
        + (single_karatsuba_n_7_input_limb_10 * single_karatsuba_n_7_input_limb_22))
        + (single_karatsuba_n_7_input_limb_11 * single_karatsuba_n_7_input_limb_21));
    let z2_tmp_90dd9_1_limb_5: QM31 = ((((((single_karatsuba_n_7_input_limb_7
        * single_karatsuba_n_7_input_limb_26)
        + (single_karatsuba_n_7_input_limb_8 * single_karatsuba_n_7_input_limb_25))
        + (single_karatsuba_n_7_input_limb_9 * single_karatsuba_n_7_input_limb_24))
        + (single_karatsuba_n_7_input_limb_10 * single_karatsuba_n_7_input_limb_23))
        + (single_karatsuba_n_7_input_limb_11 * single_karatsuba_n_7_input_limb_22))
        + (single_karatsuba_n_7_input_limb_12 * single_karatsuba_n_7_input_limb_21));
    let z2_tmp_90dd9_1_limb_6: QM31 = (((((((single_karatsuba_n_7_input_limb_7
        * single_karatsuba_n_7_input_limb_27)
        + (single_karatsuba_n_7_input_limb_8 * single_karatsuba_n_7_input_limb_26))
        + (single_karatsuba_n_7_input_limb_9 * single_karatsuba_n_7_input_limb_25))
        + (single_karatsuba_n_7_input_limb_10 * single_karatsuba_n_7_input_limb_24))
        + (single_karatsuba_n_7_input_limb_11 * single_karatsuba_n_7_input_limb_23))
        + (single_karatsuba_n_7_input_limb_12 * single_karatsuba_n_7_input_limb_22))
        + (single_karatsuba_n_7_input_limb_13 * single_karatsuba_n_7_input_limb_21));
    let z2_tmp_90dd9_1_limb_7: QM31 = ((((((single_karatsuba_n_7_input_limb_8
        * single_karatsuba_n_7_input_limb_27)
        + (single_karatsuba_n_7_input_limb_9 * single_karatsuba_n_7_input_limb_26))
        + (single_karatsuba_n_7_input_limb_10 * single_karatsuba_n_7_input_limb_25))
        + (single_karatsuba_n_7_input_limb_11 * single_karatsuba_n_7_input_limb_24))
        + (single_karatsuba_n_7_input_limb_12 * single_karatsuba_n_7_input_limb_23))
        + (single_karatsuba_n_7_input_limb_13 * single_karatsuba_n_7_input_limb_22));
    let z2_tmp_90dd9_1_limb_8: QM31 = (((((single_karatsuba_n_7_input_limb_9
        * single_karatsuba_n_7_input_limb_27)
        + (single_karatsuba_n_7_input_limb_10 * single_karatsuba_n_7_input_limb_26))
        + (single_karatsuba_n_7_input_limb_11 * single_karatsuba_n_7_input_limb_25))
        + (single_karatsuba_n_7_input_limb_12 * single_karatsuba_n_7_input_limb_24))
        + (single_karatsuba_n_7_input_limb_13 * single_karatsuba_n_7_input_limb_23));
    let z2_tmp_90dd9_1_limb_9: QM31 = ((((single_karatsuba_n_7_input_limb_10
        * single_karatsuba_n_7_input_limb_27)
        + (single_karatsuba_n_7_input_limb_11 * single_karatsuba_n_7_input_limb_26))
        + (single_karatsuba_n_7_input_limb_12 * single_karatsuba_n_7_input_limb_25))
        + (single_karatsuba_n_7_input_limb_13 * single_karatsuba_n_7_input_limb_24));
    let z2_tmp_90dd9_1_limb_10: QM31 = (((single_karatsuba_n_7_input_limb_11
        * single_karatsuba_n_7_input_limb_27)
        + (single_karatsuba_n_7_input_limb_12 * single_karatsuba_n_7_input_limb_26))
        + (single_karatsuba_n_7_input_limb_13 * single_karatsuba_n_7_input_limb_25));
    let z2_tmp_90dd9_1_limb_11: QM31 = ((single_karatsuba_n_7_input_limb_12
        * single_karatsuba_n_7_input_limb_27)
        + (single_karatsuba_n_7_input_limb_13 * single_karatsuba_n_7_input_limb_26));
    let z2_tmp_90dd9_1_limb_12: QM31 = (single_karatsuba_n_7_input_limb_13
        * single_karatsuba_n_7_input_limb_27);
    let x_sum_tmp_90dd9_2_limb_0: QM31 = (single_karatsuba_n_7_input_limb_0
        + single_karatsuba_n_7_input_limb_7);
    let x_sum_tmp_90dd9_2_limb_1: QM31 = (single_karatsuba_n_7_input_limb_1
        + single_karatsuba_n_7_input_limb_8);
    let x_sum_tmp_90dd9_2_limb_2: QM31 = (single_karatsuba_n_7_input_limb_2
        + single_karatsuba_n_7_input_limb_9);
    let x_sum_tmp_90dd9_2_limb_3: QM31 = (single_karatsuba_n_7_input_limb_3
        + single_karatsuba_n_7_input_limb_10);
    let x_sum_tmp_90dd9_2_limb_4: QM31 = (single_karatsuba_n_7_input_limb_4
        + single_karatsuba_n_7_input_limb_11);
    let x_sum_tmp_90dd9_2_limb_5: QM31 = (single_karatsuba_n_7_input_limb_5
        + single_karatsuba_n_7_input_limb_12);
    let x_sum_tmp_90dd9_2_limb_6: QM31 = (single_karatsuba_n_7_input_limb_6
        + single_karatsuba_n_7_input_limb_13);
    let y_sum_tmp_90dd9_3_limb_0: QM31 = (single_karatsuba_n_7_input_limb_14
        + single_karatsuba_n_7_input_limb_21);
    let y_sum_tmp_90dd9_3_limb_1: QM31 = (single_karatsuba_n_7_input_limb_15
        + single_karatsuba_n_7_input_limb_22);
    let y_sum_tmp_90dd9_3_limb_2: QM31 = (single_karatsuba_n_7_input_limb_16
        + single_karatsuba_n_7_input_limb_23);
    let y_sum_tmp_90dd9_3_limb_3: QM31 = (single_karatsuba_n_7_input_limb_17
        + single_karatsuba_n_7_input_limb_24);
    let y_sum_tmp_90dd9_3_limb_4: QM31 = (single_karatsuba_n_7_input_limb_18
        + single_karatsuba_n_7_input_limb_25);
    let y_sum_tmp_90dd9_3_limb_5: QM31 = (single_karatsuba_n_7_input_limb_19
        + single_karatsuba_n_7_input_limb_26);
    let y_sum_tmp_90dd9_3_limb_6: QM31 = (single_karatsuba_n_7_input_limb_20
        + single_karatsuba_n_7_input_limb_27);

    core::internal::revoke_ap_tracking();

    [
        z0_tmp_90dd9_0_limb_0, z0_tmp_90dd9_0_limb_1, z0_tmp_90dd9_0_limb_2, z0_tmp_90dd9_0_limb_3,
        z0_tmp_90dd9_0_limb_4, z0_tmp_90dd9_0_limb_5, z0_tmp_90dd9_0_limb_6,
        (z0_tmp_90dd9_0_limb_7
            + (((x_sum_tmp_90dd9_2_limb_0 * y_sum_tmp_90dd9_3_limb_0) - z0_tmp_90dd9_0_limb_0)
                - z2_tmp_90dd9_1_limb_0)),
        (z0_tmp_90dd9_0_limb_8
            + ((((x_sum_tmp_90dd9_2_limb_0 * y_sum_tmp_90dd9_3_limb_1)
                + (x_sum_tmp_90dd9_2_limb_1 * y_sum_tmp_90dd9_3_limb_0))
                - z0_tmp_90dd9_0_limb_1)
                - z2_tmp_90dd9_1_limb_1)),
        (z0_tmp_90dd9_0_limb_9
            + (((((x_sum_tmp_90dd9_2_limb_0 * y_sum_tmp_90dd9_3_limb_2)
                + (x_sum_tmp_90dd9_2_limb_1 * y_sum_tmp_90dd9_3_limb_1))
                + (x_sum_tmp_90dd9_2_limb_2 * y_sum_tmp_90dd9_3_limb_0))
                - z0_tmp_90dd9_0_limb_2)
                - z2_tmp_90dd9_1_limb_2)),
        (z0_tmp_90dd9_0_limb_10
            + ((((((x_sum_tmp_90dd9_2_limb_0 * y_sum_tmp_90dd9_3_limb_3)
                + (x_sum_tmp_90dd9_2_limb_1 * y_sum_tmp_90dd9_3_limb_2))
                + (x_sum_tmp_90dd9_2_limb_2 * y_sum_tmp_90dd9_3_limb_1))
                + (x_sum_tmp_90dd9_2_limb_3 * y_sum_tmp_90dd9_3_limb_0))
                - z0_tmp_90dd9_0_limb_3)
                - z2_tmp_90dd9_1_limb_3)),
        (z0_tmp_90dd9_0_limb_11
            + (((((((x_sum_tmp_90dd9_2_limb_0 * y_sum_tmp_90dd9_3_limb_4)
                + (x_sum_tmp_90dd9_2_limb_1 * y_sum_tmp_90dd9_3_limb_3))
                + (x_sum_tmp_90dd9_2_limb_2 * y_sum_tmp_90dd9_3_limb_2))
                + (x_sum_tmp_90dd9_2_limb_3 * y_sum_tmp_90dd9_3_limb_1))
                + (x_sum_tmp_90dd9_2_limb_4 * y_sum_tmp_90dd9_3_limb_0))
                - z0_tmp_90dd9_0_limb_4)
                - z2_tmp_90dd9_1_limb_4)),
        (z0_tmp_90dd9_0_limb_12
            + ((((((((x_sum_tmp_90dd9_2_limb_0 * y_sum_tmp_90dd9_3_limb_5)
                + (x_sum_tmp_90dd9_2_limb_1 * y_sum_tmp_90dd9_3_limb_4))
                + (x_sum_tmp_90dd9_2_limb_2 * y_sum_tmp_90dd9_3_limb_3))
                + (x_sum_tmp_90dd9_2_limb_3 * y_sum_tmp_90dd9_3_limb_2))
                + (x_sum_tmp_90dd9_2_limb_4 * y_sum_tmp_90dd9_3_limb_1))
                + (x_sum_tmp_90dd9_2_limb_5 * y_sum_tmp_90dd9_3_limb_0))
                - z0_tmp_90dd9_0_limb_5)
                - z2_tmp_90dd9_1_limb_5)),
        (((((((((x_sum_tmp_90dd9_2_limb_0 * y_sum_tmp_90dd9_3_limb_6)
            + (x_sum_tmp_90dd9_2_limb_1 * y_sum_tmp_90dd9_3_limb_5))
            + (x_sum_tmp_90dd9_2_limb_2 * y_sum_tmp_90dd9_3_limb_4))
            + (x_sum_tmp_90dd9_2_limb_3 * y_sum_tmp_90dd9_3_limb_3))
            + (x_sum_tmp_90dd9_2_limb_4 * y_sum_tmp_90dd9_3_limb_2))
            + (x_sum_tmp_90dd9_2_limb_5 * y_sum_tmp_90dd9_3_limb_1))
            + (x_sum_tmp_90dd9_2_limb_6 * y_sum_tmp_90dd9_3_limb_0))
            - z0_tmp_90dd9_0_limb_6)
            - z2_tmp_90dd9_1_limb_6),
        (z2_tmp_90dd9_1_limb_0
            + ((((((((x_sum_tmp_90dd9_2_limb_1 * y_sum_tmp_90dd9_3_limb_6)
                + (x_sum_tmp_90dd9_2_limb_2 * y_sum_tmp_90dd9_3_limb_5))
                + (x_sum_tmp_90dd9_2_limb_3 * y_sum_tmp_90dd9_3_limb_4))
                + (x_sum_tmp_90dd9_2_limb_4 * y_sum_tmp_90dd9_3_limb_3))
                + (x_sum_tmp_90dd9_2_limb_5 * y_sum_tmp_90dd9_3_limb_2))
                + (x_sum_tmp_90dd9_2_limb_6 * y_sum_tmp_90dd9_3_limb_1))
                - z0_tmp_90dd9_0_limb_7)
                - z2_tmp_90dd9_1_limb_7)),
        (z2_tmp_90dd9_1_limb_1
            + (((((((x_sum_tmp_90dd9_2_limb_2 * y_sum_tmp_90dd9_3_limb_6)
                + (x_sum_tmp_90dd9_2_limb_3 * y_sum_tmp_90dd9_3_limb_5))
                + (x_sum_tmp_90dd9_2_limb_4 * y_sum_tmp_90dd9_3_limb_4))
                + (x_sum_tmp_90dd9_2_limb_5 * y_sum_tmp_90dd9_3_limb_3))
                + (x_sum_tmp_90dd9_2_limb_6 * y_sum_tmp_90dd9_3_limb_2))
                - z0_tmp_90dd9_0_limb_8)
                - z2_tmp_90dd9_1_limb_8)),
        (z2_tmp_90dd9_1_limb_2
            + ((((((x_sum_tmp_90dd9_2_limb_3 * y_sum_tmp_90dd9_3_limb_6)
                + (x_sum_tmp_90dd9_2_limb_4 * y_sum_tmp_90dd9_3_limb_5))
                + (x_sum_tmp_90dd9_2_limb_5 * y_sum_tmp_90dd9_3_limb_4))
                + (x_sum_tmp_90dd9_2_limb_6 * y_sum_tmp_90dd9_3_limb_3))
                - z0_tmp_90dd9_0_limb_9)
                - z2_tmp_90dd9_1_limb_9)),
        (z2_tmp_90dd9_1_limb_3
            + (((((x_sum_tmp_90dd9_2_limb_4 * y_sum_tmp_90dd9_3_limb_6)
                + (x_sum_tmp_90dd9_2_limb_5 * y_sum_tmp_90dd9_3_limb_5))
                + (x_sum_tmp_90dd9_2_limb_6 * y_sum_tmp_90dd9_3_limb_4))
                - z0_tmp_90dd9_0_limb_10)
                - z2_tmp_90dd9_1_limb_10)),
        (z2_tmp_90dd9_1_limb_4
            + ((((x_sum_tmp_90dd9_2_limb_5 * y_sum_tmp_90dd9_3_limb_6)
                + (x_sum_tmp_90dd9_2_limb_6 * y_sum_tmp_90dd9_3_limb_5))
                - z0_tmp_90dd9_0_limb_11)
                - z2_tmp_90dd9_1_limb_11)),
        (z2_tmp_90dd9_1_limb_5
            + (((x_sum_tmp_90dd9_2_limb_6 * y_sum_tmp_90dd9_3_limb_6) - z0_tmp_90dd9_0_limb_12)
                - z2_tmp_90dd9_1_limb_12)),
        z2_tmp_90dd9_1_limb_6, z2_tmp_90dd9_1_limb_7, z2_tmp_90dd9_1_limb_8, z2_tmp_90dd9_1_limb_9,
        z2_tmp_90dd9_1_limb_10, z2_tmp_90dd9_1_limb_11, z2_tmp_90dd9_1_limb_12,
    ]
}
