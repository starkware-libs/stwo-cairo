// This file was created by the AIR team.

use crate::components::prelude::*;
use crate::components::subroutines::read_id::ReadId;

pub const N_TRACE_COLUMNS: usize = 3;
pub const RELATION_USES_PER_ROW: [RelationUse; 2] = [
    RelationUse {
        relation_id: "MemoryAddressToId",
        uses: 3,
    },
    RelationUse {
        relation_id: "PedersenAggregatorWindowBits18",
        uses: 1,
    },
];

pub struct Eval {
    pub log_size: u32,
    pub pedersen_builtin_segment_start: u32,
    pub common_lookup_elements: relations::CommonLookupElements,
}

pub fn log_sizes(log_size: u32) -> TreeVec<Vec<u32>> {
    let trace_log_sizes = vec![log_size; N_TRACE_COLUMNS];
    let interaction_log_sizes = vec![log_size; SECURE_EXTENSION_DEGREE * 2];
    TreeVec::new(vec![vec![], trace_log_sizes, interaction_log_sizes])
}

pub type Component = FrameworkComponent<Eval>;

impl FrameworkEval for Eval {
    fn log_size(&self) -> u32 {
        self.log_size
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
        let M31_3 = E::F::from(M31::from(3));
        let M31_520578465 = E::F::from(M31::from(520578465));
        let seq = eval.get_preprocessed_column(Seq::new(self.log_size()).id());
        let input_state_0_id_col0 = eval.next_trace_mask();
        let input_state_1_id_col1 = eval.next_trace_mask();
        let output_state_id_col2 = eval.next_trace_mask();

        let instance_addr_tmp_3bd90_0 = eval.add_intermediate(
            ((seq.clone() * M31_3.clone())
                + E::F::from(M31::from(self.pedersen_builtin_segment_start))),
        );
        ReadId::evaluate(
            [instance_addr_tmp_3bd90_0.clone()],
            input_state_0_id_col0.clone(),
            &self.common_lookup_elements,
            &mut eval,
        );
        ReadId::evaluate(
            [(instance_addr_tmp_3bd90_0.clone() + M31_1.clone())],
            input_state_1_id_col1.clone(),
            &self.common_lookup_elements,
            &mut eval,
        );
        ReadId::evaluate(
            [(instance_addr_tmp_3bd90_0.clone() + M31_2.clone())],
            output_state_id_col2.clone(),
            &self.common_lookup_elements,
            &mut eval,
        );
        eval.add_to_relation(RelationEntry::new(
            &self.common_lookup_elements,
            E::EF::one(),
            &[
                M31_520578465.clone(),
                input_state_0_id_col0.clone(),
                input_state_1_id_col1.clone(),
                output_state_id_col2.clone(),
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
    use crate::components::constraints_regression_test_values::PEDERSEN_BUILTIN;

    #[test]
    fn pedersen_builtin_constraints_regression() {
        let mut rng = SmallRng::seed_from_u64(0);
        let eval = Eval {
            log_size: 4,
            pedersen_builtin_segment_start: rng.gen::<u32>(),
            common_lookup_elements: relations::CommonLookupElements::dummy(),
        };
        let expr_eval = eval.evaluate(ExprEvaluator::new());
        let assignment = expr_eval.random_assignment();

        let mut sum = QM31::zero();
        for c in expr_eval.constraints {
            sum += c.assign(&assignment) * rng.gen::<QM31>();
        }

        PEDERSEN_BUILTIN.assert_debug_eq(&sum);
    }
}
