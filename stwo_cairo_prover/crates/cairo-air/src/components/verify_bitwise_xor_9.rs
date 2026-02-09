// This file was created by the AIR team.

use crate::components::prelude::*;

pub const N_TRACE_COLUMNS: usize = 1;
pub const LOG_SIZE: u32 = 18;
pub const RELATION_USES_PER_ROW: [RelationUse; 0] = [];

pub struct Eval {
    pub common_lookup_elements: relations::CommonLookupElements,
}

pub fn log_sizes(log_size: u32) -> TreeVec<Vec<u32>> {
    let trace_log_sizes = vec![log_size; N_TRACE_COLUMNS];
    let interaction_log_sizes = vec![log_size; SECURE_EXTENSION_DEGREE];
    TreeVec::new(vec![vec![], trace_log_sizes, interaction_log_sizes])
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
        let M31_95781001 = E::F::from(M31::from(95781001));
        let bitwise_xor_9_0 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "bitwise_xor_9_0".to_owned(),
        });
        let bitwise_xor_9_1 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "bitwise_xor_9_1".to_owned(),
        });
        let bitwise_xor_9_2 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "bitwise_xor_9_2".to_owned(),
        });
        let multiplicity_0 = eval.next_trace_mask();

        eval.add_to_relation(RelationEntry::new(
            &self.common_lookup_elements,
            -E::EF::from(multiplicity_0),
            &[
                M31_95781001.clone(),
                bitwise_xor_9_0.clone(),
                bitwise_xor_9_1.clone(),
                bitwise_xor_9_2.clone(),
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
            common_lookup_elements: relations::CommonLookupElements::dummy(),
        };
        let expr_eval = eval.evaluate(ExprEvaluator::new());
        let assignment = expr_eval.random_assignment();

        let mut sum = QM31::zero();
        for c in expr_eval.constraints {
            sum += c.assign(&assignment) * rng.gen::<QM31>();
        }

        VERIFY_BITWISE_XOR_9.assert_debug_eq(&sum);
    }
}
