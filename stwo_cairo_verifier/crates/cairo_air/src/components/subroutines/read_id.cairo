// This file was created by the AIR team.

use crate::prelude::*;


pub fn read_id_evaluate(
    input: QM31,
    id_col0: QM31,
    memory_address_to_id_lookup_elements: @crate::MemoryAddressToIdElements,
    ref memory_address_to_id_sum_0: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 0] {
    let read_id_input = input;

    memory_address_to_id_sum_0 = memory_address_to_id_lookup_elements
        .combine_qm31([read_id_input, id_col0]);

    []
}
