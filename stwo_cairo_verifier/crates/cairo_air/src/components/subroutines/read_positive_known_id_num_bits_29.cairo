// This file was created by the AIR team.

use crate::components::subroutines::range_check_last_limb_bits_in_ms_limb_2::range_check_last_limb_bits_in_ms_limb_2_evaluate;
use crate::prelude::*;


pub fn read_positive_known_id_num_bits_29_evaluate(
    input: QM31,
    value_limb_0_col0: QM31,
    value_limb_1_col1: QM31,
    value_limb_2_col2: QM31,
    value_limb_3_col3: QM31,
    partial_limb_msb_col4: QM31,
    common_lookup_elements: @CommonLookupElements,
    ref memory_id_to_big_sum_0: QM31,
    ref sum: QM31,
    random_coeff: QM31,
) -> [QM31; 0] {
    let read_positive_known_id_num_bits_29_input = input;
    range_check_last_limb_bits_in_ms_limb_2_evaluate(
        value_limb_3_col3, partial_limb_msb_col4, common_lookup_elements, ref sum, random_coeff,
    );

    memory_id_to_big_sum_0 = common_lookup_elements
        .combine_qm31(
            [
                qm31_const::<1662111297, 0, 0, 0>(), read_positive_known_id_num_bits_29_input,
                value_limb_0_col0, value_limb_1_col1, value_limb_2_col2, value_limb_3_col3,
                qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(),
                qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(),
                qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(),
                qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(),
                qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(),
                qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(),
                qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(),
                qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(),
            ]
                .span(),
        );

    []
}
