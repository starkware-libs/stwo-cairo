use crate::components::prelude::*;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct VerifyAdd252 {}

impl VerifyAdd252 {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    pub fn evaluate<E: EvalAtRow>(
        [verify_add_252_input_limb_0, verify_add_252_input_limb_1, verify_add_252_input_limb_2, verify_add_252_input_limb_3, verify_add_252_input_limb_4, verify_add_252_input_limb_5, verify_add_252_input_limb_6, verify_add_252_input_limb_7, verify_add_252_input_limb_8, verify_add_252_input_limb_9, verify_add_252_input_limb_10, verify_add_252_input_limb_11, verify_add_252_input_limb_12, verify_add_252_input_limb_13, verify_add_252_input_limb_14, verify_add_252_input_limb_15, verify_add_252_input_limb_16, verify_add_252_input_limb_17, verify_add_252_input_limb_18, verify_add_252_input_limb_19, verify_add_252_input_limb_20, verify_add_252_input_limb_21, verify_add_252_input_limb_22, verify_add_252_input_limb_23, verify_add_252_input_limb_24, verify_add_252_input_limb_25, verify_add_252_input_limb_26, verify_add_252_input_limb_27, verify_add_252_input_limb_28, verify_add_252_input_limb_29, verify_add_252_input_limb_30, verify_add_252_input_limb_31, verify_add_252_input_limb_32, verify_add_252_input_limb_33, verify_add_252_input_limb_34, verify_add_252_input_limb_35, verify_add_252_input_limb_36, verify_add_252_input_limb_37, verify_add_252_input_limb_38, verify_add_252_input_limb_39, verify_add_252_input_limb_40, verify_add_252_input_limb_41, verify_add_252_input_limb_42, verify_add_252_input_limb_43, verify_add_252_input_limb_44, verify_add_252_input_limb_45, verify_add_252_input_limb_46, verify_add_252_input_limb_47, verify_add_252_input_limb_48, verify_add_252_input_limb_49, verify_add_252_input_limb_50, verify_add_252_input_limb_51, verify_add_252_input_limb_52, verify_add_252_input_limb_53, verify_add_252_input_limb_54, verify_add_252_input_limb_55, verify_add_252_input_limb_56, verify_add_252_input_limb_57, verify_add_252_input_limb_58, verify_add_252_input_limb_59, verify_add_252_input_limb_60, verify_add_252_input_limb_61, verify_add_252_input_limb_62, verify_add_252_input_limb_63, verify_add_252_input_limb_64, verify_add_252_input_limb_65, verify_add_252_input_limb_66, verify_add_252_input_limb_67, verify_add_252_input_limb_68, verify_add_252_input_limb_69, verify_add_252_input_limb_70, verify_add_252_input_limb_71, verify_add_252_input_limb_72, verify_add_252_input_limb_73, verify_add_252_input_limb_74, verify_add_252_input_limb_75, verify_add_252_input_limb_76, verify_add_252_input_limb_77, verify_add_252_input_limb_78, verify_add_252_input_limb_79, verify_add_252_input_limb_80, verify_add_252_input_limb_81, verify_add_252_input_limb_82, verify_add_252_input_limb_83]: [E::F; 84],
        sub_p_bit_col0: E::F,
        eval: &mut E,
    ) -> () {
        let M31_1 = E::F::from(M31::from(1));
        let M31_136 = E::F::from(M31::from(136));
        let M31_256 = E::F::from(M31::from(256));
        let M31_4194304 = E::F::from(M31::from(4194304));

        // sub_p_bit is a bit.
        eval.add_constraint((sub_p_bit_col0.clone() * (sub_p_bit_col0.clone() - M31_1.clone())));
        let carry_tmp_4afb1_1 = eval.add_intermediate(
            ((((verify_add_252_input_limb_0.clone() + verify_add_252_input_limb_28.clone())
                - verify_add_252_input_limb_56.clone())
                - sub_p_bit_col0.clone())
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_4afb1_1.clone()
                * ((carry_tmp_4afb1_1.clone() * carry_tmp_4afb1_1.clone()) - M31_1.clone())),
        );
        let carry_tmp_4afb1_2 = eval.add_intermediate(
            ((((verify_add_252_input_limb_1.clone() + verify_add_252_input_limb_29.clone())
                + carry_tmp_4afb1_1.clone())
                - verify_add_252_input_limb_57.clone())
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_4afb1_2.clone()
                * ((carry_tmp_4afb1_2.clone() * carry_tmp_4afb1_2.clone()) - M31_1.clone())),
        );
        let carry_tmp_4afb1_3 = eval.add_intermediate(
            ((((verify_add_252_input_limb_2.clone() + verify_add_252_input_limb_30.clone())
                + carry_tmp_4afb1_2.clone())
                - verify_add_252_input_limb_58.clone())
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_4afb1_3.clone()
                * ((carry_tmp_4afb1_3.clone() * carry_tmp_4afb1_3.clone()) - M31_1.clone())),
        );
        let carry_tmp_4afb1_4 = eval.add_intermediate(
            ((((verify_add_252_input_limb_3.clone() + verify_add_252_input_limb_31.clone())
                + carry_tmp_4afb1_3.clone())
                - verify_add_252_input_limb_59.clone())
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_4afb1_4.clone()
                * ((carry_tmp_4afb1_4.clone() * carry_tmp_4afb1_4.clone()) - M31_1.clone())),
        );
        let carry_tmp_4afb1_5 = eval.add_intermediate(
            ((((verify_add_252_input_limb_4.clone() + verify_add_252_input_limb_32.clone())
                + carry_tmp_4afb1_4.clone())
                - verify_add_252_input_limb_60.clone())
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_4afb1_5.clone()
                * ((carry_tmp_4afb1_5.clone() * carry_tmp_4afb1_5.clone()) - M31_1.clone())),
        );
        let carry_tmp_4afb1_6 = eval.add_intermediate(
            ((((verify_add_252_input_limb_5.clone() + verify_add_252_input_limb_33.clone())
                + carry_tmp_4afb1_5.clone())
                - verify_add_252_input_limb_61.clone())
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_4afb1_6.clone()
                * ((carry_tmp_4afb1_6.clone() * carry_tmp_4afb1_6.clone()) - M31_1.clone())),
        );
        let carry_tmp_4afb1_7 = eval.add_intermediate(
            ((((verify_add_252_input_limb_6.clone() + verify_add_252_input_limb_34.clone())
                + carry_tmp_4afb1_6.clone())
                - verify_add_252_input_limb_62.clone())
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_4afb1_7.clone()
                * ((carry_tmp_4afb1_7.clone() * carry_tmp_4afb1_7.clone()) - M31_1.clone())),
        );
        let carry_tmp_4afb1_8 = eval.add_intermediate(
            ((((verify_add_252_input_limb_7.clone() + verify_add_252_input_limb_35.clone())
                + carry_tmp_4afb1_7.clone())
                - verify_add_252_input_limb_63.clone())
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_4afb1_8.clone()
                * ((carry_tmp_4afb1_8.clone() * carry_tmp_4afb1_8.clone()) - M31_1.clone())),
        );
        let carry_tmp_4afb1_9 = eval.add_intermediate(
            ((((verify_add_252_input_limb_8.clone() + verify_add_252_input_limb_36.clone())
                + carry_tmp_4afb1_8.clone())
                - verify_add_252_input_limb_64.clone())
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_4afb1_9.clone()
                * ((carry_tmp_4afb1_9.clone() * carry_tmp_4afb1_9.clone()) - M31_1.clone())),
        );
        let carry_tmp_4afb1_10 = eval.add_intermediate(
            ((((verify_add_252_input_limb_9.clone() + verify_add_252_input_limb_37.clone())
                + carry_tmp_4afb1_9.clone())
                - verify_add_252_input_limb_65.clone())
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_4afb1_10.clone()
                * ((carry_tmp_4afb1_10.clone() * carry_tmp_4afb1_10.clone()) - M31_1.clone())),
        );
        let carry_tmp_4afb1_11 = eval.add_intermediate(
            ((((verify_add_252_input_limb_10.clone() + verify_add_252_input_limb_38.clone())
                + carry_tmp_4afb1_10.clone())
                - verify_add_252_input_limb_66.clone())
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_4afb1_11.clone()
                * ((carry_tmp_4afb1_11.clone() * carry_tmp_4afb1_11.clone()) - M31_1.clone())),
        );
        let carry_tmp_4afb1_12 = eval.add_intermediate(
            ((((verify_add_252_input_limb_11.clone() + verify_add_252_input_limb_39.clone())
                + carry_tmp_4afb1_11.clone())
                - verify_add_252_input_limb_67.clone())
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_4afb1_12.clone()
                * ((carry_tmp_4afb1_12.clone() * carry_tmp_4afb1_12.clone()) - M31_1.clone())),
        );
        let carry_tmp_4afb1_13 = eval.add_intermediate(
            ((((verify_add_252_input_limb_12.clone() + verify_add_252_input_limb_40.clone())
                + carry_tmp_4afb1_12.clone())
                - verify_add_252_input_limb_68.clone())
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_4afb1_13.clone()
                * ((carry_tmp_4afb1_13.clone() * carry_tmp_4afb1_13.clone()) - M31_1.clone())),
        );
        let carry_tmp_4afb1_14 = eval.add_intermediate(
            ((((verify_add_252_input_limb_13.clone() + verify_add_252_input_limb_41.clone())
                + carry_tmp_4afb1_13.clone())
                - verify_add_252_input_limb_69.clone())
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_4afb1_14.clone()
                * ((carry_tmp_4afb1_14.clone() * carry_tmp_4afb1_14.clone()) - M31_1.clone())),
        );
        let carry_tmp_4afb1_15 = eval.add_intermediate(
            ((((verify_add_252_input_limb_14.clone() + verify_add_252_input_limb_42.clone())
                + carry_tmp_4afb1_14.clone())
                - verify_add_252_input_limb_70.clone())
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_4afb1_15.clone()
                * ((carry_tmp_4afb1_15.clone() * carry_tmp_4afb1_15.clone()) - M31_1.clone())),
        );
        let carry_tmp_4afb1_16 = eval.add_intermediate(
            ((((verify_add_252_input_limb_15.clone() + verify_add_252_input_limb_43.clone())
                + carry_tmp_4afb1_15.clone())
                - verify_add_252_input_limb_71.clone())
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_4afb1_16.clone()
                * ((carry_tmp_4afb1_16.clone() * carry_tmp_4afb1_16.clone()) - M31_1.clone())),
        );
        let carry_tmp_4afb1_17 = eval.add_intermediate(
            ((((verify_add_252_input_limb_16.clone() + verify_add_252_input_limb_44.clone())
                + carry_tmp_4afb1_16.clone())
                - verify_add_252_input_limb_72.clone())
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_4afb1_17.clone()
                * ((carry_tmp_4afb1_17.clone() * carry_tmp_4afb1_17.clone()) - M31_1.clone())),
        );
        let carry_tmp_4afb1_18 = eval.add_intermediate(
            ((((verify_add_252_input_limb_17.clone() + verify_add_252_input_limb_45.clone())
                + carry_tmp_4afb1_17.clone())
                - verify_add_252_input_limb_73.clone())
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_4afb1_18.clone()
                * ((carry_tmp_4afb1_18.clone() * carry_tmp_4afb1_18.clone()) - M31_1.clone())),
        );
        let carry_tmp_4afb1_19 = eval.add_intermediate(
            ((((verify_add_252_input_limb_18.clone() + verify_add_252_input_limb_46.clone())
                + carry_tmp_4afb1_18.clone())
                - verify_add_252_input_limb_74.clone())
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_4afb1_19.clone()
                * ((carry_tmp_4afb1_19.clone() * carry_tmp_4afb1_19.clone()) - M31_1.clone())),
        );
        let carry_tmp_4afb1_20 = eval.add_intermediate(
            ((((verify_add_252_input_limb_19.clone() + verify_add_252_input_limb_47.clone())
                + carry_tmp_4afb1_19.clone())
                - verify_add_252_input_limb_75.clone())
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_4afb1_20.clone()
                * ((carry_tmp_4afb1_20.clone() * carry_tmp_4afb1_20.clone()) - M31_1.clone())),
        );
        let carry_tmp_4afb1_21 = eval.add_intermediate(
            ((((verify_add_252_input_limb_20.clone() + verify_add_252_input_limb_48.clone())
                + carry_tmp_4afb1_20.clone())
                - verify_add_252_input_limb_76.clone())
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_4afb1_21.clone()
                * ((carry_tmp_4afb1_21.clone() * carry_tmp_4afb1_21.clone()) - M31_1.clone())),
        );
        let carry_tmp_4afb1_22 = eval.add_intermediate(
            (((((verify_add_252_input_limb_21.clone() + verify_add_252_input_limb_49.clone())
                + carry_tmp_4afb1_21.clone())
                - verify_add_252_input_limb_77.clone())
                - (M31_136.clone() * sub_p_bit_col0.clone()))
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_4afb1_22.clone()
                * ((carry_tmp_4afb1_22.clone() * carry_tmp_4afb1_22.clone()) - M31_1.clone())),
        );
        let carry_tmp_4afb1_23 = eval.add_intermediate(
            ((((verify_add_252_input_limb_22.clone() + verify_add_252_input_limb_50.clone())
                + carry_tmp_4afb1_22.clone())
                - verify_add_252_input_limb_78.clone())
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_4afb1_23.clone()
                * ((carry_tmp_4afb1_23.clone() * carry_tmp_4afb1_23.clone()) - M31_1.clone())),
        );
        let carry_tmp_4afb1_24 = eval.add_intermediate(
            ((((verify_add_252_input_limb_23.clone() + verify_add_252_input_limb_51.clone())
                + carry_tmp_4afb1_23.clone())
                - verify_add_252_input_limb_79.clone())
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_4afb1_24.clone()
                * ((carry_tmp_4afb1_24.clone() * carry_tmp_4afb1_24.clone()) - M31_1.clone())),
        );
        let carry_tmp_4afb1_25 = eval.add_intermediate(
            ((((verify_add_252_input_limb_24.clone() + verify_add_252_input_limb_52.clone())
                + carry_tmp_4afb1_24.clone())
                - verify_add_252_input_limb_80.clone())
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_4afb1_25.clone()
                * ((carry_tmp_4afb1_25.clone() * carry_tmp_4afb1_25.clone()) - M31_1.clone())),
        );
        let carry_tmp_4afb1_26 = eval.add_intermediate(
            ((((verify_add_252_input_limb_25.clone() + verify_add_252_input_limb_53.clone())
                + carry_tmp_4afb1_25.clone())
                - verify_add_252_input_limb_81.clone())
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_4afb1_26.clone()
                * ((carry_tmp_4afb1_26.clone() * carry_tmp_4afb1_26.clone()) - M31_1.clone())),
        );
        let carry_tmp_4afb1_27 = eval.add_intermediate(
            ((((verify_add_252_input_limb_26.clone() + verify_add_252_input_limb_54.clone())
                + carry_tmp_4afb1_26.clone())
                - verify_add_252_input_limb_82.clone())
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_4afb1_27.clone()
                * ((carry_tmp_4afb1_27.clone() * carry_tmp_4afb1_27.clone()) - M31_1.clone())),
        );
        eval.add_constraint(
            ((((verify_add_252_input_limb_27.clone() + verify_add_252_input_limb_55.clone())
                + carry_tmp_4afb1_27.clone())
                - verify_add_252_input_limb_83.clone())
                - (M31_256.clone() * sub_p_bit_col0.clone())),
        );
        ()
    }
}
