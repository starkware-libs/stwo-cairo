// This file was created by the AIR team.

use crate::components::prelude::*;
use crate::components::subroutines::cond_range_check_2::CondRangeCheck2;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct CondFelt252AsAddr {}

impl CondFelt252AsAddr {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [cond_felt_252_as_addr_input_limb_0, cond_felt_252_as_addr_input_limb_1, cond_felt_252_as_addr_input_limb_2, cond_felt_252_as_addr_input_limb_3, cond_felt_252_as_addr_input_limb_4, cond_felt_252_as_addr_input_limb_5, cond_felt_252_as_addr_input_limb_6, cond_felt_252_as_addr_input_limb_7, cond_felt_252_as_addr_input_limb_8, cond_felt_252_as_addr_input_limb_9, cond_felt_252_as_addr_input_limb_10, cond_felt_252_as_addr_input_limb_11, cond_felt_252_as_addr_input_limb_12, cond_felt_252_as_addr_input_limb_13, cond_felt_252_as_addr_input_limb_14, cond_felt_252_as_addr_input_limb_15, cond_felt_252_as_addr_input_limb_16, cond_felt_252_as_addr_input_limb_17, cond_felt_252_as_addr_input_limb_18, cond_felt_252_as_addr_input_limb_19, cond_felt_252_as_addr_input_limb_20, cond_felt_252_as_addr_input_limb_21, cond_felt_252_as_addr_input_limb_22, cond_felt_252_as_addr_input_limb_23, cond_felt_252_as_addr_input_limb_24, cond_felt_252_as_addr_input_limb_25, cond_felt_252_as_addr_input_limb_26, cond_felt_252_as_addr_input_limb_27, cond_felt_252_as_addr_input_limb_28]: [E::F; 29],
        partial_limb_msb_col0: E::F,
        eval: &mut E,
    ) -> [E::F; 1] {
        let M31_134217728 = E::F::from(M31::from(134217728));
        let M31_262144 = E::F::from(M31::from(262144));
        let M31_512 = E::F::from(M31::from(512));

        // When the condition holds, the high limbs must be zero for an address.
        eval.add_constraint(
            (cond_felt_252_as_addr_input_limb_28.clone()
                * (((((((((((((((((((((((cond_felt_252_as_addr_input_limb_4
                    .clone()
                    + cond_felt_252_as_addr_input_limb_5.clone())
                    + cond_felt_252_as_addr_input_limb_6.clone())
                    + cond_felt_252_as_addr_input_limb_7.clone())
                    + cond_felt_252_as_addr_input_limb_8.clone())
                    + cond_felt_252_as_addr_input_limb_9.clone())
                    + cond_felt_252_as_addr_input_limb_10.clone())
                    + cond_felt_252_as_addr_input_limb_11.clone())
                    + cond_felt_252_as_addr_input_limb_12.clone())
                    + cond_felt_252_as_addr_input_limb_13.clone())
                    + cond_felt_252_as_addr_input_limb_14.clone())
                    + cond_felt_252_as_addr_input_limb_15.clone())
                    + cond_felt_252_as_addr_input_limb_16.clone())
                    + cond_felt_252_as_addr_input_limb_17.clone())
                    + cond_felt_252_as_addr_input_limb_18.clone())
                    + cond_felt_252_as_addr_input_limb_19.clone())
                    + cond_felt_252_as_addr_input_limb_20.clone())
                    + cond_felt_252_as_addr_input_limb_21.clone())
                    + cond_felt_252_as_addr_input_limb_22.clone())
                    + cond_felt_252_as_addr_input_limb_23.clone())
                    + cond_felt_252_as_addr_input_limb_24.clone())
                    + cond_felt_252_as_addr_input_limb_25.clone())
                    + cond_felt_252_as_addr_input_limb_26.clone())
                    + cond_felt_252_as_addr_input_limb_27.clone())),
        );
        CondRangeCheck2::evaluate(
            [
                cond_felt_252_as_addr_input_limb_3.clone(),
                cond_felt_252_as_addr_input_limb_28.clone(),
            ],
            partial_limb_msb_col0.clone(),
            eval,
        );
        [(((cond_felt_252_as_addr_input_limb_0.clone()
            + (cond_felt_252_as_addr_input_limb_1.clone() * M31_512.clone()))
            + (cond_felt_252_as_addr_input_limb_2.clone() * M31_262144.clone()))
            + (cond_felt_252_as_addr_input_limb_3.clone() * M31_134217728.clone()))]
    }
}
