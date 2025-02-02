use stwo_constraint_framework::{
    PreprocessedColumn, PreprocessedColumnSet, PreprocessedColumnSetImpl,
};
use stwo_verifier_core::circle::{
    CirclePoint, CirclePointIndex, CirclePointIndexImpl, CirclePointQM31AddCirclePointM31Impl,
};
use stwo_verifier_core::fields::m31::{M31, m31};
use stwo_verifier_core::fields::qm31::{QM31, QM31Impl, qm31};
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
    pub MemoryIdToBig_alpha0: QM31,
    pub MemoryIdToBig_alpha1: QM31,
    pub MemoryIdToBig_alpha2: QM31,
    pub MemoryIdToBig_alpha3: QM31,
    pub MemoryIdToBig_alpha4: QM31,
    pub MemoryIdToBig_alpha5: QM31,
    pub MemoryIdToBig_alpha6: QM31,
    pub MemoryIdToBig_alpha7: QM31,
    pub MemoryIdToBig_alpha8: QM31,
    pub MemoryIdToBig_z: QM31,
    pub RangeCheck_9_9_alpha0: QM31,
    pub RangeCheck_9_9_alpha1: QM31,
    pub RangeCheck_9_9_z: QM31,
    pub claimed_sum: QM31,
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
        MemoryIdToBig_alpha0,
        MemoryIdToBig_alpha1,
        MemoryIdToBig_alpha2,
        MemoryIdToBig_alpha3,
        MemoryIdToBig_alpha4,
        MemoryIdToBig_alpha5,
        MemoryIdToBig_alpha6,
        MemoryIdToBig_alpha7,
        MemoryIdToBig_alpha8,
        MemoryIdToBig_z,
        RangeCheck_9_9_alpha0,
        RangeCheck_9_9_alpha1,
        RangeCheck_9_9_z,
        claimed_sum,
    } = params;

    let [trace_1_column_0_offset_0]: [QM31; 1] = (*trace_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_1_column_1_offset_0]: [QM31; 1] = (*trace_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_1_column_2_offset_0]: [QM31; 1] = (*trace_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_1_column_3_offset_0]: [QM31; 1] = (*trace_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_1_column_4_offset_0]: [QM31; 1] = (*trace_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_1_column_5_offset_0]: [QM31; 1] = (*trace_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_1_column_6_offset_0]: [QM31; 1] = (*trace_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_1_column_7_offset_0]: [QM31; 1] = (*trace_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_1_column_8_offset_0]: [QM31; 1] = (*trace_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_1_column_9_offset_0]: [QM31; 1] = (*trace_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_10_offset_0]: [QM31; 1] = (*interaction_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_11_offset_0]: [QM31; 1] = (*interaction_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_12_offset_0]: [QM31; 1] = (*interaction_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_13_offset_0]: [QM31; 1] = (*interaction_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_14_offset_0]: [QM31; 1] = (*interaction_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_15_offset_0]: [QM31; 1] = (*interaction_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_16_offset_0]: [QM31; 1] = (*interaction_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_17_offset_0]: [QM31; 1] = (*interaction_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_18_offset_0]: [QM31; 1] = (*interaction_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_19_offset_0]: [QM31; 1] = (*interaction_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_20_offset_0]: [QM31; 1] = (*interaction_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_21_offset_0]: [QM31; 1] = (*interaction_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_22_offset_0]: [QM31; 1] = (*interaction_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_23_offset_0]: [QM31; 1] = (*interaction_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_24_offset_0]: [QM31; 1] = (*interaction_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_25_offset_0]: [QM31; 1] = (*interaction_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_26_offset_neg_1, trace_2_column_26_offset_0]: [QM31; 2] =
        (*interaction_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_27_offset_neg_1, trace_2_column_27_offset_0]: [QM31; 2] =
        (*interaction_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_28_offset_neg_1, trace_2_column_28_offset_0]: [QM31; 2] =
        (*interaction_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_29_offset_neg_1, trace_2_column_29_offset_0]: [QM31; 2] =
        (*interaction_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    core::internal::revoke_ap_tracking();

    let mut intermediates = intermediates(
        MemoryIdToBig_alpha0,
        MemoryIdToBig_alpha1,
        MemoryIdToBig_alpha2,
        MemoryIdToBig_alpha3,
        MemoryIdToBig_alpha4,
        MemoryIdToBig_alpha5,
        MemoryIdToBig_alpha6,
        MemoryIdToBig_alpha7,
        MemoryIdToBig_alpha8,
        MemoryIdToBig_z,
        RangeCheck_9_9_alpha0,
        RangeCheck_9_9_alpha1,
        RangeCheck_9_9_z,
        trace_1_column_0_offset_0,
        trace_1_column_1_offset_0,
        trace_1_column_2_offset_0,
        trace_1_column_3_offset_0,
        trace_1_column_4_offset_0,
        trace_1_column_5_offset_0,
        trace_1_column_6_offset_0,
        trace_1_column_7_offset_0,
        trace_1_column_8_offset_0,
    )
        .span();
    let intermediate0 = *intermediates.pop_front().unwrap();
    let intermediate1 = *intermediates.pop_front().unwrap();
    let intermediate2 = *intermediates.pop_front().unwrap();
    let intermediate3 = *intermediates.pop_front().unwrap();
    let intermediate4 = *intermediates.pop_front().unwrap();

    // Constrait 0
    let constraint_quotient = ((QM31Impl::from_partial_evals(
        [
            trace_2_column_10_offset_0, trace_2_column_11_offset_0, trace_2_column_12_offset_0,
            trace_2_column_13_offset_0,
        ],
    ))
        * (intermediate0)
        - (qm31(1, 0, 0, 0)))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constrait 1
    let constraint_quotient = ((QM31Impl::from_partial_evals(
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
        - (qm31(1, 0, 0, 0)))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constrait 2
    let constraint_quotient = ((QM31Impl::from_partial_evals(
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
        - (qm31(1, 0, 0, 0)))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constrait 3
    let constraint_quotient = ((QM31Impl::from_partial_evals(
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
        - (qm31(1, 0, 0, 0)))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constrait 4
    let constraint_quotient = ((QM31Impl::from_partial_evals(
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
        ))
        - (QM31Impl::from_partial_evals(
            [
                trace_2_column_22_offset_0, trace_2_column_23_offset_0, trace_2_column_24_offset_0,
                trace_2_column_25_offset_0,
            ],
        ))
        + (claimed_sum) * (qm31(32768, 0, 0, 0)))
        * (intermediate4)
        - (-(trace_1_column_9_offset_0)))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;
}


fn intermediates(
    MemoryIdToBig_alpha0: QM31,
    MemoryIdToBig_alpha1: QM31,
    MemoryIdToBig_alpha2: QM31,
    MemoryIdToBig_alpha3: QM31,
    MemoryIdToBig_alpha4: QM31,
    MemoryIdToBig_alpha5: QM31,
    MemoryIdToBig_alpha6: QM31,
    MemoryIdToBig_alpha7: QM31,
    MemoryIdToBig_alpha8: QM31,
    MemoryIdToBig_z: QM31,
    RangeCheck_9_9_alpha0: QM31,
    RangeCheck_9_9_alpha1: QM31,
    RangeCheck_9_9_z: QM31,
    trace_1_column_0_offset_0: QM31,
    trace_1_column_1_offset_0: QM31,
    trace_1_column_2_offset_0: QM31,
    trace_1_column_3_offset_0: QM31,
    trace_1_column_4_offset_0: QM31,
    trace_1_column_5_offset_0: QM31,
    trace_1_column_6_offset_0: QM31,
    trace_1_column_7_offset_0: QM31,
    trace_1_column_8_offset_0: QM31,
) -> Array<QM31> {
    let intermediate0 = intermediate0(
        RangeCheck_9_9_alpha0,
        RangeCheck_9_9_alpha1,
        RangeCheck_9_9_z,
        trace_1_column_1_offset_0,
        trace_1_column_2_offset_0,
    );

    let intermediate1 = intermediate1(
        RangeCheck_9_9_alpha0,
        RangeCheck_9_9_alpha1,
        RangeCheck_9_9_z,
        trace_1_column_3_offset_0,
        trace_1_column_4_offset_0,
    );

    let intermediate2 = intermediate2(
        RangeCheck_9_9_alpha0,
        RangeCheck_9_9_alpha1,
        RangeCheck_9_9_z,
        trace_1_column_5_offset_0,
        trace_1_column_6_offset_0,
    );

    let intermediate3 = intermediate3(
        RangeCheck_9_9_alpha0,
        RangeCheck_9_9_alpha1,
        RangeCheck_9_9_z,
        trace_1_column_7_offset_0,
        trace_1_column_8_offset_0,
    );

    let intermediate4 = intermediate4(
        MemoryIdToBig_alpha0,
        MemoryIdToBig_alpha1,
        MemoryIdToBig_alpha2,
        MemoryIdToBig_alpha3,
        MemoryIdToBig_alpha4,
        MemoryIdToBig_alpha5,
        MemoryIdToBig_alpha6,
        MemoryIdToBig_alpha7,
        MemoryIdToBig_alpha8,
        MemoryIdToBig_z,
        trace_1_column_0_offset_0,
        trace_1_column_1_offset_0,
        trace_1_column_2_offset_0,
        trace_1_column_3_offset_0,
        trace_1_column_4_offset_0,
        trace_1_column_5_offset_0,
        trace_1_column_6_offset_0,
        trace_1_column_7_offset_0,
        trace_1_column_8_offset_0,
    );
    array![intermediate0, intermediate1, intermediate2, intermediate3, intermediate4]
}


pub fn intermediate0(
    RangeCheck_9_9_alpha0: QM31,
    RangeCheck_9_9_alpha1: QM31,
    RangeCheck_9_9_z: QM31,
    trace_1_column_1_offset_0: QM31,
    trace_1_column_2_offset_0: QM31,
) -> QM31 {
    (RangeCheck_9_9_alpha0) * (trace_1_column_1_offset_0)
        + (RangeCheck_9_9_alpha1) * (trace_1_column_2_offset_0)
        - (RangeCheck_9_9_z)
}

pub fn intermediate1(
    RangeCheck_9_9_alpha0: QM31,
    RangeCheck_9_9_alpha1: QM31,
    RangeCheck_9_9_z: QM31,
    trace_1_column_3_offset_0: QM31,
    trace_1_column_4_offset_0: QM31,
) -> QM31 {
    (RangeCheck_9_9_alpha0) * (trace_1_column_3_offset_0)
        + (RangeCheck_9_9_alpha1) * (trace_1_column_4_offset_0)
        - (RangeCheck_9_9_z)
}

pub fn intermediate2(
    RangeCheck_9_9_alpha0: QM31,
    RangeCheck_9_9_alpha1: QM31,
    RangeCheck_9_9_z: QM31,
    trace_1_column_5_offset_0: QM31,
    trace_1_column_6_offset_0: QM31,
) -> QM31 {
    (RangeCheck_9_9_alpha0) * (trace_1_column_5_offset_0)
        + (RangeCheck_9_9_alpha1) * (trace_1_column_6_offset_0)
        - (RangeCheck_9_9_z)
}

pub fn intermediate3(
    RangeCheck_9_9_alpha0: QM31,
    RangeCheck_9_9_alpha1: QM31,
    RangeCheck_9_9_z: QM31,
    trace_1_column_7_offset_0: QM31,
    trace_1_column_8_offset_0: QM31,
) -> QM31 {
    (RangeCheck_9_9_alpha0) * (trace_1_column_7_offset_0)
        + (RangeCheck_9_9_alpha1) * (trace_1_column_8_offset_0)
        - (RangeCheck_9_9_z)
}

pub fn intermediate4(
    MemoryIdToBig_alpha0: QM31,
    MemoryIdToBig_alpha1: QM31,
    MemoryIdToBig_alpha2: QM31,
    MemoryIdToBig_alpha3: QM31,
    MemoryIdToBig_alpha4: QM31,
    MemoryIdToBig_alpha5: QM31,
    MemoryIdToBig_alpha6: QM31,
    MemoryIdToBig_alpha7: QM31,
    MemoryIdToBig_alpha8: QM31,
    MemoryIdToBig_z: QM31,
    trace_1_column_0_offset_0: QM31,
    trace_1_column_1_offset_0: QM31,
    trace_1_column_2_offset_0: QM31,
    trace_1_column_3_offset_0: QM31,
    trace_1_column_4_offset_0: QM31,
    trace_1_column_5_offset_0: QM31,
    trace_1_column_6_offset_0: QM31,
    trace_1_column_7_offset_0: QM31,
    trace_1_column_8_offset_0: QM31,
) -> QM31 {
    (MemoryIdToBig_alpha0) * (trace_1_column_0_offset_0)
        + (MemoryIdToBig_alpha1) * (trace_1_column_1_offset_0)
        + (MemoryIdToBig_alpha2) * (trace_1_column_2_offset_0)
        + (MemoryIdToBig_alpha3) * (trace_1_column_3_offset_0)
        + (MemoryIdToBig_alpha4) * (trace_1_column_4_offset_0)
        + (MemoryIdToBig_alpha5) * (trace_1_column_5_offset_0)
        + (MemoryIdToBig_alpha6) * (trace_1_column_6_offset_0)
        + (MemoryIdToBig_alpha7) * (trace_1_column_7_offset_0)
        + (MemoryIdToBig_alpha8) * (trace_1_column_8_offset_0)
        - (MemoryIdToBig_z)
}

