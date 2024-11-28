use stwo_verifier_core::circle::{
    CirclePoint, CirclePointIndex, CirclePointIndexImpl, CirclePointQM31AddCirclePointM31Impl,
};
use stwo_verifier_core::fields::m31::{M31, m31};
use stwo_verifier_core::fields::qm31::{QM31, QM31Impl, qm31};

use stwo_verifier_core::{ColumnArray, ColumnSpan};


pub fn mask_points(
    ref trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
    ref interaction_trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
    point: CirclePoint<QM31>,
    trace_gen: CirclePointIndex,
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
    pub IdToValue_alpha0: QM31,
    pub IdToValue_alpha1: QM31,
    pub IdToValue_alpha2: QM31,
    pub IdToValue_alpha3: QM31,
    pub IdToValue_alpha4: QM31,
    pub IdToValue_alpha5: QM31,
    pub IdToValue_alpha6: QM31,
    pub IdToValue_alpha7: QM31,
    pub IdToValue_alpha8: QM31,
    pub IdToValue_z: QM31,
    pub RangeCheck_9_9_alpha0: QM31,
    pub RangeCheck_9_9_alpha1: QM31,
    pub RangeCheck_9_9_z: QM31,
    pub preprocessed_is_first: QM31,
    pub total_sum: QM31,
}

pub fn evaluate_constraints_at_point(
    ref sum: QM31,
    ref trace_mask_values: ColumnSpan<Array<QM31>>,
    ref interaction_mask_values: ColumnSpan<Array<QM31>>,
    params: ConstraintParams,
    random_coeff: QM31,
    domain_vanish_at_point_inv: QM31,
) {
    let ConstraintParams {
        IdToValue_alpha0,
        IdToValue_alpha1,
        IdToValue_alpha2,
        IdToValue_alpha3,
        IdToValue_alpha4,
        IdToValue_alpha5,
        IdToValue_alpha6,
        IdToValue_alpha7,
        IdToValue_alpha8,
        IdToValue_z,
        RangeCheck_9_9_alpha0,
        RangeCheck_9_9_alpha1,
        RangeCheck_9_9_z,
        preprocessed_is_first,
        total_sum,
    } = params;

    let mut trace_1_column_0 = trace_mask_values.pop_front().unwrap().span();
    let trace_1_column_0_offset_0 = *trace_1_column_0.pop_front().unwrap();
    let mut trace_1_column_1 = trace_mask_values.pop_front().unwrap().span();
    let trace_1_column_1_offset_0 = *trace_1_column_1.pop_front().unwrap();
    let mut trace_1_column_2 = trace_mask_values.pop_front().unwrap().span();
    let trace_1_column_2_offset_0 = *trace_1_column_2.pop_front().unwrap();
    let mut trace_1_column_3 = trace_mask_values.pop_front().unwrap().span();
    let trace_1_column_3_offset_0 = *trace_1_column_3.pop_front().unwrap();
    let mut trace_1_column_4 = trace_mask_values.pop_front().unwrap().span();
    let trace_1_column_4_offset_0 = *trace_1_column_4.pop_front().unwrap();
    let mut trace_1_column_5 = trace_mask_values.pop_front().unwrap().span();
    let trace_1_column_5_offset_0 = *trace_1_column_5.pop_front().unwrap();
    let mut trace_1_column_6 = trace_mask_values.pop_front().unwrap().span();
    let trace_1_column_6_offset_0 = *trace_1_column_6.pop_front().unwrap();
    let mut trace_1_column_7 = trace_mask_values.pop_front().unwrap().span();
    let trace_1_column_7_offset_0 = *trace_1_column_7.pop_front().unwrap();
    let mut trace_1_column_8 = trace_mask_values.pop_front().unwrap().span();
    let trace_1_column_8_offset_0 = *trace_1_column_8.pop_front().unwrap();
    let mut trace_1_column_9 = trace_mask_values.pop_front().unwrap().span();
    let trace_1_column_9_offset_0 = *trace_1_column_9.pop_front().unwrap();
    let mut trace_2_column_10 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_10_offset_0 = *trace_2_column_10.pop_front().unwrap();
    let mut trace_2_column_11 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_11_offset_0 = *trace_2_column_11.pop_front().unwrap();
    let mut trace_2_column_12 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_12_offset_0 = *trace_2_column_12.pop_front().unwrap();
    let mut trace_2_column_13 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_13_offset_0 = *trace_2_column_13.pop_front().unwrap();
    let mut trace_2_column_14 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_14_offset_0 = *trace_2_column_14.pop_front().unwrap();
    let mut trace_2_column_15 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_15_offset_0 = *trace_2_column_15.pop_front().unwrap();
    let mut trace_2_column_16 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_16_offset_0 = *trace_2_column_16.pop_front().unwrap();
    let mut trace_2_column_17 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_17_offset_0 = *trace_2_column_17.pop_front().unwrap();
    let mut trace_2_column_18 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_18_offset_0 = *trace_2_column_18.pop_front().unwrap();
    let mut trace_2_column_19 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_19_offset_0 = *trace_2_column_19.pop_front().unwrap();
    let mut trace_2_column_20 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_20_offset_0 = *trace_2_column_20.pop_front().unwrap();
    let mut trace_2_column_21 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_21_offset_0 = *trace_2_column_21.pop_front().unwrap();
    let mut trace_2_column_22 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_22_offset_0 = *trace_2_column_22.pop_front().unwrap();
    let mut trace_2_column_23 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_23_offset_0 = *trace_2_column_23.pop_front().unwrap();
    let mut trace_2_column_24 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_24_offset_0 = *trace_2_column_24.pop_front().unwrap();
    let mut trace_2_column_25 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_25_offset_0 = *trace_2_column_25.pop_front().unwrap();
    let mut trace_2_column_26 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_26_offset_neg_1 = *trace_2_column_26.pop_front().unwrap();
    let trace_2_column_26_offset_0 = *trace_2_column_26.pop_front().unwrap();
    let mut trace_2_column_27 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_27_offset_neg_1 = *trace_2_column_27.pop_front().unwrap();
    let trace_2_column_27_offset_0 = *trace_2_column_27.pop_front().unwrap();
    let mut trace_2_column_28 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_28_offset_neg_1 = *trace_2_column_28.pop_front().unwrap();
    let trace_2_column_28_offset_0 = *trace_2_column_28.pop_front().unwrap();
    let mut trace_2_column_29 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_29_offset_neg_1 = *trace_2_column_29.pop_front().unwrap();
    let trace_2_column_29_offset_0 = *trace_2_column_29.pop_front().unwrap();
    let intermediate0 = (IdToValue_alpha0) * (trace_1_column_0_offset_0)
        + (IdToValue_alpha1) * (trace_1_column_1_offset_0)
        + (IdToValue_alpha2) * (trace_1_column_2_offset_0)
        + (IdToValue_alpha3) * (trace_1_column_3_offset_0)
        + (IdToValue_alpha4) * (trace_1_column_4_offset_0)
        + (IdToValue_alpha5) * (trace_1_column_5_offset_0)
        + (IdToValue_alpha6) * (trace_1_column_6_offset_0)
        + (IdToValue_alpha7) * (trace_1_column_7_offset_0)
        + (IdToValue_alpha8) * (trace_1_column_8_offset_0)
        - (IdToValue_z);

    let intermediate1 = (RangeCheck_9_9_alpha0) * (trace_1_column_1_offset_0)
        + (RangeCheck_9_9_alpha1) * (trace_1_column_2_offset_0)
        - (RangeCheck_9_9_z);

    let intermediate2 = (RangeCheck_9_9_alpha0) * (trace_1_column_3_offset_0)
        + (RangeCheck_9_9_alpha1) * (trace_1_column_4_offset_0)
        - (RangeCheck_9_9_z);

    let intermediate3 = (RangeCheck_9_9_alpha0) * (trace_1_column_5_offset_0)
        + (RangeCheck_9_9_alpha1) * (trace_1_column_6_offset_0)
        - (RangeCheck_9_9_z);

    let intermediate4 = (RangeCheck_9_9_alpha0) * (trace_1_column_7_offset_0)
        + (RangeCheck_9_9_alpha1) * (trace_1_column_8_offset_0)
        - (RangeCheck_9_9_z);

    let constraint_0 = (QM31Impl::from_partial_evals(
        [
            trace_2_column_10_offset_0, trace_2_column_11_offset_0, trace_2_column_12_offset_0,
            trace_2_column_13_offset_0,
        ],
    ))
        * (intermediate0)
        - (-(trace_1_column_9_offset_0));

    let constraint_1 = (QM31Impl::from_partial_evals(
        [
            trace_2_column_14_offset_0, trace_2_column_15_offset_0, trace_2_column_16_offset_0,
            trace_2_column_17_offset_0,
        ],
    )
        - (QM31Impl::from_partial_evals(
            [
                trace_2_column_10_offset_0, trace_2_column_11_offset_0, trace_2_column_12_offset_0,
                trace_2_column_13_offset_0,
            ],
        )))
        * (intermediate1)
        - (qm31(1, 0, 0, 0));

    let constraint_2 = (QM31Impl::from_partial_evals(
        [
            trace_2_column_18_offset_0, trace_2_column_19_offset_0, trace_2_column_20_offset_0,
            trace_2_column_21_offset_0,
        ],
    )
        - (QM31Impl::from_partial_evals(
            [
                trace_2_column_14_offset_0, trace_2_column_15_offset_0, trace_2_column_16_offset_0,
                trace_2_column_17_offset_0,
            ],
        )))
        * (intermediate2)
        - (qm31(1, 0, 0, 0));

    let constraint_3 = (QM31Impl::from_partial_evals(
        [
            trace_2_column_22_offset_0, trace_2_column_23_offset_0, trace_2_column_24_offset_0,
            trace_2_column_25_offset_0,
        ],
    )
        - (QM31Impl::from_partial_evals(
            [
                trace_2_column_18_offset_0, trace_2_column_19_offset_0, trace_2_column_20_offset_0,
                trace_2_column_21_offset_0,
            ],
        )))
        * (intermediate3)
        - (qm31(1, 0, 0, 0));

    let constraint_4 = (QM31Impl::from_partial_evals(
        [
            trace_2_column_26_offset_0, trace_2_column_27_offset_0, trace_2_column_28_offset_0,
            trace_2_column_29_offset_0,
        ],
    )
        - (QM31Impl::from_partial_evals(
            [
                trace_2_column_26_offset_neg_1, trace_2_column_27_offset_neg_1,
                trace_2_column_28_offset_neg_1, trace_2_column_29_offset_neg_1,
            ],
        )
            - ((total_sum) * (preprocessed_is_first)))
        - (QM31Impl::from_partial_evals(
            [
                trace_2_column_22_offset_0, trace_2_column_23_offset_0, trace_2_column_24_offset_0,
                trace_2_column_25_offset_0,
            ],
        )))
        * (intermediate4)
        - (qm31(1, 0, 0, 0));
    // TODO: Batch `domain_vanish_at_point_inv` multiplication.
    sum = sum * random_coeff + constraint_0 * domain_vanish_at_point_inv;
    // TODO: Batch `domain_vanish_at_point_inv` multiplication.
    sum = sum * random_coeff + constraint_1 * domain_vanish_at_point_inv;
    // TODO: Batch `domain_vanish_at_point_inv` multiplication.
    sum = sum * random_coeff + constraint_2 * domain_vanish_at_point_inv;
    // TODO: Batch `domain_vanish_at_point_inv` multiplication.
    sum = sum * random_coeff + constraint_3 * domain_vanish_at_point_inv;
    // TODO: Batch `domain_vanish_at_point_inv` multiplication.
    sum = sum * random_coeff + constraint_4 * domain_vanish_at_point_inv;
}

