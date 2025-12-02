// This file was created by the AIR team.

use crate::components::prelude::*;

pub const N_TRACE_COLUMNS: usize = 8;
pub const LOG_SIZE: u32 = 18;
pub const RELATION_USES_PER_ROW: [RelationUse; 0] = [];

pub struct Eval {
    pub claim: Claim,
    pub range_check_9_9_lookup_elements: relations::RangeCheck_9_9,
    pub range_check_9_9_b_lookup_elements: relations::RangeCheck_9_9_B,
    pub range_check_9_9_c_lookup_elements: relations::RangeCheck_9_9_C,
    pub range_check_9_9_d_lookup_elements: relations::RangeCheck_9_9_D,
    pub range_check_9_9_e_lookup_elements: relations::RangeCheck_9_9_E,
    pub range_check_9_9_f_lookup_elements: relations::RangeCheck_9_9_F,
    pub range_check_9_9_g_lookup_elements: relations::RangeCheck_9_9_G,
    pub range_check_9_9_h_lookup_elements: relations::RangeCheck_9_9_H,
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
        let range_check_9_9_column_0 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "range_check_9_9_column_0".to_owned(),
        });
        let range_check_9_9_column_1 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "range_check_9_9_column_1".to_owned(),
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
            &self.range_check_9_9_lookup_elements,
            -E::EF::from(multiplicity_0),
            &[
                range_check_9_9_column_0.clone(),
                range_check_9_9_column_1.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_9_9_b_lookup_elements,
            -E::EF::from(multiplicity_1),
            &[
                range_check_9_9_column_0.clone(),
                range_check_9_9_column_1.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_9_9_c_lookup_elements,
            -E::EF::from(multiplicity_2),
            &[
                range_check_9_9_column_0.clone(),
                range_check_9_9_column_1.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_9_9_d_lookup_elements,
            -E::EF::from(multiplicity_3),
            &[
                range_check_9_9_column_0.clone(),
                range_check_9_9_column_1.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_9_9_e_lookup_elements,
            -E::EF::from(multiplicity_4),
            &[
                range_check_9_9_column_0.clone(),
                range_check_9_9_column_1.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_9_9_f_lookup_elements,
            -E::EF::from(multiplicity_5),
            &[
                range_check_9_9_column_0.clone(),
                range_check_9_9_column_1.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_9_9_g_lookup_elements,
            -E::EF::from(multiplicity_6),
            &[
                range_check_9_9_column_0.clone(),
                range_check_9_9_column_1.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_9_9_h_lookup_elements,
            -E::EF::from(multiplicity_7),
            &[
                range_check_9_9_column_0.clone(),
                range_check_9_9_column_1.clone(),
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
    use crate::components::constraints_regression_test_values::RANGE_CHECK_9_9;

    #[test]
    fn range_check_9_9_constraints_regression() {
        let mut rng = SmallRng::seed_from_u64(0);
        let eval = Eval {
            claim: Claim {},
            range_check_9_9_lookup_elements: relations::RangeCheck_9_9::dummy(),
            range_check_9_9_b_lookup_elements: relations::RangeCheck_9_9_B::dummy(),
            range_check_9_9_c_lookup_elements: relations::RangeCheck_9_9_C::dummy(),
            range_check_9_9_d_lookup_elements: relations::RangeCheck_9_9_D::dummy(),
            range_check_9_9_e_lookup_elements: relations::RangeCheck_9_9_E::dummy(),
            range_check_9_9_f_lookup_elements: relations::RangeCheck_9_9_F::dummy(),
            range_check_9_9_g_lookup_elements: relations::RangeCheck_9_9_G::dummy(),
            range_check_9_9_h_lookup_elements: relations::RangeCheck_9_9_H::dummy(),
        };
        let expr_eval = eval.evaluate(ExprEvaluator::new());
        let assignment = expr_eval.random_assignment();

        let mut sum = QM31::zero();
        for c in expr_eval.constraints {
            sum += c.assign(&assignment) * rng.gen::<QM31>();
        }

        assert_eq!(sum, RANGE_CHECK_9_9);
    }
}
