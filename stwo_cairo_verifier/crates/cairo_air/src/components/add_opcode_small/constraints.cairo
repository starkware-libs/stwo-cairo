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
    claimed_sum_offset: usize,
) {
    let point_offset_neg_1 = point.add_circle_point_m31(-trace_gen.mul(1).to_point());
    let point_offset_claimed_sum = point
        .add_circle_point_m31(trace_gen.mul(claimed_sum_offset).to_point());
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
    interaction_trace_mask_points
        .append(array![point_offset_neg_1, point, point_offset_claimed_sum]);
    interaction_trace_mask_points
        .append(array![point_offset_neg_1, point, point_offset_claimed_sum]);
    interaction_trace_mask_points
        .append(array![point_offset_neg_1, point, point_offset_claimed_sum]);
    interaction_trace_mask_points
        .append(array![point_offset_neg_1, point, point_offset_claimed_sum]);
}

#[derive(Drop)]
pub struct ConstraintParams {
    pub MemoryAddressToId_alpha0: QM31,
    pub MemoryAddressToId_alpha1: QM31,
    pub MemoryAddressToId_z: QM31,
    pub MemoryIdToBig_alpha0: QM31,
    pub MemoryIdToBig_alpha1: QM31,
    pub MemoryIdToBig_alpha10: QM31,
    pub MemoryIdToBig_alpha11: QM31,
    pub MemoryIdToBig_alpha12: QM31,
    pub MemoryIdToBig_alpha13: QM31,
    pub MemoryIdToBig_alpha14: QM31,
    pub MemoryIdToBig_alpha15: QM31,
    pub MemoryIdToBig_alpha16: QM31,
    pub MemoryIdToBig_alpha17: QM31,
    pub MemoryIdToBig_alpha18: QM31,
    pub MemoryIdToBig_alpha19: QM31,
    pub MemoryIdToBig_alpha2: QM31,
    pub MemoryIdToBig_alpha20: QM31,
    pub MemoryIdToBig_alpha21: QM31,
    pub MemoryIdToBig_alpha22: QM31,
    pub MemoryIdToBig_alpha23: QM31,
    pub MemoryIdToBig_alpha24: QM31,
    pub MemoryIdToBig_alpha25: QM31,
    pub MemoryIdToBig_alpha26: QM31,
    pub MemoryIdToBig_alpha27: QM31,
    pub MemoryIdToBig_alpha28: QM31,
    pub MemoryIdToBig_alpha3: QM31,
    pub MemoryIdToBig_alpha4: QM31,
    pub MemoryIdToBig_alpha5: QM31,
    pub MemoryIdToBig_alpha6: QM31,
    pub MemoryIdToBig_alpha7: QM31,
    pub MemoryIdToBig_alpha8: QM31,
    pub MemoryIdToBig_alpha9: QM31,
    pub MemoryIdToBig_z: QM31,
    pub Opcodes_alpha0: QM31,
    pub Opcodes_alpha1: QM31,
    pub Opcodes_alpha2: QM31,
    pub Opcodes_z: QM31,
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
    pub claimed_sum: QM31,
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
        MemoryIdToBig_alpha10,
        MemoryIdToBig_alpha11,
        MemoryIdToBig_alpha12,
        MemoryIdToBig_alpha13,
        MemoryIdToBig_alpha14,
        MemoryIdToBig_alpha15,
        MemoryIdToBig_alpha16,
        MemoryIdToBig_alpha17,
        MemoryIdToBig_alpha18,
        MemoryIdToBig_alpha19,
        MemoryIdToBig_alpha2,
        MemoryIdToBig_alpha20,
        MemoryIdToBig_alpha21,
        MemoryIdToBig_alpha22,
        MemoryIdToBig_alpha23,
        MemoryIdToBig_alpha24,
        MemoryIdToBig_alpha25,
        MemoryIdToBig_alpha26,
        MemoryIdToBig_alpha27,
        MemoryIdToBig_alpha28,
        MemoryIdToBig_alpha3,
        MemoryIdToBig_alpha4,
        MemoryIdToBig_alpha5,
        MemoryIdToBig_alpha6,
        MemoryIdToBig_alpha7,
        MemoryIdToBig_alpha8,
        MemoryIdToBig_alpha9,
        MemoryIdToBig_z,
        Opcodes_alpha0,
        Opcodes_alpha1,
        Opcodes_alpha2,
        Opcodes_z,
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
        claimed_sum,
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
    let mut trace_1_column_29 = trace_mask_values.pop_front().unwrap().span();
    let trace_1_column_29_offset_0 = *trace_1_column_29.pop_front().unwrap();
    let mut trace_1_column_30 = trace_mask_values.pop_front().unwrap().span();
    let trace_1_column_30_offset_0 = *trace_1_column_30.pop_front().unwrap();
    let mut trace_1_column_31 = trace_mask_values.pop_front().unwrap().span();
    let trace_1_column_31_offset_0 = *trace_1_column_31.pop_front().unwrap();
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
    let trace_2_column_37_offset_0 = *trace_2_column_37.pop_front().unwrap();
    let mut trace_2_column_38 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_38_offset_0 = *trace_2_column_38.pop_front().unwrap();
    let mut trace_2_column_39 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_39_offset_0 = *trace_2_column_39.pop_front().unwrap();
    let mut trace_2_column_40 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_40_offset_0 = *trace_2_column_40.pop_front().unwrap();
    let mut trace_2_column_41 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_41_offset_0 = *trace_2_column_41.pop_front().unwrap();
    let mut trace_2_column_42 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_42_offset_0 = *trace_2_column_42.pop_front().unwrap();
    let mut trace_2_column_43 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_43_offset_0 = *trace_2_column_43.pop_front().unwrap();
    let mut trace_2_column_44 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_44_offset_0 = *trace_2_column_44.pop_front().unwrap();
    let mut trace_2_column_45 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_45_offset_0 = *trace_2_column_45.pop_front().unwrap();
    let mut trace_2_column_46 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_46_offset_0 = *trace_2_column_46.pop_front().unwrap();
    let mut trace_2_column_47 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_47_offset_0 = *trace_2_column_47.pop_front().unwrap();
    let mut trace_2_column_48 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_48_offset_neg_1 = *trace_2_column_48.pop_front().unwrap();
    let trace_2_column_48_offset_0 = *trace_2_column_48.pop_front().unwrap();
    let trace_2_column_48_offset_claimed_sum = *trace_2_column_48.pop_front().unwrap();
    let mut trace_2_column_49 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_49_offset_neg_1 = *trace_2_column_49.pop_front().unwrap();
    let trace_2_column_49_offset_0 = *trace_2_column_49.pop_front().unwrap();
    let trace_2_column_49_offset_claimed_sum = *trace_2_column_49.pop_front().unwrap();
    let mut trace_2_column_50 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_50_offset_neg_1 = *trace_2_column_50.pop_front().unwrap();
    let trace_2_column_50_offset_0 = *trace_2_column_50.pop_front().unwrap();
    let trace_2_column_50_offset_claimed_sum = *trace_2_column_50.pop_front().unwrap();
    let mut trace_2_column_51 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_51_offset_neg_1 = *trace_2_column_51.pop_front().unwrap();
    let trace_2_column_51_offset_0 = *trace_2_column_51.pop_front().unwrap();
    let trace_2_column_51_offset_claimed_sum = *trace_2_column_51.pop_front().unwrap();
    core::internal::revoke_ap_tracking();

    let mut intermediates = intermediates(
        MemoryAddressToId_alpha0,
        MemoryAddressToId_alpha1,
        MemoryAddressToId_z,
        MemoryIdToBig_alpha0,
        MemoryIdToBig_alpha1,
        MemoryIdToBig_alpha10,
        MemoryIdToBig_alpha11,
        MemoryIdToBig_alpha12,
        MemoryIdToBig_alpha13,
        MemoryIdToBig_alpha14,
        MemoryIdToBig_alpha15,
        MemoryIdToBig_alpha16,
        MemoryIdToBig_alpha17,
        MemoryIdToBig_alpha18,
        MemoryIdToBig_alpha19,
        MemoryIdToBig_alpha2,
        MemoryIdToBig_alpha20,
        MemoryIdToBig_alpha21,
        MemoryIdToBig_alpha22,
        MemoryIdToBig_alpha28,
        MemoryIdToBig_alpha3,
        MemoryIdToBig_alpha4,
        MemoryIdToBig_alpha5,
        MemoryIdToBig_alpha6,
        MemoryIdToBig_alpha7,
        MemoryIdToBig_alpha8,
        MemoryIdToBig_alpha9,
        MemoryIdToBig_z,
        Opcodes_alpha0,
        Opcodes_alpha1,
        Opcodes_alpha2,
        Opcodes_z,
        VerifyInstruction_alpha0,
        VerifyInstruction_alpha1,
        VerifyInstruction_alpha15,
        VerifyInstruction_alpha18,
        VerifyInstruction_alpha2,
        VerifyInstruction_alpha3,
        VerifyInstruction_alpha4,
        VerifyInstruction_alpha5,
        VerifyInstruction_alpha7,
        VerifyInstruction_alpha8,
        VerifyInstruction_alpha9,
        VerifyInstruction_z,
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
        trace_1_column_20_offset_0,
        trace_1_column_21_offset_0,
        trace_1_column_22_offset_0,
        trace_1_column_23_offset_0,
        trace_1_column_24_offset_0,
        trace_1_column_25_offset_0,
        trace_1_column_26_offset_0,
        trace_1_column_27_offset_0,
        trace_1_column_28_offset_0,
        trace_1_column_29_offset_0,
        trace_1_column_2_offset_0,
        trace_1_column_30_offset_0,
        trace_1_column_31_offset_0,
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

    let constraint_0 = trace_1_column_11_offset_0
        - ((trace_1_column_6_offset_0) * (trace_1_column_2_offset_0)
            + (m31(1).into() - (trace_1_column_6_offset_0)) * (trace_1_column_1_offset_0));

    let constraint_1 = trace_1_column_12_offset_0
        - ((trace_1_column_7_offset_0) * (trace_1_column_2_offset_0)
            + (m31(1).into() - (trace_1_column_7_offset_0)) * (trace_1_column_1_offset_0));

    let constraint_2 = trace_1_column_8_offset_0 + trace_1_column_9_offset_0 - (m31(1).into());

    let constraint_3 = trace_1_column_13_offset_0
        - ((trace_1_column_8_offset_0) * (trace_1_column_2_offset_0)
            + (trace_1_column_9_offset_0) * (trace_1_column_1_offset_0));

    let constraint_4 = (trace_1_column_15_offset_0)
        * (trace_1_column_15_offset_0 - (m31(1).into()));

    let constraint_5 = (trace_1_column_16_offset_0)
        * (trace_1_column_16_offset_0 - (m31(1).into()));

    let constraint_6 = (trace_1_column_16_offset_0)
        * (trace_1_column_15_offset_0 - (m31(1).into()));

    let constraint_7 = (trace_1_column_21_offset_0)
        * (trace_1_column_21_offset_0 - (m31(1).into()));

    let constraint_8 = (trace_1_column_22_offset_0)
        * (trace_1_column_22_offset_0 - (m31(1).into()));

    let constraint_9 = (trace_1_column_22_offset_0)
        * (trace_1_column_21_offset_0 - (m31(1).into()));

    let constraint_10 = (trace_1_column_27_offset_0)
        * (trace_1_column_27_offset_0 - (m31(1).into()));

    let constraint_11 = (trace_1_column_28_offset_0)
        * (trace_1_column_28_offset_0 - (m31(1).into()));

    let constraint_12 = (trace_1_column_28_offset_0)
        * (trace_1_column_27_offset_0 - (m31(1).into()));

    let constraint_13 = trace_1_column_17_offset_0
        + (trace_1_column_18_offset_0) * (m31(512).into())
        + (trace_1_column_19_offset_0) * (m31(262144).into())
        - (trace_1_column_15_offset_0)
        - ((m31(134217728).into()) * (trace_1_column_16_offset_0))
        - (trace_1_column_23_offset_0
            + (trace_1_column_24_offset_0) * (m31(512).into())
            + (trace_1_column_25_offset_0) * (m31(262144).into())
            - (trace_1_column_21_offset_0)
            - ((m31(134217728).into()) * (trace_1_column_22_offset_0))
            + trace_1_column_29_offset_0
            + (trace_1_column_30_offset_0) * (m31(512).into())
            + (trace_1_column_31_offset_0) * (m31(262144).into())
            - (trace_1_column_27_offset_0)
            - ((m31(134217728).into()) * (trace_1_column_28_offset_0)));

    let constraint_14 = (QM31Impl::from_partial_evals(
        [
            trace_2_column_32_offset_0, trace_2_column_33_offset_0, trace_2_column_34_offset_0,
            trace_2_column_35_offset_0,
        ],
    ))
        * ((intermediate0) * (intermediate1))
        - (intermediate1 + intermediate0);

    let constraint_15 = (QM31Impl::from_partial_evals(
        [
            trace_2_column_36_offset_0, trace_2_column_37_offset_0, trace_2_column_38_offset_0,
            trace_2_column_39_offset_0,
        ],
    )
        - (QM31Impl::from_partial_evals(
            [
                trace_2_column_32_offset_0, trace_2_column_33_offset_0, trace_2_column_34_offset_0,
                trace_2_column_35_offset_0,
            ],
        )))
        * ((intermediate2) * (intermediate3))
        - (intermediate3 + intermediate2);

    let constraint_16 = (QM31Impl::from_partial_evals(
        [
            trace_2_column_40_offset_0, trace_2_column_41_offset_0, trace_2_column_42_offset_0,
            trace_2_column_43_offset_0,
        ],
    )
        - (QM31Impl::from_partial_evals(
            [
                trace_2_column_36_offset_0, trace_2_column_37_offset_0, trace_2_column_38_offset_0,
                trace_2_column_39_offset_0,
            ],
        )))
        * ((intermediate4) * (intermediate5))
        - (intermediate5 + intermediate4);

    let constraint_17 = (QM31Impl::from_partial_evals(
        [
            trace_2_column_44_offset_0, trace_2_column_45_offset_0, trace_2_column_46_offset_0,
            trace_2_column_47_offset_0,
        ],
    )
        - (QM31Impl::from_partial_evals(
            [
                trace_2_column_40_offset_0, trace_2_column_41_offset_0, trace_2_column_42_offset_0,
                trace_2_column_43_offset_0,
            ],
        )))
        * ((intermediate6) * (intermediate7))
        - (intermediate7 + intermediate6);

    let constraint_18 = (QM31Impl::from_partial_evals(
        [
            trace_2_column_48_offset_claimed_sum, trace_2_column_49_offset_claimed_sum,
            trace_2_column_50_offset_claimed_sum, trace_2_column_51_offset_claimed_sum,
        ],
    )
        - (claimed_sum))
        * (preprocessed_is_first);

    let constraint_19 = (QM31Impl::from_partial_evals(
        [
            trace_2_column_48_offset_0, trace_2_column_49_offset_0, trace_2_column_50_offset_0,
            trace_2_column_51_offset_0,
        ],
    )
        - (QM31Impl::from_partial_evals(
            [
                trace_2_column_48_offset_neg_1, trace_2_column_49_offset_neg_1,
                trace_2_column_50_offset_neg_1, trace_2_column_51_offset_neg_1,
            ],
        )
            - ((total_sum) * (preprocessed_is_first)))
        - (QM31Impl::from_partial_evals(
            [
                trace_2_column_44_offset_0, trace_2_column_45_offset_0, trace_2_column_46_offset_0,
                trace_2_column_47_offset_0,
            ],
        )))
        * (intermediate8)
        - (qm31(2147483646, 0, 0, 0));
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
}


fn intermediates(
    MemoryAddressToId_alpha0: QM31,
    MemoryAddressToId_alpha1: QM31,
    MemoryAddressToId_z: QM31,
    MemoryIdToBig_alpha0: QM31,
    MemoryIdToBig_alpha1: QM31,
    MemoryIdToBig_alpha10: QM31,
    MemoryIdToBig_alpha11: QM31,
    MemoryIdToBig_alpha12: QM31,
    MemoryIdToBig_alpha13: QM31,
    MemoryIdToBig_alpha14: QM31,
    MemoryIdToBig_alpha15: QM31,
    MemoryIdToBig_alpha16: QM31,
    MemoryIdToBig_alpha17: QM31,
    MemoryIdToBig_alpha18: QM31,
    MemoryIdToBig_alpha19: QM31,
    MemoryIdToBig_alpha2: QM31,
    MemoryIdToBig_alpha20: QM31,
    MemoryIdToBig_alpha21: QM31,
    MemoryIdToBig_alpha22: QM31,
    MemoryIdToBig_alpha28: QM31,
    MemoryIdToBig_alpha3: QM31,
    MemoryIdToBig_alpha4: QM31,
    MemoryIdToBig_alpha5: QM31,
    MemoryIdToBig_alpha6: QM31,
    MemoryIdToBig_alpha7: QM31,
    MemoryIdToBig_alpha8: QM31,
    MemoryIdToBig_alpha9: QM31,
    MemoryIdToBig_z: QM31,
    Opcodes_alpha0: QM31,
    Opcodes_alpha1: QM31,
    Opcodes_alpha2: QM31,
    Opcodes_z: QM31,
    VerifyInstruction_alpha0: QM31,
    VerifyInstruction_alpha1: QM31,
    VerifyInstruction_alpha15: QM31,
    VerifyInstruction_alpha18: QM31,
    VerifyInstruction_alpha2: QM31,
    VerifyInstruction_alpha3: QM31,
    VerifyInstruction_alpha4: QM31,
    VerifyInstruction_alpha5: QM31,
    VerifyInstruction_alpha7: QM31,
    VerifyInstruction_alpha8: QM31,
    VerifyInstruction_alpha9: QM31,
    VerifyInstruction_z: QM31,
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
    trace_1_column_20_offset_0: QM31,
    trace_1_column_21_offset_0: QM31,
    trace_1_column_22_offset_0: QM31,
    trace_1_column_23_offset_0: QM31,
    trace_1_column_24_offset_0: QM31,
    trace_1_column_25_offset_0: QM31,
    trace_1_column_26_offset_0: QM31,
    trace_1_column_27_offset_0: QM31,
    trace_1_column_28_offset_0: QM31,
    trace_1_column_29_offset_0: QM31,
    trace_1_column_2_offset_0: QM31,
    trace_1_column_30_offset_0: QM31,
    trace_1_column_31_offset_0: QM31,
    trace_1_column_3_offset_0: QM31,
    trace_1_column_4_offset_0: QM31,
    trace_1_column_5_offset_0: QM31,
    trace_1_column_6_offset_0: QM31,
    trace_1_column_7_offset_0: QM31,
    trace_1_column_8_offset_0: QM31,
    trace_1_column_9_offset_0: QM31,
) -> Array<QM31> {
    let intermediate0 = intermediate0(
        VerifyInstruction_alpha0,
        VerifyInstruction_alpha1,
        VerifyInstruction_alpha15,
        VerifyInstruction_alpha18,
        VerifyInstruction_alpha2,
        VerifyInstruction_alpha3,
        VerifyInstruction_alpha4,
        VerifyInstruction_alpha5,
        VerifyInstruction_alpha7,
        VerifyInstruction_alpha8,
        VerifyInstruction_alpha9,
        VerifyInstruction_z,
        trace_1_column_0_offset_0,
        trace_1_column_10_offset_0,
        trace_1_column_3_offset_0,
        trace_1_column_4_offset_0,
        trace_1_column_5_offset_0,
        trace_1_column_6_offset_0,
        trace_1_column_7_offset_0,
        trace_1_column_8_offset_0,
        trace_1_column_9_offset_0,
    );

    let intermediate1 = intermediate1(
        MemoryAddressToId_alpha0,
        MemoryAddressToId_alpha1,
        MemoryAddressToId_z,
        trace_1_column_11_offset_0,
        trace_1_column_14_offset_0,
        trace_1_column_3_offset_0,
    );

    let intermediate2 = intermediate2(
        MemoryIdToBig_alpha0,
        MemoryIdToBig_alpha1,
        MemoryIdToBig_alpha10,
        MemoryIdToBig_alpha11,
        MemoryIdToBig_alpha12,
        MemoryIdToBig_alpha13,
        MemoryIdToBig_alpha14,
        MemoryIdToBig_alpha15,
        MemoryIdToBig_alpha16,
        MemoryIdToBig_alpha17,
        MemoryIdToBig_alpha18,
        MemoryIdToBig_alpha19,
        MemoryIdToBig_alpha2,
        MemoryIdToBig_alpha20,
        MemoryIdToBig_alpha21,
        MemoryIdToBig_alpha22,
        MemoryIdToBig_alpha28,
        MemoryIdToBig_alpha3,
        MemoryIdToBig_alpha4,
        MemoryIdToBig_alpha5,
        MemoryIdToBig_alpha6,
        MemoryIdToBig_alpha7,
        MemoryIdToBig_alpha8,
        MemoryIdToBig_alpha9,
        MemoryIdToBig_z,
        trace_1_column_14_offset_0,
        trace_1_column_15_offset_0,
        trace_1_column_16_offset_0,
        trace_1_column_17_offset_0,
        trace_1_column_18_offset_0,
        trace_1_column_19_offset_0,
    );

    let intermediate3 = intermediate3(
        MemoryAddressToId_alpha0,
        MemoryAddressToId_alpha1,
        MemoryAddressToId_z,
        trace_1_column_12_offset_0,
        trace_1_column_20_offset_0,
        trace_1_column_4_offset_0,
    );

    let intermediate4 = intermediate4(
        MemoryIdToBig_alpha0,
        MemoryIdToBig_alpha1,
        MemoryIdToBig_alpha10,
        MemoryIdToBig_alpha11,
        MemoryIdToBig_alpha12,
        MemoryIdToBig_alpha13,
        MemoryIdToBig_alpha14,
        MemoryIdToBig_alpha15,
        MemoryIdToBig_alpha16,
        MemoryIdToBig_alpha17,
        MemoryIdToBig_alpha18,
        MemoryIdToBig_alpha19,
        MemoryIdToBig_alpha2,
        MemoryIdToBig_alpha20,
        MemoryIdToBig_alpha21,
        MemoryIdToBig_alpha22,
        MemoryIdToBig_alpha28,
        MemoryIdToBig_alpha3,
        MemoryIdToBig_alpha4,
        MemoryIdToBig_alpha5,
        MemoryIdToBig_alpha6,
        MemoryIdToBig_alpha7,
        MemoryIdToBig_alpha8,
        MemoryIdToBig_alpha9,
        MemoryIdToBig_z,
        trace_1_column_20_offset_0,
        trace_1_column_21_offset_0,
        trace_1_column_22_offset_0,
        trace_1_column_23_offset_0,
        trace_1_column_24_offset_0,
        trace_1_column_25_offset_0,
    );

    let intermediate5 = intermediate5(
        MemoryAddressToId_alpha0,
        MemoryAddressToId_alpha1,
        MemoryAddressToId_z,
        trace_1_column_13_offset_0,
        trace_1_column_26_offset_0,
        trace_1_column_5_offset_0,
    );

    let intermediate6 = intermediate6(
        MemoryIdToBig_alpha0,
        MemoryIdToBig_alpha1,
        MemoryIdToBig_alpha10,
        MemoryIdToBig_alpha11,
        MemoryIdToBig_alpha12,
        MemoryIdToBig_alpha13,
        MemoryIdToBig_alpha14,
        MemoryIdToBig_alpha15,
        MemoryIdToBig_alpha16,
        MemoryIdToBig_alpha17,
        MemoryIdToBig_alpha18,
        MemoryIdToBig_alpha19,
        MemoryIdToBig_alpha2,
        MemoryIdToBig_alpha20,
        MemoryIdToBig_alpha21,
        MemoryIdToBig_alpha22,
        MemoryIdToBig_alpha28,
        MemoryIdToBig_alpha3,
        MemoryIdToBig_alpha4,
        MemoryIdToBig_alpha5,
        MemoryIdToBig_alpha6,
        MemoryIdToBig_alpha7,
        MemoryIdToBig_alpha8,
        MemoryIdToBig_alpha9,
        MemoryIdToBig_z,
        trace_1_column_26_offset_0,
        trace_1_column_27_offset_0,
        trace_1_column_28_offset_0,
        trace_1_column_29_offset_0,
        trace_1_column_30_offset_0,
        trace_1_column_31_offset_0,
    );

    let intermediate7 = intermediate7(
        Opcodes_alpha0,
        Opcodes_alpha1,
        Opcodes_alpha2,
        Opcodes_z,
        trace_1_column_0_offset_0,
        trace_1_column_1_offset_0,
        trace_1_column_2_offset_0,
    );

    let intermediate8 = intermediate8(
        Opcodes_alpha0,
        Opcodes_alpha1,
        Opcodes_alpha2,
        Opcodes_z,
        trace_1_column_0_offset_0,
        trace_1_column_10_offset_0,
        trace_1_column_1_offset_0,
        trace_1_column_2_offset_0,
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
    ]
}


pub fn intermediate0(
    VerifyInstruction_alpha0: QM31,
    VerifyInstruction_alpha1: QM31,
    VerifyInstruction_alpha15: QM31,
    VerifyInstruction_alpha18: QM31,
    VerifyInstruction_alpha2: QM31,
    VerifyInstruction_alpha3: QM31,
    VerifyInstruction_alpha4: QM31,
    VerifyInstruction_alpha5: QM31,
    VerifyInstruction_alpha7: QM31,
    VerifyInstruction_alpha8: QM31,
    VerifyInstruction_alpha9: QM31,
    VerifyInstruction_z: QM31,
    trace_1_column_0_offset_0: QM31,
    trace_1_column_10_offset_0: QM31,
    trace_1_column_3_offset_0: QM31,
    trace_1_column_4_offset_0: QM31,
    trace_1_column_5_offset_0: QM31,
    trace_1_column_6_offset_0: QM31,
    trace_1_column_7_offset_0: QM31,
    trace_1_column_8_offset_0: QM31,
    trace_1_column_9_offset_0: QM31,
) -> QM31 {
    (VerifyInstruction_alpha0) * (trace_1_column_0_offset_0)
        + (VerifyInstruction_alpha1) * (trace_1_column_3_offset_0)
        + (VerifyInstruction_alpha2) * (trace_1_column_4_offset_0)
        + (VerifyInstruction_alpha3) * (trace_1_column_5_offset_0)
        + (VerifyInstruction_alpha4) * (trace_1_column_6_offset_0)
        + (VerifyInstruction_alpha5) * (trace_1_column_7_offset_0)
        + (VerifyInstruction_alpha7) * (trace_1_column_8_offset_0)
        + (VerifyInstruction_alpha8) * (trace_1_column_9_offset_0)
        + VerifyInstruction_alpha9
        + (VerifyInstruction_alpha15) * (trace_1_column_10_offset_0)
        + VerifyInstruction_alpha18
        - (VerifyInstruction_z)
}

pub fn intermediate1(
    MemoryAddressToId_alpha0: QM31,
    MemoryAddressToId_alpha1: QM31,
    MemoryAddressToId_z: QM31,
    trace_1_column_11_offset_0: QM31,
    trace_1_column_14_offset_0: QM31,
    trace_1_column_3_offset_0: QM31,
) -> QM31 {
    (MemoryAddressToId_alpha0)
        * (trace_1_column_11_offset_0 + trace_1_column_3_offset_0 - (m31(32768).into()))
        + (MemoryAddressToId_alpha1) * (trace_1_column_14_offset_0)
        - (MemoryAddressToId_z)
}

pub fn intermediate2(
    MemoryIdToBig_alpha0: QM31,
    MemoryIdToBig_alpha1: QM31,
    MemoryIdToBig_alpha10: QM31,
    MemoryIdToBig_alpha11: QM31,
    MemoryIdToBig_alpha12: QM31,
    MemoryIdToBig_alpha13: QM31,
    MemoryIdToBig_alpha14: QM31,
    MemoryIdToBig_alpha15: QM31,
    MemoryIdToBig_alpha16: QM31,
    MemoryIdToBig_alpha17: QM31,
    MemoryIdToBig_alpha18: QM31,
    MemoryIdToBig_alpha19: QM31,
    MemoryIdToBig_alpha2: QM31,
    MemoryIdToBig_alpha20: QM31,
    MemoryIdToBig_alpha21: QM31,
    MemoryIdToBig_alpha22: QM31,
    MemoryIdToBig_alpha28: QM31,
    MemoryIdToBig_alpha3: QM31,
    MemoryIdToBig_alpha4: QM31,
    MemoryIdToBig_alpha5: QM31,
    MemoryIdToBig_alpha6: QM31,
    MemoryIdToBig_alpha7: QM31,
    MemoryIdToBig_alpha8: QM31,
    MemoryIdToBig_alpha9: QM31,
    MemoryIdToBig_z: QM31,
    trace_1_column_14_offset_0: QM31,
    trace_1_column_15_offset_0: QM31,
    trace_1_column_16_offset_0: QM31,
    trace_1_column_17_offset_0: QM31,
    trace_1_column_18_offset_0: QM31,
    trace_1_column_19_offset_0: QM31,
) -> QM31 {
    (MemoryIdToBig_alpha0) * (trace_1_column_14_offset_0)
        + (MemoryIdToBig_alpha1) * (trace_1_column_17_offset_0)
        + (MemoryIdToBig_alpha2) * (trace_1_column_18_offset_0)
        + (MemoryIdToBig_alpha3) * (trace_1_column_19_offset_0)
        + (MemoryIdToBig_alpha4) * ((trace_1_column_16_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha5) * ((trace_1_column_16_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha6) * ((trace_1_column_16_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha7) * ((trace_1_column_16_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha8) * ((trace_1_column_16_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha9) * ((trace_1_column_16_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha10) * ((trace_1_column_16_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha11) * ((trace_1_column_16_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha12) * ((trace_1_column_16_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha13) * ((trace_1_column_16_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha14) * ((trace_1_column_16_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha15) * ((trace_1_column_16_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha16) * ((trace_1_column_16_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha17) * ((trace_1_column_16_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha18) * ((trace_1_column_16_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha19) * ((trace_1_column_16_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha20) * ((trace_1_column_16_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha21) * ((trace_1_column_16_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha22)
            * ((m31(136).into()) * (trace_1_column_15_offset_0) - (trace_1_column_16_offset_0))
        + (MemoryIdToBig_alpha28) * ((trace_1_column_15_offset_0) * (m31(256).into()))
        - (MemoryIdToBig_z)
}

pub fn intermediate3(
    MemoryAddressToId_alpha0: QM31,
    MemoryAddressToId_alpha1: QM31,
    MemoryAddressToId_z: QM31,
    trace_1_column_12_offset_0: QM31,
    trace_1_column_20_offset_0: QM31,
    trace_1_column_4_offset_0: QM31,
) -> QM31 {
    (MemoryAddressToId_alpha0)
        * (trace_1_column_12_offset_0 + trace_1_column_4_offset_0 - (m31(32768).into()))
        + (MemoryAddressToId_alpha1) * (trace_1_column_20_offset_0)
        - (MemoryAddressToId_z)
}

pub fn intermediate4(
    MemoryIdToBig_alpha0: QM31,
    MemoryIdToBig_alpha1: QM31,
    MemoryIdToBig_alpha10: QM31,
    MemoryIdToBig_alpha11: QM31,
    MemoryIdToBig_alpha12: QM31,
    MemoryIdToBig_alpha13: QM31,
    MemoryIdToBig_alpha14: QM31,
    MemoryIdToBig_alpha15: QM31,
    MemoryIdToBig_alpha16: QM31,
    MemoryIdToBig_alpha17: QM31,
    MemoryIdToBig_alpha18: QM31,
    MemoryIdToBig_alpha19: QM31,
    MemoryIdToBig_alpha2: QM31,
    MemoryIdToBig_alpha20: QM31,
    MemoryIdToBig_alpha21: QM31,
    MemoryIdToBig_alpha22: QM31,
    MemoryIdToBig_alpha28: QM31,
    MemoryIdToBig_alpha3: QM31,
    MemoryIdToBig_alpha4: QM31,
    MemoryIdToBig_alpha5: QM31,
    MemoryIdToBig_alpha6: QM31,
    MemoryIdToBig_alpha7: QM31,
    MemoryIdToBig_alpha8: QM31,
    MemoryIdToBig_alpha9: QM31,
    MemoryIdToBig_z: QM31,
    trace_1_column_20_offset_0: QM31,
    trace_1_column_21_offset_0: QM31,
    trace_1_column_22_offset_0: QM31,
    trace_1_column_23_offset_0: QM31,
    trace_1_column_24_offset_0: QM31,
    trace_1_column_25_offset_0: QM31,
) -> QM31 {
    (MemoryIdToBig_alpha0) * (trace_1_column_20_offset_0)
        + (MemoryIdToBig_alpha1) * (trace_1_column_23_offset_0)
        + (MemoryIdToBig_alpha2) * (trace_1_column_24_offset_0)
        + (MemoryIdToBig_alpha3) * (trace_1_column_25_offset_0)
        + (MemoryIdToBig_alpha4) * ((trace_1_column_22_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha5) * ((trace_1_column_22_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha6) * ((trace_1_column_22_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha7) * ((trace_1_column_22_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha8) * ((trace_1_column_22_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha9) * ((trace_1_column_22_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha10) * ((trace_1_column_22_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha11) * ((trace_1_column_22_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha12) * ((trace_1_column_22_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha13) * ((trace_1_column_22_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha14) * ((trace_1_column_22_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha15) * ((trace_1_column_22_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha16) * ((trace_1_column_22_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha17) * ((trace_1_column_22_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha18) * ((trace_1_column_22_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha19) * ((trace_1_column_22_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha20) * ((trace_1_column_22_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha21) * ((trace_1_column_22_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha22)
            * ((m31(136).into()) * (trace_1_column_21_offset_0) - (trace_1_column_22_offset_0))
        + (MemoryIdToBig_alpha28) * ((trace_1_column_21_offset_0) * (m31(256).into()))
        - (MemoryIdToBig_z)
}

pub fn intermediate5(
    MemoryAddressToId_alpha0: QM31,
    MemoryAddressToId_alpha1: QM31,
    MemoryAddressToId_z: QM31,
    trace_1_column_13_offset_0: QM31,
    trace_1_column_26_offset_0: QM31,
    trace_1_column_5_offset_0: QM31,
) -> QM31 {
    (MemoryAddressToId_alpha0)
        * (trace_1_column_13_offset_0 + trace_1_column_5_offset_0 - (m31(32768).into()))
        + (MemoryAddressToId_alpha1) * (trace_1_column_26_offset_0)
        - (MemoryAddressToId_z)
}

pub fn intermediate6(
    MemoryIdToBig_alpha0: QM31,
    MemoryIdToBig_alpha1: QM31,
    MemoryIdToBig_alpha10: QM31,
    MemoryIdToBig_alpha11: QM31,
    MemoryIdToBig_alpha12: QM31,
    MemoryIdToBig_alpha13: QM31,
    MemoryIdToBig_alpha14: QM31,
    MemoryIdToBig_alpha15: QM31,
    MemoryIdToBig_alpha16: QM31,
    MemoryIdToBig_alpha17: QM31,
    MemoryIdToBig_alpha18: QM31,
    MemoryIdToBig_alpha19: QM31,
    MemoryIdToBig_alpha2: QM31,
    MemoryIdToBig_alpha20: QM31,
    MemoryIdToBig_alpha21: QM31,
    MemoryIdToBig_alpha22: QM31,
    MemoryIdToBig_alpha28: QM31,
    MemoryIdToBig_alpha3: QM31,
    MemoryIdToBig_alpha4: QM31,
    MemoryIdToBig_alpha5: QM31,
    MemoryIdToBig_alpha6: QM31,
    MemoryIdToBig_alpha7: QM31,
    MemoryIdToBig_alpha8: QM31,
    MemoryIdToBig_alpha9: QM31,
    MemoryIdToBig_z: QM31,
    trace_1_column_26_offset_0: QM31,
    trace_1_column_27_offset_0: QM31,
    trace_1_column_28_offset_0: QM31,
    trace_1_column_29_offset_0: QM31,
    trace_1_column_30_offset_0: QM31,
    trace_1_column_31_offset_0: QM31,
) -> QM31 {
    (MemoryIdToBig_alpha0) * (trace_1_column_26_offset_0)
        + (MemoryIdToBig_alpha1) * (trace_1_column_29_offset_0)
        + (MemoryIdToBig_alpha2) * (trace_1_column_30_offset_0)
        + (MemoryIdToBig_alpha3) * (trace_1_column_31_offset_0)
        + (MemoryIdToBig_alpha4) * ((trace_1_column_28_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha5) * ((trace_1_column_28_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha6) * ((trace_1_column_28_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha7) * ((trace_1_column_28_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha8) * ((trace_1_column_28_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha9) * ((trace_1_column_28_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha10) * ((trace_1_column_28_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha11) * ((trace_1_column_28_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha12) * ((trace_1_column_28_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha13) * ((trace_1_column_28_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha14) * ((trace_1_column_28_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha15) * ((trace_1_column_28_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha16) * ((trace_1_column_28_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha17) * ((trace_1_column_28_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha18) * ((trace_1_column_28_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha19) * ((trace_1_column_28_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha20) * ((trace_1_column_28_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha21) * ((trace_1_column_28_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha22)
            * ((m31(136).into()) * (trace_1_column_27_offset_0) - (trace_1_column_28_offset_0))
        + (MemoryIdToBig_alpha28) * ((trace_1_column_27_offset_0) * (m31(256).into()))
        - (MemoryIdToBig_z)
}

pub fn intermediate7(
    Opcodes_alpha0: QM31,
    Opcodes_alpha1: QM31,
    Opcodes_alpha2: QM31,
    Opcodes_z: QM31,
    trace_1_column_0_offset_0: QM31,
    trace_1_column_1_offset_0: QM31,
    trace_1_column_2_offset_0: QM31,
) -> QM31 {
    (Opcodes_alpha0) * (trace_1_column_0_offset_0)
        + (Opcodes_alpha1) * (trace_1_column_1_offset_0)
        + (Opcodes_alpha2) * (trace_1_column_2_offset_0)
        - (Opcodes_z)
}

pub fn intermediate8(
    Opcodes_alpha0: QM31,
    Opcodes_alpha1: QM31,
    Opcodes_alpha2: QM31,
    Opcodes_z: QM31,
    trace_1_column_0_offset_0: QM31,
    trace_1_column_10_offset_0: QM31,
    trace_1_column_1_offset_0: QM31,
    trace_1_column_2_offset_0: QM31,
) -> QM31 {
    (Opcodes_alpha0) * (trace_1_column_0_offset_0 + m31(1).into())
        + (Opcodes_alpha1) * (trace_1_column_1_offset_0 + trace_1_column_10_offset_0)
        + (Opcodes_alpha2) * (trace_1_column_2_offset_0)
        - (Opcodes_z)
}

