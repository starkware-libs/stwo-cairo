// This file was created by the AIR team.

use crate::components::prelude::*;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct DecodeSmallSign {}

impl DecodeSmallSign {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        []: [E::F; 0],
        msb_col0: E::F,
        mid_limbs_set_col1: E::F,
        eval: &mut E,
    ) -> [E::F; 4] {
        let M31_1 = E::F::from(M31::from(1));
        let M31_136 = E::F::from(M31::from(136));
        let M31_256 = E::F::from(M31::from(256));
        let M31_508 = E::F::from(M31::from(508));
        let M31_511 = E::F::from(M31::from(511));

        // msb is a bit.
        eval.add_constraint((msb_col0.clone() * (msb_col0.clone() - M31_1.clone())));
        // mid_limbs_set is a bit.
        eval.add_constraint(
            (mid_limbs_set_col1.clone() * (mid_limbs_set_col1.clone() - M31_1.clone())),
        );
        // Cannot have msb equals 0 and mid_limbs_set equals 1.
        eval.add_constraint((mid_limbs_set_col1.clone() * (msb_col0.clone() - M31_1.clone())));
        [
            (mid_limbs_set_col1.clone() * M31_508.clone()),
            (mid_limbs_set_col1.clone() * M31_511.clone()),
            ((msb_col0.clone() * M31_136.clone()) - mid_limbs_set_col1.clone()),
            (msb_col0.clone() * M31_256.clone()),
        ]
    }
}
