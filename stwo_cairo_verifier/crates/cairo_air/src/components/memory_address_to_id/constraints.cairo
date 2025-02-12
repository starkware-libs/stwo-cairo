use stwo_constraint_framework::{
    PreprocessedColumn, PreprocessedColumnSet, PreprocessedColumnSetImpl,
};
use stwo_verifier_core::circle::{
    CirclePoint, CirclePointIndex, CirclePointIndexImpl, CirclePointQM31AddCirclePointM31Impl,
};
use stwo_verifier_core::fields::Invertible;
use stwo_verifier_core::fields::m31::{M31, m31};
use stwo_verifier_core::fields::qm31::{QM31, QM31Impl, qm31};
use stwo_verifier_core::utils::pow2;
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
    pub seq: QM31,
    pub log_size: u32,
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
        MemoryAddressToId_alpha0,
        MemoryAddressToId_alpha1,
        MemoryAddressToId_z,
        claimed_sum,
        seq,
        log_size,
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
    ]: [Span<QM31>; 16] =
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

    let [trace_2_column_28_offset_neg_1, trace_2_column_28_offset_0]: [QM31; 2] =
        (*trace_2_column_28
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_29_offset_neg_1, trace_2_column_29_offset_0]: [QM31; 2] =
        (*trace_2_column_29
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_30_offset_neg_1, trace_2_column_30_offset_0]: [QM31; 2] =
        (*trace_2_column_30
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_31_offset_neg_1, trace_2_column_31_offset_0]: [QM31; 2] =
        (*trace_2_column_31
        .try_into()
        .unwrap())
        .unbox();

    core::internal::revoke_ap_tracking();

    let mut intermediates = intermediates(
        MemoryAddressToId_alpha0,
        MemoryAddressToId_alpha1,
        MemoryAddressToId_z,
        seq,
        trace_1_column_0_offset_0,
        trace_1_column_10_offset_0,
        trace_1_column_12_offset_0,
        trace_1_column_14_offset_0,
        trace_1_column_2_offset_0,
        trace_1_column_4_offset_0,
        trace_1_column_6_offset_0,
        trace_1_column_8_offset_0,
        log_size,
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

    // Constraint 0
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

    // Constraint 1
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

    // Constraint 2
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

    // Constraint 3
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
        + (claimed_sum) * (m31(pow2(log_size)).inverse().into()))
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
    seq: QM31,
    trace_1_column_0_offset_0: QM31,
    trace_1_column_10_offset_0: QM31,
    trace_1_column_12_offset_0: QM31,
    trace_1_column_14_offset_0: QM31,
    trace_1_column_2_offset_0: QM31,
    trace_1_column_4_offset_0: QM31,
    trace_1_column_6_offset_0: QM31,
    trace_1_column_8_offset_0: QM31,
    log_size: u32,
) -> Array<QM31> {
    let intermediate0 = intermediate0(
        MemoryAddressToId_alpha0,
        MemoryAddressToId_alpha1,
        MemoryAddressToId_z,
        seq,
        trace_1_column_0_offset_0,
    );

    let intermediate1 = intermediate1(
        MemoryAddressToId_alpha0,
        MemoryAddressToId_alpha1,
        MemoryAddressToId_z,
        seq,
        trace_1_column_2_offset_0,
        log_size,
    );

    let intermediate2 = intermediate2(
        MemoryAddressToId_alpha0,
        MemoryAddressToId_alpha1,
        MemoryAddressToId_z,
        seq,
        trace_1_column_4_offset_0,
        log_size,
    );

    let intermediate3 = intermediate3(
        MemoryAddressToId_alpha0,
        MemoryAddressToId_alpha1,
        MemoryAddressToId_z,
        seq,
        trace_1_column_6_offset_0,
        log_size,
    );

    let intermediate4 = intermediate4(
        MemoryAddressToId_alpha0,
        MemoryAddressToId_alpha1,
        MemoryAddressToId_z,
        seq,
        trace_1_column_8_offset_0,
        log_size,
    );

    let intermediate5 = intermediate5(
        MemoryAddressToId_alpha0,
        MemoryAddressToId_alpha1,
        MemoryAddressToId_z,
        seq,
        trace_1_column_10_offset_0,
        log_size,
    );

    let intermediate6 = intermediate6(
        MemoryAddressToId_alpha0,
        MemoryAddressToId_alpha1,
        MemoryAddressToId_z,
        seq,
        trace_1_column_12_offset_0,
        log_size,
    );

    let intermediate7 = intermediate7(
        MemoryAddressToId_alpha0,
        MemoryAddressToId_alpha1,
        MemoryAddressToId_z,
        seq,
        trace_1_column_14_offset_0,
        log_size,
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
    seq: QM31,
    trace_1_column_0_offset_0: QM31,
) -> QM31 {
    (MemoryAddressToId_alpha0) * (seq + m31(1).into())
        + (MemoryAddressToId_alpha1) * (trace_1_column_0_offset_0)
        - (MemoryAddressToId_z)
}

pub fn intermediate1(
    MemoryAddressToId_alpha0: QM31,
    MemoryAddressToId_alpha1: QM31,
    MemoryAddressToId_z: QM31,
    seq: QM31,
    trace_1_column_2_offset_0: QM31,
    log_size: u32,
) -> QM31 {
    (MemoryAddressToId_alpha0) * (seq + m31(1).into() + m31(pow2(log_size)).into())
        + (MemoryAddressToId_alpha1) * (trace_1_column_2_offset_0)
        - (MemoryAddressToId_z)
}

pub fn intermediate2(
    MemoryAddressToId_alpha0: QM31,
    MemoryAddressToId_alpha1: QM31,
    MemoryAddressToId_z: QM31,
    seq: QM31,
    trace_1_column_4_offset_0: QM31,
    log_size: u32,
) -> QM31 {
    (MemoryAddressToId_alpha0) * (seq + m31(1).into() + m31(pow2(log_size) * 2).into())
        + (MemoryAddressToId_alpha1) * (trace_1_column_4_offset_0)
        - (MemoryAddressToId_z)
}

pub fn intermediate3(
    MemoryAddressToId_alpha0: QM31,
    MemoryAddressToId_alpha1: QM31,
    MemoryAddressToId_z: QM31,
    seq: QM31,
    trace_1_column_6_offset_0: QM31,
    log_size: u32,
) -> QM31 {
    (MemoryAddressToId_alpha0) * (seq + m31(1).into() + m31(pow2(log_size) * 3).into())
        + (MemoryAddressToId_alpha1) * (trace_1_column_6_offset_0)
        - (MemoryAddressToId_z)
}

pub fn intermediate4(
    MemoryAddressToId_alpha0: QM31,
    MemoryAddressToId_alpha1: QM31,
    MemoryAddressToId_z: QM31,
    seq: QM31,
    trace_1_column_8_offset_0: QM31,
    log_size: u32,
) -> QM31 {
    (MemoryAddressToId_alpha0) * (seq + m31(1).into() + m31(pow2(log_size) * 4).into())
        + (MemoryAddressToId_alpha1) * (trace_1_column_8_offset_0)
        - (MemoryAddressToId_z)
}

pub fn intermediate5(
    MemoryAddressToId_alpha0: QM31,
    MemoryAddressToId_alpha1: QM31,
    MemoryAddressToId_z: QM31,
    seq: QM31,
    trace_1_column_10_offset_0: QM31,
    log_size: u32,
) -> QM31 {
    (MemoryAddressToId_alpha0) * (seq + m31(1).into() + m31(pow2(log_size) * 5).into())
        + (MemoryAddressToId_alpha1) * (trace_1_column_10_offset_0)
        - (MemoryAddressToId_z)
}

pub fn intermediate6(
    MemoryAddressToId_alpha0: QM31,
    MemoryAddressToId_alpha1: QM31,
    MemoryAddressToId_z: QM31,
    seq: QM31,
    trace_1_column_12_offset_0: QM31,
    log_size: u32,
) -> QM31 {
    (MemoryAddressToId_alpha0) * (seq + m31(1).into() + m31(pow2(log_size) * 6).into())
        + (MemoryAddressToId_alpha1) * (trace_1_column_12_offset_0)
        - (MemoryAddressToId_z)
}

pub fn intermediate7(
    MemoryAddressToId_alpha0: QM31,
    MemoryAddressToId_alpha1: QM31,
    MemoryAddressToId_z: QM31,
    seq: QM31,
    trace_1_column_14_offset_0: QM31,
    log_size: u32,
) -> QM31 {
    (MemoryAddressToId_alpha0) * (seq + m31(1).into() + m31(pow2(log_size) * 7).into())
        + (MemoryAddressToId_alpha1) * (trace_1_column_14_offset_0)
        - (MemoryAddressToId_z)
}

