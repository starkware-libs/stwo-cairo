use crate::prelude::*;

#[derive(Drop)]
pub struct ConstraintParams {
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
    pub RangeCheck_9_9_e_alpha0: QM31,
    pub RangeCheck_9_9_e_alpha1: QM31,
    pub RangeCheck_9_9_e_z: QM31,
    pub RangeCheck_9_9_f_alpha0: QM31,
    pub RangeCheck_9_9_f_alpha1: QM31,
    pub RangeCheck_9_9_f_z: QM31,
    pub RangeCheck_9_9_g_alpha0: QM31,
    pub RangeCheck_9_9_g_alpha1: QM31,
    pub RangeCheck_9_9_g_z: QM31,
    pub RangeCheck_9_9_h_alpha0: QM31,
    pub RangeCheck_9_9_h_alpha1: QM31,
    pub RangeCheck_9_9_h_z: QM31,
    pub claimed_sum: QM31,
    pub seq: QM31,
    pub column_size: M31,
    pub offset: M31,
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
        RangeCheck_9_9_e_alpha0,
        RangeCheck_9_9_e_alpha1,
        RangeCheck_9_9_e_z,
        RangeCheck_9_9_f_alpha0,
        RangeCheck_9_9_f_alpha1,
        RangeCheck_9_9_f_z,
        RangeCheck_9_9_g_alpha0,
        RangeCheck_9_9_g_alpha1,
        RangeCheck_9_9_g_z,
        RangeCheck_9_9_h_alpha0,
        RangeCheck_9_9_h_alpha1,
        RangeCheck_9_9_h_z,
        claimed_sum,
        seq,
        column_size,
        offset,
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
        trace_1_column_17,
        trace_1_column_18,
        trace_1_column_19,
        trace_1_column_20,
        trace_1_column_21,
        trace_1_column_22,
        trace_1_column_23,
        trace_1_column_24,
        trace_1_column_25,
        trace_1_column_26,
        trace_1_column_27,
        trace_1_column_28,
    ]: [Span<QM31>; 29] =
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

    let [trace_1_column_17_offset_0]: [QM31; 1] = (*trace_1_column_17.try_into().unwrap()).unbox();

    let [trace_1_column_18_offset_0]: [QM31; 1] = (*trace_1_column_18.try_into().unwrap()).unbox();

    let [trace_1_column_19_offset_0]: [QM31; 1] = (*trace_1_column_19.try_into().unwrap()).unbox();

    let [trace_1_column_20_offset_0]: [QM31; 1] = (*trace_1_column_20.try_into().unwrap()).unbox();

    let [trace_1_column_21_offset_0]: [QM31; 1] = (*trace_1_column_21.try_into().unwrap()).unbox();

    let [trace_1_column_22_offset_0]: [QM31; 1] = (*trace_1_column_22.try_into().unwrap()).unbox();

    let [trace_1_column_23_offset_0]: [QM31; 1] = (*trace_1_column_23.try_into().unwrap()).unbox();

    let [trace_1_column_24_offset_0]: [QM31; 1] = (*trace_1_column_24.try_into().unwrap()).unbox();

    let [trace_1_column_25_offset_0]: [QM31; 1] = (*trace_1_column_25.try_into().unwrap()).unbox();

    let [trace_1_column_26_offset_0]: [QM31; 1] = (*trace_1_column_26.try_into().unwrap()).unbox();

    let [trace_1_column_27_offset_0]: [QM31; 1] = (*trace_1_column_27.try_into().unwrap()).unbox();

    let [trace_1_column_28_offset_0]: [QM31; 1] = (*trace_1_column_28.try_into().unwrap()).unbox();

    let [
        trace_2_column_29,
        trace_2_column_30,
        trace_2_column_31,
        trace_2_column_32,
        trace_2_column_33,
        trace_2_column_34,
        trace_2_column_35,
        trace_2_column_36,
        trace_2_column_37,
        trace_2_column_38,
        trace_2_column_39,
        trace_2_column_40,
        trace_2_column_41,
        trace_2_column_42,
        trace_2_column_43,
        trace_2_column_44,
        trace_2_column_45,
        trace_2_column_46,
        trace_2_column_47,
        trace_2_column_48,
        trace_2_column_49,
        trace_2_column_50,
        trace_2_column_51,
        trace_2_column_52,
        trace_2_column_53,
        trace_2_column_54,
        trace_2_column_55,
        trace_2_column_56,
        trace_2_column_57,
        trace_2_column_58,
        trace_2_column_59,
        trace_2_column_60,
    ]: [Span<QM31>; 32] =
        (*interaction_mask_values
        .multi_pop_front()
        .unwrap())
        .unbox();

    let [trace_2_column_29_offset_0]: [QM31; 1] = (*trace_2_column_29.try_into().unwrap()).unbox();

    let [trace_2_column_30_offset_0]: [QM31; 1] = (*trace_2_column_30.try_into().unwrap()).unbox();

    let [trace_2_column_31_offset_0]: [QM31; 1] = (*trace_2_column_31.try_into().unwrap()).unbox();

    let [trace_2_column_32_offset_0]: [QM31; 1] = (*trace_2_column_32.try_into().unwrap()).unbox();

    let [trace_2_column_33_offset_0]: [QM31; 1] = (*trace_2_column_33.try_into().unwrap()).unbox();

    let [trace_2_column_34_offset_0]: [QM31; 1] = (*trace_2_column_34.try_into().unwrap()).unbox();

    let [trace_2_column_35_offset_0]: [QM31; 1] = (*trace_2_column_35.try_into().unwrap()).unbox();

    let [trace_2_column_36_offset_0]: [QM31; 1] = (*trace_2_column_36.try_into().unwrap()).unbox();

    let [trace_2_column_37_offset_0]: [QM31; 1] = (*trace_2_column_37.try_into().unwrap()).unbox();

    let [trace_2_column_38_offset_0]: [QM31; 1] = (*trace_2_column_38.try_into().unwrap()).unbox();

    let [trace_2_column_39_offset_0]: [QM31; 1] = (*trace_2_column_39.try_into().unwrap()).unbox();

    let [trace_2_column_40_offset_0]: [QM31; 1] = (*trace_2_column_40.try_into().unwrap()).unbox();

    let [trace_2_column_41_offset_0]: [QM31; 1] = (*trace_2_column_41.try_into().unwrap()).unbox();

    let [trace_2_column_42_offset_0]: [QM31; 1] = (*trace_2_column_42.try_into().unwrap()).unbox();

    let [trace_2_column_43_offset_0]: [QM31; 1] = (*trace_2_column_43.try_into().unwrap()).unbox();

    let [trace_2_column_44_offset_0]: [QM31; 1] = (*trace_2_column_44.try_into().unwrap()).unbox();

    let [trace_2_column_45_offset_0]: [QM31; 1] = (*trace_2_column_45.try_into().unwrap()).unbox();

    let [trace_2_column_46_offset_0]: [QM31; 1] = (*trace_2_column_46.try_into().unwrap()).unbox();

    let [trace_2_column_47_offset_0]: [QM31; 1] = (*trace_2_column_47.try_into().unwrap()).unbox();

    let [trace_2_column_48_offset_0]: [QM31; 1] = (*trace_2_column_48.try_into().unwrap()).unbox();

    let [trace_2_column_49_offset_0]: [QM31; 1] = (*trace_2_column_49.try_into().unwrap()).unbox();

    let [trace_2_column_50_offset_0]: [QM31; 1] = (*trace_2_column_50.try_into().unwrap()).unbox();

    let [trace_2_column_51_offset_0]: [QM31; 1] = (*trace_2_column_51.try_into().unwrap()).unbox();

    let [trace_2_column_52_offset_0]: [QM31; 1] = (*trace_2_column_52.try_into().unwrap()).unbox();

    let [trace_2_column_53_offset_0]: [QM31; 1] = (*trace_2_column_53.try_into().unwrap()).unbox();

    let [trace_2_column_54_offset_0]: [QM31; 1] = (*trace_2_column_54.try_into().unwrap()).unbox();

    let [trace_2_column_55_offset_0]: [QM31; 1] = (*trace_2_column_55.try_into().unwrap()).unbox();

    let [trace_2_column_56_offset_0]: [QM31; 1] = (*trace_2_column_56.try_into().unwrap()).unbox();

    let [trace_2_column_57_offset_neg_1, trace_2_column_57_offset_0]: [QM31; 2] =
        (*trace_2_column_57
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_58_offset_neg_1, trace_2_column_58_offset_0]: [QM31; 2] =
        (*trace_2_column_58
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_59_offset_neg_1, trace_2_column_59_offset_0]: [QM31; 2] =
        (*trace_2_column_59
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_60_offset_neg_1, trace_2_column_60_offset_0]: [QM31; 2] =
        (*trace_2_column_60
        .try_into()
        .unwrap())
        .unbox();

    core::internal::revoke_ap_tracking();

    let mut intermediates = intermediates(
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
        RangeCheck_9_9_e_alpha0,
        RangeCheck_9_9_e_alpha1,
        RangeCheck_9_9_e_z,
        RangeCheck_9_9_f_alpha0,
        RangeCheck_9_9_f_alpha1,
        RangeCheck_9_9_f_z,
        RangeCheck_9_9_g_alpha0,
        RangeCheck_9_9_g_alpha1,
        RangeCheck_9_9_g_z,
        RangeCheck_9_9_h_alpha0,
        RangeCheck_9_9_h_alpha1,
        RangeCheck_9_9_h_z,
        seq,
        offset.into(),
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
    let intermediate5 = *intermediates.pop_front().unwrap();
    let intermediate6 = *intermediates.pop_front().unwrap();
    let intermediate7 = *intermediates.pop_front().unwrap();
    let intermediate8 = *intermediates.pop_front().unwrap();
    let intermediate9 = *intermediates.pop_front().unwrap();
    let intermediate10 = *intermediates.pop_front().unwrap();
    let intermediate11 = *intermediates.pop_front().unwrap();
    let intermediate12 = *intermediates.pop_front().unwrap();
    let intermediate13 = *intermediates.pop_front().unwrap();
    let intermediate14 = *intermediates.pop_front().unwrap();

    // Constraint 0
    let constraint_quotient = ((QM31Trait::from_partial_evals(
        [
            trace_2_column_29_offset_0, trace_2_column_30_offset_0, trace_2_column_31_offset_0,
            trace_2_column_32_offset_0,
        ],
    ))
        * ((intermediate0) * (intermediate1))
        - (intermediate1 + intermediate0))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 1
    let constraint_quotient = ((QM31Trait::from_partial_evals(
        [
            trace_2_column_33_offset_0, trace_2_column_34_offset_0, trace_2_column_35_offset_0,
            trace_2_column_36_offset_0,
        ],
    )
        - (QM31Trait::from_partial_evals(
            [
                trace_2_column_29_offset_0, trace_2_column_30_offset_0, trace_2_column_31_offset_0,
                trace_2_column_32_offset_0,
            ],
        )))
        * ((intermediate2) * (intermediate3))
        - (intermediate3 + intermediate2))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 2
    let constraint_quotient = ((QM31Trait::from_partial_evals(
        [
            trace_2_column_37_offset_0, trace_2_column_38_offset_0, trace_2_column_39_offset_0,
            trace_2_column_40_offset_0,
        ],
    )
        - (QM31Trait::from_partial_evals(
            [
                trace_2_column_33_offset_0, trace_2_column_34_offset_0, trace_2_column_35_offset_0,
                trace_2_column_36_offset_0,
            ],
        )))
        * ((intermediate4) * (intermediate5))
        - (intermediate5 + intermediate4))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 3
    let constraint_quotient = ((QM31Trait::from_partial_evals(
        [
            trace_2_column_41_offset_0, trace_2_column_42_offset_0, trace_2_column_43_offset_0,
            trace_2_column_44_offset_0,
        ],
    )
        - (QM31Trait::from_partial_evals(
            [
                trace_2_column_37_offset_0, trace_2_column_38_offset_0, trace_2_column_39_offset_0,
                trace_2_column_40_offset_0,
            ],
        )))
        * ((intermediate6) * (intermediate7))
        - (intermediate7 + intermediate6))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 4
    let constraint_quotient = ((QM31Trait::from_partial_evals(
        [
            trace_2_column_45_offset_0, trace_2_column_46_offset_0, trace_2_column_47_offset_0,
            trace_2_column_48_offset_0,
        ],
    )
        - (QM31Trait::from_partial_evals(
            [
                trace_2_column_41_offset_0, trace_2_column_42_offset_0, trace_2_column_43_offset_0,
                trace_2_column_44_offset_0,
            ],
        )))
        * ((intermediate8) * (intermediate9))
        - (intermediate9 + intermediate8))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 5
    let constraint_quotient = ((QM31Trait::from_partial_evals(
        [
            trace_2_column_49_offset_0, trace_2_column_50_offset_0, trace_2_column_51_offset_0,
            trace_2_column_52_offset_0,
        ],
    )
        - (QM31Trait::from_partial_evals(
            [
                trace_2_column_45_offset_0, trace_2_column_46_offset_0, trace_2_column_47_offset_0,
                trace_2_column_48_offset_0,
            ],
        )))
        * ((intermediate10) * (intermediate11))
        - (intermediate11 + intermediate10))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 6
    let constraint_quotient = ((QM31Trait::from_partial_evals(
        [
            trace_2_column_53_offset_0, trace_2_column_54_offset_0, trace_2_column_55_offset_0,
            trace_2_column_56_offset_0,
        ],
    )
        - (QM31Trait::from_partial_evals(
            [
                trace_2_column_49_offset_0, trace_2_column_50_offset_0, trace_2_column_51_offset_0,
                trace_2_column_52_offset_0,
            ],
        )))
        * ((intermediate12) * (intermediate13))
        - (intermediate13 + intermediate12))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 7
    let constraint_quotient = ((QM31Trait::from_partial_evals(
        [
            trace_2_column_57_offset_0, trace_2_column_58_offset_0, trace_2_column_59_offset_0,
            trace_2_column_60_offset_0,
        ],
    )
        - (QM31Trait::from_partial_evals(
            [
                trace_2_column_57_offset_neg_1, trace_2_column_58_offset_neg_1,
                trace_2_column_59_offset_neg_1, trace_2_column_60_offset_neg_1,
            ],
        ))
        - (QM31Trait::from_partial_evals(
            [
                trace_2_column_53_offset_0, trace_2_column_54_offset_0, trace_2_column_55_offset_0,
                trace_2_column_56_offset_0,
            ],
        ))
        + (claimed_sum) * (column_size.inverse().into()))
        * (intermediate14)
        - (-(trace_1_column_28_offset_0)))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;
}


fn intermediates(
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
    MemoryIdToBig_alpha23: QM31,
    MemoryIdToBig_alpha24: QM31,
    MemoryIdToBig_alpha25: QM31,
    MemoryIdToBig_alpha26: QM31,
    MemoryIdToBig_alpha27: QM31,
    MemoryIdToBig_alpha28: QM31,
    MemoryIdToBig_alpha3: QM31,
    MemoryIdToBig_alpha4: QM31,
    MemoryIdToBig_alpha5: QM31,
    MemoryIdToBig_alpha6: QM31,
    MemoryIdToBig_alpha7: QM31,
    MemoryIdToBig_alpha8: QM31,
    MemoryIdToBig_alpha9: QM31,
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
    RangeCheck_9_9_e_alpha0: QM31,
    RangeCheck_9_9_e_alpha1: QM31,
    RangeCheck_9_9_e_z: QM31,
    RangeCheck_9_9_f_alpha0: QM31,
    RangeCheck_9_9_f_alpha1: QM31,
    RangeCheck_9_9_f_z: QM31,
    RangeCheck_9_9_g_alpha0: QM31,
    RangeCheck_9_9_g_alpha1: QM31,
    RangeCheck_9_9_g_z: QM31,
    RangeCheck_9_9_h_alpha0: QM31,
    RangeCheck_9_9_h_alpha1: QM31,
    RangeCheck_9_9_h_z: QM31,
    seq: QM31,
    offset: QM31,
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
    let intermediate1 = intermediate1(
        RangeCheck_9_9_b_alpha0,
        RangeCheck_9_9_b_alpha1,
        RangeCheck_9_9_b_z,
        trace_1_column_2_offset_0,
        trace_1_column_3_offset_0,
    );

    let intermediate9 = intermediate9(
        RangeCheck_9_9_b_alpha0,
        RangeCheck_9_9_b_alpha1,
        RangeCheck_9_9_b_z,
        trace_1_column_18_offset_0,
        trace_1_column_19_offset_0,
    );

    let intermediate10 = intermediate10(
        RangeCheck_9_9_c_alpha0,
        RangeCheck_9_9_c_alpha1,
        RangeCheck_9_9_c_z,
        trace_1_column_20_offset_0,
        trace_1_column_21_offset_0,
    );

    let intermediate14 = intermediate14(
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
        seq,
        offset,
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
    );

    let intermediate12 = intermediate12(
        RangeCheck_9_9_e_alpha0,
        RangeCheck_9_9_e_alpha1,
        RangeCheck_9_9_e_z,
        trace_1_column_24_offset_0,
        trace_1_column_25_offset_0,
    );

    let intermediate11 = intermediate11(
        RangeCheck_9_9_d_alpha0,
        RangeCheck_9_9_d_alpha1,
        RangeCheck_9_9_d_z,
        trace_1_column_22_offset_0,
        trace_1_column_23_offset_0,
    );

    let intermediate6 = intermediate6(
        RangeCheck_9_9_g_alpha0,
        RangeCheck_9_9_g_alpha1,
        RangeCheck_9_9_g_z,
        trace_1_column_12_offset_0,
        trace_1_column_13_offset_0,
    );

    let intermediate5 = intermediate5(
        RangeCheck_9_9_f_alpha0,
        RangeCheck_9_9_f_alpha1,
        RangeCheck_9_9_f_z,
        trace_1_column_10_offset_0,
        trace_1_column_11_offset_0,
    );

    let intermediate4 = intermediate4(
        RangeCheck_9_9_e_alpha0,
        RangeCheck_9_9_e_alpha1,
        RangeCheck_9_9_e_z,
        trace_1_column_8_offset_0,
        trace_1_column_9_offset_0,
    );

    let intermediate7 = intermediate7(
        RangeCheck_9_9_h_alpha0,
        RangeCheck_9_9_h_alpha1,
        RangeCheck_9_9_h_z,
        trace_1_column_14_offset_0,
        trace_1_column_15_offset_0,
    );

    let intermediate0 = intermediate0(
        RangeCheck_9_9_alpha0,
        RangeCheck_9_9_alpha1,
        RangeCheck_9_9_z,
        trace_1_column_0_offset_0,
        trace_1_column_1_offset_0,
    );

    let intermediate13 = intermediate13(
        RangeCheck_9_9_f_alpha0,
        RangeCheck_9_9_f_alpha1,
        RangeCheck_9_9_f_z,
        trace_1_column_26_offset_0,
        trace_1_column_27_offset_0,
    );

    let intermediate8 = intermediate8(
        RangeCheck_9_9_alpha0,
        RangeCheck_9_9_alpha1,
        RangeCheck_9_9_z,
        trace_1_column_16_offset_0,
        trace_1_column_17_offset_0,
    );

    let intermediate3 = intermediate3(
        RangeCheck_9_9_d_alpha0,
        RangeCheck_9_9_d_alpha1,
        RangeCheck_9_9_d_z,
        trace_1_column_6_offset_0,
        trace_1_column_7_offset_0,
    );

    let intermediate2 = intermediate2(
        RangeCheck_9_9_c_alpha0,
        RangeCheck_9_9_c_alpha1,
        RangeCheck_9_9_c_z,
        trace_1_column_4_offset_0,
        trace_1_column_5_offset_0,
    );
    array![
        intermediate0, intermediate1, intermediate2, intermediate3, intermediate4, intermediate5,
        intermediate6, intermediate7, intermediate8, intermediate9, intermediate10, intermediate11,
        intermediate12, intermediate13, intermediate14,
    ]
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

pub fn intermediate9(
    RangeCheck_9_9_b_alpha0: QM31,
    RangeCheck_9_9_b_alpha1: QM31,
    RangeCheck_9_9_b_z: QM31,
    trace_1_column_18_offset_0: QM31,
    trace_1_column_19_offset_0: QM31,
) -> QM31 {
    (RangeCheck_9_9_b_alpha0) * (trace_1_column_18_offset_0)
        + (RangeCheck_9_9_b_alpha1) * (trace_1_column_19_offset_0)
        - (RangeCheck_9_9_b_z)
}

pub fn intermediate10(
    RangeCheck_9_9_c_alpha0: QM31,
    RangeCheck_9_9_c_alpha1: QM31,
    RangeCheck_9_9_c_z: QM31,
    trace_1_column_20_offset_0: QM31,
    trace_1_column_21_offset_0: QM31,
) -> QM31 {
    (RangeCheck_9_9_c_alpha0) * (trace_1_column_20_offset_0)
        + (RangeCheck_9_9_c_alpha1) * (trace_1_column_21_offset_0)
        - (RangeCheck_9_9_c_z)
}

pub fn intermediate14(
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
    MemoryIdToBig_alpha23: QM31,
    MemoryIdToBig_alpha24: QM31,
    MemoryIdToBig_alpha25: QM31,
    MemoryIdToBig_alpha26: QM31,
    MemoryIdToBig_alpha27: QM31,
    MemoryIdToBig_alpha28: QM31,
    MemoryIdToBig_alpha3: QM31,
    MemoryIdToBig_alpha4: QM31,
    MemoryIdToBig_alpha5: QM31,
    MemoryIdToBig_alpha6: QM31,
    MemoryIdToBig_alpha7: QM31,
    MemoryIdToBig_alpha8: QM31,
    MemoryIdToBig_alpha9: QM31,
    MemoryIdToBig_z: QM31,
    seq: QM31,
    offset: QM31,
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
) -> QM31 {
    (MemoryIdToBig_alpha0) * (seq + offset)
        + (MemoryIdToBig_alpha1) * (trace_1_column_0_offset_0)
        + (MemoryIdToBig_alpha2) * (trace_1_column_1_offset_0)
        + (MemoryIdToBig_alpha3) * (trace_1_column_2_offset_0)
        + (MemoryIdToBig_alpha4) * (trace_1_column_3_offset_0)
        + (MemoryIdToBig_alpha5) * (trace_1_column_4_offset_0)
        + (MemoryIdToBig_alpha6) * (trace_1_column_5_offset_0)
        + (MemoryIdToBig_alpha7) * (trace_1_column_6_offset_0)
        + (MemoryIdToBig_alpha8) * (trace_1_column_7_offset_0)
        + (MemoryIdToBig_alpha9) * (trace_1_column_8_offset_0)
        + (MemoryIdToBig_alpha10) * (trace_1_column_9_offset_0)
        + (MemoryIdToBig_alpha11) * (trace_1_column_10_offset_0)
        + (MemoryIdToBig_alpha12) * (trace_1_column_11_offset_0)
        + (MemoryIdToBig_alpha13) * (trace_1_column_12_offset_0)
        + (MemoryIdToBig_alpha14) * (trace_1_column_13_offset_0)
        + (MemoryIdToBig_alpha15) * (trace_1_column_14_offset_0)
        + (MemoryIdToBig_alpha16) * (trace_1_column_15_offset_0)
        + (MemoryIdToBig_alpha17) * (trace_1_column_16_offset_0)
        + (MemoryIdToBig_alpha18) * (trace_1_column_17_offset_0)
        + (MemoryIdToBig_alpha19) * (trace_1_column_18_offset_0)
        + (MemoryIdToBig_alpha20) * (trace_1_column_19_offset_0)
        + (MemoryIdToBig_alpha21) * (trace_1_column_20_offset_0)
        + (MemoryIdToBig_alpha22) * (trace_1_column_21_offset_0)
        + (MemoryIdToBig_alpha23) * (trace_1_column_22_offset_0)
        + (MemoryIdToBig_alpha24) * (trace_1_column_23_offset_0)
        + (MemoryIdToBig_alpha25) * (trace_1_column_24_offset_0)
        + (MemoryIdToBig_alpha26) * (trace_1_column_25_offset_0)
        + (MemoryIdToBig_alpha27) * (trace_1_column_26_offset_0)
        + (MemoryIdToBig_alpha28) * (trace_1_column_27_offset_0)
        - (MemoryIdToBig_z)
}

pub fn intermediate12(
    RangeCheck_9_9_e_alpha0: QM31,
    RangeCheck_9_9_e_alpha1: QM31,
    RangeCheck_9_9_e_z: QM31,
    trace_1_column_24_offset_0: QM31,
    trace_1_column_25_offset_0: QM31,
) -> QM31 {
    (RangeCheck_9_9_e_alpha0) * (trace_1_column_24_offset_0)
        + (RangeCheck_9_9_e_alpha1) * (trace_1_column_25_offset_0)
        - (RangeCheck_9_9_e_z)
}

pub fn intermediate11(
    RangeCheck_9_9_d_alpha0: QM31,
    RangeCheck_9_9_d_alpha1: QM31,
    RangeCheck_9_9_d_z: QM31,
    trace_1_column_22_offset_0: QM31,
    trace_1_column_23_offset_0: QM31,
) -> QM31 {
    (RangeCheck_9_9_d_alpha0) * (trace_1_column_22_offset_0)
        + (RangeCheck_9_9_d_alpha1) * (trace_1_column_23_offset_0)
        - (RangeCheck_9_9_d_z)
}

pub fn intermediate6(
    RangeCheck_9_9_g_alpha0: QM31,
    RangeCheck_9_9_g_alpha1: QM31,
    RangeCheck_9_9_g_z: QM31,
    trace_1_column_12_offset_0: QM31,
    trace_1_column_13_offset_0: QM31,
) -> QM31 {
    (RangeCheck_9_9_g_alpha0) * (trace_1_column_12_offset_0)
        + (RangeCheck_9_9_g_alpha1) * (trace_1_column_13_offset_0)
        - (RangeCheck_9_9_g_z)
}

pub fn intermediate5(
    RangeCheck_9_9_f_alpha0: QM31,
    RangeCheck_9_9_f_alpha1: QM31,
    RangeCheck_9_9_f_z: QM31,
    trace_1_column_10_offset_0: QM31,
    trace_1_column_11_offset_0: QM31,
) -> QM31 {
    (RangeCheck_9_9_f_alpha0) * (trace_1_column_10_offset_0)
        + (RangeCheck_9_9_f_alpha1) * (trace_1_column_11_offset_0)
        - (RangeCheck_9_9_f_z)
}

pub fn intermediate4(
    RangeCheck_9_9_e_alpha0: QM31,
    RangeCheck_9_9_e_alpha1: QM31,
    RangeCheck_9_9_e_z: QM31,
    trace_1_column_8_offset_0: QM31,
    trace_1_column_9_offset_0: QM31,
) -> QM31 {
    (RangeCheck_9_9_e_alpha0) * (trace_1_column_8_offset_0)
        + (RangeCheck_9_9_e_alpha1) * (trace_1_column_9_offset_0)
        - (RangeCheck_9_9_e_z)
}

pub fn intermediate7(
    RangeCheck_9_9_h_alpha0: QM31,
    RangeCheck_9_9_h_alpha1: QM31,
    RangeCheck_9_9_h_z: QM31,
    trace_1_column_14_offset_0: QM31,
    trace_1_column_15_offset_0: QM31,
) -> QM31 {
    (RangeCheck_9_9_h_alpha0) * (trace_1_column_14_offset_0)
        + (RangeCheck_9_9_h_alpha1) * (trace_1_column_15_offset_0)
        - (RangeCheck_9_9_h_z)
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

pub fn intermediate13(
    RangeCheck_9_9_f_alpha0: QM31,
    RangeCheck_9_9_f_alpha1: QM31,
    RangeCheck_9_9_f_z: QM31,
    trace_1_column_26_offset_0: QM31,
    trace_1_column_27_offset_0: QM31,
) -> QM31 {
    (RangeCheck_9_9_f_alpha0) * (trace_1_column_26_offset_0)
        + (RangeCheck_9_9_f_alpha1) * (trace_1_column_27_offset_0)
        - (RangeCheck_9_9_f_z)
}

pub fn intermediate8(
    RangeCheck_9_9_alpha0: QM31,
    RangeCheck_9_9_alpha1: QM31,
    RangeCheck_9_9_z: QM31,
    trace_1_column_16_offset_0: QM31,
    trace_1_column_17_offset_0: QM31,
) -> QM31 {
    (RangeCheck_9_9_alpha0) * (trace_1_column_16_offset_0)
        + (RangeCheck_9_9_alpha1) * (trace_1_column_17_offset_0)
        - (RangeCheck_9_9_z)
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

