// This file was created by the AIR team.

use crate::components::prelude::*;
use crate::components::subroutines::bitwise_xor_num_bits_8::BitwiseXorNumBits8;
use crate::components::subroutines::bitwise_xor_num_bits_8_b::BitwiseXorNumBits8B;
use crate::components::subroutines::split_16_low_part_size_8::Split16LowPartSize8;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct XorRot32R8 {}

impl XorRot32R8 {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [xor_rot_32_r_8_input_limb_0, xor_rot_32_r_8_input_limb_1, xor_rot_32_r_8_input_limb_2, xor_rot_32_r_8_input_limb_3]: [E::F; 4],
        ms_8_bits_col0: E::F,
        ms_8_bits_col1: E::F,
        ms_8_bits_col2: E::F,
        ms_8_bits_col3: E::F,
        xor_col4: E::F,
        xor_col5: E::F,
        xor_col6: E::F,
        xor_col7: E::F,
        verify_bitwise_xor_8_lookup_elements: &relations::VerifyBitwiseXor_8,
        verify_bitwise_xor_8_b_lookup_elements: &relations::VerifyBitwiseXor_8_B,
        eval: &mut E,
    ) -> [E::F; 2] {
        let M31_256 = E::F::from(M31::from(256));

        let [split_16_low_part_size_8_output_tmp_aa6bd_1_limb_0] = Split16LowPartSize8::evaluate(
            [xor_rot_32_r_8_input_limb_0.clone()],
            ms_8_bits_col0.clone(),
            eval,
        );
        let [split_16_low_part_size_8_output_tmp_aa6bd_3_limb_0] = Split16LowPartSize8::evaluate(
            [xor_rot_32_r_8_input_limb_1.clone()],
            ms_8_bits_col1.clone(),
            eval,
        );
        let [split_16_low_part_size_8_output_tmp_aa6bd_5_limb_0] = Split16LowPartSize8::evaluate(
            [xor_rot_32_r_8_input_limb_2.clone()],
            ms_8_bits_col2.clone(),
            eval,
        );
        let [split_16_low_part_size_8_output_tmp_aa6bd_7_limb_0] = Split16LowPartSize8::evaluate(
            [xor_rot_32_r_8_input_limb_3.clone()],
            ms_8_bits_col3.clone(),
            eval,
        );
        BitwiseXorNumBits8::evaluate(
            [
                split_16_low_part_size_8_output_tmp_aa6bd_1_limb_0.clone(),
                split_16_low_part_size_8_output_tmp_aa6bd_5_limb_0.clone(),
            ],
            xor_col4.clone(),
            verify_bitwise_xor_8_lookup_elements,
            eval,
        );
        BitwiseXorNumBits8::evaluate(
            [ms_8_bits_col0.clone(), ms_8_bits_col2.clone()],
            xor_col5.clone(),
            verify_bitwise_xor_8_lookup_elements,
            eval,
        );
        BitwiseXorNumBits8B::evaluate(
            [
                split_16_low_part_size_8_output_tmp_aa6bd_3_limb_0.clone(),
                split_16_low_part_size_8_output_tmp_aa6bd_7_limb_0.clone(),
            ],
            xor_col6.clone(),
            verify_bitwise_xor_8_b_lookup_elements,
            eval,
        );
        BitwiseXorNumBits8B::evaluate(
            [ms_8_bits_col1.clone(), ms_8_bits_col3.clone()],
            xor_col7.clone(),
            verify_bitwise_xor_8_b_lookup_elements,
            eval,
        );
        let xor_rot_8_output_tmp_aa6bd_16_limb_0 =
            eval.add_intermediate((xor_col5.clone() + (xor_col6.clone() * M31_256.clone())));
        let xor_rot_8_output_tmp_aa6bd_16_limb_1 =
            eval.add_intermediate((xor_col7.clone() + (xor_col4.clone() * M31_256.clone())));
        [
            xor_rot_8_output_tmp_aa6bd_16_limb_0.clone(),
            xor_rot_8_output_tmp_aa6bd_16_limb_1.clone(),
        ]
    }
}
