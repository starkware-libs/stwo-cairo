// This file was created by the AIR team.

use crate::components::prelude::*;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct VerifyMulSmall {}

impl VerifyMulSmall {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [verify_mul_small_input_a_limb_0, verify_mul_small_input_a_limb_1, verify_mul_small_input_a_limb_2, verify_mul_small_input_a_limb_3, verify_mul_small_input_b_limb_0, verify_mul_small_input_b_limb_1, verify_mul_small_input_b_limb_2, verify_mul_small_input_b_limb_3, verify_mul_small_input_c_limb_0, verify_mul_small_input_c_limb_1, verify_mul_small_input_c_limb_2, verify_mul_small_input_c_limb_3, verify_mul_small_input_c_limb_4, verify_mul_small_input_c_limb_5, verify_mul_small_input_c_limb_6, verify_mul_small_input_c_limb_7]: [E::F; 16],
        carry_1_col0: E::F,
        carry_3_col1: E::F,
        carry_5_col2: E::F,
        range_check_11_lookup_elements: &relations::RangeCheck_11,
        eval: &mut E,
    ) -> [E::F; 0] {
        let M31_262144 = E::F::from(M31::from(262144));
        let M31_512 = E::F::from(M31::from(512));

        eval.add_to_relation(RelationEntry::new(
            range_check_11_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&carry_1_col0),
        ));

        // carry 1 definition.
        eval.add_constraint(
            ((carry_1_col0.clone() * M31_262144.clone())
                - (((verify_mul_small_input_a_limb_0.clone()
                    * verify_mul_small_input_b_limb_0.clone())
                    - verify_mul_small_input_c_limb_0.clone())
                    + ((((verify_mul_small_input_a_limb_0.clone()
                        * verify_mul_small_input_b_limb_1.clone())
                        + (verify_mul_small_input_a_limb_1.clone()
                            * verify_mul_small_input_b_limb_0.clone()))
                        - verify_mul_small_input_c_limb_1.clone())
                        * M31_512.clone()))),
        );
        eval.add_to_relation(RelationEntry::new(
            range_check_11_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&carry_3_col1),
        ));

        // carry 3 definition.
        eval.add_constraint(
            ((carry_3_col1.clone() * M31_262144.clone())
                - ((carry_1_col0.clone()
                    + ((((verify_mul_small_input_a_limb_0.clone()
                        * verify_mul_small_input_b_limb_2.clone())
                        + (verify_mul_small_input_a_limb_1.clone()
                            * verify_mul_small_input_b_limb_1.clone()))
                        + (verify_mul_small_input_a_limb_2.clone()
                            * verify_mul_small_input_b_limb_0.clone()))
                        - verify_mul_small_input_c_limb_2.clone()))
                    + ((((((verify_mul_small_input_a_limb_0.clone()
                        * verify_mul_small_input_b_limb_3.clone())
                        + (verify_mul_small_input_a_limb_1.clone()
                            * verify_mul_small_input_b_limb_2.clone()))
                        + (verify_mul_small_input_a_limb_2.clone()
                            * verify_mul_small_input_b_limb_1.clone()))
                        + (verify_mul_small_input_a_limb_3.clone()
                            * verify_mul_small_input_b_limb_0.clone()))
                        - verify_mul_small_input_c_limb_3.clone())
                        * M31_512.clone()))),
        );
        eval.add_to_relation(RelationEntry::new(
            range_check_11_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&carry_5_col2),
        ));

        // carry 5 definition.
        eval.add_constraint(
            ((carry_5_col2.clone() * M31_262144.clone())
                - ((carry_3_col1.clone()
                    + ((((verify_mul_small_input_a_limb_1.clone()
                        * verify_mul_small_input_b_limb_3.clone())
                        + (verify_mul_small_input_a_limb_2.clone()
                            * verify_mul_small_input_b_limb_2.clone()))
                        + (verify_mul_small_input_a_limb_3.clone()
                            * verify_mul_small_input_b_limb_1.clone()))
                        - verify_mul_small_input_c_limb_4.clone()))
                    + ((((verify_mul_small_input_a_limb_2.clone()
                        * verify_mul_small_input_b_limb_3.clone())
                        + (verify_mul_small_input_a_limb_3.clone()
                            * verify_mul_small_input_b_limb_2.clone()))
                        - verify_mul_small_input_c_limb_5.clone())
                        * M31_512.clone()))),
        );
        // final limb constraint.
        eval.add_constraint(
            (((carry_5_col2.clone()
                + (verify_mul_small_input_a_limb_3.clone()
                    * verify_mul_small_input_b_limb_3.clone()))
                - (verify_mul_small_input_c_limb_7.clone() * M31_512.clone()))
                - verify_mul_small_input_c_limb_6.clone()),
        );
        []
    }
}
