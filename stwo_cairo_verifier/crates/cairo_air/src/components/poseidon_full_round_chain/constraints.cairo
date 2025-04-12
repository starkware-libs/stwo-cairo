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
    pub Cube252_alpha0: QM31,
    pub Cube252_alpha1: QM31,
    pub Cube252_alpha10: QM31,
    pub Cube252_alpha11: QM31,
    pub Cube252_alpha12: QM31,
    pub Cube252_alpha13: QM31,
    pub Cube252_alpha14: QM31,
    pub Cube252_alpha15: QM31,
    pub Cube252_alpha16: QM31,
    pub Cube252_alpha17: QM31,
    pub Cube252_alpha18: QM31,
    pub Cube252_alpha19: QM31,
    pub Cube252_alpha2: QM31,
    pub Cube252_alpha3: QM31,
    pub Cube252_alpha4: QM31,
    pub Cube252_alpha5: QM31,
    pub Cube252_alpha6: QM31,
    pub Cube252_alpha7: QM31,
    pub Cube252_alpha8: QM31,
    pub Cube252_alpha9: QM31,
    pub Cube252_z: QM31,
    pub PoseidonFullRoundChain_alpha0: QM31,
    pub PoseidonFullRoundChain_alpha1: QM31,
    pub PoseidonFullRoundChain_alpha10: QM31,
    pub PoseidonFullRoundChain_alpha11: QM31,
    pub PoseidonFullRoundChain_alpha12: QM31,
    pub PoseidonFullRoundChain_alpha13: QM31,
    pub PoseidonFullRoundChain_alpha14: QM31,
    pub PoseidonFullRoundChain_alpha15: QM31,
    pub PoseidonFullRoundChain_alpha16: QM31,
    pub PoseidonFullRoundChain_alpha17: QM31,
    pub PoseidonFullRoundChain_alpha18: QM31,
    pub PoseidonFullRoundChain_alpha19: QM31,
    pub PoseidonFullRoundChain_alpha2: QM31,
    pub PoseidonFullRoundChain_alpha20: QM31,
    pub PoseidonFullRoundChain_alpha21: QM31,
    pub PoseidonFullRoundChain_alpha22: QM31,
    pub PoseidonFullRoundChain_alpha23: QM31,
    pub PoseidonFullRoundChain_alpha24: QM31,
    pub PoseidonFullRoundChain_alpha25: QM31,
    pub PoseidonFullRoundChain_alpha26: QM31,
    pub PoseidonFullRoundChain_alpha27: QM31,
    pub PoseidonFullRoundChain_alpha28: QM31,
    pub PoseidonFullRoundChain_alpha29: QM31,
    pub PoseidonFullRoundChain_alpha3: QM31,
    pub PoseidonFullRoundChain_alpha30: QM31,
    pub PoseidonFullRoundChain_alpha31: QM31,
    pub PoseidonFullRoundChain_alpha4: QM31,
    pub PoseidonFullRoundChain_alpha5: QM31,
    pub PoseidonFullRoundChain_alpha6: QM31,
    pub PoseidonFullRoundChain_alpha7: QM31,
    pub PoseidonFullRoundChain_alpha8: QM31,
    pub PoseidonFullRoundChain_alpha9: QM31,
    pub PoseidonFullRoundChain_z: QM31,
    pub PoseidonRoundKeys_alpha0: QM31,
    pub PoseidonRoundKeys_alpha1: QM31,
    pub PoseidonRoundKeys_alpha10: QM31,
    pub PoseidonRoundKeys_alpha11: QM31,
    pub PoseidonRoundKeys_alpha12: QM31,
    pub PoseidonRoundKeys_alpha13: QM31,
    pub PoseidonRoundKeys_alpha14: QM31,
    pub PoseidonRoundKeys_alpha15: QM31,
    pub PoseidonRoundKeys_alpha16: QM31,
    pub PoseidonRoundKeys_alpha17: QM31,
    pub PoseidonRoundKeys_alpha18: QM31,
    pub PoseidonRoundKeys_alpha19: QM31,
    pub PoseidonRoundKeys_alpha2: QM31,
    pub PoseidonRoundKeys_alpha20: QM31,
    pub PoseidonRoundKeys_alpha21: QM31,
    pub PoseidonRoundKeys_alpha22: QM31,
    pub PoseidonRoundKeys_alpha23: QM31,
    pub PoseidonRoundKeys_alpha24: QM31,
    pub PoseidonRoundKeys_alpha25: QM31,
    pub PoseidonRoundKeys_alpha26: QM31,
    pub PoseidonRoundKeys_alpha27: QM31,
    pub PoseidonRoundKeys_alpha28: QM31,
    pub PoseidonRoundKeys_alpha29: QM31,
    pub PoseidonRoundKeys_alpha3: QM31,
    pub PoseidonRoundKeys_alpha30: QM31,
    pub PoseidonRoundKeys_alpha4: QM31,
    pub PoseidonRoundKeys_alpha5: QM31,
    pub PoseidonRoundKeys_alpha6: QM31,
    pub PoseidonRoundKeys_alpha7: QM31,
    pub PoseidonRoundKeys_alpha8: QM31,
    pub PoseidonRoundKeys_alpha9: QM31,
    pub PoseidonRoundKeys_z: QM31,
    pub RangeCheck_3_3_3_3_3_alpha0: QM31,
    pub RangeCheck_3_3_3_3_3_alpha1: QM31,
    pub RangeCheck_3_3_3_3_3_alpha2: QM31,
    pub RangeCheck_3_3_3_3_3_alpha3: QM31,
    pub RangeCheck_3_3_3_3_3_alpha4: QM31,
    pub RangeCheck_3_3_3_3_3_z: QM31,
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
        Cube252_alpha0,
        Cube252_alpha1,
        Cube252_alpha10,
        Cube252_alpha11,
        Cube252_alpha12,
        Cube252_alpha13,
        Cube252_alpha14,
        Cube252_alpha15,
        Cube252_alpha16,
        Cube252_alpha17,
        Cube252_alpha18,
        Cube252_alpha19,
        Cube252_alpha2,
        Cube252_alpha3,
        Cube252_alpha4,
        Cube252_alpha5,
        Cube252_alpha6,
        Cube252_alpha7,
        Cube252_alpha8,
        Cube252_alpha9,
        Cube252_z,
        PoseidonFullRoundChain_alpha0,
        PoseidonFullRoundChain_alpha1,
        PoseidonFullRoundChain_alpha10,
        PoseidonFullRoundChain_alpha11,
        PoseidonFullRoundChain_alpha12,
        PoseidonFullRoundChain_alpha13,
        PoseidonFullRoundChain_alpha14,
        PoseidonFullRoundChain_alpha15,
        PoseidonFullRoundChain_alpha16,
        PoseidonFullRoundChain_alpha17,
        PoseidonFullRoundChain_alpha18,
        PoseidonFullRoundChain_alpha19,
        PoseidonFullRoundChain_alpha2,
        PoseidonFullRoundChain_alpha20,
        PoseidonFullRoundChain_alpha21,
        PoseidonFullRoundChain_alpha22,
        PoseidonFullRoundChain_alpha23,
        PoseidonFullRoundChain_alpha24,
        PoseidonFullRoundChain_alpha25,
        PoseidonFullRoundChain_alpha26,
        PoseidonFullRoundChain_alpha27,
        PoseidonFullRoundChain_alpha28,
        PoseidonFullRoundChain_alpha29,
        PoseidonFullRoundChain_alpha3,
        PoseidonFullRoundChain_alpha30,
        PoseidonFullRoundChain_alpha31,
        PoseidonFullRoundChain_alpha4,
        PoseidonFullRoundChain_alpha5,
        PoseidonFullRoundChain_alpha6,
        PoseidonFullRoundChain_alpha7,
        PoseidonFullRoundChain_alpha8,
        PoseidonFullRoundChain_alpha9,
        PoseidonFullRoundChain_z,
        PoseidonRoundKeys_alpha0,
        PoseidonRoundKeys_alpha1,
        PoseidonRoundKeys_alpha10,
        PoseidonRoundKeys_alpha11,
        PoseidonRoundKeys_alpha12,
        PoseidonRoundKeys_alpha13,
        PoseidonRoundKeys_alpha14,
        PoseidonRoundKeys_alpha15,
        PoseidonRoundKeys_alpha16,
        PoseidonRoundKeys_alpha17,
        PoseidonRoundKeys_alpha18,
        PoseidonRoundKeys_alpha19,
        PoseidonRoundKeys_alpha2,
        PoseidonRoundKeys_alpha20,
        PoseidonRoundKeys_alpha21,
        PoseidonRoundKeys_alpha22,
        PoseidonRoundKeys_alpha23,
        PoseidonRoundKeys_alpha24,
        PoseidonRoundKeys_alpha25,
        PoseidonRoundKeys_alpha26,
        PoseidonRoundKeys_alpha27,
        PoseidonRoundKeys_alpha28,
        PoseidonRoundKeys_alpha29,
        PoseidonRoundKeys_alpha3,
        PoseidonRoundKeys_alpha30,
        PoseidonRoundKeys_alpha4,
        PoseidonRoundKeys_alpha5,
        PoseidonRoundKeys_alpha6,
        PoseidonRoundKeys_alpha7,
        PoseidonRoundKeys_alpha8,
        PoseidonRoundKeys_alpha9,
        PoseidonRoundKeys_z,
        RangeCheck_3_3_3_3_3_alpha0,
        RangeCheck_3_3_3_3_3_alpha1,
        RangeCheck_3_3_3_3_3_alpha2,
        RangeCheck_3_3_3_3_3_alpha3,
        RangeCheck_3_3_3_3_3_alpha4,
        RangeCheck_3_3_3_3_3_z,
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
        trace_1_column_99,
        trace_1_column_100,
        trace_1_column_101,
        trace_1_column_102,
        trace_1_column_103,
        trace_1_column_104,
        trace_1_column_105,
        trace_1_column_106,
        trace_1_column_107,
        trace_1_column_108,
        trace_1_column_109,
        trace_1_column_110,
        trace_1_column_111,
        trace_1_column_112,
        trace_1_column_113,
        trace_1_column_114,
        trace_1_column_115,
        trace_1_column_116,
        trace_1_column_117,
        trace_1_column_118,
        trace_1_column_119,
        trace_1_column_120,
        trace_1_column_121,
        trace_1_column_122,
        trace_1_column_123,
        trace_1_column_124,
        trace_1_column_125,
    ]: [Span<QM31>; 126] =
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

    let [trace_1_column_99_offset_0]: [QM31; 1] = (*trace_1_column_99.try_into().unwrap()).unbox();

    let [trace_1_column_100_offset_0]: [QM31; 1] = (*trace_1_column_100.try_into().unwrap())
        .unbox();

    let [trace_1_column_101_offset_0]: [QM31; 1] = (*trace_1_column_101.try_into().unwrap())
        .unbox();

    let [trace_1_column_102_offset_0]: [QM31; 1] = (*trace_1_column_102.try_into().unwrap())
        .unbox();

    let [trace_1_column_103_offset_0]: [QM31; 1] = (*trace_1_column_103.try_into().unwrap())
        .unbox();

    let [trace_1_column_104_offset_0]: [QM31; 1] = (*trace_1_column_104.try_into().unwrap())
        .unbox();

    let [trace_1_column_105_offset_0]: [QM31; 1] = (*trace_1_column_105.try_into().unwrap())
        .unbox();

    let [trace_1_column_106_offset_0]: [QM31; 1] = (*trace_1_column_106.try_into().unwrap())
        .unbox();

    let [trace_1_column_107_offset_0]: [QM31; 1] = (*trace_1_column_107.try_into().unwrap())
        .unbox();

    let [trace_1_column_108_offset_0]: [QM31; 1] = (*trace_1_column_108.try_into().unwrap())
        .unbox();

    let [trace_1_column_109_offset_0]: [QM31; 1] = (*trace_1_column_109.try_into().unwrap())
        .unbox();

    let [trace_1_column_110_offset_0]: [QM31; 1] = (*trace_1_column_110.try_into().unwrap())
        .unbox();

    let [trace_1_column_111_offset_0]: [QM31; 1] = (*trace_1_column_111.try_into().unwrap())
        .unbox();

    let [trace_1_column_112_offset_0]: [QM31; 1] = (*trace_1_column_112.try_into().unwrap())
        .unbox();

    let [trace_1_column_113_offset_0]: [QM31; 1] = (*trace_1_column_113.try_into().unwrap())
        .unbox();

    let [trace_1_column_114_offset_0]: [QM31; 1] = (*trace_1_column_114.try_into().unwrap())
        .unbox();

    let [trace_1_column_115_offset_0]: [QM31; 1] = (*trace_1_column_115.try_into().unwrap())
        .unbox();

    let [trace_1_column_116_offset_0]: [QM31; 1] = (*trace_1_column_116.try_into().unwrap())
        .unbox();

    let [trace_1_column_117_offset_0]: [QM31; 1] = (*trace_1_column_117.try_into().unwrap())
        .unbox();

    let [trace_1_column_118_offset_0]: [QM31; 1] = (*trace_1_column_118.try_into().unwrap())
        .unbox();

    let [trace_1_column_119_offset_0]: [QM31; 1] = (*trace_1_column_119.try_into().unwrap())
        .unbox();

    let [trace_1_column_120_offset_0]: [QM31; 1] = (*trace_1_column_120.try_into().unwrap())
        .unbox();

    let [trace_1_column_121_offset_0]: [QM31; 1] = (*trace_1_column_121.try_into().unwrap())
        .unbox();

    let [trace_1_column_122_offset_0]: [QM31; 1] = (*trace_1_column_122.try_into().unwrap())
        .unbox();

    let [trace_1_column_123_offset_0]: [QM31; 1] = (*trace_1_column_123.try_into().unwrap())
        .unbox();

    let [trace_1_column_124_offset_0]: [QM31; 1] = (*trace_1_column_124.try_into().unwrap())
        .unbox();

    let [trace_1_column_125_offset_0]: [QM31; 1] = (*trace_1_column_125.try_into().unwrap())
        .unbox();

    let [
        trace_2_column_126,
        trace_2_column_127,
        trace_2_column_128,
        trace_2_column_129,
        trace_2_column_130,
        trace_2_column_131,
        trace_2_column_132,
        trace_2_column_133,
        trace_2_column_134,
        trace_2_column_135,
        trace_2_column_136,
        trace_2_column_137,
        trace_2_column_138,
        trace_2_column_139,
        trace_2_column_140,
        trace_2_column_141,
        trace_2_column_142,
        trace_2_column_143,
        trace_2_column_144,
        trace_2_column_145,
        trace_2_column_146,
        trace_2_column_147,
        trace_2_column_148,
        trace_2_column_149,
    ]: [Span<QM31>; 24] =
        (*interaction_mask_values
        .multi_pop_front()
        .unwrap())
        .unbox();

    let [trace_2_column_126_offset_0]: [QM31; 1] = (*trace_2_column_126.try_into().unwrap())
        .unbox();

    let [trace_2_column_127_offset_0]: [QM31; 1] = (*trace_2_column_127.try_into().unwrap())
        .unbox();

    let [trace_2_column_128_offset_0]: [QM31; 1] = (*trace_2_column_128.try_into().unwrap())
        .unbox();

    let [trace_2_column_129_offset_0]: [QM31; 1] = (*trace_2_column_129.try_into().unwrap())
        .unbox();

    let [trace_2_column_130_offset_0]: [QM31; 1] = (*trace_2_column_130.try_into().unwrap())
        .unbox();

    let [trace_2_column_131_offset_0]: [QM31; 1] = (*trace_2_column_131.try_into().unwrap())
        .unbox();

    let [trace_2_column_132_offset_0]: [QM31; 1] = (*trace_2_column_132.try_into().unwrap())
        .unbox();

    let [trace_2_column_133_offset_0]: [QM31; 1] = (*trace_2_column_133.try_into().unwrap())
        .unbox();

    let [trace_2_column_134_offset_0]: [QM31; 1] = (*trace_2_column_134.try_into().unwrap())
        .unbox();

    let [trace_2_column_135_offset_0]: [QM31; 1] = (*trace_2_column_135.try_into().unwrap())
        .unbox();

    let [trace_2_column_136_offset_0]: [QM31; 1] = (*trace_2_column_136.try_into().unwrap())
        .unbox();

    let [trace_2_column_137_offset_0]: [QM31; 1] = (*trace_2_column_137.try_into().unwrap())
        .unbox();

    let [trace_2_column_138_offset_0]: [QM31; 1] = (*trace_2_column_138.try_into().unwrap())
        .unbox();

    let [trace_2_column_139_offset_0]: [QM31; 1] = (*trace_2_column_139.try_into().unwrap())
        .unbox();

    let [trace_2_column_140_offset_0]: [QM31; 1] = (*trace_2_column_140.try_into().unwrap())
        .unbox();

    let [trace_2_column_141_offset_0]: [QM31; 1] = (*trace_2_column_141.try_into().unwrap())
        .unbox();

    let [trace_2_column_142_offset_0]: [QM31; 1] = (*trace_2_column_142.try_into().unwrap())
        .unbox();

    let [trace_2_column_143_offset_0]: [QM31; 1] = (*trace_2_column_143.try_into().unwrap())
        .unbox();

    let [trace_2_column_144_offset_0]: [QM31; 1] = (*trace_2_column_144.try_into().unwrap())
        .unbox();

    let [trace_2_column_145_offset_0]: [QM31; 1] = (*trace_2_column_145.try_into().unwrap())
        .unbox();

    let [trace_2_column_146_offset_neg_1, trace_2_column_146_offset_0]: [QM31; 2] =
        (*trace_2_column_146
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_147_offset_neg_1, trace_2_column_147_offset_0]: [QM31; 2] =
        (*trace_2_column_147
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_148_offset_neg_1, trace_2_column_148_offset_0]: [QM31; 2] =
        (*trace_2_column_148
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_149_offset_neg_1, trace_2_column_149_offset_0]: [QM31; 2] =
        (*trace_2_column_149
        .try_into()
        .unwrap())
        .unbox();

    core::internal::revoke_ap_tracking();

    let mut intermediates = intermediates(
        Cube252_alpha0,
        Cube252_alpha1,
        Cube252_alpha10,
        Cube252_alpha11,
        Cube252_alpha12,
        Cube252_alpha13,
        Cube252_alpha14,
        Cube252_alpha15,
        Cube252_alpha16,
        Cube252_alpha17,
        Cube252_alpha18,
        Cube252_alpha19,
        Cube252_alpha2,
        Cube252_alpha3,
        Cube252_alpha4,
        Cube252_alpha5,
        Cube252_alpha6,
        Cube252_alpha7,
        Cube252_alpha8,
        Cube252_alpha9,
        Cube252_z,
        PoseidonFullRoundChain_alpha0,
        PoseidonFullRoundChain_alpha1,
        PoseidonFullRoundChain_alpha10,
        PoseidonFullRoundChain_alpha11,
        PoseidonFullRoundChain_alpha12,
        PoseidonFullRoundChain_alpha13,
        PoseidonFullRoundChain_alpha14,
        PoseidonFullRoundChain_alpha15,
        PoseidonFullRoundChain_alpha16,
        PoseidonFullRoundChain_alpha17,
        PoseidonFullRoundChain_alpha18,
        PoseidonFullRoundChain_alpha19,
        PoseidonFullRoundChain_alpha2,
        PoseidonFullRoundChain_alpha20,
        PoseidonFullRoundChain_alpha21,
        PoseidonFullRoundChain_alpha22,
        PoseidonFullRoundChain_alpha23,
        PoseidonFullRoundChain_alpha24,
        PoseidonFullRoundChain_alpha25,
        PoseidonFullRoundChain_alpha26,
        PoseidonFullRoundChain_alpha27,
        PoseidonFullRoundChain_alpha28,
        PoseidonFullRoundChain_alpha29,
        PoseidonFullRoundChain_alpha3,
        PoseidonFullRoundChain_alpha30,
        PoseidonFullRoundChain_alpha31,
        PoseidonFullRoundChain_alpha4,
        PoseidonFullRoundChain_alpha5,
        PoseidonFullRoundChain_alpha6,
        PoseidonFullRoundChain_alpha7,
        PoseidonFullRoundChain_alpha8,
        PoseidonFullRoundChain_alpha9,
        PoseidonFullRoundChain_z,
        PoseidonRoundKeys_alpha0,
        PoseidonRoundKeys_alpha1,
        PoseidonRoundKeys_alpha10,
        PoseidonRoundKeys_alpha11,
        PoseidonRoundKeys_alpha12,
        PoseidonRoundKeys_alpha13,
        PoseidonRoundKeys_alpha14,
        PoseidonRoundKeys_alpha15,
        PoseidonRoundKeys_alpha16,
        PoseidonRoundKeys_alpha17,
        PoseidonRoundKeys_alpha18,
        PoseidonRoundKeys_alpha19,
        PoseidonRoundKeys_alpha2,
        PoseidonRoundKeys_alpha20,
        PoseidonRoundKeys_alpha21,
        PoseidonRoundKeys_alpha22,
        PoseidonRoundKeys_alpha23,
        PoseidonRoundKeys_alpha24,
        PoseidonRoundKeys_alpha25,
        PoseidonRoundKeys_alpha26,
        PoseidonRoundKeys_alpha27,
        PoseidonRoundKeys_alpha28,
        PoseidonRoundKeys_alpha29,
        PoseidonRoundKeys_alpha3,
        PoseidonRoundKeys_alpha30,
        PoseidonRoundKeys_alpha4,
        PoseidonRoundKeys_alpha5,
        PoseidonRoundKeys_alpha6,
        PoseidonRoundKeys_alpha7,
        PoseidonRoundKeys_alpha8,
        PoseidonRoundKeys_alpha9,
        PoseidonRoundKeys_z,
        RangeCheck_3_3_3_3_3_alpha0,
        RangeCheck_3_3_3_3_3_alpha1,
        RangeCheck_3_3_3_3_3_alpha2,
        RangeCheck_3_3_3_3_3_alpha3,
        RangeCheck_3_3_3_3_3_alpha4,
        RangeCheck_3_3_3_3_3_z,
        trace_1_column_0_offset_0,
        trace_1_column_100_offset_0,
        trace_1_column_101_offset_0,
        trace_1_column_102_offset_0,
        trace_1_column_103_offset_0,
        trace_1_column_104_offset_0,
        trace_1_column_105_offset_0,
        trace_1_column_106_offset_0,
        trace_1_column_107_offset_0,
        trace_1_column_108_offset_0,
        trace_1_column_109_offset_0,
        trace_1_column_10_offset_0,
        trace_1_column_110_offset_0,
        trace_1_column_111_offset_0,
        trace_1_column_112_offset_0,
        trace_1_column_113_offset_0,
        trace_1_column_114_offset_0,
        trace_1_column_115_offset_0,
        trace_1_column_116_offset_0,
        trace_1_column_117_offset_0,
        trace_1_column_118_offset_0,
        trace_1_column_119_offset_0,
        trace_1_column_11_offset_0,
        trace_1_column_120_offset_0,
        trace_1_column_121_offset_0,
        trace_1_column_122_offset_0,
        trace_1_column_123_offset_0,
        trace_1_column_124_offset_0,
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
        trace_1_column_98_offset_0,
        trace_1_column_99_offset_0,
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
    let intermediate36 = *intermediates.pop_front().unwrap();
    let intermediate37 = *intermediates.pop_front().unwrap();
    let intermediate38 = *intermediates.pop_front().unwrap();

    // Constraint 0
    let constraint_quotient = ((trace_1_column_125_offset_0) * (trace_1_column_125_offset_0)
        - (trace_1_column_125_offset_0))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 1
    let constraint_quotient = (intermediate12
        + (m31(3).into()) * (trace_1_column_41_offset_0)
        + trace_1_column_51_offset_0
        + trace_1_column_61_offset_0
        + trace_1_column_71_offset_0
        - (trace_1_column_101_offset_0)
        - ((trace_1_column_102_offset_0) * (m31(256).into())))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 2
    let constraint_quotient = (intermediate23
        + trace_1_column_41_offset_0
        - (trace_1_column_51_offset_0)
        + trace_1_column_61_offset_0
        + trace_1_column_81_offset_0
        - (trace_1_column_112_offset_0)
        - ((trace_1_column_113_offset_0) * (m31(256).into())))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 3
    let constraint_quotient = (intermediate34
        + trace_1_column_41_offset_0
        + trace_1_column_51_offset_0
        - ((m31(2).into()) * (trace_1_column_61_offset_0))
        + trace_1_column_91_offset_0
        - (trace_1_column_123_offset_0)
        - ((trace_1_column_124_offset_0) * (m31(256).into())))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 4
    let constraint_quotient = ((QM31Trait::from_partial_evals(
        [
            trace_2_column_126_offset_0, trace_2_column_127_offset_0, trace_2_column_128_offset_0,
            trace_2_column_129_offset_0,
        ],
    ))
        * ((intermediate0) * (intermediate1))
        - (intermediate1 + intermediate0))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 5
    let constraint_quotient = ((QM31Trait::from_partial_evals(
        [
            trace_2_column_130_offset_0, trace_2_column_131_offset_0, trace_2_column_132_offset_0,
            trace_2_column_133_offset_0,
        ],
    )
        - (QM31Trait::from_partial_evals(
            [
                trace_2_column_126_offset_0, trace_2_column_127_offset_0,
                trace_2_column_128_offset_0, trace_2_column_129_offset_0,
            ],
        )))
        * ((intermediate2) * (intermediate3))
        - (intermediate3 + intermediate2))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 6
    let constraint_quotient = ((QM31Trait::from_partial_evals(
        [
            trace_2_column_134_offset_0, trace_2_column_135_offset_0, trace_2_column_136_offset_0,
            trace_2_column_137_offset_0,
        ],
    )
        - (QM31Trait::from_partial_evals(
            [
                trace_2_column_130_offset_0, trace_2_column_131_offset_0,
                trace_2_column_132_offset_0, trace_2_column_133_offset_0,
            ],
        )))
        * ((intermediate13) * (intermediate14))
        - (intermediate14 + intermediate13))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 7
    let constraint_quotient = ((QM31Trait::from_partial_evals(
        [
            trace_2_column_138_offset_0, trace_2_column_139_offset_0, trace_2_column_140_offset_0,
            trace_2_column_141_offset_0,
        ],
    )
        - (QM31Trait::from_partial_evals(
            [
                trace_2_column_134_offset_0, trace_2_column_135_offset_0,
                trace_2_column_136_offset_0, trace_2_column_137_offset_0,
            ],
        )))
        * ((intermediate24) * (intermediate25))
        - (intermediate25 + intermediate24))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 8
    let constraint_quotient = ((QM31Trait::from_partial_evals(
        [
            trace_2_column_142_offset_0, trace_2_column_143_offset_0, trace_2_column_144_offset_0,
            trace_2_column_145_offset_0,
        ],
    )
        - (QM31Trait::from_partial_evals(
            [
                trace_2_column_138_offset_0, trace_2_column_139_offset_0,
                trace_2_column_140_offset_0, trace_2_column_141_offset_0,
            ],
        )))
        * ((intermediate35) * (intermediate36))
        - (intermediate36 + intermediate35))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 9
    let constraint_quotient = ((QM31Trait::from_partial_evals(
        [
            trace_2_column_146_offset_0, trace_2_column_147_offset_0, trace_2_column_148_offset_0,
            trace_2_column_149_offset_0,
        ],
    )
        - (QM31Trait::from_partial_evals(
            [
                trace_2_column_146_offset_neg_1, trace_2_column_147_offset_neg_1,
                trace_2_column_148_offset_neg_1, trace_2_column_149_offset_neg_1,
            ],
        ))
        - (QM31Trait::from_partial_evals(
            [
                trace_2_column_142_offset_0, trace_2_column_143_offset_0,
                trace_2_column_144_offset_0, trace_2_column_145_offset_0,
            ],
        ))
        + (claimed_sum) * (column_size.inverse().into()))
        * ((intermediate37) * (intermediate38))
        - ((intermediate38) * (trace_1_column_125_offset_0)
            - ((intermediate37) * (trace_1_column_125_offset_0))))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;
}


fn intermediates(
    Cube252_alpha0: QM31,
    Cube252_alpha1: QM31,
    Cube252_alpha10: QM31,
    Cube252_alpha11: QM31,
    Cube252_alpha12: QM31,
    Cube252_alpha13: QM31,
    Cube252_alpha14: QM31,
    Cube252_alpha15: QM31,
    Cube252_alpha16: QM31,
    Cube252_alpha17: QM31,
    Cube252_alpha18: QM31,
    Cube252_alpha19: QM31,
    Cube252_alpha2: QM31,
    Cube252_alpha3: QM31,
    Cube252_alpha4: QM31,
    Cube252_alpha5: QM31,
    Cube252_alpha6: QM31,
    Cube252_alpha7: QM31,
    Cube252_alpha8: QM31,
    Cube252_alpha9: QM31,
    Cube252_z: QM31,
    PoseidonFullRoundChain_alpha0: QM31,
    PoseidonFullRoundChain_alpha1: QM31,
    PoseidonFullRoundChain_alpha10: QM31,
    PoseidonFullRoundChain_alpha11: QM31,
    PoseidonFullRoundChain_alpha12: QM31,
    PoseidonFullRoundChain_alpha13: QM31,
    PoseidonFullRoundChain_alpha14: QM31,
    PoseidonFullRoundChain_alpha15: QM31,
    PoseidonFullRoundChain_alpha16: QM31,
    PoseidonFullRoundChain_alpha17: QM31,
    PoseidonFullRoundChain_alpha18: QM31,
    PoseidonFullRoundChain_alpha19: QM31,
    PoseidonFullRoundChain_alpha2: QM31,
    PoseidonFullRoundChain_alpha20: QM31,
    PoseidonFullRoundChain_alpha21: QM31,
    PoseidonFullRoundChain_alpha22: QM31,
    PoseidonFullRoundChain_alpha23: QM31,
    PoseidonFullRoundChain_alpha24: QM31,
    PoseidonFullRoundChain_alpha25: QM31,
    PoseidonFullRoundChain_alpha26: QM31,
    PoseidonFullRoundChain_alpha27: QM31,
    PoseidonFullRoundChain_alpha28: QM31,
    PoseidonFullRoundChain_alpha29: QM31,
    PoseidonFullRoundChain_alpha3: QM31,
    PoseidonFullRoundChain_alpha30: QM31,
    PoseidonFullRoundChain_alpha31: QM31,
    PoseidonFullRoundChain_alpha4: QM31,
    PoseidonFullRoundChain_alpha5: QM31,
    PoseidonFullRoundChain_alpha6: QM31,
    PoseidonFullRoundChain_alpha7: QM31,
    PoseidonFullRoundChain_alpha8: QM31,
    PoseidonFullRoundChain_alpha9: QM31,
    PoseidonFullRoundChain_z: QM31,
    PoseidonRoundKeys_alpha0: QM31,
    PoseidonRoundKeys_alpha1: QM31,
    PoseidonRoundKeys_alpha10: QM31,
    PoseidonRoundKeys_alpha11: QM31,
    PoseidonRoundKeys_alpha12: QM31,
    PoseidonRoundKeys_alpha13: QM31,
    PoseidonRoundKeys_alpha14: QM31,
    PoseidonRoundKeys_alpha15: QM31,
    PoseidonRoundKeys_alpha16: QM31,
    PoseidonRoundKeys_alpha17: QM31,
    PoseidonRoundKeys_alpha18: QM31,
    PoseidonRoundKeys_alpha19: QM31,
    PoseidonRoundKeys_alpha2: QM31,
    PoseidonRoundKeys_alpha20: QM31,
    PoseidonRoundKeys_alpha21: QM31,
    PoseidonRoundKeys_alpha22: QM31,
    PoseidonRoundKeys_alpha23: QM31,
    PoseidonRoundKeys_alpha24: QM31,
    PoseidonRoundKeys_alpha25: QM31,
    PoseidonRoundKeys_alpha26: QM31,
    PoseidonRoundKeys_alpha27: QM31,
    PoseidonRoundKeys_alpha28: QM31,
    PoseidonRoundKeys_alpha29: QM31,
    PoseidonRoundKeys_alpha3: QM31,
    PoseidonRoundKeys_alpha30: QM31,
    PoseidonRoundKeys_alpha4: QM31,
    PoseidonRoundKeys_alpha5: QM31,
    PoseidonRoundKeys_alpha6: QM31,
    PoseidonRoundKeys_alpha7: QM31,
    PoseidonRoundKeys_alpha8: QM31,
    PoseidonRoundKeys_alpha9: QM31,
    PoseidonRoundKeys_z: QM31,
    RangeCheck_3_3_3_3_3_alpha0: QM31,
    RangeCheck_3_3_3_3_3_alpha1: QM31,
    RangeCheck_3_3_3_3_3_alpha2: QM31,
    RangeCheck_3_3_3_3_3_alpha3: QM31,
    RangeCheck_3_3_3_3_3_alpha4: QM31,
    RangeCheck_3_3_3_3_3_z: QM31,
    trace_1_column_0_offset_0: QM31,
    trace_1_column_100_offset_0: QM31,
    trace_1_column_101_offset_0: QM31,
    trace_1_column_102_offset_0: QM31,
    trace_1_column_103_offset_0: QM31,
    trace_1_column_104_offset_0: QM31,
    trace_1_column_105_offset_0: QM31,
    trace_1_column_106_offset_0: QM31,
    trace_1_column_107_offset_0: QM31,
    trace_1_column_108_offset_0: QM31,
    trace_1_column_109_offset_0: QM31,
    trace_1_column_10_offset_0: QM31,
    trace_1_column_110_offset_0: QM31,
    trace_1_column_111_offset_0: QM31,
    trace_1_column_112_offset_0: QM31,
    trace_1_column_113_offset_0: QM31,
    trace_1_column_114_offset_0: QM31,
    trace_1_column_115_offset_0: QM31,
    trace_1_column_116_offset_0: QM31,
    trace_1_column_117_offset_0: QM31,
    trace_1_column_118_offset_0: QM31,
    trace_1_column_119_offset_0: QM31,
    trace_1_column_11_offset_0: QM31,
    trace_1_column_120_offset_0: QM31,
    trace_1_column_121_offset_0: QM31,
    trace_1_column_122_offset_0: QM31,
    trace_1_column_123_offset_0: QM31,
    trace_1_column_124_offset_0: QM31,
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
    trace_1_column_98_offset_0: QM31,
    trace_1_column_99_offset_0: QM31,
    trace_1_column_9_offset_0: QM31,
) -> Array<QM31> {
    let intermediate4 = intermediate4(
        trace_1_column_102_offset_0,
        trace_1_column_32_offset_0,
        trace_1_column_42_offset_0,
        trace_1_column_52_offset_0,
        trace_1_column_62_offset_0,
        trace_1_column_92_offset_0,
    );

    let intermediate5 = intermediate5(
        intermediate4,
        trace_1_column_33_offset_0,
        trace_1_column_43_offset_0,
        trace_1_column_53_offset_0,
        trace_1_column_63_offset_0,
        trace_1_column_93_offset_0,
    );

    let intermediate6 = intermediate6(
        intermediate5,
        trace_1_column_34_offset_0,
        trace_1_column_44_offset_0,
        trace_1_column_54_offset_0,
        trace_1_column_64_offset_0,
        trace_1_column_94_offset_0,
    );

    let intermediate7 = intermediate7(
        intermediate6,
        trace_1_column_35_offset_0,
        trace_1_column_45_offset_0,
        trace_1_column_55_offset_0,
        trace_1_column_65_offset_0,
        trace_1_column_95_offset_0,
    );

    let intermediate8 = intermediate8(
        intermediate7,
        trace_1_column_36_offset_0,
        trace_1_column_46_offset_0,
        trace_1_column_56_offset_0,
        trace_1_column_66_offset_0,
        trace_1_column_96_offset_0,
    );

    let intermediate9 = intermediate9(
        intermediate8,
        trace_1_column_37_offset_0,
        trace_1_column_47_offset_0,
        trace_1_column_57_offset_0,
        trace_1_column_67_offset_0,
        trace_1_column_97_offset_0,
    );

    let intermediate10 = intermediate10(
        intermediate9,
        trace_1_column_38_offset_0,
        trace_1_column_48_offset_0,
        trace_1_column_58_offset_0,
        trace_1_column_68_offset_0,
        trace_1_column_98_offset_0,
    );

    let intermediate11 = intermediate11(
        intermediate10,
        trace_1_column_102_offset_0,
        trace_1_column_39_offset_0,
        trace_1_column_49_offset_0,
        trace_1_column_59_offset_0,
        trace_1_column_69_offset_0,
        trace_1_column_99_offset_0,
    );

    let intermediate12 = intermediate12(
        intermediate11,
        trace_1_column_100_offset_0,
        trace_1_column_40_offset_0,
        trace_1_column_50_offset_0,
        trace_1_column_60_offset_0,
        trace_1_column_70_offset_0,
    );

    let intermediate15 = intermediate15(
        trace_1_column_103_offset_0,
        trace_1_column_113_offset_0,
        trace_1_column_32_offset_0,
        trace_1_column_42_offset_0,
        trace_1_column_52_offset_0,
        trace_1_column_72_offset_0,
    );

    let intermediate16 = intermediate16(
        intermediate15,
        trace_1_column_104_offset_0,
        trace_1_column_33_offset_0,
        trace_1_column_43_offset_0,
        trace_1_column_53_offset_0,
        trace_1_column_73_offset_0,
    );

    let intermediate17 = intermediate17(
        intermediate16,
        trace_1_column_105_offset_0,
        trace_1_column_34_offset_0,
        trace_1_column_44_offset_0,
        trace_1_column_54_offset_0,
        trace_1_column_74_offset_0,
    );

    let intermediate18 = intermediate18(
        intermediate17,
        trace_1_column_106_offset_0,
        trace_1_column_35_offset_0,
        trace_1_column_45_offset_0,
        trace_1_column_55_offset_0,
        trace_1_column_75_offset_0,
    );

    let intermediate19 = intermediate19(
        intermediate18,
        trace_1_column_107_offset_0,
        trace_1_column_36_offset_0,
        trace_1_column_46_offset_0,
        trace_1_column_56_offset_0,
        trace_1_column_76_offset_0,
    );

    let intermediate20 = intermediate20(
        intermediate19,
        trace_1_column_108_offset_0,
        trace_1_column_37_offset_0,
        trace_1_column_47_offset_0,
        trace_1_column_57_offset_0,
        trace_1_column_77_offset_0,
    );

    let intermediate21 = intermediate21(
        intermediate20,
        trace_1_column_109_offset_0,
        trace_1_column_38_offset_0,
        trace_1_column_48_offset_0,
        trace_1_column_58_offset_0,
        trace_1_column_78_offset_0,
    );

    let intermediate22 = intermediate22(
        intermediate21,
        trace_1_column_110_offset_0,
        trace_1_column_113_offset_0,
        trace_1_column_39_offset_0,
        trace_1_column_49_offset_0,
        trace_1_column_59_offset_0,
        trace_1_column_79_offset_0,
    );

    let intermediate23 = intermediate23(
        intermediate22,
        trace_1_column_111_offset_0,
        trace_1_column_40_offset_0,
        trace_1_column_50_offset_0,
        trace_1_column_60_offset_0,
        trace_1_column_80_offset_0,
    );

    let intermediate26 = intermediate26(
        trace_1_column_114_offset_0,
        trace_1_column_124_offset_0,
        trace_1_column_32_offset_0,
        trace_1_column_42_offset_0,
        trace_1_column_52_offset_0,
        trace_1_column_82_offset_0,
    );

    let intermediate27 = intermediate27(
        intermediate26,
        trace_1_column_115_offset_0,
        trace_1_column_33_offset_0,
        trace_1_column_43_offset_0,
        trace_1_column_53_offset_0,
        trace_1_column_83_offset_0,
    );

    let intermediate28 = intermediate28(
        intermediate27,
        trace_1_column_116_offset_0,
        trace_1_column_34_offset_0,
        trace_1_column_44_offset_0,
        trace_1_column_54_offset_0,
        trace_1_column_84_offset_0,
    );

    let intermediate29 = intermediate29(
        intermediate28,
        trace_1_column_117_offset_0,
        trace_1_column_35_offset_0,
        trace_1_column_45_offset_0,
        trace_1_column_55_offset_0,
        trace_1_column_85_offset_0,
    );

    let intermediate30 = intermediate30(
        intermediate29,
        trace_1_column_118_offset_0,
        trace_1_column_36_offset_0,
        trace_1_column_46_offset_0,
        trace_1_column_56_offset_0,
        trace_1_column_86_offset_0,
    );

    let intermediate31 = intermediate31(
        intermediate30,
        trace_1_column_119_offset_0,
        trace_1_column_37_offset_0,
        trace_1_column_47_offset_0,
        trace_1_column_57_offset_0,
        trace_1_column_87_offset_0,
    );

    let intermediate32 = intermediate32(
        intermediate31,
        trace_1_column_120_offset_0,
        trace_1_column_38_offset_0,
        trace_1_column_48_offset_0,
        trace_1_column_58_offset_0,
        trace_1_column_88_offset_0,
    );

    let intermediate33 = intermediate33(
        intermediate32,
        trace_1_column_121_offset_0,
        trace_1_column_124_offset_0,
        trace_1_column_39_offset_0,
        trace_1_column_49_offset_0,
        trace_1_column_59_offset_0,
        trace_1_column_89_offset_0,
    );

    let intermediate34 = intermediate34(
        intermediate33,
        trace_1_column_122_offset_0,
        trace_1_column_40_offset_0,
        trace_1_column_50_offset_0,
        trace_1_column_60_offset_0,
        trace_1_column_90_offset_0,
    );
    let intermediate0 = intermediate0(
        Cube252_alpha0,
        Cube252_alpha1,
        Cube252_alpha10,
        Cube252_alpha11,
        Cube252_alpha12,
        Cube252_alpha13,
        Cube252_alpha14,
        Cube252_alpha15,
        Cube252_alpha16,
        Cube252_alpha17,
        Cube252_alpha18,
        Cube252_alpha19,
        Cube252_alpha2,
        Cube252_alpha3,
        Cube252_alpha4,
        Cube252_alpha5,
        Cube252_alpha6,
        Cube252_alpha7,
        Cube252_alpha8,
        Cube252_alpha9,
        Cube252_z,
        trace_1_column_10_offset_0,
        trace_1_column_11_offset_0,
        trace_1_column_2_offset_0,
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
        trace_1_column_4_offset_0,
        trace_1_column_5_offset_0,
        trace_1_column_6_offset_0,
        trace_1_column_7_offset_0,
        trace_1_column_8_offset_0,
        trace_1_column_9_offset_0,
    );

    let intermediate25 = intermediate25(
        RangeCheck_3_3_3_3_3_alpha0,
        RangeCheck_3_3_3_3_3_alpha1,
        RangeCheck_3_3_3_3_3_alpha2,
        RangeCheck_3_3_3_3_3_alpha3,
        RangeCheck_3_3_3_3_3_alpha4,
        RangeCheck_3_3_3_3_3_z,
        intermediate19,
        intermediate20,
        intermediate21,
        intermediate22,
        intermediate23,
    );

    let intermediate38 = intermediate38(
        PoseidonFullRoundChain_alpha0,
        PoseidonFullRoundChain_alpha1,
        PoseidonFullRoundChain_alpha10,
        PoseidonFullRoundChain_alpha11,
        PoseidonFullRoundChain_alpha12,
        PoseidonFullRoundChain_alpha13,
        PoseidonFullRoundChain_alpha14,
        PoseidonFullRoundChain_alpha15,
        PoseidonFullRoundChain_alpha16,
        PoseidonFullRoundChain_alpha17,
        PoseidonFullRoundChain_alpha18,
        PoseidonFullRoundChain_alpha19,
        PoseidonFullRoundChain_alpha2,
        PoseidonFullRoundChain_alpha20,
        PoseidonFullRoundChain_alpha21,
        PoseidonFullRoundChain_alpha22,
        PoseidonFullRoundChain_alpha23,
        PoseidonFullRoundChain_alpha24,
        PoseidonFullRoundChain_alpha25,
        PoseidonFullRoundChain_alpha26,
        PoseidonFullRoundChain_alpha27,
        PoseidonFullRoundChain_alpha28,
        PoseidonFullRoundChain_alpha29,
        PoseidonFullRoundChain_alpha3,
        PoseidonFullRoundChain_alpha30,
        PoseidonFullRoundChain_alpha31,
        PoseidonFullRoundChain_alpha4,
        PoseidonFullRoundChain_alpha5,
        PoseidonFullRoundChain_alpha6,
        PoseidonFullRoundChain_alpha7,
        PoseidonFullRoundChain_alpha8,
        PoseidonFullRoundChain_alpha9,
        PoseidonFullRoundChain_z,
        trace_1_column_0_offset_0,
        trace_1_column_100_offset_0,
        trace_1_column_101_offset_0,
        trace_1_column_103_offset_0,
        trace_1_column_104_offset_0,
        trace_1_column_105_offset_0,
        trace_1_column_106_offset_0,
        trace_1_column_107_offset_0,
        trace_1_column_108_offset_0,
        trace_1_column_109_offset_0,
        trace_1_column_110_offset_0,
        trace_1_column_111_offset_0,
        trace_1_column_112_offset_0,
        trace_1_column_114_offset_0,
        trace_1_column_115_offset_0,
        trace_1_column_116_offset_0,
        trace_1_column_117_offset_0,
        trace_1_column_118_offset_0,
        trace_1_column_119_offset_0,
        trace_1_column_120_offset_0,
        trace_1_column_121_offset_0,
        trace_1_column_122_offset_0,
        trace_1_column_123_offset_0,
        trace_1_column_1_offset_0,
        trace_1_column_92_offset_0,
        trace_1_column_93_offset_0,
        trace_1_column_94_offset_0,
        trace_1_column_95_offset_0,
        trace_1_column_96_offset_0,
        trace_1_column_97_offset_0,
        trace_1_column_98_offset_0,
        trace_1_column_99_offset_0,
    );

    core::internal::revoke_ap_tracking();

    let intermediate24 = intermediate24(
        RangeCheck_3_3_3_3_3_alpha0,
        RangeCheck_3_3_3_3_3_alpha1,
        RangeCheck_3_3_3_3_3_alpha2,
        RangeCheck_3_3_3_3_3_alpha3,
        RangeCheck_3_3_3_3_3_alpha4,
        RangeCheck_3_3_3_3_3_z,
        intermediate15,
        intermediate16,
        intermediate17,
        intermediate18,
        trace_1_column_113_offset_0,
    );

    let intermediate1 = intermediate1(
        Cube252_alpha0,
        Cube252_alpha1,
        Cube252_alpha10,
        Cube252_alpha11,
        Cube252_alpha12,
        Cube252_alpha13,
        Cube252_alpha14,
        Cube252_alpha15,
        Cube252_alpha16,
        Cube252_alpha17,
        Cube252_alpha18,
        Cube252_alpha19,
        Cube252_alpha2,
        Cube252_alpha3,
        Cube252_alpha4,
        Cube252_alpha5,
        Cube252_alpha6,
        Cube252_alpha7,
        Cube252_alpha8,
        Cube252_alpha9,
        Cube252_z,
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
    );

    let intermediate3 = intermediate3(
        PoseidonRoundKeys_alpha0,
        PoseidonRoundKeys_alpha1,
        PoseidonRoundKeys_alpha10,
        PoseidonRoundKeys_alpha11,
        PoseidonRoundKeys_alpha12,
        PoseidonRoundKeys_alpha13,
        PoseidonRoundKeys_alpha14,
        PoseidonRoundKeys_alpha15,
        PoseidonRoundKeys_alpha16,
        PoseidonRoundKeys_alpha17,
        PoseidonRoundKeys_alpha18,
        PoseidonRoundKeys_alpha19,
        PoseidonRoundKeys_alpha2,
        PoseidonRoundKeys_alpha20,
        PoseidonRoundKeys_alpha21,
        PoseidonRoundKeys_alpha22,
        PoseidonRoundKeys_alpha23,
        PoseidonRoundKeys_alpha24,
        PoseidonRoundKeys_alpha25,
        PoseidonRoundKeys_alpha26,
        PoseidonRoundKeys_alpha27,
        PoseidonRoundKeys_alpha28,
        PoseidonRoundKeys_alpha29,
        PoseidonRoundKeys_alpha3,
        PoseidonRoundKeys_alpha30,
        PoseidonRoundKeys_alpha4,
        PoseidonRoundKeys_alpha5,
        PoseidonRoundKeys_alpha6,
        PoseidonRoundKeys_alpha7,
        PoseidonRoundKeys_alpha8,
        PoseidonRoundKeys_alpha9,
        PoseidonRoundKeys_z,
        trace_1_column_1_offset_0,
        trace_1_column_62_offset_0,
        trace_1_column_63_offset_0,
        trace_1_column_64_offset_0,
        trace_1_column_65_offset_0,
        trace_1_column_66_offset_0,
        trace_1_column_67_offset_0,
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
    );

    let intermediate14 = intermediate14(
        RangeCheck_3_3_3_3_3_alpha0,
        RangeCheck_3_3_3_3_3_alpha1,
        RangeCheck_3_3_3_3_3_alpha2,
        RangeCheck_3_3_3_3_3_alpha3,
        RangeCheck_3_3_3_3_3_alpha4,
        RangeCheck_3_3_3_3_3_z,
        intermediate10,
        intermediate11,
        intermediate12,
        intermediate8,
        intermediate9,
    );

    let intermediate35 = intermediate35(
        RangeCheck_3_3_3_3_3_alpha0,
        RangeCheck_3_3_3_3_3_alpha1,
        RangeCheck_3_3_3_3_3_alpha2,
        RangeCheck_3_3_3_3_3_alpha3,
        RangeCheck_3_3_3_3_3_alpha4,
        RangeCheck_3_3_3_3_3_z,
        intermediate26,
        intermediate27,
        intermediate28,
        intermediate29,
        trace_1_column_124_offset_0,
    );

    let intermediate36 = intermediate36(
        RangeCheck_3_3_3_3_3_alpha0,
        RangeCheck_3_3_3_3_3_alpha1,
        RangeCheck_3_3_3_3_3_alpha2,
        RangeCheck_3_3_3_3_3_alpha3,
        RangeCheck_3_3_3_3_3_alpha4,
        RangeCheck_3_3_3_3_3_z,
        intermediate30,
        intermediate31,
        intermediate32,
        intermediate33,
        intermediate34,
    );

    let intermediate13 = intermediate13(
        RangeCheck_3_3_3_3_3_alpha0,
        RangeCheck_3_3_3_3_3_alpha1,
        RangeCheck_3_3_3_3_3_alpha2,
        RangeCheck_3_3_3_3_3_alpha3,
        RangeCheck_3_3_3_3_3_alpha4,
        RangeCheck_3_3_3_3_3_z,
        intermediate4,
        intermediate5,
        intermediate6,
        intermediate7,
        trace_1_column_102_offset_0,
    );

    let intermediate2 = intermediate2(
        Cube252_alpha0,
        Cube252_alpha1,
        Cube252_alpha10,
        Cube252_alpha11,
        Cube252_alpha12,
        Cube252_alpha13,
        Cube252_alpha14,
        Cube252_alpha15,
        Cube252_alpha16,
        Cube252_alpha17,
        Cube252_alpha18,
        Cube252_alpha19,
        Cube252_alpha2,
        Cube252_alpha3,
        Cube252_alpha4,
        Cube252_alpha5,
        Cube252_alpha6,
        Cube252_alpha7,
        Cube252_alpha8,
        Cube252_alpha9,
        Cube252_z,
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
    );

    core::internal::revoke_ap_tracking();

    let intermediate37 = intermediate37(
        PoseidonFullRoundChain_alpha0,
        PoseidonFullRoundChain_alpha1,
        PoseidonFullRoundChain_alpha10,
        PoseidonFullRoundChain_alpha11,
        PoseidonFullRoundChain_alpha12,
        PoseidonFullRoundChain_alpha13,
        PoseidonFullRoundChain_alpha14,
        PoseidonFullRoundChain_alpha15,
        PoseidonFullRoundChain_alpha16,
        PoseidonFullRoundChain_alpha17,
        PoseidonFullRoundChain_alpha18,
        PoseidonFullRoundChain_alpha19,
        PoseidonFullRoundChain_alpha2,
        PoseidonFullRoundChain_alpha20,
        PoseidonFullRoundChain_alpha21,
        PoseidonFullRoundChain_alpha22,
        PoseidonFullRoundChain_alpha23,
        PoseidonFullRoundChain_alpha24,
        PoseidonFullRoundChain_alpha25,
        PoseidonFullRoundChain_alpha26,
        PoseidonFullRoundChain_alpha27,
        PoseidonFullRoundChain_alpha28,
        PoseidonFullRoundChain_alpha29,
        PoseidonFullRoundChain_alpha3,
        PoseidonFullRoundChain_alpha30,
        PoseidonFullRoundChain_alpha31,
        PoseidonFullRoundChain_alpha4,
        PoseidonFullRoundChain_alpha5,
        PoseidonFullRoundChain_alpha6,
        PoseidonFullRoundChain_alpha7,
        PoseidonFullRoundChain_alpha8,
        PoseidonFullRoundChain_alpha9,
        PoseidonFullRoundChain_z,
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
        intermediate36,
        intermediate37,
        intermediate38,
    ]
}

pub fn intermediate28(
    intermediate27: QM31,
    trace_1_column_116_offset_0: QM31,
    trace_1_column_34_offset_0: QM31,
    trace_1_column_44_offset_0: QM31,
    trace_1_column_54_offset_0: QM31,
    trace_1_column_84_offset_0: QM31,
) -> QM31 {
    (intermediate27
        + trace_1_column_34_offset_0
        + trace_1_column_44_offset_0
        - ((m31(2).into()) * (trace_1_column_54_offset_0))
        + trace_1_column_84_offset_0
        - (trace_1_column_116_offset_0))
        * (m31(16).into())
}

pub fn intermediate29(
    intermediate28: QM31,
    trace_1_column_117_offset_0: QM31,
    trace_1_column_35_offset_0: QM31,
    trace_1_column_45_offset_0: QM31,
    trace_1_column_55_offset_0: QM31,
    trace_1_column_85_offset_0: QM31,
) -> QM31 {
    (intermediate28
        + trace_1_column_35_offset_0
        + trace_1_column_45_offset_0
        - ((m31(2).into()) * (trace_1_column_55_offset_0))
        + trace_1_column_85_offset_0
        - (trace_1_column_117_offset_0))
        * (m31(16).into())
}

pub fn intermediate33(
    intermediate32: QM31,
    trace_1_column_121_offset_0: QM31,
    trace_1_column_124_offset_0: QM31,
    trace_1_column_39_offset_0: QM31,
    trace_1_column_49_offset_0: QM31,
    trace_1_column_59_offset_0: QM31,
    trace_1_column_89_offset_0: QM31,
) -> QM31 {
    (intermediate32
        + trace_1_column_39_offset_0
        + trace_1_column_49_offset_0
        - ((m31(2).into()) * (trace_1_column_59_offset_0))
        + trace_1_column_89_offset_0
        - (trace_1_column_121_offset_0)
        - ((trace_1_column_124_offset_0) * (m31(136).into())))
        * (m31(16).into())
}

pub fn intermediate34(
    intermediate33: QM31,
    trace_1_column_122_offset_0: QM31,
    trace_1_column_40_offset_0: QM31,
    trace_1_column_50_offset_0: QM31,
    trace_1_column_60_offset_0: QM31,
    trace_1_column_90_offset_0: QM31,
) -> QM31 {
    (intermediate33
        + trace_1_column_40_offset_0
        + trace_1_column_50_offset_0
        - ((m31(2).into()) * (trace_1_column_60_offset_0))
        + trace_1_column_90_offset_0
        - (trace_1_column_122_offset_0))
        * (m31(16).into())
}

pub fn intermediate15(
    trace_1_column_103_offset_0: QM31,
    trace_1_column_113_offset_0: QM31,
    trace_1_column_32_offset_0: QM31,
    trace_1_column_42_offset_0: QM31,
    trace_1_column_52_offset_0: QM31,
    trace_1_column_72_offset_0: QM31,
) -> QM31 {
    (trace_1_column_32_offset_0
        - (trace_1_column_42_offset_0)
        + trace_1_column_52_offset_0
        + trace_1_column_72_offset_0
        - (trace_1_column_103_offset_0)
        - (trace_1_column_113_offset_0))
        * (m31(16).into())
}

pub fn intermediate5(
    intermediate4: QM31,
    trace_1_column_33_offset_0: QM31,
    trace_1_column_43_offset_0: QM31,
    trace_1_column_53_offset_0: QM31,
    trace_1_column_63_offset_0: QM31,
    trace_1_column_93_offset_0: QM31,
) -> QM31 {
    (intermediate4
        + (m31(3).into()) * (trace_1_column_33_offset_0)
        + trace_1_column_43_offset_0
        + trace_1_column_53_offset_0
        + trace_1_column_63_offset_0
        - (trace_1_column_93_offset_0))
        * (m31(16).into())
}

pub fn intermediate18(
    intermediate17: QM31,
    trace_1_column_106_offset_0: QM31,
    trace_1_column_35_offset_0: QM31,
    trace_1_column_45_offset_0: QM31,
    trace_1_column_55_offset_0: QM31,
    trace_1_column_75_offset_0: QM31,
) -> QM31 {
    (intermediate17
        + trace_1_column_35_offset_0
        - (trace_1_column_45_offset_0)
        + trace_1_column_55_offset_0
        + trace_1_column_75_offset_0
        - (trace_1_column_106_offset_0))
        * (m31(16).into())
}

pub fn intermediate23(
    intermediate22: QM31,
    trace_1_column_111_offset_0: QM31,
    trace_1_column_40_offset_0: QM31,
    trace_1_column_50_offset_0: QM31,
    trace_1_column_60_offset_0: QM31,
    trace_1_column_80_offset_0: QM31,
) -> QM31 {
    (intermediate22
        + trace_1_column_40_offset_0
        - (trace_1_column_50_offset_0)
        + trace_1_column_60_offset_0
        + trace_1_column_80_offset_0
        - (trace_1_column_111_offset_0))
        * (m31(16).into())
}

pub fn intermediate26(
    trace_1_column_114_offset_0: QM31,
    trace_1_column_124_offset_0: QM31,
    trace_1_column_32_offset_0: QM31,
    trace_1_column_42_offset_0: QM31,
    trace_1_column_52_offset_0: QM31,
    trace_1_column_82_offset_0: QM31,
) -> QM31 {
    (trace_1_column_32_offset_0
        + trace_1_column_42_offset_0
        - ((m31(2).into()) * (trace_1_column_52_offset_0))
        + trace_1_column_82_offset_0
        - (trace_1_column_114_offset_0)
        - (trace_1_column_124_offset_0))
        * (m31(16).into())
}

pub fn intermediate10(
    intermediate9: QM31,
    trace_1_column_38_offset_0: QM31,
    trace_1_column_48_offset_0: QM31,
    trace_1_column_58_offset_0: QM31,
    trace_1_column_68_offset_0: QM31,
    trace_1_column_98_offset_0: QM31,
) -> QM31 {
    (intermediate9
        + (m31(3).into()) * (trace_1_column_38_offset_0)
        + trace_1_column_48_offset_0
        + trace_1_column_58_offset_0
        + trace_1_column_68_offset_0
        - (trace_1_column_98_offset_0))
        * (m31(16).into())
}

pub fn intermediate9(
    intermediate8: QM31,
    trace_1_column_37_offset_0: QM31,
    trace_1_column_47_offset_0: QM31,
    trace_1_column_57_offset_0: QM31,
    trace_1_column_67_offset_0: QM31,
    trace_1_column_97_offset_0: QM31,
) -> QM31 {
    (intermediate8
        + (m31(3).into()) * (trace_1_column_37_offset_0)
        + trace_1_column_47_offset_0
        + trace_1_column_57_offset_0
        + trace_1_column_67_offset_0
        - (trace_1_column_97_offset_0))
        * (m31(16).into())
}

pub fn intermediate17(
    intermediate16: QM31,
    trace_1_column_105_offset_0: QM31,
    trace_1_column_34_offset_0: QM31,
    trace_1_column_44_offset_0: QM31,
    trace_1_column_54_offset_0: QM31,
    trace_1_column_74_offset_0: QM31,
) -> QM31 {
    (intermediate16
        + trace_1_column_34_offset_0
        - (trace_1_column_44_offset_0)
        + trace_1_column_54_offset_0
        + trace_1_column_74_offset_0
        - (trace_1_column_105_offset_0))
        * (m31(16).into())
}

pub fn intermediate8(
    intermediate7: QM31,
    trace_1_column_36_offset_0: QM31,
    trace_1_column_46_offset_0: QM31,
    trace_1_column_56_offset_0: QM31,
    trace_1_column_66_offset_0: QM31,
    trace_1_column_96_offset_0: QM31,
) -> QM31 {
    (intermediate7
        + (m31(3).into()) * (trace_1_column_36_offset_0)
        + trace_1_column_46_offset_0
        + trace_1_column_56_offset_0
        + trace_1_column_66_offset_0
        - (trace_1_column_96_offset_0))
        * (m31(16).into())
}

pub fn intermediate6(
    intermediate5: QM31,
    trace_1_column_34_offset_0: QM31,
    trace_1_column_44_offset_0: QM31,
    trace_1_column_54_offset_0: QM31,
    trace_1_column_64_offset_0: QM31,
    trace_1_column_94_offset_0: QM31,
) -> QM31 {
    (intermediate5
        + (m31(3).into()) * (trace_1_column_34_offset_0)
        + trace_1_column_44_offset_0
        + trace_1_column_54_offset_0
        + trace_1_column_64_offset_0
        - (trace_1_column_94_offset_0))
        * (m31(16).into())
}

pub fn intermediate12(
    intermediate11: QM31,
    trace_1_column_100_offset_0: QM31,
    trace_1_column_40_offset_0: QM31,
    trace_1_column_50_offset_0: QM31,
    trace_1_column_60_offset_0: QM31,
    trace_1_column_70_offset_0: QM31,
) -> QM31 {
    (intermediate11
        + (m31(3).into()) * (trace_1_column_40_offset_0)
        + trace_1_column_50_offset_0
        + trace_1_column_60_offset_0
        + trace_1_column_70_offset_0
        - (trace_1_column_100_offset_0))
        * (m31(16).into())
}

pub fn intermediate19(
    intermediate18: QM31,
    trace_1_column_107_offset_0: QM31,
    trace_1_column_36_offset_0: QM31,
    trace_1_column_46_offset_0: QM31,
    trace_1_column_56_offset_0: QM31,
    trace_1_column_76_offset_0: QM31,
) -> QM31 {
    (intermediate18
        + trace_1_column_36_offset_0
        - (trace_1_column_46_offset_0)
        + trace_1_column_56_offset_0
        + trace_1_column_76_offset_0
        - (trace_1_column_107_offset_0))
        * (m31(16).into())
}

pub fn intermediate21(
    intermediate20: QM31,
    trace_1_column_109_offset_0: QM31,
    trace_1_column_38_offset_0: QM31,
    trace_1_column_48_offset_0: QM31,
    trace_1_column_58_offset_0: QM31,
    trace_1_column_78_offset_0: QM31,
) -> QM31 {
    (intermediate20
        + trace_1_column_38_offset_0
        - (trace_1_column_48_offset_0)
        + trace_1_column_58_offset_0
        + trace_1_column_78_offset_0
        - (trace_1_column_109_offset_0))
        * (m31(16).into())
}

pub fn intermediate27(
    intermediate26: QM31,
    trace_1_column_115_offset_0: QM31,
    trace_1_column_33_offset_0: QM31,
    trace_1_column_43_offset_0: QM31,
    trace_1_column_53_offset_0: QM31,
    trace_1_column_83_offset_0: QM31,
) -> QM31 {
    (intermediate26
        + trace_1_column_33_offset_0
        + trace_1_column_43_offset_0
        - ((m31(2).into()) * (trace_1_column_53_offset_0))
        + trace_1_column_83_offset_0
        - (trace_1_column_115_offset_0))
        * (m31(16).into())
}

pub fn intermediate30(
    intermediate29: QM31,
    trace_1_column_118_offset_0: QM31,
    trace_1_column_36_offset_0: QM31,
    trace_1_column_46_offset_0: QM31,
    trace_1_column_56_offset_0: QM31,
    trace_1_column_86_offset_0: QM31,
) -> QM31 {
    (intermediate29
        + trace_1_column_36_offset_0
        + trace_1_column_46_offset_0
        - ((m31(2).into()) * (trace_1_column_56_offset_0))
        + trace_1_column_86_offset_0
        - (trace_1_column_118_offset_0))
        * (m31(16).into())
}

pub fn intermediate16(
    intermediate15: QM31,
    trace_1_column_104_offset_0: QM31,
    trace_1_column_33_offset_0: QM31,
    trace_1_column_43_offset_0: QM31,
    trace_1_column_53_offset_0: QM31,
    trace_1_column_73_offset_0: QM31,
) -> QM31 {
    (intermediate15
        + trace_1_column_33_offset_0
        - (trace_1_column_43_offset_0)
        + trace_1_column_53_offset_0
        + trace_1_column_73_offset_0
        - (trace_1_column_104_offset_0))
        * (m31(16).into())
}

pub fn intermediate11(
    intermediate10: QM31,
    trace_1_column_102_offset_0: QM31,
    trace_1_column_39_offset_0: QM31,
    trace_1_column_49_offset_0: QM31,
    trace_1_column_59_offset_0: QM31,
    trace_1_column_69_offset_0: QM31,
    trace_1_column_99_offset_0: QM31,
) -> QM31 {
    (intermediate10
        + (m31(3).into()) * (trace_1_column_39_offset_0)
        + trace_1_column_49_offset_0
        + trace_1_column_59_offset_0
        + trace_1_column_69_offset_0
        - (trace_1_column_99_offset_0)
        - ((trace_1_column_102_offset_0) * (m31(136).into())))
        * (m31(16).into())
}

pub fn intermediate20(
    intermediate19: QM31,
    trace_1_column_108_offset_0: QM31,
    trace_1_column_37_offset_0: QM31,
    trace_1_column_47_offset_0: QM31,
    trace_1_column_57_offset_0: QM31,
    trace_1_column_77_offset_0: QM31,
) -> QM31 {
    (intermediate19
        + trace_1_column_37_offset_0
        - (trace_1_column_47_offset_0)
        + trace_1_column_57_offset_0
        + trace_1_column_77_offset_0
        - (trace_1_column_108_offset_0))
        * (m31(16).into())
}

pub fn intermediate22(
    intermediate21: QM31,
    trace_1_column_110_offset_0: QM31,
    trace_1_column_113_offset_0: QM31,
    trace_1_column_39_offset_0: QM31,
    trace_1_column_49_offset_0: QM31,
    trace_1_column_59_offset_0: QM31,
    trace_1_column_79_offset_0: QM31,
) -> QM31 {
    (intermediate21
        + trace_1_column_39_offset_0
        - (trace_1_column_49_offset_0)
        + trace_1_column_59_offset_0
        + trace_1_column_79_offset_0
        - (trace_1_column_110_offset_0)
        - ((trace_1_column_113_offset_0) * (m31(136).into())))
        * (m31(16).into())
}

pub fn intermediate31(
    intermediate30: QM31,
    trace_1_column_119_offset_0: QM31,
    trace_1_column_37_offset_0: QM31,
    trace_1_column_47_offset_0: QM31,
    trace_1_column_57_offset_0: QM31,
    trace_1_column_87_offset_0: QM31,
) -> QM31 {
    (intermediate30
        + trace_1_column_37_offset_0
        + trace_1_column_47_offset_0
        - ((m31(2).into()) * (trace_1_column_57_offset_0))
        + trace_1_column_87_offset_0
        - (trace_1_column_119_offset_0))
        * (m31(16).into())
}

pub fn intermediate32(
    intermediate31: QM31,
    trace_1_column_120_offset_0: QM31,
    trace_1_column_38_offset_0: QM31,
    trace_1_column_48_offset_0: QM31,
    trace_1_column_58_offset_0: QM31,
    trace_1_column_88_offset_0: QM31,
) -> QM31 {
    (intermediate31
        + trace_1_column_38_offset_0
        + trace_1_column_48_offset_0
        - ((m31(2).into()) * (trace_1_column_58_offset_0))
        + trace_1_column_88_offset_0
        - (trace_1_column_120_offset_0))
        * (m31(16).into())
}

pub fn intermediate4(
    trace_1_column_102_offset_0: QM31,
    trace_1_column_32_offset_0: QM31,
    trace_1_column_42_offset_0: QM31,
    trace_1_column_52_offset_0: QM31,
    trace_1_column_62_offset_0: QM31,
    trace_1_column_92_offset_0: QM31,
) -> QM31 {
    ((m31(3).into()) * (trace_1_column_32_offset_0)
        + trace_1_column_42_offset_0
        + trace_1_column_52_offset_0
        + trace_1_column_62_offset_0
        - (trace_1_column_92_offset_0)
        - (trace_1_column_102_offset_0))
        * (m31(16).into())
}

pub fn intermediate7(
    intermediate6: QM31,
    trace_1_column_35_offset_0: QM31,
    trace_1_column_45_offset_0: QM31,
    trace_1_column_55_offset_0: QM31,
    trace_1_column_65_offset_0: QM31,
    trace_1_column_95_offset_0: QM31,
) -> QM31 {
    (intermediate6
        + (m31(3).into()) * (trace_1_column_35_offset_0)
        + trace_1_column_45_offset_0
        + trace_1_column_55_offset_0
        + trace_1_column_65_offset_0
        - (trace_1_column_95_offset_0))
        * (m31(16).into())
}
pub fn intermediate0(
    Cube252_alpha0: QM31,
    Cube252_alpha1: QM31,
    Cube252_alpha10: QM31,
    Cube252_alpha11: QM31,
    Cube252_alpha12: QM31,
    Cube252_alpha13: QM31,
    Cube252_alpha14: QM31,
    Cube252_alpha15: QM31,
    Cube252_alpha16: QM31,
    Cube252_alpha17: QM31,
    Cube252_alpha18: QM31,
    Cube252_alpha19: QM31,
    Cube252_alpha2: QM31,
    Cube252_alpha3: QM31,
    Cube252_alpha4: QM31,
    Cube252_alpha5: QM31,
    Cube252_alpha6: QM31,
    Cube252_alpha7: QM31,
    Cube252_alpha8: QM31,
    Cube252_alpha9: QM31,
    Cube252_z: QM31,
    trace_1_column_10_offset_0: QM31,
    trace_1_column_11_offset_0: QM31,
    trace_1_column_2_offset_0: QM31,
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
    trace_1_column_4_offset_0: QM31,
    trace_1_column_5_offset_0: QM31,
    trace_1_column_6_offset_0: QM31,
    trace_1_column_7_offset_0: QM31,
    trace_1_column_8_offset_0: QM31,
    trace_1_column_9_offset_0: QM31,
) -> QM31 {
    (Cube252_alpha0) * (trace_1_column_2_offset_0)
        + (Cube252_alpha1) * (trace_1_column_3_offset_0)
        + (Cube252_alpha2) * (trace_1_column_4_offset_0)
        + (Cube252_alpha3) * (trace_1_column_5_offset_0)
        + (Cube252_alpha4) * (trace_1_column_6_offset_0)
        + (Cube252_alpha5) * (trace_1_column_7_offset_0)
        + (Cube252_alpha6) * (trace_1_column_8_offset_0)
        + (Cube252_alpha7) * (trace_1_column_9_offset_0)
        + (Cube252_alpha8) * (trace_1_column_10_offset_0)
        + (Cube252_alpha9) * (trace_1_column_11_offset_0)
        + (Cube252_alpha10) * (trace_1_column_32_offset_0)
        + (Cube252_alpha11) * (trace_1_column_33_offset_0)
        + (Cube252_alpha12) * (trace_1_column_34_offset_0)
        + (Cube252_alpha13) * (trace_1_column_35_offset_0)
        + (Cube252_alpha14) * (trace_1_column_36_offset_0)
        + (Cube252_alpha15) * (trace_1_column_37_offset_0)
        + (Cube252_alpha16) * (trace_1_column_38_offset_0)
        + (Cube252_alpha17) * (trace_1_column_39_offset_0)
        + (Cube252_alpha18) * (trace_1_column_40_offset_0)
        + (Cube252_alpha19) * (trace_1_column_41_offset_0)
        - (Cube252_z)
}

pub fn intermediate25(
    RangeCheck_3_3_3_3_3_alpha0: QM31,
    RangeCheck_3_3_3_3_3_alpha1: QM31,
    RangeCheck_3_3_3_3_3_alpha2: QM31,
    RangeCheck_3_3_3_3_3_alpha3: QM31,
    RangeCheck_3_3_3_3_3_alpha4: QM31,
    RangeCheck_3_3_3_3_3_z: QM31,
    intermediate19: QM31,
    intermediate20: QM31,
    intermediate21: QM31,
    intermediate22: QM31,
    intermediate23: QM31,
) -> QM31 {
    (RangeCheck_3_3_3_3_3_alpha0) * (intermediate19 + m31(2).into())
        + (RangeCheck_3_3_3_3_3_alpha1) * (intermediate20 + m31(2).into())
        + (RangeCheck_3_3_3_3_3_alpha2) * (intermediate21 + m31(2).into())
        + (RangeCheck_3_3_3_3_3_alpha3) * (intermediate22 + m31(2).into())
        + (RangeCheck_3_3_3_3_3_alpha4) * (intermediate23 + m31(2).into())
        - (RangeCheck_3_3_3_3_3_z)
}

pub fn intermediate38(
    PoseidonFullRoundChain_alpha0: QM31,
    PoseidonFullRoundChain_alpha1: QM31,
    PoseidonFullRoundChain_alpha10: QM31,
    PoseidonFullRoundChain_alpha11: QM31,
    PoseidonFullRoundChain_alpha12: QM31,
    PoseidonFullRoundChain_alpha13: QM31,
    PoseidonFullRoundChain_alpha14: QM31,
    PoseidonFullRoundChain_alpha15: QM31,
    PoseidonFullRoundChain_alpha16: QM31,
    PoseidonFullRoundChain_alpha17: QM31,
    PoseidonFullRoundChain_alpha18: QM31,
    PoseidonFullRoundChain_alpha19: QM31,
    PoseidonFullRoundChain_alpha2: QM31,
    PoseidonFullRoundChain_alpha20: QM31,
    PoseidonFullRoundChain_alpha21: QM31,
    PoseidonFullRoundChain_alpha22: QM31,
    PoseidonFullRoundChain_alpha23: QM31,
    PoseidonFullRoundChain_alpha24: QM31,
    PoseidonFullRoundChain_alpha25: QM31,
    PoseidonFullRoundChain_alpha26: QM31,
    PoseidonFullRoundChain_alpha27: QM31,
    PoseidonFullRoundChain_alpha28: QM31,
    PoseidonFullRoundChain_alpha29: QM31,
    PoseidonFullRoundChain_alpha3: QM31,
    PoseidonFullRoundChain_alpha30: QM31,
    PoseidonFullRoundChain_alpha31: QM31,
    PoseidonFullRoundChain_alpha4: QM31,
    PoseidonFullRoundChain_alpha5: QM31,
    PoseidonFullRoundChain_alpha6: QM31,
    PoseidonFullRoundChain_alpha7: QM31,
    PoseidonFullRoundChain_alpha8: QM31,
    PoseidonFullRoundChain_alpha9: QM31,
    PoseidonFullRoundChain_z: QM31,
    trace_1_column_0_offset_0: QM31,
    trace_1_column_100_offset_0: QM31,
    trace_1_column_101_offset_0: QM31,
    trace_1_column_103_offset_0: QM31,
    trace_1_column_104_offset_0: QM31,
    trace_1_column_105_offset_0: QM31,
    trace_1_column_106_offset_0: QM31,
    trace_1_column_107_offset_0: QM31,
    trace_1_column_108_offset_0: QM31,
    trace_1_column_109_offset_0: QM31,
    trace_1_column_110_offset_0: QM31,
    trace_1_column_111_offset_0: QM31,
    trace_1_column_112_offset_0: QM31,
    trace_1_column_114_offset_0: QM31,
    trace_1_column_115_offset_0: QM31,
    trace_1_column_116_offset_0: QM31,
    trace_1_column_117_offset_0: QM31,
    trace_1_column_118_offset_0: QM31,
    trace_1_column_119_offset_0: QM31,
    trace_1_column_120_offset_0: QM31,
    trace_1_column_121_offset_0: QM31,
    trace_1_column_122_offset_0: QM31,
    trace_1_column_123_offset_0: QM31,
    trace_1_column_1_offset_0: QM31,
    trace_1_column_92_offset_0: QM31,
    trace_1_column_93_offset_0: QM31,
    trace_1_column_94_offset_0: QM31,
    trace_1_column_95_offset_0: QM31,
    trace_1_column_96_offset_0: QM31,
    trace_1_column_97_offset_0: QM31,
    trace_1_column_98_offset_0: QM31,
    trace_1_column_99_offset_0: QM31,
) -> QM31 {
    (PoseidonFullRoundChain_alpha0) * (trace_1_column_0_offset_0)
        + (PoseidonFullRoundChain_alpha1) * (trace_1_column_1_offset_0 + m31(1).into())
        + (PoseidonFullRoundChain_alpha2) * (trace_1_column_92_offset_0)
        + (PoseidonFullRoundChain_alpha3) * (trace_1_column_93_offset_0)
        + (PoseidonFullRoundChain_alpha4) * (trace_1_column_94_offset_0)
        + (PoseidonFullRoundChain_alpha5) * (trace_1_column_95_offset_0)
        + (PoseidonFullRoundChain_alpha6) * (trace_1_column_96_offset_0)
        + (PoseidonFullRoundChain_alpha7) * (trace_1_column_97_offset_0)
        + (PoseidonFullRoundChain_alpha8) * (trace_1_column_98_offset_0)
        + (PoseidonFullRoundChain_alpha9) * (trace_1_column_99_offset_0)
        + (PoseidonFullRoundChain_alpha10) * (trace_1_column_100_offset_0)
        + (PoseidonFullRoundChain_alpha11) * (trace_1_column_101_offset_0)
        + (PoseidonFullRoundChain_alpha12) * (trace_1_column_103_offset_0)
        + (PoseidonFullRoundChain_alpha13) * (trace_1_column_104_offset_0)
        + (PoseidonFullRoundChain_alpha14) * (trace_1_column_105_offset_0)
        + (PoseidonFullRoundChain_alpha15) * (trace_1_column_106_offset_0)
        + (PoseidonFullRoundChain_alpha16) * (trace_1_column_107_offset_0)
        + (PoseidonFullRoundChain_alpha17) * (trace_1_column_108_offset_0)
        + (PoseidonFullRoundChain_alpha18) * (trace_1_column_109_offset_0)
        + (PoseidonFullRoundChain_alpha19) * (trace_1_column_110_offset_0)
        + (PoseidonFullRoundChain_alpha20) * (trace_1_column_111_offset_0)
        + (PoseidonFullRoundChain_alpha21) * (trace_1_column_112_offset_0)
        + (PoseidonFullRoundChain_alpha22) * (trace_1_column_114_offset_0)
        + (PoseidonFullRoundChain_alpha23) * (trace_1_column_115_offset_0)
        + (PoseidonFullRoundChain_alpha24) * (trace_1_column_116_offset_0)
        + (PoseidonFullRoundChain_alpha25) * (trace_1_column_117_offset_0)
        + (PoseidonFullRoundChain_alpha26) * (trace_1_column_118_offset_0)
        + (PoseidonFullRoundChain_alpha27) * (trace_1_column_119_offset_0)
        + (PoseidonFullRoundChain_alpha28) * (trace_1_column_120_offset_0)
        + (PoseidonFullRoundChain_alpha29) * (trace_1_column_121_offset_0)
        + (PoseidonFullRoundChain_alpha30) * (trace_1_column_122_offset_0)
        + (PoseidonFullRoundChain_alpha31) * (trace_1_column_123_offset_0)
        - (PoseidonFullRoundChain_z)
}

pub fn intermediate24(
    RangeCheck_3_3_3_3_3_alpha0: QM31,
    RangeCheck_3_3_3_3_3_alpha1: QM31,
    RangeCheck_3_3_3_3_3_alpha2: QM31,
    RangeCheck_3_3_3_3_3_alpha3: QM31,
    RangeCheck_3_3_3_3_3_alpha4: QM31,
    RangeCheck_3_3_3_3_3_z: QM31,
    intermediate15: QM31,
    intermediate16: QM31,
    intermediate17: QM31,
    intermediate18: QM31,
    trace_1_column_113_offset_0: QM31,
) -> QM31 {
    (RangeCheck_3_3_3_3_3_alpha0) * (trace_1_column_113_offset_0 + m31(2).into())
        + (RangeCheck_3_3_3_3_3_alpha1) * (intermediate15 + m31(2).into())
        + (RangeCheck_3_3_3_3_3_alpha2) * (intermediate16 + m31(2).into())
        + (RangeCheck_3_3_3_3_3_alpha3) * (intermediate17 + m31(2).into())
        + (RangeCheck_3_3_3_3_3_alpha4) * (intermediate18 + m31(2).into())
        - (RangeCheck_3_3_3_3_3_z)
}

pub fn intermediate1(
    Cube252_alpha0: QM31,
    Cube252_alpha1: QM31,
    Cube252_alpha10: QM31,
    Cube252_alpha11: QM31,
    Cube252_alpha12: QM31,
    Cube252_alpha13: QM31,
    Cube252_alpha14: QM31,
    Cube252_alpha15: QM31,
    Cube252_alpha16: QM31,
    Cube252_alpha17: QM31,
    Cube252_alpha18: QM31,
    Cube252_alpha19: QM31,
    Cube252_alpha2: QM31,
    Cube252_alpha3: QM31,
    Cube252_alpha4: QM31,
    Cube252_alpha5: QM31,
    Cube252_alpha6: QM31,
    Cube252_alpha7: QM31,
    Cube252_alpha8: QM31,
    Cube252_alpha9: QM31,
    Cube252_z: QM31,
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
) -> QM31 {
    (Cube252_alpha0) * (trace_1_column_12_offset_0)
        + (Cube252_alpha1) * (trace_1_column_13_offset_0)
        + (Cube252_alpha2) * (trace_1_column_14_offset_0)
        + (Cube252_alpha3) * (trace_1_column_15_offset_0)
        + (Cube252_alpha4) * (trace_1_column_16_offset_0)
        + (Cube252_alpha5) * (trace_1_column_17_offset_0)
        + (Cube252_alpha6) * (trace_1_column_18_offset_0)
        + (Cube252_alpha7) * (trace_1_column_19_offset_0)
        + (Cube252_alpha8) * (trace_1_column_20_offset_0)
        + (Cube252_alpha9) * (trace_1_column_21_offset_0)
        + (Cube252_alpha10) * (trace_1_column_42_offset_0)
        + (Cube252_alpha11) * (trace_1_column_43_offset_0)
        + (Cube252_alpha12) * (trace_1_column_44_offset_0)
        + (Cube252_alpha13) * (trace_1_column_45_offset_0)
        + (Cube252_alpha14) * (trace_1_column_46_offset_0)
        + (Cube252_alpha15) * (trace_1_column_47_offset_0)
        + (Cube252_alpha16) * (trace_1_column_48_offset_0)
        + (Cube252_alpha17) * (trace_1_column_49_offset_0)
        + (Cube252_alpha18) * (trace_1_column_50_offset_0)
        + (Cube252_alpha19) * (trace_1_column_51_offset_0)
        - (Cube252_z)
}

pub fn intermediate3(
    PoseidonRoundKeys_alpha0: QM31,
    PoseidonRoundKeys_alpha1: QM31,
    PoseidonRoundKeys_alpha10: QM31,
    PoseidonRoundKeys_alpha11: QM31,
    PoseidonRoundKeys_alpha12: QM31,
    PoseidonRoundKeys_alpha13: QM31,
    PoseidonRoundKeys_alpha14: QM31,
    PoseidonRoundKeys_alpha15: QM31,
    PoseidonRoundKeys_alpha16: QM31,
    PoseidonRoundKeys_alpha17: QM31,
    PoseidonRoundKeys_alpha18: QM31,
    PoseidonRoundKeys_alpha19: QM31,
    PoseidonRoundKeys_alpha2: QM31,
    PoseidonRoundKeys_alpha20: QM31,
    PoseidonRoundKeys_alpha21: QM31,
    PoseidonRoundKeys_alpha22: QM31,
    PoseidonRoundKeys_alpha23: QM31,
    PoseidonRoundKeys_alpha24: QM31,
    PoseidonRoundKeys_alpha25: QM31,
    PoseidonRoundKeys_alpha26: QM31,
    PoseidonRoundKeys_alpha27: QM31,
    PoseidonRoundKeys_alpha28: QM31,
    PoseidonRoundKeys_alpha29: QM31,
    PoseidonRoundKeys_alpha3: QM31,
    PoseidonRoundKeys_alpha30: QM31,
    PoseidonRoundKeys_alpha4: QM31,
    PoseidonRoundKeys_alpha5: QM31,
    PoseidonRoundKeys_alpha6: QM31,
    PoseidonRoundKeys_alpha7: QM31,
    PoseidonRoundKeys_alpha8: QM31,
    PoseidonRoundKeys_alpha9: QM31,
    PoseidonRoundKeys_z: QM31,
    trace_1_column_1_offset_0: QM31,
    trace_1_column_62_offset_0: QM31,
    trace_1_column_63_offset_0: QM31,
    trace_1_column_64_offset_0: QM31,
    trace_1_column_65_offset_0: QM31,
    trace_1_column_66_offset_0: QM31,
    trace_1_column_67_offset_0: QM31,
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
) -> QM31 {
    (PoseidonRoundKeys_alpha0) * (trace_1_column_1_offset_0)
        + (PoseidonRoundKeys_alpha1) * (trace_1_column_62_offset_0)
        + (PoseidonRoundKeys_alpha2) * (trace_1_column_63_offset_0)
        + (PoseidonRoundKeys_alpha3) * (trace_1_column_64_offset_0)
        + (PoseidonRoundKeys_alpha4) * (trace_1_column_65_offset_0)
        + (PoseidonRoundKeys_alpha5) * (trace_1_column_66_offset_0)
        + (PoseidonRoundKeys_alpha6) * (trace_1_column_67_offset_0)
        + (PoseidonRoundKeys_alpha7) * (trace_1_column_68_offset_0)
        + (PoseidonRoundKeys_alpha8) * (trace_1_column_69_offset_0)
        + (PoseidonRoundKeys_alpha9) * (trace_1_column_70_offset_0)
        + (PoseidonRoundKeys_alpha10) * (trace_1_column_71_offset_0)
        + (PoseidonRoundKeys_alpha11) * (trace_1_column_72_offset_0)
        + (PoseidonRoundKeys_alpha12) * (trace_1_column_73_offset_0)
        + (PoseidonRoundKeys_alpha13) * (trace_1_column_74_offset_0)
        + (PoseidonRoundKeys_alpha14) * (trace_1_column_75_offset_0)
        + (PoseidonRoundKeys_alpha15) * (trace_1_column_76_offset_0)
        + (PoseidonRoundKeys_alpha16) * (trace_1_column_77_offset_0)
        + (PoseidonRoundKeys_alpha17) * (trace_1_column_78_offset_0)
        + (PoseidonRoundKeys_alpha18) * (trace_1_column_79_offset_0)
        + (PoseidonRoundKeys_alpha19) * (trace_1_column_80_offset_0)
        + (PoseidonRoundKeys_alpha20) * (trace_1_column_81_offset_0)
        + (PoseidonRoundKeys_alpha21) * (trace_1_column_82_offset_0)
        + (PoseidonRoundKeys_alpha22) * (trace_1_column_83_offset_0)
        + (PoseidonRoundKeys_alpha23) * (trace_1_column_84_offset_0)
        + (PoseidonRoundKeys_alpha24) * (trace_1_column_85_offset_0)
        + (PoseidonRoundKeys_alpha25) * (trace_1_column_86_offset_0)
        + (PoseidonRoundKeys_alpha26) * (trace_1_column_87_offset_0)
        + (PoseidonRoundKeys_alpha27) * (trace_1_column_88_offset_0)
        + (PoseidonRoundKeys_alpha28) * (trace_1_column_89_offset_0)
        + (PoseidonRoundKeys_alpha29) * (trace_1_column_90_offset_0)
        + (PoseidonRoundKeys_alpha30) * (trace_1_column_91_offset_0)
        - (PoseidonRoundKeys_z)
}

pub fn intermediate14(
    RangeCheck_3_3_3_3_3_alpha0: QM31,
    RangeCheck_3_3_3_3_3_alpha1: QM31,
    RangeCheck_3_3_3_3_3_alpha2: QM31,
    RangeCheck_3_3_3_3_3_alpha3: QM31,
    RangeCheck_3_3_3_3_3_alpha4: QM31,
    RangeCheck_3_3_3_3_3_z: QM31,
    intermediate10: QM31,
    intermediate11: QM31,
    intermediate12: QM31,
    intermediate8: QM31,
    intermediate9: QM31,
) -> QM31 {
    (RangeCheck_3_3_3_3_3_alpha0) * (intermediate8 + m31(1).into())
        + (RangeCheck_3_3_3_3_3_alpha1) * (intermediate9 + m31(1).into())
        + (RangeCheck_3_3_3_3_3_alpha2) * (intermediate10 + m31(1).into())
        + (RangeCheck_3_3_3_3_3_alpha3) * (intermediate11 + m31(1).into())
        + (RangeCheck_3_3_3_3_3_alpha4) * (intermediate12 + m31(1).into())
        - (RangeCheck_3_3_3_3_3_z)
}

pub fn intermediate35(
    RangeCheck_3_3_3_3_3_alpha0: QM31,
    RangeCheck_3_3_3_3_3_alpha1: QM31,
    RangeCheck_3_3_3_3_3_alpha2: QM31,
    RangeCheck_3_3_3_3_3_alpha3: QM31,
    RangeCheck_3_3_3_3_3_alpha4: QM31,
    RangeCheck_3_3_3_3_3_z: QM31,
    intermediate26: QM31,
    intermediate27: QM31,
    intermediate28: QM31,
    intermediate29: QM31,
    trace_1_column_124_offset_0: QM31,
) -> QM31 {
    (RangeCheck_3_3_3_3_3_alpha0) * (trace_1_column_124_offset_0 + m31(3).into())
        + (RangeCheck_3_3_3_3_3_alpha1) * (intermediate26 + m31(3).into())
        + (RangeCheck_3_3_3_3_3_alpha2) * (intermediate27 + m31(3).into())
        + (RangeCheck_3_3_3_3_3_alpha3) * (intermediate28 + m31(3).into())
        + (RangeCheck_3_3_3_3_3_alpha4) * (intermediate29 + m31(3).into())
        - (RangeCheck_3_3_3_3_3_z)
}

pub fn intermediate36(
    RangeCheck_3_3_3_3_3_alpha0: QM31,
    RangeCheck_3_3_3_3_3_alpha1: QM31,
    RangeCheck_3_3_3_3_3_alpha2: QM31,
    RangeCheck_3_3_3_3_3_alpha3: QM31,
    RangeCheck_3_3_3_3_3_alpha4: QM31,
    RangeCheck_3_3_3_3_3_z: QM31,
    intermediate30: QM31,
    intermediate31: QM31,
    intermediate32: QM31,
    intermediate33: QM31,
    intermediate34: QM31,
) -> QM31 {
    (RangeCheck_3_3_3_3_3_alpha0) * (intermediate30 + m31(3).into())
        + (RangeCheck_3_3_3_3_3_alpha1) * (intermediate31 + m31(3).into())
        + (RangeCheck_3_3_3_3_3_alpha2) * (intermediate32 + m31(3).into())
        + (RangeCheck_3_3_3_3_3_alpha3) * (intermediate33 + m31(3).into())
        + (RangeCheck_3_3_3_3_3_alpha4) * (intermediate34 + m31(3).into())
        - (RangeCheck_3_3_3_3_3_z)
}

pub fn intermediate13(
    RangeCheck_3_3_3_3_3_alpha0: QM31,
    RangeCheck_3_3_3_3_3_alpha1: QM31,
    RangeCheck_3_3_3_3_3_alpha2: QM31,
    RangeCheck_3_3_3_3_3_alpha3: QM31,
    RangeCheck_3_3_3_3_3_alpha4: QM31,
    RangeCheck_3_3_3_3_3_z: QM31,
    intermediate4: QM31,
    intermediate5: QM31,
    intermediate6: QM31,
    intermediate7: QM31,
    trace_1_column_102_offset_0: QM31,
) -> QM31 {
    (RangeCheck_3_3_3_3_3_alpha0) * (trace_1_column_102_offset_0 + m31(1).into())
        + (RangeCheck_3_3_3_3_3_alpha1) * (intermediate4 + m31(1).into())
        + (RangeCheck_3_3_3_3_3_alpha2) * (intermediate5 + m31(1).into())
        + (RangeCheck_3_3_3_3_3_alpha3) * (intermediate6 + m31(1).into())
        + (RangeCheck_3_3_3_3_3_alpha4) * (intermediate7 + m31(1).into())
        - (RangeCheck_3_3_3_3_3_z)
}

pub fn intermediate2(
    Cube252_alpha0: QM31,
    Cube252_alpha1: QM31,
    Cube252_alpha10: QM31,
    Cube252_alpha11: QM31,
    Cube252_alpha12: QM31,
    Cube252_alpha13: QM31,
    Cube252_alpha14: QM31,
    Cube252_alpha15: QM31,
    Cube252_alpha16: QM31,
    Cube252_alpha17: QM31,
    Cube252_alpha18: QM31,
    Cube252_alpha19: QM31,
    Cube252_alpha2: QM31,
    Cube252_alpha3: QM31,
    Cube252_alpha4: QM31,
    Cube252_alpha5: QM31,
    Cube252_alpha6: QM31,
    Cube252_alpha7: QM31,
    Cube252_alpha8: QM31,
    Cube252_alpha9: QM31,
    Cube252_z: QM31,
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
) -> QM31 {
    (Cube252_alpha0) * (trace_1_column_22_offset_0)
        + (Cube252_alpha1) * (trace_1_column_23_offset_0)
        + (Cube252_alpha2) * (trace_1_column_24_offset_0)
        + (Cube252_alpha3) * (trace_1_column_25_offset_0)
        + (Cube252_alpha4) * (trace_1_column_26_offset_0)
        + (Cube252_alpha5) * (trace_1_column_27_offset_0)
        + (Cube252_alpha6) * (trace_1_column_28_offset_0)
        + (Cube252_alpha7) * (trace_1_column_29_offset_0)
        + (Cube252_alpha8) * (trace_1_column_30_offset_0)
        + (Cube252_alpha9) * (trace_1_column_31_offset_0)
        + (Cube252_alpha10) * (trace_1_column_52_offset_0)
        + (Cube252_alpha11) * (trace_1_column_53_offset_0)
        + (Cube252_alpha12) * (trace_1_column_54_offset_0)
        + (Cube252_alpha13) * (trace_1_column_55_offset_0)
        + (Cube252_alpha14) * (trace_1_column_56_offset_0)
        + (Cube252_alpha15) * (trace_1_column_57_offset_0)
        + (Cube252_alpha16) * (trace_1_column_58_offset_0)
        + (Cube252_alpha17) * (trace_1_column_59_offset_0)
        + (Cube252_alpha18) * (trace_1_column_60_offset_0)
        + (Cube252_alpha19) * (trace_1_column_61_offset_0)
        - (Cube252_z)
}

pub fn intermediate37(
    PoseidonFullRoundChain_alpha0: QM31,
    PoseidonFullRoundChain_alpha1: QM31,
    PoseidonFullRoundChain_alpha10: QM31,
    PoseidonFullRoundChain_alpha11: QM31,
    PoseidonFullRoundChain_alpha12: QM31,
    PoseidonFullRoundChain_alpha13: QM31,
    PoseidonFullRoundChain_alpha14: QM31,
    PoseidonFullRoundChain_alpha15: QM31,
    PoseidonFullRoundChain_alpha16: QM31,
    PoseidonFullRoundChain_alpha17: QM31,
    PoseidonFullRoundChain_alpha18: QM31,
    PoseidonFullRoundChain_alpha19: QM31,
    PoseidonFullRoundChain_alpha2: QM31,
    PoseidonFullRoundChain_alpha20: QM31,
    PoseidonFullRoundChain_alpha21: QM31,
    PoseidonFullRoundChain_alpha22: QM31,
    PoseidonFullRoundChain_alpha23: QM31,
    PoseidonFullRoundChain_alpha24: QM31,
    PoseidonFullRoundChain_alpha25: QM31,
    PoseidonFullRoundChain_alpha26: QM31,
    PoseidonFullRoundChain_alpha27: QM31,
    PoseidonFullRoundChain_alpha28: QM31,
    PoseidonFullRoundChain_alpha29: QM31,
    PoseidonFullRoundChain_alpha3: QM31,
    PoseidonFullRoundChain_alpha30: QM31,
    PoseidonFullRoundChain_alpha31: QM31,
    PoseidonFullRoundChain_alpha4: QM31,
    PoseidonFullRoundChain_alpha5: QM31,
    PoseidonFullRoundChain_alpha6: QM31,
    PoseidonFullRoundChain_alpha7: QM31,
    PoseidonFullRoundChain_alpha8: QM31,
    PoseidonFullRoundChain_alpha9: QM31,
    PoseidonFullRoundChain_z: QM31,
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
) -> QM31 {
    (PoseidonFullRoundChain_alpha0) * (trace_1_column_0_offset_0)
        + (PoseidonFullRoundChain_alpha1) * (trace_1_column_1_offset_0)
        + (PoseidonFullRoundChain_alpha2) * (trace_1_column_2_offset_0)
        + (PoseidonFullRoundChain_alpha3) * (trace_1_column_3_offset_0)
        + (PoseidonFullRoundChain_alpha4) * (trace_1_column_4_offset_0)
        + (PoseidonFullRoundChain_alpha5) * (trace_1_column_5_offset_0)
        + (PoseidonFullRoundChain_alpha6) * (trace_1_column_6_offset_0)
        + (PoseidonFullRoundChain_alpha7) * (trace_1_column_7_offset_0)
        + (PoseidonFullRoundChain_alpha8) * (trace_1_column_8_offset_0)
        + (PoseidonFullRoundChain_alpha9) * (trace_1_column_9_offset_0)
        + (PoseidonFullRoundChain_alpha10) * (trace_1_column_10_offset_0)
        + (PoseidonFullRoundChain_alpha11) * (trace_1_column_11_offset_0)
        + (PoseidonFullRoundChain_alpha12) * (trace_1_column_12_offset_0)
        + (PoseidonFullRoundChain_alpha13) * (trace_1_column_13_offset_0)
        + (PoseidonFullRoundChain_alpha14) * (trace_1_column_14_offset_0)
        + (PoseidonFullRoundChain_alpha15) * (trace_1_column_15_offset_0)
        + (PoseidonFullRoundChain_alpha16) * (trace_1_column_16_offset_0)
        + (PoseidonFullRoundChain_alpha17) * (trace_1_column_17_offset_0)
        + (PoseidonFullRoundChain_alpha18) * (trace_1_column_18_offset_0)
        + (PoseidonFullRoundChain_alpha19) * (trace_1_column_19_offset_0)
        + (PoseidonFullRoundChain_alpha20) * (trace_1_column_20_offset_0)
        + (PoseidonFullRoundChain_alpha21) * (trace_1_column_21_offset_0)
        + (PoseidonFullRoundChain_alpha22) * (trace_1_column_22_offset_0)
        + (PoseidonFullRoundChain_alpha23) * (trace_1_column_23_offset_0)
        + (PoseidonFullRoundChain_alpha24) * (trace_1_column_24_offset_0)
        + (PoseidonFullRoundChain_alpha25) * (trace_1_column_25_offset_0)
        + (PoseidonFullRoundChain_alpha26) * (trace_1_column_26_offset_0)
        + (PoseidonFullRoundChain_alpha27) * (trace_1_column_27_offset_0)
        + (PoseidonFullRoundChain_alpha28) * (trace_1_column_28_offset_0)
        + (PoseidonFullRoundChain_alpha29) * (trace_1_column_29_offset_0)
        + (PoseidonFullRoundChain_alpha30) * (trace_1_column_30_offset_0)
        + (PoseidonFullRoundChain_alpha31) * (trace_1_column_31_offset_0)
        - (PoseidonFullRoundChain_z)
}

