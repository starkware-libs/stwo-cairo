use crate::components::prelude::*;
use crate::components::subroutines::decode_instruction_15a61::DecodeInstruction15A61;
use crate::components::subroutines::read_positive_num_bits_27::ReadPositiveNumBits27;

pub const N_TRACE_COLUMNS: usize = 12;
pub const RELATION_USES_PER_ROW: [RelationUse; 4] = [
    RelationUse {
        relation_id: "MemoryAddressToId",
        uses: 2,
    },
    RelationUse {
        relation_id: "MemoryIdToBig",
        uses: 2,
    },
    RelationUse {
        relation_id: "Opcodes",
        uses: 1,
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
    pub opcodes_lookup_elements: relations::Opcodes,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct Claim {
    pub log_size: u32,
}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let trace_log_sizes = vec![self.log_size; N_TRACE_COLUMNS];
        let interaction_log_sizes = vec![self.log_size; SECURE_EXTENSION_DEGREE * 4];
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
        let M31_1 = E::F::from(M31::from(1));
        let M31_2 = E::F::from(M31::from(2));
        let M31_262144 = E::F::from(M31::from(262144));
        let M31_512 = E::F::from(M31::from(512));
        let input_pc_col0 = eval.next_trace_mask();
        let input_ap_col1 = eval.next_trace_mask();
        let input_fp_col2 = eval.next_trace_mask();
        let next_pc_id_col3 = eval.next_trace_mask();
        let next_pc_limb_0_col4 = eval.next_trace_mask();
        let next_pc_limb_1_col5 = eval.next_trace_mask();
        let next_pc_limb_2_col6 = eval.next_trace_mask();
        let next_fp_id_col7 = eval.next_trace_mask();
        let next_fp_limb_0_col8 = eval.next_trace_mask();
        let next_fp_limb_1_col9 = eval.next_trace_mask();
        let next_fp_limb_2_col10 = eval.next_trace_mask();
        let enabler = eval.next_trace_mask();

        eval.add_constraint(enabler.clone() * enabler.clone() - enabler.clone());

        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [decode_instruction_15a61_output_tmp_e23a5_2_offset0, decode_instruction_15a61_output_tmp_e23a5_2_offset1, decode_instruction_15a61_output_tmp_e23a5_2_offset2, decode_instruction_15a61_output_tmp_e23a5_2_dst_base_fp, decode_instruction_15a61_output_tmp_e23a5_2_op0_base_fp, decode_instruction_15a61_output_tmp_e23a5_2_op1_imm, decode_instruction_15a61_output_tmp_e23a5_2_op1_base_fp, decode_instruction_15a61_output_tmp_e23a5_2_op1_base_ap, decode_instruction_15a61_output_tmp_e23a5_2_res_add, decode_instruction_15a61_output_tmp_e23a5_2_res_mul, decode_instruction_15a61_output_tmp_e23a5_2_pc_update_jump, decode_instruction_15a61_output_tmp_e23a5_2_pc_update_jump_rel, decode_instruction_15a61_output_tmp_e23a5_2_pc_update_jnz, decode_instruction_15a61_output_tmp_e23a5_2_ap_update_add, decode_instruction_15a61_output_tmp_e23a5_2_ap_update_add_1, decode_instruction_15a61_output_tmp_e23a5_2_opcode_call, decode_instruction_15a61_output_tmp_e23a5_2_opcode_ret, decode_instruction_15a61_output_tmp_e23a5_2_opcode_assert_eq, decode_instruction_15a61_output_tmp_e23a5_2_opcode_extension] =
            DecodeInstruction15A61::evaluate(
                input_pc_col0.clone(),
                &self.verify_instruction_lookup_elements,
                &mut eval,
            );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [read_positive_num_bits_27_output_tmp_e23a5_5_limb_0, read_positive_num_bits_27_output_tmp_e23a5_5_limb_1, read_positive_num_bits_27_output_tmp_e23a5_5_limb_2, read_positive_num_bits_27_output_tmp_e23a5_5_limb_3, read_positive_num_bits_27_output_tmp_e23a5_5_limb_4, read_positive_num_bits_27_output_tmp_e23a5_5_limb_5, read_positive_num_bits_27_output_tmp_e23a5_5_limb_6, read_positive_num_bits_27_output_tmp_e23a5_5_limb_7, read_positive_num_bits_27_output_tmp_e23a5_5_limb_8, read_positive_num_bits_27_output_tmp_e23a5_5_limb_9, read_positive_num_bits_27_output_tmp_e23a5_5_limb_10, read_positive_num_bits_27_output_tmp_e23a5_5_limb_11, read_positive_num_bits_27_output_tmp_e23a5_5_limb_12, read_positive_num_bits_27_output_tmp_e23a5_5_limb_13, read_positive_num_bits_27_output_tmp_e23a5_5_limb_14, read_positive_num_bits_27_output_tmp_e23a5_5_limb_15, read_positive_num_bits_27_output_tmp_e23a5_5_limb_16, read_positive_num_bits_27_output_tmp_e23a5_5_limb_17, read_positive_num_bits_27_output_tmp_e23a5_5_limb_18, read_positive_num_bits_27_output_tmp_e23a5_5_limb_19, read_positive_num_bits_27_output_tmp_e23a5_5_limb_20, read_positive_num_bits_27_output_tmp_e23a5_5_limb_21, read_positive_num_bits_27_output_tmp_e23a5_5_limb_22, read_positive_num_bits_27_output_tmp_e23a5_5_limb_23, read_positive_num_bits_27_output_tmp_e23a5_5_limb_24, read_positive_num_bits_27_output_tmp_e23a5_5_limb_25, read_positive_num_bits_27_output_tmp_e23a5_5_limb_26, read_positive_num_bits_27_output_tmp_e23a5_5_limb_27, read_positive_num_bits_27_output_tmp_e23a5_5_limb_28] =
            ReadPositiveNumBits27::evaluate(
                (input_fp_col2.clone() - M31_1.clone()),
                next_pc_id_col3.clone(),
                next_pc_limb_0_col4.clone(),
                next_pc_limb_1_col5.clone(),
                next_pc_limb_2_col6.clone(),
                &self.memory_address_to_id_lookup_elements,
                &self.memory_id_to_big_lookup_elements,
                &mut eval,
            );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [read_positive_num_bits_27_output_tmp_e23a5_8_limb_0, read_positive_num_bits_27_output_tmp_e23a5_8_limb_1, read_positive_num_bits_27_output_tmp_e23a5_8_limb_2, read_positive_num_bits_27_output_tmp_e23a5_8_limb_3, read_positive_num_bits_27_output_tmp_e23a5_8_limb_4, read_positive_num_bits_27_output_tmp_e23a5_8_limb_5, read_positive_num_bits_27_output_tmp_e23a5_8_limb_6, read_positive_num_bits_27_output_tmp_e23a5_8_limb_7, read_positive_num_bits_27_output_tmp_e23a5_8_limb_8, read_positive_num_bits_27_output_tmp_e23a5_8_limb_9, read_positive_num_bits_27_output_tmp_e23a5_8_limb_10, read_positive_num_bits_27_output_tmp_e23a5_8_limb_11, read_positive_num_bits_27_output_tmp_e23a5_8_limb_12, read_positive_num_bits_27_output_tmp_e23a5_8_limb_13, read_positive_num_bits_27_output_tmp_e23a5_8_limb_14, read_positive_num_bits_27_output_tmp_e23a5_8_limb_15, read_positive_num_bits_27_output_tmp_e23a5_8_limb_16, read_positive_num_bits_27_output_tmp_e23a5_8_limb_17, read_positive_num_bits_27_output_tmp_e23a5_8_limb_18, read_positive_num_bits_27_output_tmp_e23a5_8_limb_19, read_positive_num_bits_27_output_tmp_e23a5_8_limb_20, read_positive_num_bits_27_output_tmp_e23a5_8_limb_21, read_positive_num_bits_27_output_tmp_e23a5_8_limb_22, read_positive_num_bits_27_output_tmp_e23a5_8_limb_23, read_positive_num_bits_27_output_tmp_e23a5_8_limb_24, read_positive_num_bits_27_output_tmp_e23a5_8_limb_25, read_positive_num_bits_27_output_tmp_e23a5_8_limb_26, read_positive_num_bits_27_output_tmp_e23a5_8_limb_27, read_positive_num_bits_27_output_tmp_e23a5_8_limb_28] =
            ReadPositiveNumBits27::evaluate(
                (input_fp_col2.clone() - M31_2.clone()),
                next_fp_id_col7.clone(),
                next_fp_limb_0_col8.clone(),
                next_fp_limb_1_col9.clone(),
                next_fp_limb_2_col10.clone(),
                &self.memory_address_to_id_lookup_elements,
                &self.memory_id_to_big_lookup_elements,
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
                ((next_pc_limb_0_col4.clone() + (next_pc_limb_1_col5.clone() * M31_512.clone()))
                    + (next_pc_limb_2_col6.clone() * M31_262144.clone())),
                input_ap_col1.clone(),
                ((next_fp_limb_0_col8.clone() + (next_fp_limb_1_col9.clone() * M31_512.clone()))
                    + (next_fp_limb_2_col10.clone() * M31_262144.clone())),
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
    use crate::components::constraints_regression_test_values::RET_OPCODE;

    #[test]
    fn ret_opcode_constraints_regression() {
        let mut rng = SmallRng::seed_from_u64(0);
        let eval = Eval {
            claim: Claim { log_size: 4 },
            verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
            memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
            memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
            opcodes_lookup_elements: relations::Opcodes::dummy(),
        };
        let expr_eval = eval.evaluate(ExprEvaluator::new());
        let assignment = expr_eval.random_assignment();

        let mut sum = QM31::zero();
        for c in expr_eval.constraints {
            sum += c.assign(&assignment) * rng.gen::<QM31>();
        }

        assert_eq!(sum, RET_OPCODE);
    }
}
