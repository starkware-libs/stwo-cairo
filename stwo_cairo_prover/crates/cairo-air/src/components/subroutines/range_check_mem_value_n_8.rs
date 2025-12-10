// This file was created by the AIR team.

use crate::components::prelude::*;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct RangeCheckMemValueN8 {}

impl RangeCheckMemValueN8 {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [range_check_mem_value_n_8_input_limb_0, range_check_mem_value_n_8_input_limb_1, range_check_mem_value_n_8_input_limb_2, range_check_mem_value_n_8_input_limb_3, range_check_mem_value_n_8_input_limb_4, range_check_mem_value_n_8_input_limb_5, range_check_mem_value_n_8_input_limb_6, range_check_mem_value_n_8_input_limb_7]: [E::F; 8],
        range_check_9_9_lookup_elements: &relations::RangeCheck_9_9,
        range_check_9_9_b_lookup_elements: &relations::RangeCheck_9_9_B,
        range_check_9_9_c_lookup_elements: &relations::RangeCheck_9_9_C,
        range_check_9_9_d_lookup_elements: &relations::RangeCheck_9_9_D,
        eval: &mut E,
    ) -> [E::F; 0] {
        eval.add_to_relation(RelationEntry::new(
            range_check_9_9_lookup_elements,
            E::EF::one(),
            &[
                range_check_mem_value_n_8_input_limb_0.clone(),
                range_check_mem_value_n_8_input_limb_1.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            range_check_9_9_b_lookup_elements,
            E::EF::one(),
            &[
                range_check_mem_value_n_8_input_limb_2.clone(),
                range_check_mem_value_n_8_input_limb_3.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            range_check_9_9_c_lookup_elements,
            E::EF::one(),
            &[
                range_check_mem_value_n_8_input_limb_4.clone(),
                range_check_mem_value_n_8_input_limb_5.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            range_check_9_9_d_lookup_elements,
            E::EF::one(),
            &[
                range_check_mem_value_n_8_input_limb_6.clone(),
                range_check_mem_value_n_8_input_limb_7.clone(),
            ],
        ));

        []
    }
}
