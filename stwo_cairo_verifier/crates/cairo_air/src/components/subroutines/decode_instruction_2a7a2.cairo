// This file was created by the AIR team.

use crate::prelude::*;


pub fn decode_instruction_2a7a2_evaluate(
    input: QM31,
    common_lookup_elements: @CommonLookupElements,
    ref verify_instruction_sum_0: QM31,
    ref sum: QM31,
    random_coeff: QM31,
) -> [QM31; 0] {
    let decode_instruction_2a7a2_input_pc = input;

    verify_instruction_sum_0 = common_lookup_elements
        .combine_qm31(
            [
                qm31_const::<1719106205, 0, 0, 0>(), decode_instruction_2a7a2_input_pc,
                qm31_const::<32768, 0, 0, 0>(), qm31_const::<32769, 0, 0, 0>(),
                qm31_const::<32769, 0, 0, 0>(), qm31_const::<32, 0, 0, 0>(),
                qm31_const::<68, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(),
            ]
                .span(),
        );

    []
}
