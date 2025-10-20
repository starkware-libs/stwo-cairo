// This file was created by the AIR team.

use crate::components::prelude::*;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct BitwiseXorNumBits8B {}

impl BitwiseXorNumBits8B {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [bitwise_xor_num_bits_8_b_input_limb_0, bitwise_xor_num_bits_8_b_input_limb_1]: [E::F; 2],
        xor_col0: E::F,
        verify_bitwise_xor_8_b_lookup_elements: &relations::VerifyBitwiseXor_8_B,
        eval: &mut E,
    ) -> [E::F; 0] {
        eval.add_to_relation(RelationEntry::new(
            verify_bitwise_xor_8_b_lookup_elements,
            E::EF::one(),
            &[
                bitwise_xor_num_bits_8_b_input_limb_0.clone(),
                bitwise_xor_num_bits_8_b_input_limb_1.clone(),
                xor_col0.clone(),
            ],
        ));

        []
    }
}
