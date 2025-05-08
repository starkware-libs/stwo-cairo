use crate::components::prelude::*;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct CondDecodeSmallSign {}

impl CondDecodeSmallSign {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [cond_decode_small_sign_input_limb_28]: [E::F; 1],
        msb_col0: E::F,
        mid_limbs_set_col1: E::F,
        eval: &mut E,
    ) -> [E::F; 0] {
        let M31_1 = E::F::from(M31::from(1));

        // msb is a bit.
        eval.add_constraint((msb_col0.clone() * (msb_col0.clone() - M31_1.clone())));
        // mid_limbs_set is a bit.
        eval.add_constraint(
            (mid_limbs_set_col1.clone() * (mid_limbs_set_col1.clone() - M31_1.clone())),
        );
        // Cannot have msb equals 0 and mid_limbs_set equals 1.
        eval.add_constraint(
            ((cond_decode_small_sign_input_limb_28.clone() * mid_limbs_set_col1.clone())
                * (msb_col0.clone() - M31_1.clone())),
        );
        []
    }
}
