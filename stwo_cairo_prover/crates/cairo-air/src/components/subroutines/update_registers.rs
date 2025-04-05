use crate::components::prelude::*;
use crate::components::subroutines::cond_felt_252_as_addr::CondFelt252AsAddr;
use crate::components::subroutines::cond_felt_252_as_rel_imm::CondFelt252AsRelImm;

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
        [update_registers_input_limb_0, update_registers_input_limb_1, update_registers_input_limb_2, update_registers_input_limb_3, update_registers_input_limb_4, update_registers_input_limb_5, update_registers_input_limb_6, update_registers_input_limb_7, update_registers_input_limb_8, update_registers_input_limb_9, update_registers_input_limb_10, update_registers_input_limb_11, update_registers_input_limb_12, update_registers_input_limb_13, update_registers_input_limb_14, update_registers_input_limb_15, update_registers_input_limb_16, update_registers_input_limb_17, update_registers_input_limb_18, update_registers_input_limb_19, update_registers_input_limb_20, update_registers_input_limb_21, update_registers_input_limb_22, update_registers_input_limb_23, update_registers_input_limb_24, update_registers_input_limb_25, update_registers_input_limb_26, update_registers_input_limb_27, update_registers_input_limb_28, update_registers_input_limb_29, update_registers_input_limb_30, update_registers_input_limb_31, update_registers_input_limb_32, update_registers_input_limb_33, update_registers_input_limb_34, update_registers_input_limb_35, update_registers_input_limb_36, update_registers_input_limb_37, update_registers_input_limb_38, update_registers_input_limb_39, update_registers_input_limb_40, update_registers_input_limb_41, update_registers_input_limb_42, update_registers_input_limb_43, update_registers_input_limb_44, update_registers_input_limb_45, update_registers_input_limb_46, update_registers_input_limb_47, update_registers_input_limb_48, update_registers_input_limb_49, update_registers_input_limb_50, update_registers_input_limb_51, update_registers_input_limb_52, update_registers_input_limb_53, update_registers_input_limb_54, update_registers_input_limb_55, update_registers_input_limb_56, update_registers_input_limb_57, update_registers_input_limb_58, update_registers_input_limb_59, update_registers_input_limb_60, update_registers_input_limb_61, update_registers_input_limb_62, update_registers_input_limb_63, update_registers_input_limb_64, update_registers_input_limb_65, update_registers_input_limb_66, update_registers_input_limb_67, update_registers_input_limb_68, update_registers_input_limb_69, update_registers_input_limb_70, update_registers_input_limb_71, update_registers_input_limb_72, update_registers_input_limb_73, update_registers_input_limb_74, update_registers_input_limb_75, update_registers_input_limb_76, update_registers_input_limb_77, update_registers_input_limb_78, update_registers_input_limb_79, update_registers_input_limb_80, update_registers_input_limb_81, update_registers_input_limb_82, update_registers_input_limb_83, update_registers_input_limb_84, update_registers_input_limb_85, update_registers_input_limb_86, update_registers_input_limb_87, update_registers_input_limb_88, update_registers_input_limb_89, update_registers_input_limb_90, update_registers_input_limb_91, update_registers_input_limb_92, update_registers_input_limb_93, update_registers_input_limb_94, update_registers_input_limb_95, update_registers_input_limb_96, update_registers_input_limb_97, update_registers_input_limb_98, update_registers_input_limb_99, update_registers_input_limb_100, update_registers_input_limb_101, update_registers_input_limb_102, update_registers_input_limb_103, update_registers_input_limb_104, update_registers_input_limb_105, update_registers_input_limb_106]: [E::F; 107],
        msb_col0: E::F,
        mid_limbs_set_col1: E::F,
        dst_sum_squares_inv_col2: E::F,
        dst_sum_inv_col3: E::F,
        op1_as_rel_imm_cond_col4: E::F,
        msb_col5: E::F,
        mid_limbs_set_col6: E::F,
        next_pc_jnz_col7: E::F,
        next_pc_col8: E::F,
        next_ap_col9: E::F,
        next_fp_col10: E::F,
        eval: &mut E,
    ) -> [E::F; 3] {
        let M31_1 = E::F::from(M31::from(1));
        let M31_136 = E::F::from(M31::from(136));
        let M31_2 = E::F::from(M31::from(2));
        let M31_256 = E::F::from(M31::from(256));

        let cond_felt_252_as_addr_output_tmp_783d5_0 = CondFelt252AsAddr::evaluate(
            [
                update_registers_input_limb_79.clone(),
                update_registers_input_limb_80.clone(),
                update_registers_input_limb_81.clone(),
                update_registers_input_limb_82.clone(),
                update_registers_input_limb_83.clone(),
                update_registers_input_limb_84.clone(),
                update_registers_input_limb_85.clone(),
                update_registers_input_limb_86.clone(),
                update_registers_input_limb_87.clone(),
                update_registers_input_limb_88.clone(),
                update_registers_input_limb_89.clone(),
                update_registers_input_limb_90.clone(),
                update_registers_input_limb_91.clone(),
                update_registers_input_limb_92.clone(),
                update_registers_input_limb_93.clone(),
                update_registers_input_limb_94.clone(),
                update_registers_input_limb_95.clone(),
                update_registers_input_limb_96.clone(),
                update_registers_input_limb_97.clone(),
                update_registers_input_limb_98.clone(),
                update_registers_input_limb_99.clone(),
                update_registers_input_limb_100.clone(),
                update_registers_input_limb_101.clone(),
                update_registers_input_limb_102.clone(),
                update_registers_input_limb_103.clone(),
                update_registers_input_limb_104.clone(),
                update_registers_input_limb_105.clone(),
                update_registers_input_limb_106.clone(),
                update_registers_input_limb_10.clone(),
            ],
            eval,
        );
        let cond_felt_252_as_addr_output_tmp_783d5_1 = CondFelt252AsAddr::evaluate(
            [
                update_registers_input_limb_23.clone(),
                update_registers_input_limb_24.clone(),
                update_registers_input_limb_25.clone(),
                update_registers_input_limb_26.clone(),
                update_registers_input_limb_27.clone(),
                update_registers_input_limb_28.clone(),
                update_registers_input_limb_29.clone(),
                update_registers_input_limb_30.clone(),
                update_registers_input_limb_31.clone(),
                update_registers_input_limb_32.clone(),
                update_registers_input_limb_33.clone(),
                update_registers_input_limb_34.clone(),
                update_registers_input_limb_35.clone(),
                update_registers_input_limb_36.clone(),
                update_registers_input_limb_37.clone(),
                update_registers_input_limb_38.clone(),
                update_registers_input_limb_39.clone(),
                update_registers_input_limb_40.clone(),
                update_registers_input_limb_41.clone(),
                update_registers_input_limb_42.clone(),
                update_registers_input_limb_43.clone(),
                update_registers_input_limb_44.clone(),
                update_registers_input_limb_45.clone(),
                update_registers_input_limb_46.clone(),
                update_registers_input_limb_47.clone(),
                update_registers_input_limb_48.clone(),
                update_registers_input_limb_49.clone(),
                update_registers_input_limb_50.clone(),
                update_registers_input_limb_16.clone(),
            ],
            eval,
        );
        let cond_felt_252_as_rel_imm_output_tmp_783d5_5 = CondFelt252AsRelImm::evaluate(
            [
                update_registers_input_limb_79.clone(),
                update_registers_input_limb_80.clone(),
                update_registers_input_limb_81.clone(),
                update_registers_input_limb_82.clone(),
                update_registers_input_limb_83.clone(),
                update_registers_input_limb_84.clone(),
                update_registers_input_limb_85.clone(),
                update_registers_input_limb_86.clone(),
                update_registers_input_limb_87.clone(),
                update_registers_input_limb_88.clone(),
                update_registers_input_limb_89.clone(),
                update_registers_input_limb_90.clone(),
                update_registers_input_limb_91.clone(),
                update_registers_input_limb_92.clone(),
                update_registers_input_limb_93.clone(),
                update_registers_input_limb_94.clone(),
                update_registers_input_limb_95.clone(),
                update_registers_input_limb_96.clone(),
                update_registers_input_limb_97.clone(),
                update_registers_input_limb_98.clone(),
                update_registers_input_limb_99.clone(),
                update_registers_input_limb_100.clone(),
                update_registers_input_limb_101.clone(),
                update_registers_input_limb_102.clone(),
                update_registers_input_limb_103.clone(),
                update_registers_input_limb_104.clone(),
                update_registers_input_limb_105.clone(),
                update_registers_input_limb_106.clone(),
                (update_registers_input_limb_11.clone() + update_registers_input_limb_13.clone()),
            ],
            msb_col0.clone(),
            mid_limbs_set_col1.clone(),
            eval,
        );
        let diff_from_p_tmp_783d5_6 =
            eval.add_intermediate((update_registers_input_limb_23.clone() - M31_1.clone()));
        let diff_from_p_tmp_783d5_7 =
            eval.add_intermediate((update_registers_input_limb_44.clone() - M31_136.clone()));
        let diff_from_p_tmp_783d5_8 =
            eval.add_intermediate((update_registers_input_limb_50.clone() - M31_256.clone()));
        // dst_not_p.
        eval.add_constraint(
            ((((((((((((((((((((((((((((((diff_from_p_tmp_783d5_6
                .clone()
                * diff_from_p_tmp_783d5_6.clone())
                + update_registers_input_limb_24.clone())
                + update_registers_input_limb_25.clone())
                + update_registers_input_limb_26.clone())
                + update_registers_input_limb_27.clone())
                + update_registers_input_limb_28.clone())
                + update_registers_input_limb_29.clone())
                + update_registers_input_limb_30.clone())
                + update_registers_input_limb_31.clone())
                + update_registers_input_limb_32.clone())
                + update_registers_input_limb_33.clone())
                + update_registers_input_limb_34.clone())
                + update_registers_input_limb_35.clone())
                + update_registers_input_limb_36.clone())
                + update_registers_input_limb_37.clone())
                + update_registers_input_limb_38.clone())
                + update_registers_input_limb_39.clone())
                + update_registers_input_limb_40.clone())
                + update_registers_input_limb_41.clone())
                + update_registers_input_limb_42.clone())
                + update_registers_input_limb_43.clone())
                + (diff_from_p_tmp_783d5_7.clone() * diff_from_p_tmp_783d5_7.clone()))
                + update_registers_input_limb_45.clone())
                + update_registers_input_limb_46.clone())
                + update_registers_input_limb_47.clone())
                + update_registers_input_limb_48.clone())
                + update_registers_input_limb_49.clone())
                + (diff_from_p_tmp_783d5_8.clone() * diff_from_p_tmp_783d5_8.clone()))
                * dst_sum_squares_inv_col2.clone())
                - M31_1.clone()),
        );
        let dst_sum_tmp_783d5_9 = eval.add_intermediate(
            (((((((((((((((((((((((((((update_registers_input_limb_23
                .clone()
                + update_registers_input_limb_24.clone())
                + update_registers_input_limb_25.clone())
                + update_registers_input_limb_26.clone())
                + update_registers_input_limb_27.clone())
                + update_registers_input_limb_28.clone())
                + update_registers_input_limb_29.clone())
                + update_registers_input_limb_30.clone())
                + update_registers_input_limb_31.clone())
                + update_registers_input_limb_32.clone())
                + update_registers_input_limb_33.clone())
                + update_registers_input_limb_34.clone())
                + update_registers_input_limb_35.clone())
                + update_registers_input_limb_36.clone())
                + update_registers_input_limb_37.clone())
                + update_registers_input_limb_38.clone())
                + update_registers_input_limb_39.clone())
                + update_registers_input_limb_40.clone())
                + update_registers_input_limb_41.clone())
                + update_registers_input_limb_42.clone())
                + update_registers_input_limb_43.clone())
                + update_registers_input_limb_44.clone())
                + update_registers_input_limb_45.clone())
                + update_registers_input_limb_46.clone())
                + update_registers_input_limb_47.clone())
                + update_registers_input_limb_48.clone())
                + update_registers_input_limb_49.clone())
                + update_registers_input_limb_50.clone()),
        );
        // op1_as_rel_imm_cond.
        eval.add_constraint(
            (op1_as_rel_imm_cond_col4.clone()
                - (update_registers_input_limb_12.clone() * dst_sum_tmp_783d5_9.clone())),
        );
        let cond_felt_252_as_rel_imm_output_tmp_783d5_14 = CondFelt252AsRelImm::evaluate(
            [
                update_registers_input_limb_51.clone(),
                update_registers_input_limb_52.clone(),
                update_registers_input_limb_53.clone(),
                update_registers_input_limb_54.clone(),
                update_registers_input_limb_55.clone(),
                update_registers_input_limb_56.clone(),
                update_registers_input_limb_57.clone(),
                update_registers_input_limb_58.clone(),
                update_registers_input_limb_59.clone(),
                update_registers_input_limb_60.clone(),
                update_registers_input_limb_61.clone(),
                update_registers_input_limb_62.clone(),
                update_registers_input_limb_63.clone(),
                update_registers_input_limb_64.clone(),
                update_registers_input_limb_65.clone(),
                update_registers_input_limb_66.clone(),
                update_registers_input_limb_67.clone(),
                update_registers_input_limb_68.clone(),
                update_registers_input_limb_69.clone(),
                update_registers_input_limb_70.clone(),
                update_registers_input_limb_71.clone(),
                update_registers_input_limb_72.clone(),
                update_registers_input_limb_73.clone(),
                update_registers_input_limb_74.clone(),
                update_registers_input_limb_75.clone(),
                update_registers_input_limb_76.clone(),
                update_registers_input_limb_77.clone(),
                update_registers_input_limb_78.clone(),
                op1_as_rel_imm_cond_col4.clone(),
            ],
            msb_col5.clone(),
            mid_limbs_set_col6.clone(),
            eval,
        );
        // Constraint1 for conditional jump.
        eval.add_constraint(
            ((next_pc_jnz_col7.clone()
                - (update_registers_input_limb_0.clone()
                    + cond_felt_252_as_rel_imm_output_tmp_783d5_14.clone()))
                * dst_sum_tmp_783d5_9.clone()),
        );
        // Constraint2 for conditional jump.
        eval.add_constraint(
            ((next_pc_jnz_col7.clone()
                - (update_registers_input_limb_0.clone()
                    + update_registers_input_limb_22.clone()))
                * ((dst_sum_tmp_783d5_9.clone() * dst_sum_inv_col3.clone()) - M31_1.clone())),
        );
        // next_pc.
        eval.add_constraint(
            (next_pc_col8.clone()
                - ((((update_registers_input_limb_20.clone()
                    * (update_registers_input_limb_0.clone()
                        + update_registers_input_limb_22.clone()))
                    + (update_registers_input_limb_10.clone()
                        * cond_felt_252_as_addr_output_tmp_783d5_0.clone()))
                    + (update_registers_input_limb_11.clone()
                        * (update_registers_input_limb_0.clone()
                            + cond_felt_252_as_rel_imm_output_tmp_783d5_5.clone())))
                    + (update_registers_input_limb_12.clone() * next_pc_jnz_col7.clone()))),
        );
        // next_ap.
        eval.add_constraint(
            (next_ap_col9.clone()
                - (((update_registers_input_limb_1.clone()
                    + (update_registers_input_limb_13.clone()
                        * cond_felt_252_as_rel_imm_output_tmp_783d5_5.clone()))
                    + update_registers_input_limb_14.clone())
                    + (update_registers_input_limb_15.clone() * M31_2.clone()))),
        );
        // next_fp.
        eval.add_constraint(
            (next_fp_col10.clone()
                - (((update_registers_input_limb_21.clone()
                    * update_registers_input_limb_2.clone())
                    + (update_registers_input_limb_16.clone()
                        * cond_felt_252_as_addr_output_tmp_783d5_1.clone()))
                    + (update_registers_input_limb_15.clone()
                        * (update_registers_input_limb_1.clone() + M31_2.clone())))),
        );
        [
            next_pc_col8.clone(),
            next_ap_col9.clone(),
            next_fp_col10.clone(),
        ]
    }
}
