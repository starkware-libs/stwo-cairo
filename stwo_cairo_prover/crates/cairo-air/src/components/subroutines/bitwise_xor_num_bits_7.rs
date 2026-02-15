// This file was created by the AIR team.

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
        common_lookup_elements: &relations::CommonLookupElements,
        eval: &mut E,
    ) -> [E::F; 0] {
        let M31_1 = E::F::from(M31::from(1));
        let M31_62225763 = E::F::from(M31::from(62225763));

        eval.add_to_relation(RelationEntry::new(
            common_lookup_elements,
            E::EF::from(M31_1.clone()),
            &[
                M31_62225763.clone(),
                bitwise_xor_num_bits_7_input_limb_0.clone(),
                bitwise_xor_num_bits_7_input_limb_1.clone(),
                xor_col0.clone(),
            ],
        ));

        []
    }
}
