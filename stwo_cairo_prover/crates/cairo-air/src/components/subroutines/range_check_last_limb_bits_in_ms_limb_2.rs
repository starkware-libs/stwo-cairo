use crate::components::prelude::*;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct RangeCheckLastLimbBitsInMsLimb2 {}

impl RangeCheckLastLimbBitsInMsLimb2 {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [range_check_last_limb_bits_in_ms_limb_2_input]: [E::F; 1],
        msb_col0: E::F,
        eval: &mut E,
    ) -> [E::F; 0] {
        let M31_1 = E::F::from(M31::from(1));
        let M31_2 = E::F::from(M31::from(2));

        // msb is a bit.
        eval.add_constraint((msb_col0.clone() * (M31_1.clone() - msb_col0.clone())));
        let bit_before_msb_tmp_f851f_1 = eval.add_intermediate(
            (range_check_last_limb_bits_in_ms_limb_2_input.clone()
                - (msb_col0.clone() * M31_2.clone())),
        );
        // bit before msb is a bit.
        eval.add_constraint(
            (bit_before_msb_tmp_f851f_1.clone()
                * (M31_1.clone() - bit_before_msb_tmp_f851f_1.clone())),
        );
        []
    }
}
