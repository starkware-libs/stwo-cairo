// This file was created by the AIR team.

use crate::components::prelude::*;
use crate::components::subroutines::read_id::ReadId;
use crate::components::subroutines::read_positive_known_id_num_bits_96::ReadPositiveKnownIdNumBits96;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct ReadPositiveNumBits96 {}

impl ReadPositiveNumBits96 {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [read_positive_num_bits_96_input]: [E::F; 1],
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
        memory_address_to_id_lookup_elements: &relations::MemoryAddressToId,
        range_check_6_lookup_elements: &relations::RangeCheck_6,
        memory_id_to_big_lookup_elements: &relations::MemoryIdToBig,
        eval: &mut E,
    ) -> [E::F; 0] {
        ReadId::evaluate(
            [read_positive_num_bits_96_input.clone()],
            id_col0.clone(),
            memory_address_to_id_lookup_elements,
            eval,
        );
        ReadPositiveKnownIdNumBits96::evaluate(
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
            range_check_6_lookup_elements,
            memory_id_to_big_lookup_elements,
            eval,
        );
        []
    }
}
