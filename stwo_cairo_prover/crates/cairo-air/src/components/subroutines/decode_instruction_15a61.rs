// This file was created by the AIR team.

use crate::components::prelude::*;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct DecodeInstruction15A61 {}

impl DecodeInstruction15A61 {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [decode_instruction_15a61_input_pc]: [E::F; 1],
        verify_instruction_lookup_elements: &relations::VerifyInstruction,
        eval: &mut E,
    ) -> [E::F; 0] {
        let M31_130 = E::F::from(M31::from(130));
        let M31_32766 = E::F::from(M31::from(32766));
        let M31_32767 = E::F::from(M31::from(32767));
        let M31_88 = E::F::from(M31::from(88));

        eval.add_to_relation(RelationEntry::new(
            verify_instruction_lookup_elements,
            E::EF::one(),
            &[
                decode_instruction_15a61_input_pc.clone(),
                M31_32766.clone(),
                M31_32767.clone(),
                M31_32767.clone(),
                M31_88.clone(),
                M31_130.clone(),
            ],
        ));

        []
    }
}
