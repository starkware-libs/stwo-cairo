// This file was created by the AIR team.

use crate::components::prelude::*;
use crate::components::subroutines::bitwise_xor_num_bits_7::BitwiseXorNumBits7;
use crate::components::subroutines::bitwise_xor_num_bits_9::BitwiseXorNumBits9;
use crate::components::subroutines::split_16_low_part_size_7::Split16LowPartSize7;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct XorRot32R7 {}

impl XorRot32R7 {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [xor_rot_32_r_7_input_limb_0, xor_rot_32_r_7_input_limb_1, xor_rot_32_r_7_input_limb_2, xor_rot_32_r_7_input_limb_3]: [E::F; 4],
        ms_9_bits_col0: E::F,
        ms_9_bits_col1: E::F,
        ms_9_bits_col2: E::F,
        ms_9_bits_col3: E::F,
        xor_col4: E::F,
        xor_col5: E::F,
        xor_col6: E::F,
        xor_col7: E::F,
        verify_bitwise_xor_7_lookup_elements: &relations::VerifyBitwiseXor_7,
        verify_bitwise_xor_9_lookup_elements: &relations::VerifyBitwiseXor_9,
        eval: &mut E,
    ) -> [E::F; 2] {
        let M31_512 = E::F::from(M31::from(512));

        let [split_16_low_part_size_7_output_tmp_e97b9_1_limb_0] = Split16LowPartSize7::evaluate(
            [xor_rot_32_r_7_input_limb_0.clone()],
            ms_9_bits_col0.clone(),
            eval,
        );
        let [split_16_low_part_size_7_output_tmp_e97b9_3_limb_0] = Split16LowPartSize7::evaluate(
            [xor_rot_32_r_7_input_limb_1.clone()],
            ms_9_bits_col1.clone(),
            eval,
        );
        let [split_16_low_part_size_7_output_tmp_e97b9_5_limb_0] = Split16LowPartSize7::evaluate(
            [xor_rot_32_r_7_input_limb_2.clone()],
            ms_9_bits_col2.clone(),
            eval,
        );
        let [split_16_low_part_size_7_output_tmp_e97b9_7_limb_0] = Split16LowPartSize7::evaluate(
            [xor_rot_32_r_7_input_limb_3.clone()],
            ms_9_bits_col3.clone(),
            eval,
        );
        BitwiseXorNumBits7::evaluate(
            [
                split_16_low_part_size_7_output_tmp_e97b9_1_limb_0.clone(),
                split_16_low_part_size_7_output_tmp_e97b9_5_limb_0.clone(),
            ],
            xor_col4.clone(),
            verify_bitwise_xor_7_lookup_elements,
            eval,
        );
        BitwiseXorNumBits9::evaluate(
            [ms_9_bits_col0.clone(), ms_9_bits_col2.clone()],
            xor_col5.clone(),
            verify_bitwise_xor_9_lookup_elements,
            eval,
        );
        BitwiseXorNumBits7::evaluate(
            [
                split_16_low_part_size_7_output_tmp_e97b9_3_limb_0.clone(),
                split_16_low_part_size_7_output_tmp_e97b9_7_limb_0.clone(),
            ],
            xor_col6.clone(),
            verify_bitwise_xor_7_lookup_elements,
            eval,
        );
        BitwiseXorNumBits9::evaluate(
            [ms_9_bits_col1.clone(), ms_9_bits_col3.clone()],
            xor_col7.clone(),
            verify_bitwise_xor_9_lookup_elements,
            eval,
        );
        let xor_rot_7_output_tmp_e97b9_16_limb_0 =
            eval.add_intermediate((xor_col5.clone() + (xor_col6.clone() * M31_512.clone())));
        let xor_rot_7_output_tmp_e97b9_16_limb_1 =
            eval.add_intermediate((xor_col7.clone() + (xor_col4.clone() * M31_512.clone())));
        [
            xor_rot_7_output_tmp_e97b9_16_limb_0.clone(),
            xor_rot_7_output_tmp_e97b9_16_limb_1.clone(),
        ]
    }
}
