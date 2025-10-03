// AIR version ffcad354-dirty
use crate::components::prelude::*;

pub const N_TRACE_COLUMNS: usize = 1;
pub const LOG_SIZE: u32 = 20;
pub const RELATION_USES_PER_ROW: [RelationUse; 0] = [];

pub struct Eval {
    pub claim: Claim,
    pub sha_256_small_sigma_1_o_1_lookup_elements: relations::Sha256SmallSigma1O1,
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
        let sha256sigmatable_Sha256SigmaType_SmallSigma1O1_0 = eval.get_preprocessed_column(
            (Sha256SigmaTable::new(Sha256SigmaType::SmallSigma1O1, 0)).id(),
        );
        let sha256sigmatable_Sha256SigmaType_SmallSigma1O1_1 = eval.get_preprocessed_column(
            (Sha256SigmaTable::new(Sha256SigmaType::SmallSigma1O1, 1)).id(),
        );
        let sha256sigmatable_Sha256SigmaType_SmallSigma1O1_2 = eval.get_preprocessed_column(
            (Sha256SigmaTable::new(Sha256SigmaType::SmallSigma1O1, 2)).id(),
        );
        let sha256sigmatable_Sha256SigmaType_SmallSigma1O1_3 = eval.get_preprocessed_column(
            (Sha256SigmaTable::new(Sha256SigmaType::SmallSigma1O1, 3)).id(),
        );
        let sha256sigmatable_Sha256SigmaType_SmallSigma1O1_4 = eval.get_preprocessed_column(
            (Sha256SigmaTable::new(Sha256SigmaType::SmallSigma1O1, 4)).id(),
        );
        let sha256sigmatable_Sha256SigmaType_SmallSigma1O1_5 = eval.get_preprocessed_column(
            (Sha256SigmaTable::new(Sha256SigmaType::SmallSigma1O1, 5)).id(),
        );
        let multiplicity = eval.next_trace_mask();

        eval.add_to_relation(RelationEntry::new(
            &self.sha_256_small_sigma_1_o_1_lookup_elements,
            -E::EF::from(multiplicity),
            &[
                sha256sigmatable_Sha256SigmaType_SmallSigma1O1_0.clone(),
                sha256sigmatable_Sha256SigmaType_SmallSigma1O1_1.clone(),
                sha256sigmatable_Sha256SigmaType_SmallSigma1O1_2.clone(),
                sha256sigmatable_Sha256SigmaType_SmallSigma1O1_3.clone(),
                sha256sigmatable_Sha256SigmaType_SmallSigma1O1_4.clone(),
                sha256sigmatable_Sha256SigmaType_SmallSigma1O1_5.clone(),
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
    use crate::components::constraints_regression_test_values::SHA_256_SMALL_SIGMA_1_O_1;

    #[test]
    fn sha_256_small_sigma_1_o_1_constraints_regression() {
        let mut rng = SmallRng::seed_from_u64(0);
        let eval = Eval {
            claim: Claim {},
            sha_256_small_sigma_1_o_1_lookup_elements: relations::Sha256SmallSigma1O1::dummy(),
        };
        let expr_eval = eval.evaluate(ExprEvaluator::new());
        let assignment = expr_eval.random_assignment();

        let mut sum = QM31::zero();
        for c in expr_eval.constraints {
            sum += c.assign(&assignment) * rng.gen::<QM31>();
        }

        assert_eq!(sum, SHA_256_SMALL_SIGMA_1_O_1);
    }
}
