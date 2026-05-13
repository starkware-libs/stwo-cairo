// This file was created by the AIR team.

use subroutines::verify_triple_sum_32::VerifyTripleSum32;

use crate::components::prelude::*;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct TripleSum32 {}

impl TripleSum32 {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [triple_sum_32_input_a_limb_0, triple_sum_32_input_a_limb_1, triple_sum_32_input_b_limb_0, triple_sum_32_input_b_limb_1, triple_sum_32_input_c_limb_0, triple_sum_32_input_c_limb_1]: [E::F; 6],
        triple_sum32_res_limb_0_col0: E::F,
        triple_sum32_res_limb_1_col1: E::F,
        common_lookup_elements: &relations::CommonLookupElements,
        eval: &mut E,
    ) -> [E::F; 0] {
        VerifyTripleSum32::evaluate(
            [
                triple_sum_32_input_a_limb_0.clone(),
                triple_sum_32_input_a_limb_1.clone(),
                triple_sum_32_input_b_limb_0.clone(),
                triple_sum_32_input_b_limb_1.clone(),
                triple_sum_32_input_c_limb_0.clone(),
                triple_sum_32_input_c_limb_1.clone(),
                triple_sum32_res_limb_0_col0.clone(),
                triple_sum32_res_limb_1_col1.clone(),
            ],
            common_lookup_elements,
            eval,
        );
        []
    }
}
