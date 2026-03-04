// This file was created by the AIR team.

use crate::components::prelude::*;
use crate::components::subroutines::mem_verify::MemVerify;

pub const N_TRACE_COLUMNS: usize = 1;
pub const RELATION_USES_PER_ROW: [RelationUse; 2] = [
    RelationUse {
        relation_id: "MemoryAddressToId",
        uses: 1,
    },
    RelationUse {
        relation_id: "MemoryIdToBig",
        uses: 1,
    },
];

pub struct Eval {
    pub claim: Claim,
    pub common_lookup_elements: relations::CommonLookupElements,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct Claim {
    pub log_size: u32,
    pub verify_program_segment_start: u32,
}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let trace_log_sizes = vec![self.log_size; N_TRACE_COLUMNS];
        let interaction_log_sizes = vec![self.log_size; SECURE_EXTENSION_DEGREE];
        TreeVec::new(vec![vec![], trace_log_sizes, interaction_log_sizes])
    }

    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_u64(self.log_size as u64);
        channel.mix_u64(self.verify_program_segment_start as u64);
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
        let seq = eval.get_preprocessed_column(Seq::new(self.log_size()).id());
        let curr_program_0 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "curr_program_0".to_owned(),
        });
        let curr_program_1 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "curr_program_1".to_owned(),
        });
        let curr_program_2 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "curr_program_2".to_owned(),
        });
        let curr_program_3 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "curr_program_3".to_owned(),
        });
        let curr_program_4 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "curr_program_4".to_owned(),
        });
        let curr_program_5 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "curr_program_5".to_owned(),
        });
        let curr_program_6 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "curr_program_6".to_owned(),
        });
        let curr_program_7 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "curr_program_7".to_owned(),
        });
        let curr_program_8 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "curr_program_8".to_owned(),
        });
        let curr_program_9 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "curr_program_9".to_owned(),
        });
        let curr_program_10 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "curr_program_10".to_owned(),
        });
        let curr_program_11 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "curr_program_11".to_owned(),
        });
        let curr_program_12 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "curr_program_12".to_owned(),
        });
        let curr_program_13 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "curr_program_13".to_owned(),
        });
        let curr_program_14 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "curr_program_14".to_owned(),
        });
        let curr_program_15 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "curr_program_15".to_owned(),
        });
        let curr_program_16 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "curr_program_16".to_owned(),
        });
        let curr_program_17 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "curr_program_17".to_owned(),
        });
        let curr_program_18 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "curr_program_18".to_owned(),
        });
        let curr_program_19 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "curr_program_19".to_owned(),
        });
        let curr_program_20 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "curr_program_20".to_owned(),
        });
        let curr_program_21 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "curr_program_21".to_owned(),
        });
        let curr_program_22 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "curr_program_22".to_owned(),
        });
        let curr_program_23 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "curr_program_23".to_owned(),
        });
        let curr_program_24 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "curr_program_24".to_owned(),
        });
        let curr_program_25 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "curr_program_25".to_owned(),
        });
        let curr_program_26 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "curr_program_26".to_owned(),
        });
        let curr_program_27 = eval.get_preprocessed_column(PreProcessedColumnId {
            id: "curr_program_27".to_owned(),
        });
        let address_id_col0 = eval.next_trace_mask();

        MemVerify::evaluate(
            [
                (E::F::from(M31::from(self.claim.verify_program_segment_start)) + seq.clone()),
                curr_program_0.clone(),
                curr_program_1.clone(),
                curr_program_2.clone(),
                curr_program_3.clone(),
                curr_program_4.clone(),
                curr_program_5.clone(),
                curr_program_6.clone(),
                curr_program_7.clone(),
                curr_program_8.clone(),
                curr_program_9.clone(),
                curr_program_10.clone(),
                curr_program_11.clone(),
                curr_program_12.clone(),
                curr_program_13.clone(),
                curr_program_14.clone(),
                curr_program_15.clone(),
                curr_program_16.clone(),
                curr_program_17.clone(),
                curr_program_18.clone(),
                curr_program_19.clone(),
                curr_program_20.clone(),
                curr_program_21.clone(),
                curr_program_22.clone(),
                curr_program_23.clone(),
                curr_program_24.clone(),
                curr_program_25.clone(),
                curr_program_26.clone(),
                curr_program_27.clone(),
            ],
            address_id_col0.clone(),
            &self.common_lookup_elements,
            &mut eval,
        );
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
    use crate::components::constraints_regression_test_values::VERIFY_PROGRAM_SEGMENT;

    #[test]
    fn verify_program_segment_constraints_regression() {
        let mut rng = SmallRng::seed_from_u64(0);
        let eval = Eval {
            claim: Claim {
                log_size: 4,
                verify_program_segment_start: rng.gen::<u32>(),
            },
            common_lookup_elements: relations::CommonLookupElements::dummy(),
        };
        let expr_eval = eval.evaluate(ExprEvaluator::new());
        let assignment = expr_eval.random_assignment();

        let mut sum = QM31::zero();
        for c in expr_eval.constraints {
            sum += c.assign(&assignment) * rng.gen::<QM31>();
        }

        VERIFY_PROGRAM_SEGMENT.assert_debug_eq(&sum);
    }
}
