use stwo_constraint_framework::{
    PreprocessedColumn, PreprocessedColumnSet, PreprocessedColumnSetImpl,
};
use stwo_verifier_core::circle::{
    CirclePoint, CirclePointIndex, CirclePointIndexImpl, CirclePointQM31AddCirclePointM31Impl,
};
use stwo_verifier_core::fields::Invertible;
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
    pub VerifyInstruction_alpha15: QM31,
    pub VerifyInstruction_alpha18: QM31,
    pub VerifyInstruction_alpha2: QM31,
    pub VerifyInstruction_alpha3: QM31,
    pub VerifyInstruction_alpha4: QM31,
    pub VerifyInstruction_alpha5: QM31,
    pub VerifyInstruction_alpha6: QM31,
    pub VerifyInstruction_alpha9: QM31,
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
        VerifyInstruction_alpha15,
        VerifyInstruction_alpha18,
        VerifyInstruction_alpha2,
        VerifyInstruction_alpha3,
        VerifyInstruction_alpha4,
        VerifyInstruction_alpha5,
        VerifyInstruction_alpha6,
        VerifyInstruction_alpha9,
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
        trace_1_column_52,
        trace_1_column_53,
        trace_1_column_54,
        trace_1_column_55,
        trace_1_column_56,
        trace_1_column_57,
        trace_1_column_58,
        trace_1_column_59,
        trace_1_column_60,
        trace_1_column_61,
        trace_1_column_62,
        trace_1_column_63,
        trace_1_column_64,
        trace_1_column_65,
        trace_1_column_66,
        trace_1_column_67,
        trace_1_column_68,
        trace_1_column_69,
        trace_1_column_70,
        trace_1_column_71,
        trace_1_column_72,
        trace_1_column_73,
        trace_1_column_74,
        trace_1_column_75,
        trace_1_column_76,
        trace_1_column_77,
        trace_1_column_78,
        trace_1_column_79,
        trace_1_column_80,
        trace_1_column_81,
        trace_1_column_82,
        trace_1_column_83,
        trace_1_column_84,
        trace_1_column_85,
        trace_1_column_86,
        trace_1_column_87,
        trace_1_column_88,
        trace_1_column_89,
        trace_1_column_90,
        trace_1_column_91,
        trace_1_column_92,
        trace_1_column_93,
        trace_1_column_94,
        trace_1_column_95,
        trace_1_column_96,
        trace_1_column_97,
        trace_1_column_98,
    ]: [Span<QM31>; 99] =
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

    let [trace_1_column_52_offset_0]: [QM31; 1] = (*trace_1_column_52.try_into().unwrap()).unbox();

    let [trace_1_column_53_offset_0]: [QM31; 1] = (*trace_1_column_53.try_into().unwrap()).unbox();

    let [trace_1_column_54_offset_0]: [QM31; 1] = (*trace_1_column_54.try_into().unwrap()).unbox();

    let [trace_1_column_55_offset_0]: [QM31; 1] = (*trace_1_column_55.try_into().unwrap()).unbox();

    let [trace_1_column_56_offset_0]: [QM31; 1] = (*trace_1_column_56.try_into().unwrap()).unbox();

    let [trace_1_column_57_offset_0]: [QM31; 1] = (*trace_1_column_57.try_into().unwrap()).unbox();

    let [trace_1_column_58_offset_0]: [QM31; 1] = (*trace_1_column_58.try_into().unwrap()).unbox();

    let [trace_1_column_59_offset_0]: [QM31; 1] = (*trace_1_column_59.try_into().unwrap()).unbox();

    let [trace_1_column_60_offset_0]: [QM31; 1] = (*trace_1_column_60.try_into().unwrap()).unbox();

    let [trace_1_column_61_offset_0]: [QM31; 1] = (*trace_1_column_61.try_into().unwrap()).unbox();

    let [trace_1_column_62_offset_0]: [QM31; 1] = (*trace_1_column_62.try_into().unwrap()).unbox();

    let [trace_1_column_63_offset_0]: [QM31; 1] = (*trace_1_column_63.try_into().unwrap()).unbox();

    let [trace_1_column_64_offset_0]: [QM31; 1] = (*trace_1_column_64.try_into().unwrap()).unbox();

    let [trace_1_column_65_offset_0]: [QM31; 1] = (*trace_1_column_65.try_into().unwrap()).unbox();

    let [trace_1_column_66_offset_0]: [QM31; 1] = (*trace_1_column_66.try_into().unwrap()).unbox();

    let [trace_1_column_67_offset_0]: [QM31; 1] = (*trace_1_column_67.try_into().unwrap()).unbox();

    let [trace_1_column_68_offset_0]: [QM31; 1] = (*trace_1_column_68.try_into().unwrap()).unbox();

    let [trace_1_column_69_offset_0]: [QM31; 1] = (*trace_1_column_69.try_into().unwrap()).unbox();

    let [trace_1_column_70_offset_0]: [QM31; 1] = (*trace_1_column_70.try_into().unwrap()).unbox();

    let [trace_1_column_71_offset_0]: [QM31; 1] = (*trace_1_column_71.try_into().unwrap()).unbox();

    let [trace_1_column_72_offset_0]: [QM31; 1] = (*trace_1_column_72.try_into().unwrap()).unbox();

    let [trace_1_column_73_offset_0]: [QM31; 1] = (*trace_1_column_73.try_into().unwrap()).unbox();

    let [trace_1_column_74_offset_0]: [QM31; 1] = (*trace_1_column_74.try_into().unwrap()).unbox();

    let [trace_1_column_75_offset_0]: [QM31; 1] = (*trace_1_column_75.try_into().unwrap()).unbox();

    let [trace_1_column_76_offset_0]: [QM31; 1] = (*trace_1_column_76.try_into().unwrap()).unbox();

    let [trace_1_column_77_offset_0]: [QM31; 1] = (*trace_1_column_77.try_into().unwrap()).unbox();

    let [trace_1_column_78_offset_0]: [QM31; 1] = (*trace_1_column_78.try_into().unwrap()).unbox();

    let [trace_1_column_79_offset_0]: [QM31; 1] = (*trace_1_column_79.try_into().unwrap()).unbox();

    let [trace_1_column_80_offset_0]: [QM31; 1] = (*trace_1_column_80.try_into().unwrap()).unbox();

    let [trace_1_column_81_offset_0]: [QM31; 1] = (*trace_1_column_81.try_into().unwrap()).unbox();

    let [trace_1_column_82_offset_0]: [QM31; 1] = (*trace_1_column_82.try_into().unwrap()).unbox();

    let [trace_1_column_83_offset_0]: [QM31; 1] = (*trace_1_column_83.try_into().unwrap()).unbox();

    let [trace_1_column_84_offset_0]: [QM31; 1] = (*trace_1_column_84.try_into().unwrap()).unbox();

    let [trace_1_column_85_offset_0]: [QM31; 1] = (*trace_1_column_85.try_into().unwrap()).unbox();

    let [trace_1_column_86_offset_0]: [QM31; 1] = (*trace_1_column_86.try_into().unwrap()).unbox();

    let [trace_1_column_87_offset_0]: [QM31; 1] = (*trace_1_column_87.try_into().unwrap()).unbox();

    let [trace_1_column_88_offset_0]: [QM31; 1] = (*trace_1_column_88.try_into().unwrap()).unbox();

    let [trace_1_column_89_offset_0]: [QM31; 1] = (*trace_1_column_89.try_into().unwrap()).unbox();

    let [trace_1_column_90_offset_0]: [QM31; 1] = (*trace_1_column_90.try_into().unwrap()).unbox();

    let [trace_1_column_91_offset_0]: [QM31; 1] = (*trace_1_column_91.try_into().unwrap()).unbox();

    let [trace_1_column_92_offset_0]: [QM31; 1] = (*trace_1_column_92.try_into().unwrap()).unbox();

    let [trace_1_column_93_offset_0]: [QM31; 1] = (*trace_1_column_93.try_into().unwrap()).unbox();

    let [trace_1_column_94_offset_0]: [QM31; 1] = (*trace_1_column_94.try_into().unwrap()).unbox();

    let [trace_1_column_95_offset_0]: [QM31; 1] = (*trace_1_column_95.try_into().unwrap()).unbox();

    let [trace_1_column_96_offset_0]: [QM31; 1] = (*trace_1_column_96.try_into().unwrap()).unbox();

    let [trace_1_column_97_offset_0]: [QM31; 1] = (*trace_1_column_97.try_into().unwrap()).unbox();

    let [trace_1_column_98_offset_0]: [QM31; 1] = (*trace_1_column_98.try_into().unwrap()).unbox();

    let [
        trace_2_column_99,
        trace_2_column_100,
        trace_2_column_101,
        trace_2_column_102,
        trace_2_column_103,
        trace_2_column_104,
        trace_2_column_105,
        trace_2_column_106,
        trace_2_column_107,
        trace_2_column_108,
        trace_2_column_109,
        trace_2_column_110,
        trace_2_column_111,
        trace_2_column_112,
        trace_2_column_113,
        trace_2_column_114,
        trace_2_column_115,
        trace_2_column_116,
        trace_2_column_117,
        trace_2_column_118,
    ]: [Span<QM31>; 20] =
        (*interaction_mask_values
        .multi_pop_front()
        .unwrap())
        .unbox();

    let [trace_2_column_99_offset_0]: [QM31; 1] = (*trace_2_column_99.try_into().unwrap()).unbox();

    let [trace_2_column_100_offset_0]: [QM31; 1] = (*trace_2_column_100.try_into().unwrap())
        .unbox();

    let [trace_2_column_101_offset_0]: [QM31; 1] = (*trace_2_column_101.try_into().unwrap())
        .unbox();

    let [trace_2_column_102_offset_0]: [QM31; 1] = (*trace_2_column_102.try_into().unwrap())
        .unbox();

    let [trace_2_column_103_offset_0]: [QM31; 1] = (*trace_2_column_103.try_into().unwrap())
        .unbox();

    let [trace_2_column_104_offset_0]: [QM31; 1] = (*trace_2_column_104.try_into().unwrap())
        .unbox();

    let [trace_2_column_105_offset_0]: [QM31; 1] = (*trace_2_column_105.try_into().unwrap())
        .unbox();

    let [trace_2_column_106_offset_0]: [QM31; 1] = (*trace_2_column_106.try_into().unwrap())
        .unbox();

    let [trace_2_column_107_offset_0]: [QM31; 1] = (*trace_2_column_107.try_into().unwrap())
        .unbox();

    let [trace_2_column_108_offset_0]: [QM31; 1] = (*trace_2_column_108.try_into().unwrap())
        .unbox();

    let [trace_2_column_109_offset_0]: [QM31; 1] = (*trace_2_column_109.try_into().unwrap())
        .unbox();

    let [trace_2_column_110_offset_0]: [QM31; 1] = (*trace_2_column_110.try_into().unwrap())
        .unbox();

    let [trace_2_column_111_offset_0]: [QM31; 1] = (*trace_2_column_111.try_into().unwrap())
        .unbox();

    let [trace_2_column_112_offset_0]: [QM31; 1] = (*trace_2_column_112.try_into().unwrap())
        .unbox();

    let [trace_2_column_113_offset_0]: [QM31; 1] = (*trace_2_column_113.try_into().unwrap())
        .unbox();

    let [trace_2_column_114_offset_0]: [QM31; 1] = (*trace_2_column_114.try_into().unwrap())
        .unbox();

    let [trace_2_column_115_offset_neg_1, trace_2_column_115_offset_0]: [QM31; 2] =
        (*trace_2_column_115
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_116_offset_neg_1, trace_2_column_116_offset_0]: [QM31; 2] =
        (*trace_2_column_116
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_117_offset_neg_1, trace_2_column_117_offset_0]: [QM31; 2] =
        (*trace_2_column_117
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_118_offset_neg_1, trace_2_column_118_offset_0]: [QM31; 2] =
        (*trace_2_column_118
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
        VerifyInstruction_alpha15,
        VerifyInstruction_alpha18,
        VerifyInstruction_alpha2,
        VerifyInstruction_alpha3,
        VerifyInstruction_alpha4,
        VerifyInstruction_alpha5,
        VerifyInstruction_alpha6,
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
        trace_1_column_52_offset_0,
        trace_1_column_53_offset_0,
        trace_1_column_54_offset_0,
        trace_1_column_55_offset_0,
        trace_1_column_56_offset_0,
        trace_1_column_57_offset_0,
        trace_1_column_58_offset_0,
        trace_1_column_59_offset_0,
        trace_1_column_5_offset_0,
        trace_1_column_60_offset_0,
        trace_1_column_61_offset_0,
        trace_1_column_62_offset_0,
        trace_1_column_63_offset_0,
        trace_1_column_64_offset_0,
        trace_1_column_65_offset_0,
        trace_1_column_66_offset_0,
        trace_1_column_67_offset_0,
        trace_1_column_68_offset_0,
        trace_1_column_69_offset_0,
        trace_1_column_6_offset_0,
        trace_1_column_70_offset_0,
        trace_1_column_71_offset_0,
        trace_1_column_72_offset_0,
        trace_1_column_73_offset_0,
        trace_1_column_74_offset_0,
        trace_1_column_75_offset_0,
        trace_1_column_76_offset_0,
        trace_1_column_77_offset_0,
        trace_1_column_78_offset_0,
        trace_1_column_79_offset_0,
        trace_1_column_7_offset_0,
        trace_1_column_80_offset_0,
        trace_1_column_81_offset_0,
        trace_1_column_82_offset_0,
        trace_1_column_83_offset_0,
        trace_1_column_84_offset_0,
        trace_1_column_85_offset_0,
        trace_1_column_86_offset_0,
        trace_1_column_87_offset_0,
        trace_1_column_88_offset_0,
        trace_1_column_89_offset_0,
        trace_1_column_8_offset_0,
        trace_1_column_90_offset_0,
        trace_1_column_91_offset_0,
        trace_1_column_92_offset_0,
        trace_1_column_93_offset_0,
        trace_1_column_94_offset_0,
        trace_1_column_95_offset_0,
        trace_1_column_96_offset_0,
        trace_1_column_97_offset_0,
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
    let intermediate33 = *intermediates.pop_front().unwrap();
    let intermediate34 = *intermediates.pop_front().unwrap();
    let intermediate35 = *intermediates.pop_front().unwrap();

    // Constraint 0
    let constraint_quotient = ((trace_1_column_98_offset_0) * (trace_1_column_98_offset_0)
        - (trace_1_column_98_offset_0))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 1
    let constraint_quotient = (trace_1_column_8_offset_0
        - ((trace_1_column_5_offset_0) * (trace_1_column_2_offset_0)
            + (m31(1).into() - (trace_1_column_5_offset_0)) * (trace_1_column_1_offset_0)))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 2
    let constraint_quotient = (trace_1_column_9_offset_0
        - ((trace_1_column_6_offset_0) * (trace_1_column_2_offset_0)
            + (m31(1).into() - (trace_1_column_6_offset_0)) * (trace_1_column_1_offset_0)))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 3
    let constraint_quotient = ((trace_1_column_97_offset_0)
        * (trace_1_column_97_offset_0 - (m31(1).into())))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 4
    let constraint_quotient = ((intermediate7)
        * ((intermediate7) * (intermediate7) - (m31(1).into())))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 5
    let constraint_quotient = ((intermediate8)
        * ((intermediate8) * (intermediate8) - (m31(1).into())))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 6
    let constraint_quotient = ((intermediate9)
        * ((intermediate9) * (intermediate9) - (m31(1).into())))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 7
    let constraint_quotient = ((intermediate10)
        * ((intermediate10) * (intermediate10) - (m31(1).into())))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 8
    let constraint_quotient = ((intermediate11)
        * ((intermediate11) * (intermediate11) - (m31(1).into())))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 9
    let constraint_quotient = ((intermediate12)
        * ((intermediate12) * (intermediate12) - (m31(1).into())))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 10
    let constraint_quotient = ((intermediate13)
        * ((intermediate13) * (intermediate13) - (m31(1).into())))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 11
    let constraint_quotient = ((intermediate14)
        * ((intermediate14) * (intermediate14) - (m31(1).into())))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 12
    let constraint_quotient = ((intermediate15)
        * ((intermediate15) * (intermediate15) - (m31(1).into())))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 13
    let constraint_quotient = ((intermediate16)
        * ((intermediate16) * (intermediate16) - (m31(1).into())))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 14
    let constraint_quotient = ((intermediate17)
        * ((intermediate17) * (intermediate17) - (m31(1).into())))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 15
    let constraint_quotient = ((intermediate18)
        * ((intermediate18) * (intermediate18) - (m31(1).into())))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 16
    let constraint_quotient = ((intermediate19)
        * ((intermediate19) * (intermediate19) - (m31(1).into())))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 17
    let constraint_quotient = ((intermediate20)
        * ((intermediate20) * (intermediate20) - (m31(1).into())))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 18
    let constraint_quotient = ((intermediate21)
        * ((intermediate21) * (intermediate21) - (m31(1).into())))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 19
    let constraint_quotient = ((intermediate22)
        * ((intermediate22) * (intermediate22) - (m31(1).into())))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 20
    let constraint_quotient = ((intermediate23)
        * ((intermediate23) * (intermediate23) - (m31(1).into())))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 21
    let constraint_quotient = ((intermediate24)
        * ((intermediate24) * (intermediate24) - (m31(1).into())))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 22
    let constraint_quotient = ((intermediate25)
        * ((intermediate25) * (intermediate25) - (m31(1).into())))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 23
    let constraint_quotient = ((intermediate26)
        * ((intermediate26) * (intermediate26) - (m31(1).into())))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 24
    let constraint_quotient = ((intermediate27)
        * ((intermediate27) * (intermediate27) - (m31(1).into())))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 25
    let constraint_quotient = ((intermediate28)
        * ((intermediate28) * (intermediate28) - (m31(1).into())))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 26
    let constraint_quotient = ((intermediate29)
        * ((intermediate29) * (intermediate29) - (m31(1).into())))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 27
    let constraint_quotient = ((intermediate30)
        * ((intermediate30) * (intermediate30) - (m31(1).into())))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 28
    let constraint_quotient = ((intermediate31)
        * ((intermediate31) * (intermediate31) - (m31(1).into())))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 29
    let constraint_quotient = ((intermediate32)
        * ((intermediate32) * (intermediate32) - (m31(1).into())))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 30
    let constraint_quotient = ((intermediate33)
        * ((intermediate33) * (intermediate33) - (m31(1).into())))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 31
    let constraint_quotient = (trace_1_column_67_offset_0
        + trace_1_column_96_offset_0
        + intermediate33
        - (trace_1_column_38_offset_0)
        - ((m31(256).into()) * (trace_1_column_97_offset_0)))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    core::internal::revoke_ap_tracking();

    // Constraint 32
    let constraint_quotient = ((QM31Impl::from_partial_evals(
        [
            trace_2_column_99_offset_0, trace_2_column_100_offset_0, trace_2_column_101_offset_0,
            trace_2_column_102_offset_0,
        ],
    ))
        * ((intermediate0) * (intermediate1))
        - (intermediate1 + intermediate0))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 33
    let constraint_quotient = ((QM31Impl::from_partial_evals(
        [
            trace_2_column_103_offset_0, trace_2_column_104_offset_0, trace_2_column_105_offset_0,
            trace_2_column_106_offset_0,
        ],
    )
        - (QM31Impl::from_partial_evals(
            [
                trace_2_column_99_offset_0, trace_2_column_100_offset_0,
                trace_2_column_101_offset_0, trace_2_column_102_offset_0,
            ],
        )))
        * ((intermediate2) * (intermediate3))
        - (intermediate3 + intermediate2))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 34
    let constraint_quotient = ((QM31Impl::from_partial_evals(
        [
            trace_2_column_107_offset_0, trace_2_column_108_offset_0, trace_2_column_109_offset_0,
            trace_2_column_110_offset_0,
        ],
    )
        - (QM31Impl::from_partial_evals(
            [
                trace_2_column_103_offset_0, trace_2_column_104_offset_0,
                trace_2_column_105_offset_0, trace_2_column_106_offset_0,
            ],
        )))
        * ((intermediate4) * (intermediate5))
        - (intermediate5 + intermediate4))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 35
    let constraint_quotient = ((QM31Impl::from_partial_evals(
        [
            trace_2_column_111_offset_0, trace_2_column_112_offset_0, trace_2_column_113_offset_0,
            trace_2_column_114_offset_0,
        ],
    )
        - (QM31Impl::from_partial_evals(
            [
                trace_2_column_107_offset_0, trace_2_column_108_offset_0,
                trace_2_column_109_offset_0, trace_2_column_110_offset_0,
            ],
        )))
        * ((intermediate6) * (intermediate34))
        - (intermediate34 + (intermediate6) * (trace_1_column_98_offset_0)))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 36
    let constraint_quotient = ((QM31Impl::from_partial_evals(
        [
            trace_2_column_115_offset_0, trace_2_column_116_offset_0, trace_2_column_117_offset_0,
            trace_2_column_118_offset_0,
        ],
    )
        - (QM31Impl::from_partial_evals(
            [
                trace_2_column_115_offset_neg_1, trace_2_column_116_offset_neg_1,
                trace_2_column_117_offset_neg_1, trace_2_column_118_offset_neg_1,
            ],
        ))
        - (QM31Impl::from_partial_evals(
            [
                trace_2_column_111_offset_0, trace_2_column_112_offset_0,
                trace_2_column_113_offset_0, trace_2_column_114_offset_0,
            ],
        ))
        + (claimed_sum) * (column_size.inverse().into()))
        * (intermediate35)
        + trace_1_column_98_offset_0)
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;
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
    VerifyInstruction_alpha6: QM31,
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
    trace_1_column_52_offset_0: QM31,
    trace_1_column_53_offset_0: QM31,
    trace_1_column_54_offset_0: QM31,
    trace_1_column_55_offset_0: QM31,
    trace_1_column_56_offset_0: QM31,
    trace_1_column_57_offset_0: QM31,
    trace_1_column_58_offset_0: QM31,
    trace_1_column_59_offset_0: QM31,
    trace_1_column_5_offset_0: QM31,
    trace_1_column_60_offset_0: QM31,
    trace_1_column_61_offset_0: QM31,
    trace_1_column_62_offset_0: QM31,
    trace_1_column_63_offset_0: QM31,
    trace_1_column_64_offset_0: QM31,
    trace_1_column_65_offset_0: QM31,
    trace_1_column_66_offset_0: QM31,
    trace_1_column_67_offset_0: QM31,
    trace_1_column_68_offset_0: QM31,
    trace_1_column_69_offset_0: QM31,
    trace_1_column_6_offset_0: QM31,
    trace_1_column_70_offset_0: QM31,
    trace_1_column_71_offset_0: QM31,
    trace_1_column_72_offset_0: QM31,
    trace_1_column_73_offset_0: QM31,
    trace_1_column_74_offset_0: QM31,
    trace_1_column_75_offset_0: QM31,
    trace_1_column_76_offset_0: QM31,
    trace_1_column_77_offset_0: QM31,
    trace_1_column_78_offset_0: QM31,
    trace_1_column_79_offset_0: QM31,
    trace_1_column_7_offset_0: QM31,
    trace_1_column_80_offset_0: QM31,
    trace_1_column_81_offset_0: QM31,
    trace_1_column_82_offset_0: QM31,
    trace_1_column_83_offset_0: QM31,
    trace_1_column_84_offset_0: QM31,
    trace_1_column_85_offset_0: QM31,
    trace_1_column_86_offset_0: QM31,
    trace_1_column_87_offset_0: QM31,
    trace_1_column_88_offset_0: QM31,
    trace_1_column_89_offset_0: QM31,
    trace_1_column_8_offset_0: QM31,
    trace_1_column_90_offset_0: QM31,
    trace_1_column_91_offset_0: QM31,
    trace_1_column_92_offset_0: QM31,
    trace_1_column_93_offset_0: QM31,
    trace_1_column_94_offset_0: QM31,
    trace_1_column_95_offset_0: QM31,
    trace_1_column_96_offset_0: QM31,
    trace_1_column_97_offset_0: QM31,
    trace_1_column_9_offset_0: QM31,
) -> Array<QM31> {
    let intermediate7 = intermediate7(
        trace_1_column_11_offset_0,
        trace_1_column_40_offset_0,
        trace_1_column_69_offset_0,
        trace_1_column_97_offset_0,
    );

    let intermediate8 = intermediate8(
        intermediate7,
        trace_1_column_12_offset_0,
        trace_1_column_41_offset_0,
        trace_1_column_70_offset_0,
    );

    let intermediate9 = intermediate9(
        intermediate8,
        trace_1_column_13_offset_0,
        trace_1_column_42_offset_0,
        trace_1_column_71_offset_0,
    );

    let intermediate10 = intermediate10(
        intermediate9,
        trace_1_column_14_offset_0,
        trace_1_column_43_offset_0,
        trace_1_column_72_offset_0,
    );

    let intermediate11 = intermediate11(
        intermediate10,
        trace_1_column_15_offset_0,
        trace_1_column_44_offset_0,
        trace_1_column_73_offset_0,
    );

    let intermediate12 = intermediate12(
        intermediate11,
        trace_1_column_16_offset_0,
        trace_1_column_45_offset_0,
        trace_1_column_74_offset_0,
    );

    let intermediate13 = intermediate13(
        intermediate12,
        trace_1_column_17_offset_0,
        trace_1_column_46_offset_0,
        trace_1_column_75_offset_0,
    );

    let intermediate14 = intermediate14(
        intermediate13,
        trace_1_column_18_offset_0,
        trace_1_column_47_offset_0,
        trace_1_column_76_offset_0,
    );

    let intermediate15 = intermediate15(
        intermediate14,
        trace_1_column_19_offset_0,
        trace_1_column_48_offset_0,
        trace_1_column_77_offset_0,
    );

    let intermediate16 = intermediate16(
        intermediate15,
        trace_1_column_20_offset_0,
        trace_1_column_49_offset_0,
        trace_1_column_78_offset_0,
    );

    let intermediate17 = intermediate17(
        intermediate16,
        trace_1_column_21_offset_0,
        trace_1_column_50_offset_0,
        trace_1_column_79_offset_0,
    );

    let intermediate18 = intermediate18(
        intermediate17,
        trace_1_column_22_offset_0,
        trace_1_column_51_offset_0,
        trace_1_column_80_offset_0,
    );

    let intermediate19 = intermediate19(
        intermediate18,
        trace_1_column_23_offset_0,
        trace_1_column_52_offset_0,
        trace_1_column_81_offset_0,
    );

    let intermediate20 = intermediate20(
        intermediate19,
        trace_1_column_24_offset_0,
        trace_1_column_53_offset_0,
        trace_1_column_82_offset_0,
    );

    let intermediate21 = intermediate21(
        intermediate20,
        trace_1_column_25_offset_0,
        trace_1_column_54_offset_0,
        trace_1_column_83_offset_0,
    );

    let intermediate22 = intermediate22(
        intermediate21,
        trace_1_column_26_offset_0,
        trace_1_column_55_offset_0,
        trace_1_column_84_offset_0,
    );

    let intermediate23 = intermediate23(
        intermediate22,
        trace_1_column_27_offset_0,
        trace_1_column_56_offset_0,
        trace_1_column_85_offset_0,
    );

    let intermediate24 = intermediate24(
        intermediate23,
        trace_1_column_28_offset_0,
        trace_1_column_57_offset_0,
        trace_1_column_86_offset_0,
    );

    let intermediate25 = intermediate25(
        intermediate24,
        trace_1_column_29_offset_0,
        trace_1_column_58_offset_0,
        trace_1_column_87_offset_0,
    );

    let intermediate26 = intermediate26(
        intermediate25,
        trace_1_column_30_offset_0,
        trace_1_column_59_offset_0,
        trace_1_column_88_offset_0,
    );

    let intermediate27 = intermediate27(
        intermediate26,
        trace_1_column_31_offset_0,
        trace_1_column_60_offset_0,
        trace_1_column_89_offset_0,
    );

    let intermediate28 = intermediate28(
        intermediate27,
        trace_1_column_32_offset_0,
        trace_1_column_61_offset_0,
        trace_1_column_90_offset_0,
        trace_1_column_97_offset_0,
    );

    let intermediate29 = intermediate29(
        intermediate28,
        trace_1_column_33_offset_0,
        trace_1_column_62_offset_0,
        trace_1_column_91_offset_0,
    );

    let intermediate30 = intermediate30(
        intermediate29,
        trace_1_column_34_offset_0,
        trace_1_column_63_offset_0,
        trace_1_column_92_offset_0,
    );

    let intermediate31 = intermediate31(
        intermediate30,
        trace_1_column_35_offset_0,
        trace_1_column_64_offset_0,
        trace_1_column_93_offset_0,
    );

    let intermediate32 = intermediate32(
        intermediate31,
        trace_1_column_36_offset_0,
        trace_1_column_65_offset_0,
        trace_1_column_94_offset_0,
    );

    let intermediate33 = intermediate33(
        intermediate32,
        trace_1_column_37_offset_0,
        trace_1_column_66_offset_0,
        trace_1_column_95_offset_0,
    );
    let intermediate0 = intermediate0(
        VerifyInstruction_alpha0,
        VerifyInstruction_alpha1,
        VerifyInstruction_alpha15,
        VerifyInstruction_alpha18,
        VerifyInstruction_alpha2,
        VerifyInstruction_alpha3,
        VerifyInstruction_alpha4,
        VerifyInstruction_alpha5,
        VerifyInstruction_alpha6,
        VerifyInstruction_alpha9,
        VerifyInstruction_z,
        trace_1_column_0_offset_0,
        trace_1_column_3_offset_0,
        trace_1_column_4_offset_0,
        trace_1_column_5_offset_0,
        trace_1_column_6_offset_0,
        trace_1_column_7_offset_0,
    );

    let intermediate1 = intermediate1(
        MemoryAddressToId_alpha0,
        MemoryAddressToId_alpha1,
        MemoryAddressToId_z,
        trace_1_column_10_offset_0,
        trace_1_column_3_offset_0,
        trace_1_column_8_offset_0,
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
        trace_1_column_28_offset_0,
        trace_1_column_29_offset_0,
        trace_1_column_30_offset_0,
        trace_1_column_31_offset_0,
        trace_1_column_32_offset_0,
        trace_1_column_33_offset_0,
        trace_1_column_34_offset_0,
        trace_1_column_35_offset_0,
        trace_1_column_36_offset_0,
        trace_1_column_37_offset_0,
        trace_1_column_38_offset_0,
    );

    let intermediate3 = intermediate3(
        MemoryAddressToId_alpha0,
        MemoryAddressToId_alpha1,
        MemoryAddressToId_z,
        trace_1_column_39_offset_0,
        trace_1_column_4_offset_0,
        trace_1_column_9_offset_0,
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
        trace_1_column_39_offset_0,
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
        trace_1_column_50_offset_0,
        trace_1_column_51_offset_0,
        trace_1_column_52_offset_0,
        trace_1_column_53_offset_0,
        trace_1_column_54_offset_0,
        trace_1_column_55_offset_0,
        trace_1_column_56_offset_0,
        trace_1_column_57_offset_0,
        trace_1_column_58_offset_0,
        trace_1_column_59_offset_0,
        trace_1_column_60_offset_0,
        trace_1_column_61_offset_0,
        trace_1_column_62_offset_0,
        trace_1_column_63_offset_0,
        trace_1_column_64_offset_0,
        trace_1_column_65_offset_0,
        trace_1_column_66_offset_0,
        trace_1_column_67_offset_0,
    );

    let intermediate5 = intermediate5(
        MemoryAddressToId_alpha0,
        MemoryAddressToId_alpha1,
        MemoryAddressToId_z,
        trace_1_column_0_offset_0,
        trace_1_column_68_offset_0,
    );

    core::internal::revoke_ap_tracking();

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
        trace_1_column_68_offset_0,
        trace_1_column_69_offset_0,
        trace_1_column_70_offset_0,
        trace_1_column_71_offset_0,
        trace_1_column_72_offset_0,
        trace_1_column_73_offset_0,
        trace_1_column_74_offset_0,
        trace_1_column_75_offset_0,
        trace_1_column_76_offset_0,
        trace_1_column_77_offset_0,
        trace_1_column_78_offset_0,
        trace_1_column_79_offset_0,
        trace_1_column_80_offset_0,
        trace_1_column_81_offset_0,
        trace_1_column_82_offset_0,
        trace_1_column_83_offset_0,
        trace_1_column_84_offset_0,
        trace_1_column_85_offset_0,
        trace_1_column_86_offset_0,
        trace_1_column_87_offset_0,
        trace_1_column_88_offset_0,
        trace_1_column_89_offset_0,
        trace_1_column_90_offset_0,
        trace_1_column_91_offset_0,
        trace_1_column_92_offset_0,
        trace_1_column_93_offset_0,
        trace_1_column_94_offset_0,
        trace_1_column_95_offset_0,
        trace_1_column_96_offset_0,
    );

    let intermediate34 = intermediate34(
        Opcodes_alpha0,
        Opcodes_alpha1,
        Opcodes_alpha2,
        Opcodes_z,
        trace_1_column_0_offset_0,
        trace_1_column_1_offset_0,
        trace_1_column_2_offset_0,
    );

    let intermediate35 = intermediate35(
        Opcodes_alpha0,
        Opcodes_alpha1,
        Opcodes_alpha2,
        Opcodes_z,
        trace_1_column_0_offset_0,
        trace_1_column_1_offset_0,
        trace_1_column_2_offset_0,
        trace_1_column_7_offset_0,
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
        intermediate33,
        intermediate34,
        intermediate35,
    ]
}

pub fn intermediate7(
    trace_1_column_11_offset_0: QM31,
    trace_1_column_40_offset_0: QM31,
    trace_1_column_69_offset_0: QM31,
    trace_1_column_97_offset_0: QM31,
) -> QM31 {
    (trace_1_column_40_offset_0
        + trace_1_column_69_offset_0
        - (trace_1_column_11_offset_0)
        - (trace_1_column_97_offset_0))
        * (m31(4194304).into())
}

pub fn intermediate8(
    intermediate7: QM31,
    trace_1_column_12_offset_0: QM31,
    trace_1_column_41_offset_0: QM31,
    trace_1_column_70_offset_0: QM31,
) -> QM31 {
    (trace_1_column_41_offset_0
        + trace_1_column_70_offset_0
        + intermediate7
        - (trace_1_column_12_offset_0))
        * (m31(4194304).into())
}

pub fn intermediate9(
    intermediate8: QM31,
    trace_1_column_13_offset_0: QM31,
    trace_1_column_42_offset_0: QM31,
    trace_1_column_71_offset_0: QM31,
) -> QM31 {
    (trace_1_column_42_offset_0
        + trace_1_column_71_offset_0
        + intermediate8
        - (trace_1_column_13_offset_0))
        * (m31(4194304).into())
}

pub fn intermediate10(
    intermediate9: QM31,
    trace_1_column_14_offset_0: QM31,
    trace_1_column_43_offset_0: QM31,
    trace_1_column_72_offset_0: QM31,
) -> QM31 {
    (trace_1_column_43_offset_0
        + trace_1_column_72_offset_0
        + intermediate9
        - (trace_1_column_14_offset_0))
        * (m31(4194304).into())
}

pub fn intermediate11(
    intermediate10: QM31,
    trace_1_column_15_offset_0: QM31,
    trace_1_column_44_offset_0: QM31,
    trace_1_column_73_offset_0: QM31,
) -> QM31 {
    (trace_1_column_44_offset_0
        + trace_1_column_73_offset_0
        + intermediate10
        - (trace_1_column_15_offset_0))
        * (m31(4194304).into())
}

pub fn intermediate12(
    intermediate11: QM31,
    trace_1_column_16_offset_0: QM31,
    trace_1_column_45_offset_0: QM31,
    trace_1_column_74_offset_0: QM31,
) -> QM31 {
    (trace_1_column_45_offset_0
        + trace_1_column_74_offset_0
        + intermediate11
        - (trace_1_column_16_offset_0))
        * (m31(4194304).into())
}

pub fn intermediate13(
    intermediate12: QM31,
    trace_1_column_17_offset_0: QM31,
    trace_1_column_46_offset_0: QM31,
    trace_1_column_75_offset_0: QM31,
) -> QM31 {
    (trace_1_column_46_offset_0
        + trace_1_column_75_offset_0
        + intermediate12
        - (trace_1_column_17_offset_0))
        * (m31(4194304).into())
}

pub fn intermediate14(
    intermediate13: QM31,
    trace_1_column_18_offset_0: QM31,
    trace_1_column_47_offset_0: QM31,
    trace_1_column_76_offset_0: QM31,
) -> QM31 {
    (trace_1_column_47_offset_0
        + trace_1_column_76_offset_0
        + intermediate13
        - (trace_1_column_18_offset_0))
        * (m31(4194304).into())
}

pub fn intermediate15(
    intermediate14: QM31,
    trace_1_column_19_offset_0: QM31,
    trace_1_column_48_offset_0: QM31,
    trace_1_column_77_offset_0: QM31,
) -> QM31 {
    (trace_1_column_48_offset_0
        + trace_1_column_77_offset_0
        + intermediate14
        - (trace_1_column_19_offset_0))
        * (m31(4194304).into())
}

pub fn intermediate16(
    intermediate15: QM31,
    trace_1_column_20_offset_0: QM31,
    trace_1_column_49_offset_0: QM31,
    trace_1_column_78_offset_0: QM31,
) -> QM31 {
    (trace_1_column_49_offset_0
        + trace_1_column_78_offset_0
        + intermediate15
        - (trace_1_column_20_offset_0))
        * (m31(4194304).into())
}

pub fn intermediate17(
    intermediate16: QM31,
    trace_1_column_21_offset_0: QM31,
    trace_1_column_50_offset_0: QM31,
    trace_1_column_79_offset_0: QM31,
) -> QM31 {
    (trace_1_column_50_offset_0
        + trace_1_column_79_offset_0
        + intermediate16
        - (trace_1_column_21_offset_0))
        * (m31(4194304).into())
}

pub fn intermediate18(
    intermediate17: QM31,
    trace_1_column_22_offset_0: QM31,
    trace_1_column_51_offset_0: QM31,
    trace_1_column_80_offset_0: QM31,
) -> QM31 {
    (trace_1_column_51_offset_0
        + trace_1_column_80_offset_0
        + intermediate17
        - (trace_1_column_22_offset_0))
        * (m31(4194304).into())
}

pub fn intermediate19(
    intermediate18: QM31,
    trace_1_column_23_offset_0: QM31,
    trace_1_column_52_offset_0: QM31,
    trace_1_column_81_offset_0: QM31,
) -> QM31 {
    (trace_1_column_52_offset_0
        + trace_1_column_81_offset_0
        + intermediate18
        - (trace_1_column_23_offset_0))
        * (m31(4194304).into())
}

pub fn intermediate20(
    intermediate19: QM31,
    trace_1_column_24_offset_0: QM31,
    trace_1_column_53_offset_0: QM31,
    trace_1_column_82_offset_0: QM31,
) -> QM31 {
    (trace_1_column_53_offset_0
        + trace_1_column_82_offset_0
        + intermediate19
        - (trace_1_column_24_offset_0))
        * (m31(4194304).into())
}

pub fn intermediate21(
    intermediate20: QM31,
    trace_1_column_25_offset_0: QM31,
    trace_1_column_54_offset_0: QM31,
    trace_1_column_83_offset_0: QM31,
) -> QM31 {
    (trace_1_column_54_offset_0
        + trace_1_column_83_offset_0
        + intermediate20
        - (trace_1_column_25_offset_0))
        * (m31(4194304).into())
}

pub fn intermediate22(
    intermediate21: QM31,
    trace_1_column_26_offset_0: QM31,
    trace_1_column_55_offset_0: QM31,
    trace_1_column_84_offset_0: QM31,
) -> QM31 {
    (trace_1_column_55_offset_0
        + trace_1_column_84_offset_0
        + intermediate21
        - (trace_1_column_26_offset_0))
        * (m31(4194304).into())
}

pub fn intermediate23(
    intermediate22: QM31,
    trace_1_column_27_offset_0: QM31,
    trace_1_column_56_offset_0: QM31,
    trace_1_column_85_offset_0: QM31,
) -> QM31 {
    (trace_1_column_56_offset_0
        + trace_1_column_85_offset_0
        + intermediate22
        - (trace_1_column_27_offset_0))
        * (m31(4194304).into())
}

pub fn intermediate24(
    intermediate23: QM31,
    trace_1_column_28_offset_0: QM31,
    trace_1_column_57_offset_0: QM31,
    trace_1_column_86_offset_0: QM31,
) -> QM31 {
    (trace_1_column_57_offset_0
        + trace_1_column_86_offset_0
        + intermediate23
        - (trace_1_column_28_offset_0))
        * (m31(4194304).into())
}

pub fn intermediate25(
    intermediate24: QM31,
    trace_1_column_29_offset_0: QM31,
    trace_1_column_58_offset_0: QM31,
    trace_1_column_87_offset_0: QM31,
) -> QM31 {
    (trace_1_column_58_offset_0
        + trace_1_column_87_offset_0
        + intermediate24
        - (trace_1_column_29_offset_0))
        * (m31(4194304).into())
}

pub fn intermediate26(
    intermediate25: QM31,
    trace_1_column_30_offset_0: QM31,
    trace_1_column_59_offset_0: QM31,
    trace_1_column_88_offset_0: QM31,
) -> QM31 {
    (trace_1_column_59_offset_0
        + trace_1_column_88_offset_0
        + intermediate25
        - (trace_1_column_30_offset_0))
        * (m31(4194304).into())
}

pub fn intermediate27(
    intermediate26: QM31,
    trace_1_column_31_offset_0: QM31,
    trace_1_column_60_offset_0: QM31,
    trace_1_column_89_offset_0: QM31,
) -> QM31 {
    (trace_1_column_60_offset_0
        + trace_1_column_89_offset_0
        + intermediate26
        - (trace_1_column_31_offset_0))
        * (m31(4194304).into())
}

pub fn intermediate28(
    intermediate27: QM31,
    trace_1_column_32_offset_0: QM31,
    trace_1_column_61_offset_0: QM31,
    trace_1_column_90_offset_0: QM31,
    trace_1_column_97_offset_0: QM31,
) -> QM31 {
    (trace_1_column_61_offset_0
        + trace_1_column_90_offset_0
        + intermediate27
        - (trace_1_column_32_offset_0)
        - ((m31(136).into()) * (trace_1_column_97_offset_0)))
        * (m31(4194304).into())
}

pub fn intermediate29(
    intermediate28: QM31,
    trace_1_column_33_offset_0: QM31,
    trace_1_column_62_offset_0: QM31,
    trace_1_column_91_offset_0: QM31,
) -> QM31 {
    (trace_1_column_62_offset_0
        + trace_1_column_91_offset_0
        + intermediate28
        - (trace_1_column_33_offset_0))
        * (m31(4194304).into())
}

pub fn intermediate30(
    intermediate29: QM31,
    trace_1_column_34_offset_0: QM31,
    trace_1_column_63_offset_0: QM31,
    trace_1_column_92_offset_0: QM31,
) -> QM31 {
    (trace_1_column_63_offset_0
        + trace_1_column_92_offset_0
        + intermediate29
        - (trace_1_column_34_offset_0))
        * (m31(4194304).into())
}

pub fn intermediate31(
    intermediate30: QM31,
    trace_1_column_35_offset_0: QM31,
    trace_1_column_64_offset_0: QM31,
    trace_1_column_93_offset_0: QM31,
) -> QM31 {
    (trace_1_column_64_offset_0
        + trace_1_column_93_offset_0
        + intermediate30
        - (trace_1_column_35_offset_0))
        * (m31(4194304).into())
}

pub fn intermediate32(
    intermediate31: QM31,
    trace_1_column_36_offset_0: QM31,
    trace_1_column_65_offset_0: QM31,
    trace_1_column_94_offset_0: QM31,
) -> QM31 {
    (trace_1_column_65_offset_0
        + trace_1_column_94_offset_0
        + intermediate31
        - (trace_1_column_36_offset_0))
        * (m31(4194304).into())
}

pub fn intermediate33(
    intermediate32: QM31,
    trace_1_column_37_offset_0: QM31,
    trace_1_column_66_offset_0: QM31,
    trace_1_column_95_offset_0: QM31,
) -> QM31 {
    (trace_1_column_66_offset_0
        + trace_1_column_95_offset_0
        + intermediate32
        - (trace_1_column_37_offset_0))
        * (m31(4194304).into())
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
    VerifyInstruction_alpha6: QM31,
    VerifyInstruction_alpha9: QM31,
    VerifyInstruction_z: QM31,
    trace_1_column_0_offset_0: QM31,
    trace_1_column_3_offset_0: QM31,
    trace_1_column_4_offset_0: QM31,
    trace_1_column_5_offset_0: QM31,
    trace_1_column_6_offset_0: QM31,
    trace_1_column_7_offset_0: QM31,
) -> QM31 {
    (VerifyInstruction_alpha0) * (trace_1_column_0_offset_0)
        + (VerifyInstruction_alpha1) * (trace_1_column_3_offset_0)
        + (VerifyInstruction_alpha2) * (trace_1_column_4_offset_0)
        + (VerifyInstruction_alpha3) * (qm31(32769, 0, 0, 0))
        + (VerifyInstruction_alpha4) * (trace_1_column_5_offset_0)
        + (VerifyInstruction_alpha5) * (trace_1_column_6_offset_0)
        + VerifyInstruction_alpha6
        + VerifyInstruction_alpha9
        + (VerifyInstruction_alpha15) * (trace_1_column_7_offset_0)
        + VerifyInstruction_alpha18
        - (VerifyInstruction_z)
}

pub fn intermediate1(
    MemoryAddressToId_alpha0: QM31,
    MemoryAddressToId_alpha1: QM31,
    MemoryAddressToId_z: QM31,
    trace_1_column_10_offset_0: QM31,
    trace_1_column_3_offset_0: QM31,
    trace_1_column_8_offset_0: QM31,
) -> QM31 {
    (MemoryAddressToId_alpha0)
        * (trace_1_column_8_offset_0 + trace_1_column_3_offset_0 - (m31(32768).into()))
        + (MemoryAddressToId_alpha1) * (trace_1_column_10_offset_0)
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
    trace_1_column_28_offset_0: QM31,
    trace_1_column_29_offset_0: QM31,
    trace_1_column_30_offset_0: QM31,
    trace_1_column_31_offset_0: QM31,
    trace_1_column_32_offset_0: QM31,
    trace_1_column_33_offset_0: QM31,
    trace_1_column_34_offset_0: QM31,
    trace_1_column_35_offset_0: QM31,
    trace_1_column_36_offset_0: QM31,
    trace_1_column_37_offset_0: QM31,
    trace_1_column_38_offset_0: QM31,
) -> QM31 {
    (MemoryIdToBig_alpha0) * (trace_1_column_10_offset_0)
        + (MemoryIdToBig_alpha1) * (trace_1_column_11_offset_0)
        + (MemoryIdToBig_alpha2) * (trace_1_column_12_offset_0)
        + (MemoryIdToBig_alpha3) * (trace_1_column_13_offset_0)
        + (MemoryIdToBig_alpha4) * (trace_1_column_14_offset_0)
        + (MemoryIdToBig_alpha5) * (trace_1_column_15_offset_0)
        + (MemoryIdToBig_alpha6) * (trace_1_column_16_offset_0)
        + (MemoryIdToBig_alpha7) * (trace_1_column_17_offset_0)
        + (MemoryIdToBig_alpha8) * (trace_1_column_18_offset_0)
        + (MemoryIdToBig_alpha9) * (trace_1_column_19_offset_0)
        + (MemoryIdToBig_alpha10) * (trace_1_column_20_offset_0)
        + (MemoryIdToBig_alpha11) * (trace_1_column_21_offset_0)
        + (MemoryIdToBig_alpha12) * (trace_1_column_22_offset_0)
        + (MemoryIdToBig_alpha13) * (trace_1_column_23_offset_0)
        + (MemoryIdToBig_alpha14) * (trace_1_column_24_offset_0)
        + (MemoryIdToBig_alpha15) * (trace_1_column_25_offset_0)
        + (MemoryIdToBig_alpha16) * (trace_1_column_26_offset_0)
        + (MemoryIdToBig_alpha17) * (trace_1_column_27_offset_0)
        + (MemoryIdToBig_alpha18) * (trace_1_column_28_offset_0)
        + (MemoryIdToBig_alpha19) * (trace_1_column_29_offset_0)
        + (MemoryIdToBig_alpha20) * (trace_1_column_30_offset_0)
        + (MemoryIdToBig_alpha21) * (trace_1_column_31_offset_0)
        + (MemoryIdToBig_alpha22) * (trace_1_column_32_offset_0)
        + (MemoryIdToBig_alpha23) * (trace_1_column_33_offset_0)
        + (MemoryIdToBig_alpha24) * (trace_1_column_34_offset_0)
        + (MemoryIdToBig_alpha25) * (trace_1_column_35_offset_0)
        + (MemoryIdToBig_alpha26) * (trace_1_column_36_offset_0)
        + (MemoryIdToBig_alpha27) * (trace_1_column_37_offset_0)
        + (MemoryIdToBig_alpha28) * (trace_1_column_38_offset_0)
        - (MemoryIdToBig_z)
}

pub fn intermediate3(
    MemoryAddressToId_alpha0: QM31,
    MemoryAddressToId_alpha1: QM31,
    MemoryAddressToId_z: QM31,
    trace_1_column_39_offset_0: QM31,
    trace_1_column_4_offset_0: QM31,
    trace_1_column_9_offset_0: QM31,
) -> QM31 {
    (MemoryAddressToId_alpha0)
        * (trace_1_column_9_offset_0 + trace_1_column_4_offset_0 - (m31(32768).into()))
        + (MemoryAddressToId_alpha1) * (trace_1_column_39_offset_0)
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
    trace_1_column_39_offset_0: QM31,
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
    trace_1_column_50_offset_0: QM31,
    trace_1_column_51_offset_0: QM31,
    trace_1_column_52_offset_0: QM31,
    trace_1_column_53_offset_0: QM31,
    trace_1_column_54_offset_0: QM31,
    trace_1_column_55_offset_0: QM31,
    trace_1_column_56_offset_0: QM31,
    trace_1_column_57_offset_0: QM31,
    trace_1_column_58_offset_0: QM31,
    trace_1_column_59_offset_0: QM31,
    trace_1_column_60_offset_0: QM31,
    trace_1_column_61_offset_0: QM31,
    trace_1_column_62_offset_0: QM31,
    trace_1_column_63_offset_0: QM31,
    trace_1_column_64_offset_0: QM31,
    trace_1_column_65_offset_0: QM31,
    trace_1_column_66_offset_0: QM31,
    trace_1_column_67_offset_0: QM31,
) -> QM31 {
    (MemoryIdToBig_alpha0) * (trace_1_column_39_offset_0)
        + (MemoryIdToBig_alpha1) * (trace_1_column_40_offset_0)
        + (MemoryIdToBig_alpha2) * (trace_1_column_41_offset_0)
        + (MemoryIdToBig_alpha3) * (trace_1_column_42_offset_0)
        + (MemoryIdToBig_alpha4) * (trace_1_column_43_offset_0)
        + (MemoryIdToBig_alpha5) * (trace_1_column_44_offset_0)
        + (MemoryIdToBig_alpha6) * (trace_1_column_45_offset_0)
        + (MemoryIdToBig_alpha7) * (trace_1_column_46_offset_0)
        + (MemoryIdToBig_alpha8) * (trace_1_column_47_offset_0)
        + (MemoryIdToBig_alpha9) * (trace_1_column_48_offset_0)
        + (MemoryIdToBig_alpha10) * (trace_1_column_49_offset_0)
        + (MemoryIdToBig_alpha11) * (trace_1_column_50_offset_0)
        + (MemoryIdToBig_alpha12) * (trace_1_column_51_offset_0)
        + (MemoryIdToBig_alpha13) * (trace_1_column_52_offset_0)
        + (MemoryIdToBig_alpha14) * (trace_1_column_53_offset_0)
        + (MemoryIdToBig_alpha15) * (trace_1_column_54_offset_0)
        + (MemoryIdToBig_alpha16) * (trace_1_column_55_offset_0)
        + (MemoryIdToBig_alpha17) * (trace_1_column_56_offset_0)
        + (MemoryIdToBig_alpha18) * (trace_1_column_57_offset_0)
        + (MemoryIdToBig_alpha19) * (trace_1_column_58_offset_0)
        + (MemoryIdToBig_alpha20) * (trace_1_column_59_offset_0)
        + (MemoryIdToBig_alpha21) * (trace_1_column_60_offset_0)
        + (MemoryIdToBig_alpha22) * (trace_1_column_61_offset_0)
        + (MemoryIdToBig_alpha23) * (trace_1_column_62_offset_0)
        + (MemoryIdToBig_alpha24) * (trace_1_column_63_offset_0)
        + (MemoryIdToBig_alpha25) * (trace_1_column_64_offset_0)
        + (MemoryIdToBig_alpha26) * (trace_1_column_65_offset_0)
        + (MemoryIdToBig_alpha27) * (trace_1_column_66_offset_0)
        + (MemoryIdToBig_alpha28) * (trace_1_column_67_offset_0)
        - (MemoryIdToBig_z)
}

pub fn intermediate5(
    MemoryAddressToId_alpha0: QM31,
    MemoryAddressToId_alpha1: QM31,
    MemoryAddressToId_z: QM31,
    trace_1_column_0_offset_0: QM31,
    trace_1_column_68_offset_0: QM31,
) -> QM31 {
    (MemoryAddressToId_alpha0) * (trace_1_column_0_offset_0 + m31(1).into())
        + (MemoryAddressToId_alpha1) * (trace_1_column_68_offset_0)
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
    trace_1_column_68_offset_0: QM31,
    trace_1_column_69_offset_0: QM31,
    trace_1_column_70_offset_0: QM31,
    trace_1_column_71_offset_0: QM31,
    trace_1_column_72_offset_0: QM31,
    trace_1_column_73_offset_0: QM31,
    trace_1_column_74_offset_0: QM31,
    trace_1_column_75_offset_0: QM31,
    trace_1_column_76_offset_0: QM31,
    trace_1_column_77_offset_0: QM31,
    trace_1_column_78_offset_0: QM31,
    trace_1_column_79_offset_0: QM31,
    trace_1_column_80_offset_0: QM31,
    trace_1_column_81_offset_0: QM31,
    trace_1_column_82_offset_0: QM31,
    trace_1_column_83_offset_0: QM31,
    trace_1_column_84_offset_0: QM31,
    trace_1_column_85_offset_0: QM31,
    trace_1_column_86_offset_0: QM31,
    trace_1_column_87_offset_0: QM31,
    trace_1_column_88_offset_0: QM31,
    trace_1_column_89_offset_0: QM31,
    trace_1_column_90_offset_0: QM31,
    trace_1_column_91_offset_0: QM31,
    trace_1_column_92_offset_0: QM31,
    trace_1_column_93_offset_0: QM31,
    trace_1_column_94_offset_0: QM31,
    trace_1_column_95_offset_0: QM31,
    trace_1_column_96_offset_0: QM31,
) -> QM31 {
    (MemoryIdToBig_alpha0) * (trace_1_column_68_offset_0)
        + (MemoryIdToBig_alpha1) * (trace_1_column_69_offset_0)
        + (MemoryIdToBig_alpha2) * (trace_1_column_70_offset_0)
        + (MemoryIdToBig_alpha3) * (trace_1_column_71_offset_0)
        + (MemoryIdToBig_alpha4) * (trace_1_column_72_offset_0)
        + (MemoryIdToBig_alpha5) * (trace_1_column_73_offset_0)
        + (MemoryIdToBig_alpha6) * (trace_1_column_74_offset_0)
        + (MemoryIdToBig_alpha7) * (trace_1_column_75_offset_0)
        + (MemoryIdToBig_alpha8) * (trace_1_column_76_offset_0)
        + (MemoryIdToBig_alpha9) * (trace_1_column_77_offset_0)
        + (MemoryIdToBig_alpha10) * (trace_1_column_78_offset_0)
        + (MemoryIdToBig_alpha11) * (trace_1_column_79_offset_0)
        + (MemoryIdToBig_alpha12) * (trace_1_column_80_offset_0)
        + (MemoryIdToBig_alpha13) * (trace_1_column_81_offset_0)
        + (MemoryIdToBig_alpha14) * (trace_1_column_82_offset_0)
        + (MemoryIdToBig_alpha15) * (trace_1_column_83_offset_0)
        + (MemoryIdToBig_alpha16) * (trace_1_column_84_offset_0)
        + (MemoryIdToBig_alpha17) * (trace_1_column_85_offset_0)
        + (MemoryIdToBig_alpha18) * (trace_1_column_86_offset_0)
        + (MemoryIdToBig_alpha19) * (trace_1_column_87_offset_0)
        + (MemoryIdToBig_alpha20) * (trace_1_column_88_offset_0)
        + (MemoryIdToBig_alpha21) * (trace_1_column_89_offset_0)
        + (MemoryIdToBig_alpha22) * (trace_1_column_90_offset_0)
        + (MemoryIdToBig_alpha23) * (trace_1_column_91_offset_0)
        + (MemoryIdToBig_alpha24) * (trace_1_column_92_offset_0)
        + (MemoryIdToBig_alpha25) * (trace_1_column_93_offset_0)
        + (MemoryIdToBig_alpha26) * (trace_1_column_94_offset_0)
        + (MemoryIdToBig_alpha27) * (trace_1_column_95_offset_0)
        + (MemoryIdToBig_alpha28) * (trace_1_column_96_offset_0)
        - (MemoryIdToBig_z)
}

pub fn intermediate34(
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

pub fn intermediate35(
    Opcodes_alpha0: QM31,
    Opcodes_alpha1: QM31,
    Opcodes_alpha2: QM31,
    Opcodes_z: QM31,
    trace_1_column_0_offset_0: QM31,
    trace_1_column_1_offset_0: QM31,
    trace_1_column_2_offset_0: QM31,
    trace_1_column_7_offset_0: QM31,
) -> QM31 {
    (Opcodes_alpha0) * (trace_1_column_0_offset_0 + m31(2).into())
        + (Opcodes_alpha1) * (trace_1_column_1_offset_0 + trace_1_column_7_offset_0)
        + (Opcodes_alpha2) * (trace_1_column_2_offset_0)
        - (Opcodes_z)
}

