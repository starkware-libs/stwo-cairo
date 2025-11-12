// This file was created by the AIR team.

use crate::components::prelude::*;

pub const N_TRACE_COLUMNS: usize = 1;
pub const LOG_SIZE: u32 = 4;
pub const RELATION_USES_PER_ROW: [RelationUse; 0] = [];

pub struct Eval {
    pub claim: Claim,
    pub blake_round_sigma_lookup_elements: relations::BlakeRoundSigma,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct Claim {}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let trace_log_sizes = vec![LOG_SIZE; N_TRACE_COLUMNS];
        let interaction_log_sizes = vec![LOG_SIZE; SECURE_EXTENSION_DEGREE];
        TreeVec::new(vec![vec![], trace_log_sizes, interaction_log_sizes])
    }

    pub fn mix_into(&self, _channel: &mut impl Channel) {}
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
        LOG_SIZE
    }

    fn max_constraint_log_degree_bound(&self) -> u32 {
        self.log_size() + 1
    }

    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    fn evaluate<E: EvalAtRow>(&self, mut eval: E) -> E {
        let seq_4 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "seq_4".to_owned(),
        });
        let blake_sigma_0 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "blake_sigma_0".to_owned(),
        });
        let blake_sigma_1 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "blake_sigma_1".to_owned(),
        });
        let blake_sigma_2 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "blake_sigma_2".to_owned(),
        });
        let blake_sigma_3 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "blake_sigma_3".to_owned(),
        });
        let blake_sigma_4 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "blake_sigma_4".to_owned(),
        });
        let blake_sigma_5 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "blake_sigma_5".to_owned(),
        });
        let blake_sigma_6 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "blake_sigma_6".to_owned(),
        });
        let blake_sigma_7 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "blake_sigma_7".to_owned(),
        });
        let blake_sigma_8 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "blake_sigma_8".to_owned(),
        });
        let blake_sigma_9 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "blake_sigma_9".to_owned(),
        });
        let blake_sigma_10 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "blake_sigma_10".to_owned(),
        });
        let blake_sigma_11 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "blake_sigma_11".to_owned(),
        });
        let blake_sigma_12 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "blake_sigma_12".to_owned(),
        });
        let blake_sigma_13 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "blake_sigma_13".to_owned(),
        });
        let blake_sigma_14 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "blake_sigma_14".to_owned(),
        });
        let blake_sigma_15 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "blake_sigma_15".to_owned(),
        });
        let multiplicity = eval.next_trace_mask();

        eval.add_to_relation(RelationEntry::new(
            &self.blake_round_sigma_lookup_elements,
            -E::EF::from(multiplicity),
            &[
                seq_4.clone(),
                blake_sigma_0.clone(),
                blake_sigma_1.clone(),
                blake_sigma_2.clone(),
                blake_sigma_3.clone(),
                blake_sigma_4.clone(),
                blake_sigma_5.clone(),
                blake_sigma_6.clone(),
                blake_sigma_7.clone(),
                blake_sigma_8.clone(),
                blake_sigma_9.clone(),
                blake_sigma_10.clone(),
                blake_sigma_11.clone(),
                blake_sigma_12.clone(),
                blake_sigma_13.clone(),
                blake_sigma_14.clone(),
                blake_sigma_15.clone(),
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
    use crate::components::constraints_regression_test_values::BLAKE_ROUND_SIGMA;

    #[test]
    fn blake_round_sigma_constraints_regression() {
        let mut rng = SmallRng::seed_from_u64(0);
        let eval = Eval {
            claim: Claim {},
            blake_round_sigma_lookup_elements: relations::BlakeRoundSigma::dummy(),
        };
        let expr_eval = eval.evaluate(ExprEvaluator::new());
        let assignment = expr_eval.random_assignment();

        let mut sum = QM31::zero();
        for c in expr_eval.constraints {
            sum += c.assign(&assignment) * rng.gen::<QM31>();
        }

        assert_eq!(sum, BLAKE_ROUND_SIGMA);
    }
}
