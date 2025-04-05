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
        decode_instruction_15a61c6002c544ec_input: E::F,
        eval: &mut E,
        verify_instruction_lookup_elements: &relations::VerifyInstruction,
    ) -> [E::F; 19] {
        let M31_0 = E::F::from(M31::from(0));
        let M31_1 = E::F::from(M31::from(1));
        let M31_130 = E::F::from(M31::from(130));
        let M31_2147483645 = E::F::from(M31::from(2147483645));
        let M31_2147483646 = E::F::from(M31::from(2147483646));
        let M31_32766 = E::F::from(M31::from(32766));
        let M31_32767 = E::F::from(M31::from(32767));
        let M31_88 = E::F::from(M31::from(88));

        eval.add_to_relation(RelationEntry::new(
            verify_instruction_lookup_elements,
            E::EF::one(),
            &[
                decode_instruction_15a61c6002c544ec_input.clone(),
                M31_32766.clone(),
                M31_32767.clone(),
                M31_32767.clone(),
                M31_88.clone(),
                M31_130.clone(),
            ],
        ));

        [
            M31_2147483645.clone(),
            M31_2147483646.clone(),
            M31_2147483646.clone(),
            M31_1.clone(),
            M31_1.clone(),
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
            M31_0.clone(),
            M31_1.clone(),
            M31_0.clone(),
            M31_0.clone(),
        ]
    }
}
