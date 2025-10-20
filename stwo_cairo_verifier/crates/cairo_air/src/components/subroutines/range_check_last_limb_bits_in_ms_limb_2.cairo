// This file was created by the AIR team.

use crate::components::subroutines::cond_range_check_2::cond_range_check_2_evaluate;
use crate::prelude::*;


pub fn range_check_last_limb_bits_in_ms_limb_2_evaluate(
    input: QM31,
    partial_limb_msb_col0: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 0] {
    let range_check_last_limb_bits_in_ms_limb_2_input = input;
    cond_range_check_2_evaluate(
        [range_check_last_limb_bits_in_ms_limb_2_input, qm31_const::<1, 0, 0, 0>()],
        partial_limb_msb_col0,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    []
}
