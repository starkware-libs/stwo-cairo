// This file was created by the AIR team.

use crate::components::subroutines::bitwise_xor_num_bits_8::bitwise_xor_num_bits_8_evaluate;
use crate::components::subroutines::split_16_low_part_size_8::split_16_low_part_size_8_evaluate;
use crate::prelude::*;


pub fn create_blake_round_input_evaluate(
    input: [QM31; 17],
    ms_8_bits_col0: QM31,
    ms_8_bits_col1: QM31,
    xor_col2: QM31,
    xor_col3: QM31,
    xor_col4: QM31,
    xor_col5: QM31,
    common_lookup_elements: @CommonLookupElements,
    t0: QM31,
    t1: QM31,
    ref verify_bitwise_xor_8_sum_0: QM31,
    ref numerator_0: QM31,
    ref verify_bitwise_xor_8_sum_1: QM31,
    ref numerator_1: QM31,
    ref verify_bitwise_xor_8_sum_2: QM31,
    ref numerator_2: QM31,
    ref verify_bitwise_xor_8_sum_3: QM31,
    ref numerator_3: QM31,
    ref sum: QM31,
    random_coeff: QM31,
) -> [QM31; 20] {
    let [
        create_blake_round_input_input_limb_0,
        create_blake_round_input_input_limb_1,
        create_blake_round_input_input_limb_2,
        create_blake_round_input_input_limb_3,
        create_blake_round_input_input_limb_4,
        create_blake_round_input_input_limb_5,
        create_blake_round_input_input_limb_6,
        create_blake_round_input_input_limb_7,
        create_blake_round_input_input_limb_8,
        create_blake_round_input_input_limb_9,
        create_blake_round_input_input_limb_10,
        create_blake_round_input_input_limb_11,
        create_blake_round_input_input_limb_12,
        create_blake_round_input_input_limb_13,
        create_blake_round_input_input_limb_14,
        create_blake_round_input_input_limb_15,
        create_blake_round_input_input_limb_16,
    ] =
        input;
    let split_16_low_part_size_8_output_tmp_4d188_1_limb_0: QM31 =
        split_16_low_part_size_8_evaluate(
        t0, ms_8_bits_col0, common_lookup_elements, ref sum, random_coeff,
    );
    let split_16_low_part_size_8_output_tmp_4d188_3_limb_0: QM31 =
        split_16_low_part_size_8_evaluate(
        t1, ms_8_bits_col1, common_lookup_elements, ref sum, random_coeff,
    );
    bitwise_xor_num_bits_8_evaluate(
        [split_16_low_part_size_8_output_tmp_4d188_1_limb_0, qm31_const::<127, 0, 0, 0>()],
        xor_col2,
        common_lookup_elements,
        ref verify_bitwise_xor_8_sum_0,
        ref numerator_0,
        ref sum,
        random_coeff,
    );
    bitwise_xor_num_bits_8_evaluate(
        [ms_8_bits_col0, qm31_const::<82, 0, 0, 0>()],
        xor_col3,
        common_lookup_elements,
        ref verify_bitwise_xor_8_sum_1,
        ref numerator_1,
        ref sum,
        random_coeff,
    );
    bitwise_xor_num_bits_8_evaluate(
        [split_16_low_part_size_8_output_tmp_4d188_3_limb_0, qm31_const::<14, 0, 0, 0>()],
        xor_col4,
        common_lookup_elements,
        ref verify_bitwise_xor_8_sum_2,
        ref numerator_2,
        ref sum,
        random_coeff,
    );
    bitwise_xor_num_bits_8_evaluate(
        [ms_8_bits_col1, qm31_const::<81, 0, 0, 0>()],
        xor_col5,
        common_lookup_elements,
        ref verify_bitwise_xor_8_sum_3,
        ref numerator_3,
        ref sum,
        random_coeff,
    );

    [
        create_blake_round_input_input_limb_0, create_blake_round_input_input_limb_1,
        create_blake_round_input_input_limb_2, create_blake_round_input_input_limb_3,
        create_blake_round_input_input_limb_4, create_blake_round_input_input_limb_5,
        create_blake_round_input_input_limb_6, create_blake_round_input_input_limb_7,
        create_blake_round_input_input_limb_8, create_blake_round_input_input_limb_9,
        create_blake_round_input_input_limb_10, create_blake_round_input_input_limb_11,
        create_blake_round_input_input_limb_12, create_blake_round_input_input_limb_13,
        create_blake_round_input_input_limb_14, create_blake_round_input_input_limb_15,
        (xor_col2 + (xor_col3 * qm31_const::<256, 0, 0, 0>())),
        (xor_col4 + (xor_col5 * qm31_const::<256, 0, 0, 0>())),
        ((create_blake_round_input_input_limb_16 * qm31_const::<9812, 0, 0, 0>())
            + ((qm31_const::<1, 0, 0, 0>() - create_blake_round_input_input_limb_16)
                * qm31_const::<55723, 0, 0, 0>())),
        ((create_blake_round_input_input_limb_16 * qm31_const::<57468, 0, 0, 0>())
            + ((qm31_const::<1, 0, 0, 0>() - create_blake_round_input_input_limb_16)
                * qm31_const::<8067, 0, 0, 0>())),
    ]
}
