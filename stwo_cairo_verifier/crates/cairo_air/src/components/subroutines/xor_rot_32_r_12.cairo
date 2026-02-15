// This file was created by the AIR team.

use crate::components::subroutines::bitwise_xor_num_bits_12::bitwise_xor_num_bits_12_evaluate;
use crate::components::subroutines::bitwise_xor_num_bits_4::bitwise_xor_num_bits_4_evaluate;
use crate::components::subroutines::split_16_low_part_size_12::split_16_low_part_size_12_evaluate;
use crate::prelude::*;


pub fn xor_rot_32_r_12_evaluate(
    input: [QM31; 4],
    ms_4_bits_col0: QM31,
    ms_4_bits_col1: QM31,
    ms_4_bits_col2: QM31,
    ms_4_bits_col3: QM31,
    xor_col4: QM31,
    xor_col5: QM31,
    xor_col6: QM31,
    xor_col7: QM31,
    common_lookup_elements: @CommonLookupElements,
    ref verify_bitwise_xor_12_sum_0: QM31,
    ref numerator_0: QM31,
    ref verify_bitwise_xor_4_sum_1: QM31,
    ref numerator_1: QM31,
    ref verify_bitwise_xor_12_sum_2: QM31,
    ref numerator_2: QM31,
    ref verify_bitwise_xor_4_sum_3: QM31,
    ref numerator_3: QM31,
    ref sum: QM31,
    random_coeff: QM31,
) -> [QM31; 2] {
    let [
        xor_rot_32_r_12_input_limb_0,
        xor_rot_32_r_12_input_limb_1,
        xor_rot_32_r_12_input_limb_2,
        xor_rot_32_r_12_input_limb_3,
    ] =
        input;
    let split_16_low_part_size_12_output_tmp_cf62f_1_limb_0: QM31 =
        split_16_low_part_size_12_evaluate(
        xor_rot_32_r_12_input_limb_0, ms_4_bits_col0, common_lookup_elements, ref sum, random_coeff,
    );
    let split_16_low_part_size_12_output_tmp_cf62f_3_limb_0: QM31 =
        split_16_low_part_size_12_evaluate(
        xor_rot_32_r_12_input_limb_1, ms_4_bits_col1, common_lookup_elements, ref sum, random_coeff,
    );
    let split_16_low_part_size_12_output_tmp_cf62f_5_limb_0: QM31 =
        split_16_low_part_size_12_evaluate(
        xor_rot_32_r_12_input_limb_2, ms_4_bits_col2, common_lookup_elements, ref sum, random_coeff,
    );
    let split_16_low_part_size_12_output_tmp_cf62f_7_limb_0: QM31 =
        split_16_low_part_size_12_evaluate(
        xor_rot_32_r_12_input_limb_3, ms_4_bits_col3, common_lookup_elements, ref sum, random_coeff,
    );
    bitwise_xor_num_bits_12_evaluate(
        [
            split_16_low_part_size_12_output_tmp_cf62f_1_limb_0,
            split_16_low_part_size_12_output_tmp_cf62f_5_limb_0,
        ],
        xor_col4,
        common_lookup_elements,
        ref verify_bitwise_xor_12_sum_0,
        ref numerator_0,
        ref sum,
        random_coeff,
    );
    bitwise_xor_num_bits_4_evaluate(
        [ms_4_bits_col0, ms_4_bits_col2],
        xor_col5,
        common_lookup_elements,
        ref verify_bitwise_xor_4_sum_1,
        ref numerator_1,
        ref sum,
        random_coeff,
    );
    bitwise_xor_num_bits_12_evaluate(
        [
            split_16_low_part_size_12_output_tmp_cf62f_3_limb_0,
            split_16_low_part_size_12_output_tmp_cf62f_7_limb_0,
        ],
        xor_col6,
        common_lookup_elements,
        ref verify_bitwise_xor_12_sum_2,
        ref numerator_2,
        ref sum,
        random_coeff,
    );
    bitwise_xor_num_bits_4_evaluate(
        [ms_4_bits_col1, ms_4_bits_col3],
        xor_col7,
        common_lookup_elements,
        ref verify_bitwise_xor_4_sum_3,
        ref numerator_3,
        ref sum,
        random_coeff,
    );
    let xor_rot_12_output_tmp_cf62f_16_limb_0: QM31 = (xor_col5
        + (xor_col6 * qm31_const::<16, 0, 0, 0>()));
    let xor_rot_12_output_tmp_cf62f_16_limb_1: QM31 = (xor_col7
        + (xor_col4 * qm31_const::<16, 0, 0, 0>()));

    [xor_rot_12_output_tmp_cf62f_16_limb_0, xor_rot_12_output_tmp_cf62f_16_limb_1]
}
