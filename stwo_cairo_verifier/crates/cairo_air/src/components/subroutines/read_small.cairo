// AIR version c4472dc1
use crate::components::subroutines::cond_decode_small_sign::cond_decode_small_sign_evaluate;
use crate::prelude::*;


pub fn read_small_evaluate(
    input: QM31,
    id_col0: QM31,
    msb_col1: QM31,
    mid_limbs_set_col2: QM31,
    value_limb_0_col3: QM31,
    value_limb_1_col4: QM31,
    value_limb_2_col5: QM31,
    memory_address_to_id_lookup_elements: @crate::MemoryAddressToIdElements,
    memory_id_to_big_lookup_elements: @crate::MemoryIdToBigElements,
    ref memory_address_to_id_sum_0: QM31,
    ref memory_id_to_big_sum_1: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> QM31 {
    let read_small_input = input;

    memory_address_to_id_sum_0 = memory_address_to_id_lookup_elements
        .combine_qm31([read_small_input, id_col0]);
    cond_decode_small_sign_evaluate(
        qm31_const::<1, 0, 0, 0>(),
        msb_col1,
        mid_limbs_set_col2,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    memory_id_to_big_sum_1 = memory_id_to_big_lookup_elements
        .combine_qm31(
            [
                id_col0, value_limb_0_col3, value_limb_1_col4, value_limb_2_col5,
                (mid_limbs_set_col2 * qm31_const::<511, 0, 0, 0>()),
                (mid_limbs_set_col2 * qm31_const::<511, 0, 0, 0>()),
                (mid_limbs_set_col2 * qm31_const::<511, 0, 0, 0>()),
                (mid_limbs_set_col2 * qm31_const::<511, 0, 0, 0>()),
                (mid_limbs_set_col2 * qm31_const::<511, 0, 0, 0>()),
                (mid_limbs_set_col2 * qm31_const::<511, 0, 0, 0>()),
                (mid_limbs_set_col2 * qm31_const::<511, 0, 0, 0>()),
                (mid_limbs_set_col2 * qm31_const::<511, 0, 0, 0>()),
                (mid_limbs_set_col2 * qm31_const::<511, 0, 0, 0>()),
                (mid_limbs_set_col2 * qm31_const::<511, 0, 0, 0>()),
                (mid_limbs_set_col2 * qm31_const::<511, 0, 0, 0>()),
                (mid_limbs_set_col2 * qm31_const::<511, 0, 0, 0>()),
                (mid_limbs_set_col2 * qm31_const::<511, 0, 0, 0>()),
                (mid_limbs_set_col2 * qm31_const::<511, 0, 0, 0>()),
                (mid_limbs_set_col2 * qm31_const::<511, 0, 0, 0>()),
                (mid_limbs_set_col2 * qm31_const::<511, 0, 0, 0>()),
                (mid_limbs_set_col2 * qm31_const::<511, 0, 0, 0>()),
                (mid_limbs_set_col2 * qm31_const::<511, 0, 0, 0>()),
                ((qm31_const::<136, 0, 0, 0>() * msb_col1) - mid_limbs_set_col2),
                qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(),
                qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(),
                (msb_col1 * qm31_const::<256, 0, 0, 0>()),
            ],
        );

    ((((value_limb_0_col3 + (value_limb_1_col4 * qm31_const::<512, 0, 0, 0>()))
        + (value_limb_2_col5 * qm31_const::<262144, 0, 0, 0>()))
        - msb_col1)
        - (qm31_const::<134217728, 0, 0, 0>() * mid_limbs_set_col2))
}
