// This file was created by the AIR team.

use crate::components::prelude::*;
use crate::components::subroutines::decode_instruction_3802d::DecodeInstruction3802D;
use crate::components::subroutines::qm_31_read_reduced::Qm31ReadReduced;

pub const N_TRACE_COLUMNS: usize = 73;
pub const RELATION_USES_PER_ROW: [RelationUse; 5] = [
    RelationUse {
        relation_id: "MemoryAddressToId",
        uses: 3,
    },
    RelationUse {
        relation_id: "MemoryIdToBig",
        uses: 3,
    },
    RelationUse {
        relation_id: "Opcodes",
        uses: 1,
    },
    RelationUse {
        relation_id: "RangeCheck_4_4_4_4",
        uses: 3,
    },
    RelationUse {
        relation_id: "VerifyInstruction",
        uses: 1,
    },
];

pub struct Eval {
    pub claim: Claim,
    pub verify_instruction_lookup_elements: relations::VerifyInstruction,
    pub memory_address_to_id_lookup_elements: relations::MemoryAddressToId,
    pub memory_id_to_big_lookup_elements: relations::MemoryIdToBig,
    pub range_check_4_4_4_4_lookup_elements: relations::RangeCheck_4_4_4_4,
    pub opcodes_lookup_elements: relations::Opcodes,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct Claim {
    pub log_size: u32,
}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let trace_log_sizes = vec![self.log_size; N_TRACE_COLUMNS];
        let interaction_log_sizes = vec![self.log_size; SECURE_EXTENSION_DEGREE * 6];
        TreeVec::new(vec![vec![], trace_log_sizes, interaction_log_sizes])
    }

    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_u64(self.log_size as u64);
    }
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct InteractionClaim {
    pub claimed_sum: SecureField,
}
impl InteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_felts(&[self.claimed_sum]);
    }
}

pub type Component = FrameworkComponent<Eval>;

impl FrameworkEval for Eval {
    fn log_size(&self) -> u32 {
        self.claim.log_size
    }

    fn max_constraint_log_degree_bound(&self) -> u32 {
        self.log_size() + 1
    }

    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    fn evaluate<E: EvalAtRow>(&self, mut eval: E) -> E {
        let M31_1 = E::F::from(M31::from(1));
        let M31_2 = E::F::from(M31::from(2));
        let input_pc_col0 = eval.next_trace_mask();
        let input_ap_col1 = eval.next_trace_mask();
        let input_fp_col2 = eval.next_trace_mask();
        let offset0_col3 = eval.next_trace_mask();
        let offset1_col4 = eval.next_trace_mask();
        let offset2_col5 = eval.next_trace_mask();
        let dst_base_fp_col6 = eval.next_trace_mask();
        let op0_base_fp_col7 = eval.next_trace_mask();
        let op1_imm_col8 = eval.next_trace_mask();
        let op1_base_fp_col9 = eval.next_trace_mask();
        let res_add_col10 = eval.next_trace_mask();
        let ap_update_add_1_col11 = eval.next_trace_mask();
        let mem_dst_base_col12 = eval.next_trace_mask();
        let mem0_base_col13 = eval.next_trace_mask();
        let mem1_base_col14 = eval.next_trace_mask();
        let dst_id_col15 = eval.next_trace_mask();
        let dst_limb_0_col16 = eval.next_trace_mask();
        let dst_limb_1_col17 = eval.next_trace_mask();
        let dst_limb_2_col18 = eval.next_trace_mask();
        let dst_limb_3_col19 = eval.next_trace_mask();
        let dst_limb_4_col20 = eval.next_trace_mask();
        let dst_limb_5_col21 = eval.next_trace_mask();
        let dst_limb_6_col22 = eval.next_trace_mask();
        let dst_limb_7_col23 = eval.next_trace_mask();
        let dst_limb_8_col24 = eval.next_trace_mask();
        let dst_limb_9_col25 = eval.next_trace_mask();
        let dst_limb_10_col26 = eval.next_trace_mask();
        let dst_limb_11_col27 = eval.next_trace_mask();
        let dst_limb_12_col28 = eval.next_trace_mask();
        let dst_limb_13_col29 = eval.next_trace_mask();
        let dst_limb_14_col30 = eval.next_trace_mask();
        let dst_limb_15_col31 = eval.next_trace_mask();
        let dst_delta_ab_inv_col32 = eval.next_trace_mask();
        let dst_delta_cd_inv_col33 = eval.next_trace_mask();
        let op0_id_col34 = eval.next_trace_mask();
        let op0_limb_0_col35 = eval.next_trace_mask();
        let op0_limb_1_col36 = eval.next_trace_mask();
        let op0_limb_2_col37 = eval.next_trace_mask();
        let op0_limb_3_col38 = eval.next_trace_mask();
        let op0_limb_4_col39 = eval.next_trace_mask();
        let op0_limb_5_col40 = eval.next_trace_mask();
        let op0_limb_6_col41 = eval.next_trace_mask();
        let op0_limb_7_col42 = eval.next_trace_mask();
        let op0_limb_8_col43 = eval.next_trace_mask();
        let op0_limb_9_col44 = eval.next_trace_mask();
        let op0_limb_10_col45 = eval.next_trace_mask();
        let op0_limb_11_col46 = eval.next_trace_mask();
        let op0_limb_12_col47 = eval.next_trace_mask();
        let op0_limb_13_col48 = eval.next_trace_mask();
        let op0_limb_14_col49 = eval.next_trace_mask();
        let op0_limb_15_col50 = eval.next_trace_mask();
        let op0_delta_ab_inv_col51 = eval.next_trace_mask();
        let op0_delta_cd_inv_col52 = eval.next_trace_mask();
        let op1_id_col53 = eval.next_trace_mask();
        let op1_limb_0_col54 = eval.next_trace_mask();
        let op1_limb_1_col55 = eval.next_trace_mask();
        let op1_limb_2_col56 = eval.next_trace_mask();
        let op1_limb_3_col57 = eval.next_trace_mask();
        let op1_limb_4_col58 = eval.next_trace_mask();
        let op1_limb_5_col59 = eval.next_trace_mask();
        let op1_limb_6_col60 = eval.next_trace_mask();
        let op1_limb_7_col61 = eval.next_trace_mask();
        let op1_limb_8_col62 = eval.next_trace_mask();
        let op1_limb_9_col63 = eval.next_trace_mask();
        let op1_limb_10_col64 = eval.next_trace_mask();
        let op1_limb_11_col65 = eval.next_trace_mask();
        let op1_limb_12_col66 = eval.next_trace_mask();
        let op1_limb_13_col67 = eval.next_trace_mask();
        let op1_limb_14_col68 = eval.next_trace_mask();
        let op1_limb_15_col69 = eval.next_trace_mask();
        let op1_delta_ab_inv_col70 = eval.next_trace_mask();
        let op1_delta_cd_inv_col71 = eval.next_trace_mask();
        let enabler = eval.next_trace_mask();

        eval.add_constraint(enabler.clone() * enabler.clone() - enabler.clone());

        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [decode_instruction_3802d_output_tmp_fa85a_12_offset0, decode_instruction_3802d_output_tmp_fa85a_12_offset1, decode_instruction_3802d_output_tmp_fa85a_12_offset2, decode_instruction_3802d_output_tmp_fa85a_12_op1_base_ap, decode_instruction_3802d_output_tmp_fa85a_12_res_mul] =
            DecodeInstruction3802D::evaluate(
                [input_pc_col0.clone()],
                offset0_col3.clone(),
                offset1_col4.clone(),
                offset2_col5.clone(),
                dst_base_fp_col6.clone(),
                op0_base_fp_col7.clone(),
                op1_imm_col8.clone(),
                op1_base_fp_col9.clone(),
                res_add_col10.clone(),
                ap_update_add_1_col11.clone(),
                &self.verify_instruction_lookup_elements,
                &mut eval,
            );
        // Either flag op1_imm is off or offset2 is equal to 1.
        eval.add_constraint(
            (op1_imm_col8.clone()
                * (decode_instruction_3802d_output_tmp_fa85a_12_offset2.clone() - M31_1.clone())),
        );
        // mem_dst_base.
        eval.add_constraint(
            (mem_dst_base_col12.clone()
                - ((dst_base_fp_col6.clone() * input_fp_col2.clone())
                    + ((M31_1.clone() - dst_base_fp_col6.clone()) * input_ap_col1.clone()))),
        );
        // mem0_base.
        eval.add_constraint(
            (mem0_base_col13.clone()
                - ((op0_base_fp_col7.clone() * input_fp_col2.clone())
                    + ((M31_1.clone() - op0_base_fp_col7.clone()) * input_ap_col1.clone()))),
        );
        // mem1_base.
        eval.add_constraint(
            (mem1_base_col14.clone()
                - (((op1_base_fp_col9.clone() * input_fp_col2.clone())
                    + (decode_instruction_3802d_output_tmp_fa85a_12_op1_base_ap.clone()
                        * input_ap_col1.clone()))
                    + (op1_imm_col8.clone() * input_pc_col0.clone()))),
        );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [qm_31_read_reduced_output_tmp_fa85a_18_limb_0, qm_31_read_reduced_output_tmp_fa85a_18_limb_1, qm_31_read_reduced_output_tmp_fa85a_18_limb_2, qm_31_read_reduced_output_tmp_fa85a_18_limb_3] =
            Qm31ReadReduced::evaluate(
                [(mem_dst_base_col12.clone()
                    + decode_instruction_3802d_output_tmp_fa85a_12_offset0.clone())],
                dst_id_col15.clone(),
                dst_limb_0_col16.clone(),
                dst_limb_1_col17.clone(),
                dst_limb_2_col18.clone(),
                dst_limb_3_col19.clone(),
                dst_limb_4_col20.clone(),
                dst_limb_5_col21.clone(),
                dst_limb_6_col22.clone(),
                dst_limb_7_col23.clone(),
                dst_limb_8_col24.clone(),
                dst_limb_9_col25.clone(),
                dst_limb_10_col26.clone(),
                dst_limb_11_col27.clone(),
                dst_limb_12_col28.clone(),
                dst_limb_13_col29.clone(),
                dst_limb_14_col30.clone(),
                dst_limb_15_col31.clone(),
                dst_delta_ab_inv_col32.clone(),
                dst_delta_cd_inv_col33.clone(),
                &self.memory_address_to_id_lookup_elements,
                &self.memory_id_to_big_lookup_elements,
                &self.range_check_4_4_4_4_lookup_elements,
                &mut eval,
            );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [qm_31_read_reduced_output_tmp_fa85a_24_limb_0, qm_31_read_reduced_output_tmp_fa85a_24_limb_1, qm_31_read_reduced_output_tmp_fa85a_24_limb_2, qm_31_read_reduced_output_tmp_fa85a_24_limb_3] =
            Qm31ReadReduced::evaluate(
                [(mem0_base_col13.clone()
                    + decode_instruction_3802d_output_tmp_fa85a_12_offset1.clone())],
                op0_id_col34.clone(),
                op0_limb_0_col35.clone(),
                op0_limb_1_col36.clone(),
                op0_limb_2_col37.clone(),
                op0_limb_3_col38.clone(),
                op0_limb_4_col39.clone(),
                op0_limb_5_col40.clone(),
                op0_limb_6_col41.clone(),
                op0_limb_7_col42.clone(),
                op0_limb_8_col43.clone(),
                op0_limb_9_col44.clone(),
                op0_limb_10_col45.clone(),
                op0_limb_11_col46.clone(),
                op0_limb_12_col47.clone(),
                op0_limb_13_col48.clone(),
                op0_limb_14_col49.clone(),
                op0_limb_15_col50.clone(),
                op0_delta_ab_inv_col51.clone(),
                op0_delta_cd_inv_col52.clone(),
                &self.memory_address_to_id_lookup_elements,
                &self.memory_id_to_big_lookup_elements,
                &self.range_check_4_4_4_4_lookup_elements,
                &mut eval,
            );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [qm_31_read_reduced_output_tmp_fa85a_30_limb_0, qm_31_read_reduced_output_tmp_fa85a_30_limb_1, qm_31_read_reduced_output_tmp_fa85a_30_limb_2, qm_31_read_reduced_output_tmp_fa85a_30_limb_3] =
            Qm31ReadReduced::evaluate(
                [(mem1_base_col14.clone()
                    + decode_instruction_3802d_output_tmp_fa85a_12_offset2.clone())],
                op1_id_col53.clone(),
                op1_limb_0_col54.clone(),
                op1_limb_1_col55.clone(),
                op1_limb_2_col56.clone(),
                op1_limb_3_col57.clone(),
                op1_limb_4_col58.clone(),
                op1_limb_5_col59.clone(),
                op1_limb_6_col60.clone(),
                op1_limb_7_col61.clone(),
                op1_limb_8_col62.clone(),
                op1_limb_9_col63.clone(),
                op1_limb_10_col64.clone(),
                op1_limb_11_col65.clone(),
                op1_limb_12_col66.clone(),
                op1_limb_13_col67.clone(),
                op1_limb_14_col68.clone(),
                op1_limb_15_col69.clone(),
                op1_delta_ab_inv_col70.clone(),
                op1_delta_cd_inv_col71.clone(),
                &self.memory_address_to_id_lookup_elements,
                &self.memory_id_to_big_lookup_elements,
                &self.range_check_4_4_4_4_lookup_elements,
                &mut eval,
            );
        // dst equals (op0 * op1)*flag_res_mul + (op0 + op1)*(1-flag_res_mul).
        eval.add_constraint(
            ((qm_31_read_reduced_output_tmp_fa85a_18_limb_0.clone()
                - ((((((qm_31_read_reduced_output_tmp_fa85a_24_limb_0.clone()
                    * qm_31_read_reduced_output_tmp_fa85a_30_limb_0.clone())
                    - (qm_31_read_reduced_output_tmp_fa85a_24_limb_1.clone()
                        * qm_31_read_reduced_output_tmp_fa85a_30_limb_1.clone()))
                    + (M31_2.clone()
                        * ((qm_31_read_reduced_output_tmp_fa85a_24_limb_2.clone()
                            * qm_31_read_reduced_output_tmp_fa85a_30_limb_2.clone())
                            - (qm_31_read_reduced_output_tmp_fa85a_24_limb_3.clone()
                                * qm_31_read_reduced_output_tmp_fa85a_30_limb_3.clone()))))
                    - (qm_31_read_reduced_output_tmp_fa85a_24_limb_2.clone()
                        * qm_31_read_reduced_output_tmp_fa85a_30_limb_3.clone()))
                    - (qm_31_read_reduced_output_tmp_fa85a_24_limb_3.clone()
                        * qm_31_read_reduced_output_tmp_fa85a_30_limb_2.clone()))
                    * decode_instruction_3802d_output_tmp_fa85a_12_res_mul.clone()))
                - ((qm_31_read_reduced_output_tmp_fa85a_24_limb_0.clone()
                    + qm_31_read_reduced_output_tmp_fa85a_30_limb_0.clone())
                    * res_add_col10.clone())),
        );
        // dst equals (op0 * op1)*flag_res_mul + (op0 + op1)*(1-flag_res_mul).
        eval.add_constraint(
            ((qm_31_read_reduced_output_tmp_fa85a_18_limb_1.clone()
                - ((((((qm_31_read_reduced_output_tmp_fa85a_24_limb_0.clone()
                    * qm_31_read_reduced_output_tmp_fa85a_30_limb_1.clone())
                    + (qm_31_read_reduced_output_tmp_fa85a_24_limb_1.clone()
                        * qm_31_read_reduced_output_tmp_fa85a_30_limb_0.clone()))
                    + (M31_2.clone()
                        * ((qm_31_read_reduced_output_tmp_fa85a_24_limb_2.clone()
                            * qm_31_read_reduced_output_tmp_fa85a_30_limb_3.clone())
                            + (qm_31_read_reduced_output_tmp_fa85a_24_limb_3.clone()
                                * qm_31_read_reduced_output_tmp_fa85a_30_limb_2.clone()))))
                    + (qm_31_read_reduced_output_tmp_fa85a_24_limb_2.clone()
                        * qm_31_read_reduced_output_tmp_fa85a_30_limb_2.clone()))
                    - (qm_31_read_reduced_output_tmp_fa85a_24_limb_3.clone()
                        * qm_31_read_reduced_output_tmp_fa85a_30_limb_3.clone()))
                    * decode_instruction_3802d_output_tmp_fa85a_12_res_mul.clone()))
                - ((qm_31_read_reduced_output_tmp_fa85a_24_limb_1.clone()
                    + qm_31_read_reduced_output_tmp_fa85a_30_limb_1.clone())
                    * res_add_col10.clone())),
        );
        // dst equals (op0 * op1)*flag_res_mul + (op0 + op1)*(1-flag_res_mul).
        eval.add_constraint(
            ((qm_31_read_reduced_output_tmp_fa85a_18_limb_2.clone()
                - (((((qm_31_read_reduced_output_tmp_fa85a_24_limb_0.clone()
                    * qm_31_read_reduced_output_tmp_fa85a_30_limb_2.clone())
                    - (qm_31_read_reduced_output_tmp_fa85a_24_limb_1.clone()
                        * qm_31_read_reduced_output_tmp_fa85a_30_limb_3.clone()))
                    + (qm_31_read_reduced_output_tmp_fa85a_24_limb_2.clone()
                        * qm_31_read_reduced_output_tmp_fa85a_30_limb_0.clone()))
                    - (qm_31_read_reduced_output_tmp_fa85a_24_limb_3.clone()
                        * qm_31_read_reduced_output_tmp_fa85a_30_limb_1.clone()))
                    * decode_instruction_3802d_output_tmp_fa85a_12_res_mul.clone()))
                - ((qm_31_read_reduced_output_tmp_fa85a_24_limb_2.clone()
                    + qm_31_read_reduced_output_tmp_fa85a_30_limb_2.clone())
                    * res_add_col10.clone())),
        );
        // dst equals (op0 * op1)*flag_res_mul + (op0 + op1)*(1-flag_res_mul).
        eval.add_constraint(
            ((qm_31_read_reduced_output_tmp_fa85a_18_limb_3.clone()
                - (((((qm_31_read_reduced_output_tmp_fa85a_24_limb_0.clone()
                    * qm_31_read_reduced_output_tmp_fa85a_30_limb_3.clone())
                    + (qm_31_read_reduced_output_tmp_fa85a_24_limb_1.clone()
                        * qm_31_read_reduced_output_tmp_fa85a_30_limb_2.clone()))
                    + (qm_31_read_reduced_output_tmp_fa85a_24_limb_2.clone()
                        * qm_31_read_reduced_output_tmp_fa85a_30_limb_1.clone()))
                    + (qm_31_read_reduced_output_tmp_fa85a_24_limb_3.clone()
                        * qm_31_read_reduced_output_tmp_fa85a_30_limb_0.clone()))
                    * decode_instruction_3802d_output_tmp_fa85a_12_res_mul.clone()))
                - ((qm_31_read_reduced_output_tmp_fa85a_24_limb_3.clone()
                    + qm_31_read_reduced_output_tmp_fa85a_30_limb_3.clone())
                    * res_add_col10.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.opcodes_lookup_elements,
            E::EF::from(enabler.clone()),
            &[
                input_pc_col0.clone(),
                input_ap_col1.clone(),
                input_fp_col2.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.opcodes_lookup_elements,
            -E::EF::from(enabler.clone()),
            &[
                ((input_pc_col0.clone() + M31_1.clone()) + op1_imm_col8.clone()),
                (input_ap_col1.clone() + ap_update_add_1_col11.clone()),
                input_fp_col2.clone(),
            ],
        ));

        eval.finalize_logup_in_pairs();
        eval
    }
}

#[cfg(test)]
mod tests {
    use num_traits::Zero;
    use rand::rngs::SmallRng;
    use rand::{Rng, SeedableRng};
    use stwo::core::fields::qm31::QM31;
    use stwo_constraint_framework::expr::ExprEvaluator;

    use super::*;
    use crate::components::constraints_regression_test_values::QM_31_ADD_MUL_OPCODE;

    #[test]
    fn qm_31_add_mul_opcode_constraints_regression() {
        let mut rng = SmallRng::seed_from_u64(0);
        let eval = Eval {
            claim: Claim { log_size: 4 },
            verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
            memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
            memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
            range_check_4_4_4_4_lookup_elements: relations::RangeCheck_4_4_4_4::dummy(),
            opcodes_lookup_elements: relations::Opcodes::dummy(),
        };
        let expr_eval = eval.evaluate(ExprEvaluator::new());
        let assignment = expr_eval.random_assignment();

        let mut sum = QM31::zero();
        for c in expr_eval.constraints {
            sum += c.assign(&assignment) * rng.gen::<QM31>();
        }

        assert_eq!(sum, QM_31_ADD_MUL_OPCODE);
    }
}
