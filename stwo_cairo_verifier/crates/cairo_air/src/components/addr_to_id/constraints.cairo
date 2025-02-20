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
    preprocessed_column_set.insert(PreprocessedColumn::IsFirst(log_size));
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
    pub preprocessed_is_first: QM31,
    pub preprocessed_seq: QM31,
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
        MemoryAddressToId_alpha0,
        MemoryAddressToId_alpha1,
        MemoryAddressToId_z,
        preprocessed_is_first,
        preprocessed_seq,
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
    let mut trace_1_column_10 = trace_mask_values.pop_front().unwrap().span();
    let trace_1_column_10_offset_0 = *trace_1_column_10.pop_front().unwrap();
    let mut trace_1_column_11 = trace_mask_values.pop_front().unwrap().span();
    let trace_1_column_11_offset_0 = *trace_1_column_11.pop_front().unwrap();
    let mut trace_1_column_12 = trace_mask_values.pop_front().unwrap().span();
    let trace_1_column_12_offset_0 = *trace_1_column_12.pop_front().unwrap();
    let mut trace_1_column_13 = trace_mask_values.pop_front().unwrap().span();
    let trace_1_column_13_offset_0 = *trace_1_column_13.pop_front().unwrap();
    let mut trace_1_column_14 = trace_mask_values.pop_front().unwrap().span();
    let trace_1_column_14_offset_0 = *trace_1_column_14.pop_front().unwrap();
    let mut trace_1_column_15 = trace_mask_values.pop_front().unwrap().span();
    let trace_1_column_15_offset_0 = *trace_1_column_15.pop_front().unwrap();
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
    let trace_2_column_26_offset_0 = *trace_2_column_26.pop_front().unwrap();
    let mut trace_2_column_27 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_27_offset_0 = *trace_2_column_27.pop_front().unwrap();
    let mut trace_2_column_28 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_28_offset_neg_1 = *trace_2_column_28.pop_front().unwrap();
    let trace_2_column_28_offset_0 = *trace_2_column_28.pop_front().unwrap();
    let mut trace_2_column_29 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_29_offset_neg_1 = *trace_2_column_29.pop_front().unwrap();
    let trace_2_column_29_offset_0 = *trace_2_column_29.pop_front().unwrap();
    let mut trace_2_column_30 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_30_offset_neg_1 = *trace_2_column_30.pop_front().unwrap();
    let trace_2_column_30_offset_0 = *trace_2_column_30.pop_front().unwrap();
    let mut trace_2_column_31 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_31_offset_neg_1 = *trace_2_column_31.pop_front().unwrap();
    let trace_2_column_31_offset_0 = *trace_2_column_31.pop_front().unwrap();
    let intermediate0 = (MemoryAddressToId_alpha0) * (preprocessed_seq)
        + (MemoryAddressToId_alpha1) * (trace_1_column_0_offset_0)
        - (MemoryAddressToId_z);

    let intermediate1 = (MemoryAddressToId_alpha0) * (preprocessed_seq + m31(16).into())
        + (MemoryAddressToId_alpha1) * (trace_1_column_2_offset_0)
        - (MemoryAddressToId_z);

    let intermediate2 = (MemoryAddressToId_alpha0) * (preprocessed_seq + m31(32).into())
        + (MemoryAddressToId_alpha1) * (trace_1_column_4_offset_0)
        - (MemoryAddressToId_z);

    let intermediate3 = (MemoryAddressToId_alpha0) * (preprocessed_seq + m31(48).into())
        + (MemoryAddressToId_alpha1) * (trace_1_column_6_offset_0)
        - (MemoryAddressToId_z);

    let intermediate4 = (MemoryAddressToId_alpha0) * (preprocessed_seq + m31(64).into())
        + (MemoryAddressToId_alpha1) * (trace_1_column_8_offset_0)
        - (MemoryAddressToId_z);

    let intermediate5 = (MemoryAddressToId_alpha0) * (preprocessed_seq + m31(80).into())
        + (MemoryAddressToId_alpha1) * (trace_1_column_10_offset_0)
        - (MemoryAddressToId_z);

    let intermediate6 = (MemoryAddressToId_alpha0) * (preprocessed_seq + m31(96).into())
        + (MemoryAddressToId_alpha1) * (trace_1_column_12_offset_0)
        - (MemoryAddressToId_z);

    let intermediate7 = (MemoryAddressToId_alpha0) * (preprocessed_seq + m31(112).into())
        + (MemoryAddressToId_alpha1) * (trace_1_column_14_offset_0)
        - (MemoryAddressToId_z);

    let constraint_0 = (QM31Impl::from_partial_evals(
        [
            trace_2_column_16_offset_0, trace_2_column_17_offset_0, trace_2_column_18_offset_0,
            trace_2_column_19_offset_0,
        ],
    ))
        * ((intermediate0) * (intermediate1))
        - ((intermediate1) * (-(trace_1_column_1_offset_0))
            + (intermediate0) * (-(trace_1_column_3_offset_0)));

    let constraint_1 = (QM31Impl::from_partial_evals(
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
            + (intermediate2) * (-(trace_1_column_7_offset_0)));

    let constraint_2 = (QM31Impl::from_partial_evals(
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
            + (intermediate4) * (-(trace_1_column_11_offset_0)));

    let constraint_3 = (QM31Impl::from_partial_evals(
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
        )
            - ((total_sum) * (preprocessed_is_first)))
        - (QM31Impl::from_partial_evals(
            [
                trace_2_column_24_offset_0, trace_2_column_25_offset_0, trace_2_column_26_offset_0,
                trace_2_column_27_offset_0,
            ],
        )))
        * ((intermediate6) * (intermediate7))
        - ((intermediate7) * (-(trace_1_column_13_offset_0))
            + (intermediate6) * (-(trace_1_column_15_offset_0)));
    // TODO: Batch `domain_vanish_at_point_inv` multiplication.
    sum = sum * random_coeff + constraint_0 * domain_vanish_at_point_inv;
    // TODO: Batch `domain_vanish_at_point_inv` multiplication.
    sum = sum * random_coeff + constraint_1 * domain_vanish_at_point_inv;
    // TODO: Batch `domain_vanish_at_point_inv` multiplication.
    sum = sum * random_coeff + constraint_2 * domain_vanish_at_point_inv;
    // TODO: Batch `domain_vanish_at_point_inv` multiplication.
    sum = sum * random_coeff + constraint_3 * domain_vanish_at_point_inv;
}
