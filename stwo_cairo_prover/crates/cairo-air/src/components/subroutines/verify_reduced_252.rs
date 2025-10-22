// This file was created by the AIR team.

use crate::components::prelude::*;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct VerifyReduced252 {}

impl VerifyReduced252 {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [verify_reduced_252_input_limb_0, verify_reduced_252_input_limb_1, verify_reduced_252_input_limb_2, verify_reduced_252_input_limb_3, verify_reduced_252_input_limb_4, verify_reduced_252_input_limb_5, verify_reduced_252_input_limb_6, verify_reduced_252_input_limb_7, verify_reduced_252_input_limb_8, verify_reduced_252_input_limb_9, verify_reduced_252_input_limb_10, verify_reduced_252_input_limb_11, verify_reduced_252_input_limb_12, verify_reduced_252_input_limb_13, verify_reduced_252_input_limb_14, verify_reduced_252_input_limb_15, verify_reduced_252_input_limb_16, verify_reduced_252_input_limb_17, verify_reduced_252_input_limb_18, verify_reduced_252_input_limb_19, verify_reduced_252_input_limb_20, verify_reduced_252_input_limb_21, verify_reduced_252_input_limb_22, verify_reduced_252_input_limb_23, verify_reduced_252_input_limb_24, verify_reduced_252_input_limb_25, verify_reduced_252_input_limb_26, verify_reduced_252_input_limb_27]: [E::F; 28],
        ms_limb_is_max_col0: E::F,
        ms_and_mid_limbs_are_max_col1: E::F,
        rc_input_col2: E::F,
        range_check_8_lookup_elements: &relations::RangeCheck_8,
        eval: &mut E,
    ) -> [E::F; 0] {
        let M31_1 = E::F::from(M31::from(1));
        let M31_120 = E::F::from(M31::from(120));

        // ms_max is bit.
        eval.add_constraint(
            (ms_limb_is_max_col0.clone() * (M31_1.clone() - ms_limb_is_max_col0.clone())),
        );
        // both_max is bit.
        eval.add_constraint(
            (ms_and_mid_limbs_are_max_col1.clone()
                * (M31_1.clone() - ms_and_mid_limbs_are_max_col1.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            range_check_8_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(
                &(verify_reduced_252_input_limb_27.clone() - ms_limb_is_max_col0.clone()),
            ),
        ));

        // If the MS limb is max, high limbs should be 0.
        eval.add_constraint(
            (ms_limb_is_max_col0.clone()
                * ((((verify_reduced_252_input_limb_22.clone()
                    + verify_reduced_252_input_limb_23.clone())
                    + verify_reduced_252_input_limb_24.clone())
                    + verify_reduced_252_input_limb_25.clone())
                    + verify_reduced_252_input_limb_26.clone())),
        );
        // rc_input.
        eval.add_constraint(
            (rc_input_col2.clone()
                - (ms_limb_is_max_col0.clone()
                    * ((M31_120.clone() + verify_reduced_252_input_limb_21.clone())
                        - ms_and_mid_limbs_are_max_col1.clone()))),
        );
        eval.add_to_relation(RelationEntry::new(
            range_check_8_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&rc_input_col2),
        ));

        // If the MS and mid limbs are max, low limbs should be 0.
        eval.add_constraint(
            (ms_and_mid_limbs_are_max_col1.clone()
                * ((((((((((((((((((((verify_reduced_252_input_limb_0.clone()
                    + verify_reduced_252_input_limb_1.clone())
                    + verify_reduced_252_input_limb_2.clone())
                    + verify_reduced_252_input_limb_3.clone())
                    + verify_reduced_252_input_limb_4.clone())
                    + verify_reduced_252_input_limb_5.clone())
                    + verify_reduced_252_input_limb_6.clone())
                    + verify_reduced_252_input_limb_7.clone())
                    + verify_reduced_252_input_limb_8.clone())
                    + verify_reduced_252_input_limb_9.clone())
                    + verify_reduced_252_input_limb_10.clone())
                    + verify_reduced_252_input_limb_11.clone())
                    + verify_reduced_252_input_limb_12.clone())
                    + verify_reduced_252_input_limb_13.clone())
                    + verify_reduced_252_input_limb_14.clone())
                    + verify_reduced_252_input_limb_15.clone())
                    + verify_reduced_252_input_limb_16.clone())
                    + verify_reduced_252_input_limb_17.clone())
                    + verify_reduced_252_input_limb_18.clone())
                    + verify_reduced_252_input_limb_19.clone())
                    + verify_reduced_252_input_limb_20.clone())),
        );
        []
    }
}
