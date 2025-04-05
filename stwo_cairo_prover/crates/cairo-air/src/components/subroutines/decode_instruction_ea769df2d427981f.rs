use crate::components::prelude::*;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct DecodeInstructionEa769Df2D427981F {}

impl DecodeInstructionEa769Df2D427981F {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        decode_instruction_ea769df2d427981f_input: E::F,
        offset2_col0: E::F,
        eval: &mut E,
        verify_instruction_lookup_elements: &relations::VerifyInstruction,
    ) -> [E::F; 19] {
        let M31_0 = E::F::from(M31::from(0));
        let M31_1 = E::F::from(M31::from(1));
        let M31_32768 = E::F::from(M31::from(32768));
        let M31_32769 = E::F::from(M31::from(32769));
        let M31_64 = E::F::from(M31::from(64));
        let M31_66 = E::F::from(M31::from(66));

        eval.add_to_relation(RelationEntry::new(
            verify_instruction_lookup_elements,
            E::EF::one(),
            &[
                decode_instruction_ea769df2d427981f_input.clone(),
                M31_32768.clone(),
                M31_32769.clone(),
                offset2_col0.clone(),
                M31_64.clone(),
                M31_66.clone(),
            ],
        ));

        [
            M31_0.clone(),
            M31_1.clone(),
            (offset2_col0.clone() - M31_32768.clone()),
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
            M31_0.clone(),
            M31_1.clone(),
            M31_0.clone(),
            M31_0.clone(),
            M31_0.clone(),
        ]
    }
}
