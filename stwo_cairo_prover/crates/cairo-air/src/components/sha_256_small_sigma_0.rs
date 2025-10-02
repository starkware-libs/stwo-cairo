// AIR version 98896da1
use crate::components::prelude::*;
use crate::components::subroutines::bitwise_and_num_bits_16::BitwiseAndNumBits16;
use crate::components::subroutines::bitwise_xor_num_bits_16::BitwiseXorNumBits16;

pub const N_TRACE_COLUMNS: usize = 21;
pub const RELATION_USES_PER_ROW: [RelationUse; 4] = [
    RelationUse {
        relation_id: "Sha256SmallSigma0O0",
        uses: 1,
    },
    RelationUse {
        relation_id: "Sha256SmallSigma0O1",
        uses: 1,
    },
    RelationUse {
        relation_id: "VerifyBitwiseAnd_16",
        uses: 6,
    },
    RelationUse {
        relation_id: "VerifyBitwiseXor_16",
        uses: 2,
    },
];

pub struct Eval {
    pub claim: Claim,
    pub verify_bitwise_and_16_lookup_elements: relations::VerifyBitwiseAnd_16,
    pub sha_256_small_sigma_0_o_0_lookup_elements: relations::Sha256SmallSigma0O0,
    pub sha_256_small_sigma_0_o_1_lookup_elements: relations::Sha256SmallSigma0O1,
    pub verify_bitwise_xor_16_lookup_elements: relations::VerifyBitwiseXor_16,
    pub sha_256_small_sigma_0_lookup_elements: relations::Sha256SmallSigma0,
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
        let M31_19114 = E::F::from(M31::from(19114));
        let M31_46336 = E::F::from(M31::from(46336));
        let M31_85 = E::F::from(M31::from(85));
        let input_limb_0_col0 = eval.next_trace_mask();
        let input_limb_1_col1 = eval.next_trace_mask();
        let and_col2 = eval.next_trace_mask();
        let and_col3 = eval.next_trace_mask();
        let and_col4 = eval.next_trace_mask();
        let and_col5 = eval.next_trace_mask();
        let and_col6 = eval.next_trace_mask();
        let and_col7 = eval.next_trace_mask();
        let sigma_O0_L_col8 = eval.next_trace_mask();
        let sigma_O0_H_col9 = eval.next_trace_mask();
        let sigma_O1_L_col10 = eval.next_trace_mask();
        let sigma_O1_H_col11 = eval.next_trace_mask();
        let sigma_O2_L_col12 = eval.next_trace_mask();
        let sigma_O2_H_col13 = eval.next_trace_mask();
        let sigma_O2_prime_L_col14 = eval.next_trace_mask();
        let sigma_O2_prime_H_col15 = eval.next_trace_mask();
        let xor_col16 = eval.next_trace_mask();
        let xor_col17 = eval.next_trace_mask();
        let output_low_col18 = eval.next_trace_mask();
        let output_high_col19 = eval.next_trace_mask();
        let enabler = eval.next_trace_mask();

        eval.add_constraint(enabler.clone() * enabler.clone() - enabler.clone());

        BitwiseAndNumBits16::evaluate(
            [input_limb_0_col0.clone(), M31_19114.clone()],
            and_col2.clone(),
            &self.verify_bitwise_and_16_lookup_elements,
            &mut eval,
        );
        BitwiseAndNumBits16::evaluate(
            [input_limb_0_col0.clone(), M31_85.clone()],
            and_col3.clone(),
            &self.verify_bitwise_and_16_lookup_elements,
            &mut eval,
        );
        BitwiseAndNumBits16::evaluate(
            [input_limb_0_col0.clone(), M31_46336.clone()],
            and_col4.clone(),
            &self.verify_bitwise_and_16_lookup_elements,
            &mut eval,
        );
        BitwiseAndNumBits16::evaluate(
            [input_limb_1_col1.clone(), M31_85.clone()],
            and_col5.clone(),
            &self.verify_bitwise_and_16_lookup_elements,
            &mut eval,
        );
        BitwiseAndNumBits16::evaluate(
            [input_limb_1_col1.clone(), M31_46336.clone()],
            and_col6.clone(),
            &self.verify_bitwise_and_16_lookup_elements,
            &mut eval,
        );
        BitwiseAndNumBits16::evaluate(
            [input_limb_1_col1.clone(), M31_19114.clone()],
            and_col7.clone(),
            &self.verify_bitwise_and_16_lookup_elements,
            &mut eval,
        );
        eval.add_to_relation(RelationEntry::new(
            &self.sha_256_small_sigma_0_o_0_lookup_elements,
            E::EF::one(),
            &[
                (and_col3.clone() + and_col4.clone()),
                and_col7.clone(),
                sigma_O0_L_col8.clone(),
                sigma_O0_H_col9.clone(),
                sigma_O2_L_col12.clone(),
                sigma_O2_H_col13.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.sha_256_small_sigma_0_o_1_lookup_elements,
            E::EF::one(),
            &[
                and_col2.clone(),
                (and_col5.clone() + and_col6.clone()),
                sigma_O1_L_col10.clone(),
                sigma_O1_H_col11.clone(),
                sigma_O2_prime_L_col14.clone(),
                sigma_O2_prime_H_col15.clone(),
            ],
        ));

        BitwiseXorNumBits16::evaluate(
            [sigma_O2_prime_L_col14.clone(), sigma_O2_L_col12.clone()],
            xor_col16.clone(),
            &self.verify_bitwise_xor_16_lookup_elements,
            &mut eval,
        );
        BitwiseXorNumBits16::evaluate(
            [sigma_O2_prime_H_col15.clone(), sigma_O2_H_col13.clone()],
            xor_col17.clone(),
            &self.verify_bitwise_xor_16_lookup_elements,
            &mut eval,
        );
        eval.add_to_relation(RelationEntry::new(
            &self.sha_256_small_sigma_0_lookup_elements,
            -E::EF::from(enabler.clone()),
            &[
                input_limb_0_col0.clone(),
                input_limb_1_col1.clone(),
                output_low_col18.clone(),
                output_high_col19.clone(),
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
    use crate::components::constraints_regression_test_values::SHA_256_SMALL_SIGMA_0;

    #[test]
    fn sha_256_small_sigma_0_constraints_regression() {
        let mut rng = SmallRng::seed_from_u64(0);
        let eval = Eval {
            claim: Claim { log_size: 4 },
            verify_bitwise_and_16_lookup_elements: relations::VerifyBitwiseAnd_16::dummy(),
            sha_256_small_sigma_0_o_0_lookup_elements: relations::Sha256SmallSigma0O0::dummy(),
            sha_256_small_sigma_0_o_1_lookup_elements: relations::Sha256SmallSigma0O1::dummy(),
            verify_bitwise_xor_16_lookup_elements: relations::VerifyBitwiseXor_16::dummy(),
            sha_256_small_sigma_0_lookup_elements: relations::Sha256SmallSigma0::dummy(),
        };
        let expr_eval = eval.evaluate(ExprEvaluator::new());
        let assignment = expr_eval.random_assignment();

        let mut sum = QM31::zero();
        for c in expr_eval.constraints {
            sum += c.assign(&assignment) * rng.gen::<QM31>();
        }

        assert_eq!(sum, SHA_256_SMALL_SIGMA_0);
    }
}
