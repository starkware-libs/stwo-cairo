use stwo_verifier_core::{ColumnSpan, ColumnArray};
use stwo_verifier_core::circle::{
    CirclePoint, CirclePointIndex, CirclePointIndexImpl, CirclePointQM31AddCirclePointM31Impl,
};
use stwo_verifier_core::fields::m31::{m31, M31};
use stwo_verifier_core::fields::qm31::{QM31, QM31Impl, qm31};


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
    let ConstraintParams { MemoryAddressToId_alpha0,
    MemoryAddressToId_alpha1,
    MemoryAddressToId_z,
    preprocessed_is_first,
    total_sum } =
        params;

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
    let mut trace_2_column_9 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_9_offset_0 = *trace_2_column_9.pop_front().unwrap();
    let mut trace_2_column_10 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_10_offset_0 = *trace_2_column_10.pop_front().unwrap();
    let mut trace_2_column_11 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_11_offset_0 = *trace_2_column_11.pop_front().unwrap();
    let mut trace_2_column_12 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_12_offset_0 = *trace_2_column_12.pop_front().unwrap();
    let mut trace_2_column_13 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_13_offset_neg_1 = *trace_2_column_13.pop_front().unwrap();
    let trace_2_column_13_offset_0 = *trace_2_column_13.pop_front().unwrap();
    let mut trace_2_column_14 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_14_offset_neg_1 = *trace_2_column_14.pop_front().unwrap();
    let trace_2_column_14_offset_0 = *trace_2_column_14.pop_front().unwrap();
    let mut trace_2_column_15 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_15_offset_neg_1 = *trace_2_column_15.pop_front().unwrap();
    let trace_2_column_15_offset_0 = *trace_2_column_15.pop_front().unwrap();
    let mut trace_2_column_16 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_16_offset_neg_1 = *trace_2_column_16.pop_front().unwrap();
    let trace_2_column_16_offset_0 = *trace_2_column_16.pop_front().unwrap();
    let intermediate0 = (MemoryAddressToId_alpha0) * (trace_1_column_0_offset_0)
        + (MemoryAddressToId_alpha1) * (trace_1_column_1_offset_0)
        - (MemoryAddressToId_z);

    let intermediate1 = (MemoryAddressToId_alpha0) * (trace_1_column_0_offset_0 + m31(16).into())
        + (MemoryAddressToId_alpha1) * (trace_1_column_3_offset_0)
        - (MemoryAddressToId_z);

    let intermediate2 = (MemoryAddressToId_alpha0) * (trace_1_column_0_offset_0 + m31(32).into())
        + (MemoryAddressToId_alpha1) * (trace_1_column_5_offset_0)
        - (MemoryAddressToId_z);

    let intermediate3 = (MemoryAddressToId_alpha0) * (trace_1_column_0_offset_0 + m31(48).into())
        + (MemoryAddressToId_alpha1) * (trace_1_column_7_offset_0)
        - (MemoryAddressToId_z);

    let constraint_0 = (QM31Impl::from_partial_evals(
        [
            trace_2_column_9_offset_0, trace_2_column_10_offset_0, trace_2_column_11_offset_0,
            trace_2_column_12_offset_0,
        ],
    ))
        * ((intermediate0) * (intermediate1))
        - ((intermediate1) * (-(trace_1_column_2_offset_0))
            + (intermediate0) * (-(trace_1_column_4_offset_0)));

    let constraint_1 = (QM31Impl::from_partial_evals(
        [
            trace_2_column_13_offset_0, trace_2_column_14_offset_0, trace_2_column_15_offset_0,
            trace_2_column_16_offset_0,
        ],
    )
        - (QM31Impl::from_partial_evals(
            [
                trace_2_column_13_offset_neg_1, trace_2_column_14_offset_neg_1,
                trace_2_column_15_offset_neg_1, trace_2_column_16_offset_neg_1,
            ],
        )
            - ((total_sum) * (preprocessed_is_first)))
        - (QM31Impl::from_partial_evals(
            [
                trace_2_column_9_offset_0, trace_2_column_10_offset_0, trace_2_column_11_offset_0,
                trace_2_column_12_offset_0,
            ],
        )))
        * ((intermediate2) * (intermediate3))
        - ((intermediate3) * (-(trace_1_column_6_offset_0))
            + (intermediate2) * (-(trace_1_column_8_offset_0)));
    // TODO: Batch `domain_vanish_at_point_inv` multiplication.
    sum = sum * random_coeff + constraint_0 * domain_vanish_at_point_inv;
    // TODO: Batch `domain_vanish_at_point_inv` multiplication.
    sum = sum * random_coeff + constraint_1 * domain_vanish_at_point_inv;
}
