// This file was created by the AIR team.

use crate::components::prelude::*;
use crate::components::subroutines::read_id::ReadId;
use crate::components::subroutines::read_positive_known_id_num_bits_252::ReadPositiveKnownIdNumBits252;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct ReadPositiveNumBits252 {}

impl ReadPositiveNumBits252 {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [read_positive_num_bits_252_input]: [E::F; 1],
        id_col0: E::F,
        value_limb_0_col1: E::F,
        value_limb_1_col2: E::F,
        value_limb_2_col3: E::F,
        value_limb_3_col4: E::F,
        value_limb_4_col5: E::F,
        value_limb_5_col6: E::F,
        value_limb_6_col7: E::F,
        value_limb_7_col8: E::F,
        value_limb_8_col9: E::F,
        value_limb_9_col10: E::F,
        value_limb_10_col11: E::F,
        value_limb_11_col12: E::F,
        value_limb_12_col13: E::F,
        value_limb_13_col14: E::F,
        value_limb_14_col15: E::F,
        value_limb_15_col16: E::F,
        value_limb_16_col17: E::F,
        value_limb_17_col18: E::F,
        value_limb_18_col19: E::F,
        value_limb_19_col20: E::F,
        value_limb_20_col21: E::F,
        value_limb_21_col22: E::F,
        value_limb_22_col23: E::F,
        value_limb_23_col24: E::F,
        value_limb_24_col25: E::F,
        value_limb_25_col26: E::F,
        value_limb_26_col27: E::F,
        value_limb_27_col28: E::F,
        memory_address_to_id_lookup_elements: &relations::MemoryAddressToId,
        memory_id_to_big_lookup_elements: &relations::MemoryIdToBig,
        eval: &mut E,
    ) -> [E::F; 0] {
        ReadId::evaluate(
            [read_positive_num_bits_252_input.clone()],
            id_col0.clone(),
            memory_address_to_id_lookup_elements,
            eval,
        );
        ReadPositiveKnownIdNumBits252::evaluate(
            [id_col0.clone()],
            value_limb_0_col1.clone(),
            value_limb_1_col2.clone(),
            value_limb_2_col3.clone(),
            value_limb_3_col4.clone(),
            value_limb_4_col5.clone(),
            value_limb_5_col6.clone(),
            value_limb_6_col7.clone(),
            value_limb_7_col8.clone(),
            value_limb_8_col9.clone(),
            value_limb_9_col10.clone(),
            value_limb_10_col11.clone(),
            value_limb_11_col12.clone(),
            value_limb_12_col13.clone(),
            value_limb_13_col14.clone(),
            value_limb_14_col15.clone(),
            value_limb_15_col16.clone(),
            value_limb_16_col17.clone(),
            value_limb_17_col18.clone(),
            value_limb_18_col19.clone(),
            value_limb_19_col20.clone(),
            value_limb_20_col21.clone(),
            value_limb_21_col22.clone(),
            value_limb_22_col23.clone(),
            value_limb_23_col24.clone(),
            value_limb_24_col25.clone(),
            value_limb_25_col26.clone(),
            value_limb_26_col27.clone(),
            value_limb_27_col28.clone(),
            memory_id_to_big_lookup_elements,
            eval,
        );
        []
    }
}
