// This file was created by the AIR team.

use crate::components::subroutines::split_16_low_part_size_8::split_16_low_part_size_8_evaluate;
use crate::prelude::*;


pub fn verify_xor_rot_32_r_8_evaluate(
    input: [QM31; 6],
    ms_8_bits_col0: QM31,
    ms_8_bits_col1: QM31,
    ms_8_bits_col2: QM31,
    ms_8_bits_col3: QM31,
    ms_8_bits_col4: QM31,
    ms_8_bits_col5: QM31,
    common_lookup_elements: @CommonLookupElements,
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
) -> [QM31; 0] {
    let [
        verify_xor_rot_32_r_8_input_limb_0,
        verify_xor_rot_32_r_8_input_limb_1,
        verify_xor_rot_32_r_8_input_limb_2,
        verify_xor_rot_32_r_8_input_limb_3,
        verify_xor_rot_32_r_8_input_limb_4,
        verify_xor_rot_32_r_8_input_limb_5,
    ] =
        input;
    let split_16_low_part_size_8_output_tmp_e2dc4_1_limb_0: QM31 =
        split_16_low_part_size_8_evaluate(
        verify_xor_rot_32_r_8_input_limb_0,
        ms_8_bits_col0,
        common_lookup_elements,
        ref sum,
        random_coeff,
    );
    let split_16_low_part_size_8_output_tmp_e2dc4_3_limb_0: QM31 =
        split_16_low_part_size_8_evaluate(
        verify_xor_rot_32_r_8_input_limb_1,
        ms_8_bits_col1,
        common_lookup_elements,
        ref sum,
        random_coeff,
    );
    let split_16_low_part_size_8_output_tmp_e2dc4_5_limb_0: QM31 =
        split_16_low_part_size_8_evaluate(
        verify_xor_rot_32_r_8_input_limb_2,
        ms_8_bits_col2,
        common_lookup_elements,
        ref sum,
        random_coeff,
    );
    let split_16_low_part_size_8_output_tmp_e2dc4_7_limb_0: QM31 =
        split_16_low_part_size_8_evaluate(
        verify_xor_rot_32_r_8_input_limb_3,
        ms_8_bits_col3,
        common_lookup_elements,
        ref sum,
        random_coeff,
    );
    let split_16_low_part_size_8_output_tmp_e2dc4_9_limb_0: QM31 =
        split_16_low_part_size_8_evaluate(
        verify_xor_rot_32_r_8_input_limb_4,
        ms_8_bits_col4,
        common_lookup_elements,
        ref sum,
        random_coeff,
    );
    let split_16_low_part_size_8_output_tmp_e2dc4_11_limb_0: QM31 =
        split_16_low_part_size_8_evaluate(
        verify_xor_rot_32_r_8_input_limb_5,
        ms_8_bits_col5,
        common_lookup_elements,
        ref sum,
        random_coeff,
    );

    verify_bitwise_xor_8_sum_0 = common_lookup_elements
        .combine_qm31(
            [
                qm31_const::<112558620, 0, 0, 0>(), ms_8_bits_col0, ms_8_bits_col2,
                split_16_low_part_size_8_output_tmp_e2dc4_9_limb_0,
            ]
                .span(),
        );
    numerator_0 = qm31_const::<1, 0, 0, 0>();

    verify_bitwise_xor_8_sum_1 = common_lookup_elements
        .combine_qm31(
            [
                qm31_const::<112558620, 0, 0, 0>(),
                split_16_low_part_size_8_output_tmp_e2dc4_3_limb_0,
                split_16_low_part_size_8_output_tmp_e2dc4_7_limb_0, ms_8_bits_col4,
            ]
                .span(),
        );
    numerator_1 = qm31_const::<1, 0, 0, 0>();

    verify_bitwise_xor_8_sum_2 = common_lookup_elements
        .combine_qm31(
            [
                qm31_const::<112558620, 0, 0, 0>(), ms_8_bits_col1, ms_8_bits_col3,
                split_16_low_part_size_8_output_tmp_e2dc4_11_limb_0,
            ]
                .span(),
        );
    numerator_2 = qm31_const::<1, 0, 0, 0>();

    verify_bitwise_xor_8_sum_3 = common_lookup_elements
        .combine_qm31(
            [
                qm31_const::<112558620, 0, 0, 0>(),
                split_16_low_part_size_8_output_tmp_e2dc4_1_limb_0,
                split_16_low_part_size_8_output_tmp_e2dc4_5_limb_0, ms_8_bits_col5,
            ]
                .span(),
        );
    numerator_3 = qm31_const::<1, 0, 0, 0>();

    []
}
