// This file was created by the AIR team.

use crate::components::prelude::*;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct Split16LowPartSize12 {}

impl Split16LowPartSize12 {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [split_16_low_part_size_12_input]: [E::F; 1],
        ms_4_bits_col0: E::F,
        eval: &mut E,
    ) -> [E::F; 1] {
        let M31_4096 = E::F::from(M31::from(4096));

        [(split_16_low_part_size_12_input.clone() - (ms_4_bits_col0.clone() * M31_4096.clone()))]
    }
}
