use stwo_cairo_common::preprocessed_consts::poseidon::N_ROUNDS;

use crate::components::prelude::*;

pub const N_TRACE_COLUMNS: usize = 1;
pub const LOG_SIZE: u32 = N_ROUNDS.next_power_of_two().ilog2();

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
        let poseidonroundkeys_0 = eval.get_preprocessed_column((PoseidonRoundKeys::new(0)).id());
        let poseidonroundkeys_1 = eval.get_preprocessed_column((PoseidonRoundKeys::new(1)).id());
        let poseidonroundkeys_2 = eval.get_preprocessed_column((PoseidonRoundKeys::new(2)).id());
        let poseidonroundkeys_3 = eval.get_preprocessed_column((PoseidonRoundKeys::new(3)).id());
        let poseidonroundkeys_4 = eval.get_preprocessed_column((PoseidonRoundKeys::new(4)).id());
        let poseidonroundkeys_5 = eval.get_preprocessed_column((PoseidonRoundKeys::new(5)).id());
        let poseidonroundkeys_6 = eval.get_preprocessed_column((PoseidonRoundKeys::new(6)).id());
        let poseidonroundkeys_7 = eval.get_preprocessed_column((PoseidonRoundKeys::new(7)).id());
        let poseidonroundkeys_8 = eval.get_preprocessed_column((PoseidonRoundKeys::new(8)).id());
        let poseidonroundkeys_9 = eval.get_preprocessed_column((PoseidonRoundKeys::new(9)).id());
        let poseidonroundkeys_10 = eval.get_preprocessed_column((PoseidonRoundKeys::new(10)).id());
        let poseidonroundkeys_11 = eval.get_preprocessed_column((PoseidonRoundKeys::new(11)).id());
        let poseidonroundkeys_12 = eval.get_preprocessed_column((PoseidonRoundKeys::new(12)).id());
        let poseidonroundkeys_13 = eval.get_preprocessed_column((PoseidonRoundKeys::new(13)).id());
        let poseidonroundkeys_14 = eval.get_preprocessed_column((PoseidonRoundKeys::new(14)).id());
        let poseidonroundkeys_15 = eval.get_preprocessed_column((PoseidonRoundKeys::new(15)).id());
        let poseidonroundkeys_16 = eval.get_preprocessed_column((PoseidonRoundKeys::new(16)).id());
        let poseidonroundkeys_17 = eval.get_preprocessed_column((PoseidonRoundKeys::new(17)).id());
        let poseidonroundkeys_18 = eval.get_preprocessed_column((PoseidonRoundKeys::new(18)).id());
        let poseidonroundkeys_19 = eval.get_preprocessed_column((PoseidonRoundKeys::new(19)).id());
        let poseidonroundkeys_20 = eval.get_preprocessed_column((PoseidonRoundKeys::new(20)).id());
        let poseidonroundkeys_21 = eval.get_preprocessed_column((PoseidonRoundKeys::new(21)).id());
        let poseidonroundkeys_22 = eval.get_preprocessed_column((PoseidonRoundKeys::new(22)).id());
        let poseidonroundkeys_23 = eval.get_preprocessed_column((PoseidonRoundKeys::new(23)).id());
        let poseidonroundkeys_24 = eval.get_preprocessed_column((PoseidonRoundKeys::new(24)).id());
        let poseidonroundkeys_25 = eval.get_preprocessed_column((PoseidonRoundKeys::new(25)).id());
        let poseidonroundkeys_26 = eval.get_preprocessed_column((PoseidonRoundKeys::new(26)).id());
        let poseidonroundkeys_27 = eval.get_preprocessed_column((PoseidonRoundKeys::new(27)).id());
        let poseidonroundkeys_28 = eval.get_preprocessed_column((PoseidonRoundKeys::new(28)).id());
        let poseidonroundkeys_29 = eval.get_preprocessed_column((PoseidonRoundKeys::new(29)).id());
        let seq = eval.get_preprocessed_column(Seq::new(self.log_size()).id());
        let multiplicity = eval.next_trace_mask();

        eval.add_to_relation(RelationEntry::new(
            &self.poseidon_round_keys_lookup_elements,
            -E::EF::from(multiplicity),
            &[
                seq.clone(),
                poseidonroundkeys_0.clone(),
                poseidonroundkeys_1.clone(),
                poseidonroundkeys_2.clone(),
                poseidonroundkeys_3.clone(),
                poseidonroundkeys_4.clone(),
                poseidonroundkeys_5.clone(),
                poseidonroundkeys_6.clone(),
                poseidonroundkeys_7.clone(),
                poseidonroundkeys_8.clone(),
                poseidonroundkeys_9.clone(),
                poseidonroundkeys_10.clone(),
                poseidonroundkeys_11.clone(),
                poseidonroundkeys_12.clone(),
                poseidonroundkeys_13.clone(),
                poseidonroundkeys_14.clone(),
                poseidonroundkeys_15.clone(),
                poseidonroundkeys_16.clone(),
                poseidonroundkeys_17.clone(),
                poseidonroundkeys_18.clone(),
                poseidonroundkeys_19.clone(),
                poseidonroundkeys_20.clone(),
                poseidonroundkeys_21.clone(),
                poseidonroundkeys_22.clone(),
                poseidonroundkeys_23.clone(),
                poseidonroundkeys_24.clone(),
                poseidonroundkeys_25.clone(),
                poseidonroundkeys_26.clone(),
                poseidonroundkeys_27.clone(),
                poseidonroundkeys_28.clone(),
                poseidonroundkeys_29.clone(),
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
