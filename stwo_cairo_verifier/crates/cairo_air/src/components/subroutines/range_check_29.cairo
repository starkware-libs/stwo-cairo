// This file was created by the AIR team.

use crate::prelude::*;


pub fn range_check_29_evaluate(
    input: QM31,
    range_check_29_bot11bits_col0: QM31,
    common_lookup_elements: @CommonLookupElements,
    ref range_check_18_sum_0: QM31,
    ref numerator_0: QM31,
    ref range_check_11_sum_1: QM31,
    ref numerator_1: QM31,
    ref sum: QM31,
    random_coeff: QM31,
) -> [QM31; 0] {
    let range_check_29_input = input;

    range_check_18_sum_0 = common_lookup_elements
        .combine_qm31(
            [
                qm31_const::<1109051422, 0, 0, 0>(),
                ((range_check_29_input - range_check_29_bot11bits_col0)
                    * qm31_const::<1048576, 0, 0, 0>()),
            ]
                .span(),
        );
    numerator_0 = qm31_const::<1, 0, 0, 0>();

    range_check_11_sum_1 = common_lookup_elements
        .combine_qm31([qm31_const::<991608089, 0, 0, 0>(), range_check_29_bot11bits_col0].span());
    numerator_1 = qm31_const::<1, 0, 0, 0>();

    []
}
