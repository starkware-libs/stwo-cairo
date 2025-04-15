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
    preprocessed_column_set.insert(PreprocessedColumn::RangeCheck(([18, 0, 0, 0, 0], 0)));
    let point_offset_neg_1 = point.add_circle_point_m31(-trace_gen.mul(1).to_point());
    trace_mask_points.append(array![point]);
    interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
    interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
    interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
    interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
}

#[derive(Drop)]
pub struct ConstraintParams {
    pub RangeCheck_18_alpha0: QM31,
    pub RangeCheck_18_z: QM31,
    pub claimed_sum: QM31,
    pub range_check_18_column_0: QM31,
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
        RangeCheck_18_alpha0, RangeCheck_18_z, claimed_sum, column_size, range_check_18_column_0,
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
        RangeCheck_18_alpha0, RangeCheck_18_z, range_check_18_column_0,
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
        - (-(trace_1_column_0_offset_0)))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;
}


fn intermediates(
    RangeCheck_18_alpha0: QM31, RangeCheck_18_z: QM31, range_check_18_column_0: QM31,
) -> Array<QM31> {
    let intermediate0 = intermediate0(
        RangeCheck_18_alpha0, RangeCheck_18_z, range_check_18_column_0,
    );
    array![intermediate0]
}


pub fn intermediate0(
    RangeCheck_18_alpha0: QM31, RangeCheck_18_z: QM31, range_check_18_column_0: QM31,
) -> QM31 {
    (RangeCheck_18_alpha0) * (range_check_18_column_0) - (RangeCheck_18_z)
}

