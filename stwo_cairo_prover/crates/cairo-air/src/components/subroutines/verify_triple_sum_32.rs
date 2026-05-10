// This file was created by the AIR team.

use crate::components::prelude::*;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct VerifyTripleSum32 {}

impl VerifyTripleSum32 {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [verify_triple_sum_32_input_limb_0, verify_triple_sum_32_input_limb_1, verify_triple_sum_32_input_limb_2, verify_triple_sum_32_input_limb_3, verify_triple_sum_32_input_limb_4, verify_triple_sum_32_input_limb_5, verify_triple_sum_32_input_limb_6, verify_triple_sum_32_input_limb_7]: [E::F; 8],
        common_lookup_elements: &relations::CommonLookupElements,
        eval: &mut E,
    ) -> [E::F; 0] {
        let M31_1 = E::F::from(M31::from(1));
        let M31_2 = E::F::from(M31::from(2));
        let M31_32768 = E::F::from(M31::from(32768));

        let carry_low_tmp_a7b7a_0 = eval.add_intermediate(
            ((((verify_triple_sum_32_input_limb_0.clone()
                + verify_triple_sum_32_input_limb_2.clone())
                + verify_triple_sum_32_input_limb_4.clone())
                - verify_triple_sum_32_input_limb_6.clone())
                * M31_32768.clone()),
        );
        // carry low is 0 or 1 or 2.
        eval.add_constraint(
            ((carry_low_tmp_a7b7a_0.clone() * (carry_low_tmp_a7b7a_0.clone() - M31_1.clone()))
                * (carry_low_tmp_a7b7a_0.clone() - M31_2.clone())),
        );
        let carry_high_tmp_a7b7a_1 = eval.add_intermediate(
            (((((verify_triple_sum_32_input_limb_1.clone()
                + verify_triple_sum_32_input_limb_3.clone())
                + verify_triple_sum_32_input_limb_5.clone())
                + carry_low_tmp_a7b7a_0.clone())
                - verify_triple_sum_32_input_limb_7.clone())
                * M31_32768.clone()),
        );
        // carry high is 0 or 1 or 2.
        eval.add_constraint(
            ((carry_high_tmp_a7b7a_1.clone() * (carry_high_tmp_a7b7a_1.clone() - M31_1.clone()))
                * (carry_high_tmp_a7b7a_1.clone() - M31_2.clone())),
        );
        []
    }
}
