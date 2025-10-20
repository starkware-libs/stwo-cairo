// This file was created by the AIR team.

use crate::prelude::*;


pub fn split_16_low_part_size_7_evaluate(
    input: QM31,
    ms_9_bits_col0: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> QM31 {
    let split_16_low_part_size_7_input = input;

    (split_16_low_part_size_7_input - (ms_9_bits_col0 * qm31_const::<128, 0, 0, 0>()))
}
