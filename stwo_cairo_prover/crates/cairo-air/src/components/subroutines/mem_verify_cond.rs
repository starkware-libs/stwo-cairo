// This file was created by the AIR team.

use crate::components::prelude::*;
use crate::components::subroutines::read_positive_num_bits_252::ReadPositiveNumBits252;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct MemVerifyCond {}

impl MemVerifyCond {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [mem_verify_cond_input_address, mem_verify_cond_input_value_limb_0, mem_verify_cond_input_value_limb_1, mem_verify_cond_input_value_limb_2, mem_verify_cond_input_value_limb_3, mem_verify_cond_input_value_limb_4, mem_verify_cond_input_value_limb_5, mem_verify_cond_input_value_limb_6, mem_verify_cond_input_value_limb_7, mem_verify_cond_input_value_limb_8, mem_verify_cond_input_value_limb_9, mem_verify_cond_input_value_limb_10, mem_verify_cond_input_value_limb_11, mem_verify_cond_input_value_limb_12, mem_verify_cond_input_value_limb_13, mem_verify_cond_input_value_limb_14, mem_verify_cond_input_value_limb_15, mem_verify_cond_input_value_limb_16, mem_verify_cond_input_value_limb_17, mem_verify_cond_input_value_limb_18, mem_verify_cond_input_value_limb_19, mem_verify_cond_input_value_limb_20, mem_verify_cond_input_value_limb_21, mem_verify_cond_input_value_limb_22, mem_verify_cond_input_value_limb_23, mem_verify_cond_input_value_limb_24, mem_verify_cond_input_value_limb_25, mem_verify_cond_input_value_limb_26, mem_verify_cond_input_value_limb_27, mem_verify_cond_input_cond]: [E::F; 30],
        cond_address_col0: E::F,
        cond_address_id_col1: E::F,
        cond_address_limb_0_col2: E::F,
        cond_address_limb_1_col3: E::F,
        cond_address_limb_2_col4: E::F,
        cond_address_limb_3_col5: E::F,
        cond_address_limb_4_col6: E::F,
        cond_address_limb_5_col7: E::F,
        cond_address_limb_6_col8: E::F,
        cond_address_limb_7_col9: E::F,
        cond_address_limb_8_col10: E::F,
        cond_address_limb_9_col11: E::F,
        cond_address_limb_10_col12: E::F,
        cond_address_limb_11_col13: E::F,
        cond_address_limb_12_col14: E::F,
        cond_address_limb_13_col15: E::F,
        cond_address_limb_14_col16: E::F,
        cond_address_limb_15_col17: E::F,
        cond_address_limb_16_col18: E::F,
        cond_address_limb_17_col19: E::F,
        cond_address_limb_18_col20: E::F,
        cond_address_limb_19_col21: E::F,
        cond_address_limb_20_col22: E::F,
        cond_address_limb_21_col23: E::F,
        cond_address_limb_22_col24: E::F,
        cond_address_limb_23_col25: E::F,
        cond_address_limb_24_col26: E::F,
        cond_address_limb_25_col27: E::F,
        cond_address_limb_26_col28: E::F,
        cond_address_limb_27_col29: E::F,
        common_lookup_elements: &relations::CommonLookupElements,
        eval: &mut E,
    ) -> [E::F; 0] {
        let M31_1 = E::F::from(M31::from(1));

        // cond=0 or cond=1..
        eval.add_constraint(
            (mem_verify_cond_input_cond.clone()
                * (M31_1.clone() - mem_verify_cond_input_cond.clone())),
        );
        // cond_address.
        eval.add_constraint(
            (cond_address_col0.clone()
                - (((mem_verify_cond_input_address.clone() - M31_1.clone())
                    * mem_verify_cond_input_cond.clone())
                    + M31_1.clone())),
        );
        ReadPositiveNumBits252::evaluate(
            [cond_address_col0.clone()],
            cond_address_id_col1.clone(),
            cond_address_limb_0_col2.clone(),
            cond_address_limb_1_col3.clone(),
            cond_address_limb_2_col4.clone(),
            cond_address_limb_3_col5.clone(),
            cond_address_limb_4_col6.clone(),
            cond_address_limb_5_col7.clone(),
            cond_address_limb_6_col8.clone(),
            cond_address_limb_7_col9.clone(),
            cond_address_limb_8_col10.clone(),
            cond_address_limb_9_col11.clone(),
            cond_address_limb_10_col12.clone(),
            cond_address_limb_11_col13.clone(),
            cond_address_limb_12_col14.clone(),
            cond_address_limb_13_col15.clone(),
            cond_address_limb_14_col16.clone(),
            cond_address_limb_15_col17.clone(),
            cond_address_limb_16_col18.clone(),
            cond_address_limb_17_col19.clone(),
            cond_address_limb_18_col20.clone(),
            cond_address_limb_19_col21.clone(),
            cond_address_limb_20_col22.clone(),
            cond_address_limb_21_col23.clone(),
            cond_address_limb_22_col24.clone(),
            cond_address_limb_23_col25.clone(),
            cond_address_limb_24_col26.clone(),
            cond_address_limb_25_col27.clone(),
            cond_address_limb_26_col28.clone(),
            cond_address_limb_27_col29.clone(),
            common_lookup_elements,
            eval,
        );
        // felt252 limb is zero.
        eval.add_constraint(
            ((cond_address_limb_0_col2.clone() - mem_verify_cond_input_value_limb_0.clone())
                * mem_verify_cond_input_cond.clone()),
        );
        // felt252 limb is zero.
        eval.add_constraint(
            ((cond_address_limb_1_col3.clone() - mem_verify_cond_input_value_limb_1.clone())
                * mem_verify_cond_input_cond.clone()),
        );
        // felt252 limb is zero.
        eval.add_constraint(
            ((cond_address_limb_2_col4.clone() - mem_verify_cond_input_value_limb_2.clone())
                * mem_verify_cond_input_cond.clone()),
        );
        // felt252 limb is zero.
        eval.add_constraint(
            ((cond_address_limb_3_col5.clone() - mem_verify_cond_input_value_limb_3.clone())
                * mem_verify_cond_input_cond.clone()),
        );
        // felt252 limb is zero.
        eval.add_constraint(
            ((cond_address_limb_4_col6.clone() - mem_verify_cond_input_value_limb_4.clone())
                * mem_verify_cond_input_cond.clone()),
        );
        // felt252 limb is zero.
        eval.add_constraint(
            ((cond_address_limb_5_col7.clone() - mem_verify_cond_input_value_limb_5.clone())
                * mem_verify_cond_input_cond.clone()),
        );
        // felt252 limb is zero.
        eval.add_constraint(
            ((cond_address_limb_6_col8.clone() - mem_verify_cond_input_value_limb_6.clone())
                * mem_verify_cond_input_cond.clone()),
        );
        // felt252 limb is zero.
        eval.add_constraint(
            ((cond_address_limb_7_col9.clone() - mem_verify_cond_input_value_limb_7.clone())
                * mem_verify_cond_input_cond.clone()),
        );
        // felt252 limb is zero.
        eval.add_constraint(
            ((cond_address_limb_8_col10.clone() - mem_verify_cond_input_value_limb_8.clone())
                * mem_verify_cond_input_cond.clone()),
        );
        // felt252 limb is zero.
        eval.add_constraint(
            ((cond_address_limb_9_col11.clone() - mem_verify_cond_input_value_limb_9.clone())
                * mem_verify_cond_input_cond.clone()),
        );
        // felt252 limb is zero.
        eval.add_constraint(
            ((cond_address_limb_10_col12.clone() - mem_verify_cond_input_value_limb_10.clone())
                * mem_verify_cond_input_cond.clone()),
        );
        // felt252 limb is zero.
        eval.add_constraint(
            ((cond_address_limb_11_col13.clone() - mem_verify_cond_input_value_limb_11.clone())
                * mem_verify_cond_input_cond.clone()),
        );
        // felt252 limb is zero.
        eval.add_constraint(
            ((cond_address_limb_12_col14.clone() - mem_verify_cond_input_value_limb_12.clone())
                * mem_verify_cond_input_cond.clone()),
        );
        // felt252 limb is zero.
        eval.add_constraint(
            ((cond_address_limb_13_col15.clone() - mem_verify_cond_input_value_limb_13.clone())
                * mem_verify_cond_input_cond.clone()),
        );
        // felt252 limb is zero.
        eval.add_constraint(
            ((cond_address_limb_14_col16.clone() - mem_verify_cond_input_value_limb_14.clone())
                * mem_verify_cond_input_cond.clone()),
        );
        // felt252 limb is zero.
        eval.add_constraint(
            ((cond_address_limb_15_col17.clone() - mem_verify_cond_input_value_limb_15.clone())
                * mem_verify_cond_input_cond.clone()),
        );
        // felt252 limb is zero.
        eval.add_constraint(
            ((cond_address_limb_16_col18.clone() - mem_verify_cond_input_value_limb_16.clone())
                * mem_verify_cond_input_cond.clone()),
        );
        // felt252 limb is zero.
        eval.add_constraint(
            ((cond_address_limb_17_col19.clone() - mem_verify_cond_input_value_limb_17.clone())
                * mem_verify_cond_input_cond.clone()),
        );
        // felt252 limb is zero.
        eval.add_constraint(
            ((cond_address_limb_18_col20.clone() - mem_verify_cond_input_value_limb_18.clone())
                * mem_verify_cond_input_cond.clone()),
        );
        // felt252 limb is zero.
        eval.add_constraint(
            ((cond_address_limb_19_col21.clone() - mem_verify_cond_input_value_limb_19.clone())
                * mem_verify_cond_input_cond.clone()),
        );
        // felt252 limb is zero.
        eval.add_constraint(
            ((cond_address_limb_20_col22.clone() - mem_verify_cond_input_value_limb_20.clone())
                * mem_verify_cond_input_cond.clone()),
        );
        // felt252 limb is zero.
        eval.add_constraint(
            ((cond_address_limb_21_col23.clone() - mem_verify_cond_input_value_limb_21.clone())
                * mem_verify_cond_input_cond.clone()),
        );
        // felt252 limb is zero.
        eval.add_constraint(
            ((cond_address_limb_22_col24.clone() - mem_verify_cond_input_value_limb_22.clone())
                * mem_verify_cond_input_cond.clone()),
        );
        // felt252 limb is zero.
        eval.add_constraint(
            ((cond_address_limb_23_col25.clone() - mem_verify_cond_input_value_limb_23.clone())
                * mem_verify_cond_input_cond.clone()),
        );
        // felt252 limb is zero.
        eval.add_constraint(
            ((cond_address_limb_24_col26.clone() - mem_verify_cond_input_value_limb_24.clone())
                * mem_verify_cond_input_cond.clone()),
        );
        // felt252 limb is zero.
        eval.add_constraint(
            ((cond_address_limb_25_col27.clone() - mem_verify_cond_input_value_limb_25.clone())
                * mem_verify_cond_input_cond.clone()),
        );
        // felt252 limb is zero.
        eval.add_constraint(
            ((cond_address_limb_26_col28.clone() - mem_verify_cond_input_value_limb_26.clone())
                * mem_verify_cond_input_cond.clone()),
        );
        // felt252 limb is zero.
        eval.add_constraint(
            ((cond_address_limb_27_col29.clone() - mem_verify_cond_input_value_limb_27.clone())
                * mem_verify_cond_input_cond.clone()),
        );
        []
    }
}
