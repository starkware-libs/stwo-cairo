// This file was created by the AIR team.

use crate::components::prelude::*;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct RangeCheckMemValueN28 {}

impl RangeCheckMemValueN28 {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [range_check_mem_value_n_28_input_limb_0, range_check_mem_value_n_28_input_limb_1, range_check_mem_value_n_28_input_limb_2, range_check_mem_value_n_28_input_limb_3, range_check_mem_value_n_28_input_limb_4, range_check_mem_value_n_28_input_limb_5, range_check_mem_value_n_28_input_limb_6, range_check_mem_value_n_28_input_limb_7, range_check_mem_value_n_28_input_limb_8, range_check_mem_value_n_28_input_limb_9, range_check_mem_value_n_28_input_limb_10, range_check_mem_value_n_28_input_limb_11, range_check_mem_value_n_28_input_limb_12, range_check_mem_value_n_28_input_limb_13, range_check_mem_value_n_28_input_limb_14, range_check_mem_value_n_28_input_limb_15, range_check_mem_value_n_28_input_limb_16, range_check_mem_value_n_28_input_limb_17, range_check_mem_value_n_28_input_limb_18, range_check_mem_value_n_28_input_limb_19, range_check_mem_value_n_28_input_limb_20, range_check_mem_value_n_28_input_limb_21, range_check_mem_value_n_28_input_limb_22, range_check_mem_value_n_28_input_limb_23, range_check_mem_value_n_28_input_limb_24, range_check_mem_value_n_28_input_limb_25, range_check_mem_value_n_28_input_limb_26, range_check_mem_value_n_28_input_limb_27]: [E::F; 28],
        range_check_9_9_lookup_elements: &relations::RangeCheck_9_9,
        range_check_9_9_b_lookup_elements: &relations::RangeCheck_9_9_B,
        range_check_9_9_c_lookup_elements: &relations::RangeCheck_9_9_C,
        range_check_9_9_d_lookup_elements: &relations::RangeCheck_9_9_D,
        range_check_9_9_e_lookup_elements: &relations::RangeCheck_9_9_E,
        range_check_9_9_f_lookup_elements: &relations::RangeCheck_9_9_F,
        range_check_9_9_g_lookup_elements: &relations::RangeCheck_9_9_G,
        range_check_9_9_h_lookup_elements: &relations::RangeCheck_9_9_H,
        eval: &mut E,
    ) -> [E::F; 0] {
        eval.add_to_relation(RelationEntry::new(
            range_check_9_9_lookup_elements,
            E::EF::one(),
            &[
                range_check_mem_value_n_28_input_limb_0.clone(),
                range_check_mem_value_n_28_input_limb_1.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            range_check_9_9_b_lookup_elements,
            E::EF::one(),
            &[
                range_check_mem_value_n_28_input_limb_2.clone(),
                range_check_mem_value_n_28_input_limb_3.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            range_check_9_9_c_lookup_elements,
            E::EF::one(),
            &[
                range_check_mem_value_n_28_input_limb_4.clone(),
                range_check_mem_value_n_28_input_limb_5.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            range_check_9_9_d_lookup_elements,
            E::EF::one(),
            &[
                range_check_mem_value_n_28_input_limb_6.clone(),
                range_check_mem_value_n_28_input_limb_7.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            range_check_9_9_e_lookup_elements,
            E::EF::one(),
            &[
                range_check_mem_value_n_28_input_limb_8.clone(),
                range_check_mem_value_n_28_input_limb_9.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            range_check_9_9_f_lookup_elements,
            E::EF::one(),
            &[
                range_check_mem_value_n_28_input_limb_10.clone(),
                range_check_mem_value_n_28_input_limb_11.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            range_check_9_9_g_lookup_elements,
            E::EF::one(),
            &[
                range_check_mem_value_n_28_input_limb_12.clone(),
                range_check_mem_value_n_28_input_limb_13.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            range_check_9_9_h_lookup_elements,
            E::EF::one(),
            &[
                range_check_mem_value_n_28_input_limb_14.clone(),
                range_check_mem_value_n_28_input_limb_15.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            range_check_9_9_lookup_elements,
            E::EF::one(),
            &[
                range_check_mem_value_n_28_input_limb_16.clone(),
                range_check_mem_value_n_28_input_limb_17.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            range_check_9_9_b_lookup_elements,
            E::EF::one(),
            &[
                range_check_mem_value_n_28_input_limb_18.clone(),
                range_check_mem_value_n_28_input_limb_19.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            range_check_9_9_c_lookup_elements,
            E::EF::one(),
            &[
                range_check_mem_value_n_28_input_limb_20.clone(),
                range_check_mem_value_n_28_input_limb_21.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            range_check_9_9_d_lookup_elements,
            E::EF::one(),
            &[
                range_check_mem_value_n_28_input_limb_22.clone(),
                range_check_mem_value_n_28_input_limb_23.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            range_check_9_9_e_lookup_elements,
            E::EF::one(),
            &[
                range_check_mem_value_n_28_input_limb_24.clone(),
                range_check_mem_value_n_28_input_limb_25.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            range_check_9_9_f_lookup_elements,
            E::EF::one(),
            &[
                range_check_mem_value_n_28_input_limb_26.clone(),
                range_check_mem_value_n_28_input_limb_27.clone(),
            ],
        ));

        []
    }
}
