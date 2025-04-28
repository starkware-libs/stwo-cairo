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
        [verify_mul_small_input_a_limb_0, verify_mul_small_input_a_limb_1, verify_mul_small_input_a_limb_2, verify_mul_small_input_a_limb_3, verify_mul_small_input_a_limb_4, verify_mul_small_input_a_limb_5, verify_mul_small_input_a_limb_6, verify_mul_small_input_a_limb_7, verify_mul_small_input_a_limb_8, verify_mul_small_input_a_limb_9, verify_mul_small_input_a_limb_10, verify_mul_small_input_a_limb_11, verify_mul_small_input_a_limb_12, verify_mul_small_input_a_limb_13, verify_mul_small_input_a_limb_14, verify_mul_small_input_a_limb_15, verify_mul_small_input_a_limb_16, verify_mul_small_input_a_limb_17, verify_mul_small_input_a_limb_18, verify_mul_small_input_a_limb_19, verify_mul_small_input_a_limb_20, verify_mul_small_input_a_limb_21, verify_mul_small_input_a_limb_22, verify_mul_small_input_a_limb_23, verify_mul_small_input_a_limb_24, verify_mul_small_input_a_limb_25, verify_mul_small_input_a_limb_26, verify_mul_small_input_a_limb_27, verify_mul_small_input_b_limb_0, verify_mul_small_input_b_limb_1, verify_mul_small_input_b_limb_2, verify_mul_small_input_b_limb_3, verify_mul_small_input_b_limb_4, verify_mul_small_input_b_limb_5, verify_mul_small_input_b_limb_6, verify_mul_small_input_b_limb_7, verify_mul_small_input_b_limb_8, verify_mul_small_input_b_limb_9, verify_mul_small_input_b_limb_10, verify_mul_small_input_b_limb_11, verify_mul_small_input_b_limb_12, verify_mul_small_input_b_limb_13, verify_mul_small_input_b_limb_14, verify_mul_small_input_b_limb_15, verify_mul_small_input_b_limb_16, verify_mul_small_input_b_limb_17, verify_mul_small_input_b_limb_18, verify_mul_small_input_b_limb_19, verify_mul_small_input_b_limb_20, verify_mul_small_input_b_limb_21, verify_mul_small_input_b_limb_22, verify_mul_small_input_b_limb_23, verify_mul_small_input_b_limb_24, verify_mul_small_input_b_limb_25, verify_mul_small_input_b_limb_26, verify_mul_small_input_b_limb_27, verify_mul_small_input_c_limb_0, verify_mul_small_input_c_limb_1, verify_mul_small_input_c_limb_2, verify_mul_small_input_c_limb_3, verify_mul_small_input_c_limb_4, verify_mul_small_input_c_limb_5, verify_mul_small_input_c_limb_6, verify_mul_small_input_c_limb_7, verify_mul_small_input_c_limb_8, verify_mul_small_input_c_limb_9, verify_mul_small_input_c_limb_10, verify_mul_small_input_c_limb_11, verify_mul_small_input_c_limb_12, verify_mul_small_input_c_limb_13, verify_mul_small_input_c_limb_14, verify_mul_small_input_c_limb_15, verify_mul_small_input_c_limb_16, verify_mul_small_input_c_limb_17, verify_mul_small_input_c_limb_18, verify_mul_small_input_c_limb_19, verify_mul_small_input_c_limb_20, verify_mul_small_input_c_limb_21, verify_mul_small_input_c_limb_22, verify_mul_small_input_c_limb_23, verify_mul_small_input_c_limb_24, verify_mul_small_input_c_limb_25, verify_mul_small_input_c_limb_26, verify_mul_small_input_c_limb_27]: [E::F; 84],
        carry_1_col0: E::F,
        carry_3_col1: E::F,
        carry_5_col2: E::F,
        range_check_11_lookup_elements: &relations::RangeCheck_11,
        eval: &mut E,
    ) -> () {
        let M31_262144 = E::F::from(M31::from(262144));
        let M31_512 = E::F::from(M31::from(512));

        eval.add_to_relation(RelationEntry::new(
            range_check_11_lookup_elements,
            E::EF::one(),
            &[carry_1_col0.clone()],
        ));

        // carry 1 definition.
        eval.add_constraint(
            ((carry_1_col0.clone() * M31_262144.clone())
                - (((((verify_mul_small_input_a_limb_0.clone()
                    * verify_mul_small_input_b_limb_0.clone())
                    - verify_mul_small_input_c_limb_0.clone())
                    + ((verify_mul_small_input_a_limb_0.clone()
                        * verify_mul_small_input_b_limb_1.clone())
                        * M31_512.clone()))
                    + ((verify_mul_small_input_a_limb_1.clone()
                        * verify_mul_small_input_b_limb_0.clone())
                        * M31_512.clone()))
                    - (verify_mul_small_input_c_limb_1.clone() * M31_512.clone()))),
        );
        eval.add_to_relation(RelationEntry::new(
            range_check_11_lookup_elements,
            E::EF::one(),
            &[carry_3_col1.clone()],
        ));

        // carry 3 definition.
        eval.add_constraint(
            ((carry_3_col1.clone() * M31_262144.clone())
                - (((((((((carry_1_col0.clone()
                    + (verify_mul_small_input_a_limb_0.clone()
                        * verify_mul_small_input_b_limb_2.clone()))
                    + (verify_mul_small_input_a_limb_1.clone()
                        * verify_mul_small_input_b_limb_1.clone()))
                    + (verify_mul_small_input_a_limb_2.clone()
                        * verify_mul_small_input_b_limb_0.clone()))
                    - verify_mul_small_input_c_limb_2.clone())
                    + ((verify_mul_small_input_a_limb_0.clone()
                        * verify_mul_small_input_b_limb_3.clone())
                        * M31_512.clone()))
                    + ((verify_mul_small_input_a_limb_1.clone()
                        * verify_mul_small_input_b_limb_2.clone())
                        * M31_512.clone()))
                    + ((verify_mul_small_input_a_limb_2.clone()
                        * verify_mul_small_input_b_limb_1.clone())
                        * M31_512.clone()))
                    + ((verify_mul_small_input_a_limb_3.clone()
                        * verify_mul_small_input_b_limb_0.clone())
                        * M31_512.clone()))
                    - (verify_mul_small_input_c_limb_3.clone() * M31_512.clone()))),
        );
        eval.add_to_relation(RelationEntry::new(
            range_check_11_lookup_elements,
            E::EF::one(),
            &[carry_5_col2.clone()],
        ));

        // carry 5 definition.
        eval.add_constraint(
            ((carry_5_col2.clone() * M31_262144.clone())
                - (((((((carry_3_col1.clone()
                    + (verify_mul_small_input_a_limb_1.clone()
                        * verify_mul_small_input_b_limb_3.clone()))
                    + (verify_mul_small_input_a_limb_2.clone()
                        * verify_mul_small_input_b_limb_2.clone()))
                    + (verify_mul_small_input_a_limb_3.clone()
                        * verify_mul_small_input_b_limb_1.clone()))
                    - verify_mul_small_input_c_limb_4.clone())
                    + ((verify_mul_small_input_a_limb_2.clone()
                        * verify_mul_small_input_b_limb_3.clone())
                        * M31_512.clone()))
                    + ((verify_mul_small_input_a_limb_3.clone()
                        * verify_mul_small_input_b_limb_2.clone())
                        * M31_512.clone()))
                    - (verify_mul_small_input_c_limb_5.clone() * M31_512.clone()))),
        );
        // final limb constraint.
        eval.add_constraint(
            (((carry_5_col2.clone()
                + (verify_mul_small_input_a_limb_3.clone()
                    * verify_mul_small_input_b_limb_3.clone()))
                - (verify_mul_small_input_c_limb_7.clone() * M31_512.clone()))
                - verify_mul_small_input_c_limb_6.clone()),
        );
        ()
    }
}
