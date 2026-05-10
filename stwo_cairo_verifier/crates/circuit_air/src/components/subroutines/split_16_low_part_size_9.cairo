// This file was created by the AIR team.

use crate::prelude::*;


pub fn split_16_low_part_size_9_evaluate(
    input: QM31,
    ms_7_bits_col0: QM31,
    common_lookup_elements: @CommonLookupElements,
    ref sum: QM31,
    random_coeff: QM31,
) -> QM31 {
    let split_16_low_part_size_9_input = input;

    (split_16_low_part_size_9_input - (ms_7_bits_col0 * qm31_const::<512, 0, 0, 0>()))
}
