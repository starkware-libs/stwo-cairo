// AIR version 98896da1
use crate::components::prelude::*;

pub const N_TRACE_COLUMNS: usize = 39;
pub const RELATION_USES_PER_ROW: [RelationUse; 3] = [
    RelationUse {
        relation_id: "RangeCheck_2",
        uses: 2,
    },
    RelationUse {
        relation_id: "Sha256SmallSigma0",
        uses: 1,
    },
    RelationUse {
        relation_id: "Sha256SmallSigma1",
        uses: 1,
    },
];

pub struct Eval {
    pub claim: Claim,
    pub sha_256_small_sigma_0_lookup_elements: relations::Sha256SmallSigma0,
    pub sha_256_small_sigma_1_lookup_elements: relations::Sha256SmallSigma1,
    pub range_check_2_lookup_elements: relations::RangeCheck_2,
    pub sha_256_schedule_lookup_elements: relations::Sha256Schedule,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
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
        let M31_32768 = E::F::from(M31::from(32768));
        let input_limb_0_col0 = eval.next_trace_mask();
        let input_limb_1_col1 = eval.next_trace_mask();
        let input_limb_2_col2 = eval.next_trace_mask();
        let input_limb_3_col3 = eval.next_trace_mask();
        let input_limb_4_col4 = eval.next_trace_mask();
        let input_limb_5_col5 = eval.next_trace_mask();
        let input_limb_6_col6 = eval.next_trace_mask();
        let input_limb_7_col7 = eval.next_trace_mask();
        let input_limb_8_col8 = eval.next_trace_mask();
        let input_limb_9_col9 = eval.next_trace_mask();
        let input_limb_10_col10 = eval.next_trace_mask();
        let input_limb_11_col11 = eval.next_trace_mask();
        let input_limb_12_col12 = eval.next_trace_mask();
        let input_limb_13_col13 = eval.next_trace_mask();
        let input_limb_14_col14 = eval.next_trace_mask();
        let input_limb_15_col15 = eval.next_trace_mask();
        let input_limb_16_col16 = eval.next_trace_mask();
        let input_limb_17_col17 = eval.next_trace_mask();
        let input_limb_18_col18 = eval.next_trace_mask();
        let input_limb_19_col19 = eval.next_trace_mask();
        let input_limb_20_col20 = eval.next_trace_mask();
        let input_limb_21_col21 = eval.next_trace_mask();
        let input_limb_22_col22 = eval.next_trace_mask();
        let input_limb_23_col23 = eval.next_trace_mask();
        let input_limb_24_col24 = eval.next_trace_mask();
        let input_limb_25_col25 = eval.next_trace_mask();
        let input_limb_26_col26 = eval.next_trace_mask();
        let input_limb_27_col27 = eval.next_trace_mask();
        let input_limb_28_col28 = eval.next_trace_mask();
        let input_limb_29_col29 = eval.next_trace_mask();
        let input_limb_30_col30 = eval.next_trace_mask();
        let input_limb_31_col31 = eval.next_trace_mask();
        let sha_256_small_sigma_0_output_limb_0_col32 = eval.next_trace_mask();
        let sha_256_small_sigma_0_output_limb_1_col33 = eval.next_trace_mask();
        let sha_256_small_sigma_1_output_limb_0_col34 = eval.next_trace_mask();
        let sha_256_small_sigma_1_output_limb_1_col35 = eval.next_trace_mask();
        let w_i_limb_0_col36 = eval.next_trace_mask();
        let w_i_limb_1_col37 = eval.next_trace_mask();
        let enabler = eval.next_trace_mask();

        eval.add_constraint(enabler.clone() * enabler.clone() - enabler.clone());

        eval.add_to_relation(RelationEntry::new(
            &self.sha_256_small_sigma_0_lookup_elements,
            E::EF::one(),
            &[
                input_limb_2_col2.clone(),
                input_limb_3_col3.clone(),
                sha_256_small_sigma_0_output_limb_0_col32.clone(),
                sha_256_small_sigma_0_output_limb_1_col33.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.sha_256_small_sigma_1_lookup_elements,
            E::EF::one(),
            &[
                input_limb_28_col28.clone(),
                input_limb_29_col29.clone(),
                sha_256_small_sigma_1_output_limb_0_col34.clone(),
                sha_256_small_sigma_1_output_limb_1_col35.clone(),
            ],
        ));

        let carry_low_tmp_3b5f1_3 = eval.add_intermediate(
            (((((input_limb_0_col0.clone() + sha_256_small_sigma_0_output_limb_0_col32.clone())
                + input_limb_18_col18.clone())
                + sha_256_small_sigma_1_output_limb_0_col34.clone())
                - w_i_limb_0_col36.clone())
                * M31_32768.clone()),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_2_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&carry_low_tmp_3b5f1_3),
        ));

        let carry_high_tmp_3b5f1_4 = eval.add_intermediate(
            ((((((input_limb_1_col1.clone()
                + sha_256_small_sigma_0_output_limb_1_col33.clone())
                + input_limb_19_col19.clone())
                + sha_256_small_sigma_1_output_limb_1_col35.clone())
                + carry_low_tmp_3b5f1_3.clone())
                - w_i_limb_1_col37.clone())
                * M31_32768.clone()),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_2_lookup_elements,
            E::EF::one(),
            std::slice::from_ref(&carry_high_tmp_3b5f1_4),
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.sha_256_schedule_lookup_elements,
            -E::EF::from(enabler.clone()),
            &[
                input_limb_0_col0.clone(),
                input_limb_1_col1.clone(),
                input_limb_2_col2.clone(),
                input_limb_3_col3.clone(),
                input_limb_4_col4.clone(),
                input_limb_5_col5.clone(),
                input_limb_6_col6.clone(),
                input_limb_7_col7.clone(),
                input_limb_8_col8.clone(),
                input_limb_9_col9.clone(),
                input_limb_10_col10.clone(),
                input_limb_11_col11.clone(),
                input_limb_12_col12.clone(),
                input_limb_13_col13.clone(),
                input_limb_14_col14.clone(),
                input_limb_15_col15.clone(),
                input_limb_16_col16.clone(),
                input_limb_17_col17.clone(),
                input_limb_18_col18.clone(),
                input_limb_19_col19.clone(),
                input_limb_20_col20.clone(),
                input_limb_21_col21.clone(),
                input_limb_22_col22.clone(),
                input_limb_23_col23.clone(),
                input_limb_24_col24.clone(),
                input_limb_25_col25.clone(),
                input_limb_26_col26.clone(),
                input_limb_27_col27.clone(),
                input_limb_28_col28.clone(),
                input_limb_29_col29.clone(),
                input_limb_30_col30.clone(),
                input_limb_31_col31.clone(),
                w_i_limb_0_col36.clone(),
                w_i_limb_1_col37.clone(),
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
    use crate::components::constraints_regression_test_values::SHA_256_SCHEDULE;

    #[test]
    fn sha_256_schedule_constraints_regression() {
        let mut rng = SmallRng::seed_from_u64(0);
        let eval = Eval {
            claim: Claim { log_size: 4 },
            sha_256_small_sigma_0_lookup_elements: relations::Sha256SmallSigma0::dummy(),
            sha_256_small_sigma_1_lookup_elements: relations::Sha256SmallSigma1::dummy(),
            range_check_2_lookup_elements: relations::RangeCheck_2::dummy(),
            sha_256_schedule_lookup_elements: relations::Sha256Schedule::dummy(),
        };
        let expr_eval = eval.evaluate(ExprEvaluator::new());
        let assignment = expr_eval.random_assignment();

        let mut sum = QM31::zero();
        for c in expr_eval.constraints {
            sum += c.assign(&assignment) * rng.gen::<QM31>();
        }

        assert_eq!(sum, SHA_256_SCHEDULE);
    }
}
