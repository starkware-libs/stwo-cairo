// This file was created by the AIR team.

use crate::components::prelude::*;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct Felt252UnpackFrom27 {}

impl Felt252UnpackFrom27 {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [felt_252_unpack_from_27_input_limb_0, felt_252_unpack_from_27_input_limb_1, felt_252_unpack_from_27_input_limb_2, felt_252_unpack_from_27_input_limb_3, felt_252_unpack_from_27_input_limb_4, felt_252_unpack_from_27_input_limb_5, felt_252_unpack_from_27_input_limb_6, felt_252_unpack_from_27_input_limb_7, felt_252_unpack_from_27_input_limb_8, felt_252_unpack_from_27_input_limb_9]: [E::F; 10],
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
        eval: &mut E,
    ) -> [E::F; 10] {
        let M31_512 = E::F::from(M31::from(512));
        let M31_8192 = E::F::from(M31::from(8192));

        [
            (((felt_252_unpack_from_27_input_limb_0.clone() - unpacked_limb_0_col0.clone())
                - (unpacked_limb_1_col1.clone() * M31_512.clone()))
                * M31_8192.clone()),
            (((felt_252_unpack_from_27_input_limb_1.clone() - unpacked_limb_3_col2.clone())
                - (unpacked_limb_4_col3.clone() * M31_512.clone()))
                * M31_8192.clone()),
            (((felt_252_unpack_from_27_input_limb_2.clone() - unpacked_limb_6_col4.clone())
                - (unpacked_limb_7_col5.clone() * M31_512.clone()))
                * M31_8192.clone()),
            (((felt_252_unpack_from_27_input_limb_3.clone() - unpacked_limb_9_col6.clone())
                - (unpacked_limb_10_col7.clone() * M31_512.clone()))
                * M31_8192.clone()),
            (((felt_252_unpack_from_27_input_limb_4.clone() - unpacked_limb_12_col8.clone())
                - (unpacked_limb_13_col9.clone() * M31_512.clone()))
                * M31_8192.clone()),
            (((felt_252_unpack_from_27_input_limb_5.clone() - unpacked_limb_15_col10.clone())
                - (unpacked_limb_16_col11.clone() * M31_512.clone()))
                * M31_8192.clone()),
            (((felt_252_unpack_from_27_input_limb_6.clone() - unpacked_limb_18_col12.clone())
                - (unpacked_limb_19_col13.clone() * M31_512.clone()))
                * M31_8192.clone()),
            (((felt_252_unpack_from_27_input_limb_7.clone() - unpacked_limb_21_col14.clone())
                - (unpacked_limb_22_col15.clone() * M31_512.clone()))
                * M31_8192.clone()),
            (((felt_252_unpack_from_27_input_limb_8.clone() - unpacked_limb_24_col16.clone())
                - (unpacked_limb_25_col17.clone() * M31_512.clone()))
                * M31_8192.clone()),
            felt_252_unpack_from_27_input_limb_9.clone(),
        ]
    }
}
