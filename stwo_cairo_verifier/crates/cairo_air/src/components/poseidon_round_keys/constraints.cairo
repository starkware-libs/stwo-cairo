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
    preprocessed_column_set.insert(PreprocessedColumn::Seq(log_size));
    preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys(0));
    preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys(1));
    preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys(2));
    preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys(3));
    preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys(4));
    preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys(5));
    preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys(6));
    preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys(7));
    preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys(8));
    preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys(9));
    preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys(10));
    preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys(11));
    preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys(12));
    preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys(13));
    preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys(14));
    preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys(15));
    preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys(16));
    preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys(17));
    preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys(18));
    preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys(19));
    preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys(20));
    preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys(21));
    preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys(22));
    preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys(23));
    preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys(24));
    preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys(25));
    preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys(26));
    preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys(27));
    preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys(28));
    preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys(29));
    let point_offset_neg_1 = point.add_circle_point_m31(-trace_gen.mul(1).to_point());
    trace_mask_points.append(array![point]);
    interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
    interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
    interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
    interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
}

#[derive(Drop)]
pub struct ConstraintParams {
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
    pub claimed_sum: QM31,
    pub poseidon_round_keys_0: QM31,
    pub poseidon_round_keys_1: QM31,
    pub poseidon_round_keys_10: QM31,
    pub poseidon_round_keys_11: QM31,
    pub poseidon_round_keys_12: QM31,
    pub poseidon_round_keys_13: QM31,
    pub poseidon_round_keys_14: QM31,
    pub poseidon_round_keys_15: QM31,
    pub poseidon_round_keys_16: QM31,
    pub poseidon_round_keys_17: QM31,
    pub poseidon_round_keys_18: QM31,
    pub poseidon_round_keys_19: QM31,
    pub poseidon_round_keys_2: QM31,
    pub poseidon_round_keys_20: QM31,
    pub poseidon_round_keys_21: QM31,
    pub poseidon_round_keys_22: QM31,
    pub poseidon_round_keys_23: QM31,
    pub poseidon_round_keys_24: QM31,
    pub poseidon_round_keys_25: QM31,
    pub poseidon_round_keys_26: QM31,
    pub poseidon_round_keys_27: QM31,
    pub poseidon_round_keys_28: QM31,
    pub poseidon_round_keys_29: QM31,
    pub poseidon_round_keys_3: QM31,
    pub poseidon_round_keys_4: QM31,
    pub poseidon_round_keys_5: QM31,
    pub poseidon_round_keys_6: QM31,
    pub poseidon_round_keys_7: QM31,
    pub poseidon_round_keys_8: QM31,
    pub poseidon_round_keys_9: QM31,
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
        claimed_sum,
        poseidon_round_keys_0,
        poseidon_round_keys_1,
        poseidon_round_keys_10,
        poseidon_round_keys_11,
        poseidon_round_keys_12,
        poseidon_round_keys_13,
        poseidon_round_keys_14,
        poseidon_round_keys_15,
        poseidon_round_keys_16,
        poseidon_round_keys_17,
        poseidon_round_keys_18,
        poseidon_round_keys_19,
        poseidon_round_keys_2,
        poseidon_round_keys_20,
        poseidon_round_keys_21,
        poseidon_round_keys_22,
        poseidon_round_keys_23,
        poseidon_round_keys_24,
        poseidon_round_keys_25,
        poseidon_round_keys_26,
        poseidon_round_keys_27,
        poseidon_round_keys_28,
        poseidon_round_keys_29,
        poseidon_round_keys_3,
        poseidon_round_keys_4,
        poseidon_round_keys_5,
        poseidon_round_keys_6,
        poseidon_round_keys_7,
        poseidon_round_keys_8,
        poseidon_round_keys_9,
        seq,
        column_size,
    } = params;
    let [trace_1_column_0]: [Span<QM31>; 1] = (*trace_mask_values.multi_pop_front().unwrap())
        .unbox();

    let [trace_1_column_0_offset_0]: [QM31; 1] = (*trace_1_column_0.try_into().unwrap()).unbox();

    let [trace_2_column_1, trace_2_column_2, trace_2_column_3, trace_2_column_4]: [Span<QM31>; 4] =
        (*interaction_mask_values
        .multi_pop_front()
        .unwrap())
        .unbox();

    let [trace_2_column_1_offset_neg_1, trace_2_column_1_offset_0]: [QM31; 2] = (*trace_2_column_1
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_2_offset_neg_1, trace_2_column_2_offset_0]: [QM31; 2] = (*trace_2_column_2
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_3_offset_neg_1, trace_2_column_3_offset_0]: [QM31; 2] = (*trace_2_column_3
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_4_offset_neg_1, trace_2_column_4_offset_0]: [QM31; 2] = (*trace_2_column_4
        .try_into()
        .unwrap())
        .unbox();

    core::internal::revoke_ap_tracking();

    let mut intermediates = intermediates(
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
        poseidon_round_keys_0,
        poseidon_round_keys_1,
        poseidon_round_keys_10,
        poseidon_round_keys_11,
        poseidon_round_keys_12,
        poseidon_round_keys_13,
        poseidon_round_keys_14,
        poseidon_round_keys_15,
        poseidon_round_keys_16,
        poseidon_round_keys_17,
        poseidon_round_keys_18,
        poseidon_round_keys_19,
        poseidon_round_keys_2,
        poseidon_round_keys_20,
        poseidon_round_keys_21,
        poseidon_round_keys_22,
        poseidon_round_keys_23,
        poseidon_round_keys_24,
        poseidon_round_keys_25,
        poseidon_round_keys_26,
        poseidon_round_keys_27,
        poseidon_round_keys_28,
        poseidon_round_keys_29,
        poseidon_round_keys_3,
        poseidon_round_keys_4,
        poseidon_round_keys_5,
        poseidon_round_keys_6,
        poseidon_round_keys_7,
        poseidon_round_keys_8,
        poseidon_round_keys_9,
        seq,
    )
        .span();
    let intermediate0 = *intermediates.pop_front().unwrap();

    // Constraint 0
    let constraint_quotient = ((QM31Trait::from_partial_evals(
        [
            trace_2_column_1_offset_0, trace_2_column_2_offset_0, trace_2_column_3_offset_0,
            trace_2_column_4_offset_0,
        ],
    )
        - (QM31Trait::from_partial_evals(
            [
                trace_2_column_1_offset_neg_1, trace_2_column_2_offset_neg_1,
                trace_2_column_3_offset_neg_1, trace_2_column_4_offset_neg_1,
            ],
        ))
        + (claimed_sum) * (column_size.inverse().into()))
        * (intermediate0)
        + trace_1_column_0_offset_0)
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;
}


fn intermediates(
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
    poseidon_round_keys_0: QM31,
    poseidon_round_keys_1: QM31,
    poseidon_round_keys_10: QM31,
    poseidon_round_keys_11: QM31,
    poseidon_round_keys_12: QM31,
    poseidon_round_keys_13: QM31,
    poseidon_round_keys_14: QM31,
    poseidon_round_keys_15: QM31,
    poseidon_round_keys_16: QM31,
    poseidon_round_keys_17: QM31,
    poseidon_round_keys_18: QM31,
    poseidon_round_keys_19: QM31,
    poseidon_round_keys_2: QM31,
    poseidon_round_keys_20: QM31,
    poseidon_round_keys_21: QM31,
    poseidon_round_keys_22: QM31,
    poseidon_round_keys_23: QM31,
    poseidon_round_keys_24: QM31,
    poseidon_round_keys_25: QM31,
    poseidon_round_keys_26: QM31,
    poseidon_round_keys_27: QM31,
    poseidon_round_keys_28: QM31,
    poseidon_round_keys_29: QM31,
    poseidon_round_keys_3: QM31,
    poseidon_round_keys_4: QM31,
    poseidon_round_keys_5: QM31,
    poseidon_round_keys_6: QM31,
    poseidon_round_keys_7: QM31,
    poseidon_round_keys_8: QM31,
    poseidon_round_keys_9: QM31,
    seq: QM31,
) -> Array<QM31> {
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
        poseidon_round_keys_0,
        poseidon_round_keys_1,
        poseidon_round_keys_10,
        poseidon_round_keys_11,
        poseidon_round_keys_12,
        poseidon_round_keys_13,
        poseidon_round_keys_14,
        poseidon_round_keys_15,
        poseidon_round_keys_16,
        poseidon_round_keys_17,
        poseidon_round_keys_18,
        poseidon_round_keys_19,
        poseidon_round_keys_2,
        poseidon_round_keys_20,
        poseidon_round_keys_21,
        poseidon_round_keys_22,
        poseidon_round_keys_23,
        poseidon_round_keys_24,
        poseidon_round_keys_25,
        poseidon_round_keys_26,
        poseidon_round_keys_27,
        poseidon_round_keys_28,
        poseidon_round_keys_29,
        poseidon_round_keys_3,
        poseidon_round_keys_4,
        poseidon_round_keys_5,
        poseidon_round_keys_6,
        poseidon_round_keys_7,
        poseidon_round_keys_8,
        poseidon_round_keys_9,
        seq,
    );
    array![intermediate0]
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
    poseidon_round_keys_0: QM31,
    poseidon_round_keys_1: QM31,
    poseidon_round_keys_10: QM31,
    poseidon_round_keys_11: QM31,
    poseidon_round_keys_12: QM31,
    poseidon_round_keys_13: QM31,
    poseidon_round_keys_14: QM31,
    poseidon_round_keys_15: QM31,
    poseidon_round_keys_16: QM31,
    poseidon_round_keys_17: QM31,
    poseidon_round_keys_18: QM31,
    poseidon_round_keys_19: QM31,
    poseidon_round_keys_2: QM31,
    poseidon_round_keys_20: QM31,
    poseidon_round_keys_21: QM31,
    poseidon_round_keys_22: QM31,
    poseidon_round_keys_23: QM31,
    poseidon_round_keys_24: QM31,
    poseidon_round_keys_25: QM31,
    poseidon_round_keys_26: QM31,
    poseidon_round_keys_27: QM31,
    poseidon_round_keys_28: QM31,
    poseidon_round_keys_29: QM31,
    poseidon_round_keys_3: QM31,
    poseidon_round_keys_4: QM31,
    poseidon_round_keys_5: QM31,
    poseidon_round_keys_6: QM31,
    poseidon_round_keys_7: QM31,
    poseidon_round_keys_8: QM31,
    poseidon_round_keys_9: QM31,
    seq: QM31,
) -> QM31 {
    (PoseidonRoundKeys_alpha0) * (seq)
        + (PoseidonRoundKeys_alpha1) * (poseidon_round_keys_0)
        + (PoseidonRoundKeys_alpha2) * (poseidon_round_keys_1)
        + (PoseidonRoundKeys_alpha3) * (poseidon_round_keys_2)
        + (PoseidonRoundKeys_alpha4) * (poseidon_round_keys_3)
        + (PoseidonRoundKeys_alpha5) * (poseidon_round_keys_4)
        + (PoseidonRoundKeys_alpha6) * (poseidon_round_keys_5)
        + (PoseidonRoundKeys_alpha7) * (poseidon_round_keys_6)
        + (PoseidonRoundKeys_alpha8) * (poseidon_round_keys_7)
        + (PoseidonRoundKeys_alpha9) * (poseidon_round_keys_8)
        + (PoseidonRoundKeys_alpha10) * (poseidon_round_keys_9)
        + (PoseidonRoundKeys_alpha11) * (poseidon_round_keys_10)
        + (PoseidonRoundKeys_alpha12) * (poseidon_round_keys_11)
        + (PoseidonRoundKeys_alpha13) * (poseidon_round_keys_12)
        + (PoseidonRoundKeys_alpha14) * (poseidon_round_keys_13)
        + (PoseidonRoundKeys_alpha15) * (poseidon_round_keys_14)
        + (PoseidonRoundKeys_alpha16) * (poseidon_round_keys_15)
        + (PoseidonRoundKeys_alpha17) * (poseidon_round_keys_16)
        + (PoseidonRoundKeys_alpha18) * (poseidon_round_keys_17)
        + (PoseidonRoundKeys_alpha19) * (poseidon_round_keys_18)
        + (PoseidonRoundKeys_alpha20) * (poseidon_round_keys_19)
        + (PoseidonRoundKeys_alpha21) * (poseidon_round_keys_20)
        + (PoseidonRoundKeys_alpha22) * (poseidon_round_keys_21)
        + (PoseidonRoundKeys_alpha23) * (poseidon_round_keys_22)
        + (PoseidonRoundKeys_alpha24) * (poseidon_round_keys_23)
        + (PoseidonRoundKeys_alpha25) * (poseidon_round_keys_24)
        + (PoseidonRoundKeys_alpha26) * (poseidon_round_keys_25)
        + (PoseidonRoundKeys_alpha27) * (poseidon_round_keys_26)
        + (PoseidonRoundKeys_alpha28) * (poseidon_round_keys_27)
        + (PoseidonRoundKeys_alpha29) * (poseidon_round_keys_28)
        + (PoseidonRoundKeys_alpha30) * (poseidon_round_keys_29)
        - (PoseidonRoundKeys_z)
}

