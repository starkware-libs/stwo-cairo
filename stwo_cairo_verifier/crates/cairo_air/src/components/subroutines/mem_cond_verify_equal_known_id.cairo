// AIR version aca38612
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

    memory_address_to_id_sum_0 = memory_address_to_id_lookup_elements
        .combine_qm31([mem_cond_verify_equal_known_id_input_limb_0, id_col0]);

    // Constraint - The two ids are equal if the condition is met
    let constraint_quotient = (((id_col0 - mem_cond_verify_equal_known_id_input_limb_1)
        * mem_cond_verify_equal_known_id_input_limb_2))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    []
}
