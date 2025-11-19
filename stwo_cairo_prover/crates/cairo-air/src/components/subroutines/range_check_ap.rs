// This file was created by the AIR team.

use crate::components::prelude::*;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct RangeCheckAp {}

impl RangeCheckAp {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [range_check_ap_input]: [E::F; 1],
        range_check_ap_bot11bits_col0: E::F,
        common_lookup_elements: &relations::CommonLookupElements,
        eval: &mut E,
    ) -> [E::F; 0] {
        let M31_1048576 = E::F::from(M31::from(1048576));
        let M31_1109051422 = E::F::from(M31::from(1109051422));
        let M31_991608089 = E::F::from(M31::from(991608089));

        eval.add_to_relation(RelationEntry::new(
            common_lookup_elements,
            E::EF::one(),
            &[
                M31_1109051422.clone(),
                ((range_check_ap_input.clone() - range_check_ap_bot11bits_col0.clone())
                    * M31_1048576.clone()),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            common_lookup_elements,
            E::EF::one(),
            &[M31_991608089.clone(), range_check_ap_bot11bits_col0.clone()],
        ));

        []
    }
}
