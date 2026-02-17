// This file was created by the AIR team.

use crate::components::prelude::*;

pub const N_TRACE_COLUMNS: usize = 8;
pub const LOG_SIZE: u32 = 20;
pub const RELATION_USES_PER_ROW: [RelationUse; 0] = [];

pub struct Eval {
    pub claim: Claim,
    pub common_lookup_elements: relations::CommonLookupElements,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct Claim {}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let trace_log_sizes = vec![LOG_SIZE; N_TRACE_COLUMNS];
        let interaction_log_sizes = vec![LOG_SIZE; SECURE_EXTENSION_DEGREE * 4];
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
        let M31_1410849886 = E::F::from(M31::from(1410849886));
        let M31_447122465 = E::F::from(M31::from(447122465));
        let M31_463900084 = E::F::from(M31::from(463900084));
        let M31_480677703 = E::F::from(M31::from(480677703));
        let M31_497455322 = E::F::from(M31::from(497455322));
        let M31_514232941 = E::F::from(M31::from(514232941));
        let M31_531010560 = E::F::from(M31::from(531010560));
        let M31_682009131 = E::F::from(M31::from(682009131));
        let seq_20 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "seq_20".to_owned(),
        });
        let multiplicity_0_col0 = eval.next_trace_mask();
        let multiplicity_1_col1 = eval.next_trace_mask();
        let multiplicity_2_col2 = eval.next_trace_mask();
        let multiplicity_3_col3 = eval.next_trace_mask();
        let multiplicity_4_col4 = eval.next_trace_mask();
        let multiplicity_5_col5 = eval.next_trace_mask();
        let multiplicity_6_col6 = eval.next_trace_mask();
        let multiplicity_7_col7 = eval.next_trace_mask();

        eval.add_to_relation(RelationEntry::new(
            &self.common_lookup_elements,
            -E::EF::from(multiplicity_0_col0.clone()),
            &[M31_1410849886.clone(), seq_20.clone()],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.common_lookup_elements,
            -E::EF::from(multiplicity_1_col1.clone()),
            &[M31_514232941.clone(), seq_20.clone()],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.common_lookup_elements,
            -E::EF::from(multiplicity_2_col2.clone()),
            &[M31_531010560.clone(), seq_20.clone()],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.common_lookup_elements,
            -E::EF::from(multiplicity_3_col3.clone()),
            &[M31_480677703.clone(), seq_20.clone()],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.common_lookup_elements,
            -E::EF::from(multiplicity_4_col4.clone()),
            &[M31_497455322.clone(), seq_20.clone()],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.common_lookup_elements,
            -E::EF::from(multiplicity_5_col5.clone()),
            &[M31_447122465.clone(), seq_20.clone()],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.common_lookup_elements,
            -E::EF::from(multiplicity_6_col6.clone()),
            &[M31_463900084.clone(), seq_20.clone()],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.common_lookup_elements,
            -E::EF::from(multiplicity_7_col7.clone()),
            &[M31_682009131.clone(), seq_20.clone()],
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
    use crate::components::constraints_regression_test_values::RANGE_CHECK_20;

    #[test]
    fn range_check_20_constraints_regression() {
        let mut rng = SmallRng::seed_from_u64(0);
        let eval = Eval {
            claim: Claim {},
            common_lookup_elements: relations::CommonLookupElements::dummy(),
        };
        let expr_eval = eval.evaluate(ExprEvaluator::new());
        let assignment = expr_eval.random_assignment();

        let mut sum = QM31::zero();
        for c in expr_eval.constraints {
            sum += c.assign(&assignment) * rng.gen::<QM31>();
        }

        RANGE_CHECK_20.assert_debug_eq(&sum);
    }
}
