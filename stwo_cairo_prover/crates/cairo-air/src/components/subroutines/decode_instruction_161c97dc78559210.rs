use crate::components::prelude::*;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct DecodeInstruction161C97Dc78559210 {}

impl DecodeInstruction161C97Dc78559210 {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        decode_instruction_161c97dc78559210_input: E::F,
        offset0_col0: E::F,
        dst_base_fp_col1: E::F,
        ap_update_add_1_col2: E::F,
        eval: &mut E,
        verify_instruction_lookup_elements: &relations::VerifyInstruction,
    ) -> [E::F; 19] {
        let M31_0 = E::F::from(M31::from(0));
        let M31_1 = E::F::from(M31::from(1));
        let M31_16 = E::F::from(M31::from(16));
        let M31_2147483646 = E::F::from(M31::from(2147483646));
        let M31_256 = E::F::from(M31::from(256));
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
                decode_instruction_161c97dc78559210_input.clone(),
                offset0_col0.clone(),
                M31_32767.clone(),
                M31_32769.clone(),
                (((dst_base_fp_col1.clone() * M31_8.clone()) + M31_16.clone()) + M31_32.clone()),
                ((ap_update_add_1_col2.clone() * M31_32.clone()) + M31_256.clone()),
            ],
        ));

        [
            (offset0_col0.clone() - M31_32768.clone()),
            M31_2147483646.clone(),
            M31_1.clone(),
            dst_base_fp_col1.clone(),
            M31_1.clone(),
            M31_1.clone(),
            M31_0.clone(),
            M31_0.clone(),
            M31_0.clone(),
            M31_0.clone(),
            M31_0.clone(),
            M31_0.clone(),
            M31_0.clone(),
            M31_0.clone(),
            ap_update_add_1_col2.clone(),
            M31_0.clone(),
            M31_0.clone(),
            M31_1.clone(),
            M31_0.clone(),
        ]
    }
}
