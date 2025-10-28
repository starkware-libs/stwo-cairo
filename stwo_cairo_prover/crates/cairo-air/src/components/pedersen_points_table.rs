// This file was created by the AIR team.

use crate::components::prelude::*;

pub const N_TRACE_COLUMNS: usize = 1;
pub const LOG_SIZE: u32 = 23;
pub const RELATION_USES_PER_ROW: [RelationUse; 0] = [];

pub struct Eval {
    pub claim: Claim,
    pub pedersen_points_table_lookup_elements: relations::PedersenPointsTable,
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
        let seq_23 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "seq_23".to_owned(),
        });
        let pedersen_points_0 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_0".to_owned(),
        });
        let pedersen_points_1 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_1".to_owned(),
        });
        let pedersen_points_2 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_2".to_owned(),
        });
        let pedersen_points_3 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_3".to_owned(),
        });
        let pedersen_points_4 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_4".to_owned(),
        });
        let pedersen_points_5 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_5".to_owned(),
        });
        let pedersen_points_6 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_6".to_owned(),
        });
        let pedersen_points_7 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_7".to_owned(),
        });
        let pedersen_points_8 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_8".to_owned(),
        });
        let pedersen_points_9 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_9".to_owned(),
        });
        let pedersen_points_10 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_10".to_owned(),
        });
        let pedersen_points_11 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_11".to_owned(),
        });
        let pedersen_points_12 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_12".to_owned(),
        });
        let pedersen_points_13 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_13".to_owned(),
        });
        let pedersen_points_14 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_14".to_owned(),
        });
        let pedersen_points_15 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_15".to_owned(),
        });
        let pedersen_points_16 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_16".to_owned(),
        });
        let pedersen_points_17 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_17".to_owned(),
        });
        let pedersen_points_18 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_18".to_owned(),
        });
        let pedersen_points_19 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_19".to_owned(),
        });
        let pedersen_points_20 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_20".to_owned(),
        });
        let pedersen_points_21 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_21".to_owned(),
        });
        let pedersen_points_22 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_22".to_owned(),
        });
        let pedersen_points_23 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_23".to_owned(),
        });
        let pedersen_points_24 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_24".to_owned(),
        });
        let pedersen_points_25 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_25".to_owned(),
        });
        let pedersen_points_26 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_26".to_owned(),
        });
        let pedersen_points_27 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_27".to_owned(),
        });
        let pedersen_points_28 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_28".to_owned(),
        });
        let pedersen_points_29 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_29".to_owned(),
        });
        let pedersen_points_30 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_30".to_owned(),
        });
        let pedersen_points_31 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_31".to_owned(),
        });
        let pedersen_points_32 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_32".to_owned(),
        });
        let pedersen_points_33 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_33".to_owned(),
        });
        let pedersen_points_34 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_34".to_owned(),
        });
        let pedersen_points_35 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_35".to_owned(),
        });
        let pedersen_points_36 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_36".to_owned(),
        });
        let pedersen_points_37 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_37".to_owned(),
        });
        let pedersen_points_38 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_38".to_owned(),
        });
        let pedersen_points_39 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_39".to_owned(),
        });
        let pedersen_points_40 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_40".to_owned(),
        });
        let pedersen_points_41 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_41".to_owned(),
        });
        let pedersen_points_42 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_42".to_owned(),
        });
        let pedersen_points_43 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_43".to_owned(),
        });
        let pedersen_points_44 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_44".to_owned(),
        });
        let pedersen_points_45 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_45".to_owned(),
        });
        let pedersen_points_46 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_46".to_owned(),
        });
        let pedersen_points_47 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_47".to_owned(),
        });
        let pedersen_points_48 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_48".to_owned(),
        });
        let pedersen_points_49 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_49".to_owned(),
        });
        let pedersen_points_50 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_50".to_owned(),
        });
        let pedersen_points_51 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_51".to_owned(),
        });
        let pedersen_points_52 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_52".to_owned(),
        });
        let pedersen_points_53 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_53".to_owned(),
        });
        let pedersen_points_54 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_54".to_owned(),
        });
        let pedersen_points_55 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_55".to_owned(),
        });
        let multiplicity = eval.next_trace_mask();

        eval.add_to_relation(RelationEntry::new(
            &self.pedersen_points_table_lookup_elements,
            -E::EF::from(multiplicity),
            &[
                seq_23.clone(),
                pedersen_points_0.clone(),
                pedersen_points_1.clone(),
                pedersen_points_2.clone(),
                pedersen_points_3.clone(),
                pedersen_points_4.clone(),
                pedersen_points_5.clone(),
                pedersen_points_6.clone(),
                pedersen_points_7.clone(),
                pedersen_points_8.clone(),
                pedersen_points_9.clone(),
                pedersen_points_10.clone(),
                pedersen_points_11.clone(),
                pedersen_points_12.clone(),
                pedersen_points_13.clone(),
                pedersen_points_14.clone(),
                pedersen_points_15.clone(),
                pedersen_points_16.clone(),
                pedersen_points_17.clone(),
                pedersen_points_18.clone(),
                pedersen_points_19.clone(),
                pedersen_points_20.clone(),
                pedersen_points_21.clone(),
                pedersen_points_22.clone(),
                pedersen_points_23.clone(),
                pedersen_points_24.clone(),
                pedersen_points_25.clone(),
                pedersen_points_26.clone(),
                pedersen_points_27.clone(),
                pedersen_points_28.clone(),
                pedersen_points_29.clone(),
                pedersen_points_30.clone(),
                pedersen_points_31.clone(),
                pedersen_points_32.clone(),
                pedersen_points_33.clone(),
                pedersen_points_34.clone(),
                pedersen_points_35.clone(),
                pedersen_points_36.clone(),
                pedersen_points_37.clone(),
                pedersen_points_38.clone(),
                pedersen_points_39.clone(),
                pedersen_points_40.clone(),
                pedersen_points_41.clone(),
                pedersen_points_42.clone(),
                pedersen_points_43.clone(),
                pedersen_points_44.clone(),
                pedersen_points_45.clone(),
                pedersen_points_46.clone(),
                pedersen_points_47.clone(),
                pedersen_points_48.clone(),
                pedersen_points_49.clone(),
                pedersen_points_50.clone(),
                pedersen_points_51.clone(),
                pedersen_points_52.clone(),
                pedersen_points_53.clone(),
                pedersen_points_54.clone(),
                pedersen_points_55.clone(),
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
