use crate::components::prelude::*;
use crate::components::subroutines::cond_felt_252_as_addr::CondFelt252AsAddr;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct HandleOpcodes {}

impl HandleOpcodes {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [handle_opcodes_input_limb_0, handle_opcodes_input_limb_2, handle_opcodes_input_limb_3, handle_opcodes_input_limb_4, handle_opcodes_input_limb_6, handle_opcodes_input_limb_10, handle_opcodes_input_limb_15, handle_opcodes_input_limb_16, handle_opcodes_input_limb_17, handle_opcodes_input_limb_19, handle_opcodes_input_limb_22, handle_opcodes_input_limb_23, handle_opcodes_input_limb_24, handle_opcodes_input_limb_25, handle_opcodes_input_limb_26, handle_opcodes_input_limb_27, handle_opcodes_input_limb_28, handle_opcodes_input_limb_29, handle_opcodes_input_limb_30, handle_opcodes_input_limb_31, handle_opcodes_input_limb_32, handle_opcodes_input_limb_33, handle_opcodes_input_limb_34, handle_opcodes_input_limb_35, handle_opcodes_input_limb_36, handle_opcodes_input_limb_37, handle_opcodes_input_limb_38, handle_opcodes_input_limb_39, handle_opcodes_input_limb_40, handle_opcodes_input_limb_41, handle_opcodes_input_limb_42, handle_opcodes_input_limb_43, handle_opcodes_input_limb_44, handle_opcodes_input_limb_45, handle_opcodes_input_limb_46, handle_opcodes_input_limb_47, handle_opcodes_input_limb_48, handle_opcodes_input_limb_49, handle_opcodes_input_limb_50, handle_opcodes_input_limb_51, handle_opcodes_input_limb_52, handle_opcodes_input_limb_53, handle_opcodes_input_limb_54, handle_opcodes_input_limb_55, handle_opcodes_input_limb_56, handle_opcodes_input_limb_57, handle_opcodes_input_limb_58, handle_opcodes_input_limb_59, handle_opcodes_input_limb_60, handle_opcodes_input_limb_61, handle_opcodes_input_limb_62, handle_opcodes_input_limb_63, handle_opcodes_input_limb_64, handle_opcodes_input_limb_65, handle_opcodes_input_limb_66, handle_opcodes_input_limb_67, handle_opcodes_input_limb_68, handle_opcodes_input_limb_69, handle_opcodes_input_limb_70, handle_opcodes_input_limb_71, handle_opcodes_input_limb_72, handle_opcodes_input_limb_73, handle_opcodes_input_limb_74, handle_opcodes_input_limb_75, handle_opcodes_input_limb_76, handle_opcodes_input_limb_77, handle_opcodes_input_limb_78, handle_opcodes_input_limb_79, handle_opcodes_input_limb_80, handle_opcodes_input_limb_81, handle_opcodes_input_limb_82, handle_opcodes_input_limb_83, handle_opcodes_input_limb_84, handle_opcodes_input_limb_85, handle_opcodes_input_limb_86, handle_opcodes_input_limb_87, handle_opcodes_input_limb_88, handle_opcodes_input_limb_89, handle_opcodes_input_limb_90, handle_opcodes_input_limb_91, handle_opcodes_input_limb_92, handle_opcodes_input_limb_93, handle_opcodes_input_limb_94, handle_opcodes_input_limb_95, handle_opcodes_input_limb_96, handle_opcodes_input_limb_97, handle_opcodes_input_limb_98, handle_opcodes_input_limb_99, handle_opcodes_input_limb_100, handle_opcodes_input_limb_101, handle_opcodes_input_limb_102, handle_opcodes_input_limb_103, handle_opcodes_input_limb_104, handle_opcodes_input_limb_105, handle_opcodes_input_limb_106, handle_opcodes_input_limb_107, handle_opcodes_input_limb_108, handle_opcodes_input_limb_109]: [E::F; 98],
        eval: &mut E,
    ) -> [E::F; 0] {
        let M31_1 = E::F::from(M31::from(1));
        let M31_2 = E::F::from(M31::from(2));
        let M31_4 = E::F::from(M31::from(4));

        eval.add_constraint(
            (handle_opcodes_input_limb_17.clone()
                * (handle_opcodes_input_limb_82.clone() - handle_opcodes_input_limb_26.clone())),
        );
        eval.add_constraint(
            (handle_opcodes_input_limb_17.clone()
                * (handle_opcodes_input_limb_83.clone() - handle_opcodes_input_limb_27.clone())),
        );
        eval.add_constraint(
            (handle_opcodes_input_limb_17.clone()
                * (handle_opcodes_input_limb_84.clone() - handle_opcodes_input_limb_28.clone())),
        );
        eval.add_constraint(
            (handle_opcodes_input_limb_17.clone()
                * (handle_opcodes_input_limb_85.clone() - handle_opcodes_input_limb_29.clone())),
        );
        eval.add_constraint(
            (handle_opcodes_input_limb_17.clone()
                * (handle_opcodes_input_limb_86.clone() - handle_opcodes_input_limb_30.clone())),
        );
        eval.add_constraint(
            (handle_opcodes_input_limb_17.clone()
                * (handle_opcodes_input_limb_87.clone() - handle_opcodes_input_limb_31.clone())),
        );
        eval.add_constraint(
            (handle_opcodes_input_limb_17.clone()
                * (handle_opcodes_input_limb_88.clone() - handle_opcodes_input_limb_32.clone())),
        );
        eval.add_constraint(
            (handle_opcodes_input_limb_17.clone()
                * (handle_opcodes_input_limb_89.clone() - handle_opcodes_input_limb_33.clone())),
        );
        eval.add_constraint(
            (handle_opcodes_input_limb_17.clone()
                * (handle_opcodes_input_limb_90.clone() - handle_opcodes_input_limb_34.clone())),
        );
        eval.add_constraint(
            (handle_opcodes_input_limb_17.clone()
                * (handle_opcodes_input_limb_91.clone() - handle_opcodes_input_limb_35.clone())),
        );
        eval.add_constraint(
            (handle_opcodes_input_limb_17.clone()
                * (handle_opcodes_input_limb_92.clone() - handle_opcodes_input_limb_36.clone())),
        );
        eval.add_constraint(
            (handle_opcodes_input_limb_17.clone()
                * (handle_opcodes_input_limb_93.clone() - handle_opcodes_input_limb_37.clone())),
        );
        eval.add_constraint(
            (handle_opcodes_input_limb_17.clone()
                * (handle_opcodes_input_limb_94.clone() - handle_opcodes_input_limb_38.clone())),
        );
        eval.add_constraint(
            (handle_opcodes_input_limb_17.clone()
                * (handle_opcodes_input_limb_95.clone() - handle_opcodes_input_limb_39.clone())),
        );
        eval.add_constraint(
            (handle_opcodes_input_limb_17.clone()
                * (handle_opcodes_input_limb_96.clone() - handle_opcodes_input_limb_40.clone())),
        );
        eval.add_constraint(
            (handle_opcodes_input_limb_17.clone()
                * (handle_opcodes_input_limb_97.clone() - handle_opcodes_input_limb_41.clone())),
        );
        eval.add_constraint(
            (handle_opcodes_input_limb_17.clone()
                * (handle_opcodes_input_limb_98.clone() - handle_opcodes_input_limb_42.clone())),
        );
        eval.add_constraint(
            (handle_opcodes_input_limb_17.clone()
                * (handle_opcodes_input_limb_99.clone() - handle_opcodes_input_limb_43.clone())),
        );
        eval.add_constraint(
            (handle_opcodes_input_limb_17.clone()
                * (handle_opcodes_input_limb_100.clone() - handle_opcodes_input_limb_44.clone())),
        );
        eval.add_constraint(
            (handle_opcodes_input_limb_17.clone()
                * (handle_opcodes_input_limb_101.clone() - handle_opcodes_input_limb_45.clone())),
        );
        eval.add_constraint(
            (handle_opcodes_input_limb_17.clone()
                * (handle_opcodes_input_limb_102.clone() - handle_opcodes_input_limb_46.clone())),
        );
        eval.add_constraint(
            (handle_opcodes_input_limb_17.clone()
                * (handle_opcodes_input_limb_103.clone() - handle_opcodes_input_limb_47.clone())),
        );
        eval.add_constraint(
            (handle_opcodes_input_limb_17.clone()
                * (handle_opcodes_input_limb_104.clone() - handle_opcodes_input_limb_48.clone())),
        );
        eval.add_constraint(
            (handle_opcodes_input_limb_17.clone()
                * (handle_opcodes_input_limb_105.clone() - handle_opcodes_input_limb_49.clone())),
        );
        eval.add_constraint(
            (handle_opcodes_input_limb_17.clone()
                * (handle_opcodes_input_limb_106.clone() - handle_opcodes_input_limb_50.clone())),
        );
        eval.add_constraint(
            (handle_opcodes_input_limb_17.clone()
                * (handle_opcodes_input_limb_107.clone() - handle_opcodes_input_limb_51.clone())),
        );
        eval.add_constraint(
            (handle_opcodes_input_limb_17.clone()
                * (handle_opcodes_input_limb_108.clone() - handle_opcodes_input_limb_52.clone())),
        );
        eval.add_constraint(
            (handle_opcodes_input_limb_17.clone()
                * (handle_opcodes_input_limb_109.clone() - handle_opcodes_input_limb_53.clone())),
        );
        // ret opcode offset0 equals -2.
        eval.add_constraint(
            (handle_opcodes_input_limb_16.clone()
                * (handle_opcodes_input_limb_23.clone() + M31_2.clone())),
        );
        // ret opcode offset2 equals -1.
        eval.add_constraint(
            (handle_opcodes_input_limb_16.clone()
                * (handle_opcodes_input_limb_25.clone() + M31_1.clone())),
        );
        // ret opcode flags pc_update_jump and dst_base_fp and op1_base_fp_and_res_op1 are on.
        eval.add_constraint(
            (handle_opcodes_input_limb_16.clone()
                * ((((M31_4.clone() - handle_opcodes_input_limb_10.clone())
                    - handle_opcodes_input_limb_3.clone())
                    - handle_opcodes_input_limb_6.clone())
                    - handle_opcodes_input_limb_19.clone())),
        );
        // call opcode offset0 equals 0.
        eval.add_constraint(
            (handle_opcodes_input_limb_15.clone() * handle_opcodes_input_limb_23.clone()),
        );
        // call opcode offset1 equals 1.
        eval.add_constraint(
            (handle_opcodes_input_limb_15.clone()
                * (M31_1.clone() - handle_opcodes_input_limb_24.clone())),
        );
        // call opcode flags op0_base_fp and dst_base_fp are off.
        eval.add_constraint(
            (handle_opcodes_input_limb_15.clone()
                * (handle_opcodes_input_limb_4.clone() + handle_opcodes_input_limb_3.clone())),
        );
        let [cond_felt_252_as_addr_output_tmp_aa5c5_0] = CondFelt252AsAddr::evaluate(
            [
                handle_opcodes_input_limb_26.clone(),
                handle_opcodes_input_limb_27.clone(),
                handle_opcodes_input_limb_28.clone(),
                handle_opcodes_input_limb_29.clone(),
                handle_opcodes_input_limb_30.clone(),
                handle_opcodes_input_limb_31.clone(),
                handle_opcodes_input_limb_32.clone(),
                handle_opcodes_input_limb_33.clone(),
                handle_opcodes_input_limb_34.clone(),
                handle_opcodes_input_limb_35.clone(),
                handle_opcodes_input_limb_36.clone(),
                handle_opcodes_input_limb_37.clone(),
                handle_opcodes_input_limb_38.clone(),
                handle_opcodes_input_limb_39.clone(),
                handle_opcodes_input_limb_40.clone(),
                handle_opcodes_input_limb_41.clone(),
                handle_opcodes_input_limb_42.clone(),
                handle_opcodes_input_limb_43.clone(),
                handle_opcodes_input_limb_44.clone(),
                handle_opcodes_input_limb_45.clone(),
                handle_opcodes_input_limb_46.clone(),
                handle_opcodes_input_limb_47.clone(),
                handle_opcodes_input_limb_48.clone(),
                handle_opcodes_input_limb_49.clone(),
                handle_opcodes_input_limb_50.clone(),
                handle_opcodes_input_limb_51.clone(),
                handle_opcodes_input_limb_52.clone(),
                handle_opcodes_input_limb_53.clone(),
                handle_opcodes_input_limb_15.clone(),
            ],
            eval,
        );
        eval.add_constraint(
            (handle_opcodes_input_limb_15.clone()
                * (cond_felt_252_as_addr_output_tmp_aa5c5_0.clone()
                    - handle_opcodes_input_limb_2.clone())),
        );
        let [cond_felt_252_as_addr_output_tmp_aa5c5_1] = CondFelt252AsAddr::evaluate(
            [
                handle_opcodes_input_limb_54.clone(),
                handle_opcodes_input_limb_55.clone(),
                handle_opcodes_input_limb_56.clone(),
                handle_opcodes_input_limb_57.clone(),
                handle_opcodes_input_limb_58.clone(),
                handle_opcodes_input_limb_59.clone(),
                handle_opcodes_input_limb_60.clone(),
                handle_opcodes_input_limb_61.clone(),
                handle_opcodes_input_limb_62.clone(),
                handle_opcodes_input_limb_63.clone(),
                handle_opcodes_input_limb_64.clone(),
                handle_opcodes_input_limb_65.clone(),
                handle_opcodes_input_limb_66.clone(),
                handle_opcodes_input_limb_67.clone(),
                handle_opcodes_input_limb_68.clone(),
                handle_opcodes_input_limb_69.clone(),
                handle_opcodes_input_limb_70.clone(),
                handle_opcodes_input_limb_71.clone(),
                handle_opcodes_input_limb_72.clone(),
                handle_opcodes_input_limb_73.clone(),
                handle_opcodes_input_limb_74.clone(),
                handle_opcodes_input_limb_75.clone(),
                handle_opcodes_input_limb_76.clone(),
                handle_opcodes_input_limb_77.clone(),
                handle_opcodes_input_limb_78.clone(),
                handle_opcodes_input_limb_79.clone(),
                handle_opcodes_input_limb_80.clone(),
                handle_opcodes_input_limb_81.clone(),
                handle_opcodes_input_limb_15.clone(),
            ],
            eval,
        );
        eval.add_constraint(
            (handle_opcodes_input_limb_15.clone()
                * (cond_felt_252_as_addr_output_tmp_aa5c5_1.clone()
                    - (handle_opcodes_input_limb_0.clone()
                        + handle_opcodes_input_limb_22.clone()))),
        );
        []
    }
}
