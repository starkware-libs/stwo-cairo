// AIR version aca38612
use crate::prelude::*;


pub fn split_16_low_part_size_8_evaluate(
    input: [QM31; 1],
    ms_8_bits_col0: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 1] {
    let [split_16_low_part_size_8_input] = input;

    [(split_16_low_part_size_8_input - (ms_8_bits_col0 * qm31_const::<256, 0, 0, 0>()))]
}
