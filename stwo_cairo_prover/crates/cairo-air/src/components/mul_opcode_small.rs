use crate::components::prelude::*;
use crate::components::subroutines::decode_instruction_4b8cf::DecodeInstruction4B8Cf;
use crate::components::subroutines::read_positive_num_bits_36::ReadPositiveNumBits36;
use crate::components::subroutines::read_positive_num_bits_72::ReadPositiveNumBits72;
use crate::components::subroutines::verify_mul_small::VerifyMulSmall;

pub const N_TRACE_COLUMNS: usize = 37;
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
        relation_id: "RangeCheck_11",
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
    pub range_check_11_lookup_elements: relations::RangeCheck_11,
    pub opcodes_lookup_elements: relations::Opcodes,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
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

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
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
        let M31_0 = E::F::from(M31::from(0));
        let M31_1 = E::F::from(M31::from(1));
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
        let ap_update_add_1_col10 = eval.next_trace_mask();
        let mem_dst_base_col11 = eval.next_trace_mask();
        let mem0_base_col12 = eval.next_trace_mask();
        let mem1_base_col13 = eval.next_trace_mask();
        let dst_id_col14 = eval.next_trace_mask();
        let dst_limb_0_col15 = eval.next_trace_mask();
        let dst_limb_1_col16 = eval.next_trace_mask();
        let dst_limb_2_col17 = eval.next_trace_mask();
        let dst_limb_3_col18 = eval.next_trace_mask();
        let dst_limb_4_col19 = eval.next_trace_mask();
        let dst_limb_5_col20 = eval.next_trace_mask();
        let dst_limb_6_col21 = eval.next_trace_mask();
        let dst_limb_7_col22 = eval.next_trace_mask();
        let op0_id_col23 = eval.next_trace_mask();
        let op0_limb_0_col24 = eval.next_trace_mask();
        let op0_limb_1_col25 = eval.next_trace_mask();
        let op0_limb_2_col26 = eval.next_trace_mask();
        let op0_limb_3_col27 = eval.next_trace_mask();
        let op1_id_col28 = eval.next_trace_mask();
        let op1_limb_0_col29 = eval.next_trace_mask();
        let op1_limb_1_col30 = eval.next_trace_mask();
        let op1_limb_2_col31 = eval.next_trace_mask();
        let op1_limb_3_col32 = eval.next_trace_mask();
        let carry_1_col33 = eval.next_trace_mask();
        let carry_3_col34 = eval.next_trace_mask();
        let carry_5_col35 = eval.next_trace_mask();
        let enabler = eval.next_trace_mask();

        eval.add_constraint(enabler.clone() * enabler.clone() - enabler.clone());

        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [decode_instruction_4b8cf_output_tmp_9d1ad_10_offset0, decode_instruction_4b8cf_output_tmp_9d1ad_10_offset1, decode_instruction_4b8cf_output_tmp_9d1ad_10_offset2, decode_instruction_4b8cf_output_tmp_9d1ad_10_dst_base_fp, decode_instruction_4b8cf_output_tmp_9d1ad_10_op0_base_fp, decode_instruction_4b8cf_output_tmp_9d1ad_10_op1_imm, decode_instruction_4b8cf_output_tmp_9d1ad_10_op1_base_fp, decode_instruction_4b8cf_output_tmp_9d1ad_10_op1_base_ap, decode_instruction_4b8cf_output_tmp_9d1ad_10_res_add, decode_instruction_4b8cf_output_tmp_9d1ad_10_res_mul, decode_instruction_4b8cf_output_tmp_9d1ad_10_pc_update_jump, decode_instruction_4b8cf_output_tmp_9d1ad_10_pc_update_jump_rel, decode_instruction_4b8cf_output_tmp_9d1ad_10_pc_update_jnz, decode_instruction_4b8cf_output_tmp_9d1ad_10_ap_update_add, decode_instruction_4b8cf_output_tmp_9d1ad_10_ap_update_add_1, decode_instruction_4b8cf_output_tmp_9d1ad_10_opcode_call, decode_instruction_4b8cf_output_tmp_9d1ad_10_opcode_ret, decode_instruction_4b8cf_output_tmp_9d1ad_10_opcode_assert_eq, decode_instruction_4b8cf_output_tmp_9d1ad_10_opcode_extension] =
            DecodeInstruction4B8Cf::evaluate(
                input_pc_col0.clone(),
                offset0_col3.clone(),
                offset1_col4.clone(),
                offset2_col5.clone(),
                dst_base_fp_col6.clone(),
                op0_base_fp_col7.clone(),
                op1_imm_col8.clone(),
                op1_base_fp_col9.clone(),
                ap_update_add_1_col10.clone(),
                &self.verify_instruction_lookup_elements,
                &mut eval,
            );
        // if imm then offset2 is 1.
        eval.add_constraint(
            (op1_imm_col8.clone()
                * (M31_1.clone() - decode_instruction_4b8cf_output_tmp_9d1ad_10_offset2.clone())),
        );
        // mem_dst_base.
        eval.add_constraint(
            (mem_dst_base_col11.clone()
                - ((dst_base_fp_col6.clone() * input_fp_col2.clone())
                    + ((M31_1.clone() - dst_base_fp_col6.clone()) * input_ap_col1.clone()))),
        );
        // mem0_base.
        eval.add_constraint(
            (mem0_base_col12.clone()
                - ((op0_base_fp_col7.clone() * input_fp_col2.clone())
                    + ((M31_1.clone() - op0_base_fp_col7.clone()) * input_ap_col1.clone()))),
        );
        // mem1_base.
        eval.add_constraint(
            (mem1_base_col13.clone()
                - (((op1_imm_col8.clone() * input_pc_col0.clone())
                    + (op1_base_fp_col9.clone() * input_fp_col2.clone()))
                    + (decode_instruction_4b8cf_output_tmp_9d1ad_10_op1_base_ap.clone()
                        * input_ap_col1.clone()))),
        );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [read_positive_num_bits_72_output_tmp_9d1ad_13_limb_0, read_positive_num_bits_72_output_tmp_9d1ad_13_limb_1, read_positive_num_bits_72_output_tmp_9d1ad_13_limb_2, read_positive_num_bits_72_output_tmp_9d1ad_13_limb_3, read_positive_num_bits_72_output_tmp_9d1ad_13_limb_4, read_positive_num_bits_72_output_tmp_9d1ad_13_limb_5, read_positive_num_bits_72_output_tmp_9d1ad_13_limb_6, read_positive_num_bits_72_output_tmp_9d1ad_13_limb_7, read_positive_num_bits_72_output_tmp_9d1ad_13_limb_8, read_positive_num_bits_72_output_tmp_9d1ad_13_limb_9, read_positive_num_bits_72_output_tmp_9d1ad_13_limb_10, read_positive_num_bits_72_output_tmp_9d1ad_13_limb_11, read_positive_num_bits_72_output_tmp_9d1ad_13_limb_12, read_positive_num_bits_72_output_tmp_9d1ad_13_limb_13, read_positive_num_bits_72_output_tmp_9d1ad_13_limb_14, read_positive_num_bits_72_output_tmp_9d1ad_13_limb_15, read_positive_num_bits_72_output_tmp_9d1ad_13_limb_16, read_positive_num_bits_72_output_tmp_9d1ad_13_limb_17, read_positive_num_bits_72_output_tmp_9d1ad_13_limb_18, read_positive_num_bits_72_output_tmp_9d1ad_13_limb_19, read_positive_num_bits_72_output_tmp_9d1ad_13_limb_20, read_positive_num_bits_72_output_tmp_9d1ad_13_limb_21, read_positive_num_bits_72_output_tmp_9d1ad_13_limb_22, read_positive_num_bits_72_output_tmp_9d1ad_13_limb_23, read_positive_num_bits_72_output_tmp_9d1ad_13_limb_24, read_positive_num_bits_72_output_tmp_9d1ad_13_limb_25, read_positive_num_bits_72_output_tmp_9d1ad_13_limb_26, read_positive_num_bits_72_output_tmp_9d1ad_13_limb_27, read_positive_num_bits_72_output_tmp_9d1ad_13_limb_28] =
            ReadPositiveNumBits72::evaluate(
                (mem_dst_base_col11.clone()
                    + decode_instruction_4b8cf_output_tmp_9d1ad_10_offset0.clone()),
                dst_id_col14.clone(),
                dst_limb_0_col15.clone(),
                dst_limb_1_col16.clone(),
                dst_limb_2_col17.clone(),
                dst_limb_3_col18.clone(),
                dst_limb_4_col19.clone(),
                dst_limb_5_col20.clone(),
                dst_limb_6_col21.clone(),
                dst_limb_7_col22.clone(),
                &self.memory_address_to_id_lookup_elements,
                &self.memory_id_to_big_lookup_elements,
                &mut eval,
            );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [read_positive_num_bits_36_output_tmp_9d1ad_16_limb_0, read_positive_num_bits_36_output_tmp_9d1ad_16_limb_1, read_positive_num_bits_36_output_tmp_9d1ad_16_limb_2, read_positive_num_bits_36_output_tmp_9d1ad_16_limb_3, read_positive_num_bits_36_output_tmp_9d1ad_16_limb_4, read_positive_num_bits_36_output_tmp_9d1ad_16_limb_5, read_positive_num_bits_36_output_tmp_9d1ad_16_limb_6, read_positive_num_bits_36_output_tmp_9d1ad_16_limb_7, read_positive_num_bits_36_output_tmp_9d1ad_16_limb_8, read_positive_num_bits_36_output_tmp_9d1ad_16_limb_9, read_positive_num_bits_36_output_tmp_9d1ad_16_limb_10, read_positive_num_bits_36_output_tmp_9d1ad_16_limb_11, read_positive_num_bits_36_output_tmp_9d1ad_16_limb_12, read_positive_num_bits_36_output_tmp_9d1ad_16_limb_13, read_positive_num_bits_36_output_tmp_9d1ad_16_limb_14, read_positive_num_bits_36_output_tmp_9d1ad_16_limb_15, read_positive_num_bits_36_output_tmp_9d1ad_16_limb_16, read_positive_num_bits_36_output_tmp_9d1ad_16_limb_17, read_positive_num_bits_36_output_tmp_9d1ad_16_limb_18, read_positive_num_bits_36_output_tmp_9d1ad_16_limb_19, read_positive_num_bits_36_output_tmp_9d1ad_16_limb_20, read_positive_num_bits_36_output_tmp_9d1ad_16_limb_21, read_positive_num_bits_36_output_tmp_9d1ad_16_limb_22, read_positive_num_bits_36_output_tmp_9d1ad_16_limb_23, read_positive_num_bits_36_output_tmp_9d1ad_16_limb_24, read_positive_num_bits_36_output_tmp_9d1ad_16_limb_25, read_positive_num_bits_36_output_tmp_9d1ad_16_limb_26, read_positive_num_bits_36_output_tmp_9d1ad_16_limb_27, read_positive_num_bits_36_output_tmp_9d1ad_16_limb_28] =
            ReadPositiveNumBits36::evaluate(
                (mem0_base_col12.clone()
                    + decode_instruction_4b8cf_output_tmp_9d1ad_10_offset1.clone()),
                op0_id_col23.clone(),
                op0_limb_0_col24.clone(),
                op0_limb_1_col25.clone(),
                op0_limb_2_col26.clone(),
                op0_limb_3_col27.clone(),
                &self.memory_address_to_id_lookup_elements,
                &self.memory_id_to_big_lookup_elements,
                &mut eval,
            );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [read_positive_num_bits_36_output_tmp_9d1ad_19_limb_0, read_positive_num_bits_36_output_tmp_9d1ad_19_limb_1, read_positive_num_bits_36_output_tmp_9d1ad_19_limb_2, read_positive_num_bits_36_output_tmp_9d1ad_19_limb_3, read_positive_num_bits_36_output_tmp_9d1ad_19_limb_4, read_positive_num_bits_36_output_tmp_9d1ad_19_limb_5, read_positive_num_bits_36_output_tmp_9d1ad_19_limb_6, read_positive_num_bits_36_output_tmp_9d1ad_19_limb_7, read_positive_num_bits_36_output_tmp_9d1ad_19_limb_8, read_positive_num_bits_36_output_tmp_9d1ad_19_limb_9, read_positive_num_bits_36_output_tmp_9d1ad_19_limb_10, read_positive_num_bits_36_output_tmp_9d1ad_19_limb_11, read_positive_num_bits_36_output_tmp_9d1ad_19_limb_12, read_positive_num_bits_36_output_tmp_9d1ad_19_limb_13, read_positive_num_bits_36_output_tmp_9d1ad_19_limb_14, read_positive_num_bits_36_output_tmp_9d1ad_19_limb_15, read_positive_num_bits_36_output_tmp_9d1ad_19_limb_16, read_positive_num_bits_36_output_tmp_9d1ad_19_limb_17, read_positive_num_bits_36_output_tmp_9d1ad_19_limb_18, read_positive_num_bits_36_output_tmp_9d1ad_19_limb_19, read_positive_num_bits_36_output_tmp_9d1ad_19_limb_20, read_positive_num_bits_36_output_tmp_9d1ad_19_limb_21, read_positive_num_bits_36_output_tmp_9d1ad_19_limb_22, read_positive_num_bits_36_output_tmp_9d1ad_19_limb_23, read_positive_num_bits_36_output_tmp_9d1ad_19_limb_24, read_positive_num_bits_36_output_tmp_9d1ad_19_limb_25, read_positive_num_bits_36_output_tmp_9d1ad_19_limb_26, read_positive_num_bits_36_output_tmp_9d1ad_19_limb_27, read_positive_num_bits_36_output_tmp_9d1ad_19_limb_28] =
            ReadPositiveNumBits36::evaluate(
                (mem1_base_col13.clone()
                    + decode_instruction_4b8cf_output_tmp_9d1ad_10_offset2.clone()),
                op1_id_col28.clone(),
                op1_limb_0_col29.clone(),
                op1_limb_1_col30.clone(),
                op1_limb_2_col31.clone(),
                op1_limb_3_col32.clone(),
                &self.memory_address_to_id_lookup_elements,
                &self.memory_id_to_big_lookup_elements,
                &mut eval,
            );
        VerifyMulSmall::evaluate(
            [
                op0_limb_0_col24.clone(),
                op0_limb_1_col25.clone(),
                op0_limb_2_col26.clone(),
                op0_limb_3_col27.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                op1_limb_0_col29.clone(),
                op1_limb_1_col30.clone(),
                op1_limb_2_col31.clone(),
                op1_limb_3_col32.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                dst_limb_0_col15.clone(),
                dst_limb_1_col16.clone(),
                dst_limb_2_col17.clone(),
                dst_limb_3_col18.clone(),
                dst_limb_4_col19.clone(),
                dst_limb_5_col20.clone(),
                dst_limb_6_col21.clone(),
                dst_limb_7_col22.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
            ],
            carry_1_col33.clone(),
            carry_3_col34.clone(),
            carry_5_col35.clone(),
            &self.range_check_11_lookup_elements,
            &mut eval,
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
                (input_ap_col1.clone() + ap_update_add_1_col10.clone()),
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
    use stwo_prover::constraint_framework::expr::ExprEvaluator;
    use stwo_prover::core::fields::qm31::QM31;

    use super::*;
    use crate::components::constraints_regression_test_values::MUL_OPCODE_SMALL;

    #[test]
    fn mul_opcode_small_constraints_regression() {
        let mut rng = SmallRng::seed_from_u64(0);
        let eval = Eval {
            claim: Claim { log_size: 4 },
            verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
            memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
            memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
            range_check_11_lookup_elements: relations::RangeCheck_11::dummy(),
            opcodes_lookup_elements: relations::Opcodes::dummy(),
        };
        let expr_eval = eval.evaluate(ExprEvaluator::new());
        let assignment = expr_eval.random_assignment();

        let mut sum = QM31::zero();
        for c in expr_eval.constraints {
            sum += c.assign(&assignment) * rng.gen::<QM31>();
        }

        assert_eq!(sum, MUL_OPCODE_SMALL);
    }
}
