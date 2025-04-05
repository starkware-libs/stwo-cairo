use crate::components::prelude::*;
use crate::components::subroutines::read_positive_num_bits_128::ReadPositiveNumBits128;

pub const N_TRACE_COLUMNS: usize = 17;

pub struct Eval {
    pub claim: Claim,
    pub memory_address_to_id_lookup_elements: relations::MemoryAddressToId,
    pub memory_id_to_big_lookup_elements: relations::MemoryIdToBig,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct Claim {
    pub log_size: u32,
    pub range_check_builtin_segment_start: u32,
}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let trace_log_sizes = vec![self.log_size; N_TRACE_COLUMNS];
        let interaction_log_sizes = vec![self.log_size; SECURE_EXTENSION_DEGREE];
        TreeVec::new(vec![vec![], trace_log_sizes, interaction_log_sizes])
    }

    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_u64(self.log_size as u64);
        channel.mix_u64(self.range_check_builtin_segment_start as u64);
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
        let seq = eval.get_preprocessed_column(Seq::new(self.log_size()).id());
        let value_id_col0 = eval.next_trace_mask();
        let value_limb_0_col1 = eval.next_trace_mask();
        let value_limb_1_col2 = eval.next_trace_mask();
        let value_limb_2_col3 = eval.next_trace_mask();
        let value_limb_3_col4 = eval.next_trace_mask();
        let value_limb_4_col5 = eval.next_trace_mask();
        let value_limb_5_col6 = eval.next_trace_mask();
        let value_limb_6_col7 = eval.next_trace_mask();
        let value_limb_7_col8 = eval.next_trace_mask();
        let value_limb_8_col9 = eval.next_trace_mask();
        let value_limb_9_col10 = eval.next_trace_mask();
        let value_limb_10_col11 = eval.next_trace_mask();
        let value_limb_11_col12 = eval.next_trace_mask();
        let value_limb_12_col13 = eval.next_trace_mask();
        let value_limb_13_col14 = eval.next_trace_mask();
        let value_limb_14_col15 = eval.next_trace_mask();
        let msb_col16 = eval.next_trace_mask();

        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [read_positive_num_bits_128_output_tmp_66b3a_4_limb_0, read_positive_num_bits_128_output_tmp_66b3a_4_limb_1, read_positive_num_bits_128_output_tmp_66b3a_4_limb_2, read_positive_num_bits_128_output_tmp_66b3a_4_limb_3, read_positive_num_bits_128_output_tmp_66b3a_4_limb_4, read_positive_num_bits_128_output_tmp_66b3a_4_limb_5, read_positive_num_bits_128_output_tmp_66b3a_4_limb_6, read_positive_num_bits_128_output_tmp_66b3a_4_limb_7, read_positive_num_bits_128_output_tmp_66b3a_4_limb_8, read_positive_num_bits_128_output_tmp_66b3a_4_limb_9, read_positive_num_bits_128_output_tmp_66b3a_4_limb_10, read_positive_num_bits_128_output_tmp_66b3a_4_limb_11, read_positive_num_bits_128_output_tmp_66b3a_4_limb_12, read_positive_num_bits_128_output_tmp_66b3a_4_limb_13, read_positive_num_bits_128_output_tmp_66b3a_4_limb_14, read_positive_num_bits_128_output_tmp_66b3a_4_limb_15, read_positive_num_bits_128_output_tmp_66b3a_4_limb_16, read_positive_num_bits_128_output_tmp_66b3a_4_limb_17, read_positive_num_bits_128_output_tmp_66b3a_4_limb_18, read_positive_num_bits_128_output_tmp_66b3a_4_limb_19, read_positive_num_bits_128_output_tmp_66b3a_4_limb_20, read_positive_num_bits_128_output_tmp_66b3a_4_limb_21, read_positive_num_bits_128_output_tmp_66b3a_4_limb_22, read_positive_num_bits_128_output_tmp_66b3a_4_limb_23, read_positive_num_bits_128_output_tmp_66b3a_4_limb_24, read_positive_num_bits_128_output_tmp_66b3a_4_limb_25, read_positive_num_bits_128_output_tmp_66b3a_4_limb_26, read_positive_num_bits_128_output_tmp_66b3a_4_limb_27, read_positive_num_bits_128_output_tmp_66b3a_4_limb_28] =
            ReadPositiveNumBits128::evaluate(
                (E::F::from(M31::from(self.claim.range_check_builtin_segment_start)) + seq.clone()),
                value_id_col0.clone(),
                value_limb_0_col1.clone(),
                value_limb_1_col2.clone(),
                value_limb_2_col3.clone(),
                value_limb_3_col4.clone(),
                value_limb_4_col5.clone(),
                value_limb_5_col6.clone(),
                value_limb_6_col7.clone(),
                value_limb_7_col8.clone(),
                value_limb_8_col9.clone(),
                value_limb_9_col10.clone(),
                value_limb_10_col11.clone(),
                value_limb_11_col12.clone(),
                value_limb_12_col13.clone(),
                value_limb_13_col14.clone(),
                value_limb_14_col15.clone(),
                msb_col16.clone(),
                &mut eval,
                &self.memory_address_to_id_lookup_elements,
                &self.memory_id_to_big_lookup_elements,
            );
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
    use crate::components::constraints_regression_test_values::RANGE_CHECK_BUILTIN_BITS_128;

    #[test]
    fn range_check_builtin_bits_128_constraints_regression() {
        let mut rng = SmallRng::seed_from_u64(0);
        let eval = Eval {
            claim: Claim {
                log_size: 4,
                range_check_builtin_segment_start: rng.gen::<u32>(),
            },
            memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
            memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
        };
        let expr_eval = eval.evaluate(ExprEvaluator::new());
        let assignment = expr_eval.random_assignment();

        let mut sum = QM31::zero();
        for c in expr_eval.constraints {
            sum += c.assign(&assignment) * rng.gen::<QM31>();
        }

        assert_eq!(sum, RANGE_CHECK_BUILTIN_BITS_128);
    }
}
