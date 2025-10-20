// This file was created by the AIR team.

use crate::components::prelude::*;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct DecodeInstruction7Ebc4 {}

impl DecodeInstruction7Ebc4 {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [decode_instruction_7ebc4_input_pc]: [E::F; 1],
        ap_update_add_1_col0: E::F,
        verify_instruction_lookup_elements: &relations::VerifyInstruction,
        eval: &mut E,
    ) -> [E::F; 0] {
        let M31_1 = E::F::from(M31::from(1));
        let M31_32 = E::F::from(M31::from(32));
        let M31_32767 = E::F::from(M31::from(32767));
        let M31_32769 = E::F::from(M31::from(32769));
        let M31_4 = E::F::from(M31::from(4));
        let M31_56 = E::F::from(M31::from(56));

        // Flag ap_update_add_1 is a bit.
        eval.add_constraint(
            (ap_update_add_1_col0.clone() * (M31_1.clone() - ap_update_add_1_col0.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            verify_instruction_lookup_elements,
            E::EF::one(),
            &[
                decode_instruction_7ebc4_input_pc.clone(),
                M31_32767.clone(),
                M31_32767.clone(),
                M31_32769.clone(),
                M31_56.clone(),
                (M31_4.clone() + (ap_update_add_1_col0.clone() * M31_32.clone())),
            ],
        ));

        []
    }
}
