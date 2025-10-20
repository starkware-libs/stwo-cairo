// This file was created by the AIR team.

use crate::components::prelude::*;
use crate::components::subroutines::cond_felt_252_as_addr::CondFelt252AsAddr;
use crate::components::subroutines::cond_felt_252_as_rel_imm::CondFelt252AsRelImm;
use crate::components::subroutines::range_check_ap::RangeCheckAp;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct UpdateRegisters {}

impl UpdateRegisters {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [update_registers_input_pc, update_registers_input_ap, update_registers_input_fp, update_registers_input_pc_update_jump, update_registers_input_pc_update_jump_rel, update_registers_input_pc_update_jnz, update_registers_input_ap_update_add, update_registers_input_ap_update_add_1, update_registers_input_opcode_call, update_registers_input_opcode_ret, update_registers_input_pc_update_regular, update_registers_input_fp_update_regular, update_registers_input_instruction_size, update_registers_input_dst_limb_0, update_registers_input_dst_limb_1, update_registers_input_dst_limb_2, update_registers_input_dst_limb_3, update_registers_input_dst_limb_4, update_registers_input_dst_limb_5, update_registers_input_dst_limb_6, update_registers_input_dst_limb_7, update_registers_input_dst_limb_8, update_registers_input_dst_limb_9, update_registers_input_dst_limb_10, update_registers_input_dst_limb_11, update_registers_input_dst_limb_12, update_registers_input_dst_limb_13, update_registers_input_dst_limb_14, update_registers_input_dst_limb_15, update_registers_input_dst_limb_16, update_registers_input_dst_limb_17, update_registers_input_dst_limb_18, update_registers_input_dst_limb_19, update_registers_input_dst_limb_20, update_registers_input_dst_limb_21, update_registers_input_dst_limb_22, update_registers_input_dst_limb_23, update_registers_input_dst_limb_24, update_registers_input_dst_limb_25, update_registers_input_dst_limb_26, update_registers_input_dst_limb_27, update_registers_input_op1_limb_0, update_registers_input_op1_limb_1, update_registers_input_op1_limb_2, update_registers_input_op1_limb_3, update_registers_input_op1_limb_4, update_registers_input_op1_limb_5, update_registers_input_op1_limb_6, update_registers_input_op1_limb_7, update_registers_input_op1_limb_8, update_registers_input_op1_limb_9, update_registers_input_op1_limb_10, update_registers_input_op1_limb_11, update_registers_input_op1_limb_12, update_registers_input_op1_limb_13, update_registers_input_op1_limb_14, update_registers_input_op1_limb_15, update_registers_input_op1_limb_16, update_registers_input_op1_limb_17, update_registers_input_op1_limb_18, update_registers_input_op1_limb_19, update_registers_input_op1_limb_20, update_registers_input_op1_limb_21, update_registers_input_op1_limb_22, update_registers_input_op1_limb_23, update_registers_input_op1_limb_24, update_registers_input_op1_limb_25, update_registers_input_op1_limb_26, update_registers_input_op1_limb_27, update_registers_input_res_limb_0, update_registers_input_res_limb_1, update_registers_input_res_limb_2, update_registers_input_res_limb_3, update_registers_input_res_limb_4, update_registers_input_res_limb_5, update_registers_input_res_limb_6, update_registers_input_res_limb_7, update_registers_input_res_limb_8, update_registers_input_res_limb_9, update_registers_input_res_limb_10, update_registers_input_res_limb_11, update_registers_input_res_limb_12, update_registers_input_res_limb_13, update_registers_input_res_limb_14, update_registers_input_res_limb_15, update_registers_input_res_limb_16, update_registers_input_res_limb_17, update_registers_input_res_limb_18, update_registers_input_res_limb_19, update_registers_input_res_limb_20, update_registers_input_res_limb_21, update_registers_input_res_limb_22, update_registers_input_res_limb_23, update_registers_input_res_limb_24, update_registers_input_res_limb_25, update_registers_input_res_limb_26, update_registers_input_res_limb_27]: [E::F; 97],
        partial_limb_msb_col0: E::F,
        partial_limb_msb_col1: E::F,
        msb_col2: E::F,
        mid_limbs_set_col3: E::F,
        partial_limb_msb_col4: E::F,
        dst_sum_squares_inv_col5: E::F,
        dst_sum_inv_col6: E::F,
        op1_as_rel_imm_cond_col7: E::F,
        msb_col8: E::F,
        mid_limbs_set_col9: E::F,
        partial_limb_msb_col10: E::F,
        next_pc_jnz_col11: E::F,
        next_pc_col12: E::F,
        next_ap_col13: E::F,
        range_check_ap_bot11bits_col14: E::F,
        next_fp_col15: E::F,
        range_check_18_lookup_elements: &relations::RangeCheck_18,
        range_check_11_lookup_elements: &relations::RangeCheck_11,
        eval: &mut E,
    ) -> [E::F; 0] {
        let M31_1 = E::F::from(M31::from(1));
        let M31_136 = E::F::from(M31::from(136));
        let M31_2 = E::F::from(M31::from(2));
        let M31_256 = E::F::from(M31::from(256));

        let [cond_felt_252_as_addr_output_tmp_783d5_2] = CondFelt252AsAddr::evaluate(
            [
                update_registers_input_res_limb_0.clone(),
                update_registers_input_res_limb_1.clone(),
                update_registers_input_res_limb_2.clone(),
                update_registers_input_res_limb_3.clone(),
                update_registers_input_res_limb_4.clone(),
                update_registers_input_res_limb_5.clone(),
                update_registers_input_res_limb_6.clone(),
                update_registers_input_res_limb_7.clone(),
                update_registers_input_res_limb_8.clone(),
                update_registers_input_res_limb_9.clone(),
                update_registers_input_res_limb_10.clone(),
                update_registers_input_res_limb_11.clone(),
                update_registers_input_res_limb_12.clone(),
                update_registers_input_res_limb_13.clone(),
                update_registers_input_res_limb_14.clone(),
                update_registers_input_res_limb_15.clone(),
                update_registers_input_res_limb_16.clone(),
                update_registers_input_res_limb_17.clone(),
                update_registers_input_res_limb_18.clone(),
                update_registers_input_res_limb_19.clone(),
                update_registers_input_res_limb_20.clone(),
                update_registers_input_res_limb_21.clone(),
                update_registers_input_res_limb_22.clone(),
                update_registers_input_res_limb_23.clone(),
                update_registers_input_res_limb_24.clone(),
                update_registers_input_res_limb_25.clone(),
                update_registers_input_res_limb_26.clone(),
                update_registers_input_res_limb_27.clone(),
                update_registers_input_pc_update_jump.clone(),
            ],
            partial_limb_msb_col0.clone(),
            eval,
        );
        let [cond_felt_252_as_addr_output_tmp_783d5_5] = CondFelt252AsAddr::evaluate(
            [
                update_registers_input_dst_limb_0.clone(),
                update_registers_input_dst_limb_1.clone(),
                update_registers_input_dst_limb_2.clone(),
                update_registers_input_dst_limb_3.clone(),
                update_registers_input_dst_limb_4.clone(),
                update_registers_input_dst_limb_5.clone(),
                update_registers_input_dst_limb_6.clone(),
                update_registers_input_dst_limb_7.clone(),
                update_registers_input_dst_limb_8.clone(),
                update_registers_input_dst_limb_9.clone(),
                update_registers_input_dst_limb_10.clone(),
                update_registers_input_dst_limb_11.clone(),
                update_registers_input_dst_limb_12.clone(),
                update_registers_input_dst_limb_13.clone(),
                update_registers_input_dst_limb_14.clone(),
                update_registers_input_dst_limb_15.clone(),
                update_registers_input_dst_limb_16.clone(),
                update_registers_input_dst_limb_17.clone(),
                update_registers_input_dst_limb_18.clone(),
                update_registers_input_dst_limb_19.clone(),
                update_registers_input_dst_limb_20.clone(),
                update_registers_input_dst_limb_21.clone(),
                update_registers_input_dst_limb_22.clone(),
                update_registers_input_dst_limb_23.clone(),
                update_registers_input_dst_limb_24.clone(),
                update_registers_input_dst_limb_25.clone(),
                update_registers_input_dst_limb_26.clone(),
                update_registers_input_dst_limb_27.clone(),
                update_registers_input_opcode_ret.clone(),
            ],
            partial_limb_msb_col1.clone(),
            eval,
        );
        let [cond_felt_252_as_rel_imm_output_tmp_783d5_12] = CondFelt252AsRelImm::evaluate(
            [
                update_registers_input_res_limb_0.clone(),
                update_registers_input_res_limb_1.clone(),
                update_registers_input_res_limb_2.clone(),
                update_registers_input_res_limb_3.clone(),
                update_registers_input_res_limb_4.clone(),
                update_registers_input_res_limb_5.clone(),
                update_registers_input_res_limb_6.clone(),
                update_registers_input_res_limb_7.clone(),
                update_registers_input_res_limb_8.clone(),
                update_registers_input_res_limb_9.clone(),
                update_registers_input_res_limb_10.clone(),
                update_registers_input_res_limb_11.clone(),
                update_registers_input_res_limb_12.clone(),
                update_registers_input_res_limb_13.clone(),
                update_registers_input_res_limb_14.clone(),
                update_registers_input_res_limb_15.clone(),
                update_registers_input_res_limb_16.clone(),
                update_registers_input_res_limb_17.clone(),
                update_registers_input_res_limb_18.clone(),
                update_registers_input_res_limb_19.clone(),
                update_registers_input_res_limb_20.clone(),
                update_registers_input_res_limb_21.clone(),
                update_registers_input_res_limb_22.clone(),
                update_registers_input_res_limb_23.clone(),
                update_registers_input_res_limb_24.clone(),
                update_registers_input_res_limb_25.clone(),
                update_registers_input_res_limb_26.clone(),
                update_registers_input_res_limb_27.clone(),
                (update_registers_input_pc_update_jump_rel.clone()
                    + update_registers_input_ap_update_add.clone()),
            ],
            msb_col2.clone(),
            mid_limbs_set_col3.clone(),
            partial_limb_msb_col4.clone(),
            eval,
        );
        let diff_from_p_tmp_783d5_13 =
            eval.add_intermediate((update_registers_input_dst_limb_0.clone() - M31_1.clone()));
        let diff_from_p_tmp_783d5_14 =
            eval.add_intermediate((update_registers_input_dst_limb_21.clone() - M31_136.clone()));
        let diff_from_p_tmp_783d5_15 =
            eval.add_intermediate((update_registers_input_dst_limb_27.clone() - M31_256.clone()));
        // dst_not_p.
        eval.add_constraint(
            ((((((((((((((((((((((((((((((diff_from_p_tmp_783d5_13
                .clone()
                * diff_from_p_tmp_783d5_13.clone())
                + update_registers_input_dst_limb_1.clone())
                + update_registers_input_dst_limb_2.clone())
                + update_registers_input_dst_limb_3.clone())
                + update_registers_input_dst_limb_4.clone())
                + update_registers_input_dst_limb_5.clone())
                + update_registers_input_dst_limb_6.clone())
                + update_registers_input_dst_limb_7.clone())
                + update_registers_input_dst_limb_8.clone())
                + update_registers_input_dst_limb_9.clone())
                + update_registers_input_dst_limb_10.clone())
                + update_registers_input_dst_limb_11.clone())
                + update_registers_input_dst_limb_12.clone())
                + update_registers_input_dst_limb_13.clone())
                + update_registers_input_dst_limb_14.clone())
                + update_registers_input_dst_limb_15.clone())
                + update_registers_input_dst_limb_16.clone())
                + update_registers_input_dst_limb_17.clone())
                + update_registers_input_dst_limb_18.clone())
                + update_registers_input_dst_limb_19.clone())
                + update_registers_input_dst_limb_20.clone())
                + (diff_from_p_tmp_783d5_14.clone() * diff_from_p_tmp_783d5_14.clone()))
                + update_registers_input_dst_limb_22.clone())
                + update_registers_input_dst_limb_23.clone())
                + update_registers_input_dst_limb_24.clone())
                + update_registers_input_dst_limb_25.clone())
                + update_registers_input_dst_limb_26.clone())
                + (diff_from_p_tmp_783d5_15.clone() * diff_from_p_tmp_783d5_15.clone()))
                * dst_sum_squares_inv_col5.clone())
                - M31_1.clone()),
        );
        let dst_sum_tmp_783d5_16 = eval.add_intermediate(
            (((((((((((((((((((((((((((update_registers_input_dst_limb_0
                .clone()
                + update_registers_input_dst_limb_1.clone())
                + update_registers_input_dst_limb_2.clone())
                + update_registers_input_dst_limb_3.clone())
                + update_registers_input_dst_limb_4.clone())
                + update_registers_input_dst_limb_5.clone())
                + update_registers_input_dst_limb_6.clone())
                + update_registers_input_dst_limb_7.clone())
                + update_registers_input_dst_limb_8.clone())
                + update_registers_input_dst_limb_9.clone())
                + update_registers_input_dst_limb_10.clone())
                + update_registers_input_dst_limb_11.clone())
                + update_registers_input_dst_limb_12.clone())
                + update_registers_input_dst_limb_13.clone())
                + update_registers_input_dst_limb_14.clone())
                + update_registers_input_dst_limb_15.clone())
                + update_registers_input_dst_limb_16.clone())
                + update_registers_input_dst_limb_17.clone())
                + update_registers_input_dst_limb_18.clone())
                + update_registers_input_dst_limb_19.clone())
                + update_registers_input_dst_limb_20.clone())
                + update_registers_input_dst_limb_21.clone())
                + update_registers_input_dst_limb_22.clone())
                + update_registers_input_dst_limb_23.clone())
                + update_registers_input_dst_limb_24.clone())
                + update_registers_input_dst_limb_25.clone())
                + update_registers_input_dst_limb_26.clone())
                + update_registers_input_dst_limb_27.clone()),
        );
        // op1_as_rel_imm_cond.
        eval.add_constraint(
            (op1_as_rel_imm_cond_col7.clone()
                - (update_registers_input_pc_update_jnz.clone() * dst_sum_tmp_783d5_16.clone())),
        );
        let [cond_felt_252_as_rel_imm_output_tmp_783d5_24] = CondFelt252AsRelImm::evaluate(
            [
                update_registers_input_op1_limb_0.clone(),
                update_registers_input_op1_limb_1.clone(),
                update_registers_input_op1_limb_2.clone(),
                update_registers_input_op1_limb_3.clone(),
                update_registers_input_op1_limb_4.clone(),
                update_registers_input_op1_limb_5.clone(),
                update_registers_input_op1_limb_6.clone(),
                update_registers_input_op1_limb_7.clone(),
                update_registers_input_op1_limb_8.clone(),
                update_registers_input_op1_limb_9.clone(),
                update_registers_input_op1_limb_10.clone(),
                update_registers_input_op1_limb_11.clone(),
                update_registers_input_op1_limb_12.clone(),
                update_registers_input_op1_limb_13.clone(),
                update_registers_input_op1_limb_14.clone(),
                update_registers_input_op1_limb_15.clone(),
                update_registers_input_op1_limb_16.clone(),
                update_registers_input_op1_limb_17.clone(),
                update_registers_input_op1_limb_18.clone(),
                update_registers_input_op1_limb_19.clone(),
                update_registers_input_op1_limb_20.clone(),
                update_registers_input_op1_limb_21.clone(),
                update_registers_input_op1_limb_22.clone(),
                update_registers_input_op1_limb_23.clone(),
                update_registers_input_op1_limb_24.clone(),
                update_registers_input_op1_limb_25.clone(),
                update_registers_input_op1_limb_26.clone(),
                update_registers_input_op1_limb_27.clone(),
                op1_as_rel_imm_cond_col7.clone(),
            ],
            msb_col8.clone(),
            mid_limbs_set_col9.clone(),
            partial_limb_msb_col10.clone(),
            eval,
        );
        // Constraint1 for conditional jump.
        eval.add_constraint(
            ((next_pc_jnz_col11.clone()
                - (update_registers_input_pc.clone()
                    + cond_felt_252_as_rel_imm_output_tmp_783d5_24.clone()))
                * dst_sum_tmp_783d5_16.clone()),
        );
        // Constraint2 for conditional jump.
        eval.add_constraint(
            ((next_pc_jnz_col11.clone()
                - (update_registers_input_pc.clone()
                    + update_registers_input_instruction_size.clone()))
                * ((dst_sum_tmp_783d5_16.clone() * dst_sum_inv_col6.clone()) - M31_1.clone())),
        );
        // next_pc.
        eval.add_constraint(
            (next_pc_col12.clone()
                - ((((update_registers_input_pc_update_regular.clone()
                    * (update_registers_input_pc.clone()
                        + update_registers_input_instruction_size.clone()))
                    + (update_registers_input_pc_update_jump.clone()
                        * cond_felt_252_as_addr_output_tmp_783d5_2.clone()))
                    + (update_registers_input_pc_update_jump_rel.clone()
                        * (update_registers_input_pc.clone()
                            + cond_felt_252_as_rel_imm_output_tmp_783d5_12.clone())))
                    + (update_registers_input_pc_update_jnz.clone() * next_pc_jnz_col11.clone()))),
        );
        // next_ap.
        eval.add_constraint(
            (next_ap_col13.clone()
                - (((update_registers_input_ap.clone()
                    + (update_registers_input_ap_update_add.clone()
                        * cond_felt_252_as_rel_imm_output_tmp_783d5_12.clone()))
                    + update_registers_input_ap_update_add_1.clone())
                    + (update_registers_input_opcode_call.clone() * M31_2.clone()))),
        );
        RangeCheckAp::evaluate(
            [next_ap_col13.clone()],
            range_check_ap_bot11bits_col14.clone(),
            range_check_18_lookup_elements,
            range_check_11_lookup_elements,
            eval,
        );
        // next_fp.
        eval.add_constraint(
            (next_fp_col15.clone()
                - (((update_registers_input_fp_update_regular.clone()
                    * update_registers_input_fp.clone())
                    + (update_registers_input_opcode_ret.clone()
                        * cond_felt_252_as_addr_output_tmp_783d5_5.clone()))
                    + (update_registers_input_opcode_call.clone()
                        * (update_registers_input_ap.clone() + M31_2.clone())))),
        );
        []
    }
}
