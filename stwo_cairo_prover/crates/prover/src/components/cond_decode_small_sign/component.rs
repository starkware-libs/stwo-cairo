use crate::components::prelude::constraint_eval::*;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct CondDecodeSmallSign {}

impl CondDecodeSmallSign {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    pub fn evaluate<E: EvalAtRow>(
        [cond_decode_small_sign_input_limb_0, cond_decode_small_sign_input_limb_1, cond_decode_small_sign_input_limb_2, cond_decode_small_sign_input_limb_3, cond_decode_small_sign_input_limb_4, cond_decode_small_sign_input_limb_5, cond_decode_small_sign_input_limb_6, cond_decode_small_sign_input_limb_7, cond_decode_small_sign_input_limb_8, cond_decode_small_sign_input_limb_9, cond_decode_small_sign_input_limb_10, cond_decode_small_sign_input_limb_11, cond_decode_small_sign_input_limb_12, cond_decode_small_sign_input_limb_13, cond_decode_small_sign_input_limb_14, cond_decode_small_sign_input_limb_15, cond_decode_small_sign_input_limb_16, cond_decode_small_sign_input_limb_17, cond_decode_small_sign_input_limb_18, cond_decode_small_sign_input_limb_19, cond_decode_small_sign_input_limb_20, cond_decode_small_sign_input_limb_21, cond_decode_small_sign_input_limb_22, cond_decode_small_sign_input_limb_23, cond_decode_small_sign_input_limb_24, cond_decode_small_sign_input_limb_25, cond_decode_small_sign_input_limb_26, cond_decode_small_sign_input_limb_27, cond_decode_small_sign_input_limb_28]: [E::F; 29],
        msb_col0: E::F,
        mid_limbs_set_col1: E::F,
        eval: &mut E,
    ) -> [E::F; 2] {
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
        [msb_col0.clone(), mid_limbs_set_col1.clone()]
    }
}
