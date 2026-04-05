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
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct InteractionClaim {
    pub claimed_sum: SecureField,
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
        let curr_program_0 = eval.get_preprocessed_column(ProgramColumn::new(0).id());
        let curr_program_1 = eval.get_preprocessed_column(ProgramColumn::new(1).id());
        let curr_program_2 = eval.get_preprocessed_column(ProgramColumn::new(2).id());
        let curr_program_3 = eval.get_preprocessed_column(ProgramColumn::new(3).id());
        let curr_program_4 = eval.get_preprocessed_column(ProgramColumn::new(4).id());
        let curr_program_5 = eval.get_preprocessed_column(ProgramColumn::new(5).id());
        let curr_program_6 = eval.get_preprocessed_column(ProgramColumn::new(6).id());
        let curr_program_7 = eval.get_preprocessed_column(ProgramColumn::new(7).id());
        let curr_program_8 = eval.get_preprocessed_column(ProgramColumn::new(8).id());
        let curr_program_9 = eval.get_preprocessed_column(ProgramColumn::new(9).id());
        let curr_program_10 = eval.get_preprocessed_column(ProgramColumn::new(10).id());
        let curr_program_11 = eval.get_preprocessed_column(ProgramColumn::new(11).id());
        let curr_program_12 = eval.get_preprocessed_column(ProgramColumn::new(12).id());
        let curr_program_13 = eval.get_preprocessed_column(ProgramColumn::new(13).id());
        let curr_program_14 = eval.get_preprocessed_column(ProgramColumn::new(14).id());
        let curr_program_15 = eval.get_preprocessed_column(ProgramColumn::new(15).id());
        let curr_program_16 = eval.get_preprocessed_column(ProgramColumn::new(16).id());
        let curr_program_17 = eval.get_preprocessed_column(ProgramColumn::new(17).id());
        let curr_program_18 = eval.get_preprocessed_column(ProgramColumn::new(18).id());
        let curr_program_19 = eval.get_preprocessed_column(ProgramColumn::new(19).id());
        let curr_program_20 = eval.get_preprocessed_column(ProgramColumn::new(20).id());
        let curr_program_21 = eval.get_preprocessed_column(ProgramColumn::new(21).id());
        let curr_program_22 = eval.get_preprocessed_column(ProgramColumn::new(22).id());
        let curr_program_23 = eval.get_preprocessed_column(ProgramColumn::new(23).id());
        let curr_program_24 = eval.get_preprocessed_column(ProgramColumn::new(24).id());
        let curr_program_25 = eval.get_preprocessed_column(ProgramColumn::new(25).id());
        let curr_program_26 = eval.get_preprocessed_column(ProgramColumn::new(26).id());
        let curr_program_27 = eval.get_preprocessed_column(ProgramColumn::new(27).id());
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
    use crate::components::constraints_regression_test_values::VERIFY_PROGRAM;

    #[test]
    fn verify_program_constraints_regression() {
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

        VERIFY_PROGRAM.assert_debug_eq(&sum);
    }
}
