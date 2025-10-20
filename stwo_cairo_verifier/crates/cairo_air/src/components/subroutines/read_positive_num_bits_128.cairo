// This file was created by the AIR team.

use crate::components::subroutines::read_id::read_id_evaluate;
use crate::components::subroutines::read_positive_known_id_num_bits_128::read_positive_known_id_num_bits_128_evaluate;
use crate::prelude::*;


pub fn read_positive_num_bits_128_evaluate(
    input: QM31,
    id_col0: QM31,
    value_limb_0_col1: QM31,
    value_limb_1_col2: QM31,
    value_limb_2_col3: QM31,
    value_limb_3_col4: QM31,
    value_limb_4_col5: QM31,
    value_limb_5_col6: QM31,
    value_limb_6_col7: QM31,
    value_limb_7_col8: QM31,
    value_limb_8_col9: QM31,
    value_limb_9_col10: QM31,
    value_limb_10_col11: QM31,
    value_limb_11_col12: QM31,
    value_limb_12_col13: QM31,
    value_limb_13_col14: QM31,
    value_limb_14_col15: QM31,
    partial_limb_msb_col16: QM31,
    memory_address_to_id_lookup_elements: @crate::MemoryAddressToIdElements,
    memory_id_to_big_lookup_elements: @crate::MemoryIdToBigElements,
    ref memory_address_to_id_sum_0: QM31,
    ref memory_id_to_big_sum_1: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 0] {
    let read_positive_num_bits_128_input = input;
    read_id_evaluate(
        read_positive_num_bits_128_input,
        id_col0,
        memory_address_to_id_lookup_elements,
        ref memory_address_to_id_sum_0,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );
    read_positive_known_id_num_bits_128_evaluate(
        id_col0,
        value_limb_0_col1,
        value_limb_1_col2,
        value_limb_2_col3,
        value_limb_3_col4,
        value_limb_4_col5,
        value_limb_5_col6,
        value_limb_6_col7,
        value_limb_7_col8,
        value_limb_8_col9,
        value_limb_9_col10,
        value_limb_10_col11,
        value_limb_11_col12,
        value_limb_12_col13,
        value_limb_13_col14,
        value_limb_14_col15,
        partial_limb_msb_col16,
        memory_id_to_big_lookup_elements,
        ref memory_id_to_big_sum_1,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    []
}
