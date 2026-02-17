// This file was created by the AIR team.

use crate::prelude::*;


pub fn range_check_last_limb_bits_in_ms_limb_6_evaluate(
    input: QM31,
    common_lookup_elements: @CommonLookupElements,
    ref range_check_6_sum_0: QM31,
    ref numerator_0: QM31,
    ref sum: QM31,
    random_coeff: QM31,
) -> [QM31; 0] {
    let range_check_last_limb_bits_in_ms_limb_6_input = input;

    range_check_6_sum_0 = common_lookup_elements
        .combine_qm31(
            [qm31_const::<1185356339, 0, 0, 0>(), range_check_last_limb_bits_in_ms_limb_6_input]
                .span(),
        );
    numerator_0 = qm31_const::<1, 0, 0, 0>();

    []
}
