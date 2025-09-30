// AIR version aade2df9
use crate::components::prelude::*;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct ReadPositiveKnownIdNumBits252 {}

impl ReadPositiveKnownIdNumBits252 {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [read_positive_known_id_num_bits_252_input]: [E::F; 1],
        value_limb_0_col0: E::F,
        value_limb_1_col1: E::F,
        value_limb_2_col2: E::F,
        value_limb_3_col3: E::F,
        value_limb_4_col4: E::F,
        value_limb_5_col5: E::F,
        value_limb_6_col6: E::F,
        value_limb_7_col7: E::F,
        value_limb_8_col8: E::F,
        value_limb_9_col9: E::F,
        value_limb_10_col10: E::F,
        value_limb_11_col11: E::F,
        value_limb_12_col12: E::F,
        value_limb_13_col13: E::F,
        value_limb_14_col14: E::F,
        value_limb_15_col15: E::F,
        value_limb_16_col16: E::F,
        value_limb_17_col17: E::F,
        value_limb_18_col18: E::F,
        value_limb_19_col19: E::F,
        value_limb_20_col20: E::F,
        value_limb_21_col21: E::F,
        value_limb_22_col22: E::F,
        value_limb_23_col23: E::F,
        value_limb_24_col24: E::F,
        value_limb_25_col25: E::F,
        value_limb_26_col26: E::F,
        value_limb_27_col27: E::F,
        memory_id_to_big_lookup_elements: &relations::MemoryIdToBig,
        eval: &mut E,
    ) -> [E::F; 0] {
        eval.add_to_relation(RelationEntry::new(
            memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                read_positive_known_id_num_bits_252_input.clone(),
                value_limb_0_col0.clone(),
                value_limb_1_col1.clone(),
                value_limb_2_col2.clone(),
                value_limb_3_col3.clone(),
                value_limb_4_col4.clone(),
                value_limb_5_col5.clone(),
                value_limb_6_col6.clone(),
                value_limb_7_col7.clone(),
                value_limb_8_col8.clone(),
                value_limb_9_col9.clone(),
                value_limb_10_col10.clone(),
                value_limb_11_col11.clone(),
                value_limb_12_col12.clone(),
                value_limb_13_col13.clone(),
                value_limb_14_col14.clone(),
                value_limb_15_col15.clone(),
                value_limb_16_col16.clone(),
                value_limb_17_col17.clone(),
                value_limb_18_col18.clone(),
                value_limb_19_col19.clone(),
                value_limb_20_col20.clone(),
                value_limb_21_col21.clone(),
                value_limb_22_col22.clone(),
                value_limb_23_col23.clone(),
                value_limb_24_col24.clone(),
                value_limb_25_col25.clone(),
                value_limb_26_col26.clone(),
                value_limb_27_col27.clone(),
            ],
        ));

        []
    }
}
