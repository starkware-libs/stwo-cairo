// This file was created by the AIR team.

use crate::components::prelude::*;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct DecodeInstructionDe75A {}

impl DecodeInstructionDe75A {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [decode_instruction_de75a_input_pc]: [E::F; 1],
        offset0_col0: E::F,
        dst_base_fp_col1: E::F,
        ap_update_add_1_col2: E::F,
        verify_instruction_lookup_elements: &relations::VerifyInstruction,
        eval: &mut E,
    ) -> [E::F; 1] {
        let M31_1 = E::F::from(M31::from(1));
        let M31_16 = E::F::from(M31::from(16));
        let M31_32 = E::F::from(M31::from(32));
        let M31_32767 = E::F::from(M31::from(32767));
        let M31_32768 = E::F::from(M31::from(32768));
        let M31_32769 = E::F::from(M31::from(32769));
        let M31_8 = E::F::from(M31::from(8));

        // Flag dst_base_fp is a bit.
        eval.add_constraint(
            (dst_base_fp_col1.clone() * (M31_1.clone() - dst_base_fp_col1.clone())),
        );
        // Flag ap_update_add_1 is a bit.
        eval.add_constraint(
            (ap_update_add_1_col2.clone() * (M31_1.clone() - ap_update_add_1_col2.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            verify_instruction_lookup_elements,
            E::EF::one(),
            &[
                decode_instruction_de75a_input_pc.clone(),
                offset0_col0.clone(),
                M31_32767.clone(),
                M31_32769.clone(),
                (((dst_base_fp_col1.clone() * M31_8.clone()) + M31_16.clone()) + M31_32.clone()),
                (M31_8.clone() + (ap_update_add_1_col2.clone() * M31_32.clone())),
            ],
        ));

        [(offset0_col0.clone() - M31_32768.clone())]
    }
}
