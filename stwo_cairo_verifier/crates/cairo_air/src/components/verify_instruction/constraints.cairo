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
    pub MemoryIdToBig_alpha0: QM31,
    pub MemoryIdToBig_alpha1: QM31,
    pub MemoryIdToBig_alpha2: QM31,
    pub MemoryIdToBig_alpha3: QM31,
    pub MemoryIdToBig_alpha4: QM31,
    pub MemoryIdToBig_alpha5: QM31,
    pub MemoryIdToBig_alpha6: QM31,
    pub MemoryIdToBig_alpha7: QM31,
    pub MemoryIdToBig_z: QM31,
    pub RangeCheck_4_3_alpha0: QM31,
    pub RangeCheck_4_3_alpha1: QM31,
    pub RangeCheck_4_3_z: QM31,
    pub RangeCheck_7_2_5_alpha0: QM31,
    pub RangeCheck_7_2_5_alpha1: QM31,
    pub RangeCheck_7_2_5_alpha2: QM31,
    pub RangeCheck_7_2_5_z: QM31,
    pub VerifyInstruction_alpha0: QM31,
    pub VerifyInstruction_alpha1: QM31,
    pub VerifyInstruction_alpha10: QM31,
    pub VerifyInstruction_alpha11: QM31,
    pub VerifyInstruction_alpha12: QM31,
    pub VerifyInstruction_alpha13: QM31,
    pub VerifyInstruction_alpha14: QM31,
    pub VerifyInstruction_alpha15: QM31,
    pub VerifyInstruction_alpha16: QM31,
    pub VerifyInstruction_alpha17: QM31,
    pub VerifyInstruction_alpha18: QM31,
    pub VerifyInstruction_alpha2: QM31,
    pub VerifyInstruction_alpha3: QM31,
    pub VerifyInstruction_alpha4: QM31,
    pub VerifyInstruction_alpha5: QM31,
    pub VerifyInstruction_alpha6: QM31,
    pub VerifyInstruction_alpha7: QM31,
    pub VerifyInstruction_alpha8: QM31,
    pub VerifyInstruction_alpha9: QM31,
    pub VerifyInstruction_z: QM31,
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
        MemoryAddressToId_alpha0,
        MemoryAddressToId_alpha1,
        MemoryAddressToId_z,
        MemoryIdToBig_alpha0,
        MemoryIdToBig_alpha1,
        MemoryIdToBig_alpha2,
        MemoryIdToBig_alpha3,
        MemoryIdToBig_alpha4,
        MemoryIdToBig_alpha5,
        MemoryIdToBig_alpha6,
        MemoryIdToBig_alpha7,
        MemoryIdToBig_z,
        RangeCheck_4_3_alpha0,
        RangeCheck_4_3_alpha1,
        RangeCheck_4_3_z,
        RangeCheck_7_2_5_alpha0,
        RangeCheck_7_2_5_alpha1,
        RangeCheck_7_2_5_alpha2,
        RangeCheck_7_2_5_z,
        VerifyInstruction_alpha0,
        VerifyInstruction_alpha1,
        VerifyInstruction_alpha10,
        VerifyInstruction_alpha11,
        VerifyInstruction_alpha12,
        VerifyInstruction_alpha13,
        VerifyInstruction_alpha14,
        VerifyInstruction_alpha15,
        VerifyInstruction_alpha16,
        VerifyInstruction_alpha17,
        VerifyInstruction_alpha18,
        VerifyInstruction_alpha2,
        VerifyInstruction_alpha3,
        VerifyInstruction_alpha4,
        VerifyInstruction_alpha5,
        VerifyInstruction_alpha6,
        VerifyInstruction_alpha7,
        VerifyInstruction_alpha8,
        VerifyInstruction_alpha9,
        VerifyInstruction_z,
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
    let mut trace_1_column_16 = trace_mask_values.pop_front().unwrap().span();
    let trace_1_column_16_offset_0 = *trace_1_column_16.pop_front().unwrap();
    let mut trace_1_column_17 = trace_mask_values.pop_front().unwrap().span();
    let trace_1_column_17_offset_0 = *trace_1_column_17.pop_front().unwrap();
    let mut trace_1_column_18 = trace_mask_values.pop_front().unwrap().span();
    let trace_1_column_18_offset_0 = *trace_1_column_18.pop_front().unwrap();
    let mut trace_1_column_19 = trace_mask_values.pop_front().unwrap().span();
    let trace_1_column_19_offset_0 = *trace_1_column_19.pop_front().unwrap();
    let mut trace_1_column_20 = trace_mask_values.pop_front().unwrap().span();
    let trace_1_column_20_offset_0 = *trace_1_column_20.pop_front().unwrap();
    let mut trace_1_column_21 = trace_mask_values.pop_front().unwrap().span();
    let trace_1_column_21_offset_0 = *trace_1_column_21.pop_front().unwrap();
    let mut trace_1_column_22 = trace_mask_values.pop_front().unwrap().span();
    let trace_1_column_22_offset_0 = *trace_1_column_22.pop_front().unwrap();
    let mut trace_1_column_23 = trace_mask_values.pop_front().unwrap().span();
    let trace_1_column_23_offset_0 = *trace_1_column_23.pop_front().unwrap();
    let mut trace_1_column_24 = trace_mask_values.pop_front().unwrap().span();
    let trace_1_column_24_offset_0 = *trace_1_column_24.pop_front().unwrap();
    let mut trace_1_column_25 = trace_mask_values.pop_front().unwrap().span();
    let trace_1_column_25_offset_0 = *trace_1_column_25.pop_front().unwrap();
    let mut trace_1_column_26 = trace_mask_values.pop_front().unwrap().span();
    let trace_1_column_26_offset_0 = *trace_1_column_26.pop_front().unwrap();
    let mut trace_1_column_27 = trace_mask_values.pop_front().unwrap().span();
    let trace_1_column_27_offset_0 = *trace_1_column_27.pop_front().unwrap();
    let mut trace_1_column_28 = trace_mask_values.pop_front().unwrap().span();
    let trace_1_column_28_offset_0 = *trace_1_column_28.pop_front().unwrap();
    let mut trace_2_column_29 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_29_offset_0 = *trace_2_column_29.pop_front().unwrap();
    let mut trace_2_column_30 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_30_offset_0 = *trace_2_column_30.pop_front().unwrap();
    let mut trace_2_column_31 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_31_offset_0 = *trace_2_column_31.pop_front().unwrap();
    let mut trace_2_column_32 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_32_offset_0 = *trace_2_column_32.pop_front().unwrap();
    let mut trace_2_column_33 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_33_offset_0 = *trace_2_column_33.pop_front().unwrap();
    let mut trace_2_column_34 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_34_offset_0 = *trace_2_column_34.pop_front().unwrap();
    let mut trace_2_column_35 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_35_offset_0 = *trace_2_column_35.pop_front().unwrap();
    let mut trace_2_column_36 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_36_offset_0 = *trace_2_column_36.pop_front().unwrap();
    let mut trace_2_column_37 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_37_offset_neg_1 = *trace_2_column_37.pop_front().unwrap();
    let trace_2_column_37_offset_0 = *trace_2_column_37.pop_front().unwrap();
    let mut trace_2_column_38 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_38_offset_neg_1 = *trace_2_column_38.pop_front().unwrap();
    let trace_2_column_38_offset_0 = *trace_2_column_38.pop_front().unwrap();
    let mut trace_2_column_39 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_39_offset_neg_1 = *trace_2_column_39.pop_front().unwrap();
    let trace_2_column_39_offset_0 = *trace_2_column_39.pop_front().unwrap();
    let mut trace_2_column_40 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_40_offset_neg_1 = *trace_2_column_40.pop_front().unwrap();
    let trace_2_column_40_offset_0 = *trace_2_column_40.pop_front().unwrap();
    let intermediate0 = (RangeCheck_7_2_5_alpha0) * (trace_1_column_20_offset_0)
        + (RangeCheck_7_2_5_alpha1) * (trace_1_column_21_offset_0)
        + (RangeCheck_7_2_5_alpha2) * (trace_1_column_23_offset_0)
        - (RangeCheck_7_2_5_z);

    let intermediate1 = (RangeCheck_4_3_alpha0) * (trace_1_column_24_offset_0)
        + (RangeCheck_4_3_alpha1) * (trace_1_column_26_offset_0)
        - (RangeCheck_4_3_z);

    let intermediate2 = (MemoryAddressToId_alpha0) * (trace_1_column_0_offset_0)
        + (MemoryAddressToId_alpha1) * (trace_1_column_27_offset_0)
        - (MemoryAddressToId_z);

    let intermediate3 = (MemoryIdToBig_alpha0) * (trace_1_column_27_offset_0)
        + (MemoryIdToBig_alpha1) * (trace_1_column_19_offset_0)
        + (MemoryIdToBig_alpha2)
            * (trace_1_column_20_offset_0 + (trace_1_column_21_offset_0) * (m31(128).into()))
        + (MemoryIdToBig_alpha3) * (trace_1_column_22_offset_0)
        + (MemoryIdToBig_alpha4)
            * (trace_1_column_23_offset_0 + (trace_1_column_24_offset_0) * (m31(32).into()))
        + (MemoryIdToBig_alpha5) * (trace_1_column_25_offset_0)
        + (MemoryIdToBig_alpha6)
            * (trace_1_column_26_offset_0
                + (trace_1_column_4_offset_0) * (m31(8).into())
                + (trace_1_column_5_offset_0) * (m31(16).into())
                + (trace_1_column_6_offset_0) * (m31(32).into())
                + (trace_1_column_7_offset_0) * (m31(64).into())
                + (trace_1_column_8_offset_0) * (m31(128).into())
                + (trace_1_column_9_offset_0) * (m31(256).into()))
        + (MemoryIdToBig_alpha7)
            * (trace_1_column_10_offset_0
                + (trace_1_column_11_offset_0) * (m31(2).into())
                + (trace_1_column_12_offset_0) * (m31(4).into())
                + (trace_1_column_13_offset_0) * (m31(8).into())
                + (trace_1_column_14_offset_0) * (m31(16).into())
                + (trace_1_column_15_offset_0) * (m31(32).into())
                + (trace_1_column_16_offset_0) * (m31(64).into())
                + (trace_1_column_17_offset_0) * (m31(128).into())
                + (trace_1_column_18_offset_0) * (m31(256).into()))
        - (MemoryIdToBig_z);

    let intermediate4 = (VerifyInstruction_alpha0) * (trace_1_column_0_offset_0)
        + (VerifyInstruction_alpha1) * (trace_1_column_1_offset_0)
        + (VerifyInstruction_alpha2) * (trace_1_column_2_offset_0)
        + (VerifyInstruction_alpha3) * (trace_1_column_3_offset_0)
        + (VerifyInstruction_alpha4) * (trace_1_column_4_offset_0)
        + (VerifyInstruction_alpha5) * (trace_1_column_5_offset_0)
        + (VerifyInstruction_alpha6) * (trace_1_column_6_offset_0)
        + (VerifyInstruction_alpha7) * (trace_1_column_7_offset_0)
        + (VerifyInstruction_alpha8) * (trace_1_column_8_offset_0)
        + (VerifyInstruction_alpha9) * (trace_1_column_9_offset_0)
        + (VerifyInstruction_alpha10) * (trace_1_column_10_offset_0)
        + (VerifyInstruction_alpha11) * (trace_1_column_11_offset_0)
        + (VerifyInstruction_alpha12) * (trace_1_column_12_offset_0)
        + (VerifyInstruction_alpha13) * (trace_1_column_13_offset_0)
        + (VerifyInstruction_alpha14) * (trace_1_column_14_offset_0)
        + (VerifyInstruction_alpha15) * (trace_1_column_15_offset_0)
        + (VerifyInstruction_alpha16) * (trace_1_column_16_offset_0)
        + (VerifyInstruction_alpha17) * (trace_1_column_17_offset_0)
        + (VerifyInstruction_alpha18) * (trace_1_column_18_offset_0)
        - (VerifyInstruction_z);

    let constraint_0 = trace_1_column_19_offset_0
        + (trace_1_column_20_offset_0) * (m31(512).into())
        - (trace_1_column_1_offset_0);

    let constraint_1 = trace_1_column_21_offset_0
        + (trace_1_column_22_offset_0) * (m31(4).into())
        + (trace_1_column_23_offset_0) * (m31(2048).into())
        - (trace_1_column_2_offset_0);

    let constraint_2 = trace_1_column_24_offset_0
        + (trace_1_column_25_offset_0) * (m31(16).into())
        + (trace_1_column_26_offset_0) * (m31(8192).into())
        - (trace_1_column_3_offset_0);

    let constraint_3 = (trace_1_column_4_offset_0) * (m31(1).into() - (trace_1_column_4_offset_0));

    let constraint_4 = (trace_1_column_5_offset_0) * (m31(1).into() - (trace_1_column_5_offset_0));

    let constraint_5 = (trace_1_column_6_offset_0) * (m31(1).into() - (trace_1_column_6_offset_0));

    let constraint_6 = (trace_1_column_7_offset_0) * (m31(1).into() - (trace_1_column_7_offset_0));

    let constraint_7 = (trace_1_column_8_offset_0) * (m31(1).into() - (trace_1_column_8_offset_0));

    let constraint_8 = (trace_1_column_9_offset_0) * (m31(1).into() - (trace_1_column_9_offset_0));

    let constraint_9 = (trace_1_column_10_offset_0)
        * (m31(1).into() - (trace_1_column_10_offset_0));

    let constraint_10 = (trace_1_column_11_offset_0)
        * (m31(1).into() - (trace_1_column_11_offset_0));

    let constraint_11 = (trace_1_column_12_offset_0)
        * (m31(1).into() - (trace_1_column_12_offset_0));

    let constraint_12 = (trace_1_column_13_offset_0)
        * (m31(1).into() - (trace_1_column_13_offset_0));

    let constraint_13 = (trace_1_column_14_offset_0)
        * (m31(1).into() - (trace_1_column_14_offset_0));

    let constraint_14 = (trace_1_column_15_offset_0)
        * (m31(1).into() - (trace_1_column_15_offset_0));

    let constraint_15 = (trace_1_column_16_offset_0)
        * (m31(1).into() - (trace_1_column_16_offset_0));

    let constraint_16 = (trace_1_column_17_offset_0)
        * (m31(1).into() - (trace_1_column_17_offset_0));

    let constraint_17 = (trace_1_column_18_offset_0)
        * (m31(1).into() - (trace_1_column_18_offset_0));

    let constraint_18 = (QM31Impl::from_partial_evals(
        [
            trace_2_column_29_offset_0, trace_2_column_30_offset_0, trace_2_column_31_offset_0,
            trace_2_column_32_offset_0,
        ],
    ))
        * ((intermediate0) * (intermediate1))
        - (intermediate1 + intermediate0);

    let constraint_19 = (QM31Impl::from_partial_evals(
        [
            trace_2_column_33_offset_0, trace_2_column_34_offset_0, trace_2_column_35_offset_0,
            trace_2_column_36_offset_0,
        ],
    )
        - (QM31Impl::from_partial_evals(
            [
                trace_2_column_29_offset_0, trace_2_column_30_offset_0, trace_2_column_31_offset_0,
                trace_2_column_32_offset_0,
            ],
        )))
        * ((intermediate2) * (intermediate3))
        - (intermediate3 + intermediate2);

    let constraint_20 = (QM31Impl::from_partial_evals(
        [
            trace_2_column_37_offset_0, trace_2_column_38_offset_0, trace_2_column_39_offset_0,
            trace_2_column_40_offset_0,
        ],
    )
        - (QM31Impl::from_partial_evals(
            [
                trace_2_column_37_offset_neg_1, trace_2_column_38_offset_neg_1,
                trace_2_column_39_offset_neg_1, trace_2_column_40_offset_neg_1,
            ],
        )
            - ((total_sum) * (preprocessed_is_first)))
        - (QM31Impl::from_partial_evals(
            [
                trace_2_column_33_offset_0, trace_2_column_34_offset_0, trace_2_column_35_offset_0,
                trace_2_column_36_offset_0,
            ],
        )))
        * (intermediate4)
        - (-(trace_1_column_28_offset_0));
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
    // TODO: Batch `domain_vanish_at_point_inv` multiplication.
    sum = sum * random_coeff + constraint_5 * domain_vanish_at_point_inv;
    // TODO: Batch `domain_vanish_at_point_inv` multiplication.
    sum = sum * random_coeff + constraint_6 * domain_vanish_at_point_inv;
    // TODO: Batch `domain_vanish_at_point_inv` multiplication.
    sum = sum * random_coeff + constraint_7 * domain_vanish_at_point_inv;
    // TODO: Batch `domain_vanish_at_point_inv` multiplication.
    sum = sum * random_coeff + constraint_8 * domain_vanish_at_point_inv;
    // TODO: Batch `domain_vanish_at_point_inv` multiplication.
    sum = sum * random_coeff + constraint_9 * domain_vanish_at_point_inv;
    // TODO: Batch `domain_vanish_at_point_inv` multiplication.
    sum = sum * random_coeff + constraint_10 * domain_vanish_at_point_inv;
    // TODO: Batch `domain_vanish_at_point_inv` multiplication.
    sum = sum * random_coeff + constraint_11 * domain_vanish_at_point_inv;
    // TODO: Batch `domain_vanish_at_point_inv` multiplication.
    sum = sum * random_coeff + constraint_12 * domain_vanish_at_point_inv;
    // TODO: Batch `domain_vanish_at_point_inv` multiplication.
    sum = sum * random_coeff + constraint_13 * domain_vanish_at_point_inv;
    // TODO: Batch `domain_vanish_at_point_inv` multiplication.
    sum = sum * random_coeff + constraint_14 * domain_vanish_at_point_inv;
    // TODO: Batch `domain_vanish_at_point_inv` multiplication.
    sum = sum * random_coeff + constraint_15 * domain_vanish_at_point_inv;
    // TODO: Batch `domain_vanish_at_point_inv` multiplication.
    sum = sum * random_coeff + constraint_16 * domain_vanish_at_point_inv;
    // TODO: Batch `domain_vanish_at_point_inv` multiplication.
    sum = sum * random_coeff + constraint_17 * domain_vanish_at_point_inv;
    // TODO: Batch `domain_vanish_at_point_inv` multiplication.
    sum = sum * random_coeff + constraint_18 * domain_vanish_at_point_inv;
    // TODO: Batch `domain_vanish_at_point_inv` multiplication.
    sum = sum * random_coeff + constraint_19 * domain_vanish_at_point_inv;
    // TODO: Batch `domain_vanish_at_point_inv` multiplication.
    sum = sum * random_coeff + constraint_20 * domain_vanish_at_point_inv;
}

