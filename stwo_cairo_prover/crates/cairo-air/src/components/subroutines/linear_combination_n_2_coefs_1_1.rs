// This file was created by the AIR team.

use crate::components::prelude::*;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct LinearCombinationN2Coefs11 {}

impl LinearCombinationN2Coefs11 {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [linear_combination_n_2_coefs_1_1_input_limb_0, linear_combination_n_2_coefs_1_1_input_limb_1, linear_combination_n_2_coefs_1_1_input_limb_2, linear_combination_n_2_coefs_1_1_input_limb_3, linear_combination_n_2_coefs_1_1_input_limb_4, linear_combination_n_2_coefs_1_1_input_limb_5, linear_combination_n_2_coefs_1_1_input_limb_6, linear_combination_n_2_coefs_1_1_input_limb_7, linear_combination_n_2_coefs_1_1_input_limb_8, linear_combination_n_2_coefs_1_1_input_limb_9, linear_combination_n_2_coefs_1_1_input_limb_10, linear_combination_n_2_coefs_1_1_input_limb_11, linear_combination_n_2_coefs_1_1_input_limb_12, linear_combination_n_2_coefs_1_1_input_limb_13, linear_combination_n_2_coefs_1_1_input_limb_14, linear_combination_n_2_coefs_1_1_input_limb_15, linear_combination_n_2_coefs_1_1_input_limb_16, linear_combination_n_2_coefs_1_1_input_limb_17, linear_combination_n_2_coefs_1_1_input_limb_18, linear_combination_n_2_coefs_1_1_input_limb_19]: [E::F; 20],
        combination_limb_0_col0: E::F,
        combination_limb_1_col1: E::F,
        combination_limb_2_col2: E::F,
        combination_limb_3_col3: E::F,
        combination_limb_4_col4: E::F,
        combination_limb_5_col5: E::F,
        combination_limb_6_col6: E::F,
        combination_limb_7_col7: E::F,
        combination_limb_8_col8: E::F,
        combination_limb_9_col9: E::F,
        p_coef_col10: E::F,
        eval: &mut E,
    ) -> [E::F; 0] {
        let M31_136 = E::F::from(M31::from(136));
        let M31_16 = E::F::from(M31::from(16));
        let M31_256 = E::F::from(M31::from(256));

        let carry_0_tmp_2db6d_2 = eval.add_intermediate(
            ((((linear_combination_n_2_coefs_1_1_input_limb_0.clone()
                + linear_combination_n_2_coefs_1_1_input_limb_10.clone())
                - combination_limb_0_col0.clone())
                - p_coef_col10.clone())
                * M31_16.clone()),
        );
        let carry_1_tmp_2db6d_3 = eval.add_intermediate(
            ((((carry_0_tmp_2db6d_2.clone()
                + linear_combination_n_2_coefs_1_1_input_limb_1.clone())
                + linear_combination_n_2_coefs_1_1_input_limb_11.clone())
                - combination_limb_1_col1.clone())
                * M31_16.clone()),
        );
        let carry_2_tmp_2db6d_4 = eval.add_intermediate(
            ((((carry_1_tmp_2db6d_3.clone()
                + linear_combination_n_2_coefs_1_1_input_limb_2.clone())
                + linear_combination_n_2_coefs_1_1_input_limb_12.clone())
                - combination_limb_2_col2.clone())
                * M31_16.clone()),
        );
        let carry_3_tmp_2db6d_5 = eval.add_intermediate(
            ((((carry_2_tmp_2db6d_4.clone()
                + linear_combination_n_2_coefs_1_1_input_limb_3.clone())
                + linear_combination_n_2_coefs_1_1_input_limb_13.clone())
                - combination_limb_3_col3.clone())
                * M31_16.clone()),
        );
        let carry_4_tmp_2db6d_6 = eval.add_intermediate(
            ((((carry_3_tmp_2db6d_5.clone()
                + linear_combination_n_2_coefs_1_1_input_limb_4.clone())
                + linear_combination_n_2_coefs_1_1_input_limb_14.clone())
                - combination_limb_4_col4.clone())
                * M31_16.clone()),
        );
        let carry_5_tmp_2db6d_7 = eval.add_intermediate(
            ((((carry_4_tmp_2db6d_6.clone()
                + linear_combination_n_2_coefs_1_1_input_limb_5.clone())
                + linear_combination_n_2_coefs_1_1_input_limb_15.clone())
                - combination_limb_5_col5.clone())
                * M31_16.clone()),
        );
        let carry_6_tmp_2db6d_8 = eval.add_intermediate(
            ((((carry_5_tmp_2db6d_7.clone()
                + linear_combination_n_2_coefs_1_1_input_limb_6.clone())
                + linear_combination_n_2_coefs_1_1_input_limb_16.clone())
                - combination_limb_6_col6.clone())
                * M31_16.clone()),
        );
        let carry_7_tmp_2db6d_9 = eval.add_intermediate(
            (((((carry_6_tmp_2db6d_8.clone()
                + linear_combination_n_2_coefs_1_1_input_limb_7.clone())
                + linear_combination_n_2_coefs_1_1_input_limb_17.clone())
                - combination_limb_7_col7.clone())
                - (p_coef_col10.clone() * M31_136.clone()))
                * M31_16.clone()),
        );
        let carry_8_tmp_2db6d_10 = eval.add_intermediate(
            ((((carry_7_tmp_2db6d_9.clone()
                + linear_combination_n_2_coefs_1_1_input_limb_8.clone())
                + linear_combination_n_2_coefs_1_1_input_limb_18.clone())
                - combination_limb_8_col8.clone())
                * M31_16.clone()),
        );
        // final limb constraint.
        eval.add_constraint(
            ((((carry_8_tmp_2db6d_10.clone()
                + linear_combination_n_2_coefs_1_1_input_limb_9.clone())
                + linear_combination_n_2_coefs_1_1_input_limb_19.clone())
                - combination_limb_9_col9.clone())
                - (p_coef_col10.clone() * M31_256.clone())),
        );
        // carry constraint 0.
        eval.add_constraint(
            (((p_coef_col10.clone() * p_coef_col10.clone()) * p_coef_col10.clone())
                - p_coef_col10.clone()),
        );
        let biased_carry_1_tmp_2db6d_11 = eval.add_intermediate(carry_0_tmp_2db6d_2.clone());
        // carry constraint 1.
        eval.add_constraint(
            (((biased_carry_1_tmp_2db6d_11.clone() * biased_carry_1_tmp_2db6d_11.clone())
                * biased_carry_1_tmp_2db6d_11.clone())
                - biased_carry_1_tmp_2db6d_11.clone()),
        );
        let biased_carry_2_tmp_2db6d_12 = eval.add_intermediate(carry_1_tmp_2db6d_3.clone());
        // carry constraint 2.
        eval.add_constraint(
            (((biased_carry_2_tmp_2db6d_12.clone() * biased_carry_2_tmp_2db6d_12.clone())
                * biased_carry_2_tmp_2db6d_12.clone())
                - biased_carry_2_tmp_2db6d_12.clone()),
        );
        let biased_carry_3_tmp_2db6d_13 = eval.add_intermediate(carry_2_tmp_2db6d_4.clone());
        // carry constraint 3.
        eval.add_constraint(
            (((biased_carry_3_tmp_2db6d_13.clone() * biased_carry_3_tmp_2db6d_13.clone())
                * biased_carry_3_tmp_2db6d_13.clone())
                - biased_carry_3_tmp_2db6d_13.clone()),
        );
        let biased_carry_4_tmp_2db6d_14 = eval.add_intermediate(carry_3_tmp_2db6d_5.clone());
        // carry constraint 4.
        eval.add_constraint(
            (((biased_carry_4_tmp_2db6d_14.clone() * biased_carry_4_tmp_2db6d_14.clone())
                * biased_carry_4_tmp_2db6d_14.clone())
                - biased_carry_4_tmp_2db6d_14.clone()),
        );
        let biased_carry_5_tmp_2db6d_15 = eval.add_intermediate(carry_4_tmp_2db6d_6.clone());
        // carry constraint 5.
        eval.add_constraint(
            (((biased_carry_5_tmp_2db6d_15.clone() * biased_carry_5_tmp_2db6d_15.clone())
                * biased_carry_5_tmp_2db6d_15.clone())
                - biased_carry_5_tmp_2db6d_15.clone()),
        );
        let biased_carry_6_tmp_2db6d_16 = eval.add_intermediate(carry_5_tmp_2db6d_7.clone());
        // carry constraint 6.
        eval.add_constraint(
            (((biased_carry_6_tmp_2db6d_16.clone() * biased_carry_6_tmp_2db6d_16.clone())
                * biased_carry_6_tmp_2db6d_16.clone())
                - biased_carry_6_tmp_2db6d_16.clone()),
        );
        let biased_carry_7_tmp_2db6d_17 = eval.add_intermediate(carry_6_tmp_2db6d_8.clone());
        // carry constraint 7.
        eval.add_constraint(
            (((biased_carry_7_tmp_2db6d_17.clone() * biased_carry_7_tmp_2db6d_17.clone())
                * biased_carry_7_tmp_2db6d_17.clone())
                - biased_carry_7_tmp_2db6d_17.clone()),
        );
        let biased_carry_8_tmp_2db6d_18 = eval.add_intermediate(carry_7_tmp_2db6d_9.clone());
        // carry constraint 8.
        eval.add_constraint(
            (((biased_carry_8_tmp_2db6d_18.clone() * biased_carry_8_tmp_2db6d_18.clone())
                * biased_carry_8_tmp_2db6d_18.clone())
                - biased_carry_8_tmp_2db6d_18.clone()),
        );
        let biased_carry_9_tmp_2db6d_19 = eval.add_intermediate(carry_8_tmp_2db6d_10.clone());
        // carry constraint 9.
        eval.add_constraint(
            (((biased_carry_9_tmp_2db6d_19.clone() * biased_carry_9_tmp_2db6d_19.clone())
                * biased_carry_9_tmp_2db6d_19.clone())
                - biased_carry_9_tmp_2db6d_19.clone()),
        );
        []
    }
}
