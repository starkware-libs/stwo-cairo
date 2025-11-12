// This file was created by the AIR team.

use crate::components::prelude::*;

pub const N_TRACE_COLUMNS: usize = 1;
pub const LOG_SIZE: u32 = 6;
pub const RELATION_USES_PER_ROW: [RelationUse; 0] = [];

pub struct Eval {
    pub claim: Claim,
    pub poseidon_round_keys_lookup_elements: relations::PoseidonRoundKeys,
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
        let seq_6 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "seq_6".to_owned(),
        });
        let poseidon_round_keys_0 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "poseidon_round_keys_0".to_owned(),
        });
        let poseidon_round_keys_1 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "poseidon_round_keys_1".to_owned(),
        });
        let poseidon_round_keys_2 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "poseidon_round_keys_2".to_owned(),
        });
        let poseidon_round_keys_3 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "poseidon_round_keys_3".to_owned(),
        });
        let poseidon_round_keys_4 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "poseidon_round_keys_4".to_owned(),
        });
        let poseidon_round_keys_5 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "poseidon_round_keys_5".to_owned(),
        });
        let poseidon_round_keys_6 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "poseidon_round_keys_6".to_owned(),
        });
        let poseidon_round_keys_7 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "poseidon_round_keys_7".to_owned(),
        });
        let poseidon_round_keys_8 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "poseidon_round_keys_8".to_owned(),
        });
        let poseidon_round_keys_9 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "poseidon_round_keys_9".to_owned(),
        });
        let poseidon_round_keys_10 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "poseidon_round_keys_10".to_owned(),
        });
        let poseidon_round_keys_11 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "poseidon_round_keys_11".to_owned(),
        });
        let poseidon_round_keys_12 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "poseidon_round_keys_12".to_owned(),
        });
        let poseidon_round_keys_13 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "poseidon_round_keys_13".to_owned(),
        });
        let poseidon_round_keys_14 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "poseidon_round_keys_14".to_owned(),
        });
        let poseidon_round_keys_15 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "poseidon_round_keys_15".to_owned(),
        });
        let poseidon_round_keys_16 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "poseidon_round_keys_16".to_owned(),
        });
        let poseidon_round_keys_17 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "poseidon_round_keys_17".to_owned(),
        });
        let poseidon_round_keys_18 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "poseidon_round_keys_18".to_owned(),
        });
        let poseidon_round_keys_19 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "poseidon_round_keys_19".to_owned(),
        });
        let poseidon_round_keys_20 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "poseidon_round_keys_20".to_owned(),
        });
        let poseidon_round_keys_21 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "poseidon_round_keys_21".to_owned(),
        });
        let poseidon_round_keys_22 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "poseidon_round_keys_22".to_owned(),
        });
        let poseidon_round_keys_23 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "poseidon_round_keys_23".to_owned(),
        });
        let poseidon_round_keys_24 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "poseidon_round_keys_24".to_owned(),
        });
        let poseidon_round_keys_25 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "poseidon_round_keys_25".to_owned(),
        });
        let poseidon_round_keys_26 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "poseidon_round_keys_26".to_owned(),
        });
        let poseidon_round_keys_27 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "poseidon_round_keys_27".to_owned(),
        });
        let poseidon_round_keys_28 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "poseidon_round_keys_28".to_owned(),
        });
        let poseidon_round_keys_29 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "poseidon_round_keys_29".to_owned(),
        });
        let multiplicity = eval.next_trace_mask();

        eval.add_to_relation(RelationEntry::new(
            &self.poseidon_round_keys_lookup_elements,
            -E::EF::from(multiplicity),
            &[
                seq_6.clone(),
                poseidon_round_keys_0.clone(),
                poseidon_round_keys_1.clone(),
                poseidon_round_keys_2.clone(),
                poseidon_round_keys_3.clone(),
                poseidon_round_keys_4.clone(),
                poseidon_round_keys_5.clone(),
                poseidon_round_keys_6.clone(),
                poseidon_round_keys_7.clone(),
                poseidon_round_keys_8.clone(),
                poseidon_round_keys_9.clone(),
                poseidon_round_keys_10.clone(),
                poseidon_round_keys_11.clone(),
                poseidon_round_keys_12.clone(),
                poseidon_round_keys_13.clone(),
                poseidon_round_keys_14.clone(),
                poseidon_round_keys_15.clone(),
                poseidon_round_keys_16.clone(),
                poseidon_round_keys_17.clone(),
                poseidon_round_keys_18.clone(),
                poseidon_round_keys_19.clone(),
                poseidon_round_keys_20.clone(),
                poseidon_round_keys_21.clone(),
                poseidon_round_keys_22.clone(),
                poseidon_round_keys_23.clone(),
                poseidon_round_keys_24.clone(),
                poseidon_round_keys_25.clone(),
                poseidon_round_keys_26.clone(),
                poseidon_round_keys_27.clone(),
                poseidon_round_keys_28.clone(),
                poseidon_round_keys_29.clone(),
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
    use crate::components::constraints_regression_test_values::POSEIDON_ROUND_KEYS;

    #[test]
    fn poseidon_round_keys_constraints_regression() {
        let mut rng = SmallRng::seed_from_u64(0);
        let eval = Eval {
            claim: Claim {},
            poseidon_round_keys_lookup_elements: relations::PoseidonRoundKeys::dummy(),
        };
        let expr_eval = eval.evaluate(ExprEvaluator::new());
        let assignment = expr_eval.random_assignment();

        let mut sum = QM31::zero();
        for c in expr_eval.constraints {
            sum += c.assign(&assignment) * rng.gen::<QM31>();
        }

        assert_eq!(sum, POSEIDON_ROUND_KEYS);
    }
}
