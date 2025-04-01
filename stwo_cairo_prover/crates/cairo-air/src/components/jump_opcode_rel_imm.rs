use crate::components::prelude::*;

pub const N_TRACE_COLUMNS: usize = 11;

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
        let interaction_log_sizes = vec![self.log_size; SECURE_EXTENSION_DEGREE * 3];
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
        let M31_134217728 = E::F::from(M31::from(134217728));
        let M31_136 = E::F::from(M31::from(136));
        let M31_256 = E::F::from(M31::from(256));
        let M31_262144 = E::F::from(M31::from(262144));
        let M31_32 = E::F::from(M31::from(32));
        let M31_32767 = E::F::from(M31::from(32767));
        let M31_32769 = E::F::from(M31::from(32769));
        let M31_4 = E::F::from(M31::from(4));
        let M31_511 = E::F::from(M31::from(511));
        let M31_512 = E::F::from(M31::from(512));
        let M31_56 = E::F::from(M31::from(56));
        let input_pc_col0 = eval.next_trace_mask();
        let input_ap_col1 = eval.next_trace_mask();
        let input_fp_col2 = eval.next_trace_mask();
        let ap_update_add_1_col3 = eval.next_trace_mask();
        let next_pc_id_col4 = eval.next_trace_mask();
        let msb_col5 = eval.next_trace_mask();
        let mid_limbs_set_col6 = eval.next_trace_mask();
        let next_pc_limb_0_col7 = eval.next_trace_mask();
        let next_pc_limb_1_col8 = eval.next_trace_mask();
        let next_pc_limb_2_col9 = eval.next_trace_mask();
        let enabler = eval.next_trace_mask();

        eval.add_constraint(enabler.clone() * enabler.clone() - enabler.clone());

        // Decode Instruction.

        // Flag ap_update_add_1 is a bit.
        eval.add_constraint(
            (ap_update_add_1_col3.clone() * (M31_1.clone() - ap_update_add_1_col3.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.verify_instruction_lookup_elements,
            E::EF::one(),
            &[
                input_pc_col0.clone(),
                M31_32767.clone(),
                M31_32767.clone(),
                M31_32769.clone(),
                M31_56.clone(),
                (M31_4.clone() + (ap_update_add_1_col3.clone() * M31_32.clone())),
            ],
        ));

        // Read Small.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (input_pc_col0.clone() + M31_1.clone()),
                next_pc_id_col4.clone(),
            ],
        ));

        // Cond Decode Small Sign.

        // msb is a bit.
        eval.add_constraint((msb_col5.clone() * (msb_col5.clone() - M31_1.clone())));
        // mid_limbs_set is a bit.
        eval.add_constraint(
            (mid_limbs_set_col6.clone() * (mid_limbs_set_col6.clone() - M31_1.clone())),
        );
        // Cannot have msb equals 0 and mid_limbs_set equals 1.
        eval.add_constraint((mid_limbs_set_col6.clone() * (msb_col5.clone() - M31_1.clone())));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                next_pc_id_col4.clone(),
                next_pc_limb_0_col7.clone(),
                next_pc_limb_1_col8.clone(),
                next_pc_limb_2_col9.clone(),
                (mid_limbs_set_col6.clone() * M31_511.clone()),
                (mid_limbs_set_col6.clone() * M31_511.clone()),
                (mid_limbs_set_col6.clone() * M31_511.clone()),
                (mid_limbs_set_col6.clone() * M31_511.clone()),
                (mid_limbs_set_col6.clone() * M31_511.clone()),
                (mid_limbs_set_col6.clone() * M31_511.clone()),
                (mid_limbs_set_col6.clone() * M31_511.clone()),
                (mid_limbs_set_col6.clone() * M31_511.clone()),
                (mid_limbs_set_col6.clone() * M31_511.clone()),
                (mid_limbs_set_col6.clone() * M31_511.clone()),
                (mid_limbs_set_col6.clone() * M31_511.clone()),
                (mid_limbs_set_col6.clone() * M31_511.clone()),
                (mid_limbs_set_col6.clone() * M31_511.clone()),
                (mid_limbs_set_col6.clone() * M31_511.clone()),
                (mid_limbs_set_col6.clone() * M31_511.clone()),
                (mid_limbs_set_col6.clone() * M31_511.clone()),
                (mid_limbs_set_col6.clone() * M31_511.clone()),
                (mid_limbs_set_col6.clone() * M31_511.clone()),
                ((M31_136.clone() * msb_col5.clone()) - mid_limbs_set_col6.clone()),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                (msb_col5.clone() * M31_256.clone()),
            ],
        ));

        let read_small_output_tmp_ff4f6_9_limb_0 = eval.add_intermediate(
            ((((next_pc_limb_0_col7.clone() + (next_pc_limb_1_col8.clone() * M31_512.clone()))
                + (next_pc_limb_2_col9.clone() * M31_262144.clone()))
                - msb_col5.clone())
                - (M31_134217728.clone() * mid_limbs_set_col6.clone())),
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
                (input_pc_col0.clone() + read_small_output_tmp_ff4f6_9_limb_0.clone()),
                (input_ap_col1.clone() + ap_update_add_1_col3.clone()),
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
    use crate::components::constraints_regression_test_values::JUMP_OPCODE_REL_IMM;

    #[test]
    fn jump_opcode_rel_imm_constraints_regression() {
        let eval = Eval {
            claim: Claim { log_size: 4 },
            memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
            memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
            opcodes_lookup_elements: relations::Opcodes::dummy(),
            verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
        };

        let expr_eval = eval.evaluate(ExprEvaluator::new());
        let mut rng = SmallRng::seed_from_u64(0);
        let mut sum = QM31::zero();
        for c in expr_eval.constraints {
            sum += c.random_eval() * rng.gen::<QM31>();
        }

        assert_eq!(sum, JUMP_OPCODE_REL_IMM);
    }
}
