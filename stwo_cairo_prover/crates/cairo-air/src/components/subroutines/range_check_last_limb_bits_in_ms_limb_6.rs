// This file was created by the AIR team.

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
        [range_check_last_limb_bits_in_ms_limb_6_input]: [E::F; 1],
        common_lookup_elements: &relations::CommonLookupElements,
        eval: &mut E,
    ) -> [E::F; 0] {
        let M31_1 = E::F::from(M31::from(1));
        let M31_1185356339 = E::F::from(M31::from(1185356339));

        eval.add_to_relation(RelationEntry::new(
            common_lookup_elements,
            E::EF::from(M31_1.clone()),
            &[
                M31_1185356339.clone(),
                range_check_last_limb_bits_in_ms_limb_6_input.clone(),
            ],
        ));

        []
    }
}
