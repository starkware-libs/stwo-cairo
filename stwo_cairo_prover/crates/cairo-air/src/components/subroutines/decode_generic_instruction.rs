// This file was created by the AIR team.

use crate::components::prelude::*;
use crate::components::subroutines::decode_instruction_df7a6::DecodeInstructionDf7A6;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct DecodeGenericInstruction {}

impl DecodeGenericInstruction {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [decode_generic_instruction_input]: [E::F; 1],
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
    ) -> [E::F; 8] {
        let M31_1 = E::F::from(M31::from(1));

        let [decode_instruction_df7a6_output_tmp_62f3c_20_offset0, decode_instruction_df7a6_output_tmp_62f3c_20_offset1, decode_instruction_df7a6_output_tmp_62f3c_20_offset2] =
            DecodeInstructionDf7A6::evaluate(
                [decode_generic_instruction_input.clone()],
                offset0_col0.clone(),
                offset1_col1.clone(),
                offset2_col2.clone(),
                dst_base_fp_col3.clone(),
                op0_base_fp_col4.clone(),
                op1_imm_col5.clone(),
                op1_base_fp_col6.clone(),
                op1_base_ap_col7.clone(),
                res_add_col8.clone(),
                res_mul_col9.clone(),
                pc_update_jump_col10.clone(),
                pc_update_jump_rel_col11.clone(),
                pc_update_jnz_col12.clone(),
                ap_update_add_col13.clone(),
                ap_update_add_1_col14.clone(),
                opcode_call_col15.clone(),
                opcode_ret_col16.clone(),
                opcode_assert_eq_col17.clone(),
                verify_instruction_lookup_elements,
                eval,
            );
        let op1_base_op0_tmp_62f3c_21 = eval.add_intermediate(
            (((M31_1.clone() - op1_imm_col5.clone()) - op1_base_fp_col6.clone())
                - op1_base_ap_col7.clone()),
        );
        // op1_src is 0, 1, 2, or 4.
        eval.add_constraint(
            (op1_base_op0_tmp_62f3c_21.clone()
                * (M31_1.clone() - op1_base_op0_tmp_62f3c_21.clone())),
        );
        let res_op1_tmp_62f3c_22 = eval.add_intermediate(
            (((M31_1.clone() - res_add_col8.clone()) - res_mul_col9.clone())
                - pc_update_jnz_col12.clone()),
        );
        // res_logic is 0, 1, or 2.
        eval.add_constraint(
            (res_op1_tmp_62f3c_22.clone() * (M31_1.clone() - res_op1_tmp_62f3c_22.clone())),
        );
        let pc_update_regular_tmp_62f3c_23 = eval.add_intermediate(
            (((M31_1.clone() - pc_update_jump_col10.clone()) - pc_update_jump_rel_col11.clone())
                - pc_update_jnz_col12.clone()),
        );
        // pc_update is 0, 1, 2, or 4.
        eval.add_constraint(
            (pc_update_regular_tmp_62f3c_23.clone()
                * (M31_1.clone() - pc_update_regular_tmp_62f3c_23.clone())),
        );
        let ap_update_regular_tmp_62f3c_24 = eval.add_intermediate(
            (((M31_1.clone() - ap_update_add_col13.clone()) - ap_update_add_1_col14.clone())
                - opcode_call_col15.clone()),
        );
        // ap_update is 0, 1, 2, or 4.
        eval.add_constraint(
            (ap_update_regular_tmp_62f3c_24.clone()
                * (M31_1.clone() - ap_update_regular_tmp_62f3c_24.clone())),
        );
        let fp_update_regular_tmp_62f3c_25 = eval.add_intermediate(
            ((M31_1.clone() - opcode_call_col15.clone()) - opcode_ret_col16.clone()),
        );
        // opcode is 0, 1, 2, or 4.
        eval.add_constraint(
            (fp_update_regular_tmp_62f3c_25.clone()
                * (M31_1.clone() - fp_update_regular_tmp_62f3c_25.clone())),
        );
        [
            op1_base_op0_tmp_62f3c_21.clone(),
            res_op1_tmp_62f3c_22.clone(),
            pc_update_regular_tmp_62f3c_23.clone(),
            fp_update_regular_tmp_62f3c_25.clone(),
            (M31_1.clone() + op1_imm_col5.clone()),
            decode_instruction_df7a6_output_tmp_62f3c_20_offset0.clone(),
            decode_instruction_df7a6_output_tmp_62f3c_20_offset1.clone(),
            decode_instruction_df7a6_output_tmp_62f3c_20_offset2.clone(),
        ]
    }
}
