use crate::components::prelude::*;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize, CairoDeserialize, CompactBinary)]
pub struct MemCondVerifyEqualKnownId {}

impl MemCondVerifyEqualKnownId {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [mem_cond_verify_equal_known_id_input_limb_0, mem_cond_verify_equal_known_id_input_limb_1, mem_cond_verify_equal_known_id_input_limb_2]: [E::F; 3],
        id_col0: E::F,
        memory_address_to_id_lookup_elements: &relations::MemoryAddressToId,
        eval: &mut E,
    ) -> [E::F; 0] {
        eval.add_to_relation(RelationEntry::new(
            memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                mem_cond_verify_equal_known_id_input_limb_0.clone(),
                id_col0.clone(),
            ],
        ));

        // The two ids are equal if the condition is met.
        eval.add_constraint(
            ((id_col0.clone() - mem_cond_verify_equal_known_id_input_limb_1.clone())
                * mem_cond_verify_equal_known_id_input_limb_2.clone()),
        );
        []
    }
}
