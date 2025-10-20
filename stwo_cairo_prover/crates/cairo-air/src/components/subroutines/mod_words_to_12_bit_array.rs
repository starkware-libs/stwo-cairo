// This file was created by the AIR team.

use crate::components::prelude::*;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct ModWordsTo12BitArray {}

impl ModWordsTo12BitArray {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [mod_words_to_12_bit_array_input_limb_0, mod_words_to_12_bit_array_input_limb_1, mod_words_to_12_bit_array_input_limb_2, mod_words_to_12_bit_array_input_limb_3, mod_words_to_12_bit_array_input_limb_4, mod_words_to_12_bit_array_input_limb_5, mod_words_to_12_bit_array_input_limb_6, mod_words_to_12_bit_array_input_limb_7, mod_words_to_12_bit_array_input_limb_8, mod_words_to_12_bit_array_input_limb_9, mod_words_to_12_bit_array_input_limb_10, mod_words_to_12_bit_array_input_limb_28, mod_words_to_12_bit_array_input_limb_29, mod_words_to_12_bit_array_input_limb_30, mod_words_to_12_bit_array_input_limb_31, mod_words_to_12_bit_array_input_limb_32, mod_words_to_12_bit_array_input_limb_33, mod_words_to_12_bit_array_input_limb_34, mod_words_to_12_bit_array_input_limb_35, mod_words_to_12_bit_array_input_limb_36, mod_words_to_12_bit_array_input_limb_37, mod_words_to_12_bit_array_input_limb_38]: [E::F; 22],
        limb1b_0_col0: E::F,
        limb2b_0_col1: E::F,
        limb5b_0_col2: E::F,
        limb6b_0_col3: E::F,
        limb9b_0_col4: E::F,
        limb1b_1_col5: E::F,
        limb2b_1_col6: E::F,
        limb5b_1_col7: E::F,
        limb6b_1_col8: E::F,
        limb9b_1_col9: E::F,
        range_check_3_6_6_3_lookup_elements: &relations::RangeCheck_3_6_6_3,
        eval: &mut E,
    ) -> [E::F; 16] {
        let M31_512 = E::F::from(M31::from(512));
        let M31_64 = E::F::from(M31::from(64));
        let M31_8 = E::F::from(M31::from(8));

        let limb1a_0_tmp_f4497_1 = eval.add_intermediate(
            (mod_words_to_12_bit_array_input_limb_1.clone()
                - (limb1b_0_col0.clone() * M31_8.clone())),
        );
        let limb2a_0_tmp_f4497_3 = eval.add_intermediate(
            (mod_words_to_12_bit_array_input_limb_2.clone()
                - (limb2b_0_col1.clone() * M31_64.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            range_check_3_6_6_3_lookup_elements,
            E::EF::one(),
            &[
                limb1a_0_tmp_f4497_1.clone(),
                limb1b_0_col0.clone(),
                limb2a_0_tmp_f4497_3.clone(),
                limb2b_0_col1.clone(),
            ],
        ));

        let limb5a_0_tmp_f4497_5 = eval.add_intermediate(
            (mod_words_to_12_bit_array_input_limb_5.clone()
                - (limb5b_0_col2.clone() * M31_8.clone())),
        );
        let limb6a_0_tmp_f4497_7 = eval.add_intermediate(
            (mod_words_to_12_bit_array_input_limb_6.clone()
                - (limb6b_0_col3.clone() * M31_64.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            range_check_3_6_6_3_lookup_elements,
            E::EF::one(),
            &[
                limb5a_0_tmp_f4497_5.clone(),
                limb5b_0_col2.clone(),
                limb6a_0_tmp_f4497_7.clone(),
                limb6b_0_col3.clone(),
            ],
        ));

        let limb9a_0_tmp_f4497_9 = eval.add_intermediate(
            (mod_words_to_12_bit_array_input_limb_9.clone()
                - (limb9b_0_col4.clone() * M31_8.clone())),
        );
        let limb1a_1_tmp_f4497_11 = eval.add_intermediate(
            (mod_words_to_12_bit_array_input_limb_29.clone()
                - (limb1b_1_col5.clone() * M31_8.clone())),
        );
        let limb2a_1_tmp_f4497_13 = eval.add_intermediate(
            (mod_words_to_12_bit_array_input_limb_30.clone()
                - (limb2b_1_col6.clone() * M31_64.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            range_check_3_6_6_3_lookup_elements,
            E::EF::one(),
            &[
                limb1a_1_tmp_f4497_11.clone(),
                limb1b_1_col5.clone(),
                limb2a_1_tmp_f4497_13.clone(),
                limb2b_1_col6.clone(),
            ],
        ));

        let limb5a_1_tmp_f4497_15 = eval.add_intermediate(
            (mod_words_to_12_bit_array_input_limb_33.clone()
                - (limb5b_1_col7.clone() * M31_8.clone())),
        );
        let limb6a_1_tmp_f4497_17 = eval.add_intermediate(
            (mod_words_to_12_bit_array_input_limb_34.clone()
                - (limb6b_1_col8.clone() * M31_64.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            range_check_3_6_6_3_lookup_elements,
            E::EF::one(),
            &[
                limb5a_1_tmp_f4497_15.clone(),
                limb5b_1_col7.clone(),
                limb6a_1_tmp_f4497_17.clone(),
                limb6b_1_col8.clone(),
            ],
        ));

        let limb9a_1_tmp_f4497_19 = eval.add_intermediate(
            (mod_words_to_12_bit_array_input_limb_37.clone()
                - (limb9b_1_col9.clone() * M31_8.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            range_check_3_6_6_3_lookup_elements,
            E::EF::one(),
            &[
                limb9a_0_tmp_f4497_9.clone(),
                limb9b_0_col4.clone(),
                limb9b_1_col9.clone(),
                limb9a_1_tmp_f4497_19.clone(),
            ],
        ));

        [
            (mod_words_to_12_bit_array_input_limb_0.clone()
                + (M31_512.clone() * limb1a_0_tmp_f4497_1.clone())),
            (limb1b_0_col0.clone() + (M31_64.clone() * limb2a_0_tmp_f4497_3.clone())),
            (limb2b_0_col1.clone()
                + (M31_8.clone() * mod_words_to_12_bit_array_input_limb_3.clone())),
            (mod_words_to_12_bit_array_input_limb_4.clone()
                + (M31_512.clone() * limb5a_0_tmp_f4497_5.clone())),
            (limb5b_0_col2.clone() + (M31_64.clone() * limb6a_0_tmp_f4497_7.clone())),
            (limb6b_0_col3.clone()
                + (M31_8.clone() * mod_words_to_12_bit_array_input_limb_7.clone())),
            (mod_words_to_12_bit_array_input_limb_8.clone()
                + (M31_512.clone() * limb9a_0_tmp_f4497_9.clone())),
            (limb9b_0_col4.clone()
                + (M31_64.clone() * mod_words_to_12_bit_array_input_limb_10.clone())),
            (mod_words_to_12_bit_array_input_limb_28.clone()
                + (M31_512.clone() * limb1a_1_tmp_f4497_11.clone())),
            (limb1b_1_col5.clone() + (M31_64.clone() * limb2a_1_tmp_f4497_13.clone())),
            (limb2b_1_col6.clone()
                + (M31_8.clone() * mod_words_to_12_bit_array_input_limb_31.clone())),
            (mod_words_to_12_bit_array_input_limb_32.clone()
                + (M31_512.clone() * limb5a_1_tmp_f4497_15.clone())),
            (limb5b_1_col7.clone() + (M31_64.clone() * limb6a_1_tmp_f4497_17.clone())),
            (limb6b_1_col8.clone()
                + (M31_8.clone() * mod_words_to_12_bit_array_input_limb_35.clone())),
            (mod_words_to_12_bit_array_input_limb_36.clone()
                + (M31_512.clone() * limb9a_1_tmp_f4497_19.clone())),
            (limb9b_1_col9.clone()
                + (M31_64.clone() * mod_words_to_12_bit_array_input_limb_38.clone())),
        ]
    }
}
