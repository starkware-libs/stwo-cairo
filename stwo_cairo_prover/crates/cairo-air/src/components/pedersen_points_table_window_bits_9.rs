// This file was created by the AIR team.

use crate::components::prelude::*;

pub const N_TRACE_COLUMNS: usize = 1;
pub const LOG_SIZE: u32 = 15;
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
        let M31_1791500038 = E::F::from(M31::from(1791500038));
        let seq_15 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "seq_15".to_owned(),
        });
        let pedersen_points_small_0 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_small_0".to_owned(),
        });
        let pedersen_points_small_1 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_small_1".to_owned(),
        });
        let pedersen_points_small_2 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_small_2".to_owned(),
        });
        let pedersen_points_small_3 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_small_3".to_owned(),
        });
        let pedersen_points_small_4 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_small_4".to_owned(),
        });
        let pedersen_points_small_5 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_small_5".to_owned(),
        });
        let pedersen_points_small_6 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_small_6".to_owned(),
        });
        let pedersen_points_small_7 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_small_7".to_owned(),
        });
        let pedersen_points_small_8 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_small_8".to_owned(),
        });
        let pedersen_points_small_9 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_small_9".to_owned(),
        });
        let pedersen_points_small_10 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_small_10".to_owned(),
        });
        let pedersen_points_small_11 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_small_11".to_owned(),
        });
        let pedersen_points_small_12 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_small_12".to_owned(),
        });
        let pedersen_points_small_13 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_small_13".to_owned(),
        });
        let pedersen_points_small_14 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_small_14".to_owned(),
        });
        let pedersen_points_small_15 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_small_15".to_owned(),
        });
        let pedersen_points_small_16 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_small_16".to_owned(),
        });
        let pedersen_points_small_17 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_small_17".to_owned(),
        });
        let pedersen_points_small_18 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_small_18".to_owned(),
        });
        let pedersen_points_small_19 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_small_19".to_owned(),
        });
        let pedersen_points_small_20 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_small_20".to_owned(),
        });
        let pedersen_points_small_21 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_small_21".to_owned(),
        });
        let pedersen_points_small_22 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_small_22".to_owned(),
        });
        let pedersen_points_small_23 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_small_23".to_owned(),
        });
        let pedersen_points_small_24 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_small_24".to_owned(),
        });
        let pedersen_points_small_25 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_small_25".to_owned(),
        });
        let pedersen_points_small_26 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_small_26".to_owned(),
        });
        let pedersen_points_small_27 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_small_27".to_owned(),
        });
        let pedersen_points_small_28 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_small_28".to_owned(),
        });
        let pedersen_points_small_29 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_small_29".to_owned(),
        });
        let pedersen_points_small_30 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_small_30".to_owned(),
        });
        let pedersen_points_small_31 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_small_31".to_owned(),
        });
        let pedersen_points_small_32 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_small_32".to_owned(),
        });
        let pedersen_points_small_33 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_small_33".to_owned(),
        });
        let pedersen_points_small_34 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_small_34".to_owned(),
        });
        let pedersen_points_small_35 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_small_35".to_owned(),
        });
        let pedersen_points_small_36 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_small_36".to_owned(),
        });
        let pedersen_points_small_37 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_small_37".to_owned(),
        });
        let pedersen_points_small_38 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_small_38".to_owned(),
        });
        let pedersen_points_small_39 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_small_39".to_owned(),
        });
        let pedersen_points_small_40 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_small_40".to_owned(),
        });
        let pedersen_points_small_41 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_small_41".to_owned(),
        });
        let pedersen_points_small_42 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_small_42".to_owned(),
        });
        let pedersen_points_small_43 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_small_43".to_owned(),
        });
        let pedersen_points_small_44 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_small_44".to_owned(),
        });
        let pedersen_points_small_45 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_small_45".to_owned(),
        });
        let pedersen_points_small_46 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_small_46".to_owned(),
        });
        let pedersen_points_small_47 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_small_47".to_owned(),
        });
        let pedersen_points_small_48 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_small_48".to_owned(),
        });
        let pedersen_points_small_49 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_small_49".to_owned(),
        });
        let pedersen_points_small_50 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_small_50".to_owned(),
        });
        let pedersen_points_small_51 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_small_51".to_owned(),
        });
        let pedersen_points_small_52 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_small_52".to_owned(),
        });
        let pedersen_points_small_53 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_small_53".to_owned(),
        });
        let pedersen_points_small_54 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_small_54".to_owned(),
        });
        let pedersen_points_small_55 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "pedersen_points_small_55".to_owned(),
        });
        let multiplicity_0_col0 = eval.next_trace_mask();

        eval.add_to_relation(RelationEntry::new(
            &self.common_lookup_elements,
            -E::EF::from(multiplicity_0_col0.clone()),
            &[
                M31_1791500038.clone(),
                seq_15.clone(),
                pedersen_points_small_0.clone(),
                pedersen_points_small_1.clone(),
                pedersen_points_small_2.clone(),
                pedersen_points_small_3.clone(),
                pedersen_points_small_4.clone(),
                pedersen_points_small_5.clone(),
                pedersen_points_small_6.clone(),
                pedersen_points_small_7.clone(),
                pedersen_points_small_8.clone(),
                pedersen_points_small_9.clone(),
                pedersen_points_small_10.clone(),
                pedersen_points_small_11.clone(),
                pedersen_points_small_12.clone(),
                pedersen_points_small_13.clone(),
                pedersen_points_small_14.clone(),
                pedersen_points_small_15.clone(),
                pedersen_points_small_16.clone(),
                pedersen_points_small_17.clone(),
                pedersen_points_small_18.clone(),
                pedersen_points_small_19.clone(),
                pedersen_points_small_20.clone(),
                pedersen_points_small_21.clone(),
                pedersen_points_small_22.clone(),
                pedersen_points_small_23.clone(),
                pedersen_points_small_24.clone(),
                pedersen_points_small_25.clone(),
                pedersen_points_small_26.clone(),
                pedersen_points_small_27.clone(),
                pedersen_points_small_28.clone(),
                pedersen_points_small_29.clone(),
                pedersen_points_small_30.clone(),
                pedersen_points_small_31.clone(),
                pedersen_points_small_32.clone(),
                pedersen_points_small_33.clone(),
                pedersen_points_small_34.clone(),
                pedersen_points_small_35.clone(),
                pedersen_points_small_36.clone(),
                pedersen_points_small_37.clone(),
                pedersen_points_small_38.clone(),
                pedersen_points_small_39.clone(),
                pedersen_points_small_40.clone(),
                pedersen_points_small_41.clone(),
                pedersen_points_small_42.clone(),
                pedersen_points_small_43.clone(),
                pedersen_points_small_44.clone(),
                pedersen_points_small_45.clone(),
                pedersen_points_small_46.clone(),
                pedersen_points_small_47.clone(),
                pedersen_points_small_48.clone(),
                pedersen_points_small_49.clone(),
                pedersen_points_small_50.clone(),
                pedersen_points_small_51.clone(),
                pedersen_points_small_52.clone(),
                pedersen_points_small_53.clone(),
                pedersen_points_small_54.clone(),
                pedersen_points_small_55.clone(),
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
    use crate::components::constraints_regression_test_values::PEDERSEN_POINTS_TABLE_WINDOW_BITS_9;

    #[test]
    fn pedersen_points_table_window_bits_9_constraints_regression() {
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

        PEDERSEN_POINTS_TABLE_WINDOW_BITS_9.assert_debug_eq(&sum);
    }
}
