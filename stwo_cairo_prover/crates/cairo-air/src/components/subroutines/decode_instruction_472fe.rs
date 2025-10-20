// This file was created by the AIR team.

use crate::components::prelude::*;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct DecodeInstruction472Fe {}

impl DecodeInstruction472Fe {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [decode_instruction_472fe_input_pc]: [E::F; 1],
        offset0_col0: E::F,
        offset1_col1: E::F,
        offset2_col2: E::F,
        dst_base_fp_col3: E::F,
        op0_base_fp_col4: E::F,
        op1_base_fp_col5: E::F,
        ap_update_add_1_col6: E::F,
        opcode_extension_col7: E::F,
        verify_instruction_lookup_elements: &relations::VerifyInstruction,
        eval: &mut E,
    ) -> [E::F; 4] {
        let M31_1 = E::F::from(M31::from(1));
        let M31_128 = E::F::from(M31::from(128));
        let M31_16 = E::F::from(M31::from(16));
        let M31_32 = E::F::from(M31::from(32));
        let M31_32768 = E::F::from(M31::from(32768));
        let M31_64 = E::F::from(M31::from(64));
        let M31_8 = E::F::from(M31::from(8));

        // Flag dst_base_fp is a bit.
        eval.add_constraint(
            (dst_base_fp_col3.clone() * (M31_1.clone() - dst_base_fp_col3.clone())),
        );
        // Flag op0_base_fp is a bit.
        eval.add_constraint(
            (op0_base_fp_col4.clone() * (M31_1.clone() - op0_base_fp_col4.clone())),
        );
        // Flag op1_base_fp is a bit.
        eval.add_constraint(
            (op1_base_fp_col5.clone() * (M31_1.clone() - op1_base_fp_col5.clone())),
        );
        // Flag ap_update_add_1 is a bit.
        eval.add_constraint(
            (ap_update_add_1_col6.clone() * (M31_1.clone() - ap_update_add_1_col6.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            verify_instruction_lookup_elements,
            E::EF::one(),
            &[
                decode_instruction_472fe_input_pc.clone(),
                offset0_col0.clone(),
                offset1_col1.clone(),
                offset2_col2.clone(),
                ((((dst_base_fp_col3.clone() * M31_8.clone())
                    + (op0_base_fp_col4.clone() * M31_16.clone()))
                    + (op1_base_fp_col5.clone() * M31_64.clone()))
                    + ((M31_1.clone() - op1_base_fp_col5.clone()) * M31_128.clone())),
                (ap_update_add_1_col6.clone() * M31_32.clone()),
                opcode_extension_col7.clone(),
            ],
        ));

        [
            (offset0_col0.clone() - M31_32768.clone()),
            (offset1_col1.clone() - M31_32768.clone()),
            (offset2_col2.clone() - M31_32768.clone()),
            (M31_1.clone() - op1_base_fp_col5.clone()),
        ]
    }
}
