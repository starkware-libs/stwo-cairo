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

pub fn columns_as_qm31(columns: Option<@Box<[Span<QM31>; 4]>>) -> QM31 {
    let [column_0, column_1, column_2, column_3]: [Span<QM31>; 4] = columns.unwrap().unbox();

    let [coeff_0]: [QM31; 1] = (*column_0.try_into().unwrap()).unbox();
    let [coeff_1]: [QM31; 1] = (*column_1.try_into().unwrap()).unbox();
    let [coeff_2]: [QM31; 1] = (*column_2.try_into().unwrap()).unbox();
    let [coeff_3]: [QM31; 1] = (*column_3.try_into().unwrap()).unbox();

    QM31Trait::from_partial_evals([coeff_0, coeff_1, coeff_2, coeff_3])
}

pub fn columns_as_two_qm31(columns: Option<@Box<[Span<QM31>; 4]>>) -> [QM31; 2] {
    let [column_0, column_1, column_2, column_3]: [Span<QM31>; 4] = columns.unwrap().unbox();

    let [coeff_0_first, coeff_0_second]: [QM31; 2] = (*column_0.try_into().unwrap()).unbox();
    let [coeff_1_first, coeff_1_second]: [QM31; 2] = (*column_1.try_into().unwrap()).unbox();
    let [coeff_2_first, coeff_2_second]: [QM31; 2] = (*column_2.try_into().unwrap()).unbox();
    let [coeff_3_first, coeff_3_second]: [QM31; 2] = (*column_3.try_into().unwrap()).unbox();

    [
        QM31Trait::from_partial_evals([coeff_0_first, coeff_1_first, coeff_2_first, coeff_3_first]),
        QM31Trait::from_partial_evals(
            [coeff_0_second, coeff_1_second, coeff_2_second, coeff_3_second],
        ),
    ]
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
    let mut prev_cum_sum: QM31 = Zero::zero();

    // Go over all pairs of (address, id) in the trace except for the last one.
    for pair_index in 0..(MEMORY_ADDRESS_TO_ID_SPLIT / 2 - 1) {
        // Get two (id, multiplicity) from the trace.
        let [id_0, multiplicity_0, id_1, multiplicity_1]: [Span<QM31>; 4] = (*trace_mask_values
            .multi_pop_front()
            .unwrap())
            .unbox();
        let [id_0]: [QM31; 1] = (*id_0.try_into().unwrap()).unbox();
        let [multiplicity_0]: [QM31; 1] = (*multiplicity_0.try_into().unwrap()).unbox();
        let [id_1]: [QM31; 1] = (*id_1.try_into().unwrap()).unbox();
        let [multiplicity_1]: [QM31; 1] = (*multiplicity_1.try_into().unwrap()).unbox();

        // Get the corresponding cumulative logup sum qm31 coefficients.
        let curr_cum_sum = columns_as_qm31(interaction_mask_values.multi_pop_front::<4>());

        let combination_0 = lookup_elements
            .combine_qm31([seq + m31(1).into() + (column_size * m31(pair_index * 2)).into(), id_0]);
        let combination_1 = lookup_elements
            .combine_qm31(
                [seq + m31(1).into() + (column_size * m31(pair_index * 2 + 1)).into(), id_1],
            );

        // Check that:
        // (current - prev) = (-multiplicity0 / intermediate0) + (-multiplicity1 / intermediate1)
        // = (-multiplicity0 * intermediate1 -multiplicity1 * intermediate0) / (intermediate0 *
        // intermediate1)
        // ==>
        // (current - prev) * (intermediate0 * intermediate1) =
        // -multiplicity0 * intermediate1 - multiplicity1 * intermediate0
        let constraint_quotient = ((curr_cum_sum - prev_cum_sum) * combination_0 * combination_1
            - (combination_1 * (-(multiplicity_0)) + combination_0 * (-(multiplicity_1))))
            * domain_vanish_at_point_inv;
        sum = sum * random_coeff + constraint_quotient;
        prev_cum_sum = curr_cum_sum;
    }

    let [id_0, multiplicity_0, id_1, multiplicity_1]: [Span<QM31>; 4] = (*trace_mask_values
        .multi_pop_front()
        .unwrap())
        .unbox();
    let [id_0]: [QM31; 1] = (*id_0.try_into().unwrap()).unbox();
    let [multiplicity_0]: [QM31; 1] = (*multiplicity_0.try_into().unwrap()).unbox();
    let [id_1]: [QM31; 1] = (*id_1.try_into().unwrap()).unbox();
    let [multiplicity_1]: [QM31; 1] = (*multiplicity_1.try_into().unwrap()).unbox();

    // Get the current and previous row's logup sum qm31 coefficients.
    let [curr_cum_sum, neg_1_cum_sum] = columns_as_two_qm31(
        interaction_mask_values.multi_pop_front::<4>(),
    );

    let combination_0 = lookup_elements
        .combine_qm31(
            [
                seq + m31(1).into() + (column_size * m31(MEMORY_ADDRESS_TO_ID_SPLIT - 2)).into(),
                id_0,
            ],
        );
    let combination_1 = lookup_elements
        .combine_qm31(
            [
                seq + m31(1).into() + (column_size * m31(MEMORY_ADDRESS_TO_ID_SPLIT - 1)).into(),
                id_1,
            ],
        );

    // Final constraint, Check that:
    // (current_cum_sum - prev_cum_sum - neg_1_cum_sum +
    // claimed_sum/column_size) * combination_0*combination_1 = -multiplicity0 *
    // combination_1 - multiplicity1 * combination_0
    let constraint_quotient = ((curr_cum_sum
        - prev_cum_sum
        - neg_1_cum_sum
        + claimed_sum * column_size.inverse().into())
        * combination_0
        * combination_1
        - (combination_1 * (-(multiplicity_0)) + combination_0 * (-(multiplicity_1))))
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;
}
