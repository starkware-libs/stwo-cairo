use crate::components::prelude::*;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct DecodeInstruction2A7A2 {}

impl DecodeInstruction2A7A2 {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        decode_instruction_2a7a2_input: E::F,
        verify_instruction_lookup_elements: &relations::VerifyInstruction,
        eval: &mut E,
    ) -> [E::F; 19] {
        let M31_0 = E::F::from(M31::from(0));
        let M31_1 = E::F::from(M31::from(1));
        let M31_32 = E::F::from(M31::from(32));
        let M31_32768 = E::F::from(M31::from(32768));
        let M31_32769 = E::F::from(M31::from(32769));
        let M31_68 = E::F::from(M31::from(68));

        eval.add_to_relation(RelationEntry::new(
            verify_instruction_lookup_elements,
            E::EF::one(),
            &[
                decode_instruction_2a7a2_input.clone(),
                M31_32768.clone(),
                M31_32769.clone(),
                M31_32769.clone(),
                M31_32.clone(),
                M31_68.clone(),
            ],
        ));

        [
            M31_0.clone(),
            M31_1.clone(),
            M31_1.clone(),
            M31_0.clone(),
            M31_0.clone(),
            M31_1.clone(),
            M31_0.clone(),
            M31_0.clone(),
            M31_0.clone(),
            M31_0.clone(),
            M31_0.clone(),
            M31_1.clone(),
            M31_0.clone(),
            M31_0.clone(),
            M31_0.clone(),
            M31_1.clone(),
            M31_0.clone(),
            M31_0.clone(),
            M31_0.clone(),
        ]
    }
}
