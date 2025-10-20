// This file was created by the AIR team.

use crate::components::prelude::*;
use crate::components::subroutines::read_id::ReadId;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct MemVerify {}

impl MemVerify {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [mem_verify_input_address, mem_verify_input_value_limb_0, mem_verify_input_value_limb_1, mem_verify_input_value_limb_2, mem_verify_input_value_limb_3, mem_verify_input_value_limb_4, mem_verify_input_value_limb_5, mem_verify_input_value_limb_6, mem_verify_input_value_limb_7, mem_verify_input_value_limb_8, mem_verify_input_value_limb_9, mem_verify_input_value_limb_10, mem_verify_input_value_limb_11, mem_verify_input_value_limb_12, mem_verify_input_value_limb_13, mem_verify_input_value_limb_14, mem_verify_input_value_limb_15, mem_verify_input_value_limb_16, mem_verify_input_value_limb_17, mem_verify_input_value_limb_18, mem_verify_input_value_limb_19, mem_verify_input_value_limb_20, mem_verify_input_value_limb_21, mem_verify_input_value_limb_22, mem_verify_input_value_limb_23, mem_verify_input_value_limb_24, mem_verify_input_value_limb_25, mem_verify_input_value_limb_26, mem_verify_input_value_limb_27]: [E::F; 29],
        id_col0: E::F,
        memory_address_to_id_lookup_elements: &relations::MemoryAddressToId,
        memory_id_to_big_lookup_elements: &relations::MemoryIdToBig,
        eval: &mut E,
    ) -> [E::F; 0] {
        ReadId::evaluate(
            [mem_verify_input_address.clone()],
            id_col0.clone(),
            memory_address_to_id_lookup_elements,
            eval,
        );
        eval.add_to_relation(RelationEntry::new(
            memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                id_col0.clone(),
                mem_verify_input_value_limb_0.clone(),
                mem_verify_input_value_limb_1.clone(),
                mem_verify_input_value_limb_2.clone(),
                mem_verify_input_value_limb_3.clone(),
                mem_verify_input_value_limb_4.clone(),
                mem_verify_input_value_limb_5.clone(),
                mem_verify_input_value_limb_6.clone(),
                mem_verify_input_value_limb_7.clone(),
                mem_verify_input_value_limb_8.clone(),
                mem_verify_input_value_limb_9.clone(),
                mem_verify_input_value_limb_10.clone(),
                mem_verify_input_value_limb_11.clone(),
                mem_verify_input_value_limb_12.clone(),
                mem_verify_input_value_limb_13.clone(),
                mem_verify_input_value_limb_14.clone(),
                mem_verify_input_value_limb_15.clone(),
                mem_verify_input_value_limb_16.clone(),
                mem_verify_input_value_limb_17.clone(),
                mem_verify_input_value_limb_18.clone(),
                mem_verify_input_value_limb_19.clone(),
                mem_verify_input_value_limb_20.clone(),
                mem_verify_input_value_limb_21.clone(),
                mem_verify_input_value_limb_22.clone(),
                mem_verify_input_value_limb_23.clone(),
                mem_verify_input_value_limb_24.clone(),
                mem_verify_input_value_limb_25.clone(),
                mem_verify_input_value_limb_26.clone(),
                mem_verify_input_value_limb_27.clone(),
            ],
        ));

        []
    }
}
