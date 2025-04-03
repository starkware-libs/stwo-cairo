use stwo_cairo_common::preprocessed_consts::pedersen::PEDERSEN_TABLE_N_ROWS;

use crate::components::prelude::*;

pub const N_TRACE_COLUMNS: usize = 1;
pub const LOG_SIZE: u32 = PEDERSEN_TABLE_N_ROWS.next_power_of_two().ilog2();

pub struct Eval {
    pub claim: Claim,
    pub pedersen_points_table_lookup_elements: relations::PedersenPointsTable,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct Claim {}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let trace_log_sizes = vec![LOG_SIZE; N_TRACE_COLUMNS];
        let interaction_log_sizes = vec![LOG_SIZE; SECURE_EXTENSION_DEGREE];
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
        let pedersenpoints_0 = eval.get_preprocessed_column((PedersenPoints::new(0)).id());
        let pedersenpoints_1 = eval.get_preprocessed_column((PedersenPoints::new(1)).id());
        let pedersenpoints_2 = eval.get_preprocessed_column((PedersenPoints::new(2)).id());
        let pedersenpoints_3 = eval.get_preprocessed_column((PedersenPoints::new(3)).id());
        let pedersenpoints_4 = eval.get_preprocessed_column((PedersenPoints::new(4)).id());
        let pedersenpoints_5 = eval.get_preprocessed_column((PedersenPoints::new(5)).id());
        let pedersenpoints_6 = eval.get_preprocessed_column((PedersenPoints::new(6)).id());
        let pedersenpoints_7 = eval.get_preprocessed_column((PedersenPoints::new(7)).id());
        let pedersenpoints_8 = eval.get_preprocessed_column((PedersenPoints::new(8)).id());
        let pedersenpoints_9 = eval.get_preprocessed_column((PedersenPoints::new(9)).id());
        let pedersenpoints_10 = eval.get_preprocessed_column((PedersenPoints::new(10)).id());
        let pedersenpoints_11 = eval.get_preprocessed_column((PedersenPoints::new(11)).id());
        let pedersenpoints_12 = eval.get_preprocessed_column((PedersenPoints::new(12)).id());
        let pedersenpoints_13 = eval.get_preprocessed_column((PedersenPoints::new(13)).id());
        let pedersenpoints_14 = eval.get_preprocessed_column((PedersenPoints::new(14)).id());
        let pedersenpoints_15 = eval.get_preprocessed_column((PedersenPoints::new(15)).id());
        let pedersenpoints_16 = eval.get_preprocessed_column((PedersenPoints::new(16)).id());
        let pedersenpoints_17 = eval.get_preprocessed_column((PedersenPoints::new(17)).id());
        let pedersenpoints_18 = eval.get_preprocessed_column((PedersenPoints::new(18)).id());
        let pedersenpoints_19 = eval.get_preprocessed_column((PedersenPoints::new(19)).id());
        let pedersenpoints_20 = eval.get_preprocessed_column((PedersenPoints::new(20)).id());
        let pedersenpoints_21 = eval.get_preprocessed_column((PedersenPoints::new(21)).id());
        let pedersenpoints_22 = eval.get_preprocessed_column((PedersenPoints::new(22)).id());
        let pedersenpoints_23 = eval.get_preprocessed_column((PedersenPoints::new(23)).id());
        let pedersenpoints_24 = eval.get_preprocessed_column((PedersenPoints::new(24)).id());
        let pedersenpoints_25 = eval.get_preprocessed_column((PedersenPoints::new(25)).id());
        let pedersenpoints_26 = eval.get_preprocessed_column((PedersenPoints::new(26)).id());
        let pedersenpoints_27 = eval.get_preprocessed_column((PedersenPoints::new(27)).id());
        let pedersenpoints_28 = eval.get_preprocessed_column((PedersenPoints::new(28)).id());
        let pedersenpoints_29 = eval.get_preprocessed_column((PedersenPoints::new(29)).id());
        let pedersenpoints_30 = eval.get_preprocessed_column((PedersenPoints::new(30)).id());
        let pedersenpoints_31 = eval.get_preprocessed_column((PedersenPoints::new(31)).id());
        let pedersenpoints_32 = eval.get_preprocessed_column((PedersenPoints::new(32)).id());
        let pedersenpoints_33 = eval.get_preprocessed_column((PedersenPoints::new(33)).id());
        let pedersenpoints_34 = eval.get_preprocessed_column((PedersenPoints::new(34)).id());
        let pedersenpoints_35 = eval.get_preprocessed_column((PedersenPoints::new(35)).id());
        let pedersenpoints_36 = eval.get_preprocessed_column((PedersenPoints::new(36)).id());
        let pedersenpoints_37 = eval.get_preprocessed_column((PedersenPoints::new(37)).id());
        let pedersenpoints_38 = eval.get_preprocessed_column((PedersenPoints::new(38)).id());
        let pedersenpoints_39 = eval.get_preprocessed_column((PedersenPoints::new(39)).id());
        let pedersenpoints_40 = eval.get_preprocessed_column((PedersenPoints::new(40)).id());
        let pedersenpoints_41 = eval.get_preprocessed_column((PedersenPoints::new(41)).id());
        let pedersenpoints_42 = eval.get_preprocessed_column((PedersenPoints::new(42)).id());
        let pedersenpoints_43 = eval.get_preprocessed_column((PedersenPoints::new(43)).id());
        let pedersenpoints_44 = eval.get_preprocessed_column((PedersenPoints::new(44)).id());
        let pedersenpoints_45 = eval.get_preprocessed_column((PedersenPoints::new(45)).id());
        let pedersenpoints_46 = eval.get_preprocessed_column((PedersenPoints::new(46)).id());
        let pedersenpoints_47 = eval.get_preprocessed_column((PedersenPoints::new(47)).id());
        let pedersenpoints_48 = eval.get_preprocessed_column((PedersenPoints::new(48)).id());
        let pedersenpoints_49 = eval.get_preprocessed_column((PedersenPoints::new(49)).id());
        let pedersenpoints_50 = eval.get_preprocessed_column((PedersenPoints::new(50)).id());
        let pedersenpoints_51 = eval.get_preprocessed_column((PedersenPoints::new(51)).id());
        let pedersenpoints_52 = eval.get_preprocessed_column((PedersenPoints::new(52)).id());
        let pedersenpoints_53 = eval.get_preprocessed_column((PedersenPoints::new(53)).id());
        let pedersenpoints_54 = eval.get_preprocessed_column((PedersenPoints::new(54)).id());
        let pedersenpoints_55 = eval.get_preprocessed_column((PedersenPoints::new(55)).id());
        let seq = eval.get_preprocessed_column(Seq::new(self.log_size()).id());
        let multiplicity = eval.next_trace_mask();

        eval.add_to_relation(RelationEntry::new(
            &self.pedersen_points_table_lookup_elements,
            -E::EF::from(multiplicity),
            &[
                seq.clone(),
                pedersenpoints_0.clone(),
                pedersenpoints_1.clone(),
                pedersenpoints_2.clone(),
                pedersenpoints_3.clone(),
                pedersenpoints_4.clone(),
                pedersenpoints_5.clone(),
                pedersenpoints_6.clone(),
                pedersenpoints_7.clone(),
                pedersenpoints_8.clone(),
                pedersenpoints_9.clone(),
                pedersenpoints_10.clone(),
                pedersenpoints_11.clone(),
                pedersenpoints_12.clone(),
                pedersenpoints_13.clone(),
                pedersenpoints_14.clone(),
                pedersenpoints_15.clone(),
                pedersenpoints_16.clone(),
                pedersenpoints_17.clone(),
                pedersenpoints_18.clone(),
                pedersenpoints_19.clone(),
                pedersenpoints_20.clone(),
                pedersenpoints_21.clone(),
                pedersenpoints_22.clone(),
                pedersenpoints_23.clone(),
                pedersenpoints_24.clone(),
                pedersenpoints_25.clone(),
                pedersenpoints_26.clone(),
                pedersenpoints_27.clone(),
                pedersenpoints_28.clone(),
                pedersenpoints_29.clone(),
                pedersenpoints_30.clone(),
                pedersenpoints_31.clone(),
                pedersenpoints_32.clone(),
                pedersenpoints_33.clone(),
                pedersenpoints_34.clone(),
                pedersenpoints_35.clone(),
                pedersenpoints_36.clone(),
                pedersenpoints_37.clone(),
                pedersenpoints_38.clone(),
                pedersenpoints_39.clone(),
                pedersenpoints_40.clone(),
                pedersenpoints_41.clone(),
                pedersenpoints_42.clone(),
                pedersenpoints_43.clone(),
                pedersenpoints_44.clone(),
                pedersenpoints_45.clone(),
                pedersenpoints_46.clone(),
                pedersenpoints_47.clone(),
                pedersenpoints_48.clone(),
                pedersenpoints_49.clone(),
                pedersenpoints_50.clone(),
                pedersenpoints_51.clone(),
                pedersenpoints_52.clone(),
                pedersenpoints_53.clone(),
                pedersenpoints_54.clone(),
                pedersenpoints_55.clone(),
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
    use crate::components::constraints_regression_test_values::PEDERSEN_POINTS_TABLE;

    #[test]
    fn pedersen_points_table_constraints_regression() {
        let mut rng = SmallRng::seed_from_u64(0);
        let eval = Eval {
            claim: Claim {},
            pedersen_points_table_lookup_elements: relations::PedersenPointsTable::dummy(),
        };

        let expr_eval = eval.evaluate(ExprEvaluator::new());
        let assignment = expr_eval.random_assignment();

        let mut sum = QM31::zero();
        for c in expr_eval.constraints {
            sum += c.assign(&assignment) * rng.gen::<QM31>();
        }

        assert_eq!(sum, PEDERSEN_POINTS_TABLE);
    }
}
