// AIR version 98896da1-dirty
use crate::prelude::*;




pub fn read_positive_known_id_num_bits_36_evaluate(
    input: QM31,
    value_limb_0_col0: QM31,value_limb_1_col1: QM31,value_limb_2_col2: QM31,value_limb_3_col3: QM31,memory_id_to_big_lookup_elements: @crate::MemoryIdToBigElements,ref memory_id_to_big_sum_0: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 0] {
    let read_positive_known_id_num_bits_36_input = input;
    

    memory_id_to_big_sum_0 = memory_id_to_big_lookup_elements.combine_qm31(
        [
            read_positive_known_id_num_bits_36_input,
value_limb_0_col0,
value_limb_1_col1,
value_limb_2_col2,
value_limb_3_col3,
qm31_const::<0, 0, 0, 0>(),
qm31_const::<0, 0, 0, 0>(),
qm31_const::<0, 0, 0, 0>(),
qm31_const::<0, 0, 0, 0>(),
qm31_const::<0, 0, 0, 0>(),
qm31_const::<0, 0, 0, 0>(),
qm31_const::<0, 0, 0, 0>(),
qm31_const::<0, 0, 0, 0>(),
qm31_const::<0, 0, 0, 0>(),
qm31_const::<0, 0, 0, 0>(),
qm31_const::<0, 0, 0, 0>(),
qm31_const::<0, 0, 0, 0>(),
qm31_const::<0, 0, 0, 0>(),
qm31_const::<0, 0, 0, 0>(),
qm31_const::<0, 0, 0, 0>(),
qm31_const::<0, 0, 0, 0>(),
qm31_const::<0, 0, 0, 0>(),
qm31_const::<0, 0, 0, 0>(),
qm31_const::<0, 0, 0, 0>(),
qm31_const::<0, 0, 0, 0>(),
qm31_const::<0, 0, 0, 0>(),
qm31_const::<0, 0, 0, 0>(),
qm31_const::<0, 0, 0, 0>(),
qm31_const::<0, 0, 0, 0>()
        ],
    );

    []
}