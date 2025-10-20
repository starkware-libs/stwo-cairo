// This file was created by the AIR team.

use crate::prelude::*;


pub fn range_check_ap_evaluate(
    input: QM31,
    range_check_ap_bot11bits_col0: QM31,
    range_check_18_lookup_elements: @crate::RangeCheck_18Elements,
    range_check_11_lookup_elements: @crate::RangeCheck_11Elements,
    ref range_check_18_sum_0: QM31,
    ref range_check_11_sum_1: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 0] {
    let range_check_ap_input = input;

    range_check_18_sum_0 = range_check_18_lookup_elements
        .combine_qm31(
            [
                ((range_check_ap_input - range_check_ap_bot11bits_col0)
                    * qm31_const::<1048576, 0, 0, 0>())
            ],
        );

    range_check_11_sum_1 = range_check_11_lookup_elements
        .combine_qm31([range_check_ap_bot11bits_col0]);

    []
}
