// This file was created by the AIR team.

use crate::components::prelude::*;
use crate::components::subroutines::read_id::ReadId;

pub const N_TRACE_COLUMNS: usize = 3;
pub const RELATION_USES_PER_ROW: [RelationUse; 2] = [
    RelationUse {
        relation_id: "MemoryAddressToId",
        uses: 3,
    },
    RelationUse {
        relation_id: "PedersenAggregator",
        uses: 1,
    },
];

pub struct Eval {
    pub claim: Claim,
    pub memory_address_to_id_lookup_elements: relations::MemoryAddressToId,
    pub pedersen_aggregator_lookup_elements: relations::PedersenAggregator,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct Claim {
    pub log_size: u32,
    pub pedersen_builtin_segment_start: u32,
}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let trace_log_sizes = vec![self.log_size; N_TRACE_COLUMNS];
        let interaction_log_sizes = vec![self.log_size; SECURE_EXTENSION_DEGREE * 2];
        TreeVec::new(vec![vec![], trace_log_sizes, interaction_log_sizes])
    }

    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_u64(self.log_size as u64);
        channel.mix_u64(self.pedersen_builtin_segment_start as u64);
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
        let M31_1 = E::F::from(M31::from(1));
        let M31_2 = E::F::from(M31::from(2));
        let M31_3 = E::F::from(M31::from(3));
        let seq = eval.get_preprocessed_column(Seq::new(self.log_size()).id());
        let input_state_0_id_col0 = eval.next_trace_mask();
        let input_state_1_id_col1 = eval.next_trace_mask();
        let output_state_id_col2 = eval.next_trace_mask();

        let instance_addr_tmp_d00c6_0 = eval.add_intermediate(
            ((seq.clone() * M31_3.clone())
                + E::F::from(M31::from(self.claim.pedersen_builtin_segment_start))),
        );
        ReadId::evaluate(
            [instance_addr_tmp_d00c6_0.clone()],
            input_state_0_id_col0.clone(),
            &self.memory_address_to_id_lookup_elements,
            &mut eval,
        );
        ReadId::evaluate(
            [(instance_addr_tmp_d00c6_0.clone() + M31_1.clone())],
            input_state_1_id_col1.clone(),
            &self.memory_address_to_id_lookup_elements,
            &mut eval,
        );
        ReadId::evaluate(
            [(instance_addr_tmp_d00c6_0.clone() + M31_2.clone())],
            output_state_id_col2.clone(),
            &self.memory_address_to_id_lookup_elements,
            &mut eval,
        );
        eval.add_to_relation(RelationEntry::new(
            &self.pedersen_aggregator_lookup_elements,
            E::EF::one(),
            &[
                input_state_0_id_col0.clone(),
                input_state_1_id_col1.clone(),
                output_state_id_col2.clone(),
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
    use crate::components::constraints_regression_test_values::PEDERSEN_BUILTIN;

    #[test]
    fn pedersen_builtin_constraints_regression() {
        let mut rng = SmallRng::seed_from_u64(0);
        let eval = Eval {
            claim: Claim {
                log_size: 4,
                pedersen_builtin_segment_start: rng.gen::<u32>(),
            },
            memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
            pedersen_aggregator_lookup_elements: relations::PedersenAggregator::dummy(),
        };
        let expr_eval = eval.evaluate(ExprEvaluator::new());
        let assignment = expr_eval.random_assignment();

        let mut sum = QM31::zero();
        for c in expr_eval.constraints {
            sum += c.assign(&assignment) * rng.gen::<QM31>();
        }

        assert_eq!(sum, PEDERSEN_BUILTIN);
    }
}
