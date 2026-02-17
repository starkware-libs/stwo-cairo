// This file was created by the AIR team.

use crate::components::prelude::*;
use crate::components::subroutines::range_check_last_limb_bits_in_ms_limb_2::RangeCheckLastLimbBitsInMsLimb2;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct ReadPositiveKnownIdNumBits29 {}

impl ReadPositiveKnownIdNumBits29 {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [read_positive_known_id_num_bits_29_input]: [E::F; 1],
        value_limb_0_col0: E::F,
        value_limb_1_col1: E::F,
        value_limb_2_col2: E::F,
        value_limb_3_col3: E::F,
        partial_limb_msb_col4: E::F,
        common_lookup_elements: &relations::CommonLookupElements,
        eval: &mut E,
    ) -> [E::F; 0] {
        let M31_1 = E::F::from(M31::from(1));
        let M31_1662111297 = E::F::from(M31::from(1662111297));

        RangeCheckLastLimbBitsInMsLimb2::evaluate(
            [value_limb_3_col3.clone()],
            partial_limb_msb_col4.clone(),
            common_lookup_elements,
            eval,
        );
        eval.add_to_relation(RelationEntry::new(
            common_lookup_elements,
            E::EF::from(M31_1.clone()),
            &[
                M31_1662111297.clone(),
                read_positive_known_id_num_bits_29_input.clone(),
                value_limb_0_col0.clone(),
                value_limb_1_col1.clone(),
                value_limb_2_col2.clone(),
                value_limb_3_col3.clone(),
            ],
        ));

        []
    }
}
