// This file was created by the AIR team.

use crate::components::subroutines::read_id::read_id_evaluate;
use crate::prelude::*;


pub fn mem_verify_evaluate(
    input: [QM31; 29],
    id_col0: QM31,
    memory_address_to_id_lookup_elements: @crate::MemoryAddressToIdElements,
    memory_id_to_big_lookup_elements: @crate::MemoryIdToBigElements,
    ref memory_address_to_id_sum_0: QM31,
    ref memory_id_to_big_sum_1: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 0] {
    let [
        mem_verify_input_address,
        mem_verify_input_value_limb_0,
        mem_verify_input_value_limb_1,
        mem_verify_input_value_limb_2,
        mem_verify_input_value_limb_3,
        mem_verify_input_value_limb_4,
        mem_verify_input_value_limb_5,
        mem_verify_input_value_limb_6,
        mem_verify_input_value_limb_7,
        mem_verify_input_value_limb_8,
        mem_verify_input_value_limb_9,
        mem_verify_input_value_limb_10,
        mem_verify_input_value_limb_11,
        mem_verify_input_value_limb_12,
        mem_verify_input_value_limb_13,
        mem_verify_input_value_limb_14,
        mem_verify_input_value_limb_15,
        mem_verify_input_value_limb_16,
        mem_verify_input_value_limb_17,
        mem_verify_input_value_limb_18,
        mem_verify_input_value_limb_19,
        mem_verify_input_value_limb_20,
        mem_verify_input_value_limb_21,
        mem_verify_input_value_limb_22,
        mem_verify_input_value_limb_23,
        mem_verify_input_value_limb_24,
        mem_verify_input_value_limb_25,
        mem_verify_input_value_limb_26,
        mem_verify_input_value_limb_27,
    ] =
        input;
    read_id_evaluate(
        mem_verify_input_address,
        id_col0,
        memory_address_to_id_lookup_elements,
        ref memory_address_to_id_sum_0,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    memory_id_to_big_sum_1 = memory_id_to_big_lookup_elements
        .combine_qm31(
            [
                id_col0, mem_verify_input_value_limb_0, mem_verify_input_value_limb_1,
                mem_verify_input_value_limb_2, mem_verify_input_value_limb_3,
                mem_verify_input_value_limb_4, mem_verify_input_value_limb_5,
                mem_verify_input_value_limb_6, mem_verify_input_value_limb_7,
                mem_verify_input_value_limb_8, mem_verify_input_value_limb_9,
                mem_verify_input_value_limb_10, mem_verify_input_value_limb_11,
                mem_verify_input_value_limb_12, mem_verify_input_value_limb_13,
                mem_verify_input_value_limb_14, mem_verify_input_value_limb_15,
                mem_verify_input_value_limb_16, mem_verify_input_value_limb_17,
                mem_verify_input_value_limb_18, mem_verify_input_value_limb_19,
                mem_verify_input_value_limb_20, mem_verify_input_value_limb_21,
                mem_verify_input_value_limb_22, mem_verify_input_value_limb_23,
                mem_verify_input_value_limb_24, mem_verify_input_value_limb_25,
                mem_verify_input_value_limb_26, mem_verify_input_value_limb_27,
            ],
        );

    []
}
