use crate::components::prelude::*;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct MemVerifyEqual {}

impl MemVerifyEqual {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [mem_verify_equal_input_limb_0, mem_verify_equal_input_limb_1]: [E::F; 2],
        id_col0: E::F,
        memory_address_to_id_lookup_elements: &relations::MemoryAddressToId,
        eval: &mut E,
    ) -> () {
        eval.add_to_relation(RelationEntry::new(
            memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[mem_verify_equal_input_limb_0.clone(), id_col0.clone()],
        ));

        eval.add_to_relation(RelationEntry::new(
            memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[mem_verify_equal_input_limb_1.clone(), id_col0.clone()],
        ));

        ()
    }
}
