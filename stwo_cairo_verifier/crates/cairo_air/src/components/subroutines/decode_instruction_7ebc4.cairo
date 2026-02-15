// This file was created by the AIR team.

use crate::prelude::*;


pub fn decode_instruction_7ebc4_evaluate(
    input: QM31,
    ap_update_add_1_col0: QM31,
    common_lookup_elements: @CommonLookupElements,
    ref verify_instruction_sum_0: QM31,
    ref numerator_0: QM31,
    ref sum: QM31,
    random_coeff: QM31,
) -> [QM31; 0] {
    let decode_instruction_7ebc4_input_pc = input;

    // Constraint - Flag ap_update_add_1 is a bit
    let constraint_quotient = ((ap_update_add_1_col0
        * (qm31_const::<1, 0, 0, 0>() - ap_update_add_1_col0)));
    sum = sum * random_coeff + constraint_quotient;

    verify_instruction_sum_0 = common_lookup_elements
        .combine_qm31(
            [
                qm31_const::<1719106205, 0, 0, 0>(), decode_instruction_7ebc4_input_pc,
                qm31_const::<32767, 0, 0, 0>(), qm31_const::<32767, 0, 0, 0>(),
                qm31_const::<32769, 0, 0, 0>(), qm31_const::<56, 0, 0, 0>(),
                (qm31_const::<4, 0, 0, 0>() + (ap_update_add_1_col0 * qm31_const::<32, 0, 0, 0>())),
                qm31_const::<0, 0, 0, 0>(),
            ]
                .span(),
        );
    numerator_0 = qm31_const::<1, 0, 0, 0>();

    []
}
