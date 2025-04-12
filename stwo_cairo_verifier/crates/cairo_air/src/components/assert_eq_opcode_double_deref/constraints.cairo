use stwo_constraint_framework::{
    PreprocessedColumn, PreprocessedColumnSet, PreprocessedColumnSetImpl,
};
use stwo_verifier_core::circle::{
    CirclePoint, CirclePointIndex, CirclePointIndexImpl, CirclePointQM31AddCirclePointM31Impl,
};
use stwo_verifier_core::fields::Invertible;
use stwo_verifier_core::fields::m31::{M31, m31};
use stwo_verifier_core::fields::qm31::{QM31, QM31Trait, qm31_const};
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
    pub MemoryIdToBig_alpha0: QM31,
    pub MemoryIdToBig_alpha1: QM31,
    pub MemoryIdToBig_alpha2: QM31,
    pub MemoryIdToBig_alpha3: QM31,
    pub MemoryIdToBig_z: QM31,
    pub Opcodes_alpha0: QM31,
    pub Opcodes_alpha1: QM31,
    pub Opcodes_alpha2: QM31,
    pub Opcodes_z: QM31,
    pub VerifyInstruction_alpha0: QM31,
    pub VerifyInstruction_alpha1: QM31,
    pub VerifyInstruction_alpha2: QM31,
    pub VerifyInstruction_alpha3: QM31,
    pub VerifyInstruction_alpha4: QM31,
    pub VerifyInstruction_alpha5: QM31,
    pub VerifyInstruction_z: QM31,
    pub claimed_sum: QM31,
    pub column_size: M31,
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
        MemoryIdToBig_alpha0,
        MemoryIdToBig_alpha1,
        MemoryIdToBig_alpha2,
        MemoryIdToBig_alpha3,
        MemoryIdToBig_z,
        Opcodes_alpha0,
        Opcodes_alpha1,
        Opcodes_alpha2,
        Opcodes_z,
        VerifyInstruction_alpha0,
        VerifyInstruction_alpha1,
        VerifyInstruction_alpha2,
        VerifyInstruction_alpha3,
        VerifyInstruction_alpha4,
        VerifyInstruction_alpha5,
        VerifyInstruction_z,
        claimed_sum,
        column_size,
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
        trace_1_column_16,
    ]: [Span<QM31>; 17] =
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

    let [trace_1_column_16_offset_0]: [QM31; 1] = (*trace_1_column_16.try_into().unwrap()).unbox();

    let [
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
        trace_2_column_32,
    ]: [Span<QM31>; 16] =
        (*interaction_mask_values
        .multi_pop_front()
        .unwrap())
        .unbox();

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

    let [trace_2_column_28_offset_0]: [QM31; 1] = (*trace_2_column_28.try_into().unwrap()).unbox();

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

    let [trace_2_column_32_offset_neg_1, trace_2_column_32_offset_0]: [QM31; 2] =
        (*trace_2_column_32
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
        MemoryIdToBig_z,
        Opcodes_alpha0,
        Opcodes_alpha1,
        Opcodes_alpha2,
        Opcodes_z,
        VerifyInstruction_alpha0,
        VerifyInstruction_alpha1,
        VerifyInstruction_alpha2,
        VerifyInstruction_alpha3,
        VerifyInstruction_alpha4,
        VerifyInstruction_alpha5,
        VerifyInstruction_z,
        trace_1_column_0_offset_0,
        trace_1_column_10_offset_0,
        trace_1_column_11_offset_0,
        trace_1_column_12_offset_0,
        trace_1_column_13_offset_0,
        trace_1_column_14_offset_0,
        trace_1_column_15_offset_0,
        trace_1_column_1_offset_0,
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
    let intermediate5 = *intermediates.pop_front().unwrap();
    let intermediate6 = *intermediates.pop_front().unwrap();
    let intermediate7 = *intermediates.pop_front().unwrap();
    let intermediate8 = *intermediates.pop_front().unwrap();
    let intermediate9 = *intermediates.pop_front().unwrap();

    // Constraint 0
    let constraint_quotient = ((trace_1_column_16_offset_0) * (trace_1_column_16_offset_0)
        - (trace_1_column_16_offset_0))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 1
    let constraint_quotient = ((trace_1_column_6_offset_0)
        * (m31(1).into() - (trace_1_column_6_offset_0)))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 2
    let constraint_quotient = ((trace_1_column_7_offset_0)
        * (m31(1).into() - (trace_1_column_7_offset_0)))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 3
    let constraint_quotient = ((trace_1_column_8_offset_0)
        * (m31(1).into() - (trace_1_column_8_offset_0)))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 4
    let constraint_quotient = (trace_1_column_9_offset_0
        - ((trace_1_column_6_offset_0) * (trace_1_column_2_offset_0)
            + (m31(1).into() - (trace_1_column_6_offset_0)) * (trace_1_column_1_offset_0)))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 5
    let constraint_quotient = (trace_1_column_10_offset_0
        - ((trace_1_column_7_offset_0) * (trace_1_column_2_offset_0)
            + (m31(1).into() - (trace_1_column_7_offset_0)) * (trace_1_column_1_offset_0)))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 6
    let constraint_quotient = ((QM31Trait::from_partial_evals(
        [
            trace_2_column_17_offset_0, trace_2_column_18_offset_0, trace_2_column_19_offset_0,
            trace_2_column_20_offset_0,
        ],
    ))
        * ((intermediate0) * (intermediate4))
        - (intermediate4 + intermediate0))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 7
    let constraint_quotient = ((QM31Trait::from_partial_evals(
        [
            trace_2_column_21_offset_0, trace_2_column_22_offset_0, trace_2_column_23_offset_0,
            trace_2_column_24_offset_0,
        ],
    )
        - (QM31Trait::from_partial_evals(
            [
                trace_2_column_17_offset_0, trace_2_column_18_offset_0, trace_2_column_19_offset_0,
                trace_2_column_20_offset_0,
            ],
        )))
        * ((intermediate5) * (intermediate6))
        - (intermediate6 + intermediate5))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 8
    let constraint_quotient = ((QM31Trait::from_partial_evals(
        [
            trace_2_column_25_offset_0, trace_2_column_26_offset_0, trace_2_column_27_offset_0,
            trace_2_column_28_offset_0,
        ],
    )
        - (QM31Trait::from_partial_evals(
            [
                trace_2_column_21_offset_0, trace_2_column_22_offset_0, trace_2_column_23_offset_0,
                trace_2_column_24_offset_0,
            ],
        )))
        * ((intermediate7) * (intermediate8))
        - (intermediate8 + (intermediate7) * (trace_1_column_16_offset_0)))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 9
    let constraint_quotient = ((QM31Trait::from_partial_evals(
        [
            trace_2_column_29_offset_0, trace_2_column_30_offset_0, trace_2_column_31_offset_0,
            trace_2_column_32_offset_0,
        ],
    )
        - (QM31Trait::from_partial_evals(
            [
                trace_2_column_29_offset_neg_1, trace_2_column_30_offset_neg_1,
                trace_2_column_31_offset_neg_1, trace_2_column_32_offset_neg_1,
            ],
        ))
        - (QM31Trait::from_partial_evals(
            [
                trace_2_column_25_offset_0, trace_2_column_26_offset_0, trace_2_column_27_offset_0,
                trace_2_column_28_offset_0,
            ],
        ))
        + (claimed_sum) * (column_size.inverse().into()))
        * (intermediate9)
        + trace_1_column_16_offset_0)
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
    MemoryIdToBig_z: QM31,
    Opcodes_alpha0: QM31,
    Opcodes_alpha1: QM31,
    Opcodes_alpha2: QM31,
    Opcodes_z: QM31,
    VerifyInstruction_alpha0: QM31,
    VerifyInstruction_alpha1: QM31,
    VerifyInstruction_alpha2: QM31,
    VerifyInstruction_alpha3: QM31,
    VerifyInstruction_alpha4: QM31,
    VerifyInstruction_alpha5: QM31,
    VerifyInstruction_z: QM31,
    trace_1_column_0_offset_0: QM31,
    trace_1_column_10_offset_0: QM31,
    trace_1_column_11_offset_0: QM31,
    trace_1_column_12_offset_0: QM31,
    trace_1_column_13_offset_0: QM31,
    trace_1_column_14_offset_0: QM31,
    trace_1_column_15_offset_0: QM31,
    trace_1_column_1_offset_0: QM31,
    trace_1_column_2_offset_0: QM31,
    trace_1_column_3_offset_0: QM31,
    trace_1_column_4_offset_0: QM31,
    trace_1_column_5_offset_0: QM31,
    trace_1_column_6_offset_0: QM31,
    trace_1_column_7_offset_0: QM31,
    trace_1_column_8_offset_0: QM31,
    trace_1_column_9_offset_0: QM31,
) -> Array<QM31> {
    let intermediate1 = intermediate1(trace_1_column_3_offset_0);

    let intermediate2 = intermediate2(trace_1_column_4_offset_0);

    let intermediate3 = intermediate3(trace_1_column_5_offset_0);
    let intermediate6 = intermediate6(
        MemoryAddressToId_alpha0,
        MemoryAddressToId_alpha1,
        MemoryAddressToId_z,
        intermediate1,
        trace_1_column_15_offset_0,
        trace_1_column_9_offset_0,
    );

    let intermediate9 = intermediate9(
        Opcodes_alpha0,
        Opcodes_alpha1,
        Opcodes_alpha2,
        Opcodes_z,
        trace_1_column_0_offset_0,
        trace_1_column_1_offset_0,
        trace_1_column_2_offset_0,
        trace_1_column_8_offset_0,
    );

    let intermediate5 = intermediate5(
        MemoryIdToBig_alpha0,
        MemoryIdToBig_alpha1,
        MemoryIdToBig_alpha2,
        MemoryIdToBig_alpha3,
        MemoryIdToBig_z,
        trace_1_column_11_offset_0,
        trace_1_column_12_offset_0,
        trace_1_column_13_offset_0,
        trace_1_column_14_offset_0,
    );

    let intermediate4 = intermediate4(
        MemoryAddressToId_alpha0,
        MemoryAddressToId_alpha1,
        MemoryAddressToId_z,
        intermediate2,
        trace_1_column_10_offset_0,
        trace_1_column_11_offset_0,
    );

    let intermediate7 = intermediate7(
        MemoryAddressToId_alpha0,
        MemoryAddressToId_alpha1,
        MemoryAddressToId_z,
        intermediate3,
        trace_1_column_12_offset_0,
        trace_1_column_13_offset_0,
        trace_1_column_14_offset_0,
        trace_1_column_15_offset_0,
    );

    let intermediate0 = intermediate0(
        VerifyInstruction_alpha0,
        VerifyInstruction_alpha1,
        VerifyInstruction_alpha2,
        VerifyInstruction_alpha3,
        VerifyInstruction_alpha4,
        VerifyInstruction_alpha5,
        VerifyInstruction_z,
        trace_1_column_0_offset_0,
        trace_1_column_3_offset_0,
        trace_1_column_4_offset_0,
        trace_1_column_5_offset_0,
        trace_1_column_6_offset_0,
        trace_1_column_7_offset_0,
        trace_1_column_8_offset_0,
    );

    let intermediate8 = intermediate8(
        Opcodes_alpha0,
        Opcodes_alpha1,
        Opcodes_alpha2,
        Opcodes_z,
        trace_1_column_0_offset_0,
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
        intermediate9,
    ]
}

pub fn intermediate3(trace_1_column_5_offset_0: QM31) -> QM31 {
    trace_1_column_5_offset_0 - (m31(32768).into())
}

pub fn intermediate2(trace_1_column_4_offset_0: QM31) -> QM31 {
    trace_1_column_4_offset_0 - (m31(32768).into())
}

pub fn intermediate1(trace_1_column_3_offset_0: QM31) -> QM31 {
    trace_1_column_3_offset_0 - (m31(32768).into())
}
pub fn intermediate6(
    MemoryAddressToId_alpha0: QM31,
    MemoryAddressToId_alpha1: QM31,
    MemoryAddressToId_z: QM31,
    intermediate1: QM31,
    trace_1_column_15_offset_0: QM31,
    trace_1_column_9_offset_0: QM31,
) -> QM31 {
    (MemoryAddressToId_alpha0) * (trace_1_column_9_offset_0 + intermediate1)
        + (MemoryAddressToId_alpha1) * (trace_1_column_15_offset_0)
        - (MemoryAddressToId_z)
}

pub fn intermediate9(
    Opcodes_alpha0: QM31,
    Opcodes_alpha1: QM31,
    Opcodes_alpha2: QM31,
    Opcodes_z: QM31,
    trace_1_column_0_offset_0: QM31,
    trace_1_column_1_offset_0: QM31,
    trace_1_column_2_offset_0: QM31,
    trace_1_column_8_offset_0: QM31,
) -> QM31 {
    (Opcodes_alpha0) * (trace_1_column_0_offset_0 + m31(1).into())
        + (Opcodes_alpha1) * (trace_1_column_1_offset_0 + trace_1_column_8_offset_0)
        + (Opcodes_alpha2) * (trace_1_column_2_offset_0)
        - (Opcodes_z)
}

pub fn intermediate5(
    MemoryIdToBig_alpha0: QM31,
    MemoryIdToBig_alpha1: QM31,
    MemoryIdToBig_alpha2: QM31,
    MemoryIdToBig_alpha3: QM31,
    MemoryIdToBig_z: QM31,
    trace_1_column_11_offset_0: QM31,
    trace_1_column_12_offset_0: QM31,
    trace_1_column_13_offset_0: QM31,
    trace_1_column_14_offset_0: QM31,
) -> QM31 {
    (MemoryIdToBig_alpha0) * (trace_1_column_11_offset_0)
        + (MemoryIdToBig_alpha1) * (trace_1_column_12_offset_0)
        + (MemoryIdToBig_alpha2) * (trace_1_column_13_offset_0)
        + (MemoryIdToBig_alpha3) * (trace_1_column_14_offset_0)
        - (MemoryIdToBig_z)
}

pub fn intermediate4(
    MemoryAddressToId_alpha0: QM31,
    MemoryAddressToId_alpha1: QM31,
    MemoryAddressToId_z: QM31,
    intermediate2: QM31,
    trace_1_column_10_offset_0: QM31,
    trace_1_column_11_offset_0: QM31,
) -> QM31 {
    (MemoryAddressToId_alpha0) * (trace_1_column_10_offset_0 + intermediate2)
        + (MemoryAddressToId_alpha1) * (trace_1_column_11_offset_0)
        - (MemoryAddressToId_z)
}

pub fn intermediate7(
    MemoryAddressToId_alpha0: QM31,
    MemoryAddressToId_alpha1: QM31,
    MemoryAddressToId_z: QM31,
    intermediate3: QM31,
    trace_1_column_12_offset_0: QM31,
    trace_1_column_13_offset_0: QM31,
    trace_1_column_14_offset_0: QM31,
    trace_1_column_15_offset_0: QM31,
) -> QM31 {
    (MemoryAddressToId_alpha0)
        * (trace_1_column_12_offset_0
            + (trace_1_column_13_offset_0) * (m31(512).into())
            + (trace_1_column_14_offset_0) * (m31(262144).into())
            + intermediate3)
        + (MemoryAddressToId_alpha1) * (trace_1_column_15_offset_0)
        - (MemoryAddressToId_z)
}

pub fn intermediate0(
    VerifyInstruction_alpha0: QM31,
    VerifyInstruction_alpha1: QM31,
    VerifyInstruction_alpha2: QM31,
    VerifyInstruction_alpha3: QM31,
    VerifyInstruction_alpha4: QM31,
    VerifyInstruction_alpha5: QM31,
    VerifyInstruction_z: QM31,
    trace_1_column_0_offset_0: QM31,
    trace_1_column_3_offset_0: QM31,
    trace_1_column_4_offset_0: QM31,
    trace_1_column_5_offset_0: QM31,
    trace_1_column_6_offset_0: QM31,
    trace_1_column_7_offset_0: QM31,
    trace_1_column_8_offset_0: QM31,
) -> QM31 {
    (VerifyInstruction_alpha0) * (trace_1_column_0_offset_0)
        + (VerifyInstruction_alpha1) * (trace_1_column_3_offset_0)
        + (VerifyInstruction_alpha2) * (trace_1_column_4_offset_0)
        + (VerifyInstruction_alpha3) * (trace_1_column_5_offset_0)
        + (VerifyInstruction_alpha4)
            * ((trace_1_column_6_offset_0) * (m31(8).into())
                + (trace_1_column_7_offset_0) * (m31(16).into()))
        + (VerifyInstruction_alpha5)
            * ((trace_1_column_8_offset_0) * (m31(32).into()) + m31(256).into())
        - (VerifyInstruction_z)
}

pub fn intermediate8(
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

