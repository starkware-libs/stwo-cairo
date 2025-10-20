// This file was created by the AIR team.

use crate::components::prelude::*;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct BitwiseXorNumBits9 {}

impl BitwiseXorNumBits9 {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [bitwise_xor_num_bits_9_input_limb_0, bitwise_xor_num_bits_9_input_limb_1]: [E::F; 2],
        xor_col0: E::F,
        verify_bitwise_xor_9_lookup_elements: &relations::VerifyBitwiseXor_9,
        eval: &mut E,
    ) -> [E::F; 0] {
        eval.add_to_relation(RelationEntry::new(
            verify_bitwise_xor_9_lookup_elements,
            E::EF::one(),
            &[
                bitwise_xor_num_bits_9_input_limb_0.clone(),
                bitwise_xor_num_bits_9_input_limb_1.clone(),
                xor_col0.clone(),
            ],
        ));

        []
    }
}
