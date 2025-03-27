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
    interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
    interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
    interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
    interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
}

#[derive(Drop)]
pub struct ConstraintParams {
    pub BlakeG_alpha0: QM31,
    pub BlakeG_alpha1: QM31,
    pub BlakeG_alpha10: QM31,
    pub BlakeG_alpha11: QM31,
    pub BlakeG_alpha12: QM31,
    pub BlakeG_alpha13: QM31,
    pub BlakeG_alpha14: QM31,
    pub BlakeG_alpha15: QM31,
    pub BlakeG_alpha16: QM31,
    pub BlakeG_alpha17: QM31,
    pub BlakeG_alpha18: QM31,
    pub BlakeG_alpha19: QM31,
    pub BlakeG_alpha2: QM31,
    pub BlakeG_alpha3: QM31,
    pub BlakeG_alpha4: QM31,
    pub BlakeG_alpha5: QM31,
    pub BlakeG_alpha6: QM31,
    pub BlakeG_alpha7: QM31,
    pub BlakeG_alpha8: QM31,
    pub BlakeG_alpha9: QM31,
    pub BlakeG_z: QM31,
    pub VerifyBitwiseXor_12_alpha0: QM31,
    pub VerifyBitwiseXor_12_alpha1: QM31,
    pub VerifyBitwiseXor_12_alpha2: QM31,
    pub VerifyBitwiseXor_12_z: QM31,
    pub VerifyBitwiseXor_4_alpha0: QM31,
    pub VerifyBitwiseXor_4_alpha1: QM31,
    pub VerifyBitwiseXor_4_alpha2: QM31,
    pub VerifyBitwiseXor_4_z: QM31,
    pub VerifyBitwiseXor_7_alpha0: QM31,
    pub VerifyBitwiseXor_7_alpha1: QM31,
    pub VerifyBitwiseXor_7_alpha2: QM31,
    pub VerifyBitwiseXor_7_z: QM31,
    pub VerifyBitwiseXor_8_alpha0: QM31,
    pub VerifyBitwiseXor_8_alpha1: QM31,
    pub VerifyBitwiseXor_8_alpha2: QM31,
    pub VerifyBitwiseXor_8_z: QM31,
    pub VerifyBitwiseXor_9_alpha0: QM31,
    pub VerifyBitwiseXor_9_alpha1: QM31,
    pub VerifyBitwiseXor_9_alpha2: QM31,
    pub VerifyBitwiseXor_9_z: QM31,
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
        BlakeG_alpha0,
        BlakeG_alpha1,
        BlakeG_alpha10,
        BlakeG_alpha11,
        BlakeG_alpha12,
        BlakeG_alpha13,
        BlakeG_alpha14,
        BlakeG_alpha15,
        BlakeG_alpha16,
        BlakeG_alpha17,
        BlakeG_alpha18,
        BlakeG_alpha19,
        BlakeG_alpha2,
        BlakeG_alpha3,
        BlakeG_alpha4,
        BlakeG_alpha5,
        BlakeG_alpha6,
        BlakeG_alpha7,
        BlakeG_alpha8,
        BlakeG_alpha9,
        BlakeG_z,
        VerifyBitwiseXor_12_alpha0,
        VerifyBitwiseXor_12_alpha1,
        VerifyBitwiseXor_12_alpha2,
        VerifyBitwiseXor_12_z,
        VerifyBitwiseXor_4_alpha0,
        VerifyBitwiseXor_4_alpha1,
        VerifyBitwiseXor_4_alpha2,
        VerifyBitwiseXor_4_z,
        VerifyBitwiseXor_7_alpha0,
        VerifyBitwiseXor_7_alpha1,
        VerifyBitwiseXor_7_alpha2,
        VerifyBitwiseXor_7_z,
        VerifyBitwiseXor_8_alpha0,
        VerifyBitwiseXor_8_alpha1,
        VerifyBitwiseXor_8_alpha2,
        VerifyBitwiseXor_8_z,
        VerifyBitwiseXor_9_alpha0,
        VerifyBitwiseXor_9_alpha1,
        VerifyBitwiseXor_9_alpha2,
        VerifyBitwiseXor_9_z,
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
        trace_1_column_29,
        trace_1_column_30,
        trace_1_column_31,
        trace_1_column_32,
        trace_1_column_33,
        trace_1_column_34,
        trace_1_column_35,
        trace_1_column_36,
        trace_1_column_37,
        trace_1_column_38,
        trace_1_column_39,
        trace_1_column_40,
        trace_1_column_41,
        trace_1_column_42,
        trace_1_column_43,
        trace_1_column_44,
        trace_1_column_45,
        trace_1_column_46,
        trace_1_column_47,
        trace_1_column_48,
        trace_1_column_49,
        trace_1_column_50,
        trace_1_column_51,
    ]: [Span<QM31>; 52] =
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

    let [trace_1_column_29_offset_0]: [QM31; 1] = (*trace_1_column_29.try_into().unwrap()).unbox();

    let [trace_1_column_30_offset_0]: [QM31; 1] = (*trace_1_column_30.try_into().unwrap()).unbox();

    let [trace_1_column_31_offset_0]: [QM31; 1] = (*trace_1_column_31.try_into().unwrap()).unbox();

    let [trace_1_column_32_offset_0]: [QM31; 1] = (*trace_1_column_32.try_into().unwrap()).unbox();

    let [trace_1_column_33_offset_0]: [QM31; 1] = (*trace_1_column_33.try_into().unwrap()).unbox();

    let [trace_1_column_34_offset_0]: [QM31; 1] = (*trace_1_column_34.try_into().unwrap()).unbox();

    let [trace_1_column_35_offset_0]: [QM31; 1] = (*trace_1_column_35.try_into().unwrap()).unbox();

    let [trace_1_column_36_offset_0]: [QM31; 1] = (*trace_1_column_36.try_into().unwrap()).unbox();

    let [trace_1_column_37_offset_0]: [QM31; 1] = (*trace_1_column_37.try_into().unwrap()).unbox();

    let [trace_1_column_38_offset_0]: [QM31; 1] = (*trace_1_column_38.try_into().unwrap()).unbox();

    let [trace_1_column_39_offset_0]: [QM31; 1] = (*trace_1_column_39.try_into().unwrap()).unbox();

    let [trace_1_column_40_offset_0]: [QM31; 1] = (*trace_1_column_40.try_into().unwrap()).unbox();

    let [trace_1_column_41_offset_0]: [QM31; 1] = (*trace_1_column_41.try_into().unwrap()).unbox();

    let [trace_1_column_42_offset_0]: [QM31; 1] = (*trace_1_column_42.try_into().unwrap()).unbox();

    let [trace_1_column_43_offset_0]: [QM31; 1] = (*trace_1_column_43.try_into().unwrap()).unbox();

    let [trace_1_column_44_offset_0]: [QM31; 1] = (*trace_1_column_44.try_into().unwrap()).unbox();

    let [trace_1_column_45_offset_0]: [QM31; 1] = (*trace_1_column_45.try_into().unwrap()).unbox();

    let [trace_1_column_46_offset_0]: [QM31; 1] = (*trace_1_column_46.try_into().unwrap()).unbox();

    let [trace_1_column_47_offset_0]: [QM31; 1] = (*trace_1_column_47.try_into().unwrap()).unbox();

    let [trace_1_column_48_offset_0]: [QM31; 1] = (*trace_1_column_48.try_into().unwrap()).unbox();

    let [trace_1_column_49_offset_0]: [QM31; 1] = (*trace_1_column_49.try_into().unwrap()).unbox();

    let [trace_1_column_50_offset_0]: [QM31; 1] = (*trace_1_column_50.try_into().unwrap()).unbox();

    let [trace_1_column_51_offset_0]: [QM31; 1] = (*trace_1_column_51.try_into().unwrap()).unbox();

    let [
        trace_2_column_52,
        trace_2_column_53,
        trace_2_column_54,
        trace_2_column_55,
        trace_2_column_56,
        trace_2_column_57,
        trace_2_column_58,
        trace_2_column_59,
        trace_2_column_60,
        trace_2_column_61,
        trace_2_column_62,
        trace_2_column_63,
        trace_2_column_64,
        trace_2_column_65,
        trace_2_column_66,
        trace_2_column_67,
        trace_2_column_68,
        trace_2_column_69,
        trace_2_column_70,
        trace_2_column_71,
        trace_2_column_72,
        trace_2_column_73,
        trace_2_column_74,
        trace_2_column_75,
        trace_2_column_76,
        trace_2_column_77,
        trace_2_column_78,
        trace_2_column_79,
        trace_2_column_80,
        trace_2_column_81,
        trace_2_column_82,
        trace_2_column_83,
        trace_2_column_84,
        trace_2_column_85,
        trace_2_column_86,
        trace_2_column_87,
    ]: [Span<QM31>; 36] =
        (*interaction_mask_values
        .multi_pop_front()
        .unwrap())
        .unbox();

    let [trace_2_column_52_offset_0]: [QM31; 1] = (*trace_2_column_52.try_into().unwrap()).unbox();

    let [trace_2_column_53_offset_0]: [QM31; 1] = (*trace_2_column_53.try_into().unwrap()).unbox();

    let [trace_2_column_54_offset_0]: [QM31; 1] = (*trace_2_column_54.try_into().unwrap()).unbox();

    let [trace_2_column_55_offset_0]: [QM31; 1] = (*trace_2_column_55.try_into().unwrap()).unbox();

    let [trace_2_column_56_offset_0]: [QM31; 1] = (*trace_2_column_56.try_into().unwrap()).unbox();

    let [trace_2_column_57_offset_0]: [QM31; 1] = (*trace_2_column_57.try_into().unwrap()).unbox();

    let [trace_2_column_58_offset_0]: [QM31; 1] = (*trace_2_column_58.try_into().unwrap()).unbox();

    let [trace_2_column_59_offset_0]: [QM31; 1] = (*trace_2_column_59.try_into().unwrap()).unbox();

    let [trace_2_column_60_offset_0]: [QM31; 1] = (*trace_2_column_60.try_into().unwrap()).unbox();

    let [trace_2_column_61_offset_0]: [QM31; 1] = (*trace_2_column_61.try_into().unwrap()).unbox();

    let [trace_2_column_62_offset_0]: [QM31; 1] = (*trace_2_column_62.try_into().unwrap()).unbox();

    let [trace_2_column_63_offset_0]: [QM31; 1] = (*trace_2_column_63.try_into().unwrap()).unbox();

    let [trace_2_column_64_offset_0]: [QM31; 1] = (*trace_2_column_64.try_into().unwrap()).unbox();

    let [trace_2_column_65_offset_0]: [QM31; 1] = (*trace_2_column_65.try_into().unwrap()).unbox();

    let [trace_2_column_66_offset_0]: [QM31; 1] = (*trace_2_column_66.try_into().unwrap()).unbox();

    let [trace_2_column_67_offset_0]: [QM31; 1] = (*trace_2_column_67.try_into().unwrap()).unbox();

    let [trace_2_column_68_offset_0]: [QM31; 1] = (*trace_2_column_68.try_into().unwrap()).unbox();

    let [trace_2_column_69_offset_0]: [QM31; 1] = (*trace_2_column_69.try_into().unwrap()).unbox();

    let [trace_2_column_70_offset_0]: [QM31; 1] = (*trace_2_column_70.try_into().unwrap()).unbox();

    let [trace_2_column_71_offset_0]: [QM31; 1] = (*trace_2_column_71.try_into().unwrap()).unbox();

    let [trace_2_column_72_offset_0]: [QM31; 1] = (*trace_2_column_72.try_into().unwrap()).unbox();

    let [trace_2_column_73_offset_0]: [QM31; 1] = (*trace_2_column_73.try_into().unwrap()).unbox();

    let [trace_2_column_74_offset_0]: [QM31; 1] = (*trace_2_column_74.try_into().unwrap()).unbox();

    let [trace_2_column_75_offset_0]: [QM31; 1] = (*trace_2_column_75.try_into().unwrap()).unbox();

    let [trace_2_column_76_offset_0]: [QM31; 1] = (*trace_2_column_76.try_into().unwrap()).unbox();

    let [trace_2_column_77_offset_0]: [QM31; 1] = (*trace_2_column_77.try_into().unwrap()).unbox();

    let [trace_2_column_78_offset_0]: [QM31; 1] = (*trace_2_column_78.try_into().unwrap()).unbox();

    let [trace_2_column_79_offset_0]: [QM31; 1] = (*trace_2_column_79.try_into().unwrap()).unbox();

    let [trace_2_column_80_offset_0]: [QM31; 1] = (*trace_2_column_80.try_into().unwrap()).unbox();

    let [trace_2_column_81_offset_0]: [QM31; 1] = (*trace_2_column_81.try_into().unwrap()).unbox();

    let [trace_2_column_82_offset_0]: [QM31; 1] = (*trace_2_column_82.try_into().unwrap()).unbox();

    let [trace_2_column_83_offset_0]: [QM31; 1] = (*trace_2_column_83.try_into().unwrap()).unbox();

    let [trace_2_column_84_offset_neg_1, trace_2_column_84_offset_0]: [QM31; 2] =
        (*trace_2_column_84
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_85_offset_neg_1, trace_2_column_85_offset_0]: [QM31; 2] =
        (*trace_2_column_85
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_86_offset_neg_1, trace_2_column_86_offset_0]: [QM31; 2] =
        (*trace_2_column_86
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_87_offset_neg_1, trace_2_column_87_offset_0]: [QM31; 2] =
        (*trace_2_column_87
        .try_into()
        .unwrap())
        .unbox();

    core::internal::revoke_ap_tracking();

    let mut intermediates = intermediates(
        BlakeG_alpha0,
        BlakeG_alpha1,
        BlakeG_alpha10,
        BlakeG_alpha11,
        BlakeG_alpha12,
        BlakeG_alpha13,
        BlakeG_alpha14,
        BlakeG_alpha15,
        BlakeG_alpha16,
        BlakeG_alpha17,
        BlakeG_alpha18,
        BlakeG_alpha19,
        BlakeG_alpha2,
        BlakeG_alpha3,
        BlakeG_alpha4,
        BlakeG_alpha5,
        BlakeG_alpha6,
        BlakeG_alpha7,
        BlakeG_alpha8,
        BlakeG_alpha9,
        BlakeG_z,
        VerifyBitwiseXor_12_alpha0,
        VerifyBitwiseXor_12_alpha1,
        VerifyBitwiseXor_12_alpha2,
        VerifyBitwiseXor_12_z,
        VerifyBitwiseXor_4_alpha0,
        VerifyBitwiseXor_4_alpha1,
        VerifyBitwiseXor_4_alpha2,
        VerifyBitwiseXor_4_z,
        VerifyBitwiseXor_7_alpha0,
        VerifyBitwiseXor_7_alpha1,
        VerifyBitwiseXor_7_alpha2,
        VerifyBitwiseXor_7_z,
        VerifyBitwiseXor_8_alpha0,
        VerifyBitwiseXor_8_alpha1,
        VerifyBitwiseXor_8_alpha2,
        VerifyBitwiseXor_8_z,
        VerifyBitwiseXor_9_alpha0,
        VerifyBitwiseXor_9_alpha1,
        VerifyBitwiseXor_9_alpha2,
        VerifyBitwiseXor_9_z,
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
        trace_1_column_32_offset_0,
        trace_1_column_33_offset_0,
        trace_1_column_34_offset_0,
        trace_1_column_35_offset_0,
        trace_1_column_36_offset_0,
        trace_1_column_37_offset_0,
        trace_1_column_38_offset_0,
        trace_1_column_39_offset_0,
        trace_1_column_3_offset_0,
        trace_1_column_40_offset_0,
        trace_1_column_41_offset_0,
        trace_1_column_42_offset_0,
        trace_1_column_43_offset_0,
        trace_1_column_44_offset_0,
        trace_1_column_45_offset_0,
        trace_1_column_46_offset_0,
        trace_1_column_47_offset_0,
        trace_1_column_48_offset_0,
        trace_1_column_49_offset_0,
        trace_1_column_4_offset_0,
        trace_1_column_50_offset_0,
        trace_1_column_51_offset_0,
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
    let intermediate15 = *intermediates.pop_front().unwrap();
    let intermediate16 = *intermediates.pop_front().unwrap();
    let intermediate17 = *intermediates.pop_front().unwrap();
    let intermediate18 = *intermediates.pop_front().unwrap();
    let intermediate19 = *intermediates.pop_front().unwrap();
    let intermediate20 = *intermediates.pop_front().unwrap();
    let intermediate21 = *intermediates.pop_front().unwrap();
    let intermediate22 = *intermediates.pop_front().unwrap();
    let intermediate23 = *intermediates.pop_front().unwrap();
    let intermediate24 = *intermediates.pop_front().unwrap();
    let intermediate25 = *intermediates.pop_front().unwrap();
    let intermediate26 = *intermediates.pop_front().unwrap();
    let intermediate27 = *intermediates.pop_front().unwrap();
    let intermediate28 = *intermediates.pop_front().unwrap();
    let intermediate29 = *intermediates.pop_front().unwrap();
    let intermediate30 = *intermediates.pop_front().unwrap();
    let intermediate31 = *intermediates.pop_front().unwrap();
    let intermediate32 = *intermediates.pop_front().unwrap();

    // Constraint 0
    let constraint_quotient = (((intermediate0) * (intermediate0 - (m31(1).into())))
        * (intermediate0 - (m31(2).into())))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 1
    let constraint_quotient = (((intermediate1) * (intermediate1 - (m31(1).into())))
        * (intermediate1 - (m31(2).into())))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 2
    let constraint_quotient = (((intermediate8) * (intermediate8 - (m31(1).into())))
        * (intermediate8 - (m31(2).into())))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 3
    let constraint_quotient = (((intermediate9) * (intermediate9 - (m31(1).into())))
        * (intermediate9 - (m31(2).into())))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 4
    let constraint_quotient = (((intermediate16) * (intermediate16 - (m31(1).into())))
        * (intermediate16 - (m31(2).into())))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 5
    let constraint_quotient = (((intermediate17) * (intermediate17 - (m31(1).into())))
        * (intermediate17 - (m31(2).into())))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 6
    let constraint_quotient = (((intermediate24) * (intermediate24 - (m31(1).into())))
        * (intermediate24 - (m31(2).into())))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 7
    let constraint_quotient = (((intermediate25) * (intermediate25 - (m31(1).into())))
        * (intermediate25 - (m31(2).into())))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 8
    let constraint_quotient = ((QM31Trait::from_partial_evals(
        [
            trace_2_column_52_offset_0, trace_2_column_53_offset_0, trace_2_column_54_offset_0,
            trace_2_column_55_offset_0,
        ],
    ))
        * ((intermediate2) * (intermediate3))
        - (intermediate3 + intermediate2))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 9
    let constraint_quotient = ((QM31Trait::from_partial_evals(
        [
            trace_2_column_56_offset_0, trace_2_column_57_offset_0, trace_2_column_58_offset_0,
            trace_2_column_59_offset_0,
        ],
    )
        - (QM31Trait::from_partial_evals(
            [
                trace_2_column_52_offset_0, trace_2_column_53_offset_0, trace_2_column_54_offset_0,
                trace_2_column_55_offset_0,
            ],
        )))
        * ((intermediate4) * (intermediate5))
        - (intermediate5 + intermediate4))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 10
    let constraint_quotient = ((QM31Trait::from_partial_evals(
        [
            trace_2_column_60_offset_0, trace_2_column_61_offset_0, trace_2_column_62_offset_0,
            trace_2_column_63_offset_0,
        ],
    )
        - (QM31Trait::from_partial_evals(
            [
                trace_2_column_56_offset_0, trace_2_column_57_offset_0, trace_2_column_58_offset_0,
                trace_2_column_59_offset_0,
            ],
        )))
        * ((intermediate10) * (intermediate11))
        - (intermediate11 + intermediate10))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 11
    let constraint_quotient = ((QM31Trait::from_partial_evals(
        [
            trace_2_column_64_offset_0, trace_2_column_65_offset_0, trace_2_column_66_offset_0,
            trace_2_column_67_offset_0,
        ],
    )
        - (QM31Trait::from_partial_evals(
            [
                trace_2_column_60_offset_0, trace_2_column_61_offset_0, trace_2_column_62_offset_0,
                trace_2_column_63_offset_0,
            ],
        )))
        * ((intermediate12) * (intermediate13))
        - (intermediate13 + intermediate12))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 12
    let constraint_quotient = ((QM31Trait::from_partial_evals(
        [
            trace_2_column_68_offset_0, trace_2_column_69_offset_0, trace_2_column_70_offset_0,
            trace_2_column_71_offset_0,
        ],
    )
        - (QM31Trait::from_partial_evals(
            [
                trace_2_column_64_offset_0, trace_2_column_65_offset_0, trace_2_column_66_offset_0,
                trace_2_column_67_offset_0,
            ],
        )))
        * ((intermediate18) * (intermediate19))
        - (intermediate19 + intermediate18))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 13
    let constraint_quotient = ((QM31Trait::from_partial_evals(
        [
            trace_2_column_72_offset_0, trace_2_column_73_offset_0, trace_2_column_74_offset_0,
            trace_2_column_75_offset_0,
        ],
    )
        - (QM31Trait::from_partial_evals(
            [
                trace_2_column_68_offset_0, trace_2_column_69_offset_0, trace_2_column_70_offset_0,
                trace_2_column_71_offset_0,
            ],
        )))
        * ((intermediate20) * (intermediate21))
        - (intermediate21 + intermediate20))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 14
    let constraint_quotient = ((QM31Trait::from_partial_evals(
        [
            trace_2_column_76_offset_0, trace_2_column_77_offset_0, trace_2_column_78_offset_0,
            trace_2_column_79_offset_0,
        ],
    )
        - (QM31Trait::from_partial_evals(
            [
                trace_2_column_72_offset_0, trace_2_column_73_offset_0, trace_2_column_74_offset_0,
                trace_2_column_75_offset_0,
            ],
        )))
        * ((intermediate26) * (intermediate27))
        - (intermediate27 + intermediate26))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 15
    let constraint_quotient = ((QM31Trait::from_partial_evals(
        [
            trace_2_column_80_offset_0, trace_2_column_81_offset_0, trace_2_column_82_offset_0,
            trace_2_column_83_offset_0,
        ],
    )
        - (QM31Trait::from_partial_evals(
            [
                trace_2_column_76_offset_0, trace_2_column_77_offset_0, trace_2_column_78_offset_0,
                trace_2_column_79_offset_0,
            ],
        )))
        * ((intermediate28) * (intermediate29))
        - (intermediate29 + intermediate28))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 16
    let constraint_quotient = ((QM31Trait::from_partial_evals(
        [
            trace_2_column_84_offset_0, trace_2_column_85_offset_0, trace_2_column_86_offset_0,
            trace_2_column_87_offset_0,
        ],
    )
        - (QM31Trait::from_partial_evals(
            [
                trace_2_column_84_offset_neg_1, trace_2_column_85_offset_neg_1,
                trace_2_column_86_offset_neg_1, trace_2_column_87_offset_neg_1,
            ],
        ))
        - (QM31Trait::from_partial_evals(
            [
                trace_2_column_80_offset_0, trace_2_column_81_offset_0, trace_2_column_82_offset_0,
                trace_2_column_83_offset_0,
            ],
        ))
        + (claimed_sum) * (column_size.inverse().into()))
        * (intermediate32)
        - (qm31_const::<2147483646, 0, 0, 0>()))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;
}


fn intermediates(
    BlakeG_alpha0: QM31,
    BlakeG_alpha1: QM31,
    BlakeG_alpha10: QM31,
    BlakeG_alpha11: QM31,
    BlakeG_alpha12: QM31,
    BlakeG_alpha13: QM31,
    BlakeG_alpha14: QM31,
    BlakeG_alpha15: QM31,
    BlakeG_alpha16: QM31,
    BlakeG_alpha17: QM31,
    BlakeG_alpha18: QM31,
    BlakeG_alpha19: QM31,
    BlakeG_alpha2: QM31,
    BlakeG_alpha3: QM31,
    BlakeG_alpha4: QM31,
    BlakeG_alpha5: QM31,
    BlakeG_alpha6: QM31,
    BlakeG_alpha7: QM31,
    BlakeG_alpha8: QM31,
    BlakeG_alpha9: QM31,
    BlakeG_z: QM31,
    VerifyBitwiseXor_12_alpha0: QM31,
    VerifyBitwiseXor_12_alpha1: QM31,
    VerifyBitwiseXor_12_alpha2: QM31,
    VerifyBitwiseXor_12_z: QM31,
    VerifyBitwiseXor_4_alpha0: QM31,
    VerifyBitwiseXor_4_alpha1: QM31,
    VerifyBitwiseXor_4_alpha2: QM31,
    VerifyBitwiseXor_4_z: QM31,
    VerifyBitwiseXor_7_alpha0: QM31,
    VerifyBitwiseXor_7_alpha1: QM31,
    VerifyBitwiseXor_7_alpha2: QM31,
    VerifyBitwiseXor_7_z: QM31,
    VerifyBitwiseXor_8_alpha0: QM31,
    VerifyBitwiseXor_8_alpha1: QM31,
    VerifyBitwiseXor_8_alpha2: QM31,
    VerifyBitwiseXor_8_z: QM31,
    VerifyBitwiseXor_9_alpha0: QM31,
    VerifyBitwiseXor_9_alpha1: QM31,
    VerifyBitwiseXor_9_alpha2: QM31,
    VerifyBitwiseXor_9_z: QM31,
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
    trace_1_column_32_offset_0: QM31,
    trace_1_column_33_offset_0: QM31,
    trace_1_column_34_offset_0: QM31,
    trace_1_column_35_offset_0: QM31,
    trace_1_column_36_offset_0: QM31,
    trace_1_column_37_offset_0: QM31,
    trace_1_column_38_offset_0: QM31,
    trace_1_column_39_offset_0: QM31,
    trace_1_column_3_offset_0: QM31,
    trace_1_column_40_offset_0: QM31,
    trace_1_column_41_offset_0: QM31,
    trace_1_column_42_offset_0: QM31,
    trace_1_column_43_offset_0: QM31,
    trace_1_column_44_offset_0: QM31,
    trace_1_column_45_offset_0: QM31,
    trace_1_column_46_offset_0: QM31,
    trace_1_column_47_offset_0: QM31,
    trace_1_column_48_offset_0: QM31,
    trace_1_column_49_offset_0: QM31,
    trace_1_column_4_offset_0: QM31,
    trace_1_column_50_offset_0: QM31,
    trace_1_column_51_offset_0: QM31,
    trace_1_column_5_offset_0: QM31,
    trace_1_column_6_offset_0: QM31,
    trace_1_column_7_offset_0: QM31,
    trace_1_column_8_offset_0: QM31,
    trace_1_column_9_offset_0: QM31,
) -> Array<QM31> {
    let intermediate0 = intermediate0(
        trace_1_column_0_offset_0,
        trace_1_column_12_offset_0,
        trace_1_column_2_offset_0,
        trace_1_column_8_offset_0,
    );

    let intermediate1 = intermediate1(
        intermediate0,
        trace_1_column_13_offset_0,
        trace_1_column_1_offset_0,
        trace_1_column_3_offset_0,
        trace_1_column_9_offset_0,
    );

    let intermediate6 = intermediate6(trace_1_column_20_offset_0, trace_1_column_21_offset_0);

    let intermediate7 = intermediate7(trace_1_column_18_offset_0, trace_1_column_19_offset_0);

    let intermediate8 = intermediate8(
        intermediate6, trace_1_column_22_offset_0, trace_1_column_4_offset_0,
    );

    let intermediate9 = intermediate9(
        intermediate7, intermediate8, trace_1_column_23_offset_0, trace_1_column_5_offset_0,
    );

    let intermediate14 = intermediate14(trace_1_column_29_offset_0, trace_1_column_30_offset_0);

    let intermediate15 = intermediate15(trace_1_column_28_offset_0, trace_1_column_31_offset_0);

    let intermediate16 = intermediate16(
        intermediate14,
        trace_1_column_10_offset_0,
        trace_1_column_12_offset_0,
        trace_1_column_32_offset_0,
    );

    let intermediate17 = intermediate17(
        intermediate15,
        intermediate16,
        trace_1_column_11_offset_0,
        trace_1_column_13_offset_0,
        trace_1_column_33_offset_0,
    );

    let intermediate22 = intermediate22(trace_1_column_39_offset_0, trace_1_column_40_offset_0);

    let intermediate23 = intermediate23(trace_1_column_38_offset_0, trace_1_column_41_offset_0);

    let intermediate24 = intermediate24(
        intermediate22, trace_1_column_22_offset_0, trace_1_column_42_offset_0,
    );

    let intermediate25 = intermediate25(
        intermediate23, intermediate24, trace_1_column_23_offset_0, trace_1_column_43_offset_0,
    );

    let intermediate30 = intermediate30(trace_1_column_49_offset_0, trace_1_column_50_offset_0);

    let intermediate31 = intermediate31(trace_1_column_48_offset_0, trace_1_column_51_offset_0);
    let intermediate2 = intermediate2(
        VerifyBitwiseXor_8_alpha0,
        VerifyBitwiseXor_8_alpha1,
        VerifyBitwiseXor_8_alpha2,
        VerifyBitwiseXor_8_z,
        trace_1_column_12_offset_0,
        trace_1_column_14_offset_0,
        trace_1_column_16_offset_0,
        trace_1_column_18_offset_0,
        trace_1_column_6_offset_0,
    );

    let intermediate3 = intermediate3(
        VerifyBitwiseXor_8_alpha0,
        VerifyBitwiseXor_8_alpha1,
        VerifyBitwiseXor_8_alpha2,
        VerifyBitwiseXor_8_z,
        trace_1_column_14_offset_0,
        trace_1_column_16_offset_0,
        trace_1_column_19_offset_0,
    );

    let intermediate4 = intermediate4(
        VerifyBitwiseXor_8_alpha0,
        VerifyBitwiseXor_8_alpha1,
        VerifyBitwiseXor_8_alpha2,
        VerifyBitwiseXor_8_z,
        trace_1_column_13_offset_0,
        trace_1_column_15_offset_0,
        trace_1_column_17_offset_0,
        trace_1_column_20_offset_0,
        trace_1_column_7_offset_0,
    );

    let intermediate5 = intermediate5(
        VerifyBitwiseXor_8_alpha0,
        VerifyBitwiseXor_8_alpha1,
        VerifyBitwiseXor_8_alpha2,
        VerifyBitwiseXor_8_z,
        trace_1_column_15_offset_0,
        trace_1_column_17_offset_0,
        trace_1_column_21_offset_0,
    );

    let intermediate10 = intermediate10(
        VerifyBitwiseXor_12_alpha0,
        VerifyBitwiseXor_12_alpha1,
        VerifyBitwiseXor_12_alpha2,
        VerifyBitwiseXor_12_z,
        trace_1_column_22_offset_0,
        trace_1_column_24_offset_0,
        trace_1_column_26_offset_0,
        trace_1_column_28_offset_0,
        trace_1_column_2_offset_0,
    );

    let intermediate11 = intermediate11(
        VerifyBitwiseXor_4_alpha0,
        VerifyBitwiseXor_4_alpha1,
        VerifyBitwiseXor_4_alpha2,
        VerifyBitwiseXor_4_z,
        trace_1_column_24_offset_0,
        trace_1_column_26_offset_0,
        trace_1_column_29_offset_0,
    );

    let intermediate12 = intermediate12(
        VerifyBitwiseXor_12_alpha0,
        VerifyBitwiseXor_12_alpha1,
        VerifyBitwiseXor_12_alpha2,
        VerifyBitwiseXor_12_z,
        trace_1_column_23_offset_0,
        trace_1_column_25_offset_0,
        trace_1_column_27_offset_0,
        trace_1_column_30_offset_0,
        trace_1_column_3_offset_0,
    );

    let intermediate13 = intermediate13(
        VerifyBitwiseXor_4_alpha0,
        VerifyBitwiseXor_4_alpha1,
        VerifyBitwiseXor_4_alpha2,
        VerifyBitwiseXor_4_z,
        trace_1_column_25_offset_0,
        trace_1_column_27_offset_0,
        trace_1_column_31_offset_0,
    );

    let intermediate18 = intermediate18(
        VerifyBitwiseXor_8_alpha0,
        VerifyBitwiseXor_8_alpha1,
        VerifyBitwiseXor_8_alpha2,
        VerifyBitwiseXor_8_z,
        intermediate6,
        trace_1_column_32_offset_0,
        trace_1_column_34_offset_0,
        trace_1_column_36_offset_0,
        trace_1_column_38_offset_0,
    );

    let intermediate19 = intermediate19(
        VerifyBitwiseXor_8_alpha0,
        VerifyBitwiseXor_8_alpha1,
        VerifyBitwiseXor_8_alpha2,
        VerifyBitwiseXor_8_z,
        trace_1_column_34_offset_0,
        trace_1_column_36_offset_0,
        trace_1_column_39_offset_0,
    );

    let intermediate20 = intermediate20(
        VerifyBitwiseXor_8_alpha0,
        VerifyBitwiseXor_8_alpha1,
        VerifyBitwiseXor_8_alpha2,
        VerifyBitwiseXor_8_z,
        intermediate7,
        trace_1_column_33_offset_0,
        trace_1_column_35_offset_0,
        trace_1_column_37_offset_0,
        trace_1_column_40_offset_0,
    );

    let intermediate21 = intermediate21(
        VerifyBitwiseXor_8_alpha0,
        VerifyBitwiseXor_8_alpha1,
        VerifyBitwiseXor_8_alpha2,
        VerifyBitwiseXor_8_z,
        trace_1_column_35_offset_0,
        trace_1_column_37_offset_0,
        trace_1_column_41_offset_0,
    );

    let intermediate26 = intermediate26(
        VerifyBitwiseXor_7_alpha0,
        VerifyBitwiseXor_7_alpha1,
        VerifyBitwiseXor_7_alpha2,
        VerifyBitwiseXor_7_z,
        intermediate14,
        trace_1_column_42_offset_0,
        trace_1_column_44_offset_0,
        trace_1_column_46_offset_0,
        trace_1_column_48_offset_0,
    );

    let intermediate27 = intermediate27(
        VerifyBitwiseXor_9_alpha0,
        VerifyBitwiseXor_9_alpha1,
        VerifyBitwiseXor_9_alpha2,
        VerifyBitwiseXor_9_z,
        trace_1_column_44_offset_0,
        trace_1_column_46_offset_0,
        trace_1_column_49_offset_0,
    );

    let intermediate28 = intermediate28(
        VerifyBitwiseXor_7_alpha0,
        VerifyBitwiseXor_7_alpha1,
        VerifyBitwiseXor_7_alpha2,
        VerifyBitwiseXor_7_z,
        intermediate15,
        trace_1_column_43_offset_0,
        trace_1_column_45_offset_0,
        trace_1_column_47_offset_0,
        trace_1_column_50_offset_0,
    );

    let intermediate29 = intermediate29(
        VerifyBitwiseXor_9_alpha0,
        VerifyBitwiseXor_9_alpha1,
        VerifyBitwiseXor_9_alpha2,
        VerifyBitwiseXor_9_z,
        trace_1_column_45_offset_0,
        trace_1_column_47_offset_0,
        trace_1_column_51_offset_0,
    );

    let intermediate32 = intermediate32(
        BlakeG_alpha0,
        BlakeG_alpha1,
        BlakeG_alpha10,
        BlakeG_alpha11,
        BlakeG_alpha12,
        BlakeG_alpha13,
        BlakeG_alpha14,
        BlakeG_alpha15,
        BlakeG_alpha16,
        BlakeG_alpha17,
        BlakeG_alpha18,
        BlakeG_alpha19,
        BlakeG_alpha2,
        BlakeG_alpha3,
        BlakeG_alpha4,
        BlakeG_alpha5,
        BlakeG_alpha6,
        BlakeG_alpha7,
        BlakeG_alpha8,
        BlakeG_alpha9,
        BlakeG_z,
        intermediate22,
        intermediate23,
        intermediate30,
        intermediate31,
        trace_1_column_0_offset_0,
        trace_1_column_10_offset_0,
        trace_1_column_11_offset_0,
        trace_1_column_1_offset_0,
        trace_1_column_2_offset_0,
        trace_1_column_32_offset_0,
        trace_1_column_33_offset_0,
        trace_1_column_3_offset_0,
        trace_1_column_42_offset_0,
        trace_1_column_43_offset_0,
        trace_1_column_4_offset_0,
        trace_1_column_5_offset_0,
        trace_1_column_6_offset_0,
        trace_1_column_7_offset_0,
        trace_1_column_8_offset_0,
        trace_1_column_9_offset_0,
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
        intermediate10,
        intermediate11,
        intermediate12,
        intermediate13,
        intermediate14,
        intermediate15,
        intermediate16,
        intermediate17,
        intermediate18,
        intermediate19,
        intermediate20,
        intermediate21,
        intermediate22,
        intermediate23,
        intermediate24,
        intermediate25,
        intermediate26,
        intermediate27,
        intermediate28,
        intermediate29,
        intermediate30,
        intermediate31,
        intermediate32,
    ]
}

pub fn intermediate0(
    trace_1_column_0_offset_0: QM31,
    trace_1_column_12_offset_0: QM31,
    trace_1_column_2_offset_0: QM31,
    trace_1_column_8_offset_0: QM31,
) -> QM31 {
    (trace_1_column_0_offset_0
        + trace_1_column_2_offset_0
        + trace_1_column_8_offset_0
        - (trace_1_column_12_offset_0))
        * (m31(32768).into())
}

pub fn intermediate1(
    intermediate0: QM31,
    trace_1_column_13_offset_0: QM31,
    trace_1_column_1_offset_0: QM31,
    trace_1_column_3_offset_0: QM31,
    trace_1_column_9_offset_0: QM31,
) -> QM31 {
    (trace_1_column_1_offset_0
        + trace_1_column_3_offset_0
        + trace_1_column_9_offset_0
        + intermediate0
        - (trace_1_column_13_offset_0))
        * (m31(32768).into())
}

pub fn intermediate6(trace_1_column_20_offset_0: QM31, trace_1_column_21_offset_0: QM31) -> QM31 {
    trace_1_column_20_offset_0 + (trace_1_column_21_offset_0) * (m31(256).into())
}

pub fn intermediate7(trace_1_column_18_offset_0: QM31, trace_1_column_19_offset_0: QM31) -> QM31 {
    trace_1_column_18_offset_0 + (trace_1_column_19_offset_0) * (m31(256).into())
}

pub fn intermediate8(
    intermediate6: QM31, trace_1_column_22_offset_0: QM31, trace_1_column_4_offset_0: QM31,
) -> QM31 {
    (trace_1_column_4_offset_0 + intermediate6 - (trace_1_column_22_offset_0)) * (m31(32768).into())
}

pub fn intermediate9(
    intermediate7: QM31,
    intermediate8: QM31,
    trace_1_column_23_offset_0: QM31,
    trace_1_column_5_offset_0: QM31,
) -> QM31 {
    (trace_1_column_5_offset_0 + intermediate7 + intermediate8 - (trace_1_column_23_offset_0))
        * (m31(32768).into())
}

pub fn intermediate14(trace_1_column_29_offset_0: QM31, trace_1_column_30_offset_0: QM31) -> QM31 {
    trace_1_column_29_offset_0 + (trace_1_column_30_offset_0) * (m31(16).into())
}

pub fn intermediate15(trace_1_column_28_offset_0: QM31, trace_1_column_31_offset_0: QM31) -> QM31 {
    trace_1_column_31_offset_0 + (trace_1_column_28_offset_0) * (m31(16).into())
}

pub fn intermediate16(
    intermediate14: QM31,
    trace_1_column_10_offset_0: QM31,
    trace_1_column_12_offset_0: QM31,
    trace_1_column_32_offset_0: QM31,
) -> QM31 {
    (trace_1_column_12_offset_0
        + intermediate14
        + trace_1_column_10_offset_0
        - (trace_1_column_32_offset_0))
        * (m31(32768).into())
}

pub fn intermediate17(
    intermediate15: QM31,
    intermediate16: QM31,
    trace_1_column_11_offset_0: QM31,
    trace_1_column_13_offset_0: QM31,
    trace_1_column_33_offset_0: QM31,
) -> QM31 {
    (trace_1_column_13_offset_0
        + intermediate15
        + trace_1_column_11_offset_0
        + intermediate16
        - (trace_1_column_33_offset_0))
        * (m31(32768).into())
}

pub fn intermediate22(trace_1_column_39_offset_0: QM31, trace_1_column_40_offset_0: QM31) -> QM31 {
    trace_1_column_39_offset_0 + (trace_1_column_40_offset_0) * (m31(256).into())
}

pub fn intermediate23(trace_1_column_38_offset_0: QM31, trace_1_column_41_offset_0: QM31) -> QM31 {
    trace_1_column_41_offset_0 + (trace_1_column_38_offset_0) * (m31(256).into())
}

pub fn intermediate24(
    intermediate22: QM31, trace_1_column_22_offset_0: QM31, trace_1_column_42_offset_0: QM31,
) -> QM31 {
    (trace_1_column_22_offset_0 + intermediate22 - (trace_1_column_42_offset_0))
        * (m31(32768).into())
}

pub fn intermediate25(
    intermediate23: QM31,
    intermediate24: QM31,
    trace_1_column_23_offset_0: QM31,
    trace_1_column_43_offset_0: QM31,
) -> QM31 {
    (trace_1_column_23_offset_0 + intermediate23 + intermediate24 - (trace_1_column_43_offset_0))
        * (m31(32768).into())
}

pub fn intermediate30(trace_1_column_49_offset_0: QM31, trace_1_column_50_offset_0: QM31) -> QM31 {
    trace_1_column_49_offset_0 + (trace_1_column_50_offset_0) * (m31(512).into())
}

pub fn intermediate31(trace_1_column_48_offset_0: QM31, trace_1_column_51_offset_0: QM31) -> QM31 {
    trace_1_column_51_offset_0 + (trace_1_column_48_offset_0) * (m31(512).into())
}
pub fn intermediate2(
    VerifyBitwiseXor_8_alpha0: QM31,
    VerifyBitwiseXor_8_alpha1: QM31,
    VerifyBitwiseXor_8_alpha2: QM31,
    VerifyBitwiseXor_8_z: QM31,
    trace_1_column_12_offset_0: QM31,
    trace_1_column_14_offset_0: QM31,
    trace_1_column_16_offset_0: QM31,
    trace_1_column_18_offset_0: QM31,
    trace_1_column_6_offset_0: QM31,
) -> QM31 {
    (VerifyBitwiseXor_8_alpha0)
        * (trace_1_column_12_offset_0 - ((trace_1_column_14_offset_0) * (m31(256).into())))
        + (VerifyBitwiseXor_8_alpha1)
            * (trace_1_column_6_offset_0 - ((trace_1_column_16_offset_0) * (m31(256).into())))
        + (VerifyBitwiseXor_8_alpha2) * (trace_1_column_18_offset_0)
        - (VerifyBitwiseXor_8_z)
}

pub fn intermediate3(
    VerifyBitwiseXor_8_alpha0: QM31,
    VerifyBitwiseXor_8_alpha1: QM31,
    VerifyBitwiseXor_8_alpha2: QM31,
    VerifyBitwiseXor_8_z: QM31,
    trace_1_column_14_offset_0: QM31,
    trace_1_column_16_offset_0: QM31,
    trace_1_column_19_offset_0: QM31,
) -> QM31 {
    (VerifyBitwiseXor_8_alpha0) * (trace_1_column_14_offset_0)
        + (VerifyBitwiseXor_8_alpha1) * (trace_1_column_16_offset_0)
        + (VerifyBitwiseXor_8_alpha2) * (trace_1_column_19_offset_0)
        - (VerifyBitwiseXor_8_z)
}

pub fn intermediate4(
    VerifyBitwiseXor_8_alpha0: QM31,
    VerifyBitwiseXor_8_alpha1: QM31,
    VerifyBitwiseXor_8_alpha2: QM31,
    VerifyBitwiseXor_8_z: QM31,
    trace_1_column_13_offset_0: QM31,
    trace_1_column_15_offset_0: QM31,
    trace_1_column_17_offset_0: QM31,
    trace_1_column_20_offset_0: QM31,
    trace_1_column_7_offset_0: QM31,
) -> QM31 {
    (VerifyBitwiseXor_8_alpha0)
        * (trace_1_column_13_offset_0 - ((trace_1_column_15_offset_0) * (m31(256).into())))
        + (VerifyBitwiseXor_8_alpha1)
            * (trace_1_column_7_offset_0 - ((trace_1_column_17_offset_0) * (m31(256).into())))
        + (VerifyBitwiseXor_8_alpha2) * (trace_1_column_20_offset_0)
        - (VerifyBitwiseXor_8_z)
}

pub fn intermediate5(
    VerifyBitwiseXor_8_alpha0: QM31,
    VerifyBitwiseXor_8_alpha1: QM31,
    VerifyBitwiseXor_8_alpha2: QM31,
    VerifyBitwiseXor_8_z: QM31,
    trace_1_column_15_offset_0: QM31,
    trace_1_column_17_offset_0: QM31,
    trace_1_column_21_offset_0: QM31,
) -> QM31 {
    (VerifyBitwiseXor_8_alpha0) * (trace_1_column_15_offset_0)
        + (VerifyBitwiseXor_8_alpha1) * (trace_1_column_17_offset_0)
        + (VerifyBitwiseXor_8_alpha2) * (trace_1_column_21_offset_0)
        - (VerifyBitwiseXor_8_z)
}

pub fn intermediate10(
    VerifyBitwiseXor_12_alpha0: QM31,
    VerifyBitwiseXor_12_alpha1: QM31,
    VerifyBitwiseXor_12_alpha2: QM31,
    VerifyBitwiseXor_12_z: QM31,
    trace_1_column_22_offset_0: QM31,
    trace_1_column_24_offset_0: QM31,
    trace_1_column_26_offset_0: QM31,
    trace_1_column_28_offset_0: QM31,
    trace_1_column_2_offset_0: QM31,
) -> QM31 {
    (VerifyBitwiseXor_12_alpha0)
        * (trace_1_column_2_offset_0 - ((trace_1_column_24_offset_0) * (m31(4096).into())))
        + (VerifyBitwiseXor_12_alpha1)
            * (trace_1_column_22_offset_0 - ((trace_1_column_26_offset_0) * (m31(4096).into())))
        + (VerifyBitwiseXor_12_alpha2) * (trace_1_column_28_offset_0)
        - (VerifyBitwiseXor_12_z)
}

pub fn intermediate11(
    VerifyBitwiseXor_4_alpha0: QM31,
    VerifyBitwiseXor_4_alpha1: QM31,
    VerifyBitwiseXor_4_alpha2: QM31,
    VerifyBitwiseXor_4_z: QM31,
    trace_1_column_24_offset_0: QM31,
    trace_1_column_26_offset_0: QM31,
    trace_1_column_29_offset_0: QM31,
) -> QM31 {
    (VerifyBitwiseXor_4_alpha0) * (trace_1_column_24_offset_0)
        + (VerifyBitwiseXor_4_alpha1) * (trace_1_column_26_offset_0)
        + (VerifyBitwiseXor_4_alpha2) * (trace_1_column_29_offset_0)
        - (VerifyBitwiseXor_4_z)
}

pub fn intermediate12(
    VerifyBitwiseXor_12_alpha0: QM31,
    VerifyBitwiseXor_12_alpha1: QM31,
    VerifyBitwiseXor_12_alpha2: QM31,
    VerifyBitwiseXor_12_z: QM31,
    trace_1_column_23_offset_0: QM31,
    trace_1_column_25_offset_0: QM31,
    trace_1_column_27_offset_0: QM31,
    trace_1_column_30_offset_0: QM31,
    trace_1_column_3_offset_0: QM31,
) -> QM31 {
    (VerifyBitwiseXor_12_alpha0)
        * (trace_1_column_3_offset_0 - ((trace_1_column_25_offset_0) * (m31(4096).into())))
        + (VerifyBitwiseXor_12_alpha1)
            * (trace_1_column_23_offset_0 - ((trace_1_column_27_offset_0) * (m31(4096).into())))
        + (VerifyBitwiseXor_12_alpha2) * (trace_1_column_30_offset_0)
        - (VerifyBitwiseXor_12_z)
}

pub fn intermediate13(
    VerifyBitwiseXor_4_alpha0: QM31,
    VerifyBitwiseXor_4_alpha1: QM31,
    VerifyBitwiseXor_4_alpha2: QM31,
    VerifyBitwiseXor_4_z: QM31,
    trace_1_column_25_offset_0: QM31,
    trace_1_column_27_offset_0: QM31,
    trace_1_column_31_offset_0: QM31,
) -> QM31 {
    (VerifyBitwiseXor_4_alpha0) * (trace_1_column_25_offset_0)
        + (VerifyBitwiseXor_4_alpha1) * (trace_1_column_27_offset_0)
        + (VerifyBitwiseXor_4_alpha2) * (trace_1_column_31_offset_0)
        - (VerifyBitwiseXor_4_z)
}

pub fn intermediate18(
    VerifyBitwiseXor_8_alpha0: QM31,
    VerifyBitwiseXor_8_alpha1: QM31,
    VerifyBitwiseXor_8_alpha2: QM31,
    VerifyBitwiseXor_8_z: QM31,
    intermediate6: QM31,
    trace_1_column_32_offset_0: QM31,
    trace_1_column_34_offset_0: QM31,
    trace_1_column_36_offset_0: QM31,
    trace_1_column_38_offset_0: QM31,
) -> QM31 {
    (VerifyBitwiseXor_8_alpha0)
        * (trace_1_column_32_offset_0 - ((trace_1_column_34_offset_0) * (m31(256).into())))
        + (VerifyBitwiseXor_8_alpha1)
            * (intermediate6 - ((trace_1_column_36_offset_0) * (m31(256).into())))
        + (VerifyBitwiseXor_8_alpha2) * (trace_1_column_38_offset_0)
        - (VerifyBitwiseXor_8_z)
}

pub fn intermediate19(
    VerifyBitwiseXor_8_alpha0: QM31,
    VerifyBitwiseXor_8_alpha1: QM31,
    VerifyBitwiseXor_8_alpha2: QM31,
    VerifyBitwiseXor_8_z: QM31,
    trace_1_column_34_offset_0: QM31,
    trace_1_column_36_offset_0: QM31,
    trace_1_column_39_offset_0: QM31,
) -> QM31 {
    (VerifyBitwiseXor_8_alpha0) * (trace_1_column_34_offset_0)
        + (VerifyBitwiseXor_8_alpha1) * (trace_1_column_36_offset_0)
        + (VerifyBitwiseXor_8_alpha2) * (trace_1_column_39_offset_0)
        - (VerifyBitwiseXor_8_z)
}

pub fn intermediate20(
    VerifyBitwiseXor_8_alpha0: QM31,
    VerifyBitwiseXor_8_alpha1: QM31,
    VerifyBitwiseXor_8_alpha2: QM31,
    VerifyBitwiseXor_8_z: QM31,
    intermediate7: QM31,
    trace_1_column_33_offset_0: QM31,
    trace_1_column_35_offset_0: QM31,
    trace_1_column_37_offset_0: QM31,
    trace_1_column_40_offset_0: QM31,
) -> QM31 {
    (VerifyBitwiseXor_8_alpha0)
        * (trace_1_column_33_offset_0 - ((trace_1_column_35_offset_0) * (m31(256).into())))
        + (VerifyBitwiseXor_8_alpha1)
            * (intermediate7 - ((trace_1_column_37_offset_0) * (m31(256).into())))
        + (VerifyBitwiseXor_8_alpha2) * (trace_1_column_40_offset_0)
        - (VerifyBitwiseXor_8_z)
}

pub fn intermediate21(
    VerifyBitwiseXor_8_alpha0: QM31,
    VerifyBitwiseXor_8_alpha1: QM31,
    VerifyBitwiseXor_8_alpha2: QM31,
    VerifyBitwiseXor_8_z: QM31,
    trace_1_column_35_offset_0: QM31,
    trace_1_column_37_offset_0: QM31,
    trace_1_column_41_offset_0: QM31,
) -> QM31 {
    (VerifyBitwiseXor_8_alpha0) * (trace_1_column_35_offset_0)
        + (VerifyBitwiseXor_8_alpha1) * (trace_1_column_37_offset_0)
        + (VerifyBitwiseXor_8_alpha2) * (trace_1_column_41_offset_0)
        - (VerifyBitwiseXor_8_z)
}

pub fn intermediate26(
    VerifyBitwiseXor_7_alpha0: QM31,
    VerifyBitwiseXor_7_alpha1: QM31,
    VerifyBitwiseXor_7_alpha2: QM31,
    VerifyBitwiseXor_7_z: QM31,
    intermediate14: QM31,
    trace_1_column_42_offset_0: QM31,
    trace_1_column_44_offset_0: QM31,
    trace_1_column_46_offset_0: QM31,
    trace_1_column_48_offset_0: QM31,
) -> QM31 {
    (VerifyBitwiseXor_7_alpha0)
        * (intermediate14 - ((trace_1_column_44_offset_0) * (m31(128).into())))
        + (VerifyBitwiseXor_7_alpha1)
            * (trace_1_column_42_offset_0 - ((trace_1_column_46_offset_0) * (m31(128).into())))
        + (VerifyBitwiseXor_7_alpha2) * (trace_1_column_48_offset_0)
        - (VerifyBitwiseXor_7_z)
}

pub fn intermediate27(
    VerifyBitwiseXor_9_alpha0: QM31,
    VerifyBitwiseXor_9_alpha1: QM31,
    VerifyBitwiseXor_9_alpha2: QM31,
    VerifyBitwiseXor_9_z: QM31,
    trace_1_column_44_offset_0: QM31,
    trace_1_column_46_offset_0: QM31,
    trace_1_column_49_offset_0: QM31,
) -> QM31 {
    (VerifyBitwiseXor_9_alpha0) * (trace_1_column_44_offset_0)
        + (VerifyBitwiseXor_9_alpha1) * (trace_1_column_46_offset_0)
        + (VerifyBitwiseXor_9_alpha2) * (trace_1_column_49_offset_0)
        - (VerifyBitwiseXor_9_z)
}

pub fn intermediate28(
    VerifyBitwiseXor_7_alpha0: QM31,
    VerifyBitwiseXor_7_alpha1: QM31,
    VerifyBitwiseXor_7_alpha2: QM31,
    VerifyBitwiseXor_7_z: QM31,
    intermediate15: QM31,
    trace_1_column_43_offset_0: QM31,
    trace_1_column_45_offset_0: QM31,
    trace_1_column_47_offset_0: QM31,
    trace_1_column_50_offset_0: QM31,
) -> QM31 {
    (VerifyBitwiseXor_7_alpha0)
        * (intermediate15 - ((trace_1_column_45_offset_0) * (m31(128).into())))
        + (VerifyBitwiseXor_7_alpha1)
            * (trace_1_column_43_offset_0 - ((trace_1_column_47_offset_0) * (m31(128).into())))
        + (VerifyBitwiseXor_7_alpha2) * (trace_1_column_50_offset_0)
        - (VerifyBitwiseXor_7_z)
}

pub fn intermediate29(
    VerifyBitwiseXor_9_alpha0: QM31,
    VerifyBitwiseXor_9_alpha1: QM31,
    VerifyBitwiseXor_9_alpha2: QM31,
    VerifyBitwiseXor_9_z: QM31,
    trace_1_column_45_offset_0: QM31,
    trace_1_column_47_offset_0: QM31,
    trace_1_column_51_offset_0: QM31,
) -> QM31 {
    (VerifyBitwiseXor_9_alpha0) * (trace_1_column_45_offset_0)
        + (VerifyBitwiseXor_9_alpha1) * (trace_1_column_47_offset_0)
        + (VerifyBitwiseXor_9_alpha2) * (trace_1_column_51_offset_0)
        - (VerifyBitwiseXor_9_z)
}

pub fn intermediate32(
    BlakeG_alpha0: QM31,
    BlakeG_alpha1: QM31,
    BlakeG_alpha10: QM31,
    BlakeG_alpha11: QM31,
    BlakeG_alpha12: QM31,
    BlakeG_alpha13: QM31,
    BlakeG_alpha14: QM31,
    BlakeG_alpha15: QM31,
    BlakeG_alpha16: QM31,
    BlakeG_alpha17: QM31,
    BlakeG_alpha18: QM31,
    BlakeG_alpha19: QM31,
    BlakeG_alpha2: QM31,
    BlakeG_alpha3: QM31,
    BlakeG_alpha4: QM31,
    BlakeG_alpha5: QM31,
    BlakeG_alpha6: QM31,
    BlakeG_alpha7: QM31,
    BlakeG_alpha8: QM31,
    BlakeG_alpha9: QM31,
    BlakeG_z: QM31,
    intermediate22: QM31,
    intermediate23: QM31,
    intermediate30: QM31,
    intermediate31: QM31,
    trace_1_column_0_offset_0: QM31,
    trace_1_column_10_offset_0: QM31,
    trace_1_column_11_offset_0: QM31,
    trace_1_column_1_offset_0: QM31,
    trace_1_column_2_offset_0: QM31,
    trace_1_column_32_offset_0: QM31,
    trace_1_column_33_offset_0: QM31,
    trace_1_column_3_offset_0: QM31,
    trace_1_column_42_offset_0: QM31,
    trace_1_column_43_offset_0: QM31,
    trace_1_column_4_offset_0: QM31,
    trace_1_column_5_offset_0: QM31,
    trace_1_column_6_offset_0: QM31,
    trace_1_column_7_offset_0: QM31,
    trace_1_column_8_offset_0: QM31,
    trace_1_column_9_offset_0: QM31,
) -> QM31 {
    (BlakeG_alpha0) * (trace_1_column_0_offset_0)
        + (BlakeG_alpha1) * (trace_1_column_1_offset_0)
        + (BlakeG_alpha2) * (trace_1_column_2_offset_0)
        + (BlakeG_alpha3) * (trace_1_column_3_offset_0)
        + (BlakeG_alpha4) * (trace_1_column_4_offset_0)
        + (BlakeG_alpha5) * (trace_1_column_5_offset_0)
        + (BlakeG_alpha6) * (trace_1_column_6_offset_0)
        + (BlakeG_alpha7) * (trace_1_column_7_offset_0)
        + (BlakeG_alpha8) * (trace_1_column_8_offset_0)
        + (BlakeG_alpha9) * (trace_1_column_9_offset_0)
        + (BlakeG_alpha10) * (trace_1_column_10_offset_0)
        + (BlakeG_alpha11) * (trace_1_column_11_offset_0)
        + (BlakeG_alpha12) * (trace_1_column_32_offset_0)
        + (BlakeG_alpha13) * (trace_1_column_33_offset_0)
        + (BlakeG_alpha14) * (intermediate30)
        + (BlakeG_alpha15) * (intermediate31)
        + (BlakeG_alpha16) * (trace_1_column_42_offset_0)
        + (BlakeG_alpha17) * (trace_1_column_43_offset_0)
        + (BlakeG_alpha18) * (intermediate22)
        + (BlakeG_alpha19) * (intermediate23)
        - (BlakeG_z)
}

