use stwo_constraint_framework::{
    PreprocessedColumn, PreprocessedColumnSet, PreprocessedColumnSetImpl,
};
use stwo_verifier_core::circle::{
    CirclePoint, CirclePointIndex, CirclePointIndexImpl, CirclePointQM31AddCirclePointM31Impl,
};
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
    interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
    interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
    interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
    interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
}

#[derive(Drop)]
pub struct ConstraintParams {
    pub RangeCheck_7_2_5_alpha0: QM31,
    pub RangeCheck_7_2_5_alpha1: QM31,
    pub RangeCheck_7_2_5_alpha2: QM31,
    pub RangeCheck_7_2_5_z: QM31,
    pub claimed_sum: QM31,
}

pub fn evaluate_constraints_at_point(
    ref sum: QM31,
    ref trace_mask_values: ColumnSpan<Array<QM31>>,
    ref interaction_mask_values: ColumnSpan<Array<QM31>>,
    params: ConstraintParams,
    random_coeff: QM31,
    domain_vanish_at_point_inv: QM31,
) {
    let ConstraintParams {
        RangeCheck_7_2_5_alpha0,
        RangeCheck_7_2_5_alpha1,
        RangeCheck_7_2_5_alpha2,
        RangeCheck_7_2_5_z,
        claimed_sum,
    } = params;

    let [trace_1_column_0_offset_0]: [QM31; 1] = (*trace_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_1_column_1_offset_0]: [QM31; 1] = (*trace_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_1_column_2_offset_0]: [QM31; 1] = (*trace_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_1_column_3_offset_0]: [QM31; 1] = (*trace_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_4_offset_neg_1, trace_2_column_4_offset_0]: [QM31; 2] =
        (*interaction_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_5_offset_neg_1, trace_2_column_5_offset_0]: [QM31; 2] =
        (*interaction_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_6_offset_neg_1, trace_2_column_6_offset_0]: [QM31; 2] =
        (*interaction_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    let [trace_2_column_7_offset_neg_1, trace_2_column_7_offset_0]: [QM31; 2] =
        (*interaction_mask_values
        .pop_front()
        .unwrap()
        .span()
        .try_into()
        .unwrap())
        .unbox();

    core::internal::revoke_ap_tracking();

    let mut intermediates = intermediates(
        RangeCheck_7_2_5_alpha0,
        RangeCheck_7_2_5_alpha1,
        RangeCheck_7_2_5_alpha2,
        RangeCheck_7_2_5_z,
        trace_1_column_0_offset_0,
        trace_1_column_1_offset_0,
        trace_1_column_2_offset_0,
    )
        .span();
    let intermediate0 = *intermediates.pop_front().unwrap();

    // Constrait 0
    let constraint_quotient = ((QM31Impl::from_partial_evals(
        [
            trace_2_column_4_offset_0, trace_2_column_5_offset_0, trace_2_column_6_offset_0,
            trace_2_column_7_offset_0,
        ],
    )
        - (QM31Impl::from_partial_evals(
            [
                trace_2_column_4_offset_neg_1, trace_2_column_5_offset_neg_1,
                trace_2_column_6_offset_neg_1, trace_2_column_7_offset_neg_1,
            ],
        ))
        + (claimed_sum) * (qm31(32768, 0, 0, 0)))
        * (intermediate0)
        - (-(trace_1_column_3_offset_0)))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;
}


fn intermediates(
    RangeCheck_7_2_5_alpha0: QM31,
    RangeCheck_7_2_5_alpha1: QM31,
    RangeCheck_7_2_5_alpha2: QM31,
    RangeCheck_7_2_5_z: QM31,
    trace_1_column_0_offset_0: QM31,
    trace_1_column_1_offset_0: QM31,
    trace_1_column_2_offset_0: QM31,
) -> Array<QM31> {
    let intermediate0 = intermediate0(
        RangeCheck_7_2_5_alpha0,
        RangeCheck_7_2_5_alpha1,
        RangeCheck_7_2_5_alpha2,
        RangeCheck_7_2_5_z,
        trace_1_column_0_offset_0,
        trace_1_column_1_offset_0,
        trace_1_column_2_offset_0,
    );
    array![intermediate0]
}


pub fn intermediate0(
    RangeCheck_7_2_5_alpha0: QM31,
    RangeCheck_7_2_5_alpha1: QM31,
    RangeCheck_7_2_5_alpha2: QM31,
    RangeCheck_7_2_5_z: QM31,
    trace_1_column_0_offset_0: QM31,
    trace_1_column_1_offset_0: QM31,
    trace_1_column_2_offset_0: QM31,
) -> QM31 {
    (RangeCheck_7_2_5_alpha0) * (trace_1_column_0_offset_0)
        + (RangeCheck_7_2_5_alpha1) * (trace_1_column_1_offset_0)
        + (RangeCheck_7_2_5_alpha2) * (trace_1_column_2_offset_0)
        - (RangeCheck_7_2_5_z)
}

