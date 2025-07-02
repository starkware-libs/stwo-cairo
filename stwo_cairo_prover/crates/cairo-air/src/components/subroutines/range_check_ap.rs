// AIR version d3fb930e
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
        range_check_ap_bot8bits_col0: E::F,
        range_check_19_lookup_elements: &relations::RangeCheck_19,
        range_check_8_lookup_elements: &relations::RangeCheck_8,
        eval: &mut E,
    ) -> [E::F; 0] {
        let M31_8388608 = E::F::from(M31::from(8388608));

        eval.add_to_relation(RelationEntry::new(
            range_check_19_lookup_elements,
            E::EF::one(),
            &[
                ((range_check_ap_input.clone() - range_check_ap_bot8bits_col0.clone())
                    * M31_8388608.clone()),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            range_check_8_lookup_elements,
            E::EF::one(),
            &[range_check_ap_bot8bits_col0.clone()],
        ));

        []
    }
}
