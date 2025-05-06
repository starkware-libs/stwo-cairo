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
    preprocessed_column_set.insert(PreprocessedColumn::BlakeSigma(0));
    preprocessed_column_set.insert(PreprocessedColumn::BlakeSigma(1));
    preprocessed_column_set.insert(PreprocessedColumn::BlakeSigma(2));
    preprocessed_column_set.insert(PreprocessedColumn::BlakeSigma(3));
    preprocessed_column_set.insert(PreprocessedColumn::BlakeSigma(4));
    preprocessed_column_set.insert(PreprocessedColumn::BlakeSigma(5));
    preprocessed_column_set.insert(PreprocessedColumn::BlakeSigma(6));
    preprocessed_column_set.insert(PreprocessedColumn::BlakeSigma(7));
    preprocessed_column_set.insert(PreprocessedColumn::BlakeSigma(8));
    preprocessed_column_set.insert(PreprocessedColumn::BlakeSigma(9));
    preprocessed_column_set.insert(PreprocessedColumn::BlakeSigma(10));
    preprocessed_column_set.insert(PreprocessedColumn::BlakeSigma(11));
    preprocessed_column_set.insert(PreprocessedColumn::BlakeSigma(12));
    preprocessed_column_set.insert(PreprocessedColumn::BlakeSigma(13));
    preprocessed_column_set.insert(PreprocessedColumn::BlakeSigma(14));
    preprocessed_column_set.insert(PreprocessedColumn::BlakeSigma(15));
    let point_offset_neg_1 = point.add_circle_point_m31(-trace_gen.mul(1).to_point());
    trace_mask_points.append(array![point]);
    interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
    interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
    interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
    interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
}

#[derive(Drop)]
pub struct ConstraintParams {
    pub BlakeRoundSigma_alpha0: QM31,
    pub BlakeRoundSigma_alpha1: QM31,
    pub BlakeRoundSigma_alpha10: QM31,
    pub BlakeRoundSigma_alpha11: QM31,
    pub BlakeRoundSigma_alpha12: QM31,
    pub BlakeRoundSigma_alpha13: QM31,
    pub BlakeRoundSigma_alpha14: QM31,
    pub BlakeRoundSigma_alpha15: QM31,
    pub BlakeRoundSigma_alpha16: QM31,
    pub BlakeRoundSigma_alpha2: QM31,
    pub BlakeRoundSigma_alpha3: QM31,
    pub BlakeRoundSigma_alpha4: QM31,
    pub BlakeRoundSigma_alpha5: QM31,
    pub BlakeRoundSigma_alpha6: QM31,
    pub BlakeRoundSigma_alpha7: QM31,
    pub BlakeRoundSigma_alpha8: QM31,
    pub BlakeRoundSigma_alpha9: QM31,
    pub BlakeRoundSigma_z: QM31,
    pub blake_sigma_0: QM31,
    pub blake_sigma_1: QM31,
    pub blake_sigma_10: QM31,
    pub blake_sigma_11: QM31,
    pub blake_sigma_12: QM31,
    pub blake_sigma_13: QM31,
    pub blake_sigma_14: QM31,
    pub blake_sigma_15: QM31,
    pub blake_sigma_2: QM31,
    pub blake_sigma_3: QM31,
    pub blake_sigma_4: QM31,
    pub blake_sigma_5: QM31,
    pub blake_sigma_6: QM31,
    pub blake_sigma_7: QM31,
    pub blake_sigma_8: QM31,
    pub blake_sigma_9: QM31,
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
        BlakeRoundSigma_alpha0,
        BlakeRoundSigma_alpha1,
        BlakeRoundSigma_alpha10,
        BlakeRoundSigma_alpha11,
        BlakeRoundSigma_alpha12,
        BlakeRoundSigma_alpha13,
        BlakeRoundSigma_alpha14,
        BlakeRoundSigma_alpha15,
        BlakeRoundSigma_alpha16,
        BlakeRoundSigma_alpha2,
        BlakeRoundSigma_alpha3,
        BlakeRoundSigma_alpha4,
        BlakeRoundSigma_alpha5,
        BlakeRoundSigma_alpha6,
        BlakeRoundSigma_alpha7,
        BlakeRoundSigma_alpha8,
        BlakeRoundSigma_alpha9,
        BlakeRoundSigma_z,
        blake_sigma_0,
        blake_sigma_1,
        blake_sigma_10,
        blake_sigma_11,
        blake_sigma_12,
        blake_sigma_13,
        blake_sigma_14,
        blake_sigma_15,
        blake_sigma_2,
        blake_sigma_3,
        blake_sigma_4,
        blake_sigma_5,
        blake_sigma_6,
        blake_sigma_7,
        blake_sigma_8,
        blake_sigma_9,
        claimed_sum,
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
        BlakeRoundSigma_alpha0,
        BlakeRoundSigma_alpha1,
        BlakeRoundSigma_alpha10,
        BlakeRoundSigma_alpha11,
        BlakeRoundSigma_alpha12,
        BlakeRoundSigma_alpha13,
        BlakeRoundSigma_alpha14,
        BlakeRoundSigma_alpha15,
        BlakeRoundSigma_alpha16,
        BlakeRoundSigma_alpha2,
        BlakeRoundSigma_alpha3,
        BlakeRoundSigma_alpha4,
        BlakeRoundSigma_alpha5,
        BlakeRoundSigma_alpha6,
        BlakeRoundSigma_alpha7,
        BlakeRoundSigma_alpha8,
        BlakeRoundSigma_alpha9,
        BlakeRoundSigma_z,
        blake_sigma_0,
        blake_sigma_1,
        blake_sigma_10,
        blake_sigma_11,
        blake_sigma_12,
        blake_sigma_13,
        blake_sigma_14,
        blake_sigma_15,
        blake_sigma_2,
        blake_sigma_3,
        blake_sigma_4,
        blake_sigma_5,
        blake_sigma_6,
        blake_sigma_7,
        blake_sigma_8,
        blake_sigma_9,
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
    BlakeRoundSigma_alpha0: QM31,
    BlakeRoundSigma_alpha1: QM31,
    BlakeRoundSigma_alpha10: QM31,
    BlakeRoundSigma_alpha11: QM31,
    BlakeRoundSigma_alpha12: QM31,
    BlakeRoundSigma_alpha13: QM31,
    BlakeRoundSigma_alpha14: QM31,
    BlakeRoundSigma_alpha15: QM31,
    BlakeRoundSigma_alpha16: QM31,
    BlakeRoundSigma_alpha2: QM31,
    BlakeRoundSigma_alpha3: QM31,
    BlakeRoundSigma_alpha4: QM31,
    BlakeRoundSigma_alpha5: QM31,
    BlakeRoundSigma_alpha6: QM31,
    BlakeRoundSigma_alpha7: QM31,
    BlakeRoundSigma_alpha8: QM31,
    BlakeRoundSigma_alpha9: QM31,
    BlakeRoundSigma_z: QM31,
    blake_sigma_0: QM31,
    blake_sigma_1: QM31,
    blake_sigma_10: QM31,
    blake_sigma_11: QM31,
    blake_sigma_12: QM31,
    blake_sigma_13: QM31,
    blake_sigma_14: QM31,
    blake_sigma_15: QM31,
    blake_sigma_2: QM31,
    blake_sigma_3: QM31,
    blake_sigma_4: QM31,
    blake_sigma_5: QM31,
    blake_sigma_6: QM31,
    blake_sigma_7: QM31,
    blake_sigma_8: QM31,
    blake_sigma_9: QM31,
    seq: QM31,
) -> Array<QM31> {
    let intermediate0 = intermediate0(
        BlakeRoundSigma_alpha0,
        BlakeRoundSigma_alpha1,
        BlakeRoundSigma_alpha10,
        BlakeRoundSigma_alpha11,
        BlakeRoundSigma_alpha12,
        BlakeRoundSigma_alpha13,
        BlakeRoundSigma_alpha14,
        BlakeRoundSigma_alpha15,
        BlakeRoundSigma_alpha16,
        BlakeRoundSigma_alpha2,
        BlakeRoundSigma_alpha3,
        BlakeRoundSigma_alpha4,
        BlakeRoundSigma_alpha5,
        BlakeRoundSigma_alpha6,
        BlakeRoundSigma_alpha7,
        BlakeRoundSigma_alpha8,
        BlakeRoundSigma_alpha9,
        BlakeRoundSigma_z,
        blake_sigma_0,
        blake_sigma_1,
        blake_sigma_10,
        blake_sigma_11,
        blake_sigma_12,
        blake_sigma_13,
        blake_sigma_14,
        blake_sigma_15,
        blake_sigma_2,
        blake_sigma_3,
        blake_sigma_4,
        blake_sigma_5,
        blake_sigma_6,
        blake_sigma_7,
        blake_sigma_8,
        blake_sigma_9,
        seq,
    );
    array![intermediate0]
}


pub fn intermediate0(
    BlakeRoundSigma_alpha0: QM31,
    BlakeRoundSigma_alpha1: QM31,
    BlakeRoundSigma_alpha10: QM31,
    BlakeRoundSigma_alpha11: QM31,
    BlakeRoundSigma_alpha12: QM31,
    BlakeRoundSigma_alpha13: QM31,
    BlakeRoundSigma_alpha14: QM31,
    BlakeRoundSigma_alpha15: QM31,
    BlakeRoundSigma_alpha16: QM31,
    BlakeRoundSigma_alpha2: QM31,
    BlakeRoundSigma_alpha3: QM31,
    BlakeRoundSigma_alpha4: QM31,
    BlakeRoundSigma_alpha5: QM31,
    BlakeRoundSigma_alpha6: QM31,
    BlakeRoundSigma_alpha7: QM31,
    BlakeRoundSigma_alpha8: QM31,
    BlakeRoundSigma_alpha9: QM31,
    BlakeRoundSigma_z: QM31,
    blake_sigma_0: QM31,
    blake_sigma_1: QM31,
    blake_sigma_10: QM31,
    blake_sigma_11: QM31,
    blake_sigma_12: QM31,
    blake_sigma_13: QM31,
    blake_sigma_14: QM31,
    blake_sigma_15: QM31,
    blake_sigma_2: QM31,
    blake_sigma_3: QM31,
    blake_sigma_4: QM31,
    blake_sigma_5: QM31,
    blake_sigma_6: QM31,
    blake_sigma_7: QM31,
    blake_sigma_8: QM31,
    blake_sigma_9: QM31,
    seq: QM31,
) -> QM31 {
    (BlakeRoundSigma_alpha0) * (seq)
        + (BlakeRoundSigma_alpha1) * (blake_sigma_0)
        + (BlakeRoundSigma_alpha2) * (blake_sigma_1)
        + (BlakeRoundSigma_alpha3) * (blake_sigma_2)
        + (BlakeRoundSigma_alpha4) * (blake_sigma_3)
        + (BlakeRoundSigma_alpha5) * (blake_sigma_4)
        + (BlakeRoundSigma_alpha6) * (blake_sigma_5)
        + (BlakeRoundSigma_alpha7) * (blake_sigma_6)
        + (BlakeRoundSigma_alpha8) * (blake_sigma_7)
        + (BlakeRoundSigma_alpha9) * (blake_sigma_8)
        + (BlakeRoundSigma_alpha10) * (blake_sigma_9)
        + (BlakeRoundSigma_alpha11) * (blake_sigma_10)
        + (BlakeRoundSigma_alpha12) * (blake_sigma_11)
        + (BlakeRoundSigma_alpha13) * (blake_sigma_12)
        + (BlakeRoundSigma_alpha14) * (blake_sigma_13)
        + (BlakeRoundSigma_alpha15) * (blake_sigma_14)
        + (BlakeRoundSigma_alpha16) * (blake_sigma_15)
        - (BlakeRoundSigma_z)
}

