use crate::components::prelude::*;
use crate::components::subroutines::range_check_mem_value_n_28::RangeCheckMemValueN28;

pub const N_TRACE_COLUMNS: usize = 29;
pub const LOG_SIZE: u32 = 4;
pub const RELATION_USES_PER_ROW: [RelationUse; 1] = [RelationUse {
    relation_id: "RangeCheck_9_9",
    uses: 14,
}];

pub struct Eval {
    pub claim: Claim,
    pub range_check_9_9_lookup_elements: relations::RangeCheck_9_9,
    pub memory_id_to_big_lookup_elements: relations::MemoryIdToBig,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct Claim {}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let trace_log_sizes = vec![LOG_SIZE; N_TRACE_COLUMNS];
        let interaction_log_sizes = vec![LOG_SIZE; SECURE_EXTENSION_DEGREE * 8];
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
        let memoryidforbig_ = eval.get_preprocessed_column((MemoryIdForBig::new()).id());
        let memory_id_to_big_output_col0 = eval.next_trace_mask();
        let memory_id_to_big_output_col1 = eval.next_trace_mask();
        let memory_id_to_big_output_col2 = eval.next_trace_mask();
        let memory_id_to_big_output_col3 = eval.next_trace_mask();
        let memory_id_to_big_output_col4 = eval.next_trace_mask();
        let memory_id_to_big_output_col5 = eval.next_trace_mask();
        let memory_id_to_big_output_col6 = eval.next_trace_mask();
        let memory_id_to_big_output_col7 = eval.next_trace_mask();
        let memory_id_to_big_output_col8 = eval.next_trace_mask();
        let memory_id_to_big_output_col9 = eval.next_trace_mask();
        let memory_id_to_big_output_col10 = eval.next_trace_mask();
        let memory_id_to_big_output_col11 = eval.next_trace_mask();
        let memory_id_to_big_output_col12 = eval.next_trace_mask();
        let memory_id_to_big_output_col13 = eval.next_trace_mask();
        let memory_id_to_big_output_col14 = eval.next_trace_mask();
        let memory_id_to_big_output_col15 = eval.next_trace_mask();
        let memory_id_to_big_output_col16 = eval.next_trace_mask();
        let memory_id_to_big_output_col17 = eval.next_trace_mask();
        let memory_id_to_big_output_col18 = eval.next_trace_mask();
        let memory_id_to_big_output_col19 = eval.next_trace_mask();
        let memory_id_to_big_output_col20 = eval.next_trace_mask();
        let memory_id_to_big_output_col21 = eval.next_trace_mask();
        let memory_id_to_big_output_col22 = eval.next_trace_mask();
        let memory_id_to_big_output_col23 = eval.next_trace_mask();
        let memory_id_to_big_output_col24 = eval.next_trace_mask();
        let memory_id_to_big_output_col25 = eval.next_trace_mask();
        let memory_id_to_big_output_col26 = eval.next_trace_mask();
        let memory_id_to_big_output_col27 = eval.next_trace_mask();
        let multiplicity = eval.next_trace_mask();

        RangeCheckMemValueN28::evaluate(
            [
                memory_id_to_big_output_col0.clone(),
                memory_id_to_big_output_col1.clone(),
                memory_id_to_big_output_col2.clone(),
                memory_id_to_big_output_col3.clone(),
                memory_id_to_big_output_col4.clone(),
                memory_id_to_big_output_col5.clone(),
                memory_id_to_big_output_col6.clone(),
                memory_id_to_big_output_col7.clone(),
                memory_id_to_big_output_col8.clone(),
                memory_id_to_big_output_col9.clone(),
                memory_id_to_big_output_col10.clone(),
                memory_id_to_big_output_col11.clone(),
                memory_id_to_big_output_col12.clone(),
                memory_id_to_big_output_col13.clone(),
                memory_id_to_big_output_col14.clone(),
                memory_id_to_big_output_col15.clone(),
                memory_id_to_big_output_col16.clone(),
                memory_id_to_big_output_col17.clone(),
                memory_id_to_big_output_col18.clone(),
                memory_id_to_big_output_col19.clone(),
                memory_id_to_big_output_col20.clone(),
                memory_id_to_big_output_col21.clone(),
                memory_id_to_big_output_col22.clone(),
                memory_id_to_big_output_col23.clone(),
                memory_id_to_big_output_col24.clone(),
                memory_id_to_big_output_col25.clone(),
                memory_id_to_big_output_col26.clone(),
                memory_id_to_big_output_col27.clone(),
            ],
            &self.range_check_9_9_lookup_elements,
            &mut eval,
        );
        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            -E::EF::from(multiplicity),
            &[
                memoryidforbig_.clone(),
                memory_id_to_big_output_col0.clone(),
                memory_id_to_big_output_col1.clone(),
                memory_id_to_big_output_col2.clone(),
                memory_id_to_big_output_col3.clone(),
                memory_id_to_big_output_col4.clone(),
                memory_id_to_big_output_col5.clone(),
                memory_id_to_big_output_col6.clone(),
                memory_id_to_big_output_col7.clone(),
                memory_id_to_big_output_col8.clone(),
                memory_id_to_big_output_col9.clone(),
                memory_id_to_big_output_col10.clone(),
                memory_id_to_big_output_col11.clone(),
                memory_id_to_big_output_col12.clone(),
                memory_id_to_big_output_col13.clone(),
                memory_id_to_big_output_col14.clone(),
                memory_id_to_big_output_col15.clone(),
                memory_id_to_big_output_col16.clone(),
                memory_id_to_big_output_col17.clone(),
                memory_id_to_big_output_col18.clone(),
                memory_id_to_big_output_col19.clone(),
                memory_id_to_big_output_col20.clone(),
                memory_id_to_big_output_col21.clone(),
                memory_id_to_big_output_col22.clone(),
                memory_id_to_big_output_col23.clone(),
                memory_id_to_big_output_col24.clone(),
                memory_id_to_big_output_col25.clone(),
                memory_id_to_big_output_col26.clone(),
                memory_id_to_big_output_col27.clone(),
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
    use crate::components::constraints_regression_test_values::MEMORY_ID_TO_BIG;

    #[test]
    fn memory_id_to_big_constraints_regression() {
        let mut rng = SmallRng::seed_from_u64(0);
        let eval = Eval {
            claim: Claim {},
            range_check_9_9_lookup_elements: relations::RangeCheck_9_9::dummy(),
            memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
        };
        let expr_eval = eval.evaluate(ExprEvaluator::new());
        let assignment = expr_eval.random_assignment();

        let mut sum = QM31::zero();
        for c in expr_eval.constraints {
            sum += c.assign(&assignment) * rng.gen::<QM31>();
        }

        assert_eq!(sum, MEMORY_ID_TO_BIG);
    }
}
