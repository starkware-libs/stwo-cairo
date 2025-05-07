use crate::components::prelude::*;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct DecodeInstructionEa769 {}

impl DecodeInstructionEa769 {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [decode_instruction_ea769_input_pc]: [E::F; 1],
        offset2_col0: E::F,
        verify_instruction_lookup_elements: &relations::VerifyInstruction,
        eval: &mut E,
    ) -> [E::F; 1] {
        let M31_32768 = E::F::from(M31::from(32768));
        let M31_32769 = E::F::from(M31::from(32769));
        let M31_64 = E::F::from(M31::from(64));
        let M31_66 = E::F::from(M31::from(66));

        eval.add_to_relation(RelationEntry::new(
            verify_instruction_lookup_elements,
            E::EF::one(),
            &[
                decode_instruction_ea769_input_pc.clone(),
                M31_32768.clone(),
                M31_32769.clone(),
                offset2_col0.clone(),
                M31_64.clone(),
                M31_66.clone(),
            ],
        ));

        [(offset2_col0.clone() - M31_32768.clone())]
    }
}
