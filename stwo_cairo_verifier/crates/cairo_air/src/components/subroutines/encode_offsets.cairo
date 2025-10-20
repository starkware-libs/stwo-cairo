// This file was created by the AIR team.

use crate::prelude::*;


pub fn encode_offsets_evaluate(
    input: [QM31; 3],
    offset0_low_col0: QM31,
    offset0_mid_col1: QM31,
    offset1_low_col2: QM31,
    offset1_mid_col3: QM31,
    offset1_high_col4: QM31,
    offset2_low_col5: QM31,
    offset2_mid_col6: QM31,
    offset2_high_col7: QM31,
    range_check_7_2_5_lookup_elements: @crate::RangeCheck_7_2_5Elements,
    range_check_4_3_lookup_elements: @crate::RangeCheck_4_3Elements,
    ref range_check_7_2_5_sum_0: QM31,
    ref range_check_4_3_sum_1: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 2] {
    let [encode_offsets_input_offset0, encode_offsets_input_offset1, encode_offsets_input_offset2] =
        input;

    // Constraint - Reconstructed offset0 is correct
    let constraint_quotient = (((offset0_low_col0
        + (offset0_mid_col1 * qm31_const::<512, 0, 0, 0>()))
        - encode_offsets_input_offset0))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - Reconstructed offset1 is correct
    let constraint_quotient = ((((offset1_low_col2
        + (offset1_mid_col3 * qm31_const::<4, 0, 0, 0>()))
        + (offset1_high_col4 * qm31_const::<2048, 0, 0, 0>()))
        - encode_offsets_input_offset1))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - Reconstructed offset2 is correct
    let constraint_quotient = ((((offset2_low_col5
        + (offset2_mid_col6 * qm31_const::<16, 0, 0, 0>()))
        + (offset2_high_col7 * qm31_const::<8192, 0, 0, 0>()))
        - encode_offsets_input_offset2))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    range_check_7_2_5_sum_0 = range_check_7_2_5_lookup_elements
        .combine_qm31([offset0_mid_col1, offset1_low_col2, offset1_high_col4]);

    range_check_4_3_sum_1 = range_check_4_3_lookup_elements
        .combine_qm31([offset2_low_col5, offset2_high_col7]);

    [
        (offset0_mid_col1 + (offset1_low_col2 * qm31_const::<128, 0, 0, 0>())),
        (offset1_high_col4 + (offset2_low_col5 * qm31_const::<32, 0, 0, 0>())),
    ]
}
