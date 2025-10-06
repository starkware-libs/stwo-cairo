// AIR version 98896da1-dirty
use crate::components::prelude::*;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct BitwiseNotNumBits16 {}

impl BitwiseNotNumBits16 {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [bitwise_not_num_bits_16_input]: [E::F; 1],
        not_a_col0: E::F,
        verify_bitwise_not_16_lookup_elements: &relations::VerifyBitwiseNot_16,
        eval: &mut E,
    ) -> [E::F; 0] {
        eval.add_to_relation(RelationEntry::new(
            verify_bitwise_not_16_lookup_elements,
            E::EF::one(),
            &[bitwise_not_num_bits_16_input.clone(), not_a_col0.clone()],
        ));

        []
    }
}
