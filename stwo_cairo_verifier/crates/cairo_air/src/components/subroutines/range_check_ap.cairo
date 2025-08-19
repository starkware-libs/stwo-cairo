// AIR version d1591e2a
use crate::prelude::*;


pub fn range_check_ap_evaluate(
    input: QM31,
    range_check_ap_bot8bits_col0: QM31,
    range_check_19_lookup_elements: @crate::RangeCheck_19Elements,
    range_check_8_lookup_elements: @crate::RangeCheck_8Elements,
    ref range_check_19_sum_0: QM31,
    ref range_check_8_sum_1: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 0] {
    let range_check_ap_input = input;

    range_check_19_sum_0 = range_check_19_lookup_elements
        .combine_qm31(
            [
                ((range_check_ap_input - range_check_ap_bot8bits_col0)
                    * qm31_const::<8388608, 0, 0, 0>())
            ],
        );

    range_check_8_sum_1 = range_check_8_lookup_elements
        .combine_qm31([range_check_ap_bot8bits_col0]);

    []
}
