// This file was created by the AIR team.

use crate::prelude::*;


pub fn decode_instruction_cb32b_evaluate(
    input: QM31,
    offset0_col0: QM31,
    offset1_col1: QM31,
    offset2_col2: QM31,
    dst_base_fp_col3: QM31,
    op0_base_fp_col4: QM31,
    ap_update_add_1_col5: QM31,
    common_lookup_elements: @CommonLookupElements,
    ref verify_instruction_sum_0: QM31,
    ref numerator_0: QM31,
    ref sum: QM31,
    random_coeff: QM31,
) -> [QM31; 3] {
    let decode_instruction_cb32b_input_pc = input;

    // Constraint - Flag dst_base_fp is a bit
    let constraint_quotient = ((dst_base_fp_col3
        * (qm31_const::<1, 0, 0, 0>() - dst_base_fp_col3)));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - Flag op0_base_fp is a bit
    let constraint_quotient = ((op0_base_fp_col4
        * (qm31_const::<1, 0, 0, 0>() - op0_base_fp_col4)));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - Flag ap_update_add_1 is a bit
    let constraint_quotient = ((ap_update_add_1_col5
        * (qm31_const::<1, 0, 0, 0>() - ap_update_add_1_col5)));
    sum = sum * random_coeff + constraint_quotient;

    verify_instruction_sum_0 = common_lookup_elements
        .combine_qm31(
            [
                qm31_const::<1719106205, 0, 0, 0>(), decode_instruction_cb32b_input_pc,
                offset0_col0, offset1_col1, offset2_col2,
                ((dst_base_fp_col3 * qm31_const::<8, 0, 0, 0>())
                    + (op0_base_fp_col4 * qm31_const::<16, 0, 0, 0>())),
                ((ap_update_add_1_col5 * qm31_const::<32, 0, 0, 0>())
                    + qm31_const::<256, 0, 0, 0>()),
                qm31_const::<0, 0, 0, 0>(),
            ]
                .span(),
        );
    numerator_0 = qm31_const::<1, 0, 0, 0>();

    [
        (offset0_col0 - qm31_const::<32768, 0, 0, 0>()),
        (offset1_col1 - qm31_const::<32768, 0, 0, 0>()),
        (offset2_col2 - qm31_const::<32768, 0, 0, 0>()),
    ]
}
