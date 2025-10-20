// This file was created by the AIR team.

use crate::prelude::*;


pub fn decode_instruction_15a61_evaluate(
    input: QM31,
    verify_instruction_lookup_elements: @crate::VerifyInstructionElements,
    ref verify_instruction_sum_0: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 0] {
    let decode_instruction_15a61_input_pc = input;

    verify_instruction_sum_0 = verify_instruction_lookup_elements
        .combine_qm31(
            [
                decode_instruction_15a61_input_pc, qm31_const::<32766, 0, 0, 0>(),
                qm31_const::<32767, 0, 0, 0>(), qm31_const::<32767, 0, 0, 0>(),
                qm31_const::<88, 0, 0, 0>(), qm31_const::<130, 0, 0, 0>(),
                qm31_const::<0, 0, 0, 0>(),
            ],
        );

    []
}
