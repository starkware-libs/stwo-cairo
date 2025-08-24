// AIR version 97774321-dirty
use crate::components::prelude::*;
use crate::components::subroutines::read_id::ReadId;
use crate::components::subroutines::read_positive_known_id_num_bits_27::ReadPositiveknownIdNumBits27;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct ReadPositiveNumBits27 {}

impl ReadPositiveNumBits27 {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [read_positive_num_bits_27_input]: [E::F; 1],
        id_col0: E::F,
        value_limb_0_col1: E::F,
        value_limb_1_col2: E::F,
        value_limb_2_col3: E::F,
        memory_address_to_id_lookup_elements: &relations::MemoryAddressToId,
        memory_id_to_big_lookup_elements: &relations::MemoryIdToBig,
        eval: &mut E,
    ) -> [E::F; 0] {
        ReadId::evaluate(
            [read_positive_num_bits_27_input.clone()],
            id_col0.clone(),
            memory_address_to_id_lookup_elements,
            eval,
        );
        ReadPositiveknownIdNumBits27::evaluate(
            [id_col0.clone()],
            value_limb_0_col1.clone(),
            value_limb_1_col2.clone(),
            value_limb_2_col3.clone(),
            memory_id_to_big_lookup_elements,
            eval,
        );
        []
    }
}
