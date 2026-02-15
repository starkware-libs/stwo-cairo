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
        common_lookup_elements: &relations::CommonLookupElements,
        eval: &mut E,
    ) -> [E::F; 0] {
        let M31_1 = E::F::from(M31::from(1));
        let M31_1864236857 = E::F::from(M31::from(1864236857));
        let M31_1881014476 = E::F::from(M31::from(1881014476));
        let M31_1897792095 = E::F::from(M31::from(1897792095));
        let M31_517791011 = E::F::from(M31::from(517791011));

        eval.add_to_relation(RelationEntry::new(
            common_lookup_elements,
            E::EF::from(M31_1.clone()),
            &[
                M31_517791011.clone(),
                range_check_mem_value_n_8_input_limb_0.clone(),
                range_check_mem_value_n_8_input_limb_1.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            common_lookup_elements,
            E::EF::from(M31_1.clone()),
            &[
                M31_1897792095.clone(),
                range_check_mem_value_n_8_input_limb_2.clone(),
                range_check_mem_value_n_8_input_limb_3.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            common_lookup_elements,
            E::EF::from(M31_1.clone()),
            &[
                M31_1881014476.clone(),
                range_check_mem_value_n_8_input_limb_4.clone(),
                range_check_mem_value_n_8_input_limb_5.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            common_lookup_elements,
            E::EF::from(M31_1.clone()),
            &[
                M31_1864236857.clone(),
                range_check_mem_value_n_8_input_limb_6.clone(),
                range_check_mem_value_n_8_input_limb_7.clone(),
            ],
        ));

        []
    }
}
