use crate::components::prelude::*;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct BitwiseXorNumBits7 {}

impl BitwiseXorNumBits7 {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [bitwise_xor_num_bits_7_input_limb_0, bitwise_xor_num_bits_7_input_limb_1]: [E::F; 2],
        xor_col0: E::F,
        eval: &mut E,
        verify_bitwise_xor_7_lookup_elements: &relations::VerifyBitwiseXor_7,
    ) -> E::F {
        eval.add_to_relation(RelationEntry::new(
            verify_bitwise_xor_7_lookup_elements,
            E::EF::one(),
            &[
                bitwise_xor_num_bits_7_input_limb_0.clone(),
                bitwise_xor_num_bits_7_input_limb_1.clone(),
                xor_col0.clone(),
            ],
        ));

        xor_col0.clone()
    }
}
