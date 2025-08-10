// AIR version aca38612
use crate::prelude::*;


pub fn mem_verify_equal_evaluate(
    input: [QM31; 2],
    id_col0: QM31,
    memory_address_to_id_lookup_elements: @crate::MemoryAddressToIdElements,
    ref memory_address_to_id_sum_0: QM31,
    ref memory_address_to_id_sum_1: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 0] {
    let [mem_verify_equal_input_address1, mem_verify_equal_input_address2] = input;

    memory_address_to_id_sum_0 = memory_address_to_id_lookup_elements
        .combine_qm31([mem_verify_equal_input_address1, id_col0]);

    memory_address_to_id_sum_1 = memory_address_to_id_lookup_elements
        .combine_qm31([mem_verify_equal_input_address2, id_col0]);

    []
}
