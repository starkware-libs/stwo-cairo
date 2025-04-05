use crate::components::prelude::*;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct DecodeInstruction3B105 {}

impl DecodeInstruction3B105 {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        decode_instruction_3b1056363058c126_input: E::F,
        offset2_col0: E::F,
        op1_base_fp_col1: E::F,
        op1_base_ap_col2: E::F,
        ap_update_add_1_col3: E::F,
        eval: &mut E,
        verify_instruction_lookup_elements: &relations::VerifyInstruction,
    ) -> [E::F; 19] {
        let M31_0 = E::F::from(M31::from(0));
        let M31_1 = E::F::from(M31::from(1));
        let M31_128 = E::F::from(M31::from(128));
        let M31_2147483646 = E::F::from(M31::from(2147483646));
        let M31_24 = E::F::from(M31::from(24));
        let M31_32 = E::F::from(M31::from(32));
        let M31_32767 = E::F::from(M31::from(32767));
        let M31_32768 = E::F::from(M31::from(32768));
        let M31_4 = E::F::from(M31::from(4));
        let M31_64 = E::F::from(M31::from(64));

        // Flag op1_base_fp is a bit.
        eval.add_constraint(
            (op1_base_fp_col1.clone() * (M31_1.clone() - op1_base_fp_col1.clone())),
        );
        // Flag op1_base_ap is a bit.
        eval.add_constraint(
            (op1_base_ap_col2.clone() * (M31_1.clone() - op1_base_ap_col2.clone())),
        );
        // Flag ap_update_add_1 is a bit.
        eval.add_constraint(
            (ap_update_add_1_col3.clone() * (M31_1.clone() - ap_update_add_1_col3.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            verify_instruction_lookup_elements,
            E::EF::one(),
            &[
                decode_instruction_3b1056363058c126_input.clone(),
                M31_32767.clone(),
                M31_32767.clone(),
                offset2_col0.clone(),
                ((M31_24.clone() + (op1_base_fp_col1.clone() * M31_64.clone()))
                    + (op1_base_ap_col2.clone() * M31_128.clone())),
                (M31_4.clone() + (ap_update_add_1_col3.clone() * M31_32.clone())),
            ],
        ));

        [
            M31_2147483646.clone(),
            M31_2147483646.clone(),
            (offset2_col0.clone() - M31_32768.clone()),
            M31_1.clone(),
            M31_1.clone(),
            M31_0.clone(),
            op1_base_fp_col1.clone(),
            op1_base_ap_col2.clone(),
            M31_0.clone(),
            M31_0.clone(),
            M31_0.clone(),
            M31_1.clone(),
            M31_0.clone(),
            M31_0.clone(),
            ap_update_add_1_col3.clone(),
            M31_0.clone(),
            M31_0.clone(),
            M31_0.clone(),
            M31_0.clone(),
        ]
    }
}
