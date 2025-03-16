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
    pub RangeCheckFelt252Width27_alpha0: QM31,
    pub RangeCheckFelt252Width27_alpha1: QM31,
    pub RangeCheckFelt252Width27_alpha2: QM31,
    pub RangeCheckFelt252Width27_alpha3: QM31,
    pub RangeCheckFelt252Width27_alpha4: QM31,
    pub RangeCheckFelt252Width27_alpha5: QM31,
    pub RangeCheckFelt252Width27_alpha6: QM31,
    pub RangeCheckFelt252Width27_alpha7: QM31,
    pub RangeCheckFelt252Width27_alpha8: QM31,
    pub RangeCheckFelt252Width27_alpha9: QM31,
    pub RangeCheckFelt252Width27_z: QM31,
    pub RangeCheck_18_alpha0: QM31,
    pub RangeCheck_18_z: QM31,
    pub RangeCheck_9_9_alpha0: QM31,
    pub RangeCheck_9_9_alpha1: QM31,
    pub RangeCheck_9_9_z: QM31,
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
        RangeCheckFelt252Width27_alpha0,
        RangeCheckFelt252Width27_alpha1,
        RangeCheckFelt252Width27_alpha2,
        RangeCheckFelt252Width27_alpha3,
        RangeCheckFelt252Width27_alpha4,
        RangeCheckFelt252Width27_alpha5,
        RangeCheckFelt252Width27_alpha6,
        RangeCheckFelt252Width27_alpha7,
        RangeCheckFelt252Width27_alpha8,
        RangeCheckFelt252Width27_alpha9,
        RangeCheckFelt252Width27_z,
        RangeCheck_18_alpha0,
        RangeCheck_18_z,
        RangeCheck_9_9_alpha0,
        RangeCheck_9_9_alpha1,
        RangeCheck_9_9_z,
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
    ]: [Span<QM31>; 20] =
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

    let [
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
        trace_2_column_48,
        trace_2_column_49,
        trace_2_column_50,
        trace_2_column_51,
    ]: [Span<QM31>; 32] =
        (*interaction_mask_values
        .multi_pop_front()
        .unwrap())
        .unbox();

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

    let [trace_2_column_44_offset_0]: [QM31; 1] = (*trace_2_column_44.try_into().unwrap()).unbox();

    let [trace_2_column_45_offset_0]: [QM31; 1] = (*trace_2_column_45.try_into().unwrap()).unbox();

    let [trace_2_column_46_offset_0]: [QM31; 1] = (*trace_2_column_46.try_into().unwrap()).unbox();

    let [trace_2_column_47_offset_0]: [QM31; 1] = (*trace_2_column_47.try_into().unwrap()).unbox();

    let [trace_2_column_48_offset_neg_1, trace_2_column_48_offset_0]: [QM31; 2] =
        (*trace_2_column_48
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_49_offset_neg_1, trace_2_column_49_offset_0]: [QM31; 2] =
        (*trace_2_column_49
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_50_offset_neg_1, trace_2_column_50_offset_0]: [QM31; 2] =
        (*trace_2_column_50
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_51_offset_neg_1, trace_2_column_51_offset_0]: [QM31; 2] =
        (*trace_2_column_51
        .try_into()
        .unwrap())
        .unbox();

    core::internal::revoke_ap_tracking();

    let mut intermediates = intermediates(
        RangeCheckFelt252Width27_alpha0,
        RangeCheckFelt252Width27_alpha1,
        RangeCheckFelt252Width27_alpha2,
        RangeCheckFelt252Width27_alpha3,
        RangeCheckFelt252Width27_alpha4,
        RangeCheckFelt252Width27_alpha5,
        RangeCheckFelt252Width27_alpha6,
        RangeCheckFelt252Width27_alpha7,
        RangeCheckFelt252Width27_alpha8,
        RangeCheckFelt252Width27_alpha9,
        RangeCheckFelt252Width27_z,
        RangeCheck_18_alpha0,
        RangeCheck_18_z,
        RangeCheck_9_9_alpha0,
        RangeCheck_9_9_alpha1,
        RangeCheck_9_9_z,
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

    // Constraint 0
    let constraint_quotient = ((trace_1_column_19_offset_0) * (trace_1_column_19_offset_0)
        - (trace_1_column_19_offset_0))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 1
    let constraint_quotient = ((QM31Trait::from_partial_evals(
        [
            trace_2_column_20_offset_0, trace_2_column_21_offset_0, trace_2_column_22_offset_0,
            trace_2_column_23_offset_0,
        ],
    ))
        * ((intermediate0) * (intermediate1))
        - (intermediate1 + intermediate0))
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
        * ((intermediate2) * (intermediate3))
        - (intermediate3 + intermediate2))
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
        * ((intermediate4) * (intermediate5))
        - (intermediate5 + intermediate4))
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
        * ((intermediate6) * (intermediate7))
        - (intermediate7 + intermediate6))
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
        * ((intermediate8) * (intermediate9))
        - (intermediate9 + intermediate8))
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
        * ((intermediate10) * (intermediate11))
        - (intermediate11 + intermediate10))
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
                trace_2_column_40_offset_0, trace_2_column_41_offset_0, trace_2_column_42_offset_0,
                trace_2_column_43_offset_0,
            ],
        )))
        * ((intermediate12) * (intermediate13))
        - (intermediate13 + intermediate12))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 8
    let constraint_quotient = ((QM31Trait::from_partial_evals(
        [
            trace_2_column_48_offset_0, trace_2_column_49_offset_0, trace_2_column_50_offset_0,
            trace_2_column_51_offset_0,
        ],
    )
        - (QM31Trait::from_partial_evals(
            [
                trace_2_column_48_offset_neg_1, trace_2_column_49_offset_neg_1,
                trace_2_column_50_offset_neg_1, trace_2_column_51_offset_neg_1,
            ],
        ))
        - (QM31Trait::from_partial_evals(
            [
                trace_2_column_44_offset_0, trace_2_column_45_offset_0, trace_2_column_46_offset_0,
                trace_2_column_47_offset_0,
            ],
        ))
        + (claimed_sum) * (column_size.inverse().into()))
        * (intermediate14)
        + trace_1_column_19_offset_0)
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;
}


fn intermediates(
    RangeCheckFelt252Width27_alpha0: QM31,
    RangeCheckFelt252Width27_alpha1: QM31,
    RangeCheckFelt252Width27_alpha2: QM31,
    RangeCheckFelt252Width27_alpha3: QM31,
    RangeCheckFelt252Width27_alpha4: QM31,
    RangeCheckFelt252Width27_alpha5: QM31,
    RangeCheckFelt252Width27_alpha6: QM31,
    RangeCheckFelt252Width27_alpha7: QM31,
    RangeCheckFelt252Width27_alpha8: QM31,
    RangeCheckFelt252Width27_alpha9: QM31,
    RangeCheckFelt252Width27_z: QM31,
    RangeCheck_18_alpha0: QM31,
    RangeCheck_18_z: QM31,
    RangeCheck_9_9_alpha0: QM31,
    RangeCheck_9_9_alpha1: QM31,
    RangeCheck_9_9_z: QM31,
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
    let intermediate0 = intermediate0(
        RangeCheck_9_9_alpha0,
        RangeCheck_9_9_alpha1,
        RangeCheck_9_9_z,
        trace_1_column_10_offset_0,
        trace_1_column_11_offset_0,
    );

    let intermediate5 = intermediate5(
        RangeCheck_18_alpha0,
        RangeCheck_18_z,
        trace_1_column_13_offset_0,
        trace_1_column_3_offset_0,
    );

    let intermediate11 = intermediate11(
        RangeCheck_18_alpha0,
        RangeCheck_18_z,
        trace_1_column_17_offset_0,
        trace_1_column_7_offset_0,
    );

    let intermediate6 = intermediate6(
        RangeCheck_9_9_alpha0,
        RangeCheck_9_9_alpha1,
        RangeCheck_9_9_z,
        trace_1_column_14_offset_0,
        trace_1_column_15_offset_0,
    );

    let intermediate8 = intermediate8(
        RangeCheck_18_alpha0,
        RangeCheck_18_z,
        trace_1_column_15_offset_0,
        trace_1_column_5_offset_0,
    );

    let intermediate10 = intermediate10(
        RangeCheck_18_alpha0,
        RangeCheck_18_z,
        trace_1_column_16_offset_0,
        trace_1_column_6_offset_0,
    );

    let intermediate1 = intermediate1(
        RangeCheck_18_alpha0,
        RangeCheck_18_z,
        trace_1_column_0_offset_0,
        trace_1_column_10_offset_0,
    );

    let intermediate4 = intermediate4(
        RangeCheck_18_alpha0,
        RangeCheck_18_z,
        trace_1_column_12_offset_0,
        trace_1_column_2_offset_0,
    );

    let intermediate14 = intermediate14(
        RangeCheckFelt252Width27_alpha0,
        RangeCheckFelt252Width27_alpha1,
        RangeCheckFelt252Width27_alpha2,
        RangeCheckFelt252Width27_alpha3,
        RangeCheckFelt252Width27_alpha4,
        RangeCheckFelt252Width27_alpha5,
        RangeCheckFelt252Width27_alpha6,
        RangeCheckFelt252Width27_alpha7,
        RangeCheckFelt252Width27_alpha8,
        RangeCheckFelt252Width27_alpha9,
        RangeCheckFelt252Width27_z,
        trace_1_column_0_offset_0,
        trace_1_column_1_offset_0,
        trace_1_column_2_offset_0,
        trace_1_column_3_offset_0,
        trace_1_column_4_offset_0,
        trace_1_column_5_offset_0,
        trace_1_column_6_offset_0,
        trace_1_column_7_offset_0,
        trace_1_column_8_offset_0,
        trace_1_column_9_offset_0,
    );

    let intermediate12 = intermediate12(
        RangeCheck_9_9_alpha0,
        RangeCheck_9_9_alpha1,
        RangeCheck_9_9_z,
        trace_1_column_18_offset_0,
        trace_1_column_9_offset_0,
    );

    let intermediate7 = intermediate7(
        RangeCheck_18_alpha0,
        RangeCheck_18_z,
        trace_1_column_14_offset_0,
        trace_1_column_4_offset_0,
    );

    let intermediate9 = intermediate9(
        RangeCheck_9_9_alpha0,
        RangeCheck_9_9_alpha1,
        RangeCheck_9_9_z,
        trace_1_column_16_offset_0,
        trace_1_column_17_offset_0,
    );

    let intermediate13 = intermediate13(
        RangeCheck_18_alpha0,
        RangeCheck_18_z,
        trace_1_column_18_offset_0,
        trace_1_column_8_offset_0,
    );

    let intermediate2 = intermediate2(
        RangeCheck_18_alpha0,
        RangeCheck_18_z,
        trace_1_column_11_offset_0,
        trace_1_column_1_offset_0,
    );

    let intermediate3 = intermediate3(
        RangeCheck_9_9_alpha0,
        RangeCheck_9_9_alpha1,
        RangeCheck_9_9_z,
        trace_1_column_12_offset_0,
        trace_1_column_13_offset_0,
    );
    array![
        intermediate0,
        intermediate1,
        intermediate2,
        intermediate3,
        intermediate4,
        intermediate5,
        intermediate6,
        intermediate7,
        intermediate8,
        intermediate9,
        intermediate10,
        intermediate11,
        intermediate12,
        intermediate13,
        intermediate14,
    ]
}


pub fn intermediate0(
    RangeCheck_9_9_alpha0: QM31,
    RangeCheck_9_9_alpha1: QM31,
    RangeCheck_9_9_z: QM31,
    trace_1_column_10_offset_0: QM31,
    trace_1_column_11_offset_0: QM31,
) -> QM31 {
    (RangeCheck_9_9_alpha0) * (trace_1_column_10_offset_0)
        + (RangeCheck_9_9_alpha1) * (trace_1_column_11_offset_0)
        - (RangeCheck_9_9_z)
}

pub fn intermediate5(
    RangeCheck_18_alpha0: QM31,
    RangeCheck_18_z: QM31,
    trace_1_column_13_offset_0: QM31,
    trace_1_column_3_offset_0: QM31,
) -> QM31 {
    (RangeCheck_18_alpha0)
        * ((trace_1_column_3_offset_0 - (trace_1_column_13_offset_0)) * (m31(4194304).into()))
        - (RangeCheck_18_z)
}

pub fn intermediate11(
    RangeCheck_18_alpha0: QM31,
    RangeCheck_18_z: QM31,
    trace_1_column_17_offset_0: QM31,
    trace_1_column_7_offset_0: QM31,
) -> QM31 {
    (RangeCheck_18_alpha0)
        * ((trace_1_column_7_offset_0 - (trace_1_column_17_offset_0)) * (m31(4194304).into()))
        - (RangeCheck_18_z)
}

pub fn intermediate6(
    RangeCheck_9_9_alpha0: QM31,
    RangeCheck_9_9_alpha1: QM31,
    RangeCheck_9_9_z: QM31,
    trace_1_column_14_offset_0: QM31,
    trace_1_column_15_offset_0: QM31,
) -> QM31 {
    (RangeCheck_9_9_alpha0) * (trace_1_column_14_offset_0)
        + (RangeCheck_9_9_alpha1) * (trace_1_column_15_offset_0)
        - (RangeCheck_9_9_z)
}

pub fn intermediate8(
    RangeCheck_18_alpha0: QM31,
    RangeCheck_18_z: QM31,
    trace_1_column_15_offset_0: QM31,
    trace_1_column_5_offset_0: QM31,
) -> QM31 {
    (RangeCheck_18_alpha0)
        * ((trace_1_column_5_offset_0 - (trace_1_column_15_offset_0)) * (m31(4194304).into()))
        - (RangeCheck_18_z)
}

pub fn intermediate10(
    RangeCheck_18_alpha0: QM31,
    RangeCheck_18_z: QM31,
    trace_1_column_16_offset_0: QM31,
    trace_1_column_6_offset_0: QM31,
) -> QM31 {
    (RangeCheck_18_alpha0)
        * (trace_1_column_6_offset_0 - ((trace_1_column_16_offset_0) * (m31(262144).into())))
        - (RangeCheck_18_z)
}

pub fn intermediate1(
    RangeCheck_18_alpha0: QM31,
    RangeCheck_18_z: QM31,
    trace_1_column_0_offset_0: QM31,
    trace_1_column_10_offset_0: QM31,
) -> QM31 {
    (RangeCheck_18_alpha0)
        * (trace_1_column_0_offset_0 - ((trace_1_column_10_offset_0) * (m31(262144).into())))
        - (RangeCheck_18_z)
}

pub fn intermediate4(
    RangeCheck_18_alpha0: QM31,
    RangeCheck_18_z: QM31,
    trace_1_column_12_offset_0: QM31,
    trace_1_column_2_offset_0: QM31,
) -> QM31 {
    (RangeCheck_18_alpha0)
        * (trace_1_column_2_offset_0 - ((trace_1_column_12_offset_0) * (m31(262144).into())))
        - (RangeCheck_18_z)
}

pub fn intermediate14(
    RangeCheckFelt252Width27_alpha0: QM31,
    RangeCheckFelt252Width27_alpha1: QM31,
    RangeCheckFelt252Width27_alpha2: QM31,
    RangeCheckFelt252Width27_alpha3: QM31,
    RangeCheckFelt252Width27_alpha4: QM31,
    RangeCheckFelt252Width27_alpha5: QM31,
    RangeCheckFelt252Width27_alpha6: QM31,
    RangeCheckFelt252Width27_alpha7: QM31,
    RangeCheckFelt252Width27_alpha8: QM31,
    RangeCheckFelt252Width27_alpha9: QM31,
    RangeCheckFelt252Width27_z: QM31,
    trace_1_column_0_offset_0: QM31,
    trace_1_column_1_offset_0: QM31,
    trace_1_column_2_offset_0: QM31,
    trace_1_column_3_offset_0: QM31,
    trace_1_column_4_offset_0: QM31,
    trace_1_column_5_offset_0: QM31,
    trace_1_column_6_offset_0: QM31,
    trace_1_column_7_offset_0: QM31,
    trace_1_column_8_offset_0: QM31,
    trace_1_column_9_offset_0: QM31,
) -> QM31 {
    (RangeCheckFelt252Width27_alpha0) * (trace_1_column_0_offset_0)
        + (RangeCheckFelt252Width27_alpha1) * (trace_1_column_1_offset_0)
        + (RangeCheckFelt252Width27_alpha2) * (trace_1_column_2_offset_0)
        + (RangeCheckFelt252Width27_alpha3) * (trace_1_column_3_offset_0)
        + (RangeCheckFelt252Width27_alpha4) * (trace_1_column_4_offset_0)
        + (RangeCheckFelt252Width27_alpha5) * (trace_1_column_5_offset_0)
        + (RangeCheckFelt252Width27_alpha6) * (trace_1_column_6_offset_0)
        + (RangeCheckFelt252Width27_alpha7) * (trace_1_column_7_offset_0)
        + (RangeCheckFelt252Width27_alpha8) * (trace_1_column_8_offset_0)
        + (RangeCheckFelt252Width27_alpha9) * (trace_1_column_9_offset_0)
        - (RangeCheckFelt252Width27_z)
}

pub fn intermediate12(
    RangeCheck_9_9_alpha0: QM31,
    RangeCheck_9_9_alpha1: QM31,
    RangeCheck_9_9_z: QM31,
    trace_1_column_18_offset_0: QM31,
    trace_1_column_9_offset_0: QM31,
) -> QM31 {
    (RangeCheck_9_9_alpha0) * (trace_1_column_18_offset_0)
        + (RangeCheck_9_9_alpha1) * (trace_1_column_9_offset_0)
        - (RangeCheck_9_9_z)
}

pub fn intermediate7(
    RangeCheck_18_alpha0: QM31,
    RangeCheck_18_z: QM31,
    trace_1_column_14_offset_0: QM31,
    trace_1_column_4_offset_0: QM31,
) -> QM31 {
    (RangeCheck_18_alpha0)
        * (trace_1_column_4_offset_0 - ((trace_1_column_14_offset_0) * (m31(262144).into())))
        - (RangeCheck_18_z)
}

pub fn intermediate9(
    RangeCheck_9_9_alpha0: QM31,
    RangeCheck_9_9_alpha1: QM31,
    RangeCheck_9_9_z: QM31,
    trace_1_column_16_offset_0: QM31,
    trace_1_column_17_offset_0: QM31,
) -> QM31 {
    (RangeCheck_9_9_alpha0) * (trace_1_column_16_offset_0)
        + (RangeCheck_9_9_alpha1) * (trace_1_column_17_offset_0)
        - (RangeCheck_9_9_z)
}

pub fn intermediate13(
    RangeCheck_18_alpha0: QM31,
    RangeCheck_18_z: QM31,
    trace_1_column_18_offset_0: QM31,
    trace_1_column_8_offset_0: QM31,
) -> QM31 {
    (RangeCheck_18_alpha0)
        * (trace_1_column_8_offset_0 - ((trace_1_column_18_offset_0) * (m31(262144).into())))
        - (RangeCheck_18_z)
}

pub fn intermediate2(
    RangeCheck_18_alpha0: QM31,
    RangeCheck_18_z: QM31,
    trace_1_column_11_offset_0: QM31,
    trace_1_column_1_offset_0: QM31,
) -> QM31 {
    (RangeCheck_18_alpha0)
        * ((trace_1_column_1_offset_0 - (trace_1_column_11_offset_0)) * (m31(4194304).into()))
        - (RangeCheck_18_z)
}

pub fn intermediate3(
    RangeCheck_9_9_alpha0: QM31,
    RangeCheck_9_9_alpha1: QM31,
    RangeCheck_9_9_z: QM31,
    trace_1_column_12_offset_0: QM31,
    trace_1_column_13_offset_0: QM31,
) -> QM31 {
    (RangeCheck_9_9_alpha0) * (trace_1_column_12_offset_0)
        + (RangeCheck_9_9_alpha1) * (trace_1_column_13_offset_0)
        - (RangeCheck_9_9_z)
}

