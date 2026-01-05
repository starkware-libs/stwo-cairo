use crate::prelude::*;
use crate::utils::*;

#[derive(Drop)]
pub struct ConstraintParams {
    pub lookup_elements: @crate::VerifyBitwiseXor_12Elements,
    pub bitwise_xor_10_0: QM31,
    pub bitwise_xor_10_1: QM31,
    pub bitwise_xor_10_2: QM31,
    pub claimed_sum: QM31,
    pub column_size: M31,
}

// Each row in the bitwise_xor_12 component yields 16 tuples into
// the VerifyBitwiseXor_12 relation. Compute the <index>'th tuple.
fn make_xor12_triplet(xor_10_0: QM31, xor_10_1: QM31, xor_10_2: QM31, index: u32) -> [QM31; 3] {
    let i = index / 4;
    let j = index % 4;
    [
        xor_10_0 + m31(i * 1024).into(), xor_10_1 + m31(j * 1024).into(),
        xor_10_2 + m31((i ^ j) * 1024).into(),
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
    let ConstraintParams {
        lookup_elements,
        bitwise_xor_10_0,
        bitwise_xor_10_1,
        bitwise_xor_10_2,
        claimed_sum,
        column_size,
    } = params;
    let column_size: QM31 = column_size.into();

    let mut prev_cum_sum: QM31 = Zero::zero();

    // Sum the yields from the first 14 multiplicities. The last two are summed later,
    // together with the cumulative sum from the row above.
    for i in 0..7_u32 {
        // Get two multiplicities from the trace.
        let [multiplicity_0, multiplicity_1]: [Span<QM31>; 2] = (*trace_mask_values
            .multi_pop_front()
            .unwrap())
            .unbox();
        let [multiplicity_0]: [QM31; 1] = (*multiplicity_0.try_into().unwrap()).unbox();
        let [multiplicity_1]: [QM31; 1] = (*multiplicity_1.try_into().unwrap()).unbox();

        // Get the corresponding cumulative logup sum from interaction trace.
        let curr_cum_sum = as_qm31(interaction_mask_values.multi_pop_front::<4>().unwrap());

        let combination_0 = lookup_elements
            .combine_qm31(
                make_xor12_triplet(bitwise_xor_10_0, bitwise_xor_10_1, bitwise_xor_10_2, i * 2),
            );

        let combination_1 = lookup_elements
            .combine_qm31(
                make_xor12_triplet(bitwise_xor_10_0, bitwise_xor_10_1, bitwise_xor_10_2, i * 2 + 1),
            );

        // Check that:
        // (current - prev) = (-multiplicity0 / intermediate0) + (-multiplicity1 / intermediate1)
        // = (-multiplicity0 * intermediate1 -multiplicity1 * intermediate0) / (intermediate0 *
        // intermediate1)
        // ==>
        // (current - prev) * (intermediate0 * intermediate1) =
        // -multiplicity0 * intermediate1 - multiplicity1 * intermediate0
        let constraint_quotient = ((curr_cum_sum - prev_cum_sum) * combination_0 * combination_1
            + multiplicity_0 * combination_1
            + multiplicity_1 * combination_0)
            * domain_vanish_at_point_inv;
        sum = sum * random_coeff + constraint_quotient;
        prev_cum_sum = curr_cum_sum;
    }

    let [multiplicity_0, multiplicity_1]: [Span<QM31>; 2] = (*trace_mask_values
        .multi_pop_front()
        .unwrap())
        .unbox();
    let [multiplicity_0]: [QM31; 1] = (*multiplicity_0.try_into().unwrap()).unbox();
    let [multiplicity_1]: [QM31; 1] = (*multiplicity_1.try_into().unwrap()).unbox();

    // Get the current and previous row's logup sum.
    let [neg_1_cum_sum, curr_cum_sum] = as_neighboring_qm31s(
        interaction_mask_values.multi_pop_front::<4>().unwrap(),
    );

    let combination_0 = lookup_elements
        .combine_qm31(make_xor12_triplet(bitwise_xor_10_0, bitwise_xor_10_1, bitwise_xor_10_2, 14));
    let combination_1 = lookup_elements
        .combine_qm31(make_xor12_triplet(bitwise_xor_10_0, bitwise_xor_10_1, bitwise_xor_10_2, 15));

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
        + multiplicity_1 * combination_0)
        * domain_vanish_at_point_inv;
    sum = sum * random_coeff + constraint_quotient;
}

