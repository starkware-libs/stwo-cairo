use crate::components::prelude::constraint_eval::*;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct MemCondVerifyEqualKnownId {}

impl MemCondVerifyEqualKnownId {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    pub fn evaluate<E: EvalAtRow>(
        [mem_cond_verify_equal_known_id_input_limb_0, mem_cond_verify_equal_known_id_input_limb_1, mem_cond_verify_equal_known_id_input_limb_2]: [E::F; 3],
        id_col0: E::F,
        eval: &mut E,
        memory_address_to_id_lookup_elements: &relations::MemoryAddressToId,
    ) -> () {
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
        ()
    }
}
