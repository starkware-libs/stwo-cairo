// This file was created by the AIR team.

use crate::components::prelude::*;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct LinearCombinationN4Coefs3111 {}

impl LinearCombinationN4Coefs3111 {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [linear_combination_n_4_coefs_3_1_1_1_input_limb_0, linear_combination_n_4_coefs_3_1_1_1_input_limb_1, linear_combination_n_4_coefs_3_1_1_1_input_limb_2, linear_combination_n_4_coefs_3_1_1_1_input_limb_3, linear_combination_n_4_coefs_3_1_1_1_input_limb_4, linear_combination_n_4_coefs_3_1_1_1_input_limb_5, linear_combination_n_4_coefs_3_1_1_1_input_limb_6, linear_combination_n_4_coefs_3_1_1_1_input_limb_7, linear_combination_n_4_coefs_3_1_1_1_input_limb_8, linear_combination_n_4_coefs_3_1_1_1_input_limb_9, linear_combination_n_4_coefs_3_1_1_1_input_limb_10, linear_combination_n_4_coefs_3_1_1_1_input_limb_11, linear_combination_n_4_coefs_3_1_1_1_input_limb_12, linear_combination_n_4_coefs_3_1_1_1_input_limb_13, linear_combination_n_4_coefs_3_1_1_1_input_limb_14, linear_combination_n_4_coefs_3_1_1_1_input_limb_15, linear_combination_n_4_coefs_3_1_1_1_input_limb_16, linear_combination_n_4_coefs_3_1_1_1_input_limb_17, linear_combination_n_4_coefs_3_1_1_1_input_limb_18, linear_combination_n_4_coefs_3_1_1_1_input_limb_19, linear_combination_n_4_coefs_3_1_1_1_input_limb_20, linear_combination_n_4_coefs_3_1_1_1_input_limb_21, linear_combination_n_4_coefs_3_1_1_1_input_limb_22, linear_combination_n_4_coefs_3_1_1_1_input_limb_23, linear_combination_n_4_coefs_3_1_1_1_input_limb_24, linear_combination_n_4_coefs_3_1_1_1_input_limb_25, linear_combination_n_4_coefs_3_1_1_1_input_limb_26, linear_combination_n_4_coefs_3_1_1_1_input_limb_27, linear_combination_n_4_coefs_3_1_1_1_input_limb_28, linear_combination_n_4_coefs_3_1_1_1_input_limb_29, linear_combination_n_4_coefs_3_1_1_1_input_limb_30, linear_combination_n_4_coefs_3_1_1_1_input_limb_31, linear_combination_n_4_coefs_3_1_1_1_input_limb_32, linear_combination_n_4_coefs_3_1_1_1_input_limb_33, linear_combination_n_4_coefs_3_1_1_1_input_limb_34, linear_combination_n_4_coefs_3_1_1_1_input_limb_35, linear_combination_n_4_coefs_3_1_1_1_input_limb_36, linear_combination_n_4_coefs_3_1_1_1_input_limb_37, linear_combination_n_4_coefs_3_1_1_1_input_limb_38, linear_combination_n_4_coefs_3_1_1_1_input_limb_39]: [E::F; 40],
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
        range_check_3_3_3_3_3_lookup_elements: &relations::RangeCheck_3_3_3_3_3,
        eval: &mut E,
    ) -> [E::F; 0] {
        let M31_1 = E::F::from(M31::from(1));
        let M31_136 = E::F::from(M31::from(136));
        let M31_16 = E::F::from(M31::from(16));
        let M31_256 = E::F::from(M31::from(256));
        let M31_3 = E::F::from(M31::from(3));

        let carry_0_tmp_f4f81_2 = eval.add_intermediate(
            (((((((M31_3.clone() * linear_combination_n_4_coefs_3_1_1_1_input_limb_0.clone())
                + linear_combination_n_4_coefs_3_1_1_1_input_limb_10.clone())
                + linear_combination_n_4_coefs_3_1_1_1_input_limb_20.clone())
                + linear_combination_n_4_coefs_3_1_1_1_input_limb_30.clone())
                - combination_limb_0_col0.clone())
                - p_coef_col10.clone())
                * M31_16.clone()),
        );
        let carry_1_tmp_f4f81_3 = eval.add_intermediate(
            ((((((carry_0_tmp_f4f81_2.clone()
                + (M31_3.clone() * linear_combination_n_4_coefs_3_1_1_1_input_limb_1.clone()))
                + linear_combination_n_4_coefs_3_1_1_1_input_limb_11.clone())
                + linear_combination_n_4_coefs_3_1_1_1_input_limb_21.clone())
                + linear_combination_n_4_coefs_3_1_1_1_input_limb_31.clone())
                - combination_limb_1_col1.clone())
                * M31_16.clone()),
        );
        let carry_2_tmp_f4f81_4 = eval.add_intermediate(
            ((((((carry_1_tmp_f4f81_3.clone()
                + (M31_3.clone() * linear_combination_n_4_coefs_3_1_1_1_input_limb_2.clone()))
                + linear_combination_n_4_coefs_3_1_1_1_input_limb_12.clone())
                + linear_combination_n_4_coefs_3_1_1_1_input_limb_22.clone())
                + linear_combination_n_4_coefs_3_1_1_1_input_limb_32.clone())
                - combination_limb_2_col2.clone())
                * M31_16.clone()),
        );
        let carry_3_tmp_f4f81_5 = eval.add_intermediate(
            ((((((carry_2_tmp_f4f81_4.clone()
                + (M31_3.clone() * linear_combination_n_4_coefs_3_1_1_1_input_limb_3.clone()))
                + linear_combination_n_4_coefs_3_1_1_1_input_limb_13.clone())
                + linear_combination_n_4_coefs_3_1_1_1_input_limb_23.clone())
                + linear_combination_n_4_coefs_3_1_1_1_input_limb_33.clone())
                - combination_limb_3_col3.clone())
                * M31_16.clone()),
        );
        let carry_4_tmp_f4f81_6 = eval.add_intermediate(
            ((((((carry_3_tmp_f4f81_5.clone()
                + (M31_3.clone() * linear_combination_n_4_coefs_3_1_1_1_input_limb_4.clone()))
                + linear_combination_n_4_coefs_3_1_1_1_input_limb_14.clone())
                + linear_combination_n_4_coefs_3_1_1_1_input_limb_24.clone())
                + linear_combination_n_4_coefs_3_1_1_1_input_limb_34.clone())
                - combination_limb_4_col4.clone())
                * M31_16.clone()),
        );
        let carry_5_tmp_f4f81_7 = eval.add_intermediate(
            ((((((carry_4_tmp_f4f81_6.clone()
                + (M31_3.clone() * linear_combination_n_4_coefs_3_1_1_1_input_limb_5.clone()))
                + linear_combination_n_4_coefs_3_1_1_1_input_limb_15.clone())
                + linear_combination_n_4_coefs_3_1_1_1_input_limb_25.clone())
                + linear_combination_n_4_coefs_3_1_1_1_input_limb_35.clone())
                - combination_limb_5_col5.clone())
                * M31_16.clone()),
        );
        let carry_6_tmp_f4f81_8 = eval.add_intermediate(
            ((((((carry_5_tmp_f4f81_7.clone()
                + (M31_3.clone() * linear_combination_n_4_coefs_3_1_1_1_input_limb_6.clone()))
                + linear_combination_n_4_coefs_3_1_1_1_input_limb_16.clone())
                + linear_combination_n_4_coefs_3_1_1_1_input_limb_26.clone())
                + linear_combination_n_4_coefs_3_1_1_1_input_limb_36.clone())
                - combination_limb_6_col6.clone())
                * M31_16.clone()),
        );
        let carry_7_tmp_f4f81_9 = eval.add_intermediate(
            (((((((carry_6_tmp_f4f81_8.clone()
                + (M31_3.clone()
                    * linear_combination_n_4_coefs_3_1_1_1_input_limb_7.clone()))
                + linear_combination_n_4_coefs_3_1_1_1_input_limb_17.clone())
                + linear_combination_n_4_coefs_3_1_1_1_input_limb_27.clone())
                + linear_combination_n_4_coefs_3_1_1_1_input_limb_37.clone())
                - combination_limb_7_col7.clone())
                - (p_coef_col10.clone() * M31_136.clone()))
                * M31_16.clone()),
        );
        let carry_8_tmp_f4f81_10 = eval.add_intermediate(
            ((((((carry_7_tmp_f4f81_9.clone()
                + (M31_3.clone() * linear_combination_n_4_coefs_3_1_1_1_input_limb_8.clone()))
                + linear_combination_n_4_coefs_3_1_1_1_input_limb_18.clone())
                + linear_combination_n_4_coefs_3_1_1_1_input_limb_28.clone())
                + linear_combination_n_4_coefs_3_1_1_1_input_limb_38.clone())
                - combination_limb_8_col8.clone())
                * M31_16.clone()),
        );
        // final limb constraint.
        eval.add_constraint(
            ((((((carry_8_tmp_f4f81_10.clone()
                + (M31_3.clone() * linear_combination_n_4_coefs_3_1_1_1_input_limb_9.clone()))
                + linear_combination_n_4_coefs_3_1_1_1_input_limb_19.clone())
                + linear_combination_n_4_coefs_3_1_1_1_input_limb_29.clone())
                + linear_combination_n_4_coefs_3_1_1_1_input_limb_39.clone())
                - combination_limb_9_col9.clone())
                - (p_coef_col10.clone() * M31_256.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            range_check_3_3_3_3_3_lookup_elements,
            E::EF::one(),
            &[
                (p_coef_col10.clone() + M31_1.clone()),
                (carry_0_tmp_f4f81_2.clone() + M31_1.clone()),
                (carry_1_tmp_f4f81_3.clone() + M31_1.clone()),
                (carry_2_tmp_f4f81_4.clone() + M31_1.clone()),
                (carry_3_tmp_f4f81_5.clone() + M31_1.clone()),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            range_check_3_3_3_3_3_lookup_elements,
            E::EF::one(),
            &[
                (carry_4_tmp_f4f81_6.clone() + M31_1.clone()),
                (carry_5_tmp_f4f81_7.clone() + M31_1.clone()),
                (carry_6_tmp_f4f81_8.clone() + M31_1.clone()),
                (carry_7_tmp_f4f81_9.clone() + M31_1.clone()),
                (carry_8_tmp_f4f81_10.clone() + M31_1.clone()),
            ],
        ));

        []
    }
}
