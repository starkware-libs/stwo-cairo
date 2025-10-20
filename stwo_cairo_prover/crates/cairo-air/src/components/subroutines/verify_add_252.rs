// This file was created by the AIR team.

use crate::components::prelude::*;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct VerifyAdd252 {}

impl VerifyAdd252 {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [verify_add_252_input_a_limb_0, verify_add_252_input_a_limb_1, verify_add_252_input_a_limb_2, verify_add_252_input_a_limb_3, verify_add_252_input_a_limb_4, verify_add_252_input_a_limb_5, verify_add_252_input_a_limb_6, verify_add_252_input_a_limb_7, verify_add_252_input_a_limb_8, verify_add_252_input_a_limb_9, verify_add_252_input_a_limb_10, verify_add_252_input_a_limb_11, verify_add_252_input_a_limb_12, verify_add_252_input_a_limb_13, verify_add_252_input_a_limb_14, verify_add_252_input_a_limb_15, verify_add_252_input_a_limb_16, verify_add_252_input_a_limb_17, verify_add_252_input_a_limb_18, verify_add_252_input_a_limb_19, verify_add_252_input_a_limb_20, verify_add_252_input_a_limb_21, verify_add_252_input_a_limb_22, verify_add_252_input_a_limb_23, verify_add_252_input_a_limb_24, verify_add_252_input_a_limb_25, verify_add_252_input_a_limb_26, verify_add_252_input_a_limb_27, verify_add_252_input_b_limb_0, verify_add_252_input_b_limb_1, verify_add_252_input_b_limb_2, verify_add_252_input_b_limb_3, verify_add_252_input_b_limb_4, verify_add_252_input_b_limb_5, verify_add_252_input_b_limb_6, verify_add_252_input_b_limb_7, verify_add_252_input_b_limb_8, verify_add_252_input_b_limb_9, verify_add_252_input_b_limb_10, verify_add_252_input_b_limb_11, verify_add_252_input_b_limb_12, verify_add_252_input_b_limb_13, verify_add_252_input_b_limb_14, verify_add_252_input_b_limb_15, verify_add_252_input_b_limb_16, verify_add_252_input_b_limb_17, verify_add_252_input_b_limb_18, verify_add_252_input_b_limb_19, verify_add_252_input_b_limb_20, verify_add_252_input_b_limb_21, verify_add_252_input_b_limb_22, verify_add_252_input_b_limb_23, verify_add_252_input_b_limb_24, verify_add_252_input_b_limb_25, verify_add_252_input_b_limb_26, verify_add_252_input_b_limb_27, verify_add_252_input_c_limb_0, verify_add_252_input_c_limb_1, verify_add_252_input_c_limb_2, verify_add_252_input_c_limb_3, verify_add_252_input_c_limb_4, verify_add_252_input_c_limb_5, verify_add_252_input_c_limb_6, verify_add_252_input_c_limb_7, verify_add_252_input_c_limb_8, verify_add_252_input_c_limb_9, verify_add_252_input_c_limb_10, verify_add_252_input_c_limb_11, verify_add_252_input_c_limb_12, verify_add_252_input_c_limb_13, verify_add_252_input_c_limb_14, verify_add_252_input_c_limb_15, verify_add_252_input_c_limb_16, verify_add_252_input_c_limb_17, verify_add_252_input_c_limb_18, verify_add_252_input_c_limb_19, verify_add_252_input_c_limb_20, verify_add_252_input_c_limb_21, verify_add_252_input_c_limb_22, verify_add_252_input_c_limb_23, verify_add_252_input_c_limb_24, verify_add_252_input_c_limb_25, verify_add_252_input_c_limb_26, verify_add_252_input_c_limb_27]: [E::F; 84],
        sub_p_bit_col0: E::F,
        eval: &mut E,
    ) -> [E::F; 0] {
        let M31_1 = E::F::from(M31::from(1));
        let M31_136 = E::F::from(M31::from(136));
        let M31_256 = E::F::from(M31::from(256));
        let M31_4194304 = E::F::from(M31::from(4194304));

        // sub_p_bit is a bit.
        eval.add_constraint((sub_p_bit_col0.clone() * (sub_p_bit_col0.clone() - M31_1.clone())));
        let carry_tmp_4afb1_1 = eval.add_intermediate(
            ((((verify_add_252_input_a_limb_2.clone() + verify_add_252_input_b_limb_2.clone())
                + ((((verify_add_252_input_a_limb_1.clone()
                    + verify_add_252_input_b_limb_1.clone())
                    + ((((verify_add_252_input_a_limb_0.clone()
                        + verify_add_252_input_b_limb_0.clone())
                        - verify_add_252_input_c_limb_0.clone())
                        - sub_p_bit_col0.clone())
                        * M31_4194304.clone()))
                    - verify_add_252_input_c_limb_1.clone())
                    * M31_4194304.clone()))
                - verify_add_252_input_c_limb_2.clone())
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_4afb1_1.clone()
                * ((carry_tmp_4afb1_1.clone() * carry_tmp_4afb1_1.clone()) - M31_1.clone())),
        );
        let carry_tmp_4afb1_2 = eval.add_intermediate(
            ((((verify_add_252_input_a_limb_5.clone() + verify_add_252_input_b_limb_5.clone())
                + ((((verify_add_252_input_a_limb_4.clone()
                    + verify_add_252_input_b_limb_4.clone())
                    + ((((verify_add_252_input_a_limb_3.clone()
                        + verify_add_252_input_b_limb_3.clone())
                        + carry_tmp_4afb1_1.clone())
                        - verify_add_252_input_c_limb_3.clone())
                        * M31_4194304.clone()))
                    - verify_add_252_input_c_limb_4.clone())
                    * M31_4194304.clone()))
                - verify_add_252_input_c_limb_5.clone())
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_4afb1_2.clone()
                * ((carry_tmp_4afb1_2.clone() * carry_tmp_4afb1_2.clone()) - M31_1.clone())),
        );
        let carry_tmp_4afb1_3 = eval.add_intermediate(
            ((((verify_add_252_input_a_limb_8.clone() + verify_add_252_input_b_limb_8.clone())
                + ((((verify_add_252_input_a_limb_7.clone()
                    + verify_add_252_input_b_limb_7.clone())
                    + ((((verify_add_252_input_a_limb_6.clone()
                        + verify_add_252_input_b_limb_6.clone())
                        + carry_tmp_4afb1_2.clone())
                        - verify_add_252_input_c_limb_6.clone())
                        * M31_4194304.clone()))
                    - verify_add_252_input_c_limb_7.clone())
                    * M31_4194304.clone()))
                - verify_add_252_input_c_limb_8.clone())
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_4afb1_3.clone()
                * ((carry_tmp_4afb1_3.clone() * carry_tmp_4afb1_3.clone()) - M31_1.clone())),
        );
        let carry_tmp_4afb1_4 = eval.add_intermediate(
            ((((verify_add_252_input_a_limb_11.clone() + verify_add_252_input_b_limb_11.clone())
                + ((((verify_add_252_input_a_limb_10.clone()
                    + verify_add_252_input_b_limb_10.clone())
                    + ((((verify_add_252_input_a_limb_9.clone()
                        + verify_add_252_input_b_limb_9.clone())
                        + carry_tmp_4afb1_3.clone())
                        - verify_add_252_input_c_limb_9.clone())
                        * M31_4194304.clone()))
                    - verify_add_252_input_c_limb_10.clone())
                    * M31_4194304.clone()))
                - verify_add_252_input_c_limb_11.clone())
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_4afb1_4.clone()
                * ((carry_tmp_4afb1_4.clone() * carry_tmp_4afb1_4.clone()) - M31_1.clone())),
        );
        let carry_tmp_4afb1_5 = eval.add_intermediate(
            ((((verify_add_252_input_a_limb_14.clone() + verify_add_252_input_b_limb_14.clone())
                + ((((verify_add_252_input_a_limb_13.clone()
                    + verify_add_252_input_b_limb_13.clone())
                    + ((((verify_add_252_input_a_limb_12.clone()
                        + verify_add_252_input_b_limb_12.clone())
                        + carry_tmp_4afb1_4.clone())
                        - verify_add_252_input_c_limb_12.clone())
                        * M31_4194304.clone()))
                    - verify_add_252_input_c_limb_13.clone())
                    * M31_4194304.clone()))
                - verify_add_252_input_c_limb_14.clone())
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_4afb1_5.clone()
                * ((carry_tmp_4afb1_5.clone() * carry_tmp_4afb1_5.clone()) - M31_1.clone())),
        );
        let carry_tmp_4afb1_6 = eval.add_intermediate(
            ((((verify_add_252_input_a_limb_17.clone() + verify_add_252_input_b_limb_17.clone())
                + ((((verify_add_252_input_a_limb_16.clone()
                    + verify_add_252_input_b_limb_16.clone())
                    + ((((verify_add_252_input_a_limb_15.clone()
                        + verify_add_252_input_b_limb_15.clone())
                        + carry_tmp_4afb1_5.clone())
                        - verify_add_252_input_c_limb_15.clone())
                        * M31_4194304.clone()))
                    - verify_add_252_input_c_limb_16.clone())
                    * M31_4194304.clone()))
                - verify_add_252_input_c_limb_17.clone())
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_4afb1_6.clone()
                * ((carry_tmp_4afb1_6.clone() * carry_tmp_4afb1_6.clone()) - M31_1.clone())),
        );
        let carry_tmp_4afb1_7 = eval.add_intermediate(
            ((((verify_add_252_input_a_limb_20.clone() + verify_add_252_input_b_limb_20.clone())
                + ((((verify_add_252_input_a_limb_19.clone()
                    + verify_add_252_input_b_limb_19.clone())
                    + ((((verify_add_252_input_a_limb_18.clone()
                        + verify_add_252_input_b_limb_18.clone())
                        + carry_tmp_4afb1_6.clone())
                        - verify_add_252_input_c_limb_18.clone())
                        * M31_4194304.clone()))
                    - verify_add_252_input_c_limb_19.clone())
                    * M31_4194304.clone()))
                - verify_add_252_input_c_limb_20.clone())
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_4afb1_7.clone()
                * ((carry_tmp_4afb1_7.clone() * carry_tmp_4afb1_7.clone()) - M31_1.clone())),
        );
        let carry_tmp_4afb1_8 = eval.add_intermediate(
            ((((verify_add_252_input_a_limb_23.clone() + verify_add_252_input_b_limb_23.clone())
                + ((((verify_add_252_input_a_limb_22.clone()
                    + verify_add_252_input_b_limb_22.clone())
                    + (((((verify_add_252_input_a_limb_21.clone()
                        + verify_add_252_input_b_limb_21.clone())
                        + carry_tmp_4afb1_7.clone())
                        - verify_add_252_input_c_limb_21.clone())
                        - (M31_136.clone() * sub_p_bit_col0.clone()))
                        * M31_4194304.clone()))
                    - verify_add_252_input_c_limb_22.clone())
                    * M31_4194304.clone()))
                - verify_add_252_input_c_limb_23.clone())
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_4afb1_8.clone()
                * ((carry_tmp_4afb1_8.clone() * carry_tmp_4afb1_8.clone()) - M31_1.clone())),
        );
        let carry_tmp_4afb1_9 = eval.add_intermediate(
            ((((verify_add_252_input_a_limb_26.clone() + verify_add_252_input_b_limb_26.clone())
                + ((((verify_add_252_input_a_limb_25.clone()
                    + verify_add_252_input_b_limb_25.clone())
                    + ((((verify_add_252_input_a_limb_24.clone()
                        + verify_add_252_input_b_limb_24.clone())
                        + carry_tmp_4afb1_8.clone())
                        - verify_add_252_input_c_limb_24.clone())
                        * M31_4194304.clone()))
                    - verify_add_252_input_c_limb_25.clone())
                    * M31_4194304.clone()))
                - verify_add_252_input_c_limb_26.clone())
                * M31_4194304.clone()),
        );
        eval.add_constraint(
            (carry_tmp_4afb1_9.clone()
                * ((carry_tmp_4afb1_9.clone() * carry_tmp_4afb1_9.clone()) - M31_1.clone())),
        );
        eval.add_constraint(
            ((((verify_add_252_input_a_limb_27.clone() + verify_add_252_input_b_limb_27.clone())
                + carry_tmp_4afb1_9.clone())
                - verify_add_252_input_c_limb_27.clone())
                - (M31_256.clone() * sub_p_bit_col0.clone())),
        );
        []
    }
}
