use stwo_constraint_framework::{
    PreprocessedColumn, PreprocessedColumnSet, PreprocessedColumnSetImpl,
};
use stwo_verifier_core::circle::{
    CirclePoint, CirclePointIndex, CirclePointIndexImpl, CirclePointQM31AddCirclePointM31Impl,
};
use stwo_verifier_core::fields::Invertible;
use stwo_verifier_core::fields::m31::{M31, m31};
use stwo_verifier_core::fields::qm31::{QM31, QM31Trait, qm31_const};
use stwo_verifier_core::{ColumnArray, ColumnSpan};


pub fn mask_points(
    ref preprocessed_column_set: PreprocessedColumnSet,
    ref trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
    ref interaction_trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
    point: CirclePoint<QM31>,
    trace_gen: CirclePointIndex,
    log_size: u32,
) {
    let point_offset_neg_1 = point.add_circle_point_m31(-trace_gen.mul(1).to_point());
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    interaction_trace_mask_points.append(array![point]);
    interaction_trace_mask_points.append(array![point]);
    interaction_trace_mask_points.append(array![point]);
    interaction_trace_mask_points.append(array![point]);
    interaction_trace_mask_points.append(array![point]);
    interaction_trace_mask_points.append(array![point]);
    interaction_trace_mask_points.append(array![point]);
    interaction_trace_mask_points.append(array![point]);
    interaction_trace_mask_points.append(array![point]);
    interaction_trace_mask_points.append(array![point]);
    interaction_trace_mask_points.append(array![point]);
    interaction_trace_mask_points.append(array![point]);
    interaction_trace_mask_points.append(array![point]);
    interaction_trace_mask_points.append(array![point]);
    interaction_trace_mask_points.append(array![point]);
    interaction_trace_mask_points.append(array![point]);
    interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
    interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
    interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
    interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
}

#[derive(Drop)]
pub struct ConstraintParams {
    pub TripleXor32_alpha0: QM31,
    pub TripleXor32_alpha1: QM31,
    pub TripleXor32_alpha2: QM31,
    pub TripleXor32_z: QM31,
    pub VerifyBitwiseXor_8_alpha0: QM31,
    pub VerifyBitwiseXor_8_alpha1: QM31,
    pub VerifyBitwiseXor_8_alpha2: QM31,
    pub VerifyBitwiseXor_8_z: QM31,
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
        TripleXor32_alpha0,
        TripleXor32_alpha1,
        TripleXor32_alpha2,
        TripleXor32_z,
        VerifyBitwiseXor_8_alpha0,
        VerifyBitwiseXor_8_alpha1,
        VerifyBitwiseXor_8_alpha2,
        VerifyBitwiseXor_8_z,
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
        trace_1_column_16,
        trace_1_column_17,
        trace_1_column_18,
        trace_1_column_19,
        trace_1_column_20,
    ]: [Span<QM31>; 21] =
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

    let [trace_1_column_16_offset_0]: [QM31; 1] = (*trace_1_column_16.try_into().unwrap()).unbox();

    let [trace_1_column_17_offset_0]: [QM31; 1] = (*trace_1_column_17.try_into().unwrap()).unbox();

    let [trace_1_column_18_offset_0]: [QM31; 1] = (*trace_1_column_18.try_into().unwrap()).unbox();

    let [trace_1_column_19_offset_0]: [QM31; 1] = (*trace_1_column_19.try_into().unwrap()).unbox();

    let [trace_1_column_20_offset_0]: [QM31; 1] = (*trace_1_column_20.try_into().unwrap()).unbox();

    let [
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
    ]: [Span<QM31>; 20] =
        (*interaction_mask_values
        .multi_pop_front()
        .unwrap())
        .unbox();

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

    let [trace_2_column_37_offset_neg_1, trace_2_column_37_offset_0]: [QM31; 2] =
        (*trace_2_column_37
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_38_offset_neg_1, trace_2_column_38_offset_0]: [QM31; 2] =
        (*trace_2_column_38
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_39_offset_neg_1, trace_2_column_39_offset_0]: [QM31; 2] =
        (*trace_2_column_39
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_40_offset_neg_1, trace_2_column_40_offset_0]: [QM31; 2] =
        (*trace_2_column_40
        .try_into()
        .unwrap())
        .unbox();

    core::internal::revoke_ap_tracking();

    let mut intermediates = intermediates(
        TripleXor32_alpha0,
        TripleXor32_alpha1,
        TripleXor32_alpha2,
        TripleXor32_z,
        VerifyBitwiseXor_8_alpha0,
        VerifyBitwiseXor_8_alpha1,
        VerifyBitwiseXor_8_alpha2,
        VerifyBitwiseXor_8_z,
        trace_1_column_0_offset_0,
        trace_1_column_10_offset_0,
        trace_1_column_11_offset_0,
        trace_1_column_12_offset_0,
        trace_1_column_13_offset_0,
        trace_1_column_14_offset_0,
        trace_1_column_15_offset_0,
        trace_1_column_16_offset_0,
        trace_1_column_17_offset_0,
        trace_1_column_18_offset_0,
        trace_1_column_19_offset_0,
        trace_1_column_1_offset_0,
        trace_1_column_2_offset_0,
        trace_1_column_3_offset_0,
        trace_1_column_4_offset_0,
        trace_1_column_5_offset_0,
        trace_1_column_6_offset_0,
        trace_1_column_7_offset_0,
        trace_1_column_8_offset_0,
        trace_1_column_9_offset_0,
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
    let intermediate16 = *intermediates.pop_front().unwrap();

    // Constraint 0
    let constraint_quotient = ((trace_1_column_20_offset_0) * (trace_1_column_20_offset_0)
        - (trace_1_column_20_offset_0))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 1
    let constraint_quotient = ((QM31Trait::from_partial_evals(
        [
            trace_2_column_21_offset_0, trace_2_column_22_offset_0, trace_2_column_23_offset_0,
            trace_2_column_24_offset_0,
        ],
    ))
        * ((intermediate6) * (intermediate7))
        - (intermediate7 + intermediate6))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 2
    let constraint_quotient = ((QM31Trait::from_partial_evals(
        [
            trace_2_column_25_offset_0, trace_2_column_26_offset_0, trace_2_column_27_offset_0,
            trace_2_column_28_offset_0,
        ],
    )
        - (QM31Trait::from_partial_evals(
            [
                trace_2_column_21_offset_0, trace_2_column_22_offset_0, trace_2_column_23_offset_0,
                trace_2_column_24_offset_0,
            ],
        )))
        * ((intermediate8) * (intermediate9))
        - (intermediate9 + intermediate8))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 3
    let constraint_quotient = ((QM31Trait::from_partial_evals(
        [
            trace_2_column_29_offset_0, trace_2_column_30_offset_0, trace_2_column_31_offset_0,
            trace_2_column_32_offset_0,
        ],
    )
        - (QM31Trait::from_partial_evals(
            [
                trace_2_column_25_offset_0, trace_2_column_26_offset_0, trace_2_column_27_offset_0,
                trace_2_column_28_offset_0,
            ],
        )))
        * ((intermediate10) * (intermediate11))
        - (intermediate11 + intermediate10))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 4
    let constraint_quotient = ((QM31Trait::from_partial_evals(
        [
            trace_2_column_33_offset_0, trace_2_column_34_offset_0, trace_2_column_35_offset_0,
            trace_2_column_36_offset_0,
        ],
    )
        - (QM31Trait::from_partial_evals(
            [
                trace_2_column_29_offset_0, trace_2_column_30_offset_0, trace_2_column_31_offset_0,
                trace_2_column_32_offset_0,
            ],
        )))
        * ((intermediate12) * (intermediate13))
        - (intermediate13 + intermediate12))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 5
    let constraint_quotient = ((QM31Trait::from_partial_evals(
        [
            trace_2_column_37_offset_0, trace_2_column_38_offset_0, trace_2_column_39_offset_0,
            trace_2_column_40_offset_0,
        ],
    )
        - (QM31Trait::from_partial_evals(
            [
                trace_2_column_37_offset_neg_1, trace_2_column_38_offset_neg_1,
                trace_2_column_39_offset_neg_1, trace_2_column_40_offset_neg_1,
            ],
        ))
        - (QM31Trait::from_partial_evals(
            [
                trace_2_column_33_offset_0, trace_2_column_34_offset_0, trace_2_column_35_offset_0,
                trace_2_column_36_offset_0,
            ],
        ))
        + (claimed_sum) * (column_size.inverse().into()))
        * (intermediate16)
        + trace_1_column_20_offset_0)
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;
}


fn intermediates(
    TripleXor32_alpha0: QM31,
    TripleXor32_alpha1: QM31,
    TripleXor32_alpha2: QM31,
    TripleXor32_z: QM31,
    VerifyBitwiseXor_8_alpha0: QM31,
    VerifyBitwiseXor_8_alpha1: QM31,
    VerifyBitwiseXor_8_alpha2: QM31,
    VerifyBitwiseXor_8_z: QM31,
    trace_1_column_0_offset_0: QM31,
    trace_1_column_10_offset_0: QM31,
    trace_1_column_11_offset_0: QM31,
    trace_1_column_12_offset_0: QM31,
    trace_1_column_13_offset_0: QM31,
    trace_1_column_14_offset_0: QM31,
    trace_1_column_15_offset_0: QM31,
    trace_1_column_16_offset_0: QM31,
    trace_1_column_17_offset_0: QM31,
    trace_1_column_18_offset_0: QM31,
    trace_1_column_19_offset_0: QM31,
    trace_1_column_1_offset_0: QM31,
    trace_1_column_2_offset_0: QM31,
    trace_1_column_3_offset_0: QM31,
    trace_1_column_4_offset_0: QM31,
    trace_1_column_5_offset_0: QM31,
    trace_1_column_6_offset_0: QM31,
    trace_1_column_7_offset_0: QM31,
    trace_1_column_8_offset_0: QM31,
    trace_1_column_9_offset_0: QM31,
) -> Array<QM31> {
    let intermediate0 = intermediate0(trace_1_column_0_offset_0, trace_1_column_6_offset_0);

    let intermediate1 = intermediate1(trace_1_column_1_offset_0, trace_1_column_7_offset_0);

    let intermediate2 = intermediate2(trace_1_column_2_offset_0, trace_1_column_8_offset_0);

    let intermediate3 = intermediate3(trace_1_column_3_offset_0, trace_1_column_9_offset_0);

    let intermediate4 = intermediate4(trace_1_column_10_offset_0, trace_1_column_4_offset_0);

    let intermediate5 = intermediate5(trace_1_column_11_offset_0, trace_1_column_5_offset_0);

    let intermediate14 = intermediate14(trace_1_column_13_offset_0, trace_1_column_15_offset_0);

    let intermediate15 = intermediate15(trace_1_column_17_offset_0, trace_1_column_19_offset_0);
    let intermediate10 = intermediate10(
        VerifyBitwiseXor_8_alpha0,
        VerifyBitwiseXor_8_alpha1,
        VerifyBitwiseXor_8_alpha2,
        VerifyBitwiseXor_8_z,
        intermediate1,
        intermediate3,
        trace_1_column_16_offset_0,
    );

    let intermediate13 = intermediate13(
        VerifyBitwiseXor_8_alpha0,
        VerifyBitwiseXor_8_alpha1,
        VerifyBitwiseXor_8_alpha2,
        VerifyBitwiseXor_8_z,
        trace_1_column_11_offset_0,
        trace_1_column_18_offset_0,
        trace_1_column_19_offset_0,
    );

    let intermediate8 = intermediate8(
        VerifyBitwiseXor_8_alpha0,
        VerifyBitwiseXor_8_alpha1,
        VerifyBitwiseXor_8_alpha2,
        VerifyBitwiseXor_8_z,
        trace_1_column_14_offset_0,
        trace_1_column_6_offset_0,
        trace_1_column_8_offset_0,
    );

    let intermediate16 = intermediate16(
        TripleXor32_alpha0,
        TripleXor32_alpha1,
        TripleXor32_alpha2,
        TripleXor32_z,
        trace_1_column_0_offset_0,
        trace_1_column_1_offset_0,
        trace_1_column_2_offset_0,
    );

    let intermediate9 = intermediate9(
        VerifyBitwiseXor_8_alpha0,
        VerifyBitwiseXor_8_alpha1,
        VerifyBitwiseXor_8_alpha2,
        VerifyBitwiseXor_8_z,
        trace_1_column_10_offset_0,
        trace_1_column_14_offset_0,
        trace_1_column_15_offset_0,
    );

    let intermediate11 = intermediate11(
        VerifyBitwiseXor_8_alpha0,
        VerifyBitwiseXor_8_alpha1,
        VerifyBitwiseXor_8_alpha2,
        VerifyBitwiseXor_8_z,
        intermediate5,
        trace_1_column_16_offset_0,
        trace_1_column_17_offset_0,
    );

    let intermediate12 = intermediate12(
        VerifyBitwiseXor_8_alpha0,
        VerifyBitwiseXor_8_alpha1,
        VerifyBitwiseXor_8_alpha2,
        VerifyBitwiseXor_8_z,
        trace_1_column_18_offset_0,
        trace_1_column_7_offset_0,
        trace_1_column_9_offset_0,
    );

    let intermediate7 = intermediate7(
        VerifyBitwiseXor_8_alpha0,
        VerifyBitwiseXor_8_alpha1,
        VerifyBitwiseXor_8_alpha2,
        VerifyBitwiseXor_8_z,
        intermediate4,
        trace_1_column_12_offset_0,
        trace_1_column_13_offset_0,
    );

    let intermediate6 = intermediate6(
        VerifyBitwiseXor_8_alpha0,
        VerifyBitwiseXor_8_alpha1,
        VerifyBitwiseXor_8_alpha2,
        VerifyBitwiseXor_8_z,
        intermediate0,
        intermediate2,
        trace_1_column_12_offset_0,
    );
    array![
        intermediate0, intermediate1, intermediate2, intermediate3, intermediate4, intermediate5,
        intermediate6, intermediate7, intermediate8, intermediate9, intermediate10, intermediate11,
        intermediate12, intermediate13, intermediate14, intermediate15, intermediate16,
    ]
}

pub fn intermediate5(trace_1_column_11_offset_0: QM31, trace_1_column_5_offset_0: QM31) -> QM31 {
    trace_1_column_5_offset_0 - ((trace_1_column_11_offset_0) * (m31(256).into()))
}

pub fn intermediate1(trace_1_column_1_offset_0: QM31, trace_1_column_7_offset_0: QM31) -> QM31 {
    trace_1_column_1_offset_0 - ((trace_1_column_7_offset_0) * (m31(256).into()))
}

pub fn intermediate0(trace_1_column_0_offset_0: QM31, trace_1_column_6_offset_0: QM31) -> QM31 {
    trace_1_column_0_offset_0 - ((trace_1_column_6_offset_0) * (m31(256).into()))
}

pub fn intermediate2(trace_1_column_2_offset_0: QM31, trace_1_column_8_offset_0: QM31) -> QM31 {
    trace_1_column_2_offset_0 - ((trace_1_column_8_offset_0) * (m31(256).into()))
}

pub fn intermediate15(trace_1_column_17_offset_0: QM31, trace_1_column_19_offset_0: QM31) -> QM31 {
    trace_1_column_17_offset_0 + (trace_1_column_19_offset_0) * (m31(256).into())
}

pub fn intermediate4(trace_1_column_10_offset_0: QM31, trace_1_column_4_offset_0: QM31) -> QM31 {
    trace_1_column_4_offset_0 - ((trace_1_column_10_offset_0) * (m31(256).into()))
}

pub fn intermediate14(trace_1_column_13_offset_0: QM31, trace_1_column_15_offset_0: QM31) -> QM31 {
    trace_1_column_13_offset_0 + (trace_1_column_15_offset_0) * (m31(256).into())
}

pub fn intermediate3(trace_1_column_3_offset_0: QM31, trace_1_column_9_offset_0: QM31) -> QM31 {
    trace_1_column_3_offset_0 - ((trace_1_column_9_offset_0) * (m31(256).into()))
}
pub fn intermediate10(
    VerifyBitwiseXor_8_alpha0: QM31,
    VerifyBitwiseXor_8_alpha1: QM31,
    VerifyBitwiseXor_8_alpha2: QM31,
    VerifyBitwiseXor_8_z: QM31,
    intermediate1: QM31,
    intermediate3: QM31,
    trace_1_column_16_offset_0: QM31,
) -> QM31 {
    (VerifyBitwiseXor_8_alpha0) * (intermediate1)
        + (VerifyBitwiseXor_8_alpha1) * (intermediate3)
        + (VerifyBitwiseXor_8_alpha2) * (trace_1_column_16_offset_0)
        - (VerifyBitwiseXor_8_z)
}

pub fn intermediate13(
    VerifyBitwiseXor_8_alpha0: QM31,
    VerifyBitwiseXor_8_alpha1: QM31,
    VerifyBitwiseXor_8_alpha2: QM31,
    VerifyBitwiseXor_8_z: QM31,
    trace_1_column_11_offset_0: QM31,
    trace_1_column_18_offset_0: QM31,
    trace_1_column_19_offset_0: QM31,
) -> QM31 {
    (VerifyBitwiseXor_8_alpha0) * (trace_1_column_18_offset_0)
        + (VerifyBitwiseXor_8_alpha1) * (trace_1_column_11_offset_0)
        + (VerifyBitwiseXor_8_alpha2) * (trace_1_column_19_offset_0)
        - (VerifyBitwiseXor_8_z)
}

pub fn intermediate8(
    VerifyBitwiseXor_8_alpha0: QM31,
    VerifyBitwiseXor_8_alpha1: QM31,
    VerifyBitwiseXor_8_alpha2: QM31,
    VerifyBitwiseXor_8_z: QM31,
    trace_1_column_14_offset_0: QM31,
    trace_1_column_6_offset_0: QM31,
    trace_1_column_8_offset_0: QM31,
) -> QM31 {
    (VerifyBitwiseXor_8_alpha0) * (trace_1_column_6_offset_0)
        + (VerifyBitwiseXor_8_alpha1) * (trace_1_column_8_offset_0)
        + (VerifyBitwiseXor_8_alpha2) * (trace_1_column_14_offset_0)
        - (VerifyBitwiseXor_8_z)
}

pub fn intermediate16(
    TripleXor32_alpha0: QM31,
    TripleXor32_alpha1: QM31,
    TripleXor32_alpha2: QM31,
    TripleXor32_z: QM31,
    trace_1_column_0_offset_0: QM31,
    trace_1_column_1_offset_0: QM31,
    trace_1_column_2_offset_0: QM31,
) -> QM31 {
    (TripleXor32_alpha0) * (trace_1_column_0_offset_0)
        + (TripleXor32_alpha1) * (trace_1_column_1_offset_0)
        + (TripleXor32_alpha2) * (trace_1_column_2_offset_0)
        - (TripleXor32_z)
}

pub fn intermediate9(
    VerifyBitwiseXor_8_alpha0: QM31,
    VerifyBitwiseXor_8_alpha1: QM31,
    VerifyBitwiseXor_8_alpha2: QM31,
    VerifyBitwiseXor_8_z: QM31,
    trace_1_column_10_offset_0: QM31,
    trace_1_column_14_offset_0: QM31,
    trace_1_column_15_offset_0: QM31,
) -> QM31 {
    (VerifyBitwiseXor_8_alpha0) * (trace_1_column_14_offset_0)
        + (VerifyBitwiseXor_8_alpha1) * (trace_1_column_10_offset_0)
        + (VerifyBitwiseXor_8_alpha2) * (trace_1_column_15_offset_0)
        - (VerifyBitwiseXor_8_z)
}

pub fn intermediate11(
    VerifyBitwiseXor_8_alpha0: QM31,
    VerifyBitwiseXor_8_alpha1: QM31,
    VerifyBitwiseXor_8_alpha2: QM31,
    VerifyBitwiseXor_8_z: QM31,
    intermediate5: QM31,
    trace_1_column_16_offset_0: QM31,
    trace_1_column_17_offset_0: QM31,
) -> QM31 {
    (VerifyBitwiseXor_8_alpha0) * (trace_1_column_16_offset_0)
        + (VerifyBitwiseXor_8_alpha1) * (intermediate5)
        + (VerifyBitwiseXor_8_alpha2) * (trace_1_column_17_offset_0)
        - (VerifyBitwiseXor_8_z)
}

pub fn intermediate12(
    VerifyBitwiseXor_8_alpha0: QM31,
    VerifyBitwiseXor_8_alpha1: QM31,
    VerifyBitwiseXor_8_alpha2: QM31,
    VerifyBitwiseXor_8_z: QM31,
    trace_1_column_18_offset_0: QM31,
    trace_1_column_7_offset_0: QM31,
    trace_1_column_9_offset_0: QM31,
) -> QM31 {
    (VerifyBitwiseXor_8_alpha0) * (trace_1_column_7_offset_0)
        + (VerifyBitwiseXor_8_alpha1) * (trace_1_column_9_offset_0)
        + (VerifyBitwiseXor_8_alpha2) * (trace_1_column_18_offset_0)
        - (VerifyBitwiseXor_8_z)
}

pub fn intermediate7(
    VerifyBitwiseXor_8_alpha0: QM31,
    VerifyBitwiseXor_8_alpha1: QM31,
    VerifyBitwiseXor_8_alpha2: QM31,
    VerifyBitwiseXor_8_z: QM31,
    intermediate4: QM31,
    trace_1_column_12_offset_0: QM31,
    trace_1_column_13_offset_0: QM31,
) -> QM31 {
    (VerifyBitwiseXor_8_alpha0) * (trace_1_column_12_offset_0)
        + (VerifyBitwiseXor_8_alpha1) * (intermediate4)
        + (VerifyBitwiseXor_8_alpha2) * (trace_1_column_13_offset_0)
        - (VerifyBitwiseXor_8_z)
}

pub fn intermediate6(
    VerifyBitwiseXor_8_alpha0: QM31,
    VerifyBitwiseXor_8_alpha1: QM31,
    VerifyBitwiseXor_8_alpha2: QM31,
    VerifyBitwiseXor_8_z: QM31,
    intermediate0: QM31,
    intermediate2: QM31,
    trace_1_column_12_offset_0: QM31,
) -> QM31 {
    (VerifyBitwiseXor_8_alpha0) * (intermediate0)
        + (VerifyBitwiseXor_8_alpha1) * (intermediate2)
        + (VerifyBitwiseXor_8_alpha2) * (trace_1_column_12_offset_0)
        - (VerifyBitwiseXor_8_z)
}

