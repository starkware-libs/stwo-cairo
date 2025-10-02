// AIR version 98896da1
use crate::components::prelude::*;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct BitwiseAndNumBits16 {}

impl BitwiseAndNumBits16 {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [bitwise_and_num_bits_16_input_limb_0, bitwise_and_num_bits_16_input_limb_1]: [E::F; 2],
        and_col0: E::F,
        verify_bitwise_and_16_lookup_elements: &relations::VerifyBitwiseAnd_16,
        eval: &mut E,
    ) -> [E::F; 0] {
        eval.add_to_relation(RelationEntry::new(
            verify_bitwise_and_16_lookup_elements,
            E::EF::one(),
            &[
                bitwise_and_num_bits_16_input_limb_0.clone(),
                bitwise_and_num_bits_16_input_limb_1.clone(),
                and_col0.clone(),
            ],
        ));

        []
    }
}
