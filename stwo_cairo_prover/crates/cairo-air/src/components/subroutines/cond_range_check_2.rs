// This file was created by the AIR team.

use crate::components::prelude::*;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct CondRangeCheck2 {}

impl CondRangeCheck2 {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [cond_range_check_2_input_limb_0, cond_range_check_2_input_limb_1]: [E::F; 2],
        partial_limb_msb_col0: E::F,
        eval: &mut E,
    ) -> [E::F; 0] {
        let M31_1 = E::F::from(M31::from(1));
        let M31_2 = E::F::from(M31::from(2));

        // msb is a bit or condition is 0.
        eval.add_constraint(
            ((partial_limb_msb_col0.clone() * (M31_1.clone() - partial_limb_msb_col0.clone()))
                * cond_range_check_2_input_limb_1.clone()),
        );
        let partial_limb_bit_before_msb_tmp_88401_1 = eval.add_intermediate(
            (cond_range_check_2_input_limb_0.clone()
                - (partial_limb_msb_col0.clone() * M31_2.clone())),
        );
        // bit before msb is a bit or condition is 0.
        eval.add_constraint(
            ((partial_limb_bit_before_msb_tmp_88401_1.clone()
                * (M31_1.clone() - partial_limb_bit_before_msb_tmp_88401_1.clone()))
                * cond_range_check_2_input_limb_1.clone()),
        );
        []
    }
}
