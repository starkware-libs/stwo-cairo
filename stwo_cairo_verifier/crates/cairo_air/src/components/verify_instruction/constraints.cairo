use stwo_constraint_framework::{
    PreprocessedColumn, PreprocessedColumnSet, PreprocessedColumnSetImpl,
};
use stwo_verifier_core::{ColumnSpan, ColumnArray};
use stwo_verifier_core::circle::{
    CirclePoint, CirclePointIndex, CirclePointIndexImpl, CirclePointQM31AddCirclePointM31Impl,
};
use stwo_verifier_core::fields::m31::{m31, M31};
use stwo_verifier_core::fields::qm31::{QM31, QM31Impl, qm31};


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

    let [trace_1_column_16_offset_0]: [QM31; 1] = (*trace_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_1_column_17_offset_0]: [QM31; 1] = (*trace_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_1_column_18_offset_0]: [QM31; 1] = (*trace_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_1_column_19_offset_0]: [QM31; 1] = (*trace_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_1_column_20_offset_0]: [QM31; 1] = (*trace_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_1_column_21_offset_0]: [QM31; 1] = (*trace_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_1_column_22_offset_0]: [QM31; 1] = (*trace_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_1_column_23_offset_0]: [QM31; 1] = (*trace_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_1_column_24_offset_0]: [QM31; 1] = (*trace_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_1_column_25_offset_0]: [QM31; 1] = (*trace_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_1_column_26_offset_0]: [QM31; 1] = (*trace_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_1_column_27_offset_0]: [QM31; 1] = (*trace_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_1_column_28_offset_0]: [QM31; 1] = (*trace_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_29_offset_0]: [QM31; 1] = (*interaction_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_30_offset_0]: [QM31; 1] = (*interaction_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_31_offset_0]: [QM31; 1] = (*interaction_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_32_offset_0]: [QM31; 1] = (*interaction_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_33_offset_0]: [QM31; 1] = (*interaction_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_34_offset_0]: [QM31; 1] = (*interaction_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_35_offset_0]: [QM31; 1] = (*interaction_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_36_offset_0]: [QM31; 1] = (*interaction_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_37_offset_neg_1, trace_2_column_37_offset_0]: [QM31; 2] =
        (*interaction_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_38_offset_neg_1, trace_2_column_38_offset_0]: [QM31; 2] =
        (*interaction_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_39_offset_neg_1, trace_2_column_39_offset_0]: [QM31; 2] =
        (*interaction_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_40_offset_neg_1, trace_2_column_40_offset_0]: [QM31; 2] =
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

    // Constrait 0
    let constraint_quotient = (trace_1_column_19_offset_0
        + (trace_1_column_20_offset_0) * (m31(512).into())
        - (trace_1_column_1_offset_0))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constrait 1
    let constraint_quotient = (trace_1_column_21_offset_0
        + (trace_1_column_22_offset_0) * (m31(4).into())
        + (trace_1_column_23_offset_0) * (m31(2048).into())
        - (trace_1_column_2_offset_0))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constrait 2
    let constraint_quotient = (trace_1_column_24_offset_0
        + (trace_1_column_25_offset_0) * (m31(16).into())
        + (trace_1_column_26_offset_0) * (m31(8192).into())
        - (trace_1_column_3_offset_0))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constrait 3
    let constraint_quotient = ((trace_1_column_4_offset_0)
        * (m31(1).into() - (trace_1_column_4_offset_0)))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constrait 4
    let constraint_quotient = ((trace_1_column_5_offset_0)
        * (m31(1).into() - (trace_1_column_5_offset_0)))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constrait 5
    let constraint_quotient = ((trace_1_column_6_offset_0)
        * (m31(1).into() - (trace_1_column_6_offset_0)))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constrait 6
    let constraint_quotient = ((trace_1_column_7_offset_0)
        * (m31(1).into() - (trace_1_column_7_offset_0)))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constrait 7
    let constraint_quotient = ((trace_1_column_8_offset_0)
        * (m31(1).into() - (trace_1_column_8_offset_0)))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constrait 8
    let constraint_quotient = ((trace_1_column_9_offset_0)
        * (m31(1).into() - (trace_1_column_9_offset_0)))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constrait 9
    let constraint_quotient = ((trace_1_column_10_offset_0)
        * (m31(1).into() - (trace_1_column_10_offset_0)))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constrait 10
    let constraint_quotient = ((trace_1_column_11_offset_0)
        * (m31(1).into() - (trace_1_column_11_offset_0)))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constrait 11
    let constraint_quotient = ((trace_1_column_12_offset_0)
        * (m31(1).into() - (trace_1_column_12_offset_0)))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constrait 12
    let constraint_quotient = ((trace_1_column_13_offset_0)
        * (m31(1).into() - (trace_1_column_13_offset_0)))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constrait 13
    let constraint_quotient = ((trace_1_column_14_offset_0)
        * (m31(1).into() - (trace_1_column_14_offset_0)))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constrait 14
    let constraint_quotient = ((trace_1_column_15_offset_0)
        * (m31(1).into() - (trace_1_column_15_offset_0)))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constrait 15
    let constraint_quotient = ((trace_1_column_16_offset_0)
        * (m31(1).into() - (trace_1_column_16_offset_0)))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constrait 16
    let constraint_quotient = ((trace_1_column_17_offset_0)
        * (m31(1).into() - (trace_1_column_17_offset_0)))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constrait 17
    let constraint_quotient = ((trace_1_column_18_offset_0)
        * (m31(1).into() - (trace_1_column_18_offset_0)))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constrait 18
    let constraint_quotient = ((QM31Impl::from_partial_evals(
        [
            trace_2_column_29_offset_0, trace_2_column_30_offset_0, trace_2_column_31_offset_0,
            trace_2_column_32_offset_0,
        ],
    ))
        * ((intermediate0) * (intermediate1))
        - (intermediate1 + intermediate0))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constrait 19
    let constraint_quotient = ((QM31Impl::from_partial_evals(
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
        - (intermediate3 + intermediate2))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constrait 20
    let constraint_quotient = ((QM31Impl::from_partial_evals(
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
        ))
        - (QM31Impl::from_partial_evals(
            [
                trace_2_column_33_offset_0, trace_2_column_34_offset_0, trace_2_column_35_offset_0,
                trace_2_column_36_offset_0,
            ],
        ))
        + (claimed_sum) * (qm31(32768, 0, 0, 0)))
        * (intermediate4)
        - (-(trace_1_column_28_offset_0)))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;
}


fn intermediates(
    MemoryAddressToId_alpha0: QM31,
    MemoryAddressToId_alpha1: QM31,
    MemoryAddressToId_z: QM31,
    MemoryIdToBig_alpha0: QM31,
    MemoryIdToBig_alpha1: QM31,
    MemoryIdToBig_alpha2: QM31,
    MemoryIdToBig_alpha3: QM31,
    MemoryIdToBig_alpha4: QM31,
    MemoryIdToBig_alpha5: QM31,
    MemoryIdToBig_alpha6: QM31,
    MemoryIdToBig_alpha7: QM31,
    MemoryIdToBig_z: QM31,
    RangeCheck_4_3_alpha0: QM31,
    RangeCheck_4_3_alpha1: QM31,
    RangeCheck_4_3_z: QM31,
    RangeCheck_7_2_5_alpha0: QM31,
    RangeCheck_7_2_5_alpha1: QM31,
    RangeCheck_7_2_5_alpha2: QM31,
    RangeCheck_7_2_5_z: QM31,
    VerifyInstruction_alpha0: QM31,
    VerifyInstruction_alpha1: QM31,
    VerifyInstruction_alpha10: QM31,
    VerifyInstruction_alpha11: QM31,
    VerifyInstruction_alpha12: QM31,
    VerifyInstruction_alpha13: QM31,
    VerifyInstruction_alpha14: QM31,
    VerifyInstruction_alpha15: QM31,
    VerifyInstruction_alpha16: QM31,
    VerifyInstruction_alpha17: QM31,
    VerifyInstruction_alpha18: QM31,
    VerifyInstruction_alpha2: QM31,
    VerifyInstruction_alpha3: QM31,
    VerifyInstruction_alpha4: QM31,
    VerifyInstruction_alpha5: QM31,
    VerifyInstruction_alpha6: QM31,
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
        RangeCheck_7_2_5_alpha0,
        RangeCheck_7_2_5_alpha1,
        RangeCheck_7_2_5_alpha2,
        RangeCheck_7_2_5_z,
        trace_1_column_20_offset_0,
        trace_1_column_21_offset_0,
        trace_1_column_23_offset_0,
    );

    let intermediate1 = intermediate1(
        RangeCheck_4_3_alpha0,
        RangeCheck_4_3_alpha1,
        RangeCheck_4_3_z,
        trace_1_column_24_offset_0,
        trace_1_column_26_offset_0,
    );

    let intermediate2 = intermediate2(
        MemoryAddressToId_alpha0,
        MemoryAddressToId_alpha1,
        MemoryAddressToId_z,
        trace_1_column_0_offset_0,
        trace_1_column_27_offset_0,
    );

    let intermediate3 = intermediate3(
        MemoryIdToBig_alpha0,
        MemoryIdToBig_alpha1,
        MemoryIdToBig_alpha2,
        MemoryIdToBig_alpha3,
        MemoryIdToBig_alpha4,
        MemoryIdToBig_alpha5,
        MemoryIdToBig_alpha6,
        MemoryIdToBig_alpha7,
        MemoryIdToBig_z,
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
        trace_1_column_20_offset_0,
        trace_1_column_21_offset_0,
        trace_1_column_22_offset_0,
        trace_1_column_23_offset_0,
        trace_1_column_24_offset_0,
        trace_1_column_25_offset_0,
        trace_1_column_26_offset_0,
        trace_1_column_27_offset_0,
        trace_1_column_4_offset_0,
        trace_1_column_5_offset_0,
        trace_1_column_6_offset_0,
        trace_1_column_7_offset_0,
        trace_1_column_8_offset_0,
        trace_1_column_9_offset_0,
    );

    let intermediate4 = intermediate4(
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
    );
    array![intermediate0, intermediate1, intermediate2, intermediate3, intermediate4]
}


pub fn intermediate0(
    RangeCheck_7_2_5_alpha0: QM31,
    RangeCheck_7_2_5_alpha1: QM31,
    RangeCheck_7_2_5_alpha2: QM31,
    RangeCheck_7_2_5_z: QM31,
    trace_1_column_20_offset_0: QM31,
    trace_1_column_21_offset_0: QM31,
    trace_1_column_23_offset_0: QM31,
) -> QM31 {
    (RangeCheck_7_2_5_alpha0) * (trace_1_column_20_offset_0)
        + (RangeCheck_7_2_5_alpha1) * (trace_1_column_21_offset_0)
        + (RangeCheck_7_2_5_alpha2) * (trace_1_column_23_offset_0)
        - (RangeCheck_7_2_5_z)
}

pub fn intermediate1(
    RangeCheck_4_3_alpha0: QM31,
    RangeCheck_4_3_alpha1: QM31,
    RangeCheck_4_3_z: QM31,
    trace_1_column_24_offset_0: QM31,
    trace_1_column_26_offset_0: QM31,
) -> QM31 {
    (RangeCheck_4_3_alpha0) * (trace_1_column_24_offset_0)
        + (RangeCheck_4_3_alpha1) * (trace_1_column_26_offset_0)
        - (RangeCheck_4_3_z)
}

pub fn intermediate2(
    MemoryAddressToId_alpha0: QM31,
    MemoryAddressToId_alpha1: QM31,
    MemoryAddressToId_z: QM31,
    trace_1_column_0_offset_0: QM31,
    trace_1_column_27_offset_0: QM31,
) -> QM31 {
    (MemoryAddressToId_alpha0) * (trace_1_column_0_offset_0)
        + (MemoryAddressToId_alpha1) * (trace_1_column_27_offset_0)
        - (MemoryAddressToId_z)
}

pub fn intermediate3(
    MemoryIdToBig_alpha0: QM31,
    MemoryIdToBig_alpha1: QM31,
    MemoryIdToBig_alpha2: QM31,
    MemoryIdToBig_alpha3: QM31,
    MemoryIdToBig_alpha4: QM31,
    MemoryIdToBig_alpha5: QM31,
    MemoryIdToBig_alpha6: QM31,
    MemoryIdToBig_alpha7: QM31,
    MemoryIdToBig_z: QM31,
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
    trace_1_column_20_offset_0: QM31,
    trace_1_column_21_offset_0: QM31,
    trace_1_column_22_offset_0: QM31,
    trace_1_column_23_offset_0: QM31,
    trace_1_column_24_offset_0: QM31,
    trace_1_column_25_offset_0: QM31,
    trace_1_column_26_offset_0: QM31,
    trace_1_column_27_offset_0: QM31,
    trace_1_column_4_offset_0: QM31,
    trace_1_column_5_offset_0: QM31,
    trace_1_column_6_offset_0: QM31,
    trace_1_column_7_offset_0: QM31,
    trace_1_column_8_offset_0: QM31,
    trace_1_column_9_offset_0: QM31,
) -> QM31 {
    (MemoryIdToBig_alpha0) * (trace_1_column_27_offset_0)
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
        - (MemoryIdToBig_z)
}

pub fn intermediate4(
    VerifyInstruction_alpha0: QM31,
    VerifyInstruction_alpha1: QM31,
    VerifyInstruction_alpha10: QM31,
    VerifyInstruction_alpha11: QM31,
    VerifyInstruction_alpha12: QM31,
    VerifyInstruction_alpha13: QM31,
    VerifyInstruction_alpha14: QM31,
    VerifyInstruction_alpha15: QM31,
    VerifyInstruction_alpha16: QM31,
    VerifyInstruction_alpha17: QM31,
    VerifyInstruction_alpha18: QM31,
    VerifyInstruction_alpha2: QM31,
    VerifyInstruction_alpha3: QM31,
    VerifyInstruction_alpha4: QM31,
    VerifyInstruction_alpha5: QM31,
    VerifyInstruction_alpha6: QM31,
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
    (VerifyInstruction_alpha0) * (trace_1_column_0_offset_0)
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
        - (VerifyInstruction_z)
}

