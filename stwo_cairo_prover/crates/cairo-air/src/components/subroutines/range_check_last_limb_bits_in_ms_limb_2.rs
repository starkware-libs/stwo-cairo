// This file was created by the AIR team.

use crate::components::prelude::*;
use crate::components::subroutines::cond_range_check_2::CondRangeCheck2;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct RangeCheckLastLimbBitsInMsLimb2 {}

impl RangeCheckLastLimbBitsInMsLimb2 {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [range_check_last_limb_bits_in_ms_limb_2_input]: [E::F; 1],
        partial_limb_msb_col0: E::F,
        eval: &mut E,
    ) -> [E::F; 0] {
        let M31_1 = E::F::from(M31::from(1));

        CondRangeCheck2::evaluate(
            [
                range_check_last_limb_bits_in_ms_limb_2_input.clone(),
                M31_1.clone(),
            ],
            partial_limb_msb_col0.clone(),
            eval,
        );
        []
    }
}
