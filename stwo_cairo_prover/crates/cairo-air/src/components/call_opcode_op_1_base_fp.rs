use crate::components::prelude::*;

pub const N_TRACE_COLUMNS: usize = 17;

pub struct Eval {
    pub claim: Claim,
    pub memory_address_to_id_lookup_elements: relations::MemoryAddressToId,
    pub memory_id_to_big_lookup_elements: relations::MemoryIdToBig,
    pub opcodes_lookup_elements: relations::Opcodes,
    pub verify_instruction_lookup_elements: relations::VerifyInstruction,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct Claim {
    pub log_size: u32,
}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let trace_log_sizes = vec![self.log_size; N_TRACE_COLUMNS];
        let interaction_log_sizes = vec![self.log_size; SECURE_EXTENSION_DEGREE * 5];
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
        let M31_32768 = E::F::from(M31::from(32768));
        let M31_32769 = E::F::from(M31::from(32769));
        let M31_512 = E::F::from(M31::from(512));
        let M31_64 = E::F::from(M31::from(64));
        let M31_66 = E::F::from(M31::from(66));
        let input_pc_col0 = eval.next_trace_mask();
        let input_ap_col1 = eval.next_trace_mask();
        let input_fp_col2 = eval.next_trace_mask();
        let offset2_col3 = eval.next_trace_mask();
        let stored_fp_id_col4 = eval.next_trace_mask();
        let stored_fp_limb_0_col5 = eval.next_trace_mask();
        let stored_fp_limb_1_col6 = eval.next_trace_mask();
        let stored_fp_limb_2_col7 = eval.next_trace_mask();
        let stored_ret_pc_id_col8 = eval.next_trace_mask();
        let stored_ret_pc_limb_0_col9 = eval.next_trace_mask();
        let stored_ret_pc_limb_1_col10 = eval.next_trace_mask();
        let stored_ret_pc_limb_2_col11 = eval.next_trace_mask();
        let next_pc_id_col12 = eval.next_trace_mask();
        let next_pc_limb_0_col13 = eval.next_trace_mask();
        let next_pc_limb_1_col14 = eval.next_trace_mask();
        let next_pc_limb_2_col15 = eval.next_trace_mask();
        let enabler = eval.next_trace_mask();

        eval.add_constraint(enabler.clone() * enabler.clone() - enabler.clone());

        // Decode Instruction.

        eval.add_to_relation(RelationEntry::new(
            &self.verify_instruction_lookup_elements,
            E::EF::one(),
            &[
                input_pc_col0.clone(),
                M31_32768.clone(),
                M31_32769.clone(),
                offset2_col3.clone(),
                M31_64.clone(),
                M31_66.clone(),
            ],
        ));

        let decode_instruction_6e0d65e5157a9970_output_tmp_82665_3_limb_2 =
            eval.add_intermediate((offset2_col3.clone() - M31_32768.clone()));

        // Read Positive Num Bits 27.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[input_ap_col1.clone(), stored_fp_id_col4.clone()],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                stored_fp_id_col4.clone(),
                stored_fp_limb_0_col5.clone(),
                stored_fp_limb_1_col6.clone(),
                stored_fp_limb_2_col7.clone(),
            ],
        ));

        //[ap] = fp.
        eval.add_constraint(
            (((stored_fp_limb_0_col5.clone() + (stored_fp_limb_1_col6.clone() * M31_512.clone()))
                + (stored_fp_limb_2_col7.clone() * M31_262144.clone()))
                - input_fp_col2.clone()),
        );

        // Read Positive Num Bits 27.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (input_ap_col1.clone() + M31_1.clone()),
                stored_ret_pc_id_col8.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                stored_ret_pc_id_col8.clone(),
                stored_ret_pc_limb_0_col9.clone(),
                stored_ret_pc_limb_1_col10.clone(),
                stored_ret_pc_limb_2_col11.clone(),
            ],
        ));

        //[ap+1] = return_pc.
        eval.add_constraint(
            (((stored_ret_pc_limb_0_col9.clone()
                + (stored_ret_pc_limb_1_col10.clone() * M31_512.clone()))
                + (stored_ret_pc_limb_2_col11.clone() * M31_262144.clone()))
                - (input_pc_col0.clone() + M31_1.clone())),
        );

        // Read Positive Num Bits 27.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (input_fp_col2.clone()
                    + decode_instruction_6e0d65e5157a9970_output_tmp_82665_3_limb_2.clone()),
                next_pc_id_col12.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                next_pc_id_col12.clone(),
                next_pc_limb_0_col13.clone(),
                next_pc_limb_1_col14.clone(),
                next_pc_limb_2_col15.clone(),
            ],
        ));

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
                ((next_pc_limb_0_col13.clone() + (next_pc_limb_1_col14.clone() * M31_512.clone()))
                    + (next_pc_limb_2_col15.clone() * M31_262144.clone())),
                (input_ap_col1.clone() + M31_2.clone()),
                (input_ap_col1.clone() + M31_2.clone()),
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
    use crate::components::constraints_regression_test_values::CALL_OPCODE_OP_1_BASE_FP;

    #[test]
    fn call_opcode_op_1_base_fp_constraints_regression() {
        let mut rng = SmallRng::seed_from_u64(0);
        let eval = Eval {
            claim: Claim { log_size: 4 },
            memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
            memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
            opcodes_lookup_elements: relations::Opcodes::dummy(),
            verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
        };

        let expr_eval = eval.evaluate(ExprEvaluator::new());
        let assignment = expr_eval.random_assignment();

        let mut sum = QM31::zero();
        for c in expr_eval.constraints {
            sum += c.assign(&assignment) * rng.gen::<QM31>();
        }

        assert_eq!(sum, CALL_OPCODE_OP_1_BASE_FP);
    }
}
