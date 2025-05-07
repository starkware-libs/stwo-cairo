use crate::components::prelude::*;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct DecodeInstruction43E1C {}

impl DecodeInstruction43E1C {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [decode_instruction_43e1c_input_pc]: [E::F; 1],
        offset2_col0: E::F,
        op1_base_fp_col1: E::F,
        op1_base_ap_col2: E::F,
        ap_update_add_1_col3: E::F,
        verify_instruction_lookup_elements: &relations::VerifyInstruction,
        eval: &mut E,
    ) -> [E::F; 1] {
        let M31_1 = E::F::from(M31::from(1));
        let M31_128 = E::F::from(M31::from(128));
        let M31_2 = E::F::from(M31::from(2));
        let M31_24 = E::F::from(M31::from(24));
        let M31_32 = E::F::from(M31::from(32));
        let M31_32767 = E::F::from(M31::from(32767));
        let M31_32768 = E::F::from(M31::from(32768));
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
                decode_instruction_43e1c_input_pc.clone(),
                M31_32767.clone(),
                M31_32767.clone(),
                offset2_col0.clone(),
                ((M31_24.clone() + (op1_base_fp_col1.clone() * M31_64.clone()))
                    + (op1_base_ap_col2.clone() * M31_128.clone())),
                (M31_2.clone() + (ap_update_add_1_col3.clone() * M31_32.clone())),
            ],
        ));

        [(offset2_col0.clone() - M31_32768.clone())]
    }
}
