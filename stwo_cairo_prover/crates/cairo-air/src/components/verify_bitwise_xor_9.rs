use crate::components::prelude::*;

pub const N_TRACE_COLUMNS: usize = 1;
pub const N_BITS: u32 = 9;
pub const LOG_SIZE: u32 = 2 * N_BITS;

pub struct Eval {
    pub claim: Claim,
    pub verify_bitwise_xor_9_lookup_elements: relations::VerifyBitwiseXor_9,
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
        let bitwisexor_9_0 = eval.get_preprocessed_column((BitwiseXor::new(9, 0)).id());
        let bitwisexor_9_1 = eval.get_preprocessed_column((BitwiseXor::new(9, 1)).id());
        let bitwisexor_9_2 = eval.get_preprocessed_column((BitwiseXor::new(9, 2)).id());
        let multiplicity = eval.next_trace_mask();

        eval.add_to_relation(RelationEntry::new(
            &self.verify_bitwise_xor_9_lookup_elements,
            -E::EF::from(multiplicity),
            &[
                bitwisexor_9_0.clone(),
                bitwisexor_9_1.clone(),
                bitwisexor_9_2.clone(),
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
    use crate::components::constraints_regression_test_values::VERIFY_BITWISE_XOR_9;

    #[test]
    fn verify_bitwise_xor_9_constraints_regression() {
        let mut rng = SmallRng::seed_from_u64(0);
        let eval = Eval {
            claim: Claim {},
            verify_bitwise_xor_9_lookup_elements: relations::VerifyBitwiseXor_9::dummy(),
        };

        let expr_eval = eval.evaluate(ExprEvaluator::new());
        let assignment = expr_eval.random_assignment();

        let mut sum = QM31::zero();
        for c in expr_eval.constraints {
            sum += c.assign(&assignment) * rng.gen::<QM31>();
        }

        assert_eq!(sum, VERIFY_BITWISE_XOR_9);
    }
}
