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
    preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints(0));
    preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints(1));
    preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints(2));
    preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints(3));
    preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints(4));
    preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints(5));
    preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints(6));
    preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints(7));
    preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints(8));
    preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints(9));
    preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints(10));
    preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints(11));
    preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints(12));
    preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints(13));
    preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints(14));
    preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints(15));
    preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints(16));
    preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints(17));
    preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints(18));
    preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints(19));
    preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints(20));
    preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints(21));
    preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints(22));
    preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints(23));
    preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints(24));
    preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints(25));
    preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints(26));
    preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints(27));
    preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints(28));
    preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints(29));
    preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints(30));
    preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints(31));
    preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints(32));
    preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints(33));
    preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints(34));
    preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints(35));
    preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints(36));
    preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints(37));
    preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints(38));
    preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints(39));
    preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints(40));
    preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints(41));
    preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints(42));
    preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints(43));
    preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints(44));
    preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints(45));
    preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints(46));
    preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints(47));
    preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints(48));
    preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints(49));
    preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints(50));
    preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints(51));
    preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints(52));
    preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints(53));
    preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints(54));
    preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints(55));
    let point_offset_neg_1 = point.add_circle_point_m31(-trace_gen.mul(1).to_point());
    trace_mask_points.append(array![point]);
    interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
    interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
    interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
    interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
}

#[derive(Drop)]
pub struct ConstraintParams {
    pub PedersenPointsTable_alpha0: QM31,
    pub PedersenPointsTable_alpha1: QM31,
    pub PedersenPointsTable_alpha10: QM31,
    pub PedersenPointsTable_alpha11: QM31,
    pub PedersenPointsTable_alpha12: QM31,
    pub PedersenPointsTable_alpha13: QM31,
    pub PedersenPointsTable_alpha14: QM31,
    pub PedersenPointsTable_alpha15: QM31,
    pub PedersenPointsTable_alpha16: QM31,
    pub PedersenPointsTable_alpha17: QM31,
    pub PedersenPointsTable_alpha18: QM31,
    pub PedersenPointsTable_alpha19: QM31,
    pub PedersenPointsTable_alpha2: QM31,
    pub PedersenPointsTable_alpha20: QM31,
    pub PedersenPointsTable_alpha21: QM31,
    pub PedersenPointsTable_alpha22: QM31,
    pub PedersenPointsTable_alpha23: QM31,
    pub PedersenPointsTable_alpha24: QM31,
    pub PedersenPointsTable_alpha25: QM31,
    pub PedersenPointsTable_alpha26: QM31,
    pub PedersenPointsTable_alpha27: QM31,
    pub PedersenPointsTable_alpha28: QM31,
    pub PedersenPointsTable_alpha29: QM31,
    pub PedersenPointsTable_alpha3: QM31,
    pub PedersenPointsTable_alpha30: QM31,
    pub PedersenPointsTable_alpha31: QM31,
    pub PedersenPointsTable_alpha32: QM31,
    pub PedersenPointsTable_alpha33: QM31,
    pub PedersenPointsTable_alpha34: QM31,
    pub PedersenPointsTable_alpha35: QM31,
    pub PedersenPointsTable_alpha36: QM31,
    pub PedersenPointsTable_alpha37: QM31,
    pub PedersenPointsTable_alpha38: QM31,
    pub PedersenPointsTable_alpha39: QM31,
    pub PedersenPointsTable_alpha4: QM31,
    pub PedersenPointsTable_alpha40: QM31,
    pub PedersenPointsTable_alpha41: QM31,
    pub PedersenPointsTable_alpha42: QM31,
    pub PedersenPointsTable_alpha43: QM31,
    pub PedersenPointsTable_alpha44: QM31,
    pub PedersenPointsTable_alpha45: QM31,
    pub PedersenPointsTable_alpha46: QM31,
    pub PedersenPointsTable_alpha47: QM31,
    pub PedersenPointsTable_alpha48: QM31,
    pub PedersenPointsTable_alpha49: QM31,
    pub PedersenPointsTable_alpha5: QM31,
    pub PedersenPointsTable_alpha50: QM31,
    pub PedersenPointsTable_alpha51: QM31,
    pub PedersenPointsTable_alpha52: QM31,
    pub PedersenPointsTable_alpha53: QM31,
    pub PedersenPointsTable_alpha54: QM31,
    pub PedersenPointsTable_alpha55: QM31,
    pub PedersenPointsTable_alpha56: QM31,
    pub PedersenPointsTable_alpha6: QM31,
    pub PedersenPointsTable_alpha7: QM31,
    pub PedersenPointsTable_alpha8: QM31,
    pub PedersenPointsTable_alpha9: QM31,
    pub PedersenPointsTable_z: QM31,
    pub claimed_sum: QM31,
    pub pedersen_points_0: QM31,
    pub pedersen_points_1: QM31,
    pub pedersen_points_10: QM31,
    pub pedersen_points_11: QM31,
    pub pedersen_points_12: QM31,
    pub pedersen_points_13: QM31,
    pub pedersen_points_14: QM31,
    pub pedersen_points_15: QM31,
    pub pedersen_points_16: QM31,
    pub pedersen_points_17: QM31,
    pub pedersen_points_18: QM31,
    pub pedersen_points_19: QM31,
    pub pedersen_points_2: QM31,
    pub pedersen_points_20: QM31,
    pub pedersen_points_21: QM31,
    pub pedersen_points_22: QM31,
    pub pedersen_points_23: QM31,
    pub pedersen_points_24: QM31,
    pub pedersen_points_25: QM31,
    pub pedersen_points_26: QM31,
    pub pedersen_points_27: QM31,
    pub pedersen_points_28: QM31,
    pub pedersen_points_29: QM31,
    pub pedersen_points_3: QM31,
    pub pedersen_points_30: QM31,
    pub pedersen_points_31: QM31,
    pub pedersen_points_32: QM31,
    pub pedersen_points_33: QM31,
    pub pedersen_points_34: QM31,
    pub pedersen_points_35: QM31,
    pub pedersen_points_36: QM31,
    pub pedersen_points_37: QM31,
    pub pedersen_points_38: QM31,
    pub pedersen_points_39: QM31,
    pub pedersen_points_4: QM31,
    pub pedersen_points_40: QM31,
    pub pedersen_points_41: QM31,
    pub pedersen_points_42: QM31,
    pub pedersen_points_43: QM31,
    pub pedersen_points_44: QM31,
    pub pedersen_points_45: QM31,
    pub pedersen_points_46: QM31,
    pub pedersen_points_47: QM31,
    pub pedersen_points_48: QM31,
    pub pedersen_points_49: QM31,
    pub pedersen_points_5: QM31,
    pub pedersen_points_50: QM31,
    pub pedersen_points_51: QM31,
    pub pedersen_points_52: QM31,
    pub pedersen_points_53: QM31,
    pub pedersen_points_54: QM31,
    pub pedersen_points_55: QM31,
    pub pedersen_points_6: QM31,
    pub pedersen_points_7: QM31,
    pub pedersen_points_8: QM31,
    pub pedersen_points_9: QM31,
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
        PedersenPointsTable_alpha0,
        PedersenPointsTable_alpha1,
        PedersenPointsTable_alpha10,
        PedersenPointsTable_alpha11,
        PedersenPointsTable_alpha12,
        PedersenPointsTable_alpha13,
        PedersenPointsTable_alpha14,
        PedersenPointsTable_alpha15,
        PedersenPointsTable_alpha16,
        PedersenPointsTable_alpha17,
        PedersenPointsTable_alpha18,
        PedersenPointsTable_alpha19,
        PedersenPointsTable_alpha2,
        PedersenPointsTable_alpha20,
        PedersenPointsTable_alpha21,
        PedersenPointsTable_alpha22,
        PedersenPointsTable_alpha23,
        PedersenPointsTable_alpha24,
        PedersenPointsTable_alpha25,
        PedersenPointsTable_alpha26,
        PedersenPointsTable_alpha27,
        PedersenPointsTable_alpha28,
        PedersenPointsTable_alpha29,
        PedersenPointsTable_alpha3,
        PedersenPointsTable_alpha30,
        PedersenPointsTable_alpha31,
        PedersenPointsTable_alpha32,
        PedersenPointsTable_alpha33,
        PedersenPointsTable_alpha34,
        PedersenPointsTable_alpha35,
        PedersenPointsTable_alpha36,
        PedersenPointsTable_alpha37,
        PedersenPointsTable_alpha38,
        PedersenPointsTable_alpha39,
        PedersenPointsTable_alpha4,
        PedersenPointsTable_alpha40,
        PedersenPointsTable_alpha41,
        PedersenPointsTable_alpha42,
        PedersenPointsTable_alpha43,
        PedersenPointsTable_alpha44,
        PedersenPointsTable_alpha45,
        PedersenPointsTable_alpha46,
        PedersenPointsTable_alpha47,
        PedersenPointsTable_alpha48,
        PedersenPointsTable_alpha49,
        PedersenPointsTable_alpha5,
        PedersenPointsTable_alpha50,
        PedersenPointsTable_alpha51,
        PedersenPointsTable_alpha52,
        PedersenPointsTable_alpha53,
        PedersenPointsTable_alpha54,
        PedersenPointsTable_alpha55,
        PedersenPointsTable_alpha56,
        PedersenPointsTable_alpha6,
        PedersenPointsTable_alpha7,
        PedersenPointsTable_alpha8,
        PedersenPointsTable_alpha9,
        PedersenPointsTable_z,
        claimed_sum,
        column_size,
        pedersen_points_0,
        pedersen_points_1,
        pedersen_points_10,
        pedersen_points_11,
        pedersen_points_12,
        pedersen_points_13,
        pedersen_points_14,
        pedersen_points_15,
        pedersen_points_16,
        pedersen_points_17,
        pedersen_points_18,
        pedersen_points_19,
        pedersen_points_2,
        pedersen_points_20,
        pedersen_points_21,
        pedersen_points_22,
        pedersen_points_23,
        pedersen_points_24,
        pedersen_points_25,
        pedersen_points_26,
        pedersen_points_27,
        pedersen_points_28,
        pedersen_points_29,
        pedersen_points_3,
        pedersen_points_30,
        pedersen_points_31,
        pedersen_points_32,
        pedersen_points_33,
        pedersen_points_34,
        pedersen_points_35,
        pedersen_points_36,
        pedersen_points_37,
        pedersen_points_38,
        pedersen_points_39,
        pedersen_points_4,
        pedersen_points_40,
        pedersen_points_41,
        pedersen_points_42,
        pedersen_points_43,
        pedersen_points_44,
        pedersen_points_45,
        pedersen_points_46,
        pedersen_points_47,
        pedersen_points_48,
        pedersen_points_49,
        pedersen_points_5,
        pedersen_points_50,
        pedersen_points_51,
        pedersen_points_52,
        pedersen_points_53,
        pedersen_points_54,
        pedersen_points_55,
        pedersen_points_6,
        pedersen_points_7,
        pedersen_points_8,
        pedersen_points_9,
        seq,
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
        PedersenPointsTable_alpha0,
        PedersenPointsTable_alpha1,
        PedersenPointsTable_alpha10,
        PedersenPointsTable_alpha11,
        PedersenPointsTable_alpha12,
        PedersenPointsTable_alpha13,
        PedersenPointsTable_alpha14,
        PedersenPointsTable_alpha15,
        PedersenPointsTable_alpha16,
        PedersenPointsTable_alpha17,
        PedersenPointsTable_alpha18,
        PedersenPointsTable_alpha19,
        PedersenPointsTable_alpha2,
        PedersenPointsTable_alpha20,
        PedersenPointsTable_alpha21,
        PedersenPointsTable_alpha22,
        PedersenPointsTable_alpha23,
        PedersenPointsTable_alpha24,
        PedersenPointsTable_alpha25,
        PedersenPointsTable_alpha26,
        PedersenPointsTable_alpha27,
        PedersenPointsTable_alpha28,
        PedersenPointsTable_alpha29,
        PedersenPointsTable_alpha3,
        PedersenPointsTable_alpha30,
        PedersenPointsTable_alpha31,
        PedersenPointsTable_alpha32,
        PedersenPointsTable_alpha33,
        PedersenPointsTable_alpha34,
        PedersenPointsTable_alpha35,
        PedersenPointsTable_alpha36,
        PedersenPointsTable_alpha37,
        PedersenPointsTable_alpha38,
        PedersenPointsTable_alpha39,
        PedersenPointsTable_alpha4,
        PedersenPointsTable_alpha40,
        PedersenPointsTable_alpha41,
        PedersenPointsTable_alpha42,
        PedersenPointsTable_alpha43,
        PedersenPointsTable_alpha44,
        PedersenPointsTable_alpha45,
        PedersenPointsTable_alpha46,
        PedersenPointsTable_alpha47,
        PedersenPointsTable_alpha48,
        PedersenPointsTable_alpha49,
        PedersenPointsTable_alpha5,
        PedersenPointsTable_alpha50,
        PedersenPointsTable_alpha51,
        PedersenPointsTable_alpha52,
        PedersenPointsTable_alpha53,
        PedersenPointsTable_alpha54,
        PedersenPointsTable_alpha55,
        PedersenPointsTable_alpha56,
        PedersenPointsTable_alpha6,
        PedersenPointsTable_alpha7,
        PedersenPointsTable_alpha8,
        PedersenPointsTable_alpha9,
        PedersenPointsTable_z,
        pedersen_points_0,
        pedersen_points_1,
        pedersen_points_10,
        pedersen_points_11,
        pedersen_points_12,
        pedersen_points_13,
        pedersen_points_14,
        pedersen_points_15,
        pedersen_points_16,
        pedersen_points_17,
        pedersen_points_18,
        pedersen_points_19,
        pedersen_points_2,
        pedersen_points_20,
        pedersen_points_21,
        pedersen_points_22,
        pedersen_points_23,
        pedersen_points_24,
        pedersen_points_25,
        pedersen_points_26,
        pedersen_points_27,
        pedersen_points_28,
        pedersen_points_29,
        pedersen_points_3,
        pedersen_points_30,
        pedersen_points_31,
        pedersen_points_32,
        pedersen_points_33,
        pedersen_points_34,
        pedersen_points_35,
        pedersen_points_36,
        pedersen_points_37,
        pedersen_points_38,
        pedersen_points_39,
        pedersen_points_4,
        pedersen_points_40,
        pedersen_points_41,
        pedersen_points_42,
        pedersen_points_43,
        pedersen_points_44,
        pedersen_points_45,
        pedersen_points_46,
        pedersen_points_47,
        pedersen_points_48,
        pedersen_points_49,
        pedersen_points_5,
        pedersen_points_50,
        pedersen_points_51,
        pedersen_points_52,
        pedersen_points_53,
        pedersen_points_54,
        pedersen_points_55,
        pedersen_points_6,
        pedersen_points_7,
        pedersen_points_8,
        pedersen_points_9,
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
    PedersenPointsTable_alpha0: QM31,
    PedersenPointsTable_alpha1: QM31,
    PedersenPointsTable_alpha10: QM31,
    PedersenPointsTable_alpha11: QM31,
    PedersenPointsTable_alpha12: QM31,
    PedersenPointsTable_alpha13: QM31,
    PedersenPointsTable_alpha14: QM31,
    PedersenPointsTable_alpha15: QM31,
    PedersenPointsTable_alpha16: QM31,
    PedersenPointsTable_alpha17: QM31,
    PedersenPointsTable_alpha18: QM31,
    PedersenPointsTable_alpha19: QM31,
    PedersenPointsTable_alpha2: QM31,
    PedersenPointsTable_alpha20: QM31,
    PedersenPointsTable_alpha21: QM31,
    PedersenPointsTable_alpha22: QM31,
    PedersenPointsTable_alpha23: QM31,
    PedersenPointsTable_alpha24: QM31,
    PedersenPointsTable_alpha25: QM31,
    PedersenPointsTable_alpha26: QM31,
    PedersenPointsTable_alpha27: QM31,
    PedersenPointsTable_alpha28: QM31,
    PedersenPointsTable_alpha29: QM31,
    PedersenPointsTable_alpha3: QM31,
    PedersenPointsTable_alpha30: QM31,
    PedersenPointsTable_alpha31: QM31,
    PedersenPointsTable_alpha32: QM31,
    PedersenPointsTable_alpha33: QM31,
    PedersenPointsTable_alpha34: QM31,
    PedersenPointsTable_alpha35: QM31,
    PedersenPointsTable_alpha36: QM31,
    PedersenPointsTable_alpha37: QM31,
    PedersenPointsTable_alpha38: QM31,
    PedersenPointsTable_alpha39: QM31,
    PedersenPointsTable_alpha4: QM31,
    PedersenPointsTable_alpha40: QM31,
    PedersenPointsTable_alpha41: QM31,
    PedersenPointsTable_alpha42: QM31,
    PedersenPointsTable_alpha43: QM31,
    PedersenPointsTable_alpha44: QM31,
    PedersenPointsTable_alpha45: QM31,
    PedersenPointsTable_alpha46: QM31,
    PedersenPointsTable_alpha47: QM31,
    PedersenPointsTable_alpha48: QM31,
    PedersenPointsTable_alpha49: QM31,
    PedersenPointsTable_alpha5: QM31,
    PedersenPointsTable_alpha50: QM31,
    PedersenPointsTable_alpha51: QM31,
    PedersenPointsTable_alpha52: QM31,
    PedersenPointsTable_alpha53: QM31,
    PedersenPointsTable_alpha54: QM31,
    PedersenPointsTable_alpha55: QM31,
    PedersenPointsTable_alpha56: QM31,
    PedersenPointsTable_alpha6: QM31,
    PedersenPointsTable_alpha7: QM31,
    PedersenPointsTable_alpha8: QM31,
    PedersenPointsTable_alpha9: QM31,
    PedersenPointsTable_z: QM31,
    pedersen_points_0: QM31,
    pedersen_points_1: QM31,
    pedersen_points_10: QM31,
    pedersen_points_11: QM31,
    pedersen_points_12: QM31,
    pedersen_points_13: QM31,
    pedersen_points_14: QM31,
    pedersen_points_15: QM31,
    pedersen_points_16: QM31,
    pedersen_points_17: QM31,
    pedersen_points_18: QM31,
    pedersen_points_19: QM31,
    pedersen_points_2: QM31,
    pedersen_points_20: QM31,
    pedersen_points_21: QM31,
    pedersen_points_22: QM31,
    pedersen_points_23: QM31,
    pedersen_points_24: QM31,
    pedersen_points_25: QM31,
    pedersen_points_26: QM31,
    pedersen_points_27: QM31,
    pedersen_points_28: QM31,
    pedersen_points_29: QM31,
    pedersen_points_3: QM31,
    pedersen_points_30: QM31,
    pedersen_points_31: QM31,
    pedersen_points_32: QM31,
    pedersen_points_33: QM31,
    pedersen_points_34: QM31,
    pedersen_points_35: QM31,
    pedersen_points_36: QM31,
    pedersen_points_37: QM31,
    pedersen_points_38: QM31,
    pedersen_points_39: QM31,
    pedersen_points_4: QM31,
    pedersen_points_40: QM31,
    pedersen_points_41: QM31,
    pedersen_points_42: QM31,
    pedersen_points_43: QM31,
    pedersen_points_44: QM31,
    pedersen_points_45: QM31,
    pedersen_points_46: QM31,
    pedersen_points_47: QM31,
    pedersen_points_48: QM31,
    pedersen_points_49: QM31,
    pedersen_points_5: QM31,
    pedersen_points_50: QM31,
    pedersen_points_51: QM31,
    pedersen_points_52: QM31,
    pedersen_points_53: QM31,
    pedersen_points_54: QM31,
    pedersen_points_55: QM31,
    pedersen_points_6: QM31,
    pedersen_points_7: QM31,
    pedersen_points_8: QM31,
    pedersen_points_9: QM31,
    seq: QM31,
) -> Array<QM31> {
    let intermediate0 = intermediate0(
        PedersenPointsTable_alpha0,
        PedersenPointsTable_alpha1,
        PedersenPointsTable_alpha10,
        PedersenPointsTable_alpha11,
        PedersenPointsTable_alpha12,
        PedersenPointsTable_alpha13,
        PedersenPointsTable_alpha14,
        PedersenPointsTable_alpha15,
        PedersenPointsTable_alpha16,
        PedersenPointsTable_alpha17,
        PedersenPointsTable_alpha18,
        PedersenPointsTable_alpha19,
        PedersenPointsTable_alpha2,
        PedersenPointsTable_alpha20,
        PedersenPointsTable_alpha21,
        PedersenPointsTable_alpha22,
        PedersenPointsTable_alpha23,
        PedersenPointsTable_alpha24,
        PedersenPointsTable_alpha25,
        PedersenPointsTable_alpha26,
        PedersenPointsTable_alpha27,
        PedersenPointsTable_alpha28,
        PedersenPointsTable_alpha29,
        PedersenPointsTable_alpha3,
        PedersenPointsTable_alpha30,
        PedersenPointsTable_alpha31,
        PedersenPointsTable_alpha32,
        PedersenPointsTable_alpha33,
        PedersenPointsTable_alpha34,
        PedersenPointsTable_alpha35,
        PedersenPointsTable_alpha36,
        PedersenPointsTable_alpha37,
        PedersenPointsTable_alpha38,
        PedersenPointsTable_alpha39,
        PedersenPointsTable_alpha4,
        PedersenPointsTable_alpha40,
        PedersenPointsTable_alpha41,
        PedersenPointsTable_alpha42,
        PedersenPointsTable_alpha43,
        PedersenPointsTable_alpha44,
        PedersenPointsTable_alpha45,
        PedersenPointsTable_alpha46,
        PedersenPointsTable_alpha47,
        PedersenPointsTable_alpha48,
        PedersenPointsTable_alpha49,
        PedersenPointsTable_alpha5,
        PedersenPointsTable_alpha50,
        PedersenPointsTable_alpha51,
        PedersenPointsTable_alpha52,
        PedersenPointsTable_alpha53,
        PedersenPointsTable_alpha54,
        PedersenPointsTable_alpha55,
        PedersenPointsTable_alpha56,
        PedersenPointsTable_alpha6,
        PedersenPointsTable_alpha7,
        PedersenPointsTable_alpha8,
        PedersenPointsTable_alpha9,
        PedersenPointsTable_z,
        pedersen_points_0,
        pedersen_points_1,
        pedersen_points_10,
        pedersen_points_11,
        pedersen_points_12,
        pedersen_points_13,
        pedersen_points_14,
        pedersen_points_15,
        pedersen_points_16,
        pedersen_points_17,
        pedersen_points_18,
        pedersen_points_19,
        pedersen_points_2,
        pedersen_points_20,
        pedersen_points_21,
        pedersen_points_22,
        pedersen_points_23,
        pedersen_points_24,
        pedersen_points_25,
        pedersen_points_26,
        pedersen_points_27,
        pedersen_points_28,
        pedersen_points_29,
        pedersen_points_3,
        pedersen_points_30,
        pedersen_points_31,
        pedersen_points_32,
        pedersen_points_33,
        pedersen_points_34,
        pedersen_points_35,
        pedersen_points_36,
        pedersen_points_37,
        pedersen_points_38,
        pedersen_points_39,
        pedersen_points_4,
        pedersen_points_40,
        pedersen_points_41,
        pedersen_points_42,
        pedersen_points_43,
        pedersen_points_44,
        pedersen_points_45,
        pedersen_points_46,
        pedersen_points_47,
        pedersen_points_48,
        pedersen_points_49,
        pedersen_points_5,
        pedersen_points_50,
        pedersen_points_51,
        pedersen_points_52,
        pedersen_points_53,
        pedersen_points_54,
        pedersen_points_55,
        pedersen_points_6,
        pedersen_points_7,
        pedersen_points_8,
        pedersen_points_9,
        seq,
    );
    array![intermediate0]
}


pub fn intermediate0(
    PedersenPointsTable_alpha0: QM31,
    PedersenPointsTable_alpha1: QM31,
    PedersenPointsTable_alpha10: QM31,
    PedersenPointsTable_alpha11: QM31,
    PedersenPointsTable_alpha12: QM31,
    PedersenPointsTable_alpha13: QM31,
    PedersenPointsTable_alpha14: QM31,
    PedersenPointsTable_alpha15: QM31,
    PedersenPointsTable_alpha16: QM31,
    PedersenPointsTable_alpha17: QM31,
    PedersenPointsTable_alpha18: QM31,
    PedersenPointsTable_alpha19: QM31,
    PedersenPointsTable_alpha2: QM31,
    PedersenPointsTable_alpha20: QM31,
    PedersenPointsTable_alpha21: QM31,
    PedersenPointsTable_alpha22: QM31,
    PedersenPointsTable_alpha23: QM31,
    PedersenPointsTable_alpha24: QM31,
    PedersenPointsTable_alpha25: QM31,
    PedersenPointsTable_alpha26: QM31,
    PedersenPointsTable_alpha27: QM31,
    PedersenPointsTable_alpha28: QM31,
    PedersenPointsTable_alpha29: QM31,
    PedersenPointsTable_alpha3: QM31,
    PedersenPointsTable_alpha30: QM31,
    PedersenPointsTable_alpha31: QM31,
    PedersenPointsTable_alpha32: QM31,
    PedersenPointsTable_alpha33: QM31,
    PedersenPointsTable_alpha34: QM31,
    PedersenPointsTable_alpha35: QM31,
    PedersenPointsTable_alpha36: QM31,
    PedersenPointsTable_alpha37: QM31,
    PedersenPointsTable_alpha38: QM31,
    PedersenPointsTable_alpha39: QM31,
    PedersenPointsTable_alpha4: QM31,
    PedersenPointsTable_alpha40: QM31,
    PedersenPointsTable_alpha41: QM31,
    PedersenPointsTable_alpha42: QM31,
    PedersenPointsTable_alpha43: QM31,
    PedersenPointsTable_alpha44: QM31,
    PedersenPointsTable_alpha45: QM31,
    PedersenPointsTable_alpha46: QM31,
    PedersenPointsTable_alpha47: QM31,
    PedersenPointsTable_alpha48: QM31,
    PedersenPointsTable_alpha49: QM31,
    PedersenPointsTable_alpha5: QM31,
    PedersenPointsTable_alpha50: QM31,
    PedersenPointsTable_alpha51: QM31,
    PedersenPointsTable_alpha52: QM31,
    PedersenPointsTable_alpha53: QM31,
    PedersenPointsTable_alpha54: QM31,
    PedersenPointsTable_alpha55: QM31,
    PedersenPointsTable_alpha56: QM31,
    PedersenPointsTable_alpha6: QM31,
    PedersenPointsTable_alpha7: QM31,
    PedersenPointsTable_alpha8: QM31,
    PedersenPointsTable_alpha9: QM31,
    PedersenPointsTable_z: QM31,
    pedersen_points_0: QM31,
    pedersen_points_1: QM31,
    pedersen_points_10: QM31,
    pedersen_points_11: QM31,
    pedersen_points_12: QM31,
    pedersen_points_13: QM31,
    pedersen_points_14: QM31,
    pedersen_points_15: QM31,
    pedersen_points_16: QM31,
    pedersen_points_17: QM31,
    pedersen_points_18: QM31,
    pedersen_points_19: QM31,
    pedersen_points_2: QM31,
    pedersen_points_20: QM31,
    pedersen_points_21: QM31,
    pedersen_points_22: QM31,
    pedersen_points_23: QM31,
    pedersen_points_24: QM31,
    pedersen_points_25: QM31,
    pedersen_points_26: QM31,
    pedersen_points_27: QM31,
    pedersen_points_28: QM31,
    pedersen_points_29: QM31,
    pedersen_points_3: QM31,
    pedersen_points_30: QM31,
    pedersen_points_31: QM31,
    pedersen_points_32: QM31,
    pedersen_points_33: QM31,
    pedersen_points_34: QM31,
    pedersen_points_35: QM31,
    pedersen_points_36: QM31,
    pedersen_points_37: QM31,
    pedersen_points_38: QM31,
    pedersen_points_39: QM31,
    pedersen_points_4: QM31,
    pedersen_points_40: QM31,
    pedersen_points_41: QM31,
    pedersen_points_42: QM31,
    pedersen_points_43: QM31,
    pedersen_points_44: QM31,
    pedersen_points_45: QM31,
    pedersen_points_46: QM31,
    pedersen_points_47: QM31,
    pedersen_points_48: QM31,
    pedersen_points_49: QM31,
    pedersen_points_5: QM31,
    pedersen_points_50: QM31,
    pedersen_points_51: QM31,
    pedersen_points_52: QM31,
    pedersen_points_53: QM31,
    pedersen_points_54: QM31,
    pedersen_points_55: QM31,
    pedersen_points_6: QM31,
    pedersen_points_7: QM31,
    pedersen_points_8: QM31,
    pedersen_points_9: QM31,
    seq: QM31,
) -> QM31 {
    (PedersenPointsTable_alpha0) * (seq)
        + (PedersenPointsTable_alpha1) * (pedersen_points_0)
        + (PedersenPointsTable_alpha2) * (pedersen_points_1)
        + (PedersenPointsTable_alpha3) * (pedersen_points_2)
        + (PedersenPointsTable_alpha4) * (pedersen_points_3)
        + (PedersenPointsTable_alpha5) * (pedersen_points_4)
        + (PedersenPointsTable_alpha6) * (pedersen_points_5)
        + (PedersenPointsTable_alpha7) * (pedersen_points_6)
        + (PedersenPointsTable_alpha8) * (pedersen_points_7)
        + (PedersenPointsTable_alpha9) * (pedersen_points_8)
        + (PedersenPointsTable_alpha10) * (pedersen_points_9)
        + (PedersenPointsTable_alpha11) * (pedersen_points_10)
        + (PedersenPointsTable_alpha12) * (pedersen_points_11)
        + (PedersenPointsTable_alpha13) * (pedersen_points_12)
        + (PedersenPointsTable_alpha14) * (pedersen_points_13)
        + (PedersenPointsTable_alpha15) * (pedersen_points_14)
        + (PedersenPointsTable_alpha16) * (pedersen_points_15)
        + (PedersenPointsTable_alpha17) * (pedersen_points_16)
        + (PedersenPointsTable_alpha18) * (pedersen_points_17)
        + (PedersenPointsTable_alpha19) * (pedersen_points_18)
        + (PedersenPointsTable_alpha20) * (pedersen_points_19)
        + (PedersenPointsTable_alpha21) * (pedersen_points_20)
        + (PedersenPointsTable_alpha22) * (pedersen_points_21)
        + (PedersenPointsTable_alpha23) * (pedersen_points_22)
        + (PedersenPointsTable_alpha24) * (pedersen_points_23)
        + (PedersenPointsTable_alpha25) * (pedersen_points_24)
        + (PedersenPointsTable_alpha26) * (pedersen_points_25)
        + (PedersenPointsTable_alpha27) * (pedersen_points_26)
        + (PedersenPointsTable_alpha28) * (pedersen_points_27)
        + (PedersenPointsTable_alpha29) * (pedersen_points_28)
        + (PedersenPointsTable_alpha30) * (pedersen_points_29)
        + (PedersenPointsTable_alpha31) * (pedersen_points_30)
        + (PedersenPointsTable_alpha32) * (pedersen_points_31)
        + (PedersenPointsTable_alpha33) * (pedersen_points_32)
        + (PedersenPointsTable_alpha34) * (pedersen_points_33)
        + (PedersenPointsTable_alpha35) * (pedersen_points_34)
        + (PedersenPointsTable_alpha36) * (pedersen_points_35)
        + (PedersenPointsTable_alpha37) * (pedersen_points_36)
        + (PedersenPointsTable_alpha38) * (pedersen_points_37)
        + (PedersenPointsTable_alpha39) * (pedersen_points_38)
        + (PedersenPointsTable_alpha40) * (pedersen_points_39)
        + (PedersenPointsTable_alpha41) * (pedersen_points_40)
        + (PedersenPointsTable_alpha42) * (pedersen_points_41)
        + (PedersenPointsTable_alpha43) * (pedersen_points_42)
        + (PedersenPointsTable_alpha44) * (pedersen_points_43)
        + (PedersenPointsTable_alpha45) * (pedersen_points_44)
        + (PedersenPointsTable_alpha46) * (pedersen_points_45)
        + (PedersenPointsTable_alpha47) * (pedersen_points_46)
        + (PedersenPointsTable_alpha48) * (pedersen_points_47)
        + (PedersenPointsTable_alpha49) * (pedersen_points_48)
        + (PedersenPointsTable_alpha50) * (pedersen_points_49)
        + (PedersenPointsTable_alpha51) * (pedersen_points_50)
        + (PedersenPointsTable_alpha52) * (pedersen_points_51)
        + (PedersenPointsTable_alpha53) * (pedersen_points_52)
        + (PedersenPointsTable_alpha54) * (pedersen_points_53)
        + (PedersenPointsTable_alpha55) * (pedersen_points_54)
        + (PedersenPointsTable_alpha56) * (pedersen_points_55)
        - (PedersenPointsTable_z)
}

