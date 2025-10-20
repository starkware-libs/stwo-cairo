// This file was created by the AIR team.

use crate::components::prelude::*;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct DecodeInstructionDf7A6 {}

impl DecodeInstructionDf7A6 {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [decode_instruction_df7a6_input_pc]: [E::F; 1],
        offset0_col0: E::F,
        offset1_col1: E::F,
        offset2_col2: E::F,
        dst_base_fp_col3: E::F,
        op0_base_fp_col4: E::F,
        op1_imm_col5: E::F,
        op1_base_fp_col6: E::F,
        op1_base_ap_col7: E::F,
        res_add_col8: E::F,
        res_mul_col9: E::F,
        pc_update_jump_col10: E::F,
        pc_update_jump_rel_col11: E::F,
        pc_update_jnz_col12: E::F,
        ap_update_add_col13: E::F,
        ap_update_add_1_col14: E::F,
        opcode_call_col15: E::F,
        opcode_ret_col16: E::F,
        opcode_assert_eq_col17: E::F,
        verify_instruction_lookup_elements: &relations::VerifyInstruction,
        eval: &mut E,
    ) -> [E::F; 3] {
        let M31_1 = E::F::from(M31::from(1));
        let M31_128 = E::F::from(M31::from(128));
        let M31_16 = E::F::from(M31::from(16));
        let M31_2 = E::F::from(M31::from(2));
        let M31_256 = E::F::from(M31::from(256));
        let M31_32 = E::F::from(M31::from(32));
        let M31_32768 = E::F::from(M31::from(32768));
        let M31_4 = E::F::from(M31::from(4));
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
        // Flag op1_imm is a bit.
        eval.add_constraint((op1_imm_col5.clone() * (M31_1.clone() - op1_imm_col5.clone())));
        // Flag op1_base_fp is a bit.
        eval.add_constraint(
            (op1_base_fp_col6.clone() * (M31_1.clone() - op1_base_fp_col6.clone())),
        );
        // Flag op1_base_ap is a bit.
        eval.add_constraint(
            (op1_base_ap_col7.clone() * (M31_1.clone() - op1_base_ap_col7.clone())),
        );
        // Flag res_add is a bit.
        eval.add_constraint((res_add_col8.clone() * (M31_1.clone() - res_add_col8.clone())));
        // Flag res_mul is a bit.
        eval.add_constraint((res_mul_col9.clone() * (M31_1.clone() - res_mul_col9.clone())));
        // Flag pc_update_jump is a bit.
        eval.add_constraint(
            (pc_update_jump_col10.clone() * (M31_1.clone() - pc_update_jump_col10.clone())),
        );
        // Flag pc_update_jump_rel is a bit.
        eval.add_constraint(
            (pc_update_jump_rel_col11.clone() * (M31_1.clone() - pc_update_jump_rel_col11.clone())),
        );
        // Flag pc_update_jnz is a bit.
        eval.add_constraint(
            (pc_update_jnz_col12.clone() * (M31_1.clone() - pc_update_jnz_col12.clone())),
        );
        // Flag ap_update_add is a bit.
        eval.add_constraint(
            (ap_update_add_col13.clone() * (M31_1.clone() - ap_update_add_col13.clone())),
        );
        // Flag ap_update_add_1 is a bit.
        eval.add_constraint(
            (ap_update_add_1_col14.clone() * (M31_1.clone() - ap_update_add_1_col14.clone())),
        );
        // Flag opcode_call is a bit.
        eval.add_constraint(
            (opcode_call_col15.clone() * (M31_1.clone() - opcode_call_col15.clone())),
        );
        // Flag opcode_ret is a bit.
        eval.add_constraint(
            (opcode_ret_col16.clone() * (M31_1.clone() - opcode_ret_col16.clone())),
        );
        // Flag opcode_assert_eq is a bit.
        eval.add_constraint(
            (opcode_assert_eq_col17.clone() * (M31_1.clone() - opcode_assert_eq_col17.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            verify_instruction_lookup_elements,
            E::EF::one(),
            &[
                decode_instruction_df7a6_input_pc.clone(),
                offset0_col0.clone(),
                offset1_col1.clone(),
                offset2_col2.clone(),
                ((((((dst_base_fp_col3.clone() * M31_8.clone())
                    + (op0_base_fp_col4.clone() * M31_16.clone()))
                    + (op1_imm_col5.clone() * M31_32.clone()))
                    + (op1_base_fp_col6.clone() * M31_64.clone()))
                    + (op1_base_ap_col7.clone() * M31_128.clone()))
                    + (res_add_col8.clone() * M31_256.clone())),
                ((((((((res_mul_col9.clone()
                    + (pc_update_jump_col10.clone() * M31_2.clone()))
                    + (pc_update_jump_rel_col11.clone() * M31_4.clone()))
                    + (pc_update_jnz_col12.clone() * M31_8.clone()))
                    + (ap_update_add_col13.clone() * M31_16.clone()))
                    + (ap_update_add_1_col14.clone() * M31_32.clone()))
                    + (opcode_call_col15.clone() * M31_64.clone()))
                    + (opcode_ret_col16.clone() * M31_128.clone()))
                    + (opcode_assert_eq_col17.clone() * M31_256.clone())),
            ],
        ));

        [
            (offset0_col0.clone() - M31_32768.clone()),
            (offset1_col1.clone() - M31_32768.clone()),
            (offset2_col2.clone() - M31_32768.clone()),
        ]
    }
}
