// This file was created by the AIR team.

use crate::components::subroutines::range_check_last_limb_bits_in_ms_limb_2::range_check_last_limb_bits_in_ms_limb_2_evaluate;
use crate::prelude::*;


pub fn read_positive_known_id_num_bits_128_evaluate(
    input: QM31,
    value_limb_0_col0: QM31,
    value_limb_1_col1: QM31,
    value_limb_2_col2: QM31,
    value_limb_3_col3: QM31,
    value_limb_4_col4: QM31,
    value_limb_5_col5: QM31,
    value_limb_6_col6: QM31,
    value_limb_7_col7: QM31,
    value_limb_8_col8: QM31,
    value_limb_9_col9: QM31,
    value_limb_10_col10: QM31,
    value_limb_11_col11: QM31,
    value_limb_12_col12: QM31,
    value_limb_13_col13: QM31,
    value_limb_14_col14: QM31,
    partial_limb_msb_col15: QM31,
    memory_id_to_big_lookup_elements: @crate::MemoryIdToBigElements,
    ref memory_id_to_big_sum_0: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 0] {
    let read_positive_known_id_num_bits_128_input = input;
    range_check_last_limb_bits_in_ms_limb_2_evaluate(
        value_limb_14_col14,
        partial_limb_msb_col15,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    memory_id_to_big_sum_0 = memory_id_to_big_lookup_elements
        .combine_qm31(
            [
                read_positive_known_id_num_bits_128_input, value_limb_0_col0, value_limb_1_col1,
                value_limb_2_col2, value_limb_3_col3, value_limb_4_col4, value_limb_5_col5,
                value_limb_6_col6, value_limb_7_col7, value_limb_8_col8, value_limb_9_col9,
                value_limb_10_col10, value_limb_11_col11, value_limb_12_col12, value_limb_13_col13,
                value_limb_14_col14, qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(),
                qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(),
                qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(),
                qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(),
                qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(),
            ],
        );

    []
}
