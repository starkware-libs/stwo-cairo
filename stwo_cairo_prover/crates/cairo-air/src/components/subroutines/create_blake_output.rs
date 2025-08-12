// AIR version e1943601-dirty
use crate::components::prelude::*;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct CreateBlakeOutput {}

impl CreateBlakeOutput {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [create_blake_output_input_limb_0, create_blake_output_input_limb_1, create_blake_output_input_limb_2, create_blake_output_input_limb_3, create_blake_output_input_limb_4, create_blake_output_input_limb_5, create_blake_output_input_limb_6, create_blake_output_input_limb_7, create_blake_output_input_limb_8, create_blake_output_input_limb_9, create_blake_output_input_limb_10, create_blake_output_input_limb_11, create_blake_output_input_limb_12, create_blake_output_input_limb_13, create_blake_output_input_limb_14, create_blake_output_input_limb_15, create_blake_output_input_limb_16, create_blake_output_input_limb_17, create_blake_output_input_limb_18, create_blake_output_input_limb_19, create_blake_output_input_limb_20, create_blake_output_input_limb_21, create_blake_output_input_limb_22, create_blake_output_input_limb_23, create_blake_output_input_limb_24, create_blake_output_input_limb_25, create_blake_output_input_limb_26, create_blake_output_input_limb_27, create_blake_output_input_limb_28, create_blake_output_input_limb_29, create_blake_output_input_limb_30, create_blake_output_input_limb_31, create_blake_output_input_limb_32, create_blake_output_input_limb_33, create_blake_output_input_limb_34, create_blake_output_input_limb_35, create_blake_output_input_limb_36, create_blake_output_input_limb_37, create_blake_output_input_limb_38, create_blake_output_input_limb_39, create_blake_output_input_limb_40, create_blake_output_input_limb_41, create_blake_output_input_limb_42, create_blake_output_input_limb_43, create_blake_output_input_limb_44, create_blake_output_input_limb_45, create_blake_output_input_limb_46, create_blake_output_input_limb_47]: [E::F; 48],
        triple_xor_16_output_col0: E::F,
        triple_xor_16_output_col1: E::F,
        triple_xor_16_output_col2: E::F,
        triple_xor_16_output_col3: E::F,
        triple_xor_16_output_col4: E::F,
        triple_xor_16_output_col5: E::F,
        triple_xor_16_output_col6: E::F,
        triple_xor_16_output_col7: E::F,
        triple_xor_16_output_col8: E::F,
        triple_xor_16_output_col9: E::F,
        triple_xor_16_output_col10: E::F,
        triple_xor_16_output_col11: E::F,
        triple_xor_16_output_col12: E::F,
        triple_xor_16_output_col13: E::F,
        triple_xor_16_output_col14: E::F,
        triple_xor_16_output_col15: E::F,
        triple_xor_16_lookup_elements: &relations::TripleXor16,
        eval: &mut E,
    ) -> [E::F; 0] {
        eval.add_to_relation(RelationEntry::new(
            triple_xor_16_lookup_elements,
            E::EF::one(),
            &[
                create_blake_output_input_limb_16.clone(),
                create_blake_output_input_limb_32.clone(),
                create_blake_output_input_limb_0.clone(),
                triple_xor_16_output_col0.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            triple_xor_16_lookup_elements,
            E::EF::one(),
            &[
                create_blake_output_input_limb_17.clone(),
                create_blake_output_input_limb_33.clone(),
                create_blake_output_input_limb_1.clone(),
                triple_xor_16_output_col1.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            triple_xor_16_lookup_elements,
            E::EF::one(),
            &[
                create_blake_output_input_limb_18.clone(),
                create_blake_output_input_limb_34.clone(),
                create_blake_output_input_limb_2.clone(),
                triple_xor_16_output_col2.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            triple_xor_16_lookup_elements,
            E::EF::one(),
            &[
                create_blake_output_input_limb_19.clone(),
                create_blake_output_input_limb_35.clone(),
                create_blake_output_input_limb_3.clone(),
                triple_xor_16_output_col3.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            triple_xor_16_lookup_elements,
            E::EF::one(),
            &[
                create_blake_output_input_limb_20.clone(),
                create_blake_output_input_limb_36.clone(),
                create_blake_output_input_limb_4.clone(),
                triple_xor_16_output_col4.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            triple_xor_16_lookup_elements,
            E::EF::one(),
            &[
                create_blake_output_input_limb_21.clone(),
                create_blake_output_input_limb_37.clone(),
                create_blake_output_input_limb_5.clone(),
                triple_xor_16_output_col5.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            triple_xor_16_lookup_elements,
            E::EF::one(),
            &[
                create_blake_output_input_limb_22.clone(),
                create_blake_output_input_limb_38.clone(),
                create_blake_output_input_limb_6.clone(),
                triple_xor_16_output_col6.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            triple_xor_16_lookup_elements,
            E::EF::one(),
            &[
                create_blake_output_input_limb_23.clone(),
                create_blake_output_input_limb_39.clone(),
                create_blake_output_input_limb_7.clone(),
                triple_xor_16_output_col7.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            triple_xor_16_lookup_elements,
            E::EF::one(),
            &[
                create_blake_output_input_limb_24.clone(),
                create_blake_output_input_limb_40.clone(),
                create_blake_output_input_limb_8.clone(),
                triple_xor_16_output_col8.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            triple_xor_16_lookup_elements,
            E::EF::one(),
            &[
                create_blake_output_input_limb_25.clone(),
                create_blake_output_input_limb_41.clone(),
                create_blake_output_input_limb_9.clone(),
                triple_xor_16_output_col9.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            triple_xor_16_lookup_elements,
            E::EF::one(),
            &[
                create_blake_output_input_limb_26.clone(),
                create_blake_output_input_limb_42.clone(),
                create_blake_output_input_limb_10.clone(),
                triple_xor_16_output_col10.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            triple_xor_16_lookup_elements,
            E::EF::one(),
            &[
                create_blake_output_input_limb_27.clone(),
                create_blake_output_input_limb_43.clone(),
                create_blake_output_input_limb_11.clone(),
                triple_xor_16_output_col11.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            triple_xor_16_lookup_elements,
            E::EF::one(),
            &[
                create_blake_output_input_limb_28.clone(),
                create_blake_output_input_limb_44.clone(),
                create_blake_output_input_limb_12.clone(),
                triple_xor_16_output_col12.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            triple_xor_16_lookup_elements,
            E::EF::one(),
            &[
                create_blake_output_input_limb_29.clone(),
                create_blake_output_input_limb_45.clone(),
                create_blake_output_input_limb_13.clone(),
                triple_xor_16_output_col13.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            triple_xor_16_lookup_elements,
            E::EF::one(),
            &[
                create_blake_output_input_limb_30.clone(),
                create_blake_output_input_limb_46.clone(),
                create_blake_output_input_limb_14.clone(),
                triple_xor_16_output_col14.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            triple_xor_16_lookup_elements,
            E::EF::one(),
            &[
                create_blake_output_input_limb_31.clone(),
                create_blake_output_input_limb_47.clone(),
                create_blake_output_input_limb_15.clone(),
                triple_xor_16_output_col15.clone(),
            ],
        ));

        []
    }
}
