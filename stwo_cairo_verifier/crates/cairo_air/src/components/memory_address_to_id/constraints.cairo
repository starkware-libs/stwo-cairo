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
    preprocessed_column_set.insert(PreprocessedColumn::Seq(log_size));
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
    pub MemoryAddressToId_alpha0: QM31,
    pub MemoryAddressToId_alpha1: QM31,
    pub MemoryAddressToId_z: QM31,
    pub claimed_sum: QM31,
    pub preprocessed_seq: QM31,
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
        MemoryAddressToId_alpha0,
        MemoryAddressToId_alpha1,
        MemoryAddressToId_z,
        claimed_sum,
        preprocessed_seq,
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

    let [trace_1_column_10_offset_0]: [QM31; 1] = (*trace_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_1_column_11_offset_0]: [QM31; 1] = (*trace_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_1_column_12_offset_0]: [QM31; 1] = (*trace_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_1_column_13_offset_0]: [QM31; 1] = (*trace_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_1_column_14_offset_0]: [QM31; 1] = (*trace_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_1_column_15_offset_0]: [QM31; 1] = (*trace_mask_values
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

    let [trace_2_column_26_offset_0]: [QM31; 1] = (*interaction_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_27_offset_0]: [QM31; 1] = (*interaction_mask_values
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

    let [trace_2_column_30_offset_neg_1, trace_2_column_30_offset_0]: [QM31; 2] =
        (*interaction_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_31_offset_neg_1, trace_2_column_31_offset_0]: [QM31; 2] =
        (*interaction_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    core::internal::revoke_ap_tracking();

    let mut intermediates = intermediates(
        MemoryAddressToId_alpha0,
        MemoryAddressToId_alpha1,
        MemoryAddressToId_z,
        preprocessed_seq,
        trace_1_column_0_offset_0,
        trace_1_column_10_offset_0,
        trace_1_column_12_offset_0,
        trace_1_column_14_offset_0,
        trace_1_column_2_offset_0,
        trace_1_column_4_offset_0,
        trace_1_column_6_offset_0,
        trace_1_column_8_offset_0,
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

    // Constrait 0
    let constraint_quotient = ((QM31Impl::from_partial_evals(
        [
            trace_2_column_16_offset_0, trace_2_column_17_offset_0, trace_2_column_18_offset_0,
            trace_2_column_19_offset_0,
        ],
    ))
        * ((intermediate0) * (intermediate1))
        - ((intermediate1) * (-(trace_1_column_1_offset_0))
            + (intermediate0) * (-(trace_1_column_3_offset_0))))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constrait 1
    let constraint_quotient = ((QM31Impl::from_partial_evals(
        [
            trace_2_column_20_offset_0, trace_2_column_21_offset_0, trace_2_column_22_offset_0,
            trace_2_column_23_offset_0,
        ],
    )
        - (QM31Impl::from_partial_evals(
            [
                trace_2_column_16_offset_0, trace_2_column_17_offset_0, trace_2_column_18_offset_0,
                trace_2_column_19_offset_0,
            ],
        )))
        * ((intermediate2) * (intermediate3))
        - ((intermediate3) * (-(trace_1_column_5_offset_0))
            + (intermediate2) * (-(trace_1_column_7_offset_0))))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constrait 2
    let constraint_quotient = ((QM31Impl::from_partial_evals(
        [
            trace_2_column_24_offset_0, trace_2_column_25_offset_0, trace_2_column_26_offset_0,
            trace_2_column_27_offset_0,
        ],
    )
        - (QM31Impl::from_partial_evals(
            [
                trace_2_column_20_offset_0, trace_2_column_21_offset_0, trace_2_column_22_offset_0,
                trace_2_column_23_offset_0,
            ],
        )))
        * ((intermediate4) * (intermediate5))
        - ((intermediate5) * (-(trace_1_column_9_offset_0))
            + (intermediate4) * (-(trace_1_column_11_offset_0))))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constrait 3
    let constraint_quotient = ((QM31Impl::from_partial_evals(
        [
            trace_2_column_28_offset_0, trace_2_column_29_offset_0, trace_2_column_30_offset_0,
            trace_2_column_31_offset_0,
        ],
    )
        - (QM31Impl::from_partial_evals(
            [
                trace_2_column_28_offset_neg_1, trace_2_column_29_offset_neg_1,
                trace_2_column_30_offset_neg_1, trace_2_column_31_offset_neg_1,
            ],
        ))
        - (QM31Impl::from_partial_evals(
            [
                trace_2_column_24_offset_0, trace_2_column_25_offset_0, trace_2_column_26_offset_0,
                trace_2_column_27_offset_0,
            ],
        ))
        + (claimed_sum) * (qm31(32768, 0, 0, 0)))
        * ((intermediate6) * (intermediate7))
        - ((intermediate7) * (-(trace_1_column_13_offset_0))
            + (intermediate6) * (-(trace_1_column_15_offset_0))))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;
}


fn intermediates(
    MemoryAddressToId_alpha0: QM31,
    MemoryAddressToId_alpha1: QM31,
    MemoryAddressToId_z: QM31,
    preprocessed_seq: QM31,
    trace_1_column_0_offset_0: QM31,
    trace_1_column_10_offset_0: QM31,
    trace_1_column_12_offset_0: QM31,
    trace_1_column_14_offset_0: QM31,
    trace_1_column_2_offset_0: QM31,
    trace_1_column_4_offset_0: QM31,
    trace_1_column_6_offset_0: QM31,
    trace_1_column_8_offset_0: QM31,
) -> Array<QM31> {
    let intermediate0 = intermediate0(
        MemoryAddressToId_alpha0,
        MemoryAddressToId_alpha1,
        MemoryAddressToId_z,
        preprocessed_seq,
        trace_1_column_0_offset_0,
    );

    let intermediate1 = intermediate1(
        MemoryAddressToId_alpha0,
        MemoryAddressToId_alpha1,
        MemoryAddressToId_z,
        preprocessed_seq,
        trace_1_column_2_offset_0,
    );

    let intermediate2 = intermediate2(
        MemoryAddressToId_alpha0,
        MemoryAddressToId_alpha1,
        MemoryAddressToId_z,
        preprocessed_seq,
        trace_1_column_4_offset_0,
    );

    let intermediate3 = intermediate3(
        MemoryAddressToId_alpha0,
        MemoryAddressToId_alpha1,
        MemoryAddressToId_z,
        preprocessed_seq,
        trace_1_column_6_offset_0,
    );

    let intermediate4 = intermediate4(
        MemoryAddressToId_alpha0,
        MemoryAddressToId_alpha1,
        MemoryAddressToId_z,
        preprocessed_seq,
        trace_1_column_8_offset_0,
    );

    let intermediate5 = intermediate5(
        MemoryAddressToId_alpha0,
        MemoryAddressToId_alpha1,
        MemoryAddressToId_z,
        preprocessed_seq,
        trace_1_column_10_offset_0,
    );

    let intermediate6 = intermediate6(
        MemoryAddressToId_alpha0,
        MemoryAddressToId_alpha1,
        MemoryAddressToId_z,
        preprocessed_seq,
        trace_1_column_12_offset_0,
    );

    let intermediate7 = intermediate7(
        MemoryAddressToId_alpha0,
        MemoryAddressToId_alpha1,
        MemoryAddressToId_z,
        preprocessed_seq,
        trace_1_column_14_offset_0,
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
    ]
}


pub fn intermediate0(
    MemoryAddressToId_alpha0: QM31,
    MemoryAddressToId_alpha1: QM31,
    MemoryAddressToId_z: QM31,
    preprocessed_seq: QM31,
    trace_1_column_0_offset_0: QM31,
) -> QM31 {
    (MemoryAddressToId_alpha0) * (preprocessed_seq)
        + (MemoryAddressToId_alpha1) * (trace_1_column_0_offset_0)
        - (MemoryAddressToId_z)
}

pub fn intermediate1(
    MemoryAddressToId_alpha0: QM31,
    MemoryAddressToId_alpha1: QM31,
    MemoryAddressToId_z: QM31,
    preprocessed_seq: QM31,
    trace_1_column_2_offset_0: QM31,
) -> QM31 {
    (MemoryAddressToId_alpha0) * (preprocessed_seq + m31(16).into())
        + (MemoryAddressToId_alpha1) * (trace_1_column_2_offset_0)
        - (MemoryAddressToId_z)
}

pub fn intermediate2(
    MemoryAddressToId_alpha0: QM31,
    MemoryAddressToId_alpha1: QM31,
    MemoryAddressToId_z: QM31,
    preprocessed_seq: QM31,
    trace_1_column_4_offset_0: QM31,
) -> QM31 {
    (MemoryAddressToId_alpha0) * (preprocessed_seq + m31(32).into())
        + (MemoryAddressToId_alpha1) * (trace_1_column_4_offset_0)
        - (MemoryAddressToId_z)
}

pub fn intermediate3(
    MemoryAddressToId_alpha0: QM31,
    MemoryAddressToId_alpha1: QM31,
    MemoryAddressToId_z: QM31,
    preprocessed_seq: QM31,
    trace_1_column_6_offset_0: QM31,
) -> QM31 {
    (MemoryAddressToId_alpha0) * (preprocessed_seq + m31(48).into())
        + (MemoryAddressToId_alpha1) * (trace_1_column_6_offset_0)
        - (MemoryAddressToId_z)
}

pub fn intermediate4(
    MemoryAddressToId_alpha0: QM31,
    MemoryAddressToId_alpha1: QM31,
    MemoryAddressToId_z: QM31,
    preprocessed_seq: QM31,
    trace_1_column_8_offset_0: QM31,
) -> QM31 {
    (MemoryAddressToId_alpha0) * (preprocessed_seq + m31(64).into())
        + (MemoryAddressToId_alpha1) * (trace_1_column_8_offset_0)
        - (MemoryAddressToId_z)
}

pub fn intermediate5(
    MemoryAddressToId_alpha0: QM31,
    MemoryAddressToId_alpha1: QM31,
    MemoryAddressToId_z: QM31,
    preprocessed_seq: QM31,
    trace_1_column_10_offset_0: QM31,
) -> QM31 {
    (MemoryAddressToId_alpha0) * (preprocessed_seq + m31(80).into())
        + (MemoryAddressToId_alpha1) * (trace_1_column_10_offset_0)
        - (MemoryAddressToId_z)
}

pub fn intermediate6(
    MemoryAddressToId_alpha0: QM31,
    MemoryAddressToId_alpha1: QM31,
    MemoryAddressToId_z: QM31,
    preprocessed_seq: QM31,
    trace_1_column_12_offset_0: QM31,
) -> QM31 {
    (MemoryAddressToId_alpha0) * (preprocessed_seq + m31(96).into())
        + (MemoryAddressToId_alpha1) * (trace_1_column_12_offset_0)
        - (MemoryAddressToId_z)
}

pub fn intermediate7(
    MemoryAddressToId_alpha0: QM31,
    MemoryAddressToId_alpha1: QM31,
    MemoryAddressToId_z: QM31,
    preprocessed_seq: QM31,
    trace_1_column_14_offset_0: QM31,
) -> QM31 {
    (MemoryAddressToId_alpha0) * (preprocessed_seq + m31(112).into())
        + (MemoryAddressToId_alpha1) * (trace_1_column_14_offset_0)
        - (MemoryAddressToId_z)
}

