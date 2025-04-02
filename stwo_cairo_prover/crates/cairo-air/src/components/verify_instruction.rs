use crate::components::prelude::*;

pub const N_TRACE_COLUMNS: usize = 17;
pub const LOG_SIZE: u32 = 0;

pub struct Eval {
    pub claim: Claim,
    pub memory_address_to_id_lookup_elements: relations::MemoryAddressToId,
    pub memory_id_to_big_lookup_elements: relations::MemoryIdToBig,
    pub range_check_4_3_lookup_elements: relations::RangeCheck_4_3,
    pub range_check_7_2_5_lookup_elements: relations::RangeCheck_7_2_5,
    pub verify_instruction_lookup_elements: relations::VerifyInstruction,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct Claim {}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let trace_log_sizes = vec![LOG_SIZE; N_TRACE_COLUMNS];
        let interaction_log_sizes = vec![LOG_SIZE; SECURE_EXTENSION_DEGREE * 3];
        TreeVec::new(vec![vec![], trace_log_sizes, interaction_log_sizes])
    }

    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_u64(LOG_SIZE as u64);
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
        LOG_SIZE
    }

    fn max_constraint_log_degree_bound(&self) -> u32 {
        self.log_size() + 1
    }

    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    fn evaluate<E: EvalAtRow>(&self, mut eval: E) -> E {
        let M31_128 = E::F::from(M31::from(128));
        let M31_16 = E::F::from(M31::from(16));
        let M31_2048 = E::F::from(M31::from(2048));
        let M31_32 = E::F::from(M31::from(32));
        let M31_4 = E::F::from(M31::from(4));
        let M31_512 = E::F::from(M31::from(512));
        let M31_8192 = E::F::from(M31::from(8192));
        let input_limb_0_col0 = eval.next_trace_mask();
        let input_limb_1_col1 = eval.next_trace_mask();
        let input_limb_2_col2 = eval.next_trace_mask();
        let input_limb_3_col3 = eval.next_trace_mask();
        let input_limb_4_col4 = eval.next_trace_mask();
        let input_limb_5_col5 = eval.next_trace_mask();
        let input_limb_6_col6 = eval.next_trace_mask();
        let offset0_low_col7 = eval.next_trace_mask();
        let offset0_mid_col8 = eval.next_trace_mask();
        let offset1_low_col9 = eval.next_trace_mask();
        let offset1_mid_col10 = eval.next_trace_mask();
        let offset1_high_col11 = eval.next_trace_mask();
        let offset2_low_col12 = eval.next_trace_mask();
        let offset2_mid_col13 = eval.next_trace_mask();
        let offset2_high_col14 = eval.next_trace_mask();
        let instruction_id_col15 = eval.next_trace_mask();
        let multiplicity = eval.next_trace_mask();

        // Encode Offsets.

        // Reconstructed offset0 is correct.
        eval.add_constraint(
            ((offset0_low_col7.clone() + (offset0_mid_col8.clone() * M31_512.clone()))
                - input_limb_1_col1.clone()),
        );
        // Reconstructed offset1 is correct.
        eval.add_constraint(
            (((offset1_low_col9.clone() + (offset1_mid_col10.clone() * M31_4.clone()))
                + (offset1_high_col11.clone() * M31_2048.clone()))
                - input_limb_2_col2.clone()),
        );
        // Reconstructed offset2 is correct.
        eval.add_constraint(
            (((offset2_low_col12.clone() + (offset2_mid_col13.clone() * M31_16.clone()))
                + (offset2_high_col14.clone() * M31_8192.clone()))
                - input_limb_3_col3.clone()),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_7_2_5_lookup_elements,
            E::EF::one(),
            &[
                offset0_mid_col8.clone(),
                offset1_low_col9.clone(),
                offset1_high_col11.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_4_3_lookup_elements,
            E::EF::one(),
            &[offset2_low_col12.clone(), offset2_high_col14.clone()],
        ));

        let encode_offsets_output_tmp_16a4f_8_limb_1 = eval.add_intermediate(
            (offset0_mid_col8.clone() + (offset1_low_col9.clone() * M31_128.clone())),
        );
        let encode_offsets_output_tmp_16a4f_8_limb_3 = eval.add_intermediate(
            (offset1_high_col11.clone() + (offset2_low_col12.clone() * M31_32.clone())),
        );

        // Mem Verify.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[input_limb_0_col0.clone(), instruction_id_col15.clone()],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                instruction_id_col15.clone(),
                offset0_low_col7.clone(),
                encode_offsets_output_tmp_16a4f_8_limb_1.clone(),
                offset1_mid_col10.clone(),
                encode_offsets_output_tmp_16a4f_8_limb_3.clone(),
                offset2_mid_col13.clone(),
                (offset2_high_col14.clone() + input_limb_4_col4.clone()),
                input_limb_5_col5.clone(),
                input_limb_6_col6.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.verify_instruction_lookup_elements,
            -E::EF::from(multiplicity),
            &[
                input_limb_0_col0.clone(),
                input_limb_1_col1.clone(),
                input_limb_2_col2.clone(),
                input_limb_3_col3.clone(),
                input_limb_4_col4.clone(),
                input_limb_5_col5.clone(),
                input_limb_6_col6.clone(),
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
    use crate::components::constraints_regression_test_values::VERIFY_INSTRUCTION;

    #[test]
    fn verify_instruction_constraints_regression() {
        let eval = Eval {
            claim: Claim { log_size: 4 },
            memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
            memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
            range_check_4_3_lookup_elements: relations::RangeCheck_4_3::dummy(),
            range_check_7_2_5_lookup_elements: relations::RangeCheck_7_2_5::dummy(),
            verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
        };

        let expr_eval = eval.evaluate(ExprEvaluator::new());
        let mut rng = SmallRng::seed_from_u64(0);
        let mut sum = QM31::zero();
        for c in expr_eval.constraints {
            sum += c.random_eval() * rng.gen::<QM31>();
        }

        assert_eq!(sum, VERIFY_INSTRUCTION);
    }
}
