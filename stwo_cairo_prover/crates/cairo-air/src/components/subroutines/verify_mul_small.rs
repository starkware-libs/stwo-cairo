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
        [verify_mul_small_input_limb_0, verify_mul_small_input_limb_1, verify_mul_small_input_limb_2, verify_mul_small_input_limb_3, verify_mul_small_input_limb_4, verify_mul_small_input_limb_5, verify_mul_small_input_limb_6, verify_mul_small_input_limb_7, verify_mul_small_input_limb_8, verify_mul_small_input_limb_9, verify_mul_small_input_limb_10, verify_mul_small_input_limb_11, verify_mul_small_input_limb_12, verify_mul_small_input_limb_13, verify_mul_small_input_limb_14, verify_mul_small_input_limb_15, verify_mul_small_input_limb_16, verify_mul_small_input_limb_17, verify_mul_small_input_limb_18, verify_mul_small_input_limb_19, verify_mul_small_input_limb_20, verify_mul_small_input_limb_21, verify_mul_small_input_limb_22, verify_mul_small_input_limb_23, verify_mul_small_input_limb_24, verify_mul_small_input_limb_25, verify_mul_small_input_limb_26, verify_mul_small_input_limb_27, verify_mul_small_input_limb_28, verify_mul_small_input_limb_29, verify_mul_small_input_limb_30, verify_mul_small_input_limb_31, verify_mul_small_input_limb_32, verify_mul_small_input_limb_33, verify_mul_small_input_limb_34, verify_mul_small_input_limb_35, verify_mul_small_input_limb_36, verify_mul_small_input_limb_37, verify_mul_small_input_limb_38, verify_mul_small_input_limb_39, verify_mul_small_input_limb_40, verify_mul_small_input_limb_41, verify_mul_small_input_limb_42, verify_mul_small_input_limb_43, verify_mul_small_input_limb_44, verify_mul_small_input_limb_45, verify_mul_small_input_limb_46, verify_mul_small_input_limb_47, verify_mul_small_input_limb_48, verify_mul_small_input_limb_49, verify_mul_small_input_limb_50, verify_mul_small_input_limb_51, verify_mul_small_input_limb_52, verify_mul_small_input_limb_53, verify_mul_small_input_limb_54, verify_mul_small_input_limb_55, verify_mul_small_input_limb_56, verify_mul_small_input_limb_57, verify_mul_small_input_limb_58, verify_mul_small_input_limb_59, verify_mul_small_input_limb_60, verify_mul_small_input_limb_61, verify_mul_small_input_limb_62, verify_mul_small_input_limb_63, verify_mul_small_input_limb_64, verify_mul_small_input_limb_65, verify_mul_small_input_limb_66, verify_mul_small_input_limb_67, verify_mul_small_input_limb_68, verify_mul_small_input_limb_69, verify_mul_small_input_limb_70, verify_mul_small_input_limb_71, verify_mul_small_input_limb_72, verify_mul_small_input_limb_73, verify_mul_small_input_limb_74, verify_mul_small_input_limb_75, verify_mul_small_input_limb_76, verify_mul_small_input_limb_77, verify_mul_small_input_limb_78, verify_mul_small_input_limb_79, verify_mul_small_input_limb_80, verify_mul_small_input_limb_81, verify_mul_small_input_limb_82, verify_mul_small_input_limb_83]: [E::F; 84],
        carry_1_col0: E::F,
        carry_3_col1: E::F,
        carry_5_col2: E::F,
        eval: &mut E,
        range_check_11_lookup_elements: &relations::RangeCheck_11,
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
                - (((((verify_mul_small_input_limb_0.clone()
                    * verify_mul_small_input_limb_28.clone())
                    - verify_mul_small_input_limb_56.clone())
                    + ((verify_mul_small_input_limb_0.clone()
                        * verify_mul_small_input_limb_29.clone())
                        * M31_512.clone()))
                    + ((verify_mul_small_input_limb_1.clone()
                        * verify_mul_small_input_limb_28.clone())
                        * M31_512.clone()))
                    - (verify_mul_small_input_limb_57.clone() * M31_512.clone()))),
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
                    + (verify_mul_small_input_limb_0.clone()
                        * verify_mul_small_input_limb_30.clone()))
                    + (verify_mul_small_input_limb_1.clone()
                        * verify_mul_small_input_limb_29.clone()))
                    + (verify_mul_small_input_limb_2.clone()
                        * verify_mul_small_input_limb_28.clone()))
                    - verify_mul_small_input_limb_58.clone())
                    + ((verify_mul_small_input_limb_0.clone()
                        * verify_mul_small_input_limb_31.clone())
                        * M31_512.clone()))
                    + ((verify_mul_small_input_limb_1.clone()
                        * verify_mul_small_input_limb_30.clone())
                        * M31_512.clone()))
                    + ((verify_mul_small_input_limb_2.clone()
                        * verify_mul_small_input_limb_29.clone())
                        * M31_512.clone()))
                    + ((verify_mul_small_input_limb_3.clone()
                        * verify_mul_small_input_limb_28.clone())
                        * M31_512.clone()))
                    - (verify_mul_small_input_limb_59.clone() * M31_512.clone()))),
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
                    + (verify_mul_small_input_limb_1.clone()
                        * verify_mul_small_input_limb_31.clone()))
                    + (verify_mul_small_input_limb_2.clone()
                        * verify_mul_small_input_limb_30.clone()))
                    + (verify_mul_small_input_limb_3.clone()
                        * verify_mul_small_input_limb_29.clone()))
                    - verify_mul_small_input_limb_60.clone())
                    + ((verify_mul_small_input_limb_2.clone()
                        * verify_mul_small_input_limb_31.clone())
                        * M31_512.clone()))
                    + ((verify_mul_small_input_limb_3.clone()
                        * verify_mul_small_input_limb_30.clone())
                        * M31_512.clone()))
                    - (verify_mul_small_input_limb_61.clone() * M31_512.clone()))),
        );
        // final limb constraint.
        eval.add_constraint(
            (((carry_5_col2.clone()
                + (verify_mul_small_input_limb_3.clone()
                    * verify_mul_small_input_limb_31.clone()))
                - (verify_mul_small_input_limb_63.clone() * M31_512.clone()))
                - verify_mul_small_input_limb_62.clone()),
        );
        ()
    }
}
