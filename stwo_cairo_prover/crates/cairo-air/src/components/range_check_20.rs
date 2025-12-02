// This file was created by the AIR team.

use crate::components::prelude::*;

pub const N_TRACE_COLUMNS: usize = 8;
pub const LOG_SIZE: u32 = 20;
pub const RELATION_USES_PER_ROW: [RelationUse; 0] = [];

pub struct Eval {
    pub claim: Claim,
    pub range_check_20_lookup_elements: relations::RangeCheck_20,
    pub range_check_20_b_lookup_elements: relations::RangeCheck_20_B,
    pub range_check_20_c_lookup_elements: relations::RangeCheck_20_C,
    pub range_check_20_d_lookup_elements: relations::RangeCheck_20_D,
    pub range_check_20_e_lookup_elements: relations::RangeCheck_20_E,
    pub range_check_20_f_lookup_elements: relations::RangeCheck_20_F,
    pub range_check_20_g_lookup_elements: relations::RangeCheck_20_G,
    pub range_check_20_h_lookup_elements: relations::RangeCheck_20_H,
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
        let seq_20 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "seq_20".to_owned(),
        });
        let multiplicity_0 = eval.next_trace_mask();
        let multiplicity_1 = eval.next_trace_mask();
        let multiplicity_2 = eval.next_trace_mask();
        let multiplicity_3 = eval.next_trace_mask();
        let multiplicity_4 = eval.next_trace_mask();
        let multiplicity_5 = eval.next_trace_mask();
        let multiplicity_6 = eval.next_trace_mask();
        let multiplicity_7 = eval.next_trace_mask();

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_20_lookup_elements,
            -E::EF::from(multiplicity_0),
            std::slice::from_ref(&seq_20),
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_20_b_lookup_elements,
            -E::EF::from(multiplicity_1),
            std::slice::from_ref(&seq_20),
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_20_c_lookup_elements,
            -E::EF::from(multiplicity_2),
            std::slice::from_ref(&seq_20),
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_20_d_lookup_elements,
            -E::EF::from(multiplicity_3),
            std::slice::from_ref(&seq_20),
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_20_e_lookup_elements,
            -E::EF::from(multiplicity_4),
            std::slice::from_ref(&seq_20),
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_20_f_lookup_elements,
            -E::EF::from(multiplicity_5),
            std::slice::from_ref(&seq_20),
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_20_g_lookup_elements,
            -E::EF::from(multiplicity_6),
            std::slice::from_ref(&seq_20),
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_20_h_lookup_elements,
            -E::EF::from(multiplicity_7),
            std::slice::from_ref(&seq_20),
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
            range_check_20_lookup_elements: relations::RangeCheck_20::dummy(),
            range_check_20_b_lookup_elements: relations::RangeCheck_20_B::dummy(),
            range_check_20_c_lookup_elements: relations::RangeCheck_20_C::dummy(),
            range_check_20_d_lookup_elements: relations::RangeCheck_20_D::dummy(),
            range_check_20_e_lookup_elements: relations::RangeCheck_20_E::dummy(),
            range_check_20_f_lookup_elements: relations::RangeCheck_20_F::dummy(),
            range_check_20_g_lookup_elements: relations::RangeCheck_20_G::dummy(),
            range_check_20_h_lookup_elements: relations::RangeCheck_20_H::dummy(),
        };
        let expr_eval = eval.evaluate(ExprEvaluator::new());
        let assignment = expr_eval.random_assignment();

        let mut sum = QM31::zero();
        for c in expr_eval.constraints {
            sum += c.assign(&assignment) * rng.gen::<QM31>();
        }

        assert_eq!(sum, RANGE_CHECK_20);
    }
}
