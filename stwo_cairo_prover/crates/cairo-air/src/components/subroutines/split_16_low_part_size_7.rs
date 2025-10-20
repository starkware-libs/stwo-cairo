// This file was created by the AIR team.

use crate::components::prelude::*;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct Split16LowPartSize7 {}

impl Split16LowPartSize7 {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [split_16_low_part_size_7_input]: [E::F; 1],
        ms_9_bits_col0: E::F,
        eval: &mut E,
    ) -> [E::F; 1] {
        let M31_128 = E::F::from(M31::from(128));

        [(split_16_low_part_size_7_input.clone() - (ms_9_bits_col0.clone() * M31_128.clone()))]
    }
}
