// This file was created by the AIR team.

use crate::prelude::*;


pub fn qm_31_into_u_32_evaluate(
    input: [QM31; 17],
    limbi_low_col0: QM31,
    limbi_high_col1: QM31,
    limbi_inv_or_one_col2: QM31,
    limbi_low_col3: QM31,
    limbi_high_col4: QM31,
    limbi_inv_or_one_col5: QM31,
    limbi_low_col6: QM31,
    limbi_high_col7: QM31,
    limbi_inv_or_one_col8: QM31,
    limbi_low_col9: QM31,
    limbi_high_col10: QM31,
    limbi_inv_or_one_col11: QM31,
    limbi_low_col12: QM31,
    limbi_high_col13: QM31,
    limbi_inv_or_one_col14: QM31,
    limbi_low_col15: QM31,
    limbi_high_col16: QM31,
    limbi_inv_or_one_col17: QM31,
    limbi_low_col18: QM31,
    limbi_high_col19: QM31,
    limbi_inv_or_one_col20: QM31,
    limbi_low_col21: QM31,
    limbi_high_col22: QM31,
    limbi_inv_or_one_col23: QM31,
    limbi_low_col24: QM31,
    limbi_high_col25: QM31,
    limbi_inv_or_one_col26: QM31,
    limbi_low_col27: QM31,
    limbi_high_col28: QM31,
    limbi_inv_or_one_col29: QM31,
    limbi_low_col30: QM31,
    limbi_high_col31: QM31,
    limbi_inv_or_one_col32: QM31,
    limbi_low_col33: QM31,
    limbi_high_col34: QM31,
    limbi_inv_or_one_col35: QM31,
    limbi_low_col36: QM31,
    limbi_high_col37: QM31,
    limbi_inv_or_one_col38: QM31,
    limbi_low_col39: QM31,
    limbi_high_col40: QM31,
    limbi_inv_or_one_col41: QM31,
    limbi_low_col42: QM31,
    limbi_high_col43: QM31,
    limbi_inv_or_one_col44: QM31,
    limbi_low_col45: QM31,
    limbi_high_col46: QM31,
    limbi_inv_or_one_col47: QM31,
    enabler: QM31,
    common_lookup_elements: @CommonLookupElements,
    ref range_check_16_sum_0: QM31,
    ref numerator_0: QM31,
    ref range_check_15_sum_1: QM31,
    ref numerator_1: QM31,
    ref blake_message_sum_2: QM31,
    ref numerator_2: QM31,
    ref range_check_16_sum_3: QM31,
    ref numerator_3: QM31,
    ref range_check_15_sum_4: QM31,
    ref numerator_4: QM31,
    ref blake_message_sum_5: QM31,
    ref numerator_5: QM31,
    ref range_check_16_sum_6: QM31,
    ref numerator_6: QM31,
    ref range_check_15_sum_7: QM31,
    ref numerator_7: QM31,
    ref blake_message_sum_8: QM31,
    ref numerator_8: QM31,
    ref range_check_16_sum_9: QM31,
    ref numerator_9: QM31,
    ref range_check_15_sum_10: QM31,
    ref numerator_10: QM31,
    ref blake_message_sum_11: QM31,
    ref numerator_11: QM31,
    ref range_check_16_sum_12: QM31,
    ref numerator_12: QM31,
    ref range_check_15_sum_13: QM31,
    ref numerator_13: QM31,
    ref blake_message_sum_14: QM31,
    ref numerator_14: QM31,
    ref range_check_16_sum_15: QM31,
    ref numerator_15: QM31,
    ref range_check_15_sum_16: QM31,
    ref numerator_16: QM31,
    ref blake_message_sum_17: QM31,
    ref numerator_17: QM31,
    ref range_check_16_sum_18: QM31,
    ref numerator_18: QM31,
    ref range_check_15_sum_19: QM31,
    ref numerator_19: QM31,
    ref blake_message_sum_20: QM31,
    ref numerator_20: QM31,
    ref range_check_16_sum_21: QM31,
    ref numerator_21: QM31,
    ref range_check_15_sum_22: QM31,
    ref numerator_22: QM31,
    ref blake_message_sum_23: QM31,
    ref numerator_23: QM31,
    ref range_check_16_sum_24: QM31,
    ref numerator_24: QM31,
    ref range_check_15_sum_25: QM31,
    ref numerator_25: QM31,
    ref blake_message_sum_26: QM31,
    ref numerator_26: QM31,
    ref range_check_16_sum_27: QM31,
    ref numerator_27: QM31,
    ref range_check_15_sum_28: QM31,
    ref numerator_28: QM31,
    ref blake_message_sum_29: QM31,
    ref numerator_29: QM31,
    ref range_check_16_sum_30: QM31,
    ref numerator_30: QM31,
    ref range_check_15_sum_31: QM31,
    ref numerator_31: QM31,
    ref blake_message_sum_32: QM31,
    ref numerator_32: QM31,
    ref range_check_16_sum_33: QM31,
    ref numerator_33: QM31,
    ref range_check_15_sum_34: QM31,
    ref numerator_34: QM31,
    ref blake_message_sum_35: QM31,
    ref numerator_35: QM31,
    ref range_check_16_sum_36: QM31,
    ref numerator_36: QM31,
    ref range_check_15_sum_37: QM31,
    ref numerator_37: QM31,
    ref blake_message_sum_38: QM31,
    ref numerator_38: QM31,
    ref range_check_16_sum_39: QM31,
    ref numerator_39: QM31,
    ref range_check_15_sum_40: QM31,
    ref numerator_40: QM31,
    ref blake_message_sum_41: QM31,
    ref numerator_41: QM31,
    ref range_check_16_sum_42: QM31,
    ref numerator_42: QM31,
    ref range_check_15_sum_43: QM31,
    ref numerator_43: QM31,
    ref blake_message_sum_44: QM31,
    ref numerator_44: QM31,
    ref range_check_16_sum_45: QM31,
    ref numerator_45: QM31,
    ref range_check_15_sum_46: QM31,
    ref numerator_46: QM31,
    ref blake_message_sum_47: QM31,
    ref numerator_47: QM31,
    ref sum: QM31,
    random_coeff: QM31,
) -> [QM31; 0] {
    let [
        qm_31_into_u_32_input_limb_0,
        qm_31_into_u_32_input_limb_1,
        qm_31_into_u_32_input_limb_2,
        qm_31_into_u_32_input_limb_3,
        qm_31_into_u_32_input_limb_4,
        qm_31_into_u_32_input_limb_5,
        qm_31_into_u_32_input_limb_6,
        qm_31_into_u_32_input_limb_7,
        qm_31_into_u_32_input_limb_8,
        qm_31_into_u_32_input_limb_9,
        qm_31_into_u_32_input_limb_10,
        qm_31_into_u_32_input_limb_11,
        qm_31_into_u_32_input_limb_12,
        qm_31_into_u_32_input_limb_13,
        qm_31_into_u_32_input_limb_14,
        qm_31_into_u_32_input_limb_15,
        qm_31_into_u_32_input_limb_16,
    ] =
        input;

    range_check_16_sum_0 = common_lookup_elements
        .combine_qm31([qm31_const::<1008385708, 0, 0, 0>(), limbi_low_col0].span());
    numerator_0 = enabler;

    range_check_15_sum_1 = common_lookup_elements
        .combine_qm31([qm31_const::<1058718565, 0, 0, 0>(), limbi_high_col1].span());
    numerator_1 = enabler;

    // Constraint - limbi is zero then limbi_low is zero
    let constraint_quotient = ((((qm_31_into_u_32_input_limb_0 * limbi_inv_or_one_col2)
        - qm31_const::<1, 0, 0, 0>())
        * limbi_low_col0));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - limb 0 reconstruction
    let constraint_quotient = ((qm_31_into_u_32_input_limb_0
        - (limbi_low_col0 + (limbi_high_col1 * qm31_const::<65536, 0, 0, 0>()))));
    sum = sum * random_coeff + constraint_quotient;

    blake_message_sum_2 = common_lookup_elements
        .combine_qm31(
            [
                qm31_const::<1492981981, 0, 0, 0>(), qm_31_into_u_32_input_limb_16,
                qm31_const::<0, 0, 0, 0>(), limbi_low_col0, limbi_high_col1,
            ]
                .span(),
        );
    numerator_2 = enabler * qm31_const::<10, 0, 0, 0>();

    range_check_16_sum_3 = common_lookup_elements
        .combine_qm31([qm31_const::<1008385708, 0, 0, 0>(), limbi_low_col3].span());
    numerator_3 = enabler;

    range_check_15_sum_4 = common_lookup_elements
        .combine_qm31([qm31_const::<1058718565, 0, 0, 0>(), limbi_high_col4].span());
    numerator_4 = enabler;

    // Constraint - limbi is zero then limbi_low is zero
    let constraint_quotient = ((((qm_31_into_u_32_input_limb_1 * limbi_inv_or_one_col5)
        - qm31_const::<1, 0, 0, 0>())
        * limbi_low_col3));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - limb 1 reconstruction
    let constraint_quotient = ((qm_31_into_u_32_input_limb_1
        - (limbi_low_col3 + (limbi_high_col4 * qm31_const::<65536, 0, 0, 0>()))));
    sum = sum * random_coeff + constraint_quotient;

    blake_message_sum_5 = common_lookup_elements
        .combine_qm31(
            [
                qm31_const::<1492981981, 0, 0, 0>(), qm_31_into_u_32_input_limb_16,
                qm31_const::<1, 0, 0, 0>(), limbi_low_col3, limbi_high_col4,
            ]
                .span(),
        );
    numerator_5 = enabler * qm31_const::<10, 0, 0, 0>();

    range_check_16_sum_6 = common_lookup_elements
        .combine_qm31([qm31_const::<1008385708, 0, 0, 0>(), limbi_low_col6].span());
    numerator_6 = enabler;

    range_check_15_sum_7 = common_lookup_elements
        .combine_qm31([qm31_const::<1058718565, 0, 0, 0>(), limbi_high_col7].span());
    numerator_7 = enabler;

    // Constraint - limbi is zero then limbi_low is zero
    let constraint_quotient = ((((qm_31_into_u_32_input_limb_2 * limbi_inv_or_one_col8)
        - qm31_const::<1, 0, 0, 0>())
        * limbi_low_col6));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - limb 2 reconstruction
    let constraint_quotient = ((qm_31_into_u_32_input_limb_2
        - (limbi_low_col6 + (limbi_high_col7 * qm31_const::<65536, 0, 0, 0>()))));
    sum = sum * random_coeff + constraint_quotient;

    blake_message_sum_8 = common_lookup_elements
        .combine_qm31(
            [
                qm31_const::<1492981981, 0, 0, 0>(), qm_31_into_u_32_input_limb_16,
                qm31_const::<2, 0, 0, 0>(), limbi_low_col6, limbi_high_col7,
            ]
                .span(),
        );
    numerator_8 = enabler * qm31_const::<10, 0, 0, 0>();

    range_check_16_sum_9 = common_lookup_elements
        .combine_qm31([qm31_const::<1008385708, 0, 0, 0>(), limbi_low_col9].span());
    numerator_9 = enabler;

    range_check_15_sum_10 = common_lookup_elements
        .combine_qm31([qm31_const::<1058718565, 0, 0, 0>(), limbi_high_col10].span());
    numerator_10 = enabler;

    // Constraint - limbi is zero then limbi_low is zero
    let constraint_quotient = ((((qm_31_into_u_32_input_limb_3 * limbi_inv_or_one_col11)
        - qm31_const::<1, 0, 0, 0>())
        * limbi_low_col9));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - limb 3 reconstruction
    let constraint_quotient = ((qm_31_into_u_32_input_limb_3
        - (limbi_low_col9 + (limbi_high_col10 * qm31_const::<65536, 0, 0, 0>()))));
    sum = sum * random_coeff + constraint_quotient;

    blake_message_sum_11 = common_lookup_elements
        .combine_qm31(
            [
                qm31_const::<1492981981, 0, 0, 0>(), qm_31_into_u_32_input_limb_16,
                qm31_const::<3, 0, 0, 0>(), limbi_low_col9, limbi_high_col10,
            ]
                .span(),
        );
    numerator_11 = enabler * qm31_const::<10, 0, 0, 0>();

    range_check_16_sum_12 = common_lookup_elements
        .combine_qm31([qm31_const::<1008385708, 0, 0, 0>(), limbi_low_col12].span());
    numerator_12 = enabler;

    range_check_15_sum_13 = common_lookup_elements
        .combine_qm31([qm31_const::<1058718565, 0, 0, 0>(), limbi_high_col13].span());
    numerator_13 = enabler;

    // Constraint - limbi is zero then limbi_low is zero
    let constraint_quotient = ((((qm_31_into_u_32_input_limb_4 * limbi_inv_or_one_col14)
        - qm31_const::<1, 0, 0, 0>())
        * limbi_low_col12));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - limb 4 reconstruction
    let constraint_quotient = ((qm_31_into_u_32_input_limb_4
        - (limbi_low_col12 + (limbi_high_col13 * qm31_const::<65536, 0, 0, 0>()))));
    sum = sum * random_coeff + constraint_quotient;

    blake_message_sum_14 = common_lookup_elements
        .combine_qm31(
            [
                qm31_const::<1492981981, 0, 0, 0>(), qm_31_into_u_32_input_limb_16,
                qm31_const::<4, 0, 0, 0>(), limbi_low_col12, limbi_high_col13,
            ]
                .span(),
        );
    numerator_14 = enabler * qm31_const::<10, 0, 0, 0>();

    range_check_16_sum_15 = common_lookup_elements
        .combine_qm31([qm31_const::<1008385708, 0, 0, 0>(), limbi_low_col15].span());
    numerator_15 = enabler;

    range_check_15_sum_16 = common_lookup_elements
        .combine_qm31([qm31_const::<1058718565, 0, 0, 0>(), limbi_high_col16].span());
    numerator_16 = enabler;

    // Constraint - limbi is zero then limbi_low is zero
    let constraint_quotient = ((((qm_31_into_u_32_input_limb_5 * limbi_inv_or_one_col17)
        - qm31_const::<1, 0, 0, 0>())
        * limbi_low_col15));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - limb 5 reconstruction
    let constraint_quotient = ((qm_31_into_u_32_input_limb_5
        - (limbi_low_col15 + (limbi_high_col16 * qm31_const::<65536, 0, 0, 0>()))));
    sum = sum * random_coeff + constraint_quotient;

    blake_message_sum_17 = common_lookup_elements
        .combine_qm31(
            [
                qm31_const::<1492981981, 0, 0, 0>(), qm_31_into_u_32_input_limb_16,
                qm31_const::<5, 0, 0, 0>(), limbi_low_col15, limbi_high_col16,
            ]
                .span(),
        );
    numerator_17 = enabler * qm31_const::<10, 0, 0, 0>();

    range_check_16_sum_18 = common_lookup_elements
        .combine_qm31([qm31_const::<1008385708, 0, 0, 0>(), limbi_low_col18].span());
    numerator_18 = enabler;

    range_check_15_sum_19 = common_lookup_elements
        .combine_qm31([qm31_const::<1058718565, 0, 0, 0>(), limbi_high_col19].span());
    numerator_19 = enabler;

    // Constraint - limbi is zero then limbi_low is zero
    let constraint_quotient = ((((qm_31_into_u_32_input_limb_6 * limbi_inv_or_one_col20)
        - qm31_const::<1, 0, 0, 0>())
        * limbi_low_col18));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - limb 6 reconstruction
    let constraint_quotient = ((qm_31_into_u_32_input_limb_6
        - (limbi_low_col18 + (limbi_high_col19 * qm31_const::<65536, 0, 0, 0>()))));
    sum = sum * random_coeff + constraint_quotient;

    blake_message_sum_20 = common_lookup_elements
        .combine_qm31(
            [
                qm31_const::<1492981981, 0, 0, 0>(), qm_31_into_u_32_input_limb_16,
                qm31_const::<6, 0, 0, 0>(), limbi_low_col18, limbi_high_col19,
            ]
                .span(),
        );
    numerator_20 = enabler * qm31_const::<10, 0, 0, 0>();

    range_check_16_sum_21 = common_lookup_elements
        .combine_qm31([qm31_const::<1008385708, 0, 0, 0>(), limbi_low_col21].span());
    numerator_21 = enabler;

    range_check_15_sum_22 = common_lookup_elements
        .combine_qm31([qm31_const::<1058718565, 0, 0, 0>(), limbi_high_col22].span());
    numerator_22 = enabler;

    // Constraint - limbi is zero then limbi_low is zero
    let constraint_quotient = ((((qm_31_into_u_32_input_limb_7 * limbi_inv_or_one_col23)
        - qm31_const::<1, 0, 0, 0>())
        * limbi_low_col21));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - limb 7 reconstruction
    let constraint_quotient = ((qm_31_into_u_32_input_limb_7
        - (limbi_low_col21 + (limbi_high_col22 * qm31_const::<65536, 0, 0, 0>()))));
    sum = sum * random_coeff + constraint_quotient;

    blake_message_sum_23 = common_lookup_elements
        .combine_qm31(
            [
                qm31_const::<1492981981, 0, 0, 0>(), qm_31_into_u_32_input_limb_16,
                qm31_const::<7, 0, 0, 0>(), limbi_low_col21, limbi_high_col22,
            ]
                .span(),
        );
    numerator_23 = enabler * qm31_const::<10, 0, 0, 0>();

    range_check_16_sum_24 = common_lookup_elements
        .combine_qm31([qm31_const::<1008385708, 0, 0, 0>(), limbi_low_col24].span());
    numerator_24 = enabler;

    range_check_15_sum_25 = common_lookup_elements
        .combine_qm31([qm31_const::<1058718565, 0, 0, 0>(), limbi_high_col25].span());
    numerator_25 = enabler;

    // Constraint - limbi is zero then limbi_low is zero
    let constraint_quotient = ((((qm_31_into_u_32_input_limb_8 * limbi_inv_or_one_col26)
        - qm31_const::<1, 0, 0, 0>())
        * limbi_low_col24));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - limb 8 reconstruction
    let constraint_quotient = ((qm_31_into_u_32_input_limb_8
        - (limbi_low_col24 + (limbi_high_col25 * qm31_const::<65536, 0, 0, 0>()))));
    sum = sum * random_coeff + constraint_quotient;

    blake_message_sum_26 = common_lookup_elements
        .combine_qm31(
            [
                qm31_const::<1492981981, 0, 0, 0>(), qm_31_into_u_32_input_limb_16,
                qm31_const::<8, 0, 0, 0>(), limbi_low_col24, limbi_high_col25,
            ]
                .span(),
        );
    numerator_26 = enabler * qm31_const::<10, 0, 0, 0>();

    range_check_16_sum_27 = common_lookup_elements
        .combine_qm31([qm31_const::<1008385708, 0, 0, 0>(), limbi_low_col27].span());
    numerator_27 = enabler;

    range_check_15_sum_28 = common_lookup_elements
        .combine_qm31([qm31_const::<1058718565, 0, 0, 0>(), limbi_high_col28].span());
    numerator_28 = enabler;

    // Constraint - limbi is zero then limbi_low is zero
    let constraint_quotient = ((((qm_31_into_u_32_input_limb_9 * limbi_inv_or_one_col29)
        - qm31_const::<1, 0, 0, 0>())
        * limbi_low_col27));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - limb 9 reconstruction
    let constraint_quotient = ((qm_31_into_u_32_input_limb_9
        - (limbi_low_col27 + (limbi_high_col28 * qm31_const::<65536, 0, 0, 0>()))));
    sum = sum * random_coeff + constraint_quotient;

    blake_message_sum_29 = common_lookup_elements
        .combine_qm31(
            [
                qm31_const::<1492981981, 0, 0, 0>(), qm_31_into_u_32_input_limb_16,
                qm31_const::<9, 0, 0, 0>(), limbi_low_col27, limbi_high_col28,
            ]
                .span(),
        );
    numerator_29 = enabler * qm31_const::<10, 0, 0, 0>();

    range_check_16_sum_30 = common_lookup_elements
        .combine_qm31([qm31_const::<1008385708, 0, 0, 0>(), limbi_low_col30].span());
    numerator_30 = enabler;

    range_check_15_sum_31 = common_lookup_elements
        .combine_qm31([qm31_const::<1058718565, 0, 0, 0>(), limbi_high_col31].span());
    numerator_31 = enabler;

    // Constraint - limbi is zero then limbi_low is zero
    let constraint_quotient = ((((qm_31_into_u_32_input_limb_10 * limbi_inv_or_one_col32)
        - qm31_const::<1, 0, 0, 0>())
        * limbi_low_col30));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - limb 10 reconstruction
    let constraint_quotient = ((qm_31_into_u_32_input_limb_10
        - (limbi_low_col30 + (limbi_high_col31 * qm31_const::<65536, 0, 0, 0>()))));
    sum = sum * random_coeff + constraint_quotient;

    blake_message_sum_32 = common_lookup_elements
        .combine_qm31(
            [
                qm31_const::<1492981981, 0, 0, 0>(), qm_31_into_u_32_input_limb_16,
                qm31_const::<10, 0, 0, 0>(), limbi_low_col30, limbi_high_col31,
            ]
                .span(),
        );
    numerator_32 = enabler * qm31_const::<10, 0, 0, 0>();

    range_check_16_sum_33 = common_lookup_elements
        .combine_qm31([qm31_const::<1008385708, 0, 0, 0>(), limbi_low_col33].span());
    numerator_33 = enabler;

    range_check_15_sum_34 = common_lookup_elements
        .combine_qm31([qm31_const::<1058718565, 0, 0, 0>(), limbi_high_col34].span());
    numerator_34 = enabler;

    // Constraint - limbi is zero then limbi_low is zero
    let constraint_quotient = ((((qm_31_into_u_32_input_limb_11 * limbi_inv_or_one_col35)
        - qm31_const::<1, 0, 0, 0>())
        * limbi_low_col33));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - limb 11 reconstruction
    let constraint_quotient = ((qm_31_into_u_32_input_limb_11
        - (limbi_low_col33 + (limbi_high_col34 * qm31_const::<65536, 0, 0, 0>()))));
    sum = sum * random_coeff + constraint_quotient;

    blake_message_sum_35 = common_lookup_elements
        .combine_qm31(
            [
                qm31_const::<1492981981, 0, 0, 0>(), qm_31_into_u_32_input_limb_16,
                qm31_const::<11, 0, 0, 0>(), limbi_low_col33, limbi_high_col34,
            ]
                .span(),
        );
    numerator_35 = enabler * qm31_const::<10, 0, 0, 0>();

    range_check_16_sum_36 = common_lookup_elements
        .combine_qm31([qm31_const::<1008385708, 0, 0, 0>(), limbi_low_col36].span());
    numerator_36 = enabler;

    range_check_15_sum_37 = common_lookup_elements
        .combine_qm31([qm31_const::<1058718565, 0, 0, 0>(), limbi_high_col37].span());
    numerator_37 = enabler;

    // Constraint - limbi is zero then limbi_low is zero
    let constraint_quotient = ((((qm_31_into_u_32_input_limb_12 * limbi_inv_or_one_col38)
        - qm31_const::<1, 0, 0, 0>())
        * limbi_low_col36));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - limb 12 reconstruction
    let constraint_quotient = ((qm_31_into_u_32_input_limb_12
        - (limbi_low_col36 + (limbi_high_col37 * qm31_const::<65536, 0, 0, 0>()))));
    sum = sum * random_coeff + constraint_quotient;

    blake_message_sum_38 = common_lookup_elements
        .combine_qm31(
            [
                qm31_const::<1492981981, 0, 0, 0>(), qm_31_into_u_32_input_limb_16,
                qm31_const::<12, 0, 0, 0>(), limbi_low_col36, limbi_high_col37,
            ]
                .span(),
        );
    numerator_38 = enabler * qm31_const::<10, 0, 0, 0>();

    range_check_16_sum_39 = common_lookup_elements
        .combine_qm31([qm31_const::<1008385708, 0, 0, 0>(), limbi_low_col39].span());
    numerator_39 = enabler;

    range_check_15_sum_40 = common_lookup_elements
        .combine_qm31([qm31_const::<1058718565, 0, 0, 0>(), limbi_high_col40].span());
    numerator_40 = enabler;

    // Constraint - limbi is zero then limbi_low is zero
    let constraint_quotient = ((((qm_31_into_u_32_input_limb_13 * limbi_inv_or_one_col41)
        - qm31_const::<1, 0, 0, 0>())
        * limbi_low_col39));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - limb 13 reconstruction
    let constraint_quotient = ((qm_31_into_u_32_input_limb_13
        - (limbi_low_col39 + (limbi_high_col40 * qm31_const::<65536, 0, 0, 0>()))));
    sum = sum * random_coeff + constraint_quotient;

    blake_message_sum_41 = common_lookup_elements
        .combine_qm31(
            [
                qm31_const::<1492981981, 0, 0, 0>(), qm_31_into_u_32_input_limb_16,
                qm31_const::<13, 0, 0, 0>(), limbi_low_col39, limbi_high_col40,
            ]
                .span(),
        );
    numerator_41 = enabler * qm31_const::<10, 0, 0, 0>();

    range_check_16_sum_42 = common_lookup_elements
        .combine_qm31([qm31_const::<1008385708, 0, 0, 0>(), limbi_low_col42].span());
    numerator_42 = enabler;

    range_check_15_sum_43 = common_lookup_elements
        .combine_qm31([qm31_const::<1058718565, 0, 0, 0>(), limbi_high_col43].span());
    numerator_43 = enabler;

    // Constraint - limbi is zero then limbi_low is zero
    let constraint_quotient = ((((qm_31_into_u_32_input_limb_14 * limbi_inv_or_one_col44)
        - qm31_const::<1, 0, 0, 0>())
        * limbi_low_col42));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - limb 14 reconstruction
    let constraint_quotient = ((qm_31_into_u_32_input_limb_14
        - (limbi_low_col42 + (limbi_high_col43 * qm31_const::<65536, 0, 0, 0>()))));
    sum = sum * random_coeff + constraint_quotient;

    blake_message_sum_44 = common_lookup_elements
        .combine_qm31(
            [
                qm31_const::<1492981981, 0, 0, 0>(), qm_31_into_u_32_input_limb_16,
                qm31_const::<14, 0, 0, 0>(), limbi_low_col42, limbi_high_col43,
            ]
                .span(),
        );
    numerator_44 = enabler * qm31_const::<10, 0, 0, 0>();

    range_check_16_sum_45 = common_lookup_elements
        .combine_qm31([qm31_const::<1008385708, 0, 0, 0>(), limbi_low_col45].span());
    numerator_45 = enabler;

    range_check_15_sum_46 = common_lookup_elements
        .combine_qm31([qm31_const::<1058718565, 0, 0, 0>(), limbi_high_col46].span());
    numerator_46 = enabler;

    // Constraint - limbi is zero then limbi_low is zero
    let constraint_quotient = ((((qm_31_into_u_32_input_limb_15 * limbi_inv_or_one_col47)
        - qm31_const::<1, 0, 0, 0>())
        * limbi_low_col45));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - limb 15 reconstruction
    let constraint_quotient = ((qm_31_into_u_32_input_limb_15
        - (limbi_low_col45 + (limbi_high_col46 * qm31_const::<65536, 0, 0, 0>()))));
    sum = sum * random_coeff + constraint_quotient;

    blake_message_sum_47 = common_lookup_elements
        .combine_qm31(
            [
                qm31_const::<1492981981, 0, 0, 0>(), qm_31_into_u_32_input_limb_16,
                qm31_const::<15, 0, 0, 0>(), limbi_low_col45, limbi_high_col46,
            ]
                .span(),
        );
    numerator_47 = enabler * qm31_const::<10, 0, 0, 0>();

    []
}
