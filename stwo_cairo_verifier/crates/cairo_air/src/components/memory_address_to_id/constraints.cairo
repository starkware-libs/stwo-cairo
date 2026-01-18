use crate::cairo_air::MEMORY_ADDRESS_TO_ID_RELATION_ID;
use crate::prelude::*;
use crate::utils::*;
use super::N_INTERACTION_TRACE_QM31_COLUMNS;

#[derive(Drop)]
pub struct ConstraintParams {
    pub common_lookup_elements: @CommonLookupElements,
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
) {
    let ConstraintParams { common_lookup_elements, claimed_sum, seq, column_size } = params;
    let column_size: QM31 = column_size.into();

    let mut prev_cum_sum: QM31 = Zero::zero();
    let mut address: QM31 = seq + m31(1).into();

    // This loop executes `N_INTERACTION_TRACE_QM31_COLUMNS - 1` iterations, each enforcing a
    // column-wise pairwise sum constraint. After this loop, a final constraint will handle both a
    // row-wise sum and a column-wise pairwise sum.
    for _ in 0..N_INTERACTION_TRACE_QM31_COLUMNS - 1 {
        // Get two (id, multiplicity) from the trace.
        let [id_0, multiplicity_0, id_1, multiplicity_1]: [Span<QM31>; 4] = (*trace_mask_values
            .multi_pop_front()
            .unwrap())
            .unbox();
        let [id_0]: [QM31; 1] = (*id_0.try_into().unwrap()).unbox();
        let [multiplicity_0]: [QM31; 1] = (*multiplicity_0.try_into().unwrap()).unbox();
        let [id_1]: [QM31; 1] = (*id_1.try_into().unwrap()).unbox();
        let [multiplicity_1]: [QM31; 1] = (*multiplicity_1.try_into().unwrap()).unbox();

        // Get the corresponding cumulative logup sum from interaction trace.
        let curr_cum_sum = as_qm31(interaction_mask_values.multi_pop_front::<4>().unwrap());

        let combination_0 = common_lookup_elements
            .combine_qm31(
                [Into::<M31, QM31>::into(MEMORY_ADDRESS_TO_ID_RELATION_ID), address, id_0].span(),
            );
        address += column_size;

        let combination_1 = common_lookup_elements
            .combine_qm31(
                [Into::<M31, QM31>::into(MEMORY_ADDRESS_TO_ID_RELATION_ID), address, id_1].span(),
            );
        address += column_size;

        // Check that:
        // (current - prev) = (-multiplicity0 / intermediate0) + (-multiplicity1 / intermediate1)
        // = (-multiplicity0 * intermediate1 -multiplicity1 * intermediate0) / (intermediate0 *
        // intermediate1)
        // ==>
        // (current - prev) * (intermediate0 * intermediate1) =
        // -multiplicity0 * intermediate1 - multiplicity1 * intermediate0
        let constraint_quotient = ((curr_cum_sum - prev_cum_sum) * combination_0 * combination_1
            + multiplicity_0 * combination_1
            + multiplicity_1 * combination_0);
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

    // Get the current and previous row's logup sum.
    let [neg_1_cum_sum, curr_cum_sum] = as_neighboring_qm31s(
        interaction_mask_values.multi_pop_front::<4>().unwrap(),
    );

    let combination_0 = common_lookup_elements
        .combine_qm31(
            [Into::<M31, QM31>::into(MEMORY_ADDRESS_TO_ID_RELATION_ID), address, id_0].span(),
        );
    address += column_size;
    let combination_1 = common_lookup_elements
        .combine_qm31(
            [Into::<M31, QM31>::into(MEMORY_ADDRESS_TO_ID_RELATION_ID), address, id_1].span(),
        );

    // Final constraint, Check that:
    // (current_cum_sum - prev_cum_sum - neg_1_cum_sum + claimed_sum/column_size) *
    // combination_0*combination_1 = -multiplicity0 * combination_1 - multiplicity1 * combination_0
    let constraint_quotient = ((curr_cum_sum
        - prev_cum_sum
        - neg_1_cum_sum
        + claimed_sum * column_size.inverse().into())
        * combination_0
        * combination_1
        + multiplicity_0 * combination_1
        + multiplicity_1 * combination_0);
    sum = sum * random_coeff + constraint_quotient;
}
