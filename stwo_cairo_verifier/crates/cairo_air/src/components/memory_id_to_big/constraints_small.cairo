use crate::prelude::*;

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
    pub RangeCheck_9_9_b_alpha0: QM31,
    pub RangeCheck_9_9_b_alpha1: QM31,
    pub RangeCheck_9_9_b_z: QM31,
    pub RangeCheck_9_9_c_alpha0: QM31,
    pub RangeCheck_9_9_c_alpha1: QM31,
    pub RangeCheck_9_9_c_z: QM31,
    pub RangeCheck_9_9_d_alpha0: QM31,
    pub RangeCheck_9_9_d_alpha1: QM31,
    pub RangeCheck_9_9_d_z: QM31,
    pub claimed_sum: QM31,
    pub seq: QM31,
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
        RangeCheck_9_9_b_alpha0,
        RangeCheck_9_9_b_alpha1,
        RangeCheck_9_9_b_z,
        RangeCheck_9_9_c_alpha0,
        RangeCheck_9_9_c_alpha1,
        RangeCheck_9_9_c_z,
        RangeCheck_9_9_d_alpha0,
        RangeCheck_9_9_d_alpha1,
        RangeCheck_9_9_d_z,
        claimed_sum,
        seq,
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
    ]: [Span<QM31>; 9] =
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

    let [
        trace_2_column_9,
        trace_2_column_10,
        trace_2_column_11,
        trace_2_column_12,
        trace_2_column_13,
        trace_2_column_14,
        trace_2_column_15,
        trace_2_column_16,
        trace_2_column_17,
        trace_2_column_18,
        trace_2_column_19,
        trace_2_column_20,
    ]: [Span<QM31>; 12] =
        (*interaction_mask_values
        .multi_pop_front()
        .unwrap())
        .unbox();

    let [trace_2_column_9_offset_0]: [QM31; 1] = (*trace_2_column_9.try_into().unwrap()).unbox();

    let [trace_2_column_10_offset_0]: [QM31; 1] = (*trace_2_column_10.try_into().unwrap()).unbox();

    let [trace_2_column_11_offset_0]: [QM31; 1] = (*trace_2_column_11.try_into().unwrap()).unbox();

    let [trace_2_column_12_offset_0]: [QM31; 1] = (*trace_2_column_12.try_into().unwrap()).unbox();

    let [trace_2_column_13_offset_0]: [QM31; 1] = (*trace_2_column_13.try_into().unwrap()).unbox();

    let [trace_2_column_14_offset_0]: [QM31; 1] = (*trace_2_column_14.try_into().unwrap()).unbox();

    let [trace_2_column_15_offset_0]: [QM31; 1] = (*trace_2_column_15.try_into().unwrap()).unbox();

    let [trace_2_column_16_offset_0]: [QM31; 1] = (*trace_2_column_16.try_into().unwrap()).unbox();

    let [trace_2_column_17_offset_neg_1, trace_2_column_17_offset_0]: [QM31; 2] =
        (*trace_2_column_17
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_18_offset_neg_1, trace_2_column_18_offset_0]: [QM31; 2] =
        (*trace_2_column_18
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_19_offset_neg_1, trace_2_column_19_offset_0]: [QM31; 2] =
        (*trace_2_column_19
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_20_offset_neg_1, trace_2_column_20_offset_0]: [QM31; 2] =
        (*trace_2_column_20
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
        RangeCheck_9_9_b_alpha0,
        RangeCheck_9_9_b_alpha1,
        RangeCheck_9_9_b_z,
        RangeCheck_9_9_c_alpha0,
        RangeCheck_9_9_c_alpha1,
        RangeCheck_9_9_c_z,
        RangeCheck_9_9_d_alpha0,
        RangeCheck_9_9_d_alpha1,
        RangeCheck_9_9_d_z,
        seq,
        trace_1_column_0_offset_0,
        trace_1_column_1_offset_0,
        trace_1_column_2_offset_0,
        trace_1_column_3_offset_0,
        trace_1_column_4_offset_0,
        trace_1_column_5_offset_0,
        trace_1_column_6_offset_0,
        trace_1_column_7_offset_0,
    )
        .span();
    let intermediate0 = *intermediates.pop_front().unwrap();
    let intermediate1 = *intermediates.pop_front().unwrap();
    let intermediate2 = *intermediates.pop_front().unwrap();
    let intermediate3 = *intermediates.pop_front().unwrap();
    let intermediate4 = *intermediates.pop_front().unwrap();

    // Constraint 0
    let constraint_quotient = ((QM31Trait::from_partial_evals(
        [
            trace_2_column_9_offset_0, trace_2_column_10_offset_0, trace_2_column_11_offset_0,
            trace_2_column_12_offset_0,
        ],
    ))
        * ((intermediate0) * (intermediate1))
        - (intermediate1 + intermediate0))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 1
    let constraint_quotient = ((QM31Trait::from_partial_evals(
        [
            trace_2_column_13_offset_0, trace_2_column_14_offset_0, trace_2_column_15_offset_0,
            trace_2_column_16_offset_0,
        ],
    )
        - (QM31Trait::from_partial_evals(
            [
                trace_2_column_9_offset_0, trace_2_column_10_offset_0, trace_2_column_11_offset_0,
                trace_2_column_12_offset_0,
            ],
        )))
        * ((intermediate2) * (intermediate3))
        - (intermediate3 + intermediate2))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 2
    let constraint_quotient = ((QM31Trait::from_partial_evals(
        [
            trace_2_column_17_offset_0, trace_2_column_18_offset_0, trace_2_column_19_offset_0,
            trace_2_column_20_offset_0,
        ],
    )
        - (QM31Trait::from_partial_evals(
            [
                trace_2_column_13_offset_0, trace_2_column_14_offset_0, trace_2_column_15_offset_0,
                trace_2_column_16_offset_0,
            ],
        ))
        - (QM31Trait::from_partial_evals(
            [
                trace_2_column_17_offset_neg_1, trace_2_column_18_offset_neg_1,
                trace_2_column_19_offset_neg_1, trace_2_column_20_offset_neg_1,
            ],
        ))
        + (claimed_sum) * (column_size.inverse().into()))
        * (intermediate4)
        - (-(trace_1_column_8_offset_0)))
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
    RangeCheck_9_9_b_alpha0: QM31,
    RangeCheck_9_9_b_alpha1: QM31,
    RangeCheck_9_9_b_z: QM31,
    RangeCheck_9_9_c_alpha0: QM31,
    RangeCheck_9_9_c_alpha1: QM31,
    RangeCheck_9_9_c_z: QM31,
    RangeCheck_9_9_d_alpha0: QM31,
    RangeCheck_9_9_d_alpha1: QM31,
    RangeCheck_9_9_d_z: QM31,
    seq: QM31,
    trace_1_column_0_offset_0: QM31,
    trace_1_column_1_offset_0: QM31,
    trace_1_column_2_offset_0: QM31,
    trace_1_column_3_offset_0: QM31,
    trace_1_column_4_offset_0: QM31,
    trace_1_column_5_offset_0: QM31,
    trace_1_column_6_offset_0: QM31,
    trace_1_column_7_offset_0: QM31,
) -> Array<QM31> {
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
        seq,
        trace_1_column_0_offset_0,
        trace_1_column_1_offset_0,
        trace_1_column_2_offset_0,
        trace_1_column_3_offset_0,
        trace_1_column_4_offset_0,
        trace_1_column_5_offset_0,
        trace_1_column_6_offset_0,
        trace_1_column_7_offset_0,
    );

    let intermediate3 = intermediate3(
        RangeCheck_9_9_d_alpha0,
        RangeCheck_9_9_d_alpha1,
        RangeCheck_9_9_d_z,
        trace_1_column_6_offset_0,
        trace_1_column_7_offset_0,
    );

    let intermediate0 = intermediate0(
        RangeCheck_9_9_alpha0,
        RangeCheck_9_9_alpha1,
        RangeCheck_9_9_z,
        trace_1_column_0_offset_0,
        trace_1_column_1_offset_0,
    );

    let intermediate1 = intermediate1(
        RangeCheck_9_9_b_alpha0,
        RangeCheck_9_9_b_alpha1,
        RangeCheck_9_9_b_z,
        trace_1_column_2_offset_0,
        trace_1_column_3_offset_0,
    );

    let intermediate2 = intermediate2(
        RangeCheck_9_9_c_alpha0,
        RangeCheck_9_9_c_alpha1,
        RangeCheck_9_9_c_z,
        trace_1_column_4_offset_0,
        trace_1_column_5_offset_0,
    );
    array![intermediate0, intermediate1, intermediate2, intermediate3, intermediate4]
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
    seq: QM31,
    trace_1_column_0_offset_0: QM31,
    trace_1_column_1_offset_0: QM31,
    trace_1_column_2_offset_0: QM31,
    trace_1_column_3_offset_0: QM31,
    trace_1_column_4_offset_0: QM31,
    trace_1_column_5_offset_0: QM31,
    trace_1_column_6_offset_0: QM31,
    trace_1_column_7_offset_0: QM31,
) -> QM31 {
    (MemoryIdToBig_alpha0) * (seq)
        + (MemoryIdToBig_alpha1) * (trace_1_column_0_offset_0)
        + (MemoryIdToBig_alpha2) * (trace_1_column_1_offset_0)
        + (MemoryIdToBig_alpha3) * (trace_1_column_2_offset_0)
        + (MemoryIdToBig_alpha4) * (trace_1_column_3_offset_0)
        + (MemoryIdToBig_alpha5) * (trace_1_column_4_offset_0)
        + (MemoryIdToBig_alpha6) * (trace_1_column_5_offset_0)
        + (MemoryIdToBig_alpha7) * (trace_1_column_6_offset_0)
        + (MemoryIdToBig_alpha8) * (trace_1_column_7_offset_0)
        - (MemoryIdToBig_z)
}

pub fn intermediate3(
    RangeCheck_9_9_d_alpha0: QM31,
    RangeCheck_9_9_d_alpha1: QM31,
    RangeCheck_9_9_d_z: QM31,
    trace_1_column_6_offset_0: QM31,
    trace_1_column_7_offset_0: QM31,
) -> QM31 {
    (RangeCheck_9_9_d_alpha0) * (trace_1_column_6_offset_0)
        + (RangeCheck_9_9_d_alpha1) * (trace_1_column_7_offset_0)
        - (RangeCheck_9_9_d_z)
}

pub fn intermediate0(
    RangeCheck_9_9_alpha0: QM31,
    RangeCheck_9_9_alpha1: QM31,
    RangeCheck_9_9_z: QM31,
    trace_1_column_0_offset_0: QM31,
    trace_1_column_1_offset_0: QM31,
) -> QM31 {
    (RangeCheck_9_9_alpha0) * (trace_1_column_0_offset_0)
        + (RangeCheck_9_9_alpha1) * (trace_1_column_1_offset_0)
        - (RangeCheck_9_9_z)
}

pub fn intermediate1(
    RangeCheck_9_9_b_alpha0: QM31,
    RangeCheck_9_9_b_alpha1: QM31,
    RangeCheck_9_9_b_z: QM31,
    trace_1_column_2_offset_0: QM31,
    trace_1_column_3_offset_0: QM31,
) -> QM31 {
    (RangeCheck_9_9_b_alpha0) * (trace_1_column_2_offset_0)
        + (RangeCheck_9_9_b_alpha1) * (trace_1_column_3_offset_0)
        - (RangeCheck_9_9_b_z)
}

pub fn intermediate2(
    RangeCheck_9_9_c_alpha0: QM31,
    RangeCheck_9_9_c_alpha1: QM31,
    RangeCheck_9_9_c_z: QM31,
    trace_1_column_4_offset_0: QM31,
    trace_1_column_5_offset_0: QM31,
) -> QM31 {
    (RangeCheck_9_9_c_alpha0) * (trace_1_column_4_offset_0)
        + (RangeCheck_9_9_c_alpha1) * (trace_1_column_5_offset_0)
        - (RangeCheck_9_9_c_z)
}
