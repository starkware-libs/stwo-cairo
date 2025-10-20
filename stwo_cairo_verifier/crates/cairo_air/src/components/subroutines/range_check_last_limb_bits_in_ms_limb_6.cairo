// This file was created by the AIR team.

use crate::prelude::*;


pub fn range_check_last_limb_bits_in_ms_limb_6_evaluate(
    input: QM31,
    range_check_6_lookup_elements: @crate::RangeCheck_6Elements,
    ref range_check_6_sum_0: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 0] {
    let range_check_last_limb_bits_in_ms_limb_6_input = input;

    range_check_6_sum_0 = range_check_6_lookup_elements
        .combine_qm31([range_check_last_limb_bits_in_ms_limb_6_input]);

    []
}
