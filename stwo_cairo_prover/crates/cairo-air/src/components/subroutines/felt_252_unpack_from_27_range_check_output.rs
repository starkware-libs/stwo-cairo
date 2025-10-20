// This file was created by the AIR team.

use crate::components::prelude::*;
use crate::components::subroutines::range_check_mem_value_n_28::RangeCheckMemValueN28;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct Felt252UnpackFrom27RangeCheckOutput {}

impl Felt252UnpackFrom27RangeCheckOutput {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [felt_252_unpack_from_27_range_check_output_input_limb_0, felt_252_unpack_from_27_range_check_output_input_limb_1, felt_252_unpack_from_27_range_check_output_input_limb_2, felt_252_unpack_from_27_range_check_output_input_limb_3, felt_252_unpack_from_27_range_check_output_input_limb_4, felt_252_unpack_from_27_range_check_output_input_limb_5, felt_252_unpack_from_27_range_check_output_input_limb_6, felt_252_unpack_from_27_range_check_output_input_limb_7, felt_252_unpack_from_27_range_check_output_input_limb_8, felt_252_unpack_from_27_range_check_output_input_limb_9]: [E::F; 10],
        unpacked_limb_0_col0: E::F,
        unpacked_limb_1_col1: E::F,
        unpacked_limb_3_col2: E::F,
        unpacked_limb_4_col3: E::F,
        unpacked_limb_6_col4: E::F,
        unpacked_limb_7_col5: E::F,
        unpacked_limb_9_col6: E::F,
        unpacked_limb_10_col7: E::F,
        unpacked_limb_12_col8: E::F,
        unpacked_limb_13_col9: E::F,
        unpacked_limb_15_col10: E::F,
        unpacked_limb_16_col11: E::F,
        unpacked_limb_18_col12: E::F,
        unpacked_limb_19_col13: E::F,
        unpacked_limb_21_col14: E::F,
        unpacked_limb_22_col15: E::F,
        unpacked_limb_24_col16: E::F,
        unpacked_limb_25_col17: E::F,
        range_check_9_9_lookup_elements: &relations::RangeCheck_9_9,
        range_check_9_9_b_lookup_elements: &relations::RangeCheck_9_9_B,
        range_check_9_9_c_lookup_elements: &relations::RangeCheck_9_9_C,
        range_check_9_9_d_lookup_elements: &relations::RangeCheck_9_9_D,
        range_check_9_9_e_lookup_elements: &relations::RangeCheck_9_9_E,
        range_check_9_9_f_lookup_elements: &relations::RangeCheck_9_9_F,
        range_check_9_9_g_lookup_elements: &relations::RangeCheck_9_9_G,
        range_check_9_9_h_lookup_elements: &relations::RangeCheck_9_9_H,
        eval: &mut E,
    ) -> [E::F; 10] {
        let M31_512 = E::F::from(M31::from(512));
        let M31_8192 = E::F::from(M31::from(8192));

        let unpacked_tmp_4f7f8_1_limb_2 = eval.add_intermediate(
            (((felt_252_unpack_from_27_range_check_output_input_limb_0.clone()
                - unpacked_limb_0_col0.clone())
                - (unpacked_limb_1_col1.clone() * M31_512.clone()))
                * M31_8192.clone()),
        );
        let unpacked_tmp_4f7f8_1_limb_5 = eval.add_intermediate(
            (((felt_252_unpack_from_27_range_check_output_input_limb_1.clone()
                - unpacked_limb_3_col2.clone())
                - (unpacked_limb_4_col3.clone() * M31_512.clone()))
                * M31_8192.clone()),
        );
        let unpacked_tmp_4f7f8_1_limb_8 = eval.add_intermediate(
            (((felt_252_unpack_from_27_range_check_output_input_limb_2.clone()
                - unpacked_limb_6_col4.clone())
                - (unpacked_limb_7_col5.clone() * M31_512.clone()))
                * M31_8192.clone()),
        );
        let unpacked_tmp_4f7f8_1_limb_11 = eval.add_intermediate(
            (((felt_252_unpack_from_27_range_check_output_input_limb_3.clone()
                - unpacked_limb_9_col6.clone())
                - (unpacked_limb_10_col7.clone() * M31_512.clone()))
                * M31_8192.clone()),
        );
        let unpacked_tmp_4f7f8_1_limb_14 = eval.add_intermediate(
            (((felt_252_unpack_from_27_range_check_output_input_limb_4.clone()
                - unpacked_limb_12_col8.clone())
                - (unpacked_limb_13_col9.clone() * M31_512.clone()))
                * M31_8192.clone()),
        );
        let unpacked_tmp_4f7f8_1_limb_17 = eval.add_intermediate(
            (((felt_252_unpack_from_27_range_check_output_input_limb_5.clone()
                - unpacked_limb_15_col10.clone())
                - (unpacked_limb_16_col11.clone() * M31_512.clone()))
                * M31_8192.clone()),
        );
        let unpacked_tmp_4f7f8_1_limb_20 = eval.add_intermediate(
            (((felt_252_unpack_from_27_range_check_output_input_limb_6.clone()
                - unpacked_limb_18_col12.clone())
                - (unpacked_limb_19_col13.clone() * M31_512.clone()))
                * M31_8192.clone()),
        );
        let unpacked_tmp_4f7f8_1_limb_23 = eval.add_intermediate(
            (((felt_252_unpack_from_27_range_check_output_input_limb_7.clone()
                - unpacked_limb_21_col14.clone())
                - (unpacked_limb_22_col15.clone() * M31_512.clone()))
                * M31_8192.clone()),
        );
        let unpacked_tmp_4f7f8_1_limb_26 = eval.add_intermediate(
            (((felt_252_unpack_from_27_range_check_output_input_limb_8.clone()
                - unpacked_limb_24_col16.clone())
                - (unpacked_limb_25_col17.clone() * M31_512.clone()))
                * M31_8192.clone()),
        );
        let unpacked_tmp_4f7f8_1_limb_27 =
            eval.add_intermediate(felt_252_unpack_from_27_range_check_output_input_limb_9.clone());
        RangeCheckMemValueN28::evaluate(
            [
                unpacked_limb_0_col0.clone(),
                unpacked_limb_1_col1.clone(),
                unpacked_tmp_4f7f8_1_limb_2.clone(),
                unpacked_limb_3_col2.clone(),
                unpacked_limb_4_col3.clone(),
                unpacked_tmp_4f7f8_1_limb_5.clone(),
                unpacked_limb_6_col4.clone(),
                unpacked_limb_7_col5.clone(),
                unpacked_tmp_4f7f8_1_limb_8.clone(),
                unpacked_limb_9_col6.clone(),
                unpacked_limb_10_col7.clone(),
                unpacked_tmp_4f7f8_1_limb_11.clone(),
                unpacked_limb_12_col8.clone(),
                unpacked_limb_13_col9.clone(),
                unpacked_tmp_4f7f8_1_limb_14.clone(),
                unpacked_limb_15_col10.clone(),
                unpacked_limb_16_col11.clone(),
                unpacked_tmp_4f7f8_1_limb_17.clone(),
                unpacked_limb_18_col12.clone(),
                unpacked_limb_19_col13.clone(),
                unpacked_tmp_4f7f8_1_limb_20.clone(),
                unpacked_limb_21_col14.clone(),
                unpacked_limb_22_col15.clone(),
                unpacked_tmp_4f7f8_1_limb_23.clone(),
                unpacked_limb_24_col16.clone(),
                unpacked_limb_25_col17.clone(),
                unpacked_tmp_4f7f8_1_limb_26.clone(),
                unpacked_tmp_4f7f8_1_limb_27.clone(),
            ],
            range_check_9_9_lookup_elements,
            range_check_9_9_b_lookup_elements,
            range_check_9_9_c_lookup_elements,
            range_check_9_9_d_lookup_elements,
            range_check_9_9_e_lookup_elements,
            range_check_9_9_f_lookup_elements,
            range_check_9_9_g_lookup_elements,
            range_check_9_9_h_lookup_elements,
            eval,
        );
        [
            unpacked_tmp_4f7f8_1_limb_2.clone(),
            unpacked_tmp_4f7f8_1_limb_5.clone(),
            unpacked_tmp_4f7f8_1_limb_8.clone(),
            unpacked_tmp_4f7f8_1_limb_11.clone(),
            unpacked_tmp_4f7f8_1_limb_14.clone(),
            unpacked_tmp_4f7f8_1_limb_17.clone(),
            unpacked_tmp_4f7f8_1_limb_20.clone(),
            unpacked_tmp_4f7f8_1_limb_23.clone(),
            unpacked_tmp_4f7f8_1_limb_26.clone(),
            unpacked_tmp_4f7f8_1_limb_27.clone(),
        ]
    }
}
