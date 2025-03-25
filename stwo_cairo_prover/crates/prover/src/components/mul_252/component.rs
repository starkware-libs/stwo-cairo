use crate::components::prelude::constraint_eval::*;
use crate::components::range_check_mem_value_n_28::component::RangeCheckMemValueN28;
use crate::components::verify_mul_252::component::VerifyMul252;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct Mul252 {}

impl Mul252 {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    pub fn evaluate<E: EvalAtRow>(
        [mul_252_input_limb_0, mul_252_input_limb_1, mul_252_input_limb_2, mul_252_input_limb_3, mul_252_input_limb_4, mul_252_input_limb_5, mul_252_input_limb_6, mul_252_input_limb_7, mul_252_input_limb_8, mul_252_input_limb_9, mul_252_input_limb_10, mul_252_input_limb_11, mul_252_input_limb_12, mul_252_input_limb_13, mul_252_input_limb_14, mul_252_input_limb_15, mul_252_input_limb_16, mul_252_input_limb_17, mul_252_input_limb_18, mul_252_input_limb_19, mul_252_input_limb_20, mul_252_input_limb_21, mul_252_input_limb_22, mul_252_input_limb_23, mul_252_input_limb_24, mul_252_input_limb_25, mul_252_input_limb_26, mul_252_input_limb_27, mul_252_input_limb_28, mul_252_input_limb_29, mul_252_input_limb_30, mul_252_input_limb_31, mul_252_input_limb_32, mul_252_input_limb_33, mul_252_input_limb_34, mul_252_input_limb_35, mul_252_input_limb_36, mul_252_input_limb_37, mul_252_input_limb_38, mul_252_input_limb_39, mul_252_input_limb_40, mul_252_input_limb_41, mul_252_input_limb_42, mul_252_input_limb_43, mul_252_input_limb_44, mul_252_input_limb_45, mul_252_input_limb_46, mul_252_input_limb_47, mul_252_input_limb_48, mul_252_input_limb_49, mul_252_input_limb_50, mul_252_input_limb_51, mul_252_input_limb_52, mul_252_input_limb_53, mul_252_input_limb_54, mul_252_input_limb_55]: [E::F; 56],
        mul_res_limb_0_col0: E::F,
        mul_res_limb_1_col1: E::F,
        mul_res_limb_2_col2: E::F,
        mul_res_limb_3_col3: E::F,
        mul_res_limb_4_col4: E::F,
        mul_res_limb_5_col5: E::F,
        mul_res_limb_6_col6: E::F,
        mul_res_limb_7_col7: E::F,
        mul_res_limb_8_col8: E::F,
        mul_res_limb_9_col9: E::F,
        mul_res_limb_10_col10: E::F,
        mul_res_limb_11_col11: E::F,
        mul_res_limb_12_col12: E::F,
        mul_res_limb_13_col13: E::F,
        mul_res_limb_14_col14: E::F,
        mul_res_limb_15_col15: E::F,
        mul_res_limb_16_col16: E::F,
        mul_res_limb_17_col17: E::F,
        mul_res_limb_18_col18: E::F,
        mul_res_limb_19_col19: E::F,
        mul_res_limb_20_col20: E::F,
        mul_res_limb_21_col21: E::F,
        mul_res_limb_22_col22: E::F,
        mul_res_limb_23_col23: E::F,
        mul_res_limb_24_col24: E::F,
        mul_res_limb_25_col25: E::F,
        mul_res_limb_26_col26: E::F,
        mul_res_limb_27_col27: E::F,
        k_col28: E::F,
        carry_0_col29: E::F,
        carry_1_col30: E::F,
        carry_2_col31: E::F,
        carry_3_col32: E::F,
        carry_4_col33: E::F,
        carry_5_col34: E::F,
        carry_6_col35: E::F,
        carry_7_col36: E::F,
        carry_8_col37: E::F,
        carry_9_col38: E::F,
        carry_10_col39: E::F,
        carry_11_col40: E::F,
        carry_12_col41: E::F,
        carry_13_col42: E::F,
        carry_14_col43: E::F,
        carry_15_col44: E::F,
        carry_16_col45: E::F,
        carry_17_col46: E::F,
        carry_18_col47: E::F,
        carry_19_col48: E::F,
        carry_20_col49: E::F,
        carry_21_col50: E::F,
        carry_22_col51: E::F,
        carry_23_col52: E::F,
        carry_24_col53: E::F,
        carry_25_col54: E::F,
        carry_26_col55: E::F,
        eval: &mut E,
        range_check_19_lookup_elements: &relations::RangeCheck_19,
        range_check_9_9_lookup_elements: &relations::RangeCheck_9_9,
    ) -> [E::F; 28] {
        let () = RangeCheckMemValueN28::evaluate(
            [
                mul_res_limb_0_col0.clone(),
                mul_res_limb_1_col1.clone(),
                mul_res_limb_2_col2.clone(),
                mul_res_limb_3_col3.clone(),
                mul_res_limb_4_col4.clone(),
                mul_res_limb_5_col5.clone(),
                mul_res_limb_6_col6.clone(),
                mul_res_limb_7_col7.clone(),
                mul_res_limb_8_col8.clone(),
                mul_res_limb_9_col9.clone(),
                mul_res_limb_10_col10.clone(),
                mul_res_limb_11_col11.clone(),
                mul_res_limb_12_col12.clone(),
                mul_res_limb_13_col13.clone(),
                mul_res_limb_14_col14.clone(),
                mul_res_limb_15_col15.clone(),
                mul_res_limb_16_col16.clone(),
                mul_res_limb_17_col17.clone(),
                mul_res_limb_18_col18.clone(),
                mul_res_limb_19_col19.clone(),
                mul_res_limb_20_col20.clone(),
                mul_res_limb_21_col21.clone(),
                mul_res_limb_22_col22.clone(),
                mul_res_limb_23_col23.clone(),
                mul_res_limb_24_col24.clone(),
                mul_res_limb_25_col25.clone(),
                mul_res_limb_26_col26.clone(),
                mul_res_limb_27_col27.clone(),
            ],
            eval,
            range_check_9_9_lookup_elements,
        );
        let () = VerifyMul252::evaluate(
            [
                mul_252_input_limb_0.clone(),
                mul_252_input_limb_1.clone(),
                mul_252_input_limb_2.clone(),
                mul_252_input_limb_3.clone(),
                mul_252_input_limb_4.clone(),
                mul_252_input_limb_5.clone(),
                mul_252_input_limb_6.clone(),
                mul_252_input_limb_7.clone(),
                mul_252_input_limb_8.clone(),
                mul_252_input_limb_9.clone(),
                mul_252_input_limb_10.clone(),
                mul_252_input_limb_11.clone(),
                mul_252_input_limb_12.clone(),
                mul_252_input_limb_13.clone(),
                mul_252_input_limb_14.clone(),
                mul_252_input_limb_15.clone(),
                mul_252_input_limb_16.clone(),
                mul_252_input_limb_17.clone(),
                mul_252_input_limb_18.clone(),
                mul_252_input_limb_19.clone(),
                mul_252_input_limb_20.clone(),
                mul_252_input_limb_21.clone(),
                mul_252_input_limb_22.clone(),
                mul_252_input_limb_23.clone(),
                mul_252_input_limb_24.clone(),
                mul_252_input_limb_25.clone(),
                mul_252_input_limb_26.clone(),
                mul_252_input_limb_27.clone(),
                mul_252_input_limb_28.clone(),
                mul_252_input_limb_29.clone(),
                mul_252_input_limb_30.clone(),
                mul_252_input_limb_31.clone(),
                mul_252_input_limb_32.clone(),
                mul_252_input_limb_33.clone(),
                mul_252_input_limb_34.clone(),
                mul_252_input_limb_35.clone(),
                mul_252_input_limb_36.clone(),
                mul_252_input_limb_37.clone(),
                mul_252_input_limb_38.clone(),
                mul_252_input_limb_39.clone(),
                mul_252_input_limb_40.clone(),
                mul_252_input_limb_41.clone(),
                mul_252_input_limb_42.clone(),
                mul_252_input_limb_43.clone(),
                mul_252_input_limb_44.clone(),
                mul_252_input_limb_45.clone(),
                mul_252_input_limb_46.clone(),
                mul_252_input_limb_47.clone(),
                mul_252_input_limb_48.clone(),
                mul_252_input_limb_49.clone(),
                mul_252_input_limb_50.clone(),
                mul_252_input_limb_51.clone(),
                mul_252_input_limb_52.clone(),
                mul_252_input_limb_53.clone(),
                mul_252_input_limb_54.clone(),
                mul_252_input_limb_55.clone(),
                mul_res_limb_0_col0.clone(),
                mul_res_limb_1_col1.clone(),
                mul_res_limb_2_col2.clone(),
                mul_res_limb_3_col3.clone(),
                mul_res_limb_4_col4.clone(),
                mul_res_limb_5_col5.clone(),
                mul_res_limb_6_col6.clone(),
                mul_res_limb_7_col7.clone(),
                mul_res_limb_8_col8.clone(),
                mul_res_limb_9_col9.clone(),
                mul_res_limb_10_col10.clone(),
                mul_res_limb_11_col11.clone(),
                mul_res_limb_12_col12.clone(),
                mul_res_limb_13_col13.clone(),
                mul_res_limb_14_col14.clone(),
                mul_res_limb_15_col15.clone(),
                mul_res_limb_16_col16.clone(),
                mul_res_limb_17_col17.clone(),
                mul_res_limb_18_col18.clone(),
                mul_res_limb_19_col19.clone(),
                mul_res_limb_20_col20.clone(),
                mul_res_limb_21_col21.clone(),
                mul_res_limb_22_col22.clone(),
                mul_res_limb_23_col23.clone(),
                mul_res_limb_24_col24.clone(),
                mul_res_limb_25_col25.clone(),
                mul_res_limb_26_col26.clone(),
                mul_res_limb_27_col27.clone(),
            ],
            k_col28.clone(),
            carry_0_col29.clone(),
            carry_1_col30.clone(),
            carry_2_col31.clone(),
            carry_3_col32.clone(),
            carry_4_col33.clone(),
            carry_5_col34.clone(),
            carry_6_col35.clone(),
            carry_7_col36.clone(),
            carry_8_col37.clone(),
            carry_9_col38.clone(),
            carry_10_col39.clone(),
            carry_11_col40.clone(),
            carry_12_col41.clone(),
            carry_13_col42.clone(),
            carry_14_col43.clone(),
            carry_15_col44.clone(),
            carry_16_col45.clone(),
            carry_17_col46.clone(),
            carry_18_col47.clone(),
            carry_19_col48.clone(),
            carry_20_col49.clone(),
            carry_21_col50.clone(),
            carry_22_col51.clone(),
            carry_23_col52.clone(),
            carry_24_col53.clone(),
            carry_25_col54.clone(),
            carry_26_col55.clone(),
            eval,
            range_check_19_lookup_elements,
        );
        [
            mul_res_limb_0_col0.clone(),
            mul_res_limb_1_col1.clone(),
            mul_res_limb_2_col2.clone(),
            mul_res_limb_3_col3.clone(),
            mul_res_limb_4_col4.clone(),
            mul_res_limb_5_col5.clone(),
            mul_res_limb_6_col6.clone(),
            mul_res_limb_7_col7.clone(),
            mul_res_limb_8_col8.clone(),
            mul_res_limb_9_col9.clone(),
            mul_res_limb_10_col10.clone(),
            mul_res_limb_11_col11.clone(),
            mul_res_limb_12_col12.clone(),
            mul_res_limb_13_col13.clone(),
            mul_res_limb_14_col14.clone(),
            mul_res_limb_15_col15.clone(),
            mul_res_limb_16_col16.clone(),
            mul_res_limb_17_col17.clone(),
            mul_res_limb_18_col18.clone(),
            mul_res_limb_19_col19.clone(),
            mul_res_limb_20_col20.clone(),
            mul_res_limb_21_col21.clone(),
            mul_res_limb_22_col22.clone(),
            mul_res_limb_23_col23.clone(),
            mul_res_limb_24_col24.clone(),
            mul_res_limb_25_col25.clone(),
            mul_res_limb_26_col26.clone(),
            mul_res_limb_27_col27.clone(),
        ]
    }
}
