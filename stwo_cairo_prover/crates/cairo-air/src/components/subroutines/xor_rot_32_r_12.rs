// This file was created by the AIR team.

use crate::components::prelude::*;
use crate::components::subroutines::bitwise_xor_num_bits_12::BitwiseXorNumBits12;
use crate::components::subroutines::bitwise_xor_num_bits_4::BitwiseXorNumBits4;
use crate::components::subroutines::split_16_low_part_size_12::Split16LowPartSize12;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct XorRot32R12 {}

impl XorRot32R12 {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [xor_rot_32_r_12_input_limb_0, xor_rot_32_r_12_input_limb_1, xor_rot_32_r_12_input_limb_2, xor_rot_32_r_12_input_limb_3]: [E::F; 4],
        ms_4_bits_col0: E::F,
        ms_4_bits_col1: E::F,
        ms_4_bits_col2: E::F,
        ms_4_bits_col3: E::F,
        xor_col4: E::F,
        xor_col5: E::F,
        xor_col6: E::F,
        xor_col7: E::F,
        common_lookup_elements: &relations::CommonLookupElements,
        eval: &mut E,
    ) -> [E::F; 2] {
        let M31_16 = E::F::from(M31::from(16));

        let [split_16_low_part_size_12_output_tmp_cf62f_1_limb_0] = Split16LowPartSize12::evaluate(
            [xor_rot_32_r_12_input_limb_0.clone()],
            ms_4_bits_col0.clone(),
            common_lookup_elements,
            eval,
        );
        let [split_16_low_part_size_12_output_tmp_cf62f_3_limb_0] = Split16LowPartSize12::evaluate(
            [xor_rot_32_r_12_input_limb_1.clone()],
            ms_4_bits_col1.clone(),
            common_lookup_elements,
            eval,
        );
        let [split_16_low_part_size_12_output_tmp_cf62f_5_limb_0] = Split16LowPartSize12::evaluate(
            [xor_rot_32_r_12_input_limb_2.clone()],
            ms_4_bits_col2.clone(),
            common_lookup_elements,
            eval,
        );
        let [split_16_low_part_size_12_output_tmp_cf62f_7_limb_0] = Split16LowPartSize12::evaluate(
            [xor_rot_32_r_12_input_limb_3.clone()],
            ms_4_bits_col3.clone(),
            common_lookup_elements,
            eval,
        );
        BitwiseXorNumBits12::evaluate(
            [
                split_16_low_part_size_12_output_tmp_cf62f_1_limb_0.clone(),
                split_16_low_part_size_12_output_tmp_cf62f_5_limb_0.clone(),
            ],
            xor_col4.clone(),
            common_lookup_elements,
            eval,
        );
        BitwiseXorNumBits4::evaluate(
            [ms_4_bits_col0.clone(), ms_4_bits_col2.clone()],
            xor_col5.clone(),
            common_lookup_elements,
            eval,
        );
        BitwiseXorNumBits12::evaluate(
            [
                split_16_low_part_size_12_output_tmp_cf62f_3_limb_0.clone(),
                split_16_low_part_size_12_output_tmp_cf62f_7_limb_0.clone(),
            ],
            xor_col6.clone(),
            common_lookup_elements,
            eval,
        );
        BitwiseXorNumBits4::evaluate(
            [ms_4_bits_col1.clone(), ms_4_bits_col3.clone()],
            xor_col7.clone(),
            common_lookup_elements,
            eval,
        );
        let xor_rot_12_output_tmp_cf62f_16_limb_0 =
            eval.add_intermediate((xor_col5.clone() + (xor_col6.clone() * M31_16.clone())));
        let xor_rot_12_output_tmp_cf62f_16_limb_1 =
            eval.add_intermediate((xor_col7.clone() + (xor_col4.clone() * M31_16.clone())));
        [
            xor_rot_12_output_tmp_cf62f_16_limb_0.clone(),
            xor_rot_12_output_tmp_cf62f_16_limb_1.clone(),
        ]
    }
}
