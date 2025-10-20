// This file was created by the AIR team.

use crate::components::subroutines::read_id::read_id_evaluate;
use crate::prelude::*;


pub fn mem_cond_verify_equal_known_id_evaluate(
    input: [QM31; 3],
    id_col0: QM31,
    memory_address_to_id_lookup_elements: @crate::MemoryAddressToIdElements,
    ref memory_address_to_id_sum_0: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 0] {
    let [
        mem_cond_verify_equal_known_id_input_limb_0,
        mem_cond_verify_equal_known_id_input_limb_1,
        mem_cond_verify_equal_known_id_input_limb_2,
    ] =
        input;
    read_id_evaluate(
        mem_cond_verify_equal_known_id_input_limb_0,
        id_col0,
        memory_address_to_id_lookup_elements,
        ref memory_address_to_id_sum_0,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    // Constraint - The two ids are equal if the condition is met
    let constraint_quotient = (((id_col0 - mem_cond_verify_equal_known_id_input_limb_1)
        * mem_cond_verify_equal_known_id_input_limb_2))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    []
}
