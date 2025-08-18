use crate::prelude::*;
use super::MEMORY_ADDRESS_TO_ID_SPLIT;

pub fn mask_points(
    ref preprocessed_column_set: PreprocessedColumnSet,
    ref trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
    ref interaction_trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
    point: CirclePoint<QM31>,
    trace_gen: CirclePointIndex,
    log_size: u32,
) {
    preprocessed_column_set.insert(PreprocessedColumn::Seq(log_size));
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
    pub lookup_elements: @crate::MemoryAddressToIdElements,
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
    let ConstraintParams { lookup_elements, claimed_sum, seq, column_size } = params;
    let mut prev: QM31 = Zero::zero();

    // Go over all pairs of (address, id) in the trace except for the last one.
    for pair_index in 0..(MEMORY_ADDRESS_TO_ID_SPLIT / 2 - 1) {
        // Get two (address, id) from the trace.
        let [
            trace_1_column_0, trace_1_column_1, trace_1_column_2, trace_1_column_3,
        ]: [Span<QM31>; 4] =
            (*trace_mask_values
            .multi_pop_front()
            .unwrap())
            .unbox();
        let [trace_1_column_0_offset_0]: [QM31; 1] = (*trace_1_column_0.try_into().unwrap())
            .unbox();
        let [trace_1_column_1_offset_0]: [QM31; 1] = (*trace_1_column_1.try_into().unwrap())
            .unbox();
        let [trace_1_column_2_offset_0]: [QM31; 1] = (*trace_1_column_2.try_into().unwrap())
            .unbox();
        let [trace_1_column_3_offset_0]: [QM31; 1] = (*trace_1_column_3.try_into().unwrap())
            .unbox();

        // Get the corresponding logup sum qm31 element.
        let [
            trace_2_column_0, trace_2_column_1, trace_2_column_2, trace_2_column_3,
        ]: [Span<QM31>; 4] =
            (*interaction_mask_values
            .multi_pop_front()
            .unwrap())
            .unbox();
        let [trace_2_column_0_offset_0]: [QM31; 1] = (*trace_2_column_0.try_into().unwrap())
            .unbox();
        let [trace_2_column_1_offset_0]: [QM31; 1] = (*trace_2_column_1.try_into().unwrap())
            .unbox();
        let [trace_2_column_2_offset_0]: [QM31; 1] = (*trace_2_column_2.try_into().unwrap())
            .unbox();
        let [trace_2_column_3_offset_0]: [QM31; 1] = (*trace_2_column_3.try_into().unwrap())
            .unbox();

        let current = QM31Trait::from_partial_evals(
            [
                trace_2_column_0_offset_0, trace_2_column_1_offset_0, trace_2_column_2_offset_0,
                trace_2_column_3_offset_0,
            ],
        );

        let intermediate_0 = lookup_elements
            .combine_qm31(
                [
                    seq + m31(1).into() + (column_size * m31(pair_index * 2)).into(),
                    trace_1_column_0_offset_0,
                ],
            );
        let intermediate_1 = lookup_elements
            .combine_qm31(
                [
                    seq + m31(1).into() + (column_size * m31(pair_index * 2 + 1)).into(),
                    trace_1_column_2_offset_0,
                ],
            );

        // Check that:
        // (current - prev) = (-multiplicity0 / intermediate0) + (-multiplicity1 / intermediate1)
        // = (-multiplicity0 * intermediate1 -multiplicity1 * intermediate0) / (intermediate0 *
        // intermediate1)
        // ==>
        // (current - prev) * (intermediate0 * intermediate1) =
        // -multiplicity0 * intermediate1 - multiplicity1 * intermediate0
        let constraint_quotient = ((current - prev) * intermediate_0 * intermediate_1
            - (intermediate_1 * (-(trace_1_column_1_offset_0))
                + intermediate_0 * (-(trace_1_column_3_offset_0))))
            * domain_vanish_at_point_inv;
        sum = sum * random_coeff + constraint_quotient;
        prev = current;
    }

    let [trace_1_column_0, trace_1_column_1, trace_1_column_2, trace_1_column_3]: [Span<QM31>; 4] =
        (*trace_mask_values
        .multi_pop_front()
        .unwrap())
        .unbox();
    let [trace_1_column_0_offset_0]: [QM31; 1] = (*trace_1_column_0.try_into().unwrap()).unbox();
    let [trace_1_column_1_offset_0]: [QM31; 1] = (*trace_1_column_1.try_into().unwrap()).unbox();
    let [trace_1_column_2_offset_0]: [QM31; 1] = (*trace_1_column_2.try_into().unwrap()).unbox();
    let [trace_1_column_3_offset_0]: [QM31; 1] = (*trace_1_column_3.try_into().unwrap()).unbox();

    // Get the corresponding logup sum qm31 element.
    let [trace_2_column_0, trace_2_column_1, trace_2_column_2, trace_2_column_3]: [Span<QM31>; 4] =
        (*interaction_mask_values
        .multi_pop_front()
        .unwrap())
        .unbox();
    let [trace_2_column_0_offset_neg_1, trace_2_column_0_offset_0]: [QM31; 2] = (*trace_2_column_0
        .try_into()
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

    let current = QM31Trait::from_partial_evals(
        [
            trace_2_column_0_offset_0, trace_2_column_1_offset_0, trace_2_column_2_offset_0,
            trace_2_column_3_offset_0,
        ],
    );

    let intermediate_0 = lookup_elements
        .combine_qm31(
            [
                seq + m31(1).into() + (column_size * m31(MEMORY_ADDRESS_TO_ID_SPLIT - 2)).into(),
                trace_1_column_0_offset_0,
            ],
        );
    let intermediate_1 = lookup_elements
        .combine_qm31(
            [
                seq + m31(1).into() + (column_size * m31(MEMORY_ADDRESS_TO_ID_SPLIT - 1)).into(),
                trace_1_column_2_offset_0,
            ],
        );

    // Final constraint, Check that:
    // (current - prev - neg1 + claimed_sum/column_size) * I0*I1 = -multiplicity0 * I1 -
    // multiplicity1 * I0
    let constraint_quotient = ((current
        - prev
        - QM31Trait::from_partial_evals(
            [
                trace_2_column_0_offset_neg_1, trace_2_column_1_offset_neg_1,
                trace_2_column_2_offset_neg_1, trace_2_column_3_offset_neg_1,
            ],
        )
        + claimed_sum * column_size.inverse().into())
        * intermediate_0
        * intermediate_1
        - (intermediate_1 * (-(trace_1_column_1_offset_0))
            + intermediate_0 * (-(trace_1_column_3_offset_0))))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;
}
