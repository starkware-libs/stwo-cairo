// This file was created by the AIR team.

use crate::components::prelude::*;
use crate::components::subroutines::cond_felt_252_as_addr::CondFelt252AsAddr;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct HandleOpcodes {}

impl HandleOpcodes {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [handle_opcodes_input_pc, handle_opcodes_input_fp, handle_opcodes_input_dst_base_fp, handle_opcodes_input_op0_base_fp, handle_opcodes_input_op1_base_fp, handle_opcodes_input_pc_update_jump, handle_opcodes_input_opcode_call, handle_opcodes_input_opcode_ret, handle_opcodes_input_opcode_assert_eq, handle_opcodes_input_res_op1, handle_opcodes_input_instruction_size, handle_opcodes_input_offset0, handle_opcodes_input_offset1, handle_opcodes_input_offset2, handle_opcodes_input_dst_limb_0, handle_opcodes_input_dst_limb_1, handle_opcodes_input_dst_limb_2, handle_opcodes_input_dst_limb_3, handle_opcodes_input_dst_limb_4, handle_opcodes_input_dst_limb_5, handle_opcodes_input_dst_limb_6, handle_opcodes_input_dst_limb_7, handle_opcodes_input_dst_limb_8, handle_opcodes_input_dst_limb_9, handle_opcodes_input_dst_limb_10, handle_opcodes_input_dst_limb_11, handle_opcodes_input_dst_limb_12, handle_opcodes_input_dst_limb_13, handle_opcodes_input_dst_limb_14, handle_opcodes_input_dst_limb_15, handle_opcodes_input_dst_limb_16, handle_opcodes_input_dst_limb_17, handle_opcodes_input_dst_limb_18, handle_opcodes_input_dst_limb_19, handle_opcodes_input_dst_limb_20, handle_opcodes_input_dst_limb_21, handle_opcodes_input_dst_limb_22, handle_opcodes_input_dst_limb_23, handle_opcodes_input_dst_limb_24, handle_opcodes_input_dst_limb_25, handle_opcodes_input_dst_limb_26, handle_opcodes_input_dst_limb_27, handle_opcodes_input_op0_limb_0, handle_opcodes_input_op0_limb_1, handle_opcodes_input_op0_limb_2, handle_opcodes_input_op0_limb_3, handle_opcodes_input_op0_limb_4, handle_opcodes_input_op0_limb_5, handle_opcodes_input_op0_limb_6, handle_opcodes_input_op0_limb_7, handle_opcodes_input_op0_limb_8, handle_opcodes_input_op0_limb_9, handle_opcodes_input_op0_limb_10, handle_opcodes_input_op0_limb_11, handle_opcodes_input_op0_limb_12, handle_opcodes_input_op0_limb_13, handle_opcodes_input_op0_limb_14, handle_opcodes_input_op0_limb_15, handle_opcodes_input_op0_limb_16, handle_opcodes_input_op0_limb_17, handle_opcodes_input_op0_limb_18, handle_opcodes_input_op0_limb_19, handle_opcodes_input_op0_limb_20, handle_opcodes_input_op0_limb_21, handle_opcodes_input_op0_limb_22, handle_opcodes_input_op0_limb_23, handle_opcodes_input_op0_limb_24, handle_opcodes_input_op0_limb_25, handle_opcodes_input_op0_limb_26, handle_opcodes_input_op0_limb_27, handle_opcodes_input_res_limb_0, handle_opcodes_input_res_limb_1, handle_opcodes_input_res_limb_2, handle_opcodes_input_res_limb_3, handle_opcodes_input_res_limb_4, handle_opcodes_input_res_limb_5, handle_opcodes_input_res_limb_6, handle_opcodes_input_res_limb_7, handle_opcodes_input_res_limb_8, handle_opcodes_input_res_limb_9, handle_opcodes_input_res_limb_10, handle_opcodes_input_res_limb_11, handle_opcodes_input_res_limb_12, handle_opcodes_input_res_limb_13, handle_opcodes_input_res_limb_14, handle_opcodes_input_res_limb_15, handle_opcodes_input_res_limb_16, handle_opcodes_input_res_limb_17, handle_opcodes_input_res_limb_18, handle_opcodes_input_res_limb_19, handle_opcodes_input_res_limb_20, handle_opcodes_input_res_limb_21, handle_opcodes_input_res_limb_22, handle_opcodes_input_res_limb_23, handle_opcodes_input_res_limb_24, handle_opcodes_input_res_limb_25, handle_opcodes_input_res_limb_26, handle_opcodes_input_res_limb_27]: [E::F; 98],
        partial_limb_msb_col0: E::F,
        partial_limb_msb_col1: E::F,
        eval: &mut E,
    ) -> [E::F; 0] {
        let M31_1 = E::F::from(M31::from(1));
        let M31_2 = E::F::from(M31::from(2));
        let M31_4 = E::F::from(M31::from(4));

        eval.add_constraint(
            (handle_opcodes_input_opcode_assert_eq.clone()
                * (handle_opcodes_input_res_limb_0.clone()
                    - handle_opcodes_input_dst_limb_0.clone())),
        );
        eval.add_constraint(
            (handle_opcodes_input_opcode_assert_eq.clone()
                * (handle_opcodes_input_res_limb_1.clone()
                    - handle_opcodes_input_dst_limb_1.clone())),
        );
        eval.add_constraint(
            (handle_opcodes_input_opcode_assert_eq.clone()
                * (handle_opcodes_input_res_limb_2.clone()
                    - handle_opcodes_input_dst_limb_2.clone())),
        );
        eval.add_constraint(
            (handle_opcodes_input_opcode_assert_eq.clone()
                * (handle_opcodes_input_res_limb_3.clone()
                    - handle_opcodes_input_dst_limb_3.clone())),
        );
        eval.add_constraint(
            (handle_opcodes_input_opcode_assert_eq.clone()
                * (handle_opcodes_input_res_limb_4.clone()
                    - handle_opcodes_input_dst_limb_4.clone())),
        );
        eval.add_constraint(
            (handle_opcodes_input_opcode_assert_eq.clone()
                * (handle_opcodes_input_res_limb_5.clone()
                    - handle_opcodes_input_dst_limb_5.clone())),
        );
        eval.add_constraint(
            (handle_opcodes_input_opcode_assert_eq.clone()
                * (handle_opcodes_input_res_limb_6.clone()
                    - handle_opcodes_input_dst_limb_6.clone())),
        );
        eval.add_constraint(
            (handle_opcodes_input_opcode_assert_eq.clone()
                * (handle_opcodes_input_res_limb_7.clone()
                    - handle_opcodes_input_dst_limb_7.clone())),
        );
        eval.add_constraint(
            (handle_opcodes_input_opcode_assert_eq.clone()
                * (handle_opcodes_input_res_limb_8.clone()
                    - handle_opcodes_input_dst_limb_8.clone())),
        );
        eval.add_constraint(
            (handle_opcodes_input_opcode_assert_eq.clone()
                * (handle_opcodes_input_res_limb_9.clone()
                    - handle_opcodes_input_dst_limb_9.clone())),
        );
        eval.add_constraint(
            (handle_opcodes_input_opcode_assert_eq.clone()
                * (handle_opcodes_input_res_limb_10.clone()
                    - handle_opcodes_input_dst_limb_10.clone())),
        );
        eval.add_constraint(
            (handle_opcodes_input_opcode_assert_eq.clone()
                * (handle_opcodes_input_res_limb_11.clone()
                    - handle_opcodes_input_dst_limb_11.clone())),
        );
        eval.add_constraint(
            (handle_opcodes_input_opcode_assert_eq.clone()
                * (handle_opcodes_input_res_limb_12.clone()
                    - handle_opcodes_input_dst_limb_12.clone())),
        );
        eval.add_constraint(
            (handle_opcodes_input_opcode_assert_eq.clone()
                * (handle_opcodes_input_res_limb_13.clone()
                    - handle_opcodes_input_dst_limb_13.clone())),
        );
        eval.add_constraint(
            (handle_opcodes_input_opcode_assert_eq.clone()
                * (handle_opcodes_input_res_limb_14.clone()
                    - handle_opcodes_input_dst_limb_14.clone())),
        );
        eval.add_constraint(
            (handle_opcodes_input_opcode_assert_eq.clone()
                * (handle_opcodes_input_res_limb_15.clone()
                    - handle_opcodes_input_dst_limb_15.clone())),
        );
        eval.add_constraint(
            (handle_opcodes_input_opcode_assert_eq.clone()
                * (handle_opcodes_input_res_limb_16.clone()
                    - handle_opcodes_input_dst_limb_16.clone())),
        );
        eval.add_constraint(
            (handle_opcodes_input_opcode_assert_eq.clone()
                * (handle_opcodes_input_res_limb_17.clone()
                    - handle_opcodes_input_dst_limb_17.clone())),
        );
        eval.add_constraint(
            (handle_opcodes_input_opcode_assert_eq.clone()
                * (handle_opcodes_input_res_limb_18.clone()
                    - handle_opcodes_input_dst_limb_18.clone())),
        );
        eval.add_constraint(
            (handle_opcodes_input_opcode_assert_eq.clone()
                * (handle_opcodes_input_res_limb_19.clone()
                    - handle_opcodes_input_dst_limb_19.clone())),
        );
        eval.add_constraint(
            (handle_opcodes_input_opcode_assert_eq.clone()
                * (handle_opcodes_input_res_limb_20.clone()
                    - handle_opcodes_input_dst_limb_20.clone())),
        );
        eval.add_constraint(
            (handle_opcodes_input_opcode_assert_eq.clone()
                * (handle_opcodes_input_res_limb_21.clone()
                    - handle_opcodes_input_dst_limb_21.clone())),
        );
        eval.add_constraint(
            (handle_opcodes_input_opcode_assert_eq.clone()
                * (handle_opcodes_input_res_limb_22.clone()
                    - handle_opcodes_input_dst_limb_22.clone())),
        );
        eval.add_constraint(
            (handle_opcodes_input_opcode_assert_eq.clone()
                * (handle_opcodes_input_res_limb_23.clone()
                    - handle_opcodes_input_dst_limb_23.clone())),
        );
        eval.add_constraint(
            (handle_opcodes_input_opcode_assert_eq.clone()
                * (handle_opcodes_input_res_limb_24.clone()
                    - handle_opcodes_input_dst_limb_24.clone())),
        );
        eval.add_constraint(
            (handle_opcodes_input_opcode_assert_eq.clone()
                * (handle_opcodes_input_res_limb_25.clone()
                    - handle_opcodes_input_dst_limb_25.clone())),
        );
        eval.add_constraint(
            (handle_opcodes_input_opcode_assert_eq.clone()
                * (handle_opcodes_input_res_limb_26.clone()
                    - handle_opcodes_input_dst_limb_26.clone())),
        );
        eval.add_constraint(
            (handle_opcodes_input_opcode_assert_eq.clone()
                * (handle_opcodes_input_res_limb_27.clone()
                    - handle_opcodes_input_dst_limb_27.clone())),
        );
        // ret opcode offset0 equals -2.
        eval.add_constraint(
            (handle_opcodes_input_opcode_ret.clone()
                * (handle_opcodes_input_offset0.clone() + M31_2.clone())),
        );
        // ret opcode offset2 equals -1.
        eval.add_constraint(
            (handle_opcodes_input_opcode_ret.clone()
                * (handle_opcodes_input_offset2.clone() + M31_1.clone())),
        );
        // ret opcode flags pc_update_jump and dst_base_fp and op1_base_fp_and_res_op1 are on.
        eval.add_constraint(
            (handle_opcodes_input_opcode_ret.clone()
                * ((((M31_4.clone() - handle_opcodes_input_pc_update_jump.clone())
                    - handle_opcodes_input_dst_base_fp.clone())
                    - handle_opcodes_input_op1_base_fp.clone())
                    - handle_opcodes_input_res_op1.clone())),
        );
        // call opcode offset0 equals 0.
        eval.add_constraint(
            (handle_opcodes_input_opcode_call.clone() * handle_opcodes_input_offset0.clone()),
        );
        // call opcode offset1 equals 1.
        eval.add_constraint(
            (handle_opcodes_input_opcode_call.clone()
                * (M31_1.clone() - handle_opcodes_input_offset1.clone())),
        );
        // call opcode flags op0_base_fp and dst_base_fp are off.
        eval.add_constraint(
            (handle_opcodes_input_opcode_call.clone()
                * (handle_opcodes_input_op0_base_fp.clone()
                    + handle_opcodes_input_dst_base_fp.clone())),
        );
        let [cond_felt_252_as_addr_output_tmp_aa5c5_2] = CondFelt252AsAddr::evaluate(
            [
                handle_opcodes_input_dst_limb_0.clone(),
                handle_opcodes_input_dst_limb_1.clone(),
                handle_opcodes_input_dst_limb_2.clone(),
                handle_opcodes_input_dst_limb_3.clone(),
                handle_opcodes_input_dst_limb_4.clone(),
                handle_opcodes_input_dst_limb_5.clone(),
                handle_opcodes_input_dst_limb_6.clone(),
                handle_opcodes_input_dst_limb_7.clone(),
                handle_opcodes_input_dst_limb_8.clone(),
                handle_opcodes_input_dst_limb_9.clone(),
                handle_opcodes_input_dst_limb_10.clone(),
                handle_opcodes_input_dst_limb_11.clone(),
                handle_opcodes_input_dst_limb_12.clone(),
                handle_opcodes_input_dst_limb_13.clone(),
                handle_opcodes_input_dst_limb_14.clone(),
                handle_opcodes_input_dst_limb_15.clone(),
                handle_opcodes_input_dst_limb_16.clone(),
                handle_opcodes_input_dst_limb_17.clone(),
                handle_opcodes_input_dst_limb_18.clone(),
                handle_opcodes_input_dst_limb_19.clone(),
                handle_opcodes_input_dst_limb_20.clone(),
                handle_opcodes_input_dst_limb_21.clone(),
                handle_opcodes_input_dst_limb_22.clone(),
                handle_opcodes_input_dst_limb_23.clone(),
                handle_opcodes_input_dst_limb_24.clone(),
                handle_opcodes_input_dst_limb_25.clone(),
                handle_opcodes_input_dst_limb_26.clone(),
                handle_opcodes_input_dst_limb_27.clone(),
                handle_opcodes_input_opcode_call.clone(),
            ],
            partial_limb_msb_col0.clone(),
            eval,
        );
        eval.add_constraint(
            (handle_opcodes_input_opcode_call.clone()
                * (cond_felt_252_as_addr_output_tmp_aa5c5_2.clone()
                    - handle_opcodes_input_fp.clone())),
        );
        let [cond_felt_252_as_addr_output_tmp_aa5c5_5] = CondFelt252AsAddr::evaluate(
            [
                handle_opcodes_input_op0_limb_0.clone(),
                handle_opcodes_input_op0_limb_1.clone(),
                handle_opcodes_input_op0_limb_2.clone(),
                handle_opcodes_input_op0_limb_3.clone(),
                handle_opcodes_input_op0_limb_4.clone(),
                handle_opcodes_input_op0_limb_5.clone(),
                handle_opcodes_input_op0_limb_6.clone(),
                handle_opcodes_input_op0_limb_7.clone(),
                handle_opcodes_input_op0_limb_8.clone(),
                handle_opcodes_input_op0_limb_9.clone(),
                handle_opcodes_input_op0_limb_10.clone(),
                handle_opcodes_input_op0_limb_11.clone(),
                handle_opcodes_input_op0_limb_12.clone(),
                handle_opcodes_input_op0_limb_13.clone(),
                handle_opcodes_input_op0_limb_14.clone(),
                handle_opcodes_input_op0_limb_15.clone(),
                handle_opcodes_input_op0_limb_16.clone(),
                handle_opcodes_input_op0_limb_17.clone(),
                handle_opcodes_input_op0_limb_18.clone(),
                handle_opcodes_input_op0_limb_19.clone(),
                handle_opcodes_input_op0_limb_20.clone(),
                handle_opcodes_input_op0_limb_21.clone(),
                handle_opcodes_input_op0_limb_22.clone(),
                handle_opcodes_input_op0_limb_23.clone(),
                handle_opcodes_input_op0_limb_24.clone(),
                handle_opcodes_input_op0_limb_25.clone(),
                handle_opcodes_input_op0_limb_26.clone(),
                handle_opcodes_input_op0_limb_27.clone(),
                handle_opcodes_input_opcode_call.clone(),
            ],
            partial_limb_msb_col1.clone(),
            eval,
        );
        eval.add_constraint(
            (handle_opcodes_input_opcode_call.clone()
                * (cond_felt_252_as_addr_output_tmp_aa5c5_5.clone()
                    - (handle_opcodes_input_pc.clone()
                        + handle_opcodes_input_instruction_size.clone()))),
        );
        []
    }
}
