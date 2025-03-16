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
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
    trace_mask_points.append(array![point]);
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
    pub Poseidon3PartialRoundsChain_alpha0: QM31,
    pub Poseidon3PartialRoundsChain_alpha1: QM31,
    pub Poseidon3PartialRoundsChain_alpha10: QM31,
    pub Poseidon3PartialRoundsChain_alpha11: QM31,
    pub Poseidon3PartialRoundsChain_alpha12: QM31,
    pub Poseidon3PartialRoundsChain_alpha13: QM31,
    pub Poseidon3PartialRoundsChain_alpha14: QM31,
    pub Poseidon3PartialRoundsChain_alpha15: QM31,
    pub Poseidon3PartialRoundsChain_alpha16: QM31,
    pub Poseidon3PartialRoundsChain_alpha17: QM31,
    pub Poseidon3PartialRoundsChain_alpha18: QM31,
    pub Poseidon3PartialRoundsChain_alpha19: QM31,
    pub Poseidon3PartialRoundsChain_alpha2: QM31,
    pub Poseidon3PartialRoundsChain_alpha20: QM31,
    pub Poseidon3PartialRoundsChain_alpha21: QM31,
    pub Poseidon3PartialRoundsChain_alpha22: QM31,
    pub Poseidon3PartialRoundsChain_alpha23: QM31,
    pub Poseidon3PartialRoundsChain_alpha24: QM31,
    pub Poseidon3PartialRoundsChain_alpha25: QM31,
    pub Poseidon3PartialRoundsChain_alpha26: QM31,
    pub Poseidon3PartialRoundsChain_alpha27: QM31,
    pub Poseidon3PartialRoundsChain_alpha28: QM31,
    pub Poseidon3PartialRoundsChain_alpha29: QM31,
    pub Poseidon3PartialRoundsChain_alpha3: QM31,
    pub Poseidon3PartialRoundsChain_alpha30: QM31,
    pub Poseidon3PartialRoundsChain_alpha31: QM31,
    pub Poseidon3PartialRoundsChain_alpha32: QM31,
    pub Poseidon3PartialRoundsChain_alpha33: QM31,
    pub Poseidon3PartialRoundsChain_alpha34: QM31,
    pub Poseidon3PartialRoundsChain_alpha35: QM31,
    pub Poseidon3PartialRoundsChain_alpha36: QM31,
    pub Poseidon3PartialRoundsChain_alpha37: QM31,
    pub Poseidon3PartialRoundsChain_alpha38: QM31,
    pub Poseidon3PartialRoundsChain_alpha39: QM31,
    pub Poseidon3PartialRoundsChain_alpha4: QM31,
    pub Poseidon3PartialRoundsChain_alpha40: QM31,
    pub Poseidon3PartialRoundsChain_alpha41: QM31,
    pub Poseidon3PartialRoundsChain_alpha5: QM31,
    pub Poseidon3PartialRoundsChain_alpha6: QM31,
    pub Poseidon3PartialRoundsChain_alpha7: QM31,
    pub Poseidon3PartialRoundsChain_alpha8: QM31,
    pub Poseidon3PartialRoundsChain_alpha9: QM31,
    pub Poseidon3PartialRoundsChain_z: QM31,
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
    pub RangeCheckFelt252Width27_alpha0: QM31,
    pub RangeCheckFelt252Width27_alpha1: QM31,
    pub RangeCheckFelt252Width27_alpha2: QM31,
    pub RangeCheckFelt252Width27_alpha3: QM31,
    pub RangeCheckFelt252Width27_alpha4: QM31,
    pub RangeCheckFelt252Width27_alpha5: QM31,
    pub RangeCheckFelt252Width27_alpha6: QM31,
    pub RangeCheckFelt252Width27_alpha7: QM31,
    pub RangeCheckFelt252Width27_alpha8: QM31,
    pub RangeCheckFelt252Width27_alpha9: QM31,
    pub RangeCheckFelt252Width27_z: QM31,
    pub RangeCheck_4_4_4_4_alpha0: QM31,
    pub RangeCheck_4_4_4_4_alpha1: QM31,
    pub RangeCheck_4_4_4_4_alpha2: QM31,
    pub RangeCheck_4_4_4_4_alpha3: QM31,
    pub RangeCheck_4_4_4_4_z: QM31,
    pub RangeCheck_4_4_alpha0: QM31,
    pub RangeCheck_4_4_alpha1: QM31,
    pub RangeCheck_4_4_z: QM31,
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
        Poseidon3PartialRoundsChain_alpha0,
        Poseidon3PartialRoundsChain_alpha1,
        Poseidon3PartialRoundsChain_alpha10,
        Poseidon3PartialRoundsChain_alpha11,
        Poseidon3PartialRoundsChain_alpha12,
        Poseidon3PartialRoundsChain_alpha13,
        Poseidon3PartialRoundsChain_alpha14,
        Poseidon3PartialRoundsChain_alpha15,
        Poseidon3PartialRoundsChain_alpha16,
        Poseidon3PartialRoundsChain_alpha17,
        Poseidon3PartialRoundsChain_alpha18,
        Poseidon3PartialRoundsChain_alpha19,
        Poseidon3PartialRoundsChain_alpha2,
        Poseidon3PartialRoundsChain_alpha20,
        Poseidon3PartialRoundsChain_alpha21,
        Poseidon3PartialRoundsChain_alpha22,
        Poseidon3PartialRoundsChain_alpha23,
        Poseidon3PartialRoundsChain_alpha24,
        Poseidon3PartialRoundsChain_alpha25,
        Poseidon3PartialRoundsChain_alpha26,
        Poseidon3PartialRoundsChain_alpha27,
        Poseidon3PartialRoundsChain_alpha28,
        Poseidon3PartialRoundsChain_alpha29,
        Poseidon3PartialRoundsChain_alpha3,
        Poseidon3PartialRoundsChain_alpha30,
        Poseidon3PartialRoundsChain_alpha31,
        Poseidon3PartialRoundsChain_alpha32,
        Poseidon3PartialRoundsChain_alpha33,
        Poseidon3PartialRoundsChain_alpha34,
        Poseidon3PartialRoundsChain_alpha35,
        Poseidon3PartialRoundsChain_alpha36,
        Poseidon3PartialRoundsChain_alpha37,
        Poseidon3PartialRoundsChain_alpha38,
        Poseidon3PartialRoundsChain_alpha39,
        Poseidon3PartialRoundsChain_alpha4,
        Poseidon3PartialRoundsChain_alpha40,
        Poseidon3PartialRoundsChain_alpha41,
        Poseidon3PartialRoundsChain_alpha5,
        Poseidon3PartialRoundsChain_alpha6,
        Poseidon3PartialRoundsChain_alpha7,
        Poseidon3PartialRoundsChain_alpha8,
        Poseidon3PartialRoundsChain_alpha9,
        Poseidon3PartialRoundsChain_z,
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
        RangeCheckFelt252Width27_alpha0,
        RangeCheckFelt252Width27_alpha1,
        RangeCheckFelt252Width27_alpha2,
        RangeCheckFelt252Width27_alpha3,
        RangeCheckFelt252Width27_alpha4,
        RangeCheckFelt252Width27_alpha5,
        RangeCheckFelt252Width27_alpha6,
        RangeCheckFelt252Width27_alpha7,
        RangeCheckFelt252Width27_alpha8,
        RangeCheckFelt252Width27_alpha9,
        RangeCheckFelt252Width27_z,
        RangeCheck_4_4_4_4_alpha0,
        RangeCheck_4_4_4_4_alpha1,
        RangeCheck_4_4_4_4_alpha2,
        RangeCheck_4_4_4_4_alpha3,
        RangeCheck_4_4_4_4_z,
        RangeCheck_4_4_alpha0,
        RangeCheck_4_4_alpha1,
        RangeCheck_4_4_z,
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
        trace_1_column_126,
        trace_1_column_127,
        trace_1_column_128,
        trace_1_column_129,
        trace_1_column_130,
        trace_1_column_131,
        trace_1_column_132,
        trace_1_column_133,
        trace_1_column_134,
        trace_1_column_135,
        trace_1_column_136,
        trace_1_column_137,
        trace_1_column_138,
        trace_1_column_139,
        trace_1_column_140,
        trace_1_column_141,
        trace_1_column_142,
        trace_1_column_143,
        trace_1_column_144,
        trace_1_column_145,
        trace_1_column_146,
        trace_1_column_147,
        trace_1_column_148,
        trace_1_column_149,
        trace_1_column_150,
        trace_1_column_151,
        trace_1_column_152,
        trace_1_column_153,
        trace_1_column_154,
        trace_1_column_155,
        trace_1_column_156,
        trace_1_column_157,
        trace_1_column_158,
        trace_1_column_159,
        trace_1_column_160,
        trace_1_column_161,
        trace_1_column_162,
        trace_1_column_163,
        trace_1_column_164,
        trace_1_column_165,
        trace_1_column_166,
        trace_1_column_167,
        trace_1_column_168,
    ]: [Span<QM31>; 169] =
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

    let [trace_1_column_126_offset_0]: [QM31; 1] = (*trace_1_column_126.try_into().unwrap())
        .unbox();

    let [trace_1_column_127_offset_0]: [QM31; 1] = (*trace_1_column_127.try_into().unwrap())
        .unbox();

    let [trace_1_column_128_offset_0]: [QM31; 1] = (*trace_1_column_128.try_into().unwrap())
        .unbox();

    let [trace_1_column_129_offset_0]: [QM31; 1] = (*trace_1_column_129.try_into().unwrap())
        .unbox();

    let [trace_1_column_130_offset_0]: [QM31; 1] = (*trace_1_column_130.try_into().unwrap())
        .unbox();

    let [trace_1_column_131_offset_0]: [QM31; 1] = (*trace_1_column_131.try_into().unwrap())
        .unbox();

    let [trace_1_column_132_offset_0]: [QM31; 1] = (*trace_1_column_132.try_into().unwrap())
        .unbox();

    let [trace_1_column_133_offset_0]: [QM31; 1] = (*trace_1_column_133.try_into().unwrap())
        .unbox();

    let [trace_1_column_134_offset_0]: [QM31; 1] = (*trace_1_column_134.try_into().unwrap())
        .unbox();

    let [trace_1_column_135_offset_0]: [QM31; 1] = (*trace_1_column_135.try_into().unwrap())
        .unbox();

    let [trace_1_column_136_offset_0]: [QM31; 1] = (*trace_1_column_136.try_into().unwrap())
        .unbox();

    let [trace_1_column_137_offset_0]: [QM31; 1] = (*trace_1_column_137.try_into().unwrap())
        .unbox();

    let [trace_1_column_138_offset_0]: [QM31; 1] = (*trace_1_column_138.try_into().unwrap())
        .unbox();

    let [trace_1_column_139_offset_0]: [QM31; 1] = (*trace_1_column_139.try_into().unwrap())
        .unbox();

    let [trace_1_column_140_offset_0]: [QM31; 1] = (*trace_1_column_140.try_into().unwrap())
        .unbox();

    let [trace_1_column_141_offset_0]: [QM31; 1] = (*trace_1_column_141.try_into().unwrap())
        .unbox();

    let [trace_1_column_142_offset_0]: [QM31; 1] = (*trace_1_column_142.try_into().unwrap())
        .unbox();

    let [trace_1_column_143_offset_0]: [QM31; 1] = (*trace_1_column_143.try_into().unwrap())
        .unbox();

    let [trace_1_column_144_offset_0]: [QM31; 1] = (*trace_1_column_144.try_into().unwrap())
        .unbox();

    let [trace_1_column_145_offset_0]: [QM31; 1] = (*trace_1_column_145.try_into().unwrap())
        .unbox();

    let [trace_1_column_146_offset_0]: [QM31; 1] = (*trace_1_column_146.try_into().unwrap())
        .unbox();

    let [trace_1_column_147_offset_0]: [QM31; 1] = (*trace_1_column_147.try_into().unwrap())
        .unbox();

    let [trace_1_column_148_offset_0]: [QM31; 1] = (*trace_1_column_148.try_into().unwrap())
        .unbox();

    let [trace_1_column_149_offset_0]: [QM31; 1] = (*trace_1_column_149.try_into().unwrap())
        .unbox();

    let [trace_1_column_150_offset_0]: [QM31; 1] = (*trace_1_column_150.try_into().unwrap())
        .unbox();

    let [trace_1_column_151_offset_0]: [QM31; 1] = (*trace_1_column_151.try_into().unwrap())
        .unbox();

    let [trace_1_column_152_offset_0]: [QM31; 1] = (*trace_1_column_152.try_into().unwrap())
        .unbox();

    let [trace_1_column_153_offset_0]: [QM31; 1] = (*trace_1_column_153.try_into().unwrap())
        .unbox();

    let [trace_1_column_154_offset_0]: [QM31; 1] = (*trace_1_column_154.try_into().unwrap())
        .unbox();

    let [trace_1_column_155_offset_0]: [QM31; 1] = (*trace_1_column_155.try_into().unwrap())
        .unbox();

    let [trace_1_column_156_offset_0]: [QM31; 1] = (*trace_1_column_156.try_into().unwrap())
        .unbox();

    let [trace_1_column_157_offset_0]: [QM31; 1] = (*trace_1_column_157.try_into().unwrap())
        .unbox();

    let [trace_1_column_158_offset_0]: [QM31; 1] = (*trace_1_column_158.try_into().unwrap())
        .unbox();

    let [trace_1_column_159_offset_0]: [QM31; 1] = (*trace_1_column_159.try_into().unwrap())
        .unbox();

    let [trace_1_column_160_offset_0]: [QM31; 1] = (*trace_1_column_160.try_into().unwrap())
        .unbox();

    let [trace_1_column_161_offset_0]: [QM31; 1] = (*trace_1_column_161.try_into().unwrap())
        .unbox();

    let [trace_1_column_162_offset_0]: [QM31; 1] = (*trace_1_column_162.try_into().unwrap())
        .unbox();

    let [trace_1_column_163_offset_0]: [QM31; 1] = (*trace_1_column_163.try_into().unwrap())
        .unbox();

    let [trace_1_column_164_offset_0]: [QM31; 1] = (*trace_1_column_164.try_into().unwrap())
        .unbox();

    let [trace_1_column_165_offset_0]: [QM31; 1] = (*trace_1_column_165.try_into().unwrap())
        .unbox();

    let [trace_1_column_166_offset_0]: [QM31; 1] = (*trace_1_column_166.try_into().unwrap())
        .unbox();

    let [trace_1_column_167_offset_0]: [QM31; 1] = (*trace_1_column_167.try_into().unwrap())
        .unbox();

    let [trace_1_column_168_offset_0]: [QM31; 1] = (*trace_1_column_168.try_into().unwrap())
        .unbox();

    let [
        trace_2_column_169,
        trace_2_column_170,
        trace_2_column_171,
        trace_2_column_172,
        trace_2_column_173,
        trace_2_column_174,
        trace_2_column_175,
        trace_2_column_176,
        trace_2_column_177,
        trace_2_column_178,
        trace_2_column_179,
        trace_2_column_180,
        trace_2_column_181,
        trace_2_column_182,
        trace_2_column_183,
        trace_2_column_184,
        trace_2_column_185,
        trace_2_column_186,
        trace_2_column_187,
        trace_2_column_188,
        trace_2_column_189,
        trace_2_column_190,
        trace_2_column_191,
        trace_2_column_192,
        trace_2_column_193,
        trace_2_column_194,
        trace_2_column_195,
        trace_2_column_196,
        trace_2_column_197,
        trace_2_column_198,
        trace_2_column_199,
        trace_2_column_200,
        trace_2_column_201,
        trace_2_column_202,
        trace_2_column_203,
        trace_2_column_204,
    ]: [Span<QM31>; 36] =
        (*interaction_mask_values
        .multi_pop_front()
        .unwrap())
        .unbox();

    let [trace_2_column_169_offset_0]: [QM31; 1] = (*trace_2_column_169.try_into().unwrap())
        .unbox();

    let [trace_2_column_170_offset_0]: [QM31; 1] = (*trace_2_column_170.try_into().unwrap())
        .unbox();

    let [trace_2_column_171_offset_0]: [QM31; 1] = (*trace_2_column_171.try_into().unwrap())
        .unbox();

    let [trace_2_column_172_offset_0]: [QM31; 1] = (*trace_2_column_172.try_into().unwrap())
        .unbox();

    let [trace_2_column_173_offset_0]: [QM31; 1] = (*trace_2_column_173.try_into().unwrap())
        .unbox();

    let [trace_2_column_174_offset_0]: [QM31; 1] = (*trace_2_column_174.try_into().unwrap())
        .unbox();

    let [trace_2_column_175_offset_0]: [QM31; 1] = (*trace_2_column_175.try_into().unwrap())
        .unbox();

    let [trace_2_column_176_offset_0]: [QM31; 1] = (*trace_2_column_176.try_into().unwrap())
        .unbox();

    let [trace_2_column_177_offset_0]: [QM31; 1] = (*trace_2_column_177.try_into().unwrap())
        .unbox();

    let [trace_2_column_178_offset_0]: [QM31; 1] = (*trace_2_column_178.try_into().unwrap())
        .unbox();

    let [trace_2_column_179_offset_0]: [QM31; 1] = (*trace_2_column_179.try_into().unwrap())
        .unbox();

    let [trace_2_column_180_offset_0]: [QM31; 1] = (*trace_2_column_180.try_into().unwrap())
        .unbox();

    let [trace_2_column_181_offset_0]: [QM31; 1] = (*trace_2_column_181.try_into().unwrap())
        .unbox();

    let [trace_2_column_182_offset_0]: [QM31; 1] = (*trace_2_column_182.try_into().unwrap())
        .unbox();

    let [trace_2_column_183_offset_0]: [QM31; 1] = (*trace_2_column_183.try_into().unwrap())
        .unbox();

    let [trace_2_column_184_offset_0]: [QM31; 1] = (*trace_2_column_184.try_into().unwrap())
        .unbox();

    let [trace_2_column_185_offset_0]: [QM31; 1] = (*trace_2_column_185.try_into().unwrap())
        .unbox();

    let [trace_2_column_186_offset_0]: [QM31; 1] = (*trace_2_column_186.try_into().unwrap())
        .unbox();

    let [trace_2_column_187_offset_0]: [QM31; 1] = (*trace_2_column_187.try_into().unwrap())
        .unbox();

    let [trace_2_column_188_offset_0]: [QM31; 1] = (*trace_2_column_188.try_into().unwrap())
        .unbox();

    let [trace_2_column_189_offset_0]: [QM31; 1] = (*trace_2_column_189.try_into().unwrap())
        .unbox();

    let [trace_2_column_190_offset_0]: [QM31; 1] = (*trace_2_column_190.try_into().unwrap())
        .unbox();

    let [trace_2_column_191_offset_0]: [QM31; 1] = (*trace_2_column_191.try_into().unwrap())
        .unbox();

    let [trace_2_column_192_offset_0]: [QM31; 1] = (*trace_2_column_192.try_into().unwrap())
        .unbox();

    let [trace_2_column_193_offset_0]: [QM31; 1] = (*trace_2_column_193.try_into().unwrap())
        .unbox();

    let [trace_2_column_194_offset_0]: [QM31; 1] = (*trace_2_column_194.try_into().unwrap())
        .unbox();

    let [trace_2_column_195_offset_0]: [QM31; 1] = (*trace_2_column_195.try_into().unwrap())
        .unbox();

    let [trace_2_column_196_offset_0]: [QM31; 1] = (*trace_2_column_196.try_into().unwrap())
        .unbox();

    let [trace_2_column_197_offset_0]: [QM31; 1] = (*trace_2_column_197.try_into().unwrap())
        .unbox();

    let [trace_2_column_198_offset_0]: [QM31; 1] = (*trace_2_column_198.try_into().unwrap())
        .unbox();

    let [trace_2_column_199_offset_0]: [QM31; 1] = (*trace_2_column_199.try_into().unwrap())
        .unbox();

    let [trace_2_column_200_offset_0]: [QM31; 1] = (*trace_2_column_200.try_into().unwrap())
        .unbox();

    let [trace_2_column_201_offset_neg_1, trace_2_column_201_offset_0]: [QM31; 2] =
        (*trace_2_column_201
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_202_offset_neg_1, trace_2_column_202_offset_0]: [QM31; 2] =
        (*trace_2_column_202
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_203_offset_neg_1, trace_2_column_203_offset_0]: [QM31; 2] =
        (*trace_2_column_203
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_204_offset_neg_1, trace_2_column_204_offset_0]: [QM31; 2] =
        (*trace_2_column_204
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
        Poseidon3PartialRoundsChain_alpha0,
        Poseidon3PartialRoundsChain_alpha1,
        Poseidon3PartialRoundsChain_alpha10,
        Poseidon3PartialRoundsChain_alpha11,
        Poseidon3PartialRoundsChain_alpha12,
        Poseidon3PartialRoundsChain_alpha13,
        Poseidon3PartialRoundsChain_alpha14,
        Poseidon3PartialRoundsChain_alpha15,
        Poseidon3PartialRoundsChain_alpha16,
        Poseidon3PartialRoundsChain_alpha17,
        Poseidon3PartialRoundsChain_alpha18,
        Poseidon3PartialRoundsChain_alpha19,
        Poseidon3PartialRoundsChain_alpha2,
        Poseidon3PartialRoundsChain_alpha20,
        Poseidon3PartialRoundsChain_alpha21,
        Poseidon3PartialRoundsChain_alpha22,
        Poseidon3PartialRoundsChain_alpha23,
        Poseidon3PartialRoundsChain_alpha24,
        Poseidon3PartialRoundsChain_alpha25,
        Poseidon3PartialRoundsChain_alpha26,
        Poseidon3PartialRoundsChain_alpha27,
        Poseidon3PartialRoundsChain_alpha28,
        Poseidon3PartialRoundsChain_alpha29,
        Poseidon3PartialRoundsChain_alpha3,
        Poseidon3PartialRoundsChain_alpha30,
        Poseidon3PartialRoundsChain_alpha31,
        Poseidon3PartialRoundsChain_alpha32,
        Poseidon3PartialRoundsChain_alpha33,
        Poseidon3PartialRoundsChain_alpha34,
        Poseidon3PartialRoundsChain_alpha35,
        Poseidon3PartialRoundsChain_alpha36,
        Poseidon3PartialRoundsChain_alpha37,
        Poseidon3PartialRoundsChain_alpha38,
        Poseidon3PartialRoundsChain_alpha39,
        Poseidon3PartialRoundsChain_alpha4,
        Poseidon3PartialRoundsChain_alpha40,
        Poseidon3PartialRoundsChain_alpha41,
        Poseidon3PartialRoundsChain_alpha5,
        Poseidon3PartialRoundsChain_alpha6,
        Poseidon3PartialRoundsChain_alpha7,
        Poseidon3PartialRoundsChain_alpha8,
        Poseidon3PartialRoundsChain_alpha9,
        Poseidon3PartialRoundsChain_z,
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
        RangeCheckFelt252Width27_alpha0,
        RangeCheckFelt252Width27_alpha1,
        RangeCheckFelt252Width27_alpha2,
        RangeCheckFelt252Width27_alpha3,
        RangeCheckFelt252Width27_alpha4,
        RangeCheckFelt252Width27_alpha5,
        RangeCheckFelt252Width27_alpha6,
        RangeCheckFelt252Width27_alpha7,
        RangeCheckFelt252Width27_alpha8,
        RangeCheckFelt252Width27_alpha9,
        RangeCheckFelt252Width27_z,
        RangeCheck_4_4_4_4_alpha0,
        RangeCheck_4_4_4_4_alpha1,
        RangeCheck_4_4_4_4_alpha2,
        RangeCheck_4_4_4_4_alpha3,
        RangeCheck_4_4_4_4_z,
        RangeCheck_4_4_alpha0,
        RangeCheck_4_4_alpha1,
        RangeCheck_4_4_z,
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
        trace_1_column_125_offset_0,
        trace_1_column_126_offset_0,
        trace_1_column_127_offset_0,
        trace_1_column_128_offset_0,
        trace_1_column_129_offset_0,
        trace_1_column_12_offset_0,
        trace_1_column_130_offset_0,
        trace_1_column_131_offset_0,
        trace_1_column_132_offset_0,
        trace_1_column_133_offset_0,
        trace_1_column_134_offset_0,
        trace_1_column_135_offset_0,
        trace_1_column_136_offset_0,
        trace_1_column_137_offset_0,
        trace_1_column_138_offset_0,
        trace_1_column_139_offset_0,
        trace_1_column_13_offset_0,
        trace_1_column_140_offset_0,
        trace_1_column_141_offset_0,
        trace_1_column_142_offset_0,
        trace_1_column_143_offset_0,
        trace_1_column_144_offset_0,
        trace_1_column_145_offset_0,
        trace_1_column_146_offset_0,
        trace_1_column_147_offset_0,
        trace_1_column_148_offset_0,
        trace_1_column_149_offset_0,
        trace_1_column_14_offset_0,
        trace_1_column_150_offset_0,
        trace_1_column_151_offset_0,
        trace_1_column_152_offset_0,
        trace_1_column_153_offset_0,
        trace_1_column_154_offset_0,
        trace_1_column_155_offset_0,
        trace_1_column_156_offset_0,
        trace_1_column_157_offset_0,
        trace_1_column_158_offset_0,
        trace_1_column_159_offset_0,
        trace_1_column_15_offset_0,
        trace_1_column_160_offset_0,
        trace_1_column_161_offset_0,
        trace_1_column_162_offset_0,
        trace_1_column_163_offset_0,
        trace_1_column_164_offset_0,
        trace_1_column_165_offset_0,
        trace_1_column_166_offset_0,
        trace_1_column_167_offset_0,
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
    let intermediate39 = *intermediates.pop_front().unwrap();
    let intermediate40 = *intermediates.pop_front().unwrap();
    let intermediate41 = *intermediates.pop_front().unwrap();
    let intermediate42 = *intermediates.pop_front().unwrap();
    let intermediate43 = *intermediates.pop_front().unwrap();
    let intermediate44 = *intermediates.pop_front().unwrap();
    let intermediate45 = *intermediates.pop_front().unwrap();
    let intermediate46 = *intermediates.pop_front().unwrap();
    let intermediate47 = *intermediates.pop_front().unwrap();
    let intermediate48 = *intermediates.pop_front().unwrap();
    let intermediate49 = *intermediates.pop_front().unwrap();
    let intermediate50 = *intermediates.pop_front().unwrap();
    let intermediate51 = *intermediates.pop_front().unwrap();
    let intermediate52 = *intermediates.pop_front().unwrap();
    let intermediate53 = *intermediates.pop_front().unwrap();
    let intermediate54 = *intermediates.pop_front().unwrap();
    let intermediate55 = *intermediates.pop_front().unwrap();
    let intermediate56 = *intermediates.pop_front().unwrap();
    let intermediate57 = *intermediates.pop_front().unwrap();
    let intermediate58 = *intermediates.pop_front().unwrap();
    let intermediate59 = *intermediates.pop_front().unwrap();
    let intermediate60 = *intermediates.pop_front().unwrap();
    let intermediate61 = *intermediates.pop_front().unwrap();
    let intermediate62 = *intermediates.pop_front().unwrap();
    let intermediate63 = *intermediates.pop_front().unwrap();
    let intermediate64 = *intermediates.pop_front().unwrap();
    let intermediate65 = *intermediates.pop_front().unwrap();
    let intermediate66 = *intermediates.pop_front().unwrap();
    let intermediate67 = *intermediates.pop_front().unwrap();
    let intermediate68 = *intermediates.pop_front().unwrap();
    let intermediate69 = *intermediates.pop_front().unwrap();
    let intermediate70 = *intermediates.pop_front().unwrap();
    let intermediate71 = *intermediates.pop_front().unwrap();
    let intermediate72 = *intermediates.pop_front().unwrap();
    let intermediate73 = *intermediates.pop_front().unwrap();
    let intermediate74 = *intermediates.pop_front().unwrap();
    let intermediate75 = *intermediates.pop_front().unwrap();
    let intermediate76 = *intermediates.pop_front().unwrap();
    let intermediate77 = *intermediates.pop_front().unwrap();
    let intermediate78 = *intermediates.pop_front().unwrap();
    let intermediate79 = *intermediates.pop_front().unwrap();
    let intermediate80 = *intermediates.pop_front().unwrap();
    let intermediate81 = *intermediates.pop_front().unwrap();
    let intermediate82 = *intermediates.pop_front().unwrap();
    let intermediate83 = *intermediates.pop_front().unwrap();
    let intermediate84 = *intermediates.pop_front().unwrap();
    let intermediate85 = *intermediates.pop_front().unwrap();
    let intermediate86 = *intermediates.pop_front().unwrap();
    let intermediate87 = *intermediates.pop_front().unwrap();
    let intermediate88 = *intermediates.pop_front().unwrap();
    let intermediate89 = *intermediates.pop_front().unwrap();
    let intermediate90 = *intermediates.pop_front().unwrap();
    let intermediate91 = *intermediates.pop_front().unwrap();
    let intermediate92 = *intermediates.pop_front().unwrap();
    let intermediate93 = *intermediates.pop_front().unwrap();
    let intermediate94 = *intermediates.pop_front().unwrap();
    let intermediate95 = *intermediates.pop_front().unwrap();
    let intermediate96 = *intermediates.pop_front().unwrap();
    let intermediate97 = *intermediates.pop_front().unwrap();
    let intermediate98 = *intermediates.pop_front().unwrap();
    let intermediate99 = *intermediates.pop_front().unwrap();
    let intermediate100 = *intermediates.pop_front().unwrap();
    let intermediate101 = *intermediates.pop_front().unwrap();

    // Constraint 0
    let constraint_quotient = ((trace_1_column_168_offset_0) * (trace_1_column_168_offset_0)
        - (trace_1_column_168_offset_0))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 1
    let constraint_quotient = (intermediate10
        + (m31(4).into()) * (trace_1_column_11_offset_0)
        + (m31(2).into()) * (trace_1_column_21_offset_0)
        + (m31(3).into()) * (trace_1_column_31_offset_0)
        + trace_1_column_41_offset_0
        - (trace_1_column_81_offset_0)
        + trace_1_column_51_offset_0
        - (trace_1_column_91_offset_0)
        - ((trace_1_column_92_offset_0) * (m31(256).into())))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 2
    let constraint_quotient = (intermediate23
        + (m31(2).into()) * (trace_1_column_91_offset_0)
        - (trace_1_column_102_offset_0)
        - ((trace_1_column_103_offset_0) * (m31(256).into())))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 3
    let constraint_quotient = (((intermediate24) * (intermediate24)) * (intermediate24)
        - (intermediate24))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 4
    let constraint_quotient = (((intermediate25) * (intermediate25)) * (intermediate25)
        - (intermediate25))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 5
    let constraint_quotient = (((intermediate26) * (intermediate26)) * (intermediate26)
        - (intermediate26))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 6
    let constraint_quotient = (((intermediate27) * (intermediate27)) * (intermediate27)
        - (intermediate27))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 7
    let constraint_quotient = (((intermediate28) * (intermediate28)) * (intermediate28)
        - (intermediate28))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 8
    let constraint_quotient = (((intermediate29) * (intermediate29)) * (intermediate29)
        - (intermediate29))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 9
    let constraint_quotient = (((intermediate30) * (intermediate30)) * (intermediate30)
        - (intermediate30))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 10
    let constraint_quotient = (((intermediate31) * (intermediate31)) * (intermediate31)
        - (intermediate31))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 11
    let constraint_quotient = (((intermediate32) * (intermediate32)) * (intermediate32)
        - (intermediate32))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 12
    let constraint_quotient = (((intermediate33) * (intermediate33)) * (intermediate33)
        - (intermediate33))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 13
    let constraint_quotient = (intermediate43
        + (m31(4).into()) * (trace_1_column_31_offset_0)
        + (m31(2).into()) * (trace_1_column_41_offset_0)
        + (m31(3).into()) * (trace_1_column_81_offset_0)
        + trace_1_column_102_offset_0
        - (trace_1_column_113_offset_0)
        + trace_1_column_61_offset_0
        - (trace_1_column_123_offset_0)
        - ((trace_1_column_124_offset_0) * (m31(256).into())))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 14
    let constraint_quotient = (intermediate56
        + (m31(2).into()) * (trace_1_column_123_offset_0)
        - (trace_1_column_134_offset_0)
        - ((trace_1_column_135_offset_0) * (m31(256).into())))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 15
    let constraint_quotient = (((intermediate57) * (intermediate57)) * (intermediate57)
        - (intermediate57))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 16
    let constraint_quotient = (((intermediate58) * (intermediate58)) * (intermediate58)
        - (intermediate58))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 17
    let constraint_quotient = (((intermediate59) * (intermediate59)) * (intermediate59)
        - (intermediate59))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 18
    let constraint_quotient = (((intermediate60) * (intermediate60)) * (intermediate60)
        - (intermediate60))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 19
    let constraint_quotient = (((intermediate61) * (intermediate61)) * (intermediate61)
        - (intermediate61))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 20
    let constraint_quotient = (((intermediate62) * (intermediate62)) * (intermediate62)
        - (intermediate62))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 21
    let constraint_quotient = (((intermediate63) * (intermediate63)) * (intermediate63)
        - (intermediate63))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 22
    let constraint_quotient = (((intermediate64) * (intermediate64)) * (intermediate64)
        - (intermediate64))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 23
    let constraint_quotient = (((intermediate65) * (intermediate65)) * (intermediate65)
        - (intermediate65))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 24
    let constraint_quotient = (((intermediate66) * (intermediate66)) * (intermediate66)
        - (intermediate66))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 25
    let constraint_quotient = (intermediate76
        + (m31(4).into()) * (trace_1_column_81_offset_0)
        + (m31(2).into()) * (trace_1_column_102_offset_0)
        + (m31(3).into()) * (trace_1_column_113_offset_0)
        + trace_1_column_134_offset_0
        - (trace_1_column_145_offset_0)
        + trace_1_column_71_offset_0
        - (trace_1_column_155_offset_0)
        - ((trace_1_column_156_offset_0) * (m31(256).into())))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 26
    let constraint_quotient = (intermediate89
        + (m31(2).into()) * (trace_1_column_155_offset_0)
        - (trace_1_column_166_offset_0)
        - ((trace_1_column_167_offset_0) * (m31(256).into())))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 27
    let constraint_quotient = (((intermediate90) * (intermediate90)) * (intermediate90)
        - (intermediate90))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 28
    let constraint_quotient = (((intermediate91) * (intermediate91)) * (intermediate91)
        - (intermediate91))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    core::internal::revoke_ap_tracking();

    // Constraint 29
    let constraint_quotient = (((intermediate92) * (intermediate92)) * (intermediate92)
        - (intermediate92))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    core::internal::revoke_ap_tracking();

    // Constraint 30
    let constraint_quotient = (((intermediate93) * (intermediate93)) * (intermediate93)
        - (intermediate93))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 31
    let constraint_quotient = (((intermediate94) * (intermediate94)) * (intermediate94)
        - (intermediate94))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 32
    let constraint_quotient = (((intermediate95) * (intermediate95)) * (intermediate95)
        - (intermediate95))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 33
    let constraint_quotient = (((intermediate96) * (intermediate96)) * (intermediate96)
        - (intermediate96))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 34
    let constraint_quotient = (((intermediate97) * (intermediate97)) * (intermediate97)
        - (intermediate97))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 35
    let constraint_quotient = (((intermediate98) * (intermediate98)) * (intermediate98)
        - (intermediate98))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 36
    let constraint_quotient = (((intermediate99) * (intermediate99)) * (intermediate99)
        - (intermediate99))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 37
    let constraint_quotient = ((QM31Trait::from_partial_evals(
        [
            trace_2_column_169_offset_0, trace_2_column_170_offset_0, trace_2_column_171_offset_0,
            trace_2_column_172_offset_0,
        ],
    ))
        * ((intermediate0) * (intermediate1))
        - (intermediate1 + intermediate0))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 38
    let constraint_quotient = ((QM31Trait::from_partial_evals(
        [
            trace_2_column_173_offset_0, trace_2_column_174_offset_0, trace_2_column_175_offset_0,
            trace_2_column_176_offset_0,
        ],
    )
        - (QM31Trait::from_partial_evals(
            [
                trace_2_column_169_offset_0, trace_2_column_170_offset_0,
                trace_2_column_171_offset_0, trace_2_column_172_offset_0,
            ],
        )))
        * ((intermediate11) * (intermediate12))
        - (intermediate12 + intermediate11))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 39
    let constraint_quotient = ((QM31Trait::from_partial_evals(
        [
            trace_2_column_177_offset_0, trace_2_column_178_offset_0, trace_2_column_179_offset_0,
            trace_2_column_180_offset_0,
        ],
    )
        - (QM31Trait::from_partial_evals(
            [
                trace_2_column_173_offset_0, trace_2_column_174_offset_0,
                trace_2_column_175_offset_0, trace_2_column_176_offset_0,
            ],
        )))
        * ((intermediate13) * (intermediate14))
        - (intermediate14 + intermediate13))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 40
    let constraint_quotient = ((QM31Trait::from_partial_evals(
        [
            trace_2_column_181_offset_0, trace_2_column_182_offset_0, trace_2_column_183_offset_0,
            trace_2_column_184_offset_0,
        ],
    )
        - (QM31Trait::from_partial_evals(
            [
                trace_2_column_177_offset_0, trace_2_column_178_offset_0,
                trace_2_column_179_offset_0, trace_2_column_180_offset_0,
            ],
        )))
        * ((intermediate34) * (intermediate44))
        - (intermediate44 + intermediate34))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 41
    let constraint_quotient = ((QM31Trait::from_partial_evals(
        [
            trace_2_column_185_offset_0, trace_2_column_186_offset_0, trace_2_column_187_offset_0,
            trace_2_column_188_offset_0,
        ],
    )
        - (QM31Trait::from_partial_evals(
            [
                trace_2_column_181_offset_0, trace_2_column_182_offset_0,
                trace_2_column_183_offset_0, trace_2_column_184_offset_0,
            ],
        )))
        * ((intermediate45) * (intermediate46))
        - (intermediate46 + intermediate45))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    core::internal::revoke_ap_tracking();

    // Constraint 42
    let constraint_quotient = ((QM31Trait::from_partial_evals(
        [
            trace_2_column_189_offset_0, trace_2_column_190_offset_0, trace_2_column_191_offset_0,
            trace_2_column_192_offset_0,
        ],
    )
        - (QM31Trait::from_partial_evals(
            [
                trace_2_column_185_offset_0, trace_2_column_186_offset_0,
                trace_2_column_187_offset_0, trace_2_column_188_offset_0,
            ],
        )))
        * ((intermediate47) * (intermediate67))
        - (intermediate67 + intermediate47))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 43
    let constraint_quotient = ((QM31Trait::from_partial_evals(
        [
            trace_2_column_193_offset_0, trace_2_column_194_offset_0, trace_2_column_195_offset_0,
            trace_2_column_196_offset_0,
        ],
    )
        - (QM31Trait::from_partial_evals(
            [
                trace_2_column_189_offset_0, trace_2_column_190_offset_0,
                trace_2_column_191_offset_0, trace_2_column_192_offset_0,
            ],
        )))
        * ((intermediate77) * (intermediate78))
        - (intermediate78 + intermediate77))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 44
    let constraint_quotient = ((QM31Trait::from_partial_evals(
        [
            trace_2_column_197_offset_0, trace_2_column_198_offset_0, trace_2_column_199_offset_0,
            trace_2_column_200_offset_0,
        ],
    )
        - (QM31Trait::from_partial_evals(
            [
                trace_2_column_193_offset_0, trace_2_column_194_offset_0,
                trace_2_column_195_offset_0, trace_2_column_196_offset_0,
            ],
        )))
        * ((intermediate79) * (intermediate80))
        - (intermediate80 + intermediate79))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint 45
    let constraint_quotient = ((QM31Trait::from_partial_evals(
        [
            trace_2_column_201_offset_0, trace_2_column_202_offset_0, trace_2_column_203_offset_0,
            trace_2_column_204_offset_0,
        ],
    )
        - (QM31Trait::from_partial_evals(
            [
                trace_2_column_201_offset_neg_1, trace_2_column_202_offset_neg_1,
                trace_2_column_203_offset_neg_1, trace_2_column_204_offset_neg_1,
            ],
        ))
        - (QM31Trait::from_partial_evals(
            [
                trace_2_column_197_offset_0, trace_2_column_198_offset_0,
                trace_2_column_199_offset_0, trace_2_column_200_offset_0,
            ],
        ))
        + (claimed_sum) * (column_size.inverse().into()))
        * ((intermediate100) * (intermediate101))
        - ((intermediate101) * (trace_1_column_168_offset_0)
            - ((intermediate100) * (trace_1_column_168_offset_0))))
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
    Poseidon3PartialRoundsChain_alpha0: QM31,
    Poseidon3PartialRoundsChain_alpha1: QM31,
    Poseidon3PartialRoundsChain_alpha10: QM31,
    Poseidon3PartialRoundsChain_alpha11: QM31,
    Poseidon3PartialRoundsChain_alpha12: QM31,
    Poseidon3PartialRoundsChain_alpha13: QM31,
    Poseidon3PartialRoundsChain_alpha14: QM31,
    Poseidon3PartialRoundsChain_alpha15: QM31,
    Poseidon3PartialRoundsChain_alpha16: QM31,
    Poseidon3PartialRoundsChain_alpha17: QM31,
    Poseidon3PartialRoundsChain_alpha18: QM31,
    Poseidon3PartialRoundsChain_alpha19: QM31,
    Poseidon3PartialRoundsChain_alpha2: QM31,
    Poseidon3PartialRoundsChain_alpha20: QM31,
    Poseidon3PartialRoundsChain_alpha21: QM31,
    Poseidon3PartialRoundsChain_alpha22: QM31,
    Poseidon3PartialRoundsChain_alpha23: QM31,
    Poseidon3PartialRoundsChain_alpha24: QM31,
    Poseidon3PartialRoundsChain_alpha25: QM31,
    Poseidon3PartialRoundsChain_alpha26: QM31,
    Poseidon3PartialRoundsChain_alpha27: QM31,
    Poseidon3PartialRoundsChain_alpha28: QM31,
    Poseidon3PartialRoundsChain_alpha29: QM31,
    Poseidon3PartialRoundsChain_alpha3: QM31,
    Poseidon3PartialRoundsChain_alpha30: QM31,
    Poseidon3PartialRoundsChain_alpha31: QM31,
    Poseidon3PartialRoundsChain_alpha32: QM31,
    Poseidon3PartialRoundsChain_alpha33: QM31,
    Poseidon3PartialRoundsChain_alpha34: QM31,
    Poseidon3PartialRoundsChain_alpha35: QM31,
    Poseidon3PartialRoundsChain_alpha36: QM31,
    Poseidon3PartialRoundsChain_alpha37: QM31,
    Poseidon3PartialRoundsChain_alpha38: QM31,
    Poseidon3PartialRoundsChain_alpha39: QM31,
    Poseidon3PartialRoundsChain_alpha4: QM31,
    Poseidon3PartialRoundsChain_alpha40: QM31,
    Poseidon3PartialRoundsChain_alpha41: QM31,
    Poseidon3PartialRoundsChain_alpha5: QM31,
    Poseidon3PartialRoundsChain_alpha6: QM31,
    Poseidon3PartialRoundsChain_alpha7: QM31,
    Poseidon3PartialRoundsChain_alpha8: QM31,
    Poseidon3PartialRoundsChain_alpha9: QM31,
    Poseidon3PartialRoundsChain_z: QM31,
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
    RangeCheckFelt252Width27_alpha0: QM31,
    RangeCheckFelt252Width27_alpha1: QM31,
    RangeCheckFelt252Width27_alpha2: QM31,
    RangeCheckFelt252Width27_alpha3: QM31,
    RangeCheckFelt252Width27_alpha4: QM31,
    RangeCheckFelt252Width27_alpha5: QM31,
    RangeCheckFelt252Width27_alpha6: QM31,
    RangeCheckFelt252Width27_alpha7: QM31,
    RangeCheckFelt252Width27_alpha8: QM31,
    RangeCheckFelt252Width27_alpha9: QM31,
    RangeCheckFelt252Width27_z: QM31,
    RangeCheck_4_4_4_4_alpha0: QM31,
    RangeCheck_4_4_4_4_alpha1: QM31,
    RangeCheck_4_4_4_4_alpha2: QM31,
    RangeCheck_4_4_4_4_alpha3: QM31,
    RangeCheck_4_4_4_4_z: QM31,
    RangeCheck_4_4_alpha0: QM31,
    RangeCheck_4_4_alpha1: QM31,
    RangeCheck_4_4_z: QM31,
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
    trace_1_column_125_offset_0: QM31,
    trace_1_column_126_offset_0: QM31,
    trace_1_column_127_offset_0: QM31,
    trace_1_column_128_offset_0: QM31,
    trace_1_column_129_offset_0: QM31,
    trace_1_column_12_offset_0: QM31,
    trace_1_column_130_offset_0: QM31,
    trace_1_column_131_offset_0: QM31,
    trace_1_column_132_offset_0: QM31,
    trace_1_column_133_offset_0: QM31,
    trace_1_column_134_offset_0: QM31,
    trace_1_column_135_offset_0: QM31,
    trace_1_column_136_offset_0: QM31,
    trace_1_column_137_offset_0: QM31,
    trace_1_column_138_offset_0: QM31,
    trace_1_column_139_offset_0: QM31,
    trace_1_column_13_offset_0: QM31,
    trace_1_column_140_offset_0: QM31,
    trace_1_column_141_offset_0: QM31,
    trace_1_column_142_offset_0: QM31,
    trace_1_column_143_offset_0: QM31,
    trace_1_column_144_offset_0: QM31,
    trace_1_column_145_offset_0: QM31,
    trace_1_column_146_offset_0: QM31,
    trace_1_column_147_offset_0: QM31,
    trace_1_column_148_offset_0: QM31,
    trace_1_column_149_offset_0: QM31,
    trace_1_column_14_offset_0: QM31,
    trace_1_column_150_offset_0: QM31,
    trace_1_column_151_offset_0: QM31,
    trace_1_column_152_offset_0: QM31,
    trace_1_column_153_offset_0: QM31,
    trace_1_column_154_offset_0: QM31,
    trace_1_column_155_offset_0: QM31,
    trace_1_column_156_offset_0: QM31,
    trace_1_column_157_offset_0: QM31,
    trace_1_column_158_offset_0: QM31,
    trace_1_column_159_offset_0: QM31,
    trace_1_column_15_offset_0: QM31,
    trace_1_column_160_offset_0: QM31,
    trace_1_column_161_offset_0: QM31,
    trace_1_column_162_offset_0: QM31,
    trace_1_column_163_offset_0: QM31,
    trace_1_column_164_offset_0: QM31,
    trace_1_column_165_offset_0: QM31,
    trace_1_column_166_offset_0: QM31,
    trace_1_column_167_offset_0: QM31,
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
    let intermediate2 = intermediate2(
        trace_1_column_12_offset_0,
        trace_1_column_22_offset_0,
        trace_1_column_2_offset_0,
        trace_1_column_32_offset_0,
        trace_1_column_42_offset_0,
        trace_1_column_72_offset_0,
        trace_1_column_82_offset_0,
        trace_1_column_92_offset_0,
    );

    let intermediate3 = intermediate3(
        intermediate2,
        trace_1_column_13_offset_0,
        trace_1_column_23_offset_0,
        trace_1_column_33_offset_0,
        trace_1_column_3_offset_0,
        trace_1_column_43_offset_0,
        trace_1_column_73_offset_0,
        trace_1_column_83_offset_0,
    );

    let intermediate4 = intermediate4(
        intermediate3,
        trace_1_column_14_offset_0,
        trace_1_column_24_offset_0,
        trace_1_column_34_offset_0,
        trace_1_column_44_offset_0,
        trace_1_column_4_offset_0,
        trace_1_column_74_offset_0,
        trace_1_column_84_offset_0,
    );

    let intermediate5 = intermediate5(
        intermediate4,
        trace_1_column_15_offset_0,
        trace_1_column_25_offset_0,
        trace_1_column_35_offset_0,
        trace_1_column_45_offset_0,
        trace_1_column_5_offset_0,
        trace_1_column_75_offset_0,
        trace_1_column_85_offset_0,
    );

    let intermediate6 = intermediate6(
        intermediate5,
        trace_1_column_16_offset_0,
        trace_1_column_26_offset_0,
        trace_1_column_36_offset_0,
        trace_1_column_46_offset_0,
        trace_1_column_6_offset_0,
        trace_1_column_76_offset_0,
        trace_1_column_86_offset_0,
    );

    let intermediate7 = intermediate7(
        intermediate6,
        trace_1_column_17_offset_0,
        trace_1_column_27_offset_0,
        trace_1_column_37_offset_0,
        trace_1_column_47_offset_0,
        trace_1_column_77_offset_0,
        trace_1_column_7_offset_0,
        trace_1_column_87_offset_0,
    );

    let intermediate8 = intermediate8(
        intermediate7,
        trace_1_column_18_offset_0,
        trace_1_column_28_offset_0,
        trace_1_column_38_offset_0,
        trace_1_column_48_offset_0,
        trace_1_column_78_offset_0,
        trace_1_column_88_offset_0,
        trace_1_column_8_offset_0,
    );

    let intermediate9 = intermediate9(
        intermediate8,
        trace_1_column_19_offset_0,
        trace_1_column_29_offset_0,
        trace_1_column_39_offset_0,
        trace_1_column_49_offset_0,
        trace_1_column_79_offset_0,
        trace_1_column_89_offset_0,
        trace_1_column_92_offset_0,
        trace_1_column_9_offset_0,
    );

    let intermediate10 = intermediate10(
        intermediate9,
        trace_1_column_10_offset_0,
        trace_1_column_20_offset_0,
        trace_1_column_30_offset_0,
        trace_1_column_40_offset_0,
        trace_1_column_50_offset_0,
        trace_1_column_80_offset_0,
        trace_1_column_90_offset_0,
    );

    let intermediate15 = intermediate15(
        trace_1_column_103_offset_0, trace_1_column_82_offset_0, trace_1_column_93_offset_0,
    );

    let intermediate16 = intermediate16(
        intermediate15, trace_1_column_83_offset_0, trace_1_column_94_offset_0,
    );

    let intermediate17 = intermediate17(
        intermediate16, trace_1_column_84_offset_0, trace_1_column_95_offset_0,
    );

    let intermediate18 = intermediate18(
        intermediate17, trace_1_column_85_offset_0, trace_1_column_96_offset_0,
    );

    let intermediate19 = intermediate19(
        intermediate18, trace_1_column_86_offset_0, trace_1_column_97_offset_0,
    );

    let intermediate20 = intermediate20(
        intermediate19, trace_1_column_87_offset_0, trace_1_column_98_offset_0,
    );

    let intermediate21 = intermediate21(
        intermediate20, trace_1_column_88_offset_0, trace_1_column_99_offset_0,
    );

    let intermediate22 = intermediate22(
        intermediate21,
        trace_1_column_100_offset_0,
        trace_1_column_103_offset_0,
        trace_1_column_89_offset_0,
    );

    let intermediate23 = intermediate23(
        intermediate22, trace_1_column_101_offset_0, trace_1_column_90_offset_0,
    );

    let intermediate24 = intermediate24(trace_1_column_103_offset_0);

    let intermediate25 = intermediate25(intermediate15);

    let intermediate26 = intermediate26(intermediate16);

    let intermediate27 = intermediate27(intermediate17);

    let intermediate28 = intermediate28(intermediate18);

    let intermediate29 = intermediate29(intermediate19);

    let intermediate30 = intermediate30(intermediate20);

    let intermediate31 = intermediate31(intermediate21);

    let intermediate32 = intermediate32(intermediate22);

    let intermediate33 = intermediate33(intermediate23);

    let intermediate35 = intermediate35(
        trace_1_column_104_offset_0,
        trace_1_column_114_offset_0,
        trace_1_column_124_offset_0,
        trace_1_column_22_offset_0,
        trace_1_column_32_offset_0,
        trace_1_column_52_offset_0,
        trace_1_column_72_offset_0,
        trace_1_column_93_offset_0,
    );

    let intermediate36 = intermediate36(
        intermediate35,
        trace_1_column_105_offset_0,
        trace_1_column_115_offset_0,
        trace_1_column_23_offset_0,
        trace_1_column_33_offset_0,
        trace_1_column_53_offset_0,
        trace_1_column_73_offset_0,
        trace_1_column_94_offset_0,
    );

    let intermediate37 = intermediate37(
        intermediate36,
        trace_1_column_106_offset_0,
        trace_1_column_116_offset_0,
        trace_1_column_24_offset_0,
        trace_1_column_34_offset_0,
        trace_1_column_54_offset_0,
        trace_1_column_74_offset_0,
        trace_1_column_95_offset_0,
    );

    let intermediate38 = intermediate38(
        intermediate37,
        trace_1_column_107_offset_0,
        trace_1_column_117_offset_0,
        trace_1_column_25_offset_0,
        trace_1_column_35_offset_0,
        trace_1_column_55_offset_0,
        trace_1_column_75_offset_0,
        trace_1_column_96_offset_0,
    );

    let intermediate39 = intermediate39(
        intermediate38,
        trace_1_column_108_offset_0,
        trace_1_column_118_offset_0,
        trace_1_column_26_offset_0,
        trace_1_column_36_offset_0,
        trace_1_column_56_offset_0,
        trace_1_column_76_offset_0,
        trace_1_column_97_offset_0,
    );

    let intermediate40 = intermediate40(
        intermediate39,
        trace_1_column_109_offset_0,
        trace_1_column_119_offset_0,
        trace_1_column_27_offset_0,
        trace_1_column_37_offset_0,
        trace_1_column_57_offset_0,
        trace_1_column_77_offset_0,
        trace_1_column_98_offset_0,
    );

    let intermediate41 = intermediate41(
        intermediate40,
        trace_1_column_110_offset_0,
        trace_1_column_120_offset_0,
        trace_1_column_28_offset_0,
        trace_1_column_38_offset_0,
        trace_1_column_58_offset_0,
        trace_1_column_78_offset_0,
        trace_1_column_99_offset_0,
    );

    let intermediate42 = intermediate42(
        intermediate41,
        trace_1_column_100_offset_0,
        trace_1_column_111_offset_0,
        trace_1_column_121_offset_0,
        trace_1_column_124_offset_0,
        trace_1_column_29_offset_0,
        trace_1_column_39_offset_0,
        trace_1_column_59_offset_0,
        trace_1_column_79_offset_0,
    );

    let intermediate43 = intermediate43(
        intermediate42,
        trace_1_column_101_offset_0,
        trace_1_column_112_offset_0,
        trace_1_column_122_offset_0,
        trace_1_column_30_offset_0,
        trace_1_column_40_offset_0,
        trace_1_column_60_offset_0,
        trace_1_column_80_offset_0,
    );

    core::internal::revoke_ap_tracking();

    let intermediate48 = intermediate48(
        trace_1_column_114_offset_0, trace_1_column_125_offset_0, trace_1_column_135_offset_0,
    );

    let intermediate49 = intermediate49(
        intermediate48, trace_1_column_115_offset_0, trace_1_column_126_offset_0,
    );

    let intermediate50 = intermediate50(
        intermediate49, trace_1_column_116_offset_0, trace_1_column_127_offset_0,
    );

    let intermediate51 = intermediate51(
        intermediate50, trace_1_column_117_offset_0, trace_1_column_128_offset_0,
    );

    let intermediate52 = intermediate52(
        intermediate51, trace_1_column_118_offset_0, trace_1_column_129_offset_0,
    );

    let intermediate53 = intermediate53(
        intermediate52, trace_1_column_119_offset_0, trace_1_column_130_offset_0,
    );

    let intermediate54 = intermediate54(
        intermediate53, trace_1_column_120_offset_0, trace_1_column_131_offset_0,
    );

    let intermediate55 = intermediate55(
        intermediate54,
        trace_1_column_121_offset_0,
        trace_1_column_132_offset_0,
        trace_1_column_135_offset_0,
    );

    let intermediate56 = intermediate56(
        intermediate55, trace_1_column_122_offset_0, trace_1_column_133_offset_0,
    );

    let intermediate57 = intermediate57(trace_1_column_135_offset_0);

    let intermediate58 = intermediate58(intermediate48);

    let intermediate59 = intermediate59(intermediate49);

    let intermediate60 = intermediate60(intermediate50);

    let intermediate61 = intermediate61(intermediate51);

    let intermediate62 = intermediate62(intermediate52);

    let intermediate63 = intermediate63(intermediate53);

    let intermediate64 = intermediate64(intermediate54);

    let intermediate65 = intermediate65(intermediate55);

    let intermediate66 = intermediate66(intermediate56);

    let intermediate68 = intermediate68(
        trace_1_column_104_offset_0,
        trace_1_column_125_offset_0,
        trace_1_column_136_offset_0,
        trace_1_column_146_offset_0,
        trace_1_column_156_offset_0,
        trace_1_column_62_offset_0,
        trace_1_column_72_offset_0,
        trace_1_column_93_offset_0,
    );

    let intermediate69 = intermediate69(
        intermediate68,
        trace_1_column_105_offset_0,
        trace_1_column_126_offset_0,
        trace_1_column_137_offset_0,
        trace_1_column_147_offset_0,
        trace_1_column_63_offset_0,
        trace_1_column_73_offset_0,
        trace_1_column_94_offset_0,
    );

    let intermediate70 = intermediate70(
        intermediate69,
        trace_1_column_106_offset_0,
        trace_1_column_127_offset_0,
        trace_1_column_138_offset_0,
        trace_1_column_148_offset_0,
        trace_1_column_64_offset_0,
        trace_1_column_74_offset_0,
        trace_1_column_95_offset_0,
    );

    let intermediate71 = intermediate71(
        intermediate70,
        trace_1_column_107_offset_0,
        trace_1_column_128_offset_0,
        trace_1_column_139_offset_0,
        trace_1_column_149_offset_0,
        trace_1_column_65_offset_0,
        trace_1_column_75_offset_0,
        trace_1_column_96_offset_0,
    );

    let intermediate72 = intermediate72(
        intermediate71,
        trace_1_column_108_offset_0,
        trace_1_column_129_offset_0,
        trace_1_column_140_offset_0,
        trace_1_column_150_offset_0,
        trace_1_column_66_offset_0,
        trace_1_column_76_offset_0,
        trace_1_column_97_offset_0,
    );

    let intermediate73 = intermediate73(
        intermediate72,
        trace_1_column_109_offset_0,
        trace_1_column_130_offset_0,
        trace_1_column_141_offset_0,
        trace_1_column_151_offset_0,
        trace_1_column_67_offset_0,
        trace_1_column_77_offset_0,
        trace_1_column_98_offset_0,
    );

    let intermediate74 = intermediate74(
        intermediate73,
        trace_1_column_110_offset_0,
        trace_1_column_131_offset_0,
        trace_1_column_142_offset_0,
        trace_1_column_152_offset_0,
        trace_1_column_68_offset_0,
        trace_1_column_78_offset_0,
        trace_1_column_99_offset_0,
    );

    let intermediate75 = intermediate75(
        intermediate74,
        trace_1_column_100_offset_0,
        trace_1_column_111_offset_0,
        trace_1_column_132_offset_0,
        trace_1_column_143_offset_0,
        trace_1_column_153_offset_0,
        trace_1_column_156_offset_0,
        trace_1_column_69_offset_0,
        trace_1_column_79_offset_0,
    );

    let intermediate76 = intermediate76(
        intermediate75,
        trace_1_column_101_offset_0,
        trace_1_column_112_offset_0,
        trace_1_column_133_offset_0,
        trace_1_column_144_offset_0,
        trace_1_column_154_offset_0,
        trace_1_column_70_offset_0,
        trace_1_column_80_offset_0,
    );

    let intermediate81 = intermediate81(
        trace_1_column_146_offset_0, trace_1_column_157_offset_0, trace_1_column_167_offset_0,
    );

    let intermediate82 = intermediate82(
        intermediate81, trace_1_column_147_offset_0, trace_1_column_158_offset_0,
    );

    let intermediate83 = intermediate83(
        intermediate82, trace_1_column_148_offset_0, trace_1_column_159_offset_0,
    );

    let intermediate84 = intermediate84(
        intermediate83, trace_1_column_149_offset_0, trace_1_column_160_offset_0,
    );

    let intermediate85 = intermediate85(
        intermediate84, trace_1_column_150_offset_0, trace_1_column_161_offset_0,
    );

    let intermediate86 = intermediate86(
        intermediate85, trace_1_column_151_offset_0, trace_1_column_162_offset_0,
    );

    let intermediate87 = intermediate87(
        intermediate86, trace_1_column_152_offset_0, trace_1_column_163_offset_0,
    );

    let intermediate88 = intermediate88(
        intermediate87,
        trace_1_column_153_offset_0,
        trace_1_column_164_offset_0,
        trace_1_column_167_offset_0,
    );

    let intermediate89 = intermediate89(
        intermediate88, trace_1_column_154_offset_0, trace_1_column_165_offset_0,
    );

    let intermediate90 = intermediate90(trace_1_column_167_offset_0);

    let intermediate91 = intermediate91(intermediate81);

    let intermediate92 = intermediate92(intermediate82);

    let intermediate93 = intermediate93(intermediate83);

    let intermediate94 = intermediate94(intermediate84);

    let intermediate95 = intermediate95(intermediate85);

    let intermediate96 = intermediate96(intermediate86);

    let intermediate97 = intermediate97(intermediate87);

    let intermediate98 = intermediate98(intermediate88);

    let intermediate99 = intermediate99(intermediate89);
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
        trace_1_column_32_offset_0,
        trace_1_column_33_offset_0,
        trace_1_column_34_offset_0,
        trace_1_column_35_offset_0,
        trace_1_column_36_offset_0,
        trace_1_column_37_offset_0,
        trace_1_column_38_offset_0,
        trace_1_column_39_offset_0,
        trace_1_column_40_offset_0,
        trace_1_column_41_offset_0,
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
    );

    core::internal::revoke_ap_tracking();

    let intermediate0 = intermediate0(
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
        trace_1_column_68_offset_0,
        trace_1_column_69_offset_0,
        trace_1_column_70_offset_0,
        trace_1_column_71_offset_0,
    );

    let intermediate14 = intermediate14(
        RangeCheckFelt252Width27_alpha0,
        RangeCheckFelt252Width27_alpha1,
        RangeCheckFelt252Width27_alpha2,
        RangeCheckFelt252Width27_alpha3,
        RangeCheckFelt252Width27_alpha4,
        RangeCheckFelt252Width27_alpha5,
        RangeCheckFelt252Width27_alpha6,
        RangeCheckFelt252Width27_alpha7,
        RangeCheckFelt252Width27_alpha8,
        RangeCheckFelt252Width27_alpha9,
        RangeCheckFelt252Width27_z,
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

    let intermediate44 = intermediate44(
        RangeCheck_4_4_4_4_alpha0,
        RangeCheck_4_4_4_4_alpha1,
        RangeCheck_4_4_4_4_alpha2,
        RangeCheck_4_4_4_4_alpha3,
        RangeCheck_4_4_4_4_z,
        intermediate35,
        intermediate36,
        intermediate37,
        trace_1_column_124_offset_0,
    );

    let intermediate45 = intermediate45(
        RangeCheck_4_4_4_4_alpha0,
        RangeCheck_4_4_4_4_alpha1,
        RangeCheck_4_4_4_4_alpha2,
        RangeCheck_4_4_4_4_alpha3,
        RangeCheck_4_4_4_4_z,
        intermediate38,
        intermediate39,
        intermediate40,
        intermediate41,
    );

    let intermediate80 = intermediate80(
        RangeCheckFelt252Width27_alpha0,
        RangeCheckFelt252Width27_alpha1,
        RangeCheckFelt252Width27_alpha2,
        RangeCheckFelt252Width27_alpha3,
        RangeCheckFelt252Width27_alpha4,
        RangeCheckFelt252Width27_alpha5,
        RangeCheckFelt252Width27_alpha6,
        RangeCheckFelt252Width27_alpha7,
        RangeCheckFelt252Width27_alpha8,
        RangeCheckFelt252Width27_alpha9,
        RangeCheckFelt252Width27_z,
        trace_1_column_146_offset_0,
        trace_1_column_147_offset_0,
        trace_1_column_148_offset_0,
        trace_1_column_149_offset_0,
        trace_1_column_150_offset_0,
        trace_1_column_151_offset_0,
        trace_1_column_152_offset_0,
        trace_1_column_153_offset_0,
        trace_1_column_154_offset_0,
        trace_1_column_155_offset_0,
    );

    let intermediate101 = intermediate101(
        Poseidon3PartialRoundsChain_alpha0,
        Poseidon3PartialRoundsChain_alpha1,
        Poseidon3PartialRoundsChain_alpha10,
        Poseidon3PartialRoundsChain_alpha11,
        Poseidon3PartialRoundsChain_alpha12,
        Poseidon3PartialRoundsChain_alpha13,
        Poseidon3PartialRoundsChain_alpha14,
        Poseidon3PartialRoundsChain_alpha15,
        Poseidon3PartialRoundsChain_alpha16,
        Poseidon3PartialRoundsChain_alpha17,
        Poseidon3PartialRoundsChain_alpha18,
        Poseidon3PartialRoundsChain_alpha19,
        Poseidon3PartialRoundsChain_alpha2,
        Poseidon3PartialRoundsChain_alpha20,
        Poseidon3PartialRoundsChain_alpha21,
        Poseidon3PartialRoundsChain_alpha22,
        Poseidon3PartialRoundsChain_alpha23,
        Poseidon3PartialRoundsChain_alpha24,
        Poseidon3PartialRoundsChain_alpha25,
        Poseidon3PartialRoundsChain_alpha26,
        Poseidon3PartialRoundsChain_alpha27,
        Poseidon3PartialRoundsChain_alpha28,
        Poseidon3PartialRoundsChain_alpha29,
        Poseidon3PartialRoundsChain_alpha3,
        Poseidon3PartialRoundsChain_alpha30,
        Poseidon3PartialRoundsChain_alpha31,
        Poseidon3PartialRoundsChain_alpha32,
        Poseidon3PartialRoundsChain_alpha33,
        Poseidon3PartialRoundsChain_alpha34,
        Poseidon3PartialRoundsChain_alpha35,
        Poseidon3PartialRoundsChain_alpha36,
        Poseidon3PartialRoundsChain_alpha37,
        Poseidon3PartialRoundsChain_alpha38,
        Poseidon3PartialRoundsChain_alpha39,
        Poseidon3PartialRoundsChain_alpha4,
        Poseidon3PartialRoundsChain_alpha40,
        Poseidon3PartialRoundsChain_alpha41,
        Poseidon3PartialRoundsChain_alpha5,
        Poseidon3PartialRoundsChain_alpha6,
        Poseidon3PartialRoundsChain_alpha7,
        Poseidon3PartialRoundsChain_alpha8,
        Poseidon3PartialRoundsChain_alpha9,
        Poseidon3PartialRoundsChain_z,
        trace_1_column_0_offset_0,
        trace_1_column_104_offset_0,
        trace_1_column_105_offset_0,
        trace_1_column_106_offset_0,
        trace_1_column_107_offset_0,
        trace_1_column_108_offset_0,
        trace_1_column_109_offset_0,
        trace_1_column_110_offset_0,
        trace_1_column_111_offset_0,
        trace_1_column_112_offset_0,
        trace_1_column_113_offset_0,
        trace_1_column_125_offset_0,
        trace_1_column_126_offset_0,
        trace_1_column_127_offset_0,
        trace_1_column_128_offset_0,
        trace_1_column_129_offset_0,
        trace_1_column_130_offset_0,
        trace_1_column_131_offset_0,
        trace_1_column_132_offset_0,
        trace_1_column_133_offset_0,
        trace_1_column_134_offset_0,
        trace_1_column_136_offset_0,
        trace_1_column_137_offset_0,
        trace_1_column_138_offset_0,
        trace_1_column_139_offset_0,
        trace_1_column_140_offset_0,
        trace_1_column_141_offset_0,
        trace_1_column_142_offset_0,
        trace_1_column_143_offset_0,
        trace_1_column_144_offset_0,
        trace_1_column_145_offset_0,
        trace_1_column_157_offset_0,
        trace_1_column_158_offset_0,
        trace_1_column_159_offset_0,
        trace_1_column_160_offset_0,
        trace_1_column_161_offset_0,
        trace_1_column_162_offset_0,
        trace_1_column_163_offset_0,
        trace_1_column_164_offset_0,
        trace_1_column_165_offset_0,
        trace_1_column_166_offset_0,
        trace_1_column_1_offset_0,
    );

    let intermediate77 = intermediate77(
        RangeCheck_4_4_4_4_alpha0,
        RangeCheck_4_4_4_4_alpha1,
        RangeCheck_4_4_4_4_alpha2,
        RangeCheck_4_4_4_4_alpha3,
        RangeCheck_4_4_4_4_z,
        intermediate68,
        intermediate69,
        intermediate70,
        trace_1_column_156_offset_0,
    );

    let intermediate13 = intermediate13(
        RangeCheck_4_4_alpha0,
        RangeCheck_4_4_alpha1,
        RangeCheck_4_4_z,
        intermediate10,
        intermediate9,
    );

    core::internal::revoke_ap_tracking();

    let intermediate67 = intermediate67(
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
        trace_1_column_125_offset_0,
        trace_1_column_126_offset_0,
        trace_1_column_127_offset_0,
        trace_1_column_128_offset_0,
        trace_1_column_129_offset_0,
        trace_1_column_130_offset_0,
        trace_1_column_131_offset_0,
        trace_1_column_132_offset_0,
        trace_1_column_133_offset_0,
        trace_1_column_134_offset_0,
        trace_1_column_136_offset_0,
        trace_1_column_137_offset_0,
        trace_1_column_138_offset_0,
        trace_1_column_139_offset_0,
        trace_1_column_140_offset_0,
        trace_1_column_141_offset_0,
        trace_1_column_142_offset_0,
        trace_1_column_143_offset_0,
        trace_1_column_144_offset_0,
        trace_1_column_145_offset_0,
    );

    let intermediate12 = intermediate12(
        RangeCheck_4_4_4_4_alpha0,
        RangeCheck_4_4_4_4_alpha1,
        RangeCheck_4_4_4_4_alpha2,
        RangeCheck_4_4_4_4_alpha3,
        RangeCheck_4_4_4_4_z,
        intermediate5,
        intermediate6,
        intermediate7,
        intermediate8,
    );

    let intermediate11 = intermediate11(
        RangeCheck_4_4_4_4_alpha0,
        RangeCheck_4_4_4_4_alpha1,
        RangeCheck_4_4_4_4_alpha2,
        RangeCheck_4_4_4_4_alpha3,
        RangeCheck_4_4_4_4_z,
        intermediate2,
        intermediate3,
        intermediate4,
        trace_1_column_92_offset_0,
    );

    let intermediate46 = intermediate46(
        RangeCheck_4_4_alpha0,
        RangeCheck_4_4_alpha1,
        RangeCheck_4_4_z,
        intermediate42,
        intermediate43,
    );

    let intermediate34 = intermediate34(
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
        trace_1_column_100_offset_0,
        trace_1_column_101_offset_0,
        trace_1_column_102_offset_0,
        trace_1_column_104_offset_0,
        trace_1_column_105_offset_0,
        trace_1_column_106_offset_0,
        trace_1_column_107_offset_0,
        trace_1_column_108_offset_0,
        trace_1_column_109_offset_0,
        trace_1_column_110_offset_0,
        trace_1_column_111_offset_0,
        trace_1_column_112_offset_0,
        trace_1_column_113_offset_0,
        trace_1_column_93_offset_0,
        trace_1_column_94_offset_0,
        trace_1_column_95_offset_0,
        trace_1_column_96_offset_0,
        trace_1_column_97_offset_0,
        trace_1_column_98_offset_0,
        trace_1_column_99_offset_0,
    );

    let intermediate47 = intermediate47(
        RangeCheckFelt252Width27_alpha0,
        RangeCheckFelt252Width27_alpha1,
        RangeCheckFelt252Width27_alpha2,
        RangeCheckFelt252Width27_alpha3,
        RangeCheckFelt252Width27_alpha4,
        RangeCheckFelt252Width27_alpha5,
        RangeCheckFelt252Width27_alpha6,
        RangeCheckFelt252Width27_alpha7,
        RangeCheckFelt252Width27_alpha8,
        RangeCheckFelt252Width27_alpha9,
        RangeCheckFelt252Width27_z,
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
    );

    let intermediate79 = intermediate79(
        RangeCheck_4_4_alpha0,
        RangeCheck_4_4_alpha1,
        RangeCheck_4_4_z,
        intermediate75,
        intermediate76,
    );

    let intermediate100 = intermediate100(
        Poseidon3PartialRoundsChain_alpha0,
        Poseidon3PartialRoundsChain_alpha1,
        Poseidon3PartialRoundsChain_alpha10,
        Poseidon3PartialRoundsChain_alpha11,
        Poseidon3PartialRoundsChain_alpha12,
        Poseidon3PartialRoundsChain_alpha13,
        Poseidon3PartialRoundsChain_alpha14,
        Poseidon3PartialRoundsChain_alpha15,
        Poseidon3PartialRoundsChain_alpha16,
        Poseidon3PartialRoundsChain_alpha17,
        Poseidon3PartialRoundsChain_alpha18,
        Poseidon3PartialRoundsChain_alpha19,
        Poseidon3PartialRoundsChain_alpha2,
        Poseidon3PartialRoundsChain_alpha20,
        Poseidon3PartialRoundsChain_alpha21,
        Poseidon3PartialRoundsChain_alpha22,
        Poseidon3PartialRoundsChain_alpha23,
        Poseidon3PartialRoundsChain_alpha24,
        Poseidon3PartialRoundsChain_alpha25,
        Poseidon3PartialRoundsChain_alpha26,
        Poseidon3PartialRoundsChain_alpha27,
        Poseidon3PartialRoundsChain_alpha28,
        Poseidon3PartialRoundsChain_alpha29,
        Poseidon3PartialRoundsChain_alpha3,
        Poseidon3PartialRoundsChain_alpha30,
        Poseidon3PartialRoundsChain_alpha31,
        Poseidon3PartialRoundsChain_alpha32,
        Poseidon3PartialRoundsChain_alpha33,
        Poseidon3PartialRoundsChain_alpha34,
        Poseidon3PartialRoundsChain_alpha35,
        Poseidon3PartialRoundsChain_alpha36,
        Poseidon3PartialRoundsChain_alpha37,
        Poseidon3PartialRoundsChain_alpha38,
        Poseidon3PartialRoundsChain_alpha39,
        Poseidon3PartialRoundsChain_alpha4,
        Poseidon3PartialRoundsChain_alpha40,
        Poseidon3PartialRoundsChain_alpha41,
        Poseidon3PartialRoundsChain_alpha5,
        Poseidon3PartialRoundsChain_alpha6,
        Poseidon3PartialRoundsChain_alpha7,
        Poseidon3PartialRoundsChain_alpha8,
        Poseidon3PartialRoundsChain_alpha9,
        Poseidon3PartialRoundsChain_z,
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
        trace_1_column_4_offset_0,
        trace_1_column_5_offset_0,
        trace_1_column_6_offset_0,
        trace_1_column_7_offset_0,
        trace_1_column_8_offset_0,
        trace_1_column_9_offset_0,
    );

    let intermediate78 = intermediate78(
        RangeCheck_4_4_4_4_alpha0,
        RangeCheck_4_4_4_4_alpha1,
        RangeCheck_4_4_4_4_alpha2,
        RangeCheck_4_4_4_4_alpha3,
        RangeCheck_4_4_4_4_z,
        intermediate71,
        intermediate72,
        intermediate73,
        intermediate74,
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
        intermediate39,
        intermediate40,
        intermediate41,
        intermediate42,
        intermediate43,
        intermediate44,
        intermediate45,
        intermediate46,
        intermediate47,
        intermediate48,
        intermediate49,
        intermediate50,
        intermediate51,
        intermediate52,
        intermediate53,
        intermediate54,
        intermediate55,
        intermediate56,
        intermediate57,
        intermediate58,
        intermediate59,
        intermediate60,
        intermediate61,
        intermediate62,
        intermediate63,
        intermediate64,
        intermediate65,
        intermediate66,
        intermediate67,
        intermediate68,
        intermediate69,
        intermediate70,
        intermediate71,
        intermediate72,
        intermediate73,
        intermediate74,
        intermediate75,
        intermediate76,
        intermediate77,
        intermediate78,
        intermediate79,
        intermediate80,
        intermediate81,
        intermediate82,
        intermediate83,
        intermediate84,
        intermediate85,
        intermediate86,
        intermediate87,
        intermediate88,
        intermediate89,
        intermediate90,
        intermediate91,
        intermediate92,
        intermediate93,
        intermediate94,
        intermediate95,
        intermediate96,
        intermediate97,
        intermediate98,
        intermediate99,
        intermediate100,
        intermediate101,
    ]
}

pub fn intermediate39(
    intermediate38: QM31,
    trace_1_column_108_offset_0: QM31,
    trace_1_column_118_offset_0: QM31,
    trace_1_column_26_offset_0: QM31,
    trace_1_column_36_offset_0: QM31,
    trace_1_column_56_offset_0: QM31,
    trace_1_column_76_offset_0: QM31,
    trace_1_column_97_offset_0: QM31,
) -> QM31 {
    (intermediate38
        + (m31(4).into()) * (trace_1_column_26_offset_0)
        + (m31(2).into()) * (trace_1_column_36_offset_0)
        + (m31(3).into()) * (trace_1_column_76_offset_0)
        + trace_1_column_97_offset_0
        - (trace_1_column_108_offset_0)
        + trace_1_column_56_offset_0
        - (trace_1_column_118_offset_0))
        * (m31(16).into())
}

pub fn intermediate3(
    intermediate2: QM31,
    trace_1_column_13_offset_0: QM31,
    trace_1_column_23_offset_0: QM31,
    trace_1_column_33_offset_0: QM31,
    trace_1_column_3_offset_0: QM31,
    trace_1_column_43_offset_0: QM31,
    trace_1_column_73_offset_0: QM31,
    trace_1_column_83_offset_0: QM31,
) -> QM31 {
    (intermediate2
        + (m31(4).into()) * (trace_1_column_3_offset_0)
        + (m31(2).into()) * (trace_1_column_13_offset_0)
        + (m31(3).into()) * (trace_1_column_23_offset_0)
        + trace_1_column_33_offset_0
        - (trace_1_column_73_offset_0)
        + trace_1_column_43_offset_0
        - (trace_1_column_83_offset_0))
        * (m31(16).into())
}

pub fn intermediate89(
    intermediate88: QM31, trace_1_column_154_offset_0: QM31, trace_1_column_165_offset_0: QM31,
) -> QM31 {
    (intermediate88
        + (m31(2).into()) * (trace_1_column_154_offset_0)
        - (trace_1_column_165_offset_0))
        * (m31(16).into())
}

pub fn intermediate93(intermediate83: QM31) -> QM31 {
    intermediate83 + m31(1).into() - (m31(1).into())
}

pub fn intermediate36(
    intermediate35: QM31,
    trace_1_column_105_offset_0: QM31,
    trace_1_column_115_offset_0: QM31,
    trace_1_column_23_offset_0: QM31,
    trace_1_column_33_offset_0: QM31,
    trace_1_column_53_offset_0: QM31,
    trace_1_column_73_offset_0: QM31,
    trace_1_column_94_offset_0: QM31,
) -> QM31 {
    (intermediate35
        + (m31(4).into()) * (trace_1_column_23_offset_0)
        + (m31(2).into()) * (trace_1_column_33_offset_0)
        + (m31(3).into()) * (trace_1_column_73_offset_0)
        + trace_1_column_94_offset_0
        - (trace_1_column_105_offset_0)
        + trace_1_column_53_offset_0
        - (trace_1_column_115_offset_0))
        * (m31(16).into())
}

pub fn intermediate50(
    intermediate49: QM31, trace_1_column_116_offset_0: QM31, trace_1_column_127_offset_0: QM31,
) -> QM31 {
    (intermediate49
        + (m31(2).into()) * (trace_1_column_116_offset_0)
        - (trace_1_column_127_offset_0))
        * (m31(16).into())
}

pub fn intermediate58(intermediate48: QM31) -> QM31 {
    intermediate48 + m31(1).into() - (m31(1).into())
}

pub fn intermediate72(
    intermediate71: QM31,
    trace_1_column_108_offset_0: QM31,
    trace_1_column_129_offset_0: QM31,
    trace_1_column_140_offset_0: QM31,
    trace_1_column_150_offset_0: QM31,
    trace_1_column_66_offset_0: QM31,
    trace_1_column_76_offset_0: QM31,
    trace_1_column_97_offset_0: QM31,
) -> QM31 {
    (intermediate71
        + (m31(4).into()) * (trace_1_column_76_offset_0)
        + (m31(2).into()) * (trace_1_column_97_offset_0)
        + (m31(3).into()) * (trace_1_column_108_offset_0)
        + trace_1_column_129_offset_0
        - (trace_1_column_140_offset_0)
        + trace_1_column_66_offset_0
        - (trace_1_column_150_offset_0))
        * (m31(16).into())
}

pub fn intermediate49(
    intermediate48: QM31, trace_1_column_115_offset_0: QM31, trace_1_column_126_offset_0: QM31,
) -> QM31 {
    (intermediate48
        + (m31(2).into()) * (trace_1_column_115_offset_0)
        - (trace_1_column_126_offset_0))
        * (m31(16).into())
}

pub fn intermediate48(
    trace_1_column_114_offset_0: QM31,
    trace_1_column_125_offset_0: QM31,
    trace_1_column_135_offset_0: QM31,
) -> QM31 {
    ((m31(2).into()) * (trace_1_column_114_offset_0)
        - (trace_1_column_125_offset_0)
        - (trace_1_column_135_offset_0))
        * (m31(16).into())
}

pub fn intermediate42(
    intermediate41: QM31,
    trace_1_column_100_offset_0: QM31,
    trace_1_column_111_offset_0: QM31,
    trace_1_column_121_offset_0: QM31,
    trace_1_column_124_offset_0: QM31,
    trace_1_column_29_offset_0: QM31,
    trace_1_column_39_offset_0: QM31,
    trace_1_column_59_offset_0: QM31,
    trace_1_column_79_offset_0: QM31,
) -> QM31 {
    (intermediate41
        + (m31(4).into()) * (trace_1_column_29_offset_0)
        + (m31(2).into()) * (trace_1_column_39_offset_0)
        + (m31(3).into()) * (trace_1_column_79_offset_0)
        + trace_1_column_100_offset_0
        - (trace_1_column_111_offset_0)
        + trace_1_column_59_offset_0
        - (trace_1_column_121_offset_0)
        - ((trace_1_column_124_offset_0) * (m31(136).into())))
        * (m31(16).into())
}

pub fn intermediate33(intermediate23: QM31) -> QM31 {
    intermediate23 + m31(1).into() - (m31(1).into())
}

pub fn intermediate82(
    intermediate81: QM31, trace_1_column_147_offset_0: QM31, trace_1_column_158_offset_0: QM31,
) -> QM31 {
    (intermediate81
        + (m31(2).into()) * (trace_1_column_147_offset_0)
        - (trace_1_column_158_offset_0))
        * (m31(16).into())
}

pub fn intermediate85(
    intermediate84: QM31, trace_1_column_150_offset_0: QM31, trace_1_column_161_offset_0: QM31,
) -> QM31 {
    (intermediate84
        + (m31(2).into()) * (trace_1_column_150_offset_0)
        - (trace_1_column_161_offset_0))
        * (m31(16).into())
}

pub fn intermediate87(
    intermediate86: QM31, trace_1_column_152_offset_0: QM31, trace_1_column_163_offset_0: QM31,
) -> QM31 {
    (intermediate86
        + (m31(2).into()) * (trace_1_column_152_offset_0)
        - (trace_1_column_163_offset_0))
        * (m31(16).into())
}

pub fn intermediate19(
    intermediate18: QM31, trace_1_column_86_offset_0: QM31, trace_1_column_97_offset_0: QM31,
) -> QM31 {
    (intermediate18 + (m31(2).into()) * (trace_1_column_86_offset_0) - (trace_1_column_97_offset_0))
        * (m31(16).into())
}

pub fn intermediate22(
    intermediate21: QM31,
    trace_1_column_100_offset_0: QM31,
    trace_1_column_103_offset_0: QM31,
    trace_1_column_89_offset_0: QM31,
) -> QM31 {
    (intermediate21
        + (m31(2).into()) * (trace_1_column_89_offset_0)
        - (trace_1_column_100_offset_0)
        - ((trace_1_column_103_offset_0) * (m31(136).into())))
        * (m31(16).into())
}

pub fn intermediate24(trace_1_column_103_offset_0: QM31) -> QM31 {
    trace_1_column_103_offset_0 + m31(1).into() - (m31(1).into())
}

pub fn intermediate96(intermediate86: QM31) -> QM31 {
    intermediate86 + m31(1).into() - (m31(1).into())
}

pub fn intermediate59(intermediate49: QM31) -> QM31 {
    intermediate49 + m31(1).into() - (m31(1).into())
}

pub fn intermediate26(intermediate16: QM31) -> QM31 {
    intermediate16 + m31(1).into() - (m31(1).into())
}

pub fn intermediate64(intermediate54: QM31) -> QM31 {
    intermediate54 + m31(1).into() - (m31(1).into())
}

pub fn intermediate97(intermediate87: QM31) -> QM31 {
    intermediate87 + m31(1).into() - (m31(1).into())
}

pub fn intermediate70(
    intermediate69: QM31,
    trace_1_column_106_offset_0: QM31,
    trace_1_column_127_offset_0: QM31,
    trace_1_column_138_offset_0: QM31,
    trace_1_column_148_offset_0: QM31,
    trace_1_column_64_offset_0: QM31,
    trace_1_column_74_offset_0: QM31,
    trace_1_column_95_offset_0: QM31,
) -> QM31 {
    (intermediate69
        + (m31(4).into()) * (trace_1_column_74_offset_0)
        + (m31(2).into()) * (trace_1_column_95_offset_0)
        + (m31(3).into()) * (trace_1_column_106_offset_0)
        + trace_1_column_127_offset_0
        - (trace_1_column_138_offset_0)
        + trace_1_column_64_offset_0
        - (trace_1_column_148_offset_0))
        * (m31(16).into())
}

pub fn intermediate8(
    intermediate7: QM31,
    trace_1_column_18_offset_0: QM31,
    trace_1_column_28_offset_0: QM31,
    trace_1_column_38_offset_0: QM31,
    trace_1_column_48_offset_0: QM31,
    trace_1_column_78_offset_0: QM31,
    trace_1_column_88_offset_0: QM31,
    trace_1_column_8_offset_0: QM31,
) -> QM31 {
    (intermediate7
        + (m31(4).into()) * (trace_1_column_8_offset_0)
        + (m31(2).into()) * (trace_1_column_18_offset_0)
        + (m31(3).into()) * (trace_1_column_28_offset_0)
        + trace_1_column_38_offset_0
        - (trace_1_column_78_offset_0)
        + trace_1_column_48_offset_0
        - (trace_1_column_88_offset_0))
        * (m31(16).into())
}

pub fn intermediate28(intermediate18: QM31) -> QM31 {
    intermediate18 + m31(1).into() - (m31(1).into())
}

pub fn intermediate61(intermediate51: QM31) -> QM31 {
    intermediate51 + m31(1).into() - (m31(1).into())
}

pub fn intermediate75(
    intermediate74: QM31,
    trace_1_column_100_offset_0: QM31,
    trace_1_column_111_offset_0: QM31,
    trace_1_column_132_offset_0: QM31,
    trace_1_column_143_offset_0: QM31,
    trace_1_column_153_offset_0: QM31,
    trace_1_column_156_offset_0: QM31,
    trace_1_column_69_offset_0: QM31,
    trace_1_column_79_offset_0: QM31,
) -> QM31 {
    (intermediate74
        + (m31(4).into()) * (trace_1_column_79_offset_0)
        + (m31(2).into()) * (trace_1_column_100_offset_0)
        + (m31(3).into()) * (trace_1_column_111_offset_0)
        + trace_1_column_132_offset_0
        - (trace_1_column_143_offset_0)
        + trace_1_column_69_offset_0
        - (trace_1_column_153_offset_0)
        - ((trace_1_column_156_offset_0) * (m31(136).into())))
        * (m31(16).into())
}

pub fn intermediate15(
    trace_1_column_103_offset_0: QM31,
    trace_1_column_82_offset_0: QM31,
    trace_1_column_93_offset_0: QM31,
) -> QM31 {
    ((m31(2).into()) * (trace_1_column_82_offset_0)
        - (trace_1_column_93_offset_0)
        - (trace_1_column_103_offset_0))
        * (m31(16).into())
}

pub fn intermediate41(
    intermediate40: QM31,
    trace_1_column_110_offset_0: QM31,
    trace_1_column_120_offset_0: QM31,
    trace_1_column_28_offset_0: QM31,
    trace_1_column_38_offset_0: QM31,
    trace_1_column_58_offset_0: QM31,
    trace_1_column_78_offset_0: QM31,
    trace_1_column_99_offset_0: QM31,
) -> QM31 {
    (intermediate40
        + (m31(4).into()) * (trace_1_column_28_offset_0)
        + (m31(2).into()) * (trace_1_column_38_offset_0)
        + (m31(3).into()) * (trace_1_column_78_offset_0)
        + trace_1_column_99_offset_0
        - (trace_1_column_110_offset_0)
        + trace_1_column_58_offset_0
        - (trace_1_column_120_offset_0))
        * (m31(16).into())
}

pub fn intermediate51(
    intermediate50: QM31, trace_1_column_117_offset_0: QM31, trace_1_column_128_offset_0: QM31,
) -> QM31 {
    (intermediate50
        + (m31(2).into()) * (trace_1_column_117_offset_0)
        - (trace_1_column_128_offset_0))
        * (m31(16).into())
}

pub fn intermediate43(
    intermediate42: QM31,
    trace_1_column_101_offset_0: QM31,
    trace_1_column_112_offset_0: QM31,
    trace_1_column_122_offset_0: QM31,
    trace_1_column_30_offset_0: QM31,
    trace_1_column_40_offset_0: QM31,
    trace_1_column_60_offset_0: QM31,
    trace_1_column_80_offset_0: QM31,
) -> QM31 {
    (intermediate42
        + (m31(4).into()) * (trace_1_column_30_offset_0)
        + (m31(2).into()) * (trace_1_column_40_offset_0)
        + (m31(3).into()) * (trace_1_column_80_offset_0)
        + trace_1_column_101_offset_0
        - (trace_1_column_112_offset_0)
        + trace_1_column_60_offset_0
        - (trace_1_column_122_offset_0))
        * (m31(16).into())
}

pub fn intermediate74(
    intermediate73: QM31,
    trace_1_column_110_offset_0: QM31,
    trace_1_column_131_offset_0: QM31,
    trace_1_column_142_offset_0: QM31,
    trace_1_column_152_offset_0: QM31,
    trace_1_column_68_offset_0: QM31,
    trace_1_column_78_offset_0: QM31,
    trace_1_column_99_offset_0: QM31,
) -> QM31 {
    (intermediate73
        + (m31(4).into()) * (trace_1_column_78_offset_0)
        + (m31(2).into()) * (trace_1_column_99_offset_0)
        + (m31(3).into()) * (trace_1_column_110_offset_0)
        + trace_1_column_131_offset_0
        - (trace_1_column_142_offset_0)
        + trace_1_column_68_offset_0
        - (trace_1_column_152_offset_0))
        * (m31(16).into())
}

pub fn intermediate91(intermediate81: QM31) -> QM31 {
    intermediate81 + m31(1).into() - (m31(1).into())
}

pub fn intermediate30(intermediate20: QM31) -> QM31 {
    intermediate20 + m31(1).into() - (m31(1).into())
}

pub fn intermediate90(trace_1_column_167_offset_0: QM31) -> QM31 {
    trace_1_column_167_offset_0 + m31(1).into() - (m31(1).into())
}

pub fn intermediate6(
    intermediate5: QM31,
    trace_1_column_16_offset_0: QM31,
    trace_1_column_26_offset_0: QM31,
    trace_1_column_36_offset_0: QM31,
    trace_1_column_46_offset_0: QM31,
    trace_1_column_6_offset_0: QM31,
    trace_1_column_76_offset_0: QM31,
    trace_1_column_86_offset_0: QM31,
) -> QM31 {
    (intermediate5
        + (m31(4).into()) * (trace_1_column_6_offset_0)
        + (m31(2).into()) * (trace_1_column_16_offset_0)
        + (m31(3).into()) * (trace_1_column_26_offset_0)
        + trace_1_column_36_offset_0
        - (trace_1_column_76_offset_0)
        + trace_1_column_46_offset_0
        - (trace_1_column_86_offset_0))
        * (m31(16).into())
}

pub fn intermediate60(intermediate50: QM31) -> QM31 {
    intermediate50 + m31(1).into() - (m31(1).into())
}

pub fn intermediate5(
    intermediate4: QM31,
    trace_1_column_15_offset_0: QM31,
    trace_1_column_25_offset_0: QM31,
    trace_1_column_35_offset_0: QM31,
    trace_1_column_45_offset_0: QM31,
    trace_1_column_5_offset_0: QM31,
    trace_1_column_75_offset_0: QM31,
    trace_1_column_85_offset_0: QM31,
) -> QM31 {
    (intermediate4
        + (m31(4).into()) * (trace_1_column_5_offset_0)
        + (m31(2).into()) * (trace_1_column_15_offset_0)
        + (m31(3).into()) * (trace_1_column_25_offset_0)
        + trace_1_column_35_offset_0
        - (trace_1_column_75_offset_0)
        + trace_1_column_45_offset_0
        - (trace_1_column_85_offset_0))
        * (m31(16).into())
}

pub fn intermediate88(
    intermediate87: QM31,
    trace_1_column_153_offset_0: QM31,
    trace_1_column_164_offset_0: QM31,
    trace_1_column_167_offset_0: QM31,
) -> QM31 {
    (intermediate87
        + (m31(2).into()) * (trace_1_column_153_offset_0)
        - (trace_1_column_164_offset_0)
        - ((trace_1_column_167_offset_0) * (m31(136).into())))
        * (m31(16).into())
}

pub fn intermediate99(intermediate89: QM31) -> QM31 {
    intermediate89 + m31(1).into() - (m31(1).into())
}

pub fn intermediate65(intermediate55: QM31) -> QM31 {
    intermediate55 + m31(1).into() - (m31(1).into())
}

pub fn intermediate27(intermediate17: QM31) -> QM31 {
    intermediate17 + m31(1).into() - (m31(1).into())
}

pub fn intermediate38(
    intermediate37: QM31,
    trace_1_column_107_offset_0: QM31,
    trace_1_column_117_offset_0: QM31,
    trace_1_column_25_offset_0: QM31,
    trace_1_column_35_offset_0: QM31,
    trace_1_column_55_offset_0: QM31,
    trace_1_column_75_offset_0: QM31,
    trace_1_column_96_offset_0: QM31,
) -> QM31 {
    (intermediate37
        + (m31(4).into()) * (trace_1_column_25_offset_0)
        + (m31(2).into()) * (trace_1_column_35_offset_0)
        + (m31(3).into()) * (trace_1_column_75_offset_0)
        + trace_1_column_96_offset_0
        - (trace_1_column_107_offset_0)
        + trace_1_column_55_offset_0
        - (trace_1_column_117_offset_0))
        * (m31(16).into())
}

pub fn intermediate10(
    intermediate9: QM31,
    trace_1_column_10_offset_0: QM31,
    trace_1_column_20_offset_0: QM31,
    trace_1_column_30_offset_0: QM31,
    trace_1_column_40_offset_0: QM31,
    trace_1_column_50_offset_0: QM31,
    trace_1_column_80_offset_0: QM31,
    trace_1_column_90_offset_0: QM31,
) -> QM31 {
    (intermediate9
        + (m31(4).into()) * (trace_1_column_10_offset_0)
        + (m31(2).into()) * (trace_1_column_20_offset_0)
        + (m31(3).into()) * (trace_1_column_30_offset_0)
        + trace_1_column_40_offset_0
        - (trace_1_column_80_offset_0)
        + trace_1_column_50_offset_0
        - (trace_1_column_90_offset_0))
        * (m31(16).into())
}

pub fn intermediate2(
    trace_1_column_12_offset_0: QM31,
    trace_1_column_22_offset_0: QM31,
    trace_1_column_2_offset_0: QM31,
    trace_1_column_32_offset_0: QM31,
    trace_1_column_42_offset_0: QM31,
    trace_1_column_72_offset_0: QM31,
    trace_1_column_82_offset_0: QM31,
    trace_1_column_92_offset_0: QM31,
) -> QM31 {
    ((m31(4).into()) * (trace_1_column_2_offset_0)
        + (m31(2).into()) * (trace_1_column_12_offset_0)
        + (m31(3).into()) * (trace_1_column_22_offset_0)
        + trace_1_column_32_offset_0
        - (trace_1_column_72_offset_0)
        + trace_1_column_42_offset_0
        - (trace_1_column_82_offset_0)
        - (trace_1_column_92_offset_0))
        * (m31(16).into())
}

pub fn intermediate37(
    intermediate36: QM31,
    trace_1_column_106_offset_0: QM31,
    trace_1_column_116_offset_0: QM31,
    trace_1_column_24_offset_0: QM31,
    trace_1_column_34_offset_0: QM31,
    trace_1_column_54_offset_0: QM31,
    trace_1_column_74_offset_0: QM31,
    trace_1_column_95_offset_0: QM31,
) -> QM31 {
    (intermediate36
        + (m31(4).into()) * (trace_1_column_24_offset_0)
        + (m31(2).into()) * (trace_1_column_34_offset_0)
        + (m31(3).into()) * (trace_1_column_74_offset_0)
        + trace_1_column_95_offset_0
        - (trace_1_column_106_offset_0)
        + trace_1_column_54_offset_0
        - (trace_1_column_116_offset_0))
        * (m31(16).into())
}

pub fn intermediate4(
    intermediate3: QM31,
    trace_1_column_14_offset_0: QM31,
    trace_1_column_24_offset_0: QM31,
    trace_1_column_34_offset_0: QM31,
    trace_1_column_44_offset_0: QM31,
    trace_1_column_4_offset_0: QM31,
    trace_1_column_74_offset_0: QM31,
    trace_1_column_84_offset_0: QM31,
) -> QM31 {
    (intermediate3
        + (m31(4).into()) * (trace_1_column_4_offset_0)
        + (m31(2).into()) * (trace_1_column_14_offset_0)
        + (m31(3).into()) * (trace_1_column_24_offset_0)
        + trace_1_column_34_offset_0
        - (trace_1_column_74_offset_0)
        + trace_1_column_44_offset_0
        - (trace_1_column_84_offset_0))
        * (m31(16).into())
}

pub fn intermediate18(
    intermediate17: QM31, trace_1_column_85_offset_0: QM31, trace_1_column_96_offset_0: QM31,
) -> QM31 {
    (intermediate17 + (m31(2).into()) * (trace_1_column_85_offset_0) - (trace_1_column_96_offset_0))
        * (m31(16).into())
}

pub fn intermediate69(
    intermediate68: QM31,
    trace_1_column_105_offset_0: QM31,
    trace_1_column_126_offset_0: QM31,
    trace_1_column_137_offset_0: QM31,
    trace_1_column_147_offset_0: QM31,
    trace_1_column_63_offset_0: QM31,
    trace_1_column_73_offset_0: QM31,
    trace_1_column_94_offset_0: QM31,
) -> QM31 {
    (intermediate68
        + (m31(4).into()) * (trace_1_column_73_offset_0)
        + (m31(2).into()) * (trace_1_column_94_offset_0)
        + (m31(3).into()) * (trace_1_column_105_offset_0)
        + trace_1_column_126_offset_0
        - (trace_1_column_137_offset_0)
        + trace_1_column_63_offset_0
        - (trace_1_column_147_offset_0))
        * (m31(16).into())
}

pub fn intermediate54(
    intermediate53: QM31, trace_1_column_120_offset_0: QM31, trace_1_column_131_offset_0: QM31,
) -> QM31 {
    (intermediate53
        + (m31(2).into()) * (trace_1_column_120_offset_0)
        - (trace_1_column_131_offset_0))
        * (m31(16).into())
}

pub fn intermediate73(
    intermediate72: QM31,
    trace_1_column_109_offset_0: QM31,
    trace_1_column_130_offset_0: QM31,
    trace_1_column_141_offset_0: QM31,
    trace_1_column_151_offset_0: QM31,
    trace_1_column_67_offset_0: QM31,
    trace_1_column_77_offset_0: QM31,
    trace_1_column_98_offset_0: QM31,
) -> QM31 {
    (intermediate72
        + (m31(4).into()) * (trace_1_column_77_offset_0)
        + (m31(2).into()) * (trace_1_column_98_offset_0)
        + (m31(3).into()) * (trace_1_column_109_offset_0)
        + trace_1_column_130_offset_0
        - (trace_1_column_141_offset_0)
        + trace_1_column_67_offset_0
        - (trace_1_column_151_offset_0))
        * (m31(16).into())
}

pub fn intermediate86(
    intermediate85: QM31, trace_1_column_151_offset_0: QM31, trace_1_column_162_offset_0: QM31,
) -> QM31 {
    (intermediate85
        + (m31(2).into()) * (trace_1_column_151_offset_0)
        - (trace_1_column_162_offset_0))
        * (m31(16).into())
}

pub fn intermediate32(intermediate22: QM31) -> QM31 {
    intermediate22 + m31(1).into() - (m31(1).into())
}

pub fn intermediate84(
    intermediate83: QM31, trace_1_column_149_offset_0: QM31, trace_1_column_160_offset_0: QM31,
) -> QM31 {
    (intermediate83
        + (m31(2).into()) * (trace_1_column_149_offset_0)
        - (trace_1_column_160_offset_0))
        * (m31(16).into())
}

pub fn intermediate68(
    trace_1_column_104_offset_0: QM31,
    trace_1_column_125_offset_0: QM31,
    trace_1_column_136_offset_0: QM31,
    trace_1_column_146_offset_0: QM31,
    trace_1_column_156_offset_0: QM31,
    trace_1_column_62_offset_0: QM31,
    trace_1_column_72_offset_0: QM31,
    trace_1_column_93_offset_0: QM31,
) -> QM31 {
    ((m31(4).into()) * (trace_1_column_72_offset_0)
        + (m31(2).into()) * (trace_1_column_93_offset_0)
        + (m31(3).into()) * (trace_1_column_104_offset_0)
        + trace_1_column_125_offset_0
        - (trace_1_column_136_offset_0)
        + trace_1_column_62_offset_0
        - (trace_1_column_146_offset_0)
        - (trace_1_column_156_offset_0))
        * (m31(16).into())
}

pub fn intermediate9(
    intermediate8: QM31,
    trace_1_column_19_offset_0: QM31,
    trace_1_column_29_offset_0: QM31,
    trace_1_column_39_offset_0: QM31,
    trace_1_column_49_offset_0: QM31,
    trace_1_column_79_offset_0: QM31,
    trace_1_column_89_offset_0: QM31,
    trace_1_column_92_offset_0: QM31,
    trace_1_column_9_offset_0: QM31,
) -> QM31 {
    (intermediate8
        + (m31(4).into()) * (trace_1_column_9_offset_0)
        + (m31(2).into()) * (trace_1_column_19_offset_0)
        + (m31(3).into()) * (trace_1_column_29_offset_0)
        + trace_1_column_39_offset_0
        - (trace_1_column_79_offset_0)
        + trace_1_column_49_offset_0
        - (trace_1_column_89_offset_0)
        - ((trace_1_column_92_offset_0) * (m31(136).into())))
        * (m31(16).into())
}

pub fn intermediate29(intermediate19: QM31) -> QM31 {
    intermediate19 + m31(1).into() - (m31(1).into())
}

pub fn intermediate71(
    intermediate70: QM31,
    trace_1_column_107_offset_0: QM31,
    trace_1_column_128_offset_0: QM31,
    trace_1_column_139_offset_0: QM31,
    trace_1_column_149_offset_0: QM31,
    trace_1_column_65_offset_0: QM31,
    trace_1_column_75_offset_0: QM31,
    trace_1_column_96_offset_0: QM31,
) -> QM31 {
    (intermediate70
        + (m31(4).into()) * (trace_1_column_75_offset_0)
        + (m31(2).into()) * (trace_1_column_96_offset_0)
        + (m31(3).into()) * (trace_1_column_107_offset_0)
        + trace_1_column_128_offset_0
        - (trace_1_column_139_offset_0)
        + trace_1_column_65_offset_0
        - (trace_1_column_149_offset_0))
        * (m31(16).into())
}

pub fn intermediate81(
    trace_1_column_146_offset_0: QM31,
    trace_1_column_157_offset_0: QM31,
    trace_1_column_167_offset_0: QM31,
) -> QM31 {
    ((m31(2).into()) * (trace_1_column_146_offset_0)
        - (trace_1_column_157_offset_0)
        - (trace_1_column_167_offset_0))
        * (m31(16).into())
}

pub fn intermediate94(intermediate84: QM31) -> QM31 {
    intermediate84 + m31(1).into() - (m31(1).into())
}

pub fn intermediate7(
    intermediate6: QM31,
    trace_1_column_17_offset_0: QM31,
    trace_1_column_27_offset_0: QM31,
    trace_1_column_37_offset_0: QM31,
    trace_1_column_47_offset_0: QM31,
    trace_1_column_77_offset_0: QM31,
    trace_1_column_7_offset_0: QM31,
    trace_1_column_87_offset_0: QM31,
) -> QM31 {
    (intermediate6
        + (m31(4).into()) * (trace_1_column_7_offset_0)
        + (m31(2).into()) * (trace_1_column_17_offset_0)
        + (m31(3).into()) * (trace_1_column_27_offset_0)
        + trace_1_column_37_offset_0
        - (trace_1_column_77_offset_0)
        + trace_1_column_47_offset_0
        - (trace_1_column_87_offset_0))
        * (m31(16).into())
}

pub fn intermediate31(intermediate21: QM31) -> QM31 {
    intermediate21 + m31(1).into() - (m31(1).into())
}

pub fn intermediate56(
    intermediate55: QM31, trace_1_column_122_offset_0: QM31, trace_1_column_133_offset_0: QM31,
) -> QM31 {
    (intermediate55
        + (m31(2).into()) * (trace_1_column_122_offset_0)
        - (trace_1_column_133_offset_0))
        * (m31(16).into())
}

pub fn intermediate23(
    intermediate22: QM31, trace_1_column_101_offset_0: QM31, trace_1_column_90_offset_0: QM31,
) -> QM31 {
    (intermediate22
        + (m31(2).into()) * (trace_1_column_90_offset_0)
        - (trace_1_column_101_offset_0))
        * (m31(16).into())
}

pub fn intermediate63(intermediate53: QM31) -> QM31 {
    intermediate53 + m31(1).into() - (m31(1).into())
}

pub fn intermediate95(intermediate85: QM31) -> QM31 {
    intermediate85 + m31(1).into() - (m31(1).into())
}

pub fn intermediate92(intermediate82: QM31) -> QM31 {
    intermediate82 + m31(1).into() - (m31(1).into())
}

pub fn intermediate21(
    intermediate20: QM31, trace_1_column_88_offset_0: QM31, trace_1_column_99_offset_0: QM31,
) -> QM31 {
    (intermediate20 + (m31(2).into()) * (trace_1_column_88_offset_0) - (trace_1_column_99_offset_0))
        * (m31(16).into())
}

pub fn intermediate35(
    trace_1_column_104_offset_0: QM31,
    trace_1_column_114_offset_0: QM31,
    trace_1_column_124_offset_0: QM31,
    trace_1_column_22_offset_0: QM31,
    trace_1_column_32_offset_0: QM31,
    trace_1_column_52_offset_0: QM31,
    trace_1_column_72_offset_0: QM31,
    trace_1_column_93_offset_0: QM31,
) -> QM31 {
    ((m31(4).into()) * (trace_1_column_22_offset_0)
        + (m31(2).into()) * (trace_1_column_32_offset_0)
        + (m31(3).into()) * (trace_1_column_72_offset_0)
        + trace_1_column_93_offset_0
        - (trace_1_column_104_offset_0)
        + trace_1_column_52_offset_0
        - (trace_1_column_114_offset_0)
        - (trace_1_column_124_offset_0))
        * (m31(16).into())
}

pub fn intermediate17(
    intermediate16: QM31, trace_1_column_84_offset_0: QM31, trace_1_column_95_offset_0: QM31,
) -> QM31 {
    (intermediate16 + (m31(2).into()) * (trace_1_column_84_offset_0) - (trace_1_column_95_offset_0))
        * (m31(16).into())
}

pub fn intermediate16(
    intermediate15: QM31, trace_1_column_83_offset_0: QM31, trace_1_column_94_offset_0: QM31,
) -> QM31 {
    (intermediate15 + (m31(2).into()) * (trace_1_column_83_offset_0) - (trace_1_column_94_offset_0))
        * (m31(16).into())
}

pub fn intermediate62(intermediate52: QM31) -> QM31 {
    intermediate52 + m31(1).into() - (m31(1).into())
}

pub fn intermediate76(
    intermediate75: QM31,
    trace_1_column_101_offset_0: QM31,
    trace_1_column_112_offset_0: QM31,
    trace_1_column_133_offset_0: QM31,
    trace_1_column_144_offset_0: QM31,
    trace_1_column_154_offset_0: QM31,
    trace_1_column_70_offset_0: QM31,
    trace_1_column_80_offset_0: QM31,
) -> QM31 {
    (intermediate75
        + (m31(4).into()) * (trace_1_column_80_offset_0)
        + (m31(2).into()) * (trace_1_column_101_offset_0)
        + (m31(3).into()) * (trace_1_column_112_offset_0)
        + trace_1_column_133_offset_0
        - (trace_1_column_144_offset_0)
        + trace_1_column_70_offset_0
        - (trace_1_column_154_offset_0))
        * (m31(16).into())
}

pub fn intermediate40(
    intermediate39: QM31,
    trace_1_column_109_offset_0: QM31,
    trace_1_column_119_offset_0: QM31,
    trace_1_column_27_offset_0: QM31,
    trace_1_column_37_offset_0: QM31,
    trace_1_column_57_offset_0: QM31,
    trace_1_column_77_offset_0: QM31,
    trace_1_column_98_offset_0: QM31,
) -> QM31 {
    (intermediate39
        + (m31(4).into()) * (trace_1_column_27_offset_0)
        + (m31(2).into()) * (trace_1_column_37_offset_0)
        + (m31(3).into()) * (trace_1_column_77_offset_0)
        + trace_1_column_98_offset_0
        - (trace_1_column_109_offset_0)
        + trace_1_column_57_offset_0
        - (trace_1_column_119_offset_0))
        * (m31(16).into())
}

pub fn intermediate66(intermediate56: QM31) -> QM31 {
    intermediate56 + m31(1).into() - (m31(1).into())
}

pub fn intermediate83(
    intermediate82: QM31, trace_1_column_148_offset_0: QM31, trace_1_column_159_offset_0: QM31,
) -> QM31 {
    (intermediate82
        + (m31(2).into()) * (trace_1_column_148_offset_0)
        - (trace_1_column_159_offset_0))
        * (m31(16).into())
}

pub fn intermediate52(
    intermediate51: QM31, trace_1_column_118_offset_0: QM31, trace_1_column_129_offset_0: QM31,
) -> QM31 {
    (intermediate51
        + (m31(2).into()) * (trace_1_column_118_offset_0)
        - (trace_1_column_129_offset_0))
        * (m31(16).into())
}

pub fn intermediate53(
    intermediate52: QM31, trace_1_column_119_offset_0: QM31, trace_1_column_130_offset_0: QM31,
) -> QM31 {
    (intermediate52
        + (m31(2).into()) * (trace_1_column_119_offset_0)
        - (trace_1_column_130_offset_0))
        * (m31(16).into())
}

pub fn intermediate55(
    intermediate54: QM31,
    trace_1_column_121_offset_0: QM31,
    trace_1_column_132_offset_0: QM31,
    trace_1_column_135_offset_0: QM31,
) -> QM31 {
    (intermediate54
        + (m31(2).into()) * (trace_1_column_121_offset_0)
        - (trace_1_column_132_offset_0)
        - ((trace_1_column_135_offset_0) * (m31(136).into())))
        * (m31(16).into())
}

pub fn intermediate20(
    intermediate19: QM31, trace_1_column_87_offset_0: QM31, trace_1_column_98_offset_0: QM31,
) -> QM31 {
    (intermediate19 + (m31(2).into()) * (trace_1_column_87_offset_0) - (trace_1_column_98_offset_0))
        * (m31(16).into())
}

pub fn intermediate57(trace_1_column_135_offset_0: QM31) -> QM31 {
    trace_1_column_135_offset_0 + m31(1).into() - (m31(1).into())
}

pub fn intermediate98(intermediate88: QM31) -> QM31 {
    intermediate88 + m31(1).into() - (m31(1).into())
}

pub fn intermediate25(intermediate15: QM31) -> QM31 {
    intermediate15 + m31(1).into() - (m31(1).into())
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
    trace_1_column_32_offset_0: QM31,
    trace_1_column_33_offset_0: QM31,
    trace_1_column_34_offset_0: QM31,
    trace_1_column_35_offset_0: QM31,
    trace_1_column_36_offset_0: QM31,
    trace_1_column_37_offset_0: QM31,
    trace_1_column_38_offset_0: QM31,
    trace_1_column_39_offset_0: QM31,
    trace_1_column_40_offset_0: QM31,
    trace_1_column_41_offset_0: QM31,
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
) -> QM31 {
    (Cube252_alpha0) * (trace_1_column_32_offset_0)
        + (Cube252_alpha1) * (trace_1_column_33_offset_0)
        + (Cube252_alpha2) * (trace_1_column_34_offset_0)
        + (Cube252_alpha3) * (trace_1_column_35_offset_0)
        + (Cube252_alpha4) * (trace_1_column_36_offset_0)
        + (Cube252_alpha5) * (trace_1_column_37_offset_0)
        + (Cube252_alpha6) * (trace_1_column_38_offset_0)
        + (Cube252_alpha7) * (trace_1_column_39_offset_0)
        + (Cube252_alpha8) * (trace_1_column_40_offset_0)
        + (Cube252_alpha9) * (trace_1_column_41_offset_0)
        + (Cube252_alpha10) * (trace_1_column_72_offset_0)
        + (Cube252_alpha11) * (trace_1_column_73_offset_0)
        + (Cube252_alpha12) * (trace_1_column_74_offset_0)
        + (Cube252_alpha13) * (trace_1_column_75_offset_0)
        + (Cube252_alpha14) * (trace_1_column_76_offset_0)
        + (Cube252_alpha15) * (trace_1_column_77_offset_0)
        + (Cube252_alpha16) * (trace_1_column_78_offset_0)
        + (Cube252_alpha17) * (trace_1_column_79_offset_0)
        + (Cube252_alpha18) * (trace_1_column_80_offset_0)
        + (Cube252_alpha19) * (trace_1_column_81_offset_0)
        - (Cube252_z)
}

pub fn intermediate0(
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
    trace_1_column_68_offset_0: QM31,
    trace_1_column_69_offset_0: QM31,
    trace_1_column_70_offset_0: QM31,
    trace_1_column_71_offset_0: QM31,
) -> QM31 {
    (PoseidonRoundKeys_alpha0) * (trace_1_column_1_offset_0)
        + (PoseidonRoundKeys_alpha1) * (trace_1_column_42_offset_0)
        + (PoseidonRoundKeys_alpha2) * (trace_1_column_43_offset_0)
        + (PoseidonRoundKeys_alpha3) * (trace_1_column_44_offset_0)
        + (PoseidonRoundKeys_alpha4) * (trace_1_column_45_offset_0)
        + (PoseidonRoundKeys_alpha5) * (trace_1_column_46_offset_0)
        + (PoseidonRoundKeys_alpha6) * (trace_1_column_47_offset_0)
        + (PoseidonRoundKeys_alpha7) * (trace_1_column_48_offset_0)
        + (PoseidonRoundKeys_alpha8) * (trace_1_column_49_offset_0)
        + (PoseidonRoundKeys_alpha9) * (trace_1_column_50_offset_0)
        + (PoseidonRoundKeys_alpha10) * (trace_1_column_51_offset_0)
        + (PoseidonRoundKeys_alpha11) * (trace_1_column_52_offset_0)
        + (PoseidonRoundKeys_alpha12) * (trace_1_column_53_offset_0)
        + (PoseidonRoundKeys_alpha13) * (trace_1_column_54_offset_0)
        + (PoseidonRoundKeys_alpha14) * (trace_1_column_55_offset_0)
        + (PoseidonRoundKeys_alpha15) * (trace_1_column_56_offset_0)
        + (PoseidonRoundKeys_alpha16) * (trace_1_column_57_offset_0)
        + (PoseidonRoundKeys_alpha17) * (trace_1_column_58_offset_0)
        + (PoseidonRoundKeys_alpha18) * (trace_1_column_59_offset_0)
        + (PoseidonRoundKeys_alpha19) * (trace_1_column_60_offset_0)
        + (PoseidonRoundKeys_alpha20) * (trace_1_column_61_offset_0)
        + (PoseidonRoundKeys_alpha21) * (trace_1_column_62_offset_0)
        + (PoseidonRoundKeys_alpha22) * (trace_1_column_63_offset_0)
        + (PoseidonRoundKeys_alpha23) * (trace_1_column_64_offset_0)
        + (PoseidonRoundKeys_alpha24) * (trace_1_column_65_offset_0)
        + (PoseidonRoundKeys_alpha25) * (trace_1_column_66_offset_0)
        + (PoseidonRoundKeys_alpha26) * (trace_1_column_67_offset_0)
        + (PoseidonRoundKeys_alpha27) * (trace_1_column_68_offset_0)
        + (PoseidonRoundKeys_alpha28) * (trace_1_column_69_offset_0)
        + (PoseidonRoundKeys_alpha29) * (trace_1_column_70_offset_0)
        + (PoseidonRoundKeys_alpha30) * (trace_1_column_71_offset_0)
        - (PoseidonRoundKeys_z)
}

pub fn intermediate14(
    RangeCheckFelt252Width27_alpha0: QM31,
    RangeCheckFelt252Width27_alpha1: QM31,
    RangeCheckFelt252Width27_alpha2: QM31,
    RangeCheckFelt252Width27_alpha3: QM31,
    RangeCheckFelt252Width27_alpha4: QM31,
    RangeCheckFelt252Width27_alpha5: QM31,
    RangeCheckFelt252Width27_alpha6: QM31,
    RangeCheckFelt252Width27_alpha7: QM31,
    RangeCheckFelt252Width27_alpha8: QM31,
    RangeCheckFelt252Width27_alpha9: QM31,
    RangeCheckFelt252Width27_z: QM31,
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
    (RangeCheckFelt252Width27_alpha0) * (trace_1_column_82_offset_0)
        + (RangeCheckFelt252Width27_alpha1) * (trace_1_column_83_offset_0)
        + (RangeCheckFelt252Width27_alpha2) * (trace_1_column_84_offset_0)
        + (RangeCheckFelt252Width27_alpha3) * (trace_1_column_85_offset_0)
        + (RangeCheckFelt252Width27_alpha4) * (trace_1_column_86_offset_0)
        + (RangeCheckFelt252Width27_alpha5) * (trace_1_column_87_offset_0)
        + (RangeCheckFelt252Width27_alpha6) * (trace_1_column_88_offset_0)
        + (RangeCheckFelt252Width27_alpha7) * (trace_1_column_89_offset_0)
        + (RangeCheckFelt252Width27_alpha8) * (trace_1_column_90_offset_0)
        + (RangeCheckFelt252Width27_alpha9) * (trace_1_column_91_offset_0)
        - (RangeCheckFelt252Width27_z)
}

pub fn intermediate44(
    RangeCheck_4_4_4_4_alpha0: QM31,
    RangeCheck_4_4_4_4_alpha1: QM31,
    RangeCheck_4_4_4_4_alpha2: QM31,
    RangeCheck_4_4_4_4_alpha3: QM31,
    RangeCheck_4_4_4_4_z: QM31,
    intermediate35: QM31,
    intermediate36: QM31,
    intermediate37: QM31,
    trace_1_column_124_offset_0: QM31,
) -> QM31 {
    (RangeCheck_4_4_4_4_alpha0) * (trace_1_column_124_offset_0 + m31(2).into())
        + (RangeCheck_4_4_4_4_alpha1) * (intermediate35 + m31(2).into())
        + (RangeCheck_4_4_4_4_alpha2) * (intermediate36 + m31(2).into())
        + (RangeCheck_4_4_4_4_alpha3) * (intermediate37 + m31(2).into())
        - (RangeCheck_4_4_4_4_z)
}

pub fn intermediate45(
    RangeCheck_4_4_4_4_alpha0: QM31,
    RangeCheck_4_4_4_4_alpha1: QM31,
    RangeCheck_4_4_4_4_alpha2: QM31,
    RangeCheck_4_4_4_4_alpha3: QM31,
    RangeCheck_4_4_4_4_z: QM31,
    intermediate38: QM31,
    intermediate39: QM31,
    intermediate40: QM31,
    intermediate41: QM31,
) -> QM31 {
    (RangeCheck_4_4_4_4_alpha0) * (intermediate38 + m31(2).into())
        + (RangeCheck_4_4_4_4_alpha1) * (intermediate39 + m31(2).into())
        + (RangeCheck_4_4_4_4_alpha2) * (intermediate40 + m31(2).into())
        + (RangeCheck_4_4_4_4_alpha3) * (intermediate41 + m31(2).into())
        - (RangeCheck_4_4_4_4_z)
}

pub fn intermediate80(
    RangeCheckFelt252Width27_alpha0: QM31,
    RangeCheckFelt252Width27_alpha1: QM31,
    RangeCheckFelt252Width27_alpha2: QM31,
    RangeCheckFelt252Width27_alpha3: QM31,
    RangeCheckFelt252Width27_alpha4: QM31,
    RangeCheckFelt252Width27_alpha5: QM31,
    RangeCheckFelt252Width27_alpha6: QM31,
    RangeCheckFelt252Width27_alpha7: QM31,
    RangeCheckFelt252Width27_alpha8: QM31,
    RangeCheckFelt252Width27_alpha9: QM31,
    RangeCheckFelt252Width27_z: QM31,
    trace_1_column_146_offset_0: QM31,
    trace_1_column_147_offset_0: QM31,
    trace_1_column_148_offset_0: QM31,
    trace_1_column_149_offset_0: QM31,
    trace_1_column_150_offset_0: QM31,
    trace_1_column_151_offset_0: QM31,
    trace_1_column_152_offset_0: QM31,
    trace_1_column_153_offset_0: QM31,
    trace_1_column_154_offset_0: QM31,
    trace_1_column_155_offset_0: QM31,
) -> QM31 {
    (RangeCheckFelt252Width27_alpha0) * (trace_1_column_146_offset_0)
        + (RangeCheckFelt252Width27_alpha1) * (trace_1_column_147_offset_0)
        + (RangeCheckFelt252Width27_alpha2) * (trace_1_column_148_offset_0)
        + (RangeCheckFelt252Width27_alpha3) * (trace_1_column_149_offset_0)
        + (RangeCheckFelt252Width27_alpha4) * (trace_1_column_150_offset_0)
        + (RangeCheckFelt252Width27_alpha5) * (trace_1_column_151_offset_0)
        + (RangeCheckFelt252Width27_alpha6) * (trace_1_column_152_offset_0)
        + (RangeCheckFelt252Width27_alpha7) * (trace_1_column_153_offset_0)
        + (RangeCheckFelt252Width27_alpha8) * (trace_1_column_154_offset_0)
        + (RangeCheckFelt252Width27_alpha9) * (trace_1_column_155_offset_0)
        - (RangeCheckFelt252Width27_z)
}

pub fn intermediate101(
    Poseidon3PartialRoundsChain_alpha0: QM31,
    Poseidon3PartialRoundsChain_alpha1: QM31,
    Poseidon3PartialRoundsChain_alpha10: QM31,
    Poseidon3PartialRoundsChain_alpha11: QM31,
    Poseidon3PartialRoundsChain_alpha12: QM31,
    Poseidon3PartialRoundsChain_alpha13: QM31,
    Poseidon3PartialRoundsChain_alpha14: QM31,
    Poseidon3PartialRoundsChain_alpha15: QM31,
    Poseidon3PartialRoundsChain_alpha16: QM31,
    Poseidon3PartialRoundsChain_alpha17: QM31,
    Poseidon3PartialRoundsChain_alpha18: QM31,
    Poseidon3PartialRoundsChain_alpha19: QM31,
    Poseidon3PartialRoundsChain_alpha2: QM31,
    Poseidon3PartialRoundsChain_alpha20: QM31,
    Poseidon3PartialRoundsChain_alpha21: QM31,
    Poseidon3PartialRoundsChain_alpha22: QM31,
    Poseidon3PartialRoundsChain_alpha23: QM31,
    Poseidon3PartialRoundsChain_alpha24: QM31,
    Poseidon3PartialRoundsChain_alpha25: QM31,
    Poseidon3PartialRoundsChain_alpha26: QM31,
    Poseidon3PartialRoundsChain_alpha27: QM31,
    Poseidon3PartialRoundsChain_alpha28: QM31,
    Poseidon3PartialRoundsChain_alpha29: QM31,
    Poseidon3PartialRoundsChain_alpha3: QM31,
    Poseidon3PartialRoundsChain_alpha30: QM31,
    Poseidon3PartialRoundsChain_alpha31: QM31,
    Poseidon3PartialRoundsChain_alpha32: QM31,
    Poseidon3PartialRoundsChain_alpha33: QM31,
    Poseidon3PartialRoundsChain_alpha34: QM31,
    Poseidon3PartialRoundsChain_alpha35: QM31,
    Poseidon3PartialRoundsChain_alpha36: QM31,
    Poseidon3PartialRoundsChain_alpha37: QM31,
    Poseidon3PartialRoundsChain_alpha38: QM31,
    Poseidon3PartialRoundsChain_alpha39: QM31,
    Poseidon3PartialRoundsChain_alpha4: QM31,
    Poseidon3PartialRoundsChain_alpha40: QM31,
    Poseidon3PartialRoundsChain_alpha41: QM31,
    Poseidon3PartialRoundsChain_alpha5: QM31,
    Poseidon3PartialRoundsChain_alpha6: QM31,
    Poseidon3PartialRoundsChain_alpha7: QM31,
    Poseidon3PartialRoundsChain_alpha8: QM31,
    Poseidon3PartialRoundsChain_alpha9: QM31,
    Poseidon3PartialRoundsChain_z: QM31,
    trace_1_column_0_offset_0: QM31,
    trace_1_column_104_offset_0: QM31,
    trace_1_column_105_offset_0: QM31,
    trace_1_column_106_offset_0: QM31,
    trace_1_column_107_offset_0: QM31,
    trace_1_column_108_offset_0: QM31,
    trace_1_column_109_offset_0: QM31,
    trace_1_column_110_offset_0: QM31,
    trace_1_column_111_offset_0: QM31,
    trace_1_column_112_offset_0: QM31,
    trace_1_column_113_offset_0: QM31,
    trace_1_column_125_offset_0: QM31,
    trace_1_column_126_offset_0: QM31,
    trace_1_column_127_offset_0: QM31,
    trace_1_column_128_offset_0: QM31,
    trace_1_column_129_offset_0: QM31,
    trace_1_column_130_offset_0: QM31,
    trace_1_column_131_offset_0: QM31,
    trace_1_column_132_offset_0: QM31,
    trace_1_column_133_offset_0: QM31,
    trace_1_column_134_offset_0: QM31,
    trace_1_column_136_offset_0: QM31,
    trace_1_column_137_offset_0: QM31,
    trace_1_column_138_offset_0: QM31,
    trace_1_column_139_offset_0: QM31,
    trace_1_column_140_offset_0: QM31,
    trace_1_column_141_offset_0: QM31,
    trace_1_column_142_offset_0: QM31,
    trace_1_column_143_offset_0: QM31,
    trace_1_column_144_offset_0: QM31,
    trace_1_column_145_offset_0: QM31,
    trace_1_column_157_offset_0: QM31,
    trace_1_column_158_offset_0: QM31,
    trace_1_column_159_offset_0: QM31,
    trace_1_column_160_offset_0: QM31,
    trace_1_column_161_offset_0: QM31,
    trace_1_column_162_offset_0: QM31,
    trace_1_column_163_offset_0: QM31,
    trace_1_column_164_offset_0: QM31,
    trace_1_column_165_offset_0: QM31,
    trace_1_column_166_offset_0: QM31,
    trace_1_column_1_offset_0: QM31,
) -> QM31 {
    (Poseidon3PartialRoundsChain_alpha0) * (trace_1_column_0_offset_0)
        + (Poseidon3PartialRoundsChain_alpha1) * (trace_1_column_1_offset_0 + m31(1).into())
        + (Poseidon3PartialRoundsChain_alpha2) * (trace_1_column_104_offset_0)
        + (Poseidon3PartialRoundsChain_alpha3) * (trace_1_column_105_offset_0)
        + (Poseidon3PartialRoundsChain_alpha4) * (trace_1_column_106_offset_0)
        + (Poseidon3PartialRoundsChain_alpha5) * (trace_1_column_107_offset_0)
        + (Poseidon3PartialRoundsChain_alpha6) * (trace_1_column_108_offset_0)
        + (Poseidon3PartialRoundsChain_alpha7) * (trace_1_column_109_offset_0)
        + (Poseidon3PartialRoundsChain_alpha8) * (trace_1_column_110_offset_0)
        + (Poseidon3PartialRoundsChain_alpha9) * (trace_1_column_111_offset_0)
        + (Poseidon3PartialRoundsChain_alpha10) * (trace_1_column_112_offset_0)
        + (Poseidon3PartialRoundsChain_alpha11) * (trace_1_column_113_offset_0)
        + (Poseidon3PartialRoundsChain_alpha12) * (trace_1_column_125_offset_0)
        + (Poseidon3PartialRoundsChain_alpha13) * (trace_1_column_126_offset_0)
        + (Poseidon3PartialRoundsChain_alpha14) * (trace_1_column_127_offset_0)
        + (Poseidon3PartialRoundsChain_alpha15) * (trace_1_column_128_offset_0)
        + (Poseidon3PartialRoundsChain_alpha16) * (trace_1_column_129_offset_0)
        + (Poseidon3PartialRoundsChain_alpha17) * (trace_1_column_130_offset_0)
        + (Poseidon3PartialRoundsChain_alpha18) * (trace_1_column_131_offset_0)
        + (Poseidon3PartialRoundsChain_alpha19) * (trace_1_column_132_offset_0)
        + (Poseidon3PartialRoundsChain_alpha20) * (trace_1_column_133_offset_0)
        + (Poseidon3PartialRoundsChain_alpha21) * (trace_1_column_134_offset_0)
        + (Poseidon3PartialRoundsChain_alpha22) * (trace_1_column_136_offset_0)
        + (Poseidon3PartialRoundsChain_alpha23) * (trace_1_column_137_offset_0)
        + (Poseidon3PartialRoundsChain_alpha24) * (trace_1_column_138_offset_0)
        + (Poseidon3PartialRoundsChain_alpha25) * (trace_1_column_139_offset_0)
        + (Poseidon3PartialRoundsChain_alpha26) * (trace_1_column_140_offset_0)
        + (Poseidon3PartialRoundsChain_alpha27) * (trace_1_column_141_offset_0)
        + (Poseidon3PartialRoundsChain_alpha28) * (trace_1_column_142_offset_0)
        + (Poseidon3PartialRoundsChain_alpha29) * (trace_1_column_143_offset_0)
        + (Poseidon3PartialRoundsChain_alpha30) * (trace_1_column_144_offset_0)
        + (Poseidon3PartialRoundsChain_alpha31) * (trace_1_column_145_offset_0)
        + (Poseidon3PartialRoundsChain_alpha32) * (trace_1_column_157_offset_0)
        + (Poseidon3PartialRoundsChain_alpha33) * (trace_1_column_158_offset_0)
        + (Poseidon3PartialRoundsChain_alpha34) * (trace_1_column_159_offset_0)
        + (Poseidon3PartialRoundsChain_alpha35) * (trace_1_column_160_offset_0)
        + (Poseidon3PartialRoundsChain_alpha36) * (trace_1_column_161_offset_0)
        + (Poseidon3PartialRoundsChain_alpha37) * (trace_1_column_162_offset_0)
        + (Poseidon3PartialRoundsChain_alpha38) * (trace_1_column_163_offset_0)
        + (Poseidon3PartialRoundsChain_alpha39) * (trace_1_column_164_offset_0)
        + (Poseidon3PartialRoundsChain_alpha40) * (trace_1_column_165_offset_0)
        + (Poseidon3PartialRoundsChain_alpha41) * (trace_1_column_166_offset_0)
        - (Poseidon3PartialRoundsChain_z)
}

pub fn intermediate77(
    RangeCheck_4_4_4_4_alpha0: QM31,
    RangeCheck_4_4_4_4_alpha1: QM31,
    RangeCheck_4_4_4_4_alpha2: QM31,
    RangeCheck_4_4_4_4_alpha3: QM31,
    RangeCheck_4_4_4_4_z: QM31,
    intermediate68: QM31,
    intermediate69: QM31,
    intermediate70: QM31,
    trace_1_column_156_offset_0: QM31,
) -> QM31 {
    (RangeCheck_4_4_4_4_alpha0) * (trace_1_column_156_offset_0 + m31(2).into())
        + (RangeCheck_4_4_4_4_alpha1) * (intermediate68 + m31(2).into())
        + (RangeCheck_4_4_4_4_alpha2) * (intermediate69 + m31(2).into())
        + (RangeCheck_4_4_4_4_alpha3) * (intermediate70 + m31(2).into())
        - (RangeCheck_4_4_4_4_z)
}

pub fn intermediate13(
    RangeCheck_4_4_alpha0: QM31,
    RangeCheck_4_4_alpha1: QM31,
    RangeCheck_4_4_z: QM31,
    intermediate10: QM31,
    intermediate9: QM31,
) -> QM31 {
    (RangeCheck_4_4_alpha0) * (intermediate9 + m31(2).into())
        + (RangeCheck_4_4_alpha1) * (intermediate10 + m31(2).into())
        - (RangeCheck_4_4_z)
}

pub fn intermediate67(
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
    trace_1_column_125_offset_0: QM31,
    trace_1_column_126_offset_0: QM31,
    trace_1_column_127_offset_0: QM31,
    trace_1_column_128_offset_0: QM31,
    trace_1_column_129_offset_0: QM31,
    trace_1_column_130_offset_0: QM31,
    trace_1_column_131_offset_0: QM31,
    trace_1_column_132_offset_0: QM31,
    trace_1_column_133_offset_0: QM31,
    trace_1_column_134_offset_0: QM31,
    trace_1_column_136_offset_0: QM31,
    trace_1_column_137_offset_0: QM31,
    trace_1_column_138_offset_0: QM31,
    trace_1_column_139_offset_0: QM31,
    trace_1_column_140_offset_0: QM31,
    trace_1_column_141_offset_0: QM31,
    trace_1_column_142_offset_0: QM31,
    trace_1_column_143_offset_0: QM31,
    trace_1_column_144_offset_0: QM31,
    trace_1_column_145_offset_0: QM31,
) -> QM31 {
    (Cube252_alpha0) * (trace_1_column_125_offset_0)
        + (Cube252_alpha1) * (trace_1_column_126_offset_0)
        + (Cube252_alpha2) * (trace_1_column_127_offset_0)
        + (Cube252_alpha3) * (trace_1_column_128_offset_0)
        + (Cube252_alpha4) * (trace_1_column_129_offset_0)
        + (Cube252_alpha5) * (trace_1_column_130_offset_0)
        + (Cube252_alpha6) * (trace_1_column_131_offset_0)
        + (Cube252_alpha7) * (trace_1_column_132_offset_0)
        + (Cube252_alpha8) * (trace_1_column_133_offset_0)
        + (Cube252_alpha9) * (trace_1_column_134_offset_0)
        + (Cube252_alpha10) * (trace_1_column_136_offset_0)
        + (Cube252_alpha11) * (trace_1_column_137_offset_0)
        + (Cube252_alpha12) * (trace_1_column_138_offset_0)
        + (Cube252_alpha13) * (trace_1_column_139_offset_0)
        + (Cube252_alpha14) * (trace_1_column_140_offset_0)
        + (Cube252_alpha15) * (trace_1_column_141_offset_0)
        + (Cube252_alpha16) * (trace_1_column_142_offset_0)
        + (Cube252_alpha17) * (trace_1_column_143_offset_0)
        + (Cube252_alpha18) * (trace_1_column_144_offset_0)
        + (Cube252_alpha19) * (trace_1_column_145_offset_0)
        - (Cube252_z)
}

pub fn intermediate12(
    RangeCheck_4_4_4_4_alpha0: QM31,
    RangeCheck_4_4_4_4_alpha1: QM31,
    RangeCheck_4_4_4_4_alpha2: QM31,
    RangeCheck_4_4_4_4_alpha3: QM31,
    RangeCheck_4_4_4_4_z: QM31,
    intermediate5: QM31,
    intermediate6: QM31,
    intermediate7: QM31,
    intermediate8: QM31,
) -> QM31 {
    (RangeCheck_4_4_4_4_alpha0) * (intermediate5 + m31(2).into())
        + (RangeCheck_4_4_4_4_alpha1) * (intermediate6 + m31(2).into())
        + (RangeCheck_4_4_4_4_alpha2) * (intermediate7 + m31(2).into())
        + (RangeCheck_4_4_4_4_alpha3) * (intermediate8 + m31(2).into())
        - (RangeCheck_4_4_4_4_z)
}

pub fn intermediate11(
    RangeCheck_4_4_4_4_alpha0: QM31,
    RangeCheck_4_4_4_4_alpha1: QM31,
    RangeCheck_4_4_4_4_alpha2: QM31,
    RangeCheck_4_4_4_4_alpha3: QM31,
    RangeCheck_4_4_4_4_z: QM31,
    intermediate2: QM31,
    intermediate3: QM31,
    intermediate4: QM31,
    trace_1_column_92_offset_0: QM31,
) -> QM31 {
    (RangeCheck_4_4_4_4_alpha0) * (trace_1_column_92_offset_0 + m31(2).into())
        + (RangeCheck_4_4_4_4_alpha1) * (intermediate2 + m31(2).into())
        + (RangeCheck_4_4_4_4_alpha2) * (intermediate3 + m31(2).into())
        + (RangeCheck_4_4_4_4_alpha3) * (intermediate4 + m31(2).into())
        - (RangeCheck_4_4_4_4_z)
}

pub fn intermediate46(
    RangeCheck_4_4_alpha0: QM31,
    RangeCheck_4_4_alpha1: QM31,
    RangeCheck_4_4_z: QM31,
    intermediate42: QM31,
    intermediate43: QM31,
) -> QM31 {
    (RangeCheck_4_4_alpha0) * (intermediate42 + m31(2).into())
        + (RangeCheck_4_4_alpha1) * (intermediate43 + m31(2).into())
        - (RangeCheck_4_4_z)
}

pub fn intermediate34(
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
    trace_1_column_100_offset_0: QM31,
    trace_1_column_101_offset_0: QM31,
    trace_1_column_102_offset_0: QM31,
    trace_1_column_104_offset_0: QM31,
    trace_1_column_105_offset_0: QM31,
    trace_1_column_106_offset_0: QM31,
    trace_1_column_107_offset_0: QM31,
    trace_1_column_108_offset_0: QM31,
    trace_1_column_109_offset_0: QM31,
    trace_1_column_110_offset_0: QM31,
    trace_1_column_111_offset_0: QM31,
    trace_1_column_112_offset_0: QM31,
    trace_1_column_113_offset_0: QM31,
    trace_1_column_93_offset_0: QM31,
    trace_1_column_94_offset_0: QM31,
    trace_1_column_95_offset_0: QM31,
    trace_1_column_96_offset_0: QM31,
    trace_1_column_97_offset_0: QM31,
    trace_1_column_98_offset_0: QM31,
    trace_1_column_99_offset_0: QM31,
) -> QM31 {
    (Cube252_alpha0) * (trace_1_column_93_offset_0)
        + (Cube252_alpha1) * (trace_1_column_94_offset_0)
        + (Cube252_alpha2) * (trace_1_column_95_offset_0)
        + (Cube252_alpha3) * (trace_1_column_96_offset_0)
        + (Cube252_alpha4) * (trace_1_column_97_offset_0)
        + (Cube252_alpha5) * (trace_1_column_98_offset_0)
        + (Cube252_alpha6) * (trace_1_column_99_offset_0)
        + (Cube252_alpha7) * (trace_1_column_100_offset_0)
        + (Cube252_alpha8) * (trace_1_column_101_offset_0)
        + (Cube252_alpha9) * (trace_1_column_102_offset_0)
        + (Cube252_alpha10) * (trace_1_column_104_offset_0)
        + (Cube252_alpha11) * (trace_1_column_105_offset_0)
        + (Cube252_alpha12) * (trace_1_column_106_offset_0)
        + (Cube252_alpha13) * (trace_1_column_107_offset_0)
        + (Cube252_alpha14) * (trace_1_column_108_offset_0)
        + (Cube252_alpha15) * (trace_1_column_109_offset_0)
        + (Cube252_alpha16) * (trace_1_column_110_offset_0)
        + (Cube252_alpha17) * (trace_1_column_111_offset_0)
        + (Cube252_alpha18) * (trace_1_column_112_offset_0)
        + (Cube252_alpha19) * (trace_1_column_113_offset_0)
        - (Cube252_z)
}

pub fn intermediate47(
    RangeCheckFelt252Width27_alpha0: QM31,
    RangeCheckFelt252Width27_alpha1: QM31,
    RangeCheckFelt252Width27_alpha2: QM31,
    RangeCheckFelt252Width27_alpha3: QM31,
    RangeCheckFelt252Width27_alpha4: QM31,
    RangeCheckFelt252Width27_alpha5: QM31,
    RangeCheckFelt252Width27_alpha6: QM31,
    RangeCheckFelt252Width27_alpha7: QM31,
    RangeCheckFelt252Width27_alpha8: QM31,
    RangeCheckFelt252Width27_alpha9: QM31,
    RangeCheckFelt252Width27_z: QM31,
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
) -> QM31 {
    (RangeCheckFelt252Width27_alpha0) * (trace_1_column_114_offset_0)
        + (RangeCheckFelt252Width27_alpha1) * (trace_1_column_115_offset_0)
        + (RangeCheckFelt252Width27_alpha2) * (trace_1_column_116_offset_0)
        + (RangeCheckFelt252Width27_alpha3) * (trace_1_column_117_offset_0)
        + (RangeCheckFelt252Width27_alpha4) * (trace_1_column_118_offset_0)
        + (RangeCheckFelt252Width27_alpha5) * (trace_1_column_119_offset_0)
        + (RangeCheckFelt252Width27_alpha6) * (trace_1_column_120_offset_0)
        + (RangeCheckFelt252Width27_alpha7) * (trace_1_column_121_offset_0)
        + (RangeCheckFelt252Width27_alpha8) * (trace_1_column_122_offset_0)
        + (RangeCheckFelt252Width27_alpha9) * (trace_1_column_123_offset_0)
        - (RangeCheckFelt252Width27_z)
}

pub fn intermediate79(
    RangeCheck_4_4_alpha0: QM31,
    RangeCheck_4_4_alpha1: QM31,
    RangeCheck_4_4_z: QM31,
    intermediate75: QM31,
    intermediate76: QM31,
) -> QM31 {
    (RangeCheck_4_4_alpha0) * (intermediate75 + m31(2).into())
        + (RangeCheck_4_4_alpha1) * (intermediate76 + m31(2).into())
        - (RangeCheck_4_4_z)
}

pub fn intermediate100(
    Poseidon3PartialRoundsChain_alpha0: QM31,
    Poseidon3PartialRoundsChain_alpha1: QM31,
    Poseidon3PartialRoundsChain_alpha10: QM31,
    Poseidon3PartialRoundsChain_alpha11: QM31,
    Poseidon3PartialRoundsChain_alpha12: QM31,
    Poseidon3PartialRoundsChain_alpha13: QM31,
    Poseidon3PartialRoundsChain_alpha14: QM31,
    Poseidon3PartialRoundsChain_alpha15: QM31,
    Poseidon3PartialRoundsChain_alpha16: QM31,
    Poseidon3PartialRoundsChain_alpha17: QM31,
    Poseidon3PartialRoundsChain_alpha18: QM31,
    Poseidon3PartialRoundsChain_alpha19: QM31,
    Poseidon3PartialRoundsChain_alpha2: QM31,
    Poseidon3PartialRoundsChain_alpha20: QM31,
    Poseidon3PartialRoundsChain_alpha21: QM31,
    Poseidon3PartialRoundsChain_alpha22: QM31,
    Poseidon3PartialRoundsChain_alpha23: QM31,
    Poseidon3PartialRoundsChain_alpha24: QM31,
    Poseidon3PartialRoundsChain_alpha25: QM31,
    Poseidon3PartialRoundsChain_alpha26: QM31,
    Poseidon3PartialRoundsChain_alpha27: QM31,
    Poseidon3PartialRoundsChain_alpha28: QM31,
    Poseidon3PartialRoundsChain_alpha29: QM31,
    Poseidon3PartialRoundsChain_alpha3: QM31,
    Poseidon3PartialRoundsChain_alpha30: QM31,
    Poseidon3PartialRoundsChain_alpha31: QM31,
    Poseidon3PartialRoundsChain_alpha32: QM31,
    Poseidon3PartialRoundsChain_alpha33: QM31,
    Poseidon3PartialRoundsChain_alpha34: QM31,
    Poseidon3PartialRoundsChain_alpha35: QM31,
    Poseidon3PartialRoundsChain_alpha36: QM31,
    Poseidon3PartialRoundsChain_alpha37: QM31,
    Poseidon3PartialRoundsChain_alpha38: QM31,
    Poseidon3PartialRoundsChain_alpha39: QM31,
    Poseidon3PartialRoundsChain_alpha4: QM31,
    Poseidon3PartialRoundsChain_alpha40: QM31,
    Poseidon3PartialRoundsChain_alpha41: QM31,
    Poseidon3PartialRoundsChain_alpha5: QM31,
    Poseidon3PartialRoundsChain_alpha6: QM31,
    Poseidon3PartialRoundsChain_alpha7: QM31,
    Poseidon3PartialRoundsChain_alpha8: QM31,
    Poseidon3PartialRoundsChain_alpha9: QM31,
    Poseidon3PartialRoundsChain_z: QM31,
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
    trace_1_column_4_offset_0: QM31,
    trace_1_column_5_offset_0: QM31,
    trace_1_column_6_offset_0: QM31,
    trace_1_column_7_offset_0: QM31,
    trace_1_column_8_offset_0: QM31,
    trace_1_column_9_offset_0: QM31,
) -> QM31 {
    (Poseidon3PartialRoundsChain_alpha0) * (trace_1_column_0_offset_0)
        + (Poseidon3PartialRoundsChain_alpha1) * (trace_1_column_1_offset_0)
        + (Poseidon3PartialRoundsChain_alpha2) * (trace_1_column_2_offset_0)
        + (Poseidon3PartialRoundsChain_alpha3) * (trace_1_column_3_offset_0)
        + (Poseidon3PartialRoundsChain_alpha4) * (trace_1_column_4_offset_0)
        + (Poseidon3PartialRoundsChain_alpha5) * (trace_1_column_5_offset_0)
        + (Poseidon3PartialRoundsChain_alpha6) * (trace_1_column_6_offset_0)
        + (Poseidon3PartialRoundsChain_alpha7) * (trace_1_column_7_offset_0)
        + (Poseidon3PartialRoundsChain_alpha8) * (trace_1_column_8_offset_0)
        + (Poseidon3PartialRoundsChain_alpha9) * (trace_1_column_9_offset_0)
        + (Poseidon3PartialRoundsChain_alpha10) * (trace_1_column_10_offset_0)
        + (Poseidon3PartialRoundsChain_alpha11) * (trace_1_column_11_offset_0)
        + (Poseidon3PartialRoundsChain_alpha12) * (trace_1_column_12_offset_0)
        + (Poseidon3PartialRoundsChain_alpha13) * (trace_1_column_13_offset_0)
        + (Poseidon3PartialRoundsChain_alpha14) * (trace_1_column_14_offset_0)
        + (Poseidon3PartialRoundsChain_alpha15) * (trace_1_column_15_offset_0)
        + (Poseidon3PartialRoundsChain_alpha16) * (trace_1_column_16_offset_0)
        + (Poseidon3PartialRoundsChain_alpha17) * (trace_1_column_17_offset_0)
        + (Poseidon3PartialRoundsChain_alpha18) * (trace_1_column_18_offset_0)
        + (Poseidon3PartialRoundsChain_alpha19) * (trace_1_column_19_offset_0)
        + (Poseidon3PartialRoundsChain_alpha20) * (trace_1_column_20_offset_0)
        + (Poseidon3PartialRoundsChain_alpha21) * (trace_1_column_21_offset_0)
        + (Poseidon3PartialRoundsChain_alpha22) * (trace_1_column_22_offset_0)
        + (Poseidon3PartialRoundsChain_alpha23) * (trace_1_column_23_offset_0)
        + (Poseidon3PartialRoundsChain_alpha24) * (trace_1_column_24_offset_0)
        + (Poseidon3PartialRoundsChain_alpha25) * (trace_1_column_25_offset_0)
        + (Poseidon3PartialRoundsChain_alpha26) * (trace_1_column_26_offset_0)
        + (Poseidon3PartialRoundsChain_alpha27) * (trace_1_column_27_offset_0)
        + (Poseidon3PartialRoundsChain_alpha28) * (trace_1_column_28_offset_0)
        + (Poseidon3PartialRoundsChain_alpha29) * (trace_1_column_29_offset_0)
        + (Poseidon3PartialRoundsChain_alpha30) * (trace_1_column_30_offset_0)
        + (Poseidon3PartialRoundsChain_alpha31) * (trace_1_column_31_offset_0)
        + (Poseidon3PartialRoundsChain_alpha32) * (trace_1_column_32_offset_0)
        + (Poseidon3PartialRoundsChain_alpha33) * (trace_1_column_33_offset_0)
        + (Poseidon3PartialRoundsChain_alpha34) * (trace_1_column_34_offset_0)
        + (Poseidon3PartialRoundsChain_alpha35) * (trace_1_column_35_offset_0)
        + (Poseidon3PartialRoundsChain_alpha36) * (trace_1_column_36_offset_0)
        + (Poseidon3PartialRoundsChain_alpha37) * (trace_1_column_37_offset_0)
        + (Poseidon3PartialRoundsChain_alpha38) * (trace_1_column_38_offset_0)
        + (Poseidon3PartialRoundsChain_alpha39) * (trace_1_column_39_offset_0)
        + (Poseidon3PartialRoundsChain_alpha40) * (trace_1_column_40_offset_0)
        + (Poseidon3PartialRoundsChain_alpha41) * (trace_1_column_41_offset_0)
        - (Poseidon3PartialRoundsChain_z)
}

pub fn intermediate78(
    RangeCheck_4_4_4_4_alpha0: QM31,
    RangeCheck_4_4_4_4_alpha1: QM31,
    RangeCheck_4_4_4_4_alpha2: QM31,
    RangeCheck_4_4_4_4_alpha3: QM31,
    RangeCheck_4_4_4_4_z: QM31,
    intermediate71: QM31,
    intermediate72: QM31,
    intermediate73: QM31,
    intermediate74: QM31,
) -> QM31 {
    (RangeCheck_4_4_4_4_alpha0) * (intermediate71 + m31(2).into())
        + (RangeCheck_4_4_4_4_alpha1) * (intermediate72 + m31(2).into())
        + (RangeCheck_4_4_4_4_alpha2) * (intermediate73 + m31(2).into())
        + (RangeCheck_4_4_4_4_alpha3) * (intermediate74 + m31(2).into())
        - (RangeCheck_4_4_4_4_z)
}

