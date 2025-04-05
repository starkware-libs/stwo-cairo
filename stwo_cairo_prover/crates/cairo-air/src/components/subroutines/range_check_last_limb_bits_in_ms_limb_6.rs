use crate::components::prelude::*;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct RangeCheckLastLimbBitsInMsLimb6 {}

impl RangeCheckLastLimbBitsInMsLimb6 {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        range_check_last_limb_bits_in_ms_limb_6_input: E::F,
        eval: &mut E,
        range_check_6_lookup_elements: &relations::RangeCheck_6,
    ) -> () {
        eval.add_to_relation(RelationEntry::new(
            range_check_6_lookup_elements,
            E::EF::one(),
            &[range_check_last_limb_bits_in_ms_limb_6_input.clone()],
        ));

        ()
    }
}
