// This file was created by the AIR team.

use crate::components::prelude::*;
use crate::components::subroutines::range_check_mem_value_n_8::RangeCheckMemValueN8;

pub const N_TRACE_COLUMNS: usize = 9;
pub const RELATION_USES_PER_ROW: [RelationUse; 4] = [
    RelationUse {
        relation_id: "RangeCheck_9_9",
        uses: 1,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_B",
        uses: 1,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_C",
        uses: 1,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_D",
        uses: 1,
    },
];

pub struct Eval {
    pub claim: Claim,
    pub common_lookup_elements: relations::CommonLookupElements,
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
        let M31_1662111297 = E::F::from(M31::from(1662111297));
        let seq = eval.get_preprocessed_column(Seq::new(self.log_size()).id());
        let memory_id_to_small_output_col0 = eval.next_trace_mask();
        let memory_id_to_small_output_col1 = eval.next_trace_mask();
        let memory_id_to_small_output_col2 = eval.next_trace_mask();
        let memory_id_to_small_output_col3 = eval.next_trace_mask();
        let memory_id_to_small_output_col4 = eval.next_trace_mask();
        let memory_id_to_small_output_col5 = eval.next_trace_mask();
        let memory_id_to_small_output_col6 = eval.next_trace_mask();
        let memory_id_to_small_output_col7 = eval.next_trace_mask();
        let multiplicity_0 = eval.next_trace_mask();

        RangeCheckMemValueN8::evaluate(
            [
                memory_id_to_small_output_col0.clone(),
                memory_id_to_small_output_col1.clone(),
                memory_id_to_small_output_col2.clone(),
                memory_id_to_small_output_col3.clone(),
                memory_id_to_small_output_col4.clone(),
                memory_id_to_small_output_col5.clone(),
                memory_id_to_small_output_col6.clone(),
                memory_id_to_small_output_col7.clone(),
            ],
            &self.common_lookup_elements,
            &mut eval,
        );
        eval.add_to_relation(RelationEntry::new(
            &self.common_lookup_elements,
            -E::EF::from(multiplicity_0),
            &[
                M31_1662111297.clone(),
                seq.clone(),
                memory_id_to_small_output_col0.clone(),
                memory_id_to_small_output_col1.clone(),
                memory_id_to_small_output_col2.clone(),
                memory_id_to_small_output_col3.clone(),
                memory_id_to_small_output_col4.clone(),
                memory_id_to_small_output_col5.clone(),
                memory_id_to_small_output_col6.clone(),
                memory_id_to_small_output_col7.clone(),
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
    use crate::components::constraints_regression_test_values::MEMORY_ID_TO_SMALL;

    #[test]
    fn memory_id_to_small_constraints_regression() {
        let mut rng = SmallRng::seed_from_u64(0);
        let eval = Eval {
            claim: Claim { log_size: 4 },
            common_lookup_elements: relations::CommonLookupElements::dummy(),
        };
        let expr_eval = eval.evaluate(ExprEvaluator::new());
        let assignment = expr_eval.random_assignment();

        let mut sum = QM31::zero();
        for c in expr_eval.constraints {
            sum += c.assign(&assignment) * rng.gen::<QM31>();
        }

        MEMORY_ID_TO_SMALL.assert_debug_eq(&sum);
    }
}
