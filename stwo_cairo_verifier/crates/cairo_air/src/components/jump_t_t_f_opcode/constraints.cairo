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
    let ConstraintParams { MemoryAddressToId_alpha0,
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
    let trace_2_column_18_offset_neg_1 = *trace_2_column_18.pop_front().unwrap();
    let trace_2_column_18_offset_0 = *trace_2_column_18.pop_front().unwrap();
    let trace_2_column_18_offset_claimed_sum = *trace_2_column_18.pop_front().unwrap();
    let mut trace_2_column_19 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_19_offset_neg_1 = *trace_2_column_19.pop_front().unwrap();
    let trace_2_column_19_offset_0 = *trace_2_column_19.pop_front().unwrap();
    let trace_2_column_19_offset_claimed_sum = *trace_2_column_19.pop_front().unwrap();
    let mut trace_2_column_20 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_20_offset_neg_1 = *trace_2_column_20.pop_front().unwrap();
    let trace_2_column_20_offset_0 = *trace_2_column_20.pop_front().unwrap();
    let trace_2_column_20_offset_claimed_sum = *trace_2_column_20.pop_front().unwrap();
    let mut trace_2_column_21 = interaction_mask_values.pop_front().unwrap().span();
    let trace_2_column_21_offset_neg_1 = *trace_2_column_21.pop_front().unwrap();
    let trace_2_column_21_offset_0 = *trace_2_column_21.pop_front().unwrap();
    let trace_2_column_21_offset_claimed_sum = *trace_2_column_21.pop_front().unwrap();
    let intermediate0 = (VerifyInstruction_alpha0) * (trace_1_column_0_offset_0)
        + (VerifyInstruction_alpha1) * (qm31(32767, 0, 0, 0))
        + (VerifyInstruction_alpha2) * (qm31(32767, 0, 0, 0))
        + (VerifyInstruction_alpha3) * (qm31(32769, 0, 0, 0))
        + VerifyInstruction_alpha4
        + VerifyInstruction_alpha5
        + VerifyInstruction_alpha6
        + VerifyInstruction_alpha12
        + (VerifyInstruction_alpha15) * (trace_1_column_3_offset_0)
        - (VerifyInstruction_z);

    let intermediate1 = (MemoryAddressToId_alpha0) * (trace_1_column_0_offset_0 + m31(1).into())
        + (MemoryAddressToId_alpha1) * (trace_1_column_4_offset_0)
        - (MemoryAddressToId_z);

    let intermediate2 = (MemoryIdToBig_alpha0) * (trace_1_column_4_offset_0)
        + (MemoryIdToBig_alpha1) * (trace_1_column_7_offset_0)
        + (MemoryIdToBig_alpha2) * (trace_1_column_8_offset_0)
        + (MemoryIdToBig_alpha3) * (trace_1_column_9_offset_0)
        + (MemoryIdToBig_alpha4) * ((trace_1_column_6_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha5) * ((trace_1_column_6_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha6) * ((trace_1_column_6_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha7) * ((trace_1_column_6_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha8) * ((trace_1_column_6_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha9) * ((trace_1_column_6_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha10) * ((trace_1_column_6_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha11) * ((trace_1_column_6_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha12) * ((trace_1_column_6_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha13) * ((trace_1_column_6_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha14) * ((trace_1_column_6_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha15) * ((trace_1_column_6_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha16) * ((trace_1_column_6_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha17) * ((trace_1_column_6_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha18) * ((trace_1_column_6_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha19) * ((trace_1_column_6_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha20) * ((trace_1_column_6_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha21) * ((trace_1_column_6_offset_0) * (m31(511).into()))
        + (MemoryIdToBig_alpha22)
            * ((m31(136).into()) * (trace_1_column_5_offset_0) - (trace_1_column_6_offset_0))
        + (MemoryIdToBig_alpha28) * ((trace_1_column_5_offset_0) * (m31(256).into()))
        - (MemoryIdToBig_z);

    let intermediate3 = (Opcodes_alpha0) * (trace_1_column_0_offset_0)
        + (Opcodes_alpha1) * (trace_1_column_1_offset_0)
        + (Opcodes_alpha2) * (trace_1_column_2_offset_0)
        - (Opcodes_z);

    let intermediate4 = (Opcodes_alpha0)
        * (trace_1_column_0_offset_0
            + trace_1_column_7_offset_0
            + (trace_1_column_8_offset_0) * (m31(512).into())
            + (trace_1_column_9_offset_0) * (m31(262144).into())
            - (trace_1_column_5_offset_0)
            - ((m31(134217728).into()) * (trace_1_column_6_offset_0)))
        + (Opcodes_alpha1) * (trace_1_column_1_offset_0 + trace_1_column_3_offset_0)
        + (Opcodes_alpha2) * (trace_1_column_2_offset_0)
        - (Opcodes_z);

    let constraint_0 = (trace_1_column_5_offset_0) * (trace_1_column_5_offset_0 - (m31(1).into()));

    let constraint_1 = (trace_1_column_6_offset_0) * (trace_1_column_6_offset_0 - (m31(1).into()));

    let constraint_2 = (trace_1_column_6_offset_0) * (trace_1_column_5_offset_0 - (m31(1).into()));

    let constraint_3 = (QM31Impl::from_partial_evals(
        [
            trace_2_column_10_offset_0, trace_2_column_11_offset_0, trace_2_column_12_offset_0,
            trace_2_column_13_offset_0,
        ],
    ))
        * ((intermediate0) * (intermediate1))
        - (intermediate1 + intermediate0);

    let constraint_4 = (QM31Impl::from_partial_evals(
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
        * ((intermediate2) * (intermediate3))
        - (intermediate3 + intermediate2);

    let constraint_5 = (QM31Impl::from_partial_evals(
        [
            trace_2_column_18_offset_claimed_sum, trace_2_column_19_offset_claimed_sum,
            trace_2_column_20_offset_claimed_sum, trace_2_column_21_offset_claimed_sum,
        ],
    )
        - (claimed_sum))
        * (preprocessed_is_first);

    let constraint_6 = (QM31Impl::from_partial_evals(
        [
            trace_2_column_18_offset_0, trace_2_column_19_offset_0, trace_2_column_20_offset_0,
            trace_2_column_21_offset_0,
        ],
    )
        - (QM31Impl::from_partial_evals(
            [
                trace_2_column_18_offset_neg_1, trace_2_column_19_offset_neg_1,
                trace_2_column_20_offset_neg_1, trace_2_column_21_offset_neg_1,
            ],
        )
            - ((total_sum) * (preprocessed_is_first)))
        - (QM31Impl::from_partial_evals(
            [
                trace_2_column_14_offset_0, trace_2_column_15_offset_0, trace_2_column_16_offset_0,
                trace_2_column_17_offset_0,
            ],
        )))
        * (intermediate4)
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
}
