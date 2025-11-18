use crate::prelude::*;

#[derive(Drop)]
pub struct ConstraintParams {
    pub VerifyBitwiseXor_12_alpha0: QM31,
    pub VerifyBitwiseXor_12_alpha1: QM31,
    pub VerifyBitwiseXor_12_alpha2: QM31,
    pub VerifyBitwiseXor_12_z: QM31,
    pub bitwise_xor_10_0: QM31,
    pub bitwise_xor_10_1: QM31,
    pub bitwise_xor_10_2: QM31,
    pub claimed_sum: QM31,
    pub column_size: M31,
}

pub fn evaluate_constraints_at_point(
    ref sum: QM31,
    ref trace_mask_values: ColumnSpan<Span<QM31>>,
    ref interaction_mask_values: ColumnSpan<Span<QM31>>,
    params: ConstraintParams,
    random_coeff: QM31,
    domain_vanish_at_point_inv: QM31,
) {
    let ConstraintParams {
        VerifyBitwiseXor_12_alpha0,
        VerifyBitwiseXor_12_alpha1,
        VerifyBitwiseXor_12_alpha2,
        VerifyBitwiseXor_12_z,
        bitwise_xor_10_0,
        bitwise_xor_10_1,
        bitwise_xor_10_2,
        claimed_sum,
        column_size,
    } = params;
    let [
        trace_1_column_0,
        trace_1_column_1,
        trace_1_column_2,
        trace_1_column_3,
        trace_1_column_4,
        trace_1_column_5,
        trace_1_column_6,
        trace_1_column_7,
        trace_1_column_8,
        trace_1_column_9,
        trace_1_column_10,
        trace_1_column_11,
        trace_1_column_12,
        trace_1_column_13,
        trace_1_column_14,
        trace_1_column_15,
    ]: [Span<QM31>; 16] =
        (*trace_mask_values
        .multi_pop_front()
        .unwrap())
        .unbox();

    let [trace_1_column_0_offset_0]: [QM31; 1] = (*trace_1_column_0.try_into().unwrap()).unbox();

    let [trace_1_column_1_offset_0]: [QM31; 1] = (*trace_1_column_1.try_into().unwrap()).unbox();

    let [trace_1_column_2_offset_0]: [QM31; 1] = (*trace_1_column_2.try_into().unwrap()).unbox();

    let [trace_1_column_3_offset_0]: [QM31; 1] = (*trace_1_column_3.try_into().unwrap()).unbox();

    let [trace_1_column_4_offset_0]: [QM31; 1] = (*trace_1_column_4.try_into().unwrap()).unbox();

    let [trace_1_column_5_offset_0]: [QM31; 1] = (*trace_1_column_5.try_into().unwrap()).unbox();

    let [trace_1_column_6_offset_0]: [QM31; 1] = (*trace_1_column_6.try_into().unwrap()).unbox();

    let [trace_1_column_7_offset_0]: [QM31; 1] = (*trace_1_column_7.try_into().unwrap()).unbox();

    let [trace_1_column_8_offset_0]: [QM31; 1] = (*trace_1_column_8.try_into().unwrap()).unbox();

    let [trace_1_column_9_offset_0]: [QM31; 1] = (*trace_1_column_9.try_into().unwrap()).unbox();

    let [trace_1_column_10_offset_0]: [QM31; 1] = (*trace_1_column_10.try_into().unwrap()).unbox();

    let [trace_1_column_11_offset_0]: [QM31; 1] = (*trace_1_column_11.try_into().unwrap()).unbox();

    let [trace_1_column_12_offset_0]: [QM31; 1] = (*trace_1_column_12.try_into().unwrap()).unbox();

    let [trace_1_column_13_offset_0]: [QM31; 1] = (*trace_1_column_13.try_into().unwrap()).unbox();

    let [trace_1_column_14_offset_0]: [QM31; 1] = (*trace_1_column_14.try_into().unwrap()).unbox();

    let [trace_1_column_15_offset_0]: [QM31; 1] = (*trace_1_column_15.try_into().unwrap()).unbox();

    let [
        trace_2_column_16,
        trace_2_column_17,
        trace_2_column_18,
        trace_2_column_19,
        trace_2_column_20,
        trace_2_column_21,
        trace_2_column_22,
        trace_2_column_23,
        trace_2_column_24,
        trace_2_column_25,
        trace_2_column_26,
        trace_2_column_27,
        trace_2_column_28,
        trace_2_column_29,
        trace_2_column_30,
        trace_2_column_31,
        trace_2_column_32,
        trace_2_column_33,
        trace_2_column_34,
        trace_2_column_35,
        trace_2_column_36,
        trace_2_column_37,
        trace_2_column_38,
        trace_2_column_39,
        trace_2_column_40,
        trace_2_column_41,
        trace_2_column_42,
        trace_2_column_43,
        trace_2_column_44,
        trace_2_column_45,
        trace_2_column_46,
        trace_2_column_47,
    ]: [Span<QM31>; 32] =
        (*interaction_mask_values
        .multi_pop_front()
        .unwrap())
        .unbox();

    let [trace_2_column_16_offset_0]: [QM31; 1] = (*trace_2_column_16.try_into().unwrap()).unbox();

    let [trace_2_column_17_offset_0]: [QM31; 1] = (*trace_2_column_17.try_into().unwrap()).unbox();

    let [trace_2_column_18_offset_0]: [QM31; 1] = (*trace_2_column_18.try_into().unwrap()).unbox();

    let [trace_2_column_19_offset_0]: [QM31; 1] = (*trace_2_column_19.try_into().unwrap()).unbox();

    let [trace_2_column_20_offset_0]: [QM31; 1] = (*trace_2_column_20.try_into().unwrap()).unbox();

    let [trace_2_column_21_offset_0]: [QM31; 1] = (*trace_2_column_21.try_into().unwrap()).unbox();

    let [trace_2_column_22_offset_0]: [QM31; 1] = (*trace_2_column_22.try_into().unwrap()).unbox();

    let [trace_2_column_23_offset_0]: [QM31; 1] = (*trace_2_column_23.try_into().unwrap()).unbox();

    let [trace_2_column_24_offset_0]: [QM31; 1] = (*trace_2_column_24.try_into().unwrap()).unbox();

    let [trace_2_column_25_offset_0]: [QM31; 1] = (*trace_2_column_25.try_into().unwrap()).unbox();

    let [trace_2_column_26_offset_0]: [QM31; 1] = (*trace_2_column_26.try_into().unwrap()).unbox();

    let [trace_2_column_27_offset_0]: [QM31; 1] = (*trace_2_column_27.try_into().unwrap()).unbox();

    let [trace_2_column_28_offset_0]: [QM31; 1] = (*trace_2_column_28.try_into().unwrap()).unbox();

    let [trace_2_column_29_offset_0]: [QM31; 1] = (*trace_2_column_29.try_into().unwrap()).unbox();

    let [trace_2_column_30_offset_0]: [QM31; 1] = (*trace_2_column_30.try_into().unwrap()).unbox();

    let [trace_2_column_31_offset_0]: [QM31; 1] = (*trace_2_column_31.try_into().unwrap()).unbox();

    let [trace_2_column_32_offset_0]: [QM31; 1] = (*trace_2_column_32.try_into().unwrap()).unbox();

    let [trace_2_column_33_offset_0]: [QM31; 1] = (*trace_2_column_33.try_into().unwrap()).unbox();

    let [trace_2_column_34_offset_0]: [QM31; 1] = (*trace_2_column_34.try_into().unwrap()).unbox();

    let [trace_2_column_35_offset_0]: [QM31; 1] = (*trace_2_column_35.try_into().unwrap()).unbox();

    let [trace_2_column_36_offset_0]: [QM31; 1] = (*trace_2_column_36.try_into().unwrap()).unbox();

    let [trace_2_column_37_offset_0]: [QM31; 1] = (*trace_2_column_37.try_into().unwrap()).unbox();

    let [trace_2_column_38_offset_0]: [QM31; 1] = (*trace_2_column_38.try_into().unwrap()).unbox();

    let [trace_2_column_39_offset_0]: [QM31; 1] = (*trace_2_column_39.try_into().unwrap()).unbox();

    let [trace_2_column_40_offset_0]: [QM31; 1] = (*trace_2_column_40.try_into().unwrap()).unbox();

    let [trace_2_column_41_offset_0]: [QM31; 1] = (*trace_2_column_41.try_into().unwrap()).unbox();

    let [trace_2_column_42_offset_0]: [QM31; 1] = (*trace_2_column_42.try_into().unwrap()).unbox();

    let [trace_2_column_43_offset_0]: [QM31; 1] = (*trace_2_column_43.try_into().unwrap()).unbox();

    let [trace_2_column_44_offset_neg_1, trace_2_column_44_offset_0]: [QM31; 2] =
        (*trace_2_column_44
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_45_offset_neg_1, trace_2_column_45_offset_0]: [QM31; 2] =
        (*trace_2_column_45
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_46_offset_neg_1, trace_2_column_46_offset_0]: [QM31; 2] =
        (*trace_2_column_46
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_47_offset_neg_1, trace_2_column_47_offset_0]: [QM31; 2] =
        (*trace_2_column_47
        .try_into()
        .unwrap())
        .unbox();

    core::internal::revoke_ap_tracking();

    let mut intermediates = intermediates(
        VerifyBitwiseXor_12_alpha0,
        VerifyBitwiseXor_12_alpha1,
        VerifyBitwiseXor_12_alpha2,
        VerifyBitwiseXor_12_z,
        bitwise_xor_10_0,
        bitwise_xor_10_1,
        bitwise_xor_10_2,
    )
        .span();
    let intermediate0 = *intermediates.pop_front().unwrap();
    let intermediate1 = *intermediates.pop_front().unwrap();
    let intermediate2 = *intermediates.pop_front().unwrap();
    let intermediate3 = *intermediates.pop_front().unwrap();
    let intermediate4 = *intermediates.pop_front().unwrap();
    let intermediate5 = *intermediates.pop_front().unwrap();
    let intermediate6 = *intermediates.pop_front().unwrap();
    let intermediate7 = *intermediates.pop_front().unwrap();
    let intermediate8 = *intermediates.pop_front().unwrap();
    let intermediate9 = *intermediates.pop_front().unwrap();
    let intermediate10 = *intermediates.pop_front().unwrap();
    let intermediate11 = *intermediates.pop_front().unwrap();
    let intermediate12 = *intermediates.pop_front().unwrap();
    let intermediate13 = *intermediates.pop_front().unwrap();
    let intermediate14 = *intermediates.pop_front().unwrap();
    let intermediate15 = *intermediates.pop_front().unwrap();

    // Constraint 0
    let constraint_quotient = ((QM31Trait::from_partial_evals(
        [
            trace_2_column_16_offset_0, trace_2_column_17_offset_0, trace_2_column_18_offset_0,
            trace_2_column_19_offset_0,
        ],
    ))
        * ((intermediate0) * (intermediate1))
        + (intermediate1) * (trace_1_column_0_offset_0)
        + (intermediate0) * (trace_1_column_1_offset_0))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 1
    let constraint_quotient = ((QM31Trait::from_partial_evals(
        [
            trace_2_column_20_offset_0, trace_2_column_21_offset_0, trace_2_column_22_offset_0,
            trace_2_column_23_offset_0,
        ],
    )
        - (QM31Trait::from_partial_evals(
            [
                trace_2_column_16_offset_0, trace_2_column_17_offset_0, trace_2_column_18_offset_0,
                trace_2_column_19_offset_0,
            ],
        )))
        * ((intermediate2) * (intermediate3))
        + (intermediate3) * (trace_1_column_2_offset_0)
        + (intermediate2) * (trace_1_column_3_offset_0))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 2
    let constraint_quotient = ((QM31Trait::from_partial_evals(
        [
            trace_2_column_24_offset_0, trace_2_column_25_offset_0, trace_2_column_26_offset_0,
            trace_2_column_27_offset_0,
        ],
    )
        - (QM31Trait::from_partial_evals(
            [
                trace_2_column_20_offset_0, trace_2_column_21_offset_0, trace_2_column_22_offset_0,
                trace_2_column_23_offset_0,
            ],
        )))
        * ((intermediate4) * (intermediate5))
        + (intermediate5) * (trace_1_column_4_offset_0)
        + (intermediate4) * (trace_1_column_5_offset_0))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 3
    let constraint_quotient = ((QM31Trait::from_partial_evals(
        [
            trace_2_column_28_offset_0, trace_2_column_29_offset_0, trace_2_column_30_offset_0,
            trace_2_column_31_offset_0,
        ],
    )
        - (QM31Trait::from_partial_evals(
            [
                trace_2_column_24_offset_0, trace_2_column_25_offset_0, trace_2_column_26_offset_0,
                trace_2_column_27_offset_0,
            ],
        )))
        * ((intermediate6) * (intermediate7))
        + (intermediate7) * (trace_1_column_6_offset_0)
        + (intermediate6) * (trace_1_column_7_offset_0))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 4
    let constraint_quotient = ((QM31Trait::from_partial_evals(
        [
            trace_2_column_32_offset_0, trace_2_column_33_offset_0, trace_2_column_34_offset_0,
            trace_2_column_35_offset_0,
        ],
    )
        - (QM31Trait::from_partial_evals(
            [
                trace_2_column_28_offset_0, trace_2_column_29_offset_0, trace_2_column_30_offset_0,
                trace_2_column_31_offset_0,
            ],
        )))
        * ((intermediate8) * (intermediate9))
        + (intermediate9) * (trace_1_column_8_offset_0)
        + (intermediate8) * (trace_1_column_9_offset_0))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 5
    let constraint_quotient = ((QM31Trait::from_partial_evals(
        [
            trace_2_column_36_offset_0, trace_2_column_37_offset_0, trace_2_column_38_offset_0,
            trace_2_column_39_offset_0,
        ],
    )
        - (QM31Trait::from_partial_evals(
            [
                trace_2_column_32_offset_0, trace_2_column_33_offset_0, trace_2_column_34_offset_0,
                trace_2_column_35_offset_0,
            ],
        )))
        * ((intermediate10) * (intermediate11))
        + (intermediate11) * (trace_1_column_10_offset_0)
        + (intermediate10) * (trace_1_column_11_offset_0))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 6
    let constraint_quotient = ((QM31Trait::from_partial_evals(
        [
            trace_2_column_40_offset_0, trace_2_column_41_offset_0, trace_2_column_42_offset_0,
            trace_2_column_43_offset_0,
        ],
    )
        - (QM31Trait::from_partial_evals(
            [
                trace_2_column_36_offset_0, trace_2_column_37_offset_0, trace_2_column_38_offset_0,
                trace_2_column_39_offset_0,
            ],
        )))
        * ((intermediate12) * (intermediate13))
        + (intermediate13) * (trace_1_column_12_offset_0)
        + (intermediate12) * (trace_1_column_13_offset_0))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 7
    let constraint_quotient = ((QM31Trait::from_partial_evals(
        [
            trace_2_column_44_offset_0, trace_2_column_45_offset_0, trace_2_column_46_offset_0,
            trace_2_column_47_offset_0,
        ],
    )
        - (QM31Trait::from_partial_evals(
            [
                trace_2_column_44_offset_neg_1, trace_2_column_45_offset_neg_1,
                trace_2_column_46_offset_neg_1, trace_2_column_47_offset_neg_1,
            ],
        ))
        - (QM31Trait::from_partial_evals(
            [
                trace_2_column_40_offset_0, trace_2_column_41_offset_0, trace_2_column_42_offset_0,
                trace_2_column_43_offset_0,
            ],
        ))
        + (claimed_sum) * (column_size.inverse().into()))
        * ((intermediate14) * (intermediate15))
        + (intermediate15) * (trace_1_column_14_offset_0)
        + (intermediate14) * (trace_1_column_15_offset_0))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;
}


fn intermediates(
    VerifyBitwiseXor_12_alpha0: QM31,
    VerifyBitwiseXor_12_alpha1: QM31,
    VerifyBitwiseXor_12_alpha2: QM31,
    VerifyBitwiseXor_12_z: QM31,
    bitwise_xor_10_0: QM31,
    bitwise_xor_10_1: QM31,
    bitwise_xor_10_2: QM31,
) -> Array<QM31> {
    let intermediate12 = intermediate12(
        VerifyBitwiseXor_12_alpha0,
        VerifyBitwiseXor_12_alpha1,
        VerifyBitwiseXor_12_alpha2,
        VerifyBitwiseXor_12_z,
        bitwise_xor_10_0,
        bitwise_xor_10_1,
        bitwise_xor_10_2,
    );

    let intermediate1 = intermediate1(
        VerifyBitwiseXor_12_alpha0,
        VerifyBitwiseXor_12_alpha1,
        VerifyBitwiseXor_12_alpha2,
        VerifyBitwiseXor_12_z,
        bitwise_xor_10_0,
        bitwise_xor_10_1,
        bitwise_xor_10_2,
    );

    let intermediate2 = intermediate2(
        VerifyBitwiseXor_12_alpha0,
        VerifyBitwiseXor_12_alpha1,
        VerifyBitwiseXor_12_alpha2,
        VerifyBitwiseXor_12_z,
        bitwise_xor_10_0,
        bitwise_xor_10_1,
        bitwise_xor_10_2,
    );

    let intermediate13 = intermediate13(
        VerifyBitwiseXor_12_alpha0,
        VerifyBitwiseXor_12_alpha1,
        VerifyBitwiseXor_12_alpha2,
        VerifyBitwiseXor_12_z,
        bitwise_xor_10_0,
        bitwise_xor_10_1,
        bitwise_xor_10_2,
    );

    let intermediate11 = intermediate11(
        VerifyBitwiseXor_12_alpha0,
        VerifyBitwiseXor_12_alpha1,
        VerifyBitwiseXor_12_alpha2,
        VerifyBitwiseXor_12_z,
        bitwise_xor_10_0,
        bitwise_xor_10_1,
        bitwise_xor_10_2,
    );

    let intermediate5 = intermediate5(
        VerifyBitwiseXor_12_alpha0,
        VerifyBitwiseXor_12_alpha1,
        VerifyBitwiseXor_12_alpha2,
        VerifyBitwiseXor_12_z,
        bitwise_xor_10_0,
        bitwise_xor_10_1,
        bitwise_xor_10_2,
    );

    let intermediate10 = intermediate10(
        VerifyBitwiseXor_12_alpha0,
        VerifyBitwiseXor_12_alpha1,
        VerifyBitwiseXor_12_alpha2,
        VerifyBitwiseXor_12_z,
        bitwise_xor_10_0,
        bitwise_xor_10_1,
        bitwise_xor_10_2,
    );

    let intermediate14 = intermediate14(
        VerifyBitwiseXor_12_alpha0,
        VerifyBitwiseXor_12_alpha1,
        VerifyBitwiseXor_12_alpha2,
        VerifyBitwiseXor_12_z,
        bitwise_xor_10_0,
        bitwise_xor_10_1,
        bitwise_xor_10_2,
    );

    let intermediate0 = intermediate0(
        VerifyBitwiseXor_12_alpha0,
        VerifyBitwiseXor_12_alpha1,
        VerifyBitwiseXor_12_alpha2,
        VerifyBitwiseXor_12_z,
        bitwise_xor_10_0,
        bitwise_xor_10_1,
        bitwise_xor_10_2,
    );

    let intermediate6 = intermediate6(
        VerifyBitwiseXor_12_alpha0,
        VerifyBitwiseXor_12_alpha1,
        VerifyBitwiseXor_12_alpha2,
        VerifyBitwiseXor_12_z,
        bitwise_xor_10_0,
        bitwise_xor_10_1,
        bitwise_xor_10_2,
    );

    let intermediate4 = intermediate4(
        VerifyBitwiseXor_12_alpha0,
        VerifyBitwiseXor_12_alpha1,
        VerifyBitwiseXor_12_alpha2,
        VerifyBitwiseXor_12_z,
        bitwise_xor_10_0,
        bitwise_xor_10_1,
        bitwise_xor_10_2,
    );

    let intermediate7 = intermediate7(
        VerifyBitwiseXor_12_alpha0,
        VerifyBitwiseXor_12_alpha1,
        VerifyBitwiseXor_12_alpha2,
        VerifyBitwiseXor_12_z,
        bitwise_xor_10_0,
        bitwise_xor_10_1,
        bitwise_xor_10_2,
    );

    let intermediate3 = intermediate3(
        VerifyBitwiseXor_12_alpha0,
        VerifyBitwiseXor_12_alpha1,
        VerifyBitwiseXor_12_alpha2,
        VerifyBitwiseXor_12_z,
        bitwise_xor_10_0,
        bitwise_xor_10_1,
        bitwise_xor_10_2,
    );

    let intermediate8 = intermediate8(
        VerifyBitwiseXor_12_alpha0,
        VerifyBitwiseXor_12_alpha1,
        VerifyBitwiseXor_12_alpha2,
        VerifyBitwiseXor_12_z,
        bitwise_xor_10_0,
        bitwise_xor_10_1,
        bitwise_xor_10_2,
    );

    let intermediate9 = intermediate9(
        VerifyBitwiseXor_12_alpha0,
        VerifyBitwiseXor_12_alpha1,
        VerifyBitwiseXor_12_alpha2,
        VerifyBitwiseXor_12_z,
        bitwise_xor_10_0,
        bitwise_xor_10_1,
        bitwise_xor_10_2,
    );

    let intermediate15 = intermediate15(
        VerifyBitwiseXor_12_alpha0,
        VerifyBitwiseXor_12_alpha1,
        VerifyBitwiseXor_12_alpha2,
        VerifyBitwiseXor_12_z,
        bitwise_xor_10_0,
        bitwise_xor_10_1,
        bitwise_xor_10_2,
    );
    array![
        intermediate0, intermediate1, intermediate2, intermediate3, intermediate4, intermediate5,
        intermediate6, intermediate7, intermediate8, intermediate9, intermediate10, intermediate11,
        intermediate12, intermediate13, intermediate14, intermediate15,
    ]
}


pub fn intermediate12(
    VerifyBitwiseXor_12_alpha0: QM31,
    VerifyBitwiseXor_12_alpha1: QM31,
    VerifyBitwiseXor_12_alpha2: QM31,
    VerifyBitwiseXor_12_z: QM31,
    bitwise_xor_10_0: QM31,
    bitwise_xor_10_1: QM31,
    bitwise_xor_10_2: QM31,
) -> QM31 {
    (VerifyBitwiseXor_12_alpha0) * (bitwise_xor_10_0 + m31(3072).into())
        + (VerifyBitwiseXor_12_alpha1) * (bitwise_xor_10_1)
        + (VerifyBitwiseXor_12_alpha2) * (bitwise_xor_10_2 + m31(3072).into())
        - (VerifyBitwiseXor_12_z)
}

pub fn intermediate1(
    VerifyBitwiseXor_12_alpha0: QM31,
    VerifyBitwiseXor_12_alpha1: QM31,
    VerifyBitwiseXor_12_alpha2: QM31,
    VerifyBitwiseXor_12_z: QM31,
    bitwise_xor_10_0: QM31,
    bitwise_xor_10_1: QM31,
    bitwise_xor_10_2: QM31,
) -> QM31 {
    (VerifyBitwiseXor_12_alpha0) * (bitwise_xor_10_0)
        + (VerifyBitwiseXor_12_alpha1) * (bitwise_xor_10_1 + m31(1024).into())
        + (VerifyBitwiseXor_12_alpha2) * (bitwise_xor_10_2 + m31(1024).into())
        - (VerifyBitwiseXor_12_z)
}

pub fn intermediate2(
    VerifyBitwiseXor_12_alpha0: QM31,
    VerifyBitwiseXor_12_alpha1: QM31,
    VerifyBitwiseXor_12_alpha2: QM31,
    VerifyBitwiseXor_12_z: QM31,
    bitwise_xor_10_0: QM31,
    bitwise_xor_10_1: QM31,
    bitwise_xor_10_2: QM31,
) -> QM31 {
    (VerifyBitwiseXor_12_alpha0) * (bitwise_xor_10_0)
        + (VerifyBitwiseXor_12_alpha1) * (bitwise_xor_10_1 + m31(2048).into())
        + (VerifyBitwiseXor_12_alpha2) * (bitwise_xor_10_2 + m31(2048).into())
        - (VerifyBitwiseXor_12_z)
}

pub fn intermediate13(
    VerifyBitwiseXor_12_alpha0: QM31,
    VerifyBitwiseXor_12_alpha1: QM31,
    VerifyBitwiseXor_12_alpha2: QM31,
    VerifyBitwiseXor_12_z: QM31,
    bitwise_xor_10_0: QM31,
    bitwise_xor_10_1: QM31,
    bitwise_xor_10_2: QM31,
) -> QM31 {
    (VerifyBitwiseXor_12_alpha0) * (bitwise_xor_10_0 + m31(3072).into())
        + (VerifyBitwiseXor_12_alpha1) * (bitwise_xor_10_1 + m31(1024).into())
        + (VerifyBitwiseXor_12_alpha2) * (bitwise_xor_10_2 + m31(2048).into())
        - (VerifyBitwiseXor_12_z)
}

pub fn intermediate11(
    VerifyBitwiseXor_12_alpha0: QM31,
    VerifyBitwiseXor_12_alpha1: QM31,
    VerifyBitwiseXor_12_alpha2: QM31,
    VerifyBitwiseXor_12_z: QM31,
    bitwise_xor_10_0: QM31,
    bitwise_xor_10_1: QM31,
    bitwise_xor_10_2: QM31,
) -> QM31 {
    (VerifyBitwiseXor_12_alpha0) * (bitwise_xor_10_0 + m31(2048).into())
        + (VerifyBitwiseXor_12_alpha1) * (bitwise_xor_10_1 + m31(3072).into())
        + (VerifyBitwiseXor_12_alpha2) * (bitwise_xor_10_2 + m31(1024).into())
        - (VerifyBitwiseXor_12_z)
}

pub fn intermediate5(
    VerifyBitwiseXor_12_alpha0: QM31,
    VerifyBitwiseXor_12_alpha1: QM31,
    VerifyBitwiseXor_12_alpha2: QM31,
    VerifyBitwiseXor_12_z: QM31,
    bitwise_xor_10_0: QM31,
    bitwise_xor_10_1: QM31,
    bitwise_xor_10_2: QM31,
) -> QM31 {
    (VerifyBitwiseXor_12_alpha0) * (bitwise_xor_10_0 + m31(1024).into())
        + (VerifyBitwiseXor_12_alpha1) * (bitwise_xor_10_1 + m31(1024).into())
        + (VerifyBitwiseXor_12_alpha2) * (bitwise_xor_10_2)
        - (VerifyBitwiseXor_12_z)
}

pub fn intermediate10(
    VerifyBitwiseXor_12_alpha0: QM31,
    VerifyBitwiseXor_12_alpha1: QM31,
    VerifyBitwiseXor_12_alpha2: QM31,
    VerifyBitwiseXor_12_z: QM31,
    bitwise_xor_10_0: QM31,
    bitwise_xor_10_1: QM31,
    bitwise_xor_10_2: QM31,
) -> QM31 {
    (VerifyBitwiseXor_12_alpha0) * (bitwise_xor_10_0 + m31(2048).into())
        + (VerifyBitwiseXor_12_alpha1) * (bitwise_xor_10_1 + m31(2048).into())
        + (VerifyBitwiseXor_12_alpha2) * (bitwise_xor_10_2)
        - (VerifyBitwiseXor_12_z)
}

pub fn intermediate14(
    VerifyBitwiseXor_12_alpha0: QM31,
    VerifyBitwiseXor_12_alpha1: QM31,
    VerifyBitwiseXor_12_alpha2: QM31,
    VerifyBitwiseXor_12_z: QM31,
    bitwise_xor_10_0: QM31,
    bitwise_xor_10_1: QM31,
    bitwise_xor_10_2: QM31,
) -> QM31 {
    (VerifyBitwiseXor_12_alpha0) * (bitwise_xor_10_0 + m31(3072).into())
        + (VerifyBitwiseXor_12_alpha1) * (bitwise_xor_10_1 + m31(2048).into())
        + (VerifyBitwiseXor_12_alpha2) * (bitwise_xor_10_2 + m31(1024).into())
        - (VerifyBitwiseXor_12_z)
}

pub fn intermediate0(
    VerifyBitwiseXor_12_alpha0: QM31,
    VerifyBitwiseXor_12_alpha1: QM31,
    VerifyBitwiseXor_12_alpha2: QM31,
    VerifyBitwiseXor_12_z: QM31,
    bitwise_xor_10_0: QM31,
    bitwise_xor_10_1: QM31,
    bitwise_xor_10_2: QM31,
) -> QM31 {
    (VerifyBitwiseXor_12_alpha0) * (bitwise_xor_10_0)
        + (VerifyBitwiseXor_12_alpha1) * (bitwise_xor_10_1)
        + (VerifyBitwiseXor_12_alpha2) * (bitwise_xor_10_2)
        - (VerifyBitwiseXor_12_z)
}

pub fn intermediate6(
    VerifyBitwiseXor_12_alpha0: QM31,
    VerifyBitwiseXor_12_alpha1: QM31,
    VerifyBitwiseXor_12_alpha2: QM31,
    VerifyBitwiseXor_12_z: QM31,
    bitwise_xor_10_0: QM31,
    bitwise_xor_10_1: QM31,
    bitwise_xor_10_2: QM31,
) -> QM31 {
    (VerifyBitwiseXor_12_alpha0) * (bitwise_xor_10_0 + m31(1024).into())
        + (VerifyBitwiseXor_12_alpha1) * (bitwise_xor_10_1 + m31(2048).into())
        + (VerifyBitwiseXor_12_alpha2) * (bitwise_xor_10_2 + m31(3072).into())
        - (VerifyBitwiseXor_12_z)
}

pub fn intermediate4(
    VerifyBitwiseXor_12_alpha0: QM31,
    VerifyBitwiseXor_12_alpha1: QM31,
    VerifyBitwiseXor_12_alpha2: QM31,
    VerifyBitwiseXor_12_z: QM31,
    bitwise_xor_10_0: QM31,
    bitwise_xor_10_1: QM31,
    bitwise_xor_10_2: QM31,
) -> QM31 {
    (VerifyBitwiseXor_12_alpha0) * (bitwise_xor_10_0 + m31(1024).into())
        + (VerifyBitwiseXor_12_alpha1) * (bitwise_xor_10_1)
        + (VerifyBitwiseXor_12_alpha2) * (bitwise_xor_10_2 + m31(1024).into())
        - (VerifyBitwiseXor_12_z)
}

pub fn intermediate7(
    VerifyBitwiseXor_12_alpha0: QM31,
    VerifyBitwiseXor_12_alpha1: QM31,
    VerifyBitwiseXor_12_alpha2: QM31,
    VerifyBitwiseXor_12_z: QM31,
    bitwise_xor_10_0: QM31,
    bitwise_xor_10_1: QM31,
    bitwise_xor_10_2: QM31,
) -> QM31 {
    (VerifyBitwiseXor_12_alpha0) * (bitwise_xor_10_0 + m31(1024).into())
        + (VerifyBitwiseXor_12_alpha1) * (bitwise_xor_10_1 + m31(3072).into())
        + (VerifyBitwiseXor_12_alpha2) * (bitwise_xor_10_2 + m31(2048).into())
        - (VerifyBitwiseXor_12_z)
}

pub fn intermediate3(
    VerifyBitwiseXor_12_alpha0: QM31,
    VerifyBitwiseXor_12_alpha1: QM31,
    VerifyBitwiseXor_12_alpha2: QM31,
    VerifyBitwiseXor_12_z: QM31,
    bitwise_xor_10_0: QM31,
    bitwise_xor_10_1: QM31,
    bitwise_xor_10_2: QM31,
) -> QM31 {
    (VerifyBitwiseXor_12_alpha0) * (bitwise_xor_10_0)
        + (VerifyBitwiseXor_12_alpha1) * (bitwise_xor_10_1 + m31(3072).into())
        + (VerifyBitwiseXor_12_alpha2) * (bitwise_xor_10_2 + m31(3072).into())
        - (VerifyBitwiseXor_12_z)
}

pub fn intermediate8(
    VerifyBitwiseXor_12_alpha0: QM31,
    VerifyBitwiseXor_12_alpha1: QM31,
    VerifyBitwiseXor_12_alpha2: QM31,
    VerifyBitwiseXor_12_z: QM31,
    bitwise_xor_10_0: QM31,
    bitwise_xor_10_1: QM31,
    bitwise_xor_10_2: QM31,
) -> QM31 {
    (VerifyBitwiseXor_12_alpha0) * (bitwise_xor_10_0 + m31(2048).into())
        + (VerifyBitwiseXor_12_alpha1) * (bitwise_xor_10_1)
        + (VerifyBitwiseXor_12_alpha2) * (bitwise_xor_10_2 + m31(2048).into())
        - (VerifyBitwiseXor_12_z)
}

pub fn intermediate9(
    VerifyBitwiseXor_12_alpha0: QM31,
    VerifyBitwiseXor_12_alpha1: QM31,
    VerifyBitwiseXor_12_alpha2: QM31,
    VerifyBitwiseXor_12_z: QM31,
    bitwise_xor_10_0: QM31,
    bitwise_xor_10_1: QM31,
    bitwise_xor_10_2: QM31,
) -> QM31 {
    (VerifyBitwiseXor_12_alpha0) * (bitwise_xor_10_0 + m31(2048).into())
        + (VerifyBitwiseXor_12_alpha1) * (bitwise_xor_10_1 + m31(1024).into())
        + (VerifyBitwiseXor_12_alpha2) * (bitwise_xor_10_2 + m31(3072).into())
        - (VerifyBitwiseXor_12_z)
}

pub fn intermediate15(
    VerifyBitwiseXor_12_alpha0: QM31,
    VerifyBitwiseXor_12_alpha1: QM31,
    VerifyBitwiseXor_12_alpha2: QM31,
    VerifyBitwiseXor_12_z: QM31,
    bitwise_xor_10_0: QM31,
    bitwise_xor_10_1: QM31,
    bitwise_xor_10_2: QM31,
) -> QM31 {
    (VerifyBitwiseXor_12_alpha0) * (bitwise_xor_10_0 + m31(3072).into())
        + (VerifyBitwiseXor_12_alpha1) * (bitwise_xor_10_1 + m31(3072).into())
        + (VerifyBitwiseXor_12_alpha2) * (bitwise_xor_10_2)
        - (VerifyBitwiseXor_12_z)
}

