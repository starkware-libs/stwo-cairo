// This file was created by the AIR team.

use crate::prelude::*;


pub fn mem_verify_cond_evaluate(
    input: [QM31; 30],
    id_col0: QM31,
    common_lookup_elements: @CommonLookupElements,
    ref memory_address_to_id_sum_0: QM31,
    ref numerator_0: QM31,
    ref memory_id_to_big_sum_1: QM31,
    ref numerator_1: QM31,
    ref sum: QM31,
    random_coeff: QM31,
) -> [QM31; 0] {
    let [
        mem_verify_cond_input_address,
        mem_verify_cond_input_value_limb_0,
        mem_verify_cond_input_value_limb_1,
        mem_verify_cond_input_value_limb_2,
        mem_verify_cond_input_value_limb_3,
        mem_verify_cond_input_value_limb_4,
        mem_verify_cond_input_value_limb_5,
        mem_verify_cond_input_value_limb_6,
        mem_verify_cond_input_value_limb_7,
        mem_verify_cond_input_value_limb_8,
        mem_verify_cond_input_value_limb_9,
        mem_verify_cond_input_value_limb_10,
        mem_verify_cond_input_value_limb_11,
        mem_verify_cond_input_value_limb_12,
        mem_verify_cond_input_value_limb_13,
        mem_verify_cond_input_value_limb_14,
        mem_verify_cond_input_value_limb_15,
        mem_verify_cond_input_value_limb_16,
        mem_verify_cond_input_value_limb_17,
        mem_verify_cond_input_value_limb_18,
        mem_verify_cond_input_value_limb_19,
        mem_verify_cond_input_value_limb_20,
        mem_verify_cond_input_value_limb_21,
        mem_verify_cond_input_value_limb_22,
        mem_verify_cond_input_value_limb_23,
        mem_verify_cond_input_value_limb_24,
        mem_verify_cond_input_value_limb_25,
        mem_verify_cond_input_value_limb_26,
        mem_verify_cond_input_value_limb_27,
        mem_verify_cond_input_cond,
    ] =
        input;

    // Constraint - cond=0 or cond=1.
    let constraint_quotient = ((mem_verify_cond_input_cond
        * (qm31_const::<1, 0, 0, 0>() - mem_verify_cond_input_cond)));
    sum = sum * random_coeff + constraint_quotient;

    memory_address_to_id_sum_0 = common_lookup_elements
        .combine_qm31(
            [qm31_const::<1444891767, 0, 0, 0>(), mem_verify_cond_input_address, id_col0].span(),
        );
    numerator_0 = mem_verify_cond_input_cond;

    memory_id_to_big_sum_1 = common_lookup_elements
        .combine_qm31(
            [
                qm31_const::<1662111297, 0, 0, 0>(), id_col0, mem_verify_cond_input_value_limb_0,
                mem_verify_cond_input_value_limb_1, mem_verify_cond_input_value_limb_2,
                mem_verify_cond_input_value_limb_3, mem_verify_cond_input_value_limb_4,
                mem_verify_cond_input_value_limb_5, mem_verify_cond_input_value_limb_6,
                mem_verify_cond_input_value_limb_7, mem_verify_cond_input_value_limb_8,
                mem_verify_cond_input_value_limb_9, mem_verify_cond_input_value_limb_10,
                mem_verify_cond_input_value_limb_11, mem_verify_cond_input_value_limb_12,
                mem_verify_cond_input_value_limb_13, mem_verify_cond_input_value_limb_14,
                mem_verify_cond_input_value_limb_15, mem_verify_cond_input_value_limb_16,
                mem_verify_cond_input_value_limb_17, mem_verify_cond_input_value_limb_18,
                mem_verify_cond_input_value_limb_19, mem_verify_cond_input_value_limb_20,
                mem_verify_cond_input_value_limb_21, mem_verify_cond_input_value_limb_22,
                mem_verify_cond_input_value_limb_23, mem_verify_cond_input_value_limb_24,
                mem_verify_cond_input_value_limb_25, mem_verify_cond_input_value_limb_26,
                mem_verify_cond_input_value_limb_27,
            ]
                .span(),
        );
    numerator_1 = mem_verify_cond_input_cond;

    []
}
