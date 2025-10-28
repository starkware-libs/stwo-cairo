// This file was created by the AIR team.

use crate::components::prelude::*;
use crate::components::subroutines::cond_range_check_2::CondRangeCheck2;
use crate::components::subroutines::decode_small_sign::DecodeSmallSign;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct CondFelt252AsRelImm {}

impl CondFelt252AsRelImm {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [cond_felt_252_as_rel_imm_input_limb_0, cond_felt_252_as_rel_imm_input_limb_1, cond_felt_252_as_rel_imm_input_limb_2, cond_felt_252_as_rel_imm_input_limb_3, cond_felt_252_as_rel_imm_input_limb_4, cond_felt_252_as_rel_imm_input_limb_5, cond_felt_252_as_rel_imm_input_limb_6, cond_felt_252_as_rel_imm_input_limb_7, cond_felt_252_as_rel_imm_input_limb_8, cond_felt_252_as_rel_imm_input_limb_9, cond_felt_252_as_rel_imm_input_limb_10, cond_felt_252_as_rel_imm_input_limb_11, cond_felt_252_as_rel_imm_input_limb_12, cond_felt_252_as_rel_imm_input_limb_13, cond_felt_252_as_rel_imm_input_limb_14, cond_felt_252_as_rel_imm_input_limb_15, cond_felt_252_as_rel_imm_input_limb_16, cond_felt_252_as_rel_imm_input_limb_17, cond_felt_252_as_rel_imm_input_limb_18, cond_felt_252_as_rel_imm_input_limb_19, cond_felt_252_as_rel_imm_input_limb_20, cond_felt_252_as_rel_imm_input_limb_21, cond_felt_252_as_rel_imm_input_limb_22, cond_felt_252_as_rel_imm_input_limb_23, cond_felt_252_as_rel_imm_input_limb_24, cond_felt_252_as_rel_imm_input_limb_25, cond_felt_252_as_rel_imm_input_limb_26, cond_felt_252_as_rel_imm_input_limb_27, cond_felt_252_as_rel_imm_input_limb_28]: [E::F; 29],
        msb_col0: E::F,
        mid_limbs_set_col1: E::F,
        partial_limb_msb_col2: E::F,
        eval: &mut E,
    ) -> [E::F; 1] {
        let M31_134217728 = E::F::from(M31::from(134217728));
        let M31_136 = E::F::from(M31::from(136));
        let M31_256 = E::F::from(M31::from(256));
        let M31_262144 = E::F::from(M31::from(262144));
        let M31_508 = E::F::from(M31::from(508));
        let M31_511 = E::F::from(M31::from(511));
        let M31_512 = E::F::from(M31::from(512));
        let M31_536870912 = E::F::from(M31::from(536870912));

        DecodeSmallSign::evaluate([], msb_col0.clone(), mid_limbs_set_col1.clone(), eval);
        let remainder_bits_tmp_1e9bf_3 = eval.add_intermediate(
            (cond_felt_252_as_rel_imm_input_limb_3.clone()
                - (mid_limbs_set_col1.clone() * M31_508.clone())),
        );
        CondRangeCheck2::evaluate(
            [
                remainder_bits_tmp_1e9bf_3.clone(),
                cond_felt_252_as_rel_imm_input_limb_28.clone(),
            ],
            partial_limb_msb_col2.clone(),
            eval,
        );
        // rel_imm limb 4 is fixed.
        eval.add_constraint(
            (cond_felt_252_as_rel_imm_input_limb_28.clone()
                * (cond_felt_252_as_rel_imm_input_limb_4.clone()
                    - (mid_limbs_set_col1.clone() * M31_511.clone()))),
        );
        // rel_imm limb 5 is fixed.
        eval.add_constraint(
            (cond_felt_252_as_rel_imm_input_limb_28.clone()
                * (cond_felt_252_as_rel_imm_input_limb_5.clone()
                    - (mid_limbs_set_col1.clone() * M31_511.clone()))),
        );
        // rel_imm limb 6 is fixed.
        eval.add_constraint(
            (cond_felt_252_as_rel_imm_input_limb_28.clone()
                * (cond_felt_252_as_rel_imm_input_limb_6.clone()
                    - (mid_limbs_set_col1.clone() * M31_511.clone()))),
        );
        // rel_imm limb 7 is fixed.
        eval.add_constraint(
            (cond_felt_252_as_rel_imm_input_limb_28.clone()
                * (cond_felt_252_as_rel_imm_input_limb_7.clone()
                    - (mid_limbs_set_col1.clone() * M31_511.clone()))),
        );
        // rel_imm limb 8 is fixed.
        eval.add_constraint(
            (cond_felt_252_as_rel_imm_input_limb_28.clone()
                * (cond_felt_252_as_rel_imm_input_limb_8.clone()
                    - (mid_limbs_set_col1.clone() * M31_511.clone()))),
        );
        // rel_imm limb 9 is fixed.
        eval.add_constraint(
            (cond_felt_252_as_rel_imm_input_limb_28.clone()
                * (cond_felt_252_as_rel_imm_input_limb_9.clone()
                    - (mid_limbs_set_col1.clone() * M31_511.clone()))),
        );
        // rel_imm limb 10 is fixed.
        eval.add_constraint(
            (cond_felt_252_as_rel_imm_input_limb_28.clone()
                * (cond_felt_252_as_rel_imm_input_limb_10.clone()
                    - (mid_limbs_set_col1.clone() * M31_511.clone()))),
        );
        // rel_imm limb 11 is fixed.
        eval.add_constraint(
            (cond_felt_252_as_rel_imm_input_limb_28.clone()
                * (cond_felt_252_as_rel_imm_input_limb_11.clone()
                    - (mid_limbs_set_col1.clone() * M31_511.clone()))),
        );
        // rel_imm limb 12 is fixed.
        eval.add_constraint(
            (cond_felt_252_as_rel_imm_input_limb_28.clone()
                * (cond_felt_252_as_rel_imm_input_limb_12.clone()
                    - (mid_limbs_set_col1.clone() * M31_511.clone()))),
        );
        // rel_imm limb 13 is fixed.
        eval.add_constraint(
            (cond_felt_252_as_rel_imm_input_limb_28.clone()
                * (cond_felt_252_as_rel_imm_input_limb_13.clone()
                    - (mid_limbs_set_col1.clone() * M31_511.clone()))),
        );
        // rel_imm limb 14 is fixed.
        eval.add_constraint(
            (cond_felt_252_as_rel_imm_input_limb_28.clone()
                * (cond_felt_252_as_rel_imm_input_limb_14.clone()
                    - (mid_limbs_set_col1.clone() * M31_511.clone()))),
        );
        // rel_imm limb 15 is fixed.
        eval.add_constraint(
            (cond_felt_252_as_rel_imm_input_limb_28.clone()
                * (cond_felt_252_as_rel_imm_input_limb_15.clone()
                    - (mid_limbs_set_col1.clone() * M31_511.clone()))),
        );
        // rel_imm limb 16 is fixed.
        eval.add_constraint(
            (cond_felt_252_as_rel_imm_input_limb_28.clone()
                * (cond_felt_252_as_rel_imm_input_limb_16.clone()
                    - (mid_limbs_set_col1.clone() * M31_511.clone()))),
        );
        // rel_imm limb 17 is fixed.
        eval.add_constraint(
            (cond_felt_252_as_rel_imm_input_limb_28.clone()
                * (cond_felt_252_as_rel_imm_input_limb_17.clone()
                    - (mid_limbs_set_col1.clone() * M31_511.clone()))),
        );
        // rel_imm limb 18 is fixed.
        eval.add_constraint(
            (cond_felt_252_as_rel_imm_input_limb_28.clone()
                * (cond_felt_252_as_rel_imm_input_limb_18.clone()
                    - (mid_limbs_set_col1.clone() * M31_511.clone()))),
        );
        // rel_imm limb 19 is fixed.
        eval.add_constraint(
            (cond_felt_252_as_rel_imm_input_limb_28.clone()
                * (cond_felt_252_as_rel_imm_input_limb_19.clone()
                    - (mid_limbs_set_col1.clone() * M31_511.clone()))),
        );
        // rel_imm limb 20 is fixed.
        eval.add_constraint(
            (cond_felt_252_as_rel_imm_input_limb_28.clone()
                * (cond_felt_252_as_rel_imm_input_limb_20.clone()
                    - (mid_limbs_set_col1.clone() * M31_511.clone()))),
        );
        // rel_imm limb 21 is fixed.
        eval.add_constraint(
            (cond_felt_252_as_rel_imm_input_limb_28.clone()
                * (cond_felt_252_as_rel_imm_input_limb_21.clone()
                    - ((M31_136.clone() * msb_col0.clone()) - mid_limbs_set_col1.clone()))),
        );
        // rel_imm limb 22 is fixed.
        eval.add_constraint(
            (cond_felt_252_as_rel_imm_input_limb_28.clone()
                * cond_felt_252_as_rel_imm_input_limb_22.clone()),
        );
        // rel_imm limb 23 is fixed.
        eval.add_constraint(
            (cond_felt_252_as_rel_imm_input_limb_28.clone()
                * cond_felt_252_as_rel_imm_input_limb_23.clone()),
        );
        // rel_imm limb 24 is fixed.
        eval.add_constraint(
            (cond_felt_252_as_rel_imm_input_limb_28.clone()
                * cond_felt_252_as_rel_imm_input_limb_24.clone()),
        );
        // rel_imm limb 25 is fixed.
        eval.add_constraint(
            (cond_felt_252_as_rel_imm_input_limb_28.clone()
                * cond_felt_252_as_rel_imm_input_limb_25.clone()),
        );
        // rel_imm limb 26 is fixed.
        eval.add_constraint(
            (cond_felt_252_as_rel_imm_input_limb_28.clone()
                * cond_felt_252_as_rel_imm_input_limb_26.clone()),
        );
        // rel_imm limb 27 is fixed.
        eval.add_constraint(
            (cond_felt_252_as_rel_imm_input_limb_28.clone()
                * (cond_felt_252_as_rel_imm_input_limb_27.clone()
                    - (msb_col0.clone() * M31_256.clone()))),
        );
        [(((((cond_felt_252_as_rel_imm_input_limb_0.clone()
            + (cond_felt_252_as_rel_imm_input_limb_1.clone() * M31_512.clone()))
            + (cond_felt_252_as_rel_imm_input_limb_2.clone() * M31_262144.clone()))
            + (remainder_bits_tmp_1e9bf_3.clone() * M31_134217728.clone()))
            - msb_col0.clone())
            - (M31_536870912.clone() * mid_limbs_set_col1.clone()))]
    }
}
