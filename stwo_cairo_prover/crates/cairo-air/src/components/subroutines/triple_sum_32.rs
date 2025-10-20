// This file was created by the AIR team.

use crate::components::prelude::*;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
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
        eval: &mut E,
    ) -> [E::F; 0] {
        let M31_1 = E::F::from(M31::from(1));
        let M31_2 = E::F::from(M31::from(2));
        let M31_32768 = E::F::from(M31::from(32768));

        let carry_low_tmp_541fa_1 = eval.add_intermediate(
            ((((triple_sum_32_input_a_limb_0.clone() + triple_sum_32_input_b_limb_0.clone())
                + triple_sum_32_input_c_limb_0.clone())
                - triple_sum32_res_limb_0_col0.clone())
                * M31_32768.clone()),
        );
        // carry low is 0 or 1 or 2.
        eval.add_constraint(
            ((carry_low_tmp_541fa_1.clone() * (carry_low_tmp_541fa_1.clone() - M31_1.clone()))
                * (carry_low_tmp_541fa_1.clone() - M31_2.clone())),
        );
        let carry_high_tmp_541fa_2 = eval.add_intermediate(
            (((((triple_sum_32_input_a_limb_1.clone() + triple_sum_32_input_b_limb_1.clone())
                + triple_sum_32_input_c_limb_1.clone())
                + carry_low_tmp_541fa_1.clone())
                - triple_sum32_res_limb_1_col1.clone())
                * M31_32768.clone()),
        );
        // carry high is 0 or 1 or 2.
        eval.add_constraint(
            ((carry_high_tmp_541fa_2.clone() * (carry_high_tmp_541fa_2.clone() - M31_1.clone()))
                * (carry_high_tmp_541fa_2.clone() - M31_2.clone())),
        );
        []
    }
}
