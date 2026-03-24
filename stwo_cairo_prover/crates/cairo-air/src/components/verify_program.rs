// This file was created by the AIR team.

use crate::components::prelude::*;
use crate::components::subroutines::mem_verify_cond::MemVerifyCond;

pub const N_TRACE_COLUMNS: usize = 30;
pub const RELATION_USES_PER_ROW: [RelationUse; 3] = [
    RelationUse {
        relation_id: "MemoryAddressToId",
        uses: 1,
    },
    RelationUse {
        relation_id: "MemoryIdToBig",
        uses: 1,
    },
    RelationUse {
        relation_id: "ProgramComponent",
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
        let interaction_log_sizes = vec![self.log_size; SECURE_EXTENSION_DEGREE * 2];
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
        let M31_1 = E::F::from(M31::from(1));
        let M31_1942035206 = E::F::from(M31::from(1942035206));
        let seq = eval.get_preprocessed_column(Seq::new(self.log_size()).id());
        let program_component_output_limb_0_col0 = eval.next_trace_mask();
        let program_component_output_limb_1_col1 = eval.next_trace_mask();
        let program_component_output_limb_2_col2 = eval.next_trace_mask();
        let program_component_output_limb_3_col3 = eval.next_trace_mask();
        let program_component_output_limb_4_col4 = eval.next_trace_mask();
        let program_component_output_limb_5_col5 = eval.next_trace_mask();
        let program_component_output_limb_6_col6 = eval.next_trace_mask();
        let program_component_output_limb_7_col7 = eval.next_trace_mask();
        let program_component_output_limb_8_col8 = eval.next_trace_mask();
        let program_component_output_limb_9_col9 = eval.next_trace_mask();
        let program_component_output_limb_10_col10 = eval.next_trace_mask();
        let program_component_output_limb_11_col11 = eval.next_trace_mask();
        let program_component_output_limb_12_col12 = eval.next_trace_mask();
        let program_component_output_limb_13_col13 = eval.next_trace_mask();
        let program_component_output_limb_14_col14 = eval.next_trace_mask();
        let program_component_output_limb_15_col15 = eval.next_trace_mask();
        let program_component_output_limb_16_col16 = eval.next_trace_mask();
        let program_component_output_limb_17_col17 = eval.next_trace_mask();
        let program_component_output_limb_18_col18 = eval.next_trace_mask();
        let program_component_output_limb_19_col19 = eval.next_trace_mask();
        let program_component_output_limb_20_col20 = eval.next_trace_mask();
        let program_component_output_limb_21_col21 = eval.next_trace_mask();
        let program_component_output_limb_22_col22 = eval.next_trace_mask();
        let program_component_output_limb_23_col23 = eval.next_trace_mask();
        let program_component_output_limb_24_col24 = eval.next_trace_mask();
        let program_component_output_limb_25_col25 = eval.next_trace_mask();
        let program_component_output_limb_26_col26 = eval.next_trace_mask();
        let program_component_output_limb_27_col27 = eval.next_trace_mask();
        let program_component_output_limb_28_col28 = eval.next_trace_mask();
        let address_id_col29 = eval.next_trace_mask();

        eval.add_to_relation(RelationEntry::new(
            &self.common_lookup_elements,
            E::EF::from(M31_1.clone()),
            &[
                M31_1942035206.clone(),
                seq.clone(),
                program_component_output_limb_0_col0.clone(),
                program_component_output_limb_1_col1.clone(),
                program_component_output_limb_2_col2.clone(),
                program_component_output_limb_3_col3.clone(),
                program_component_output_limb_4_col4.clone(),
                program_component_output_limb_5_col5.clone(),
                program_component_output_limb_6_col6.clone(),
                program_component_output_limb_7_col7.clone(),
                program_component_output_limb_8_col8.clone(),
                program_component_output_limb_9_col9.clone(),
                program_component_output_limb_10_col10.clone(),
                program_component_output_limb_11_col11.clone(),
                program_component_output_limb_12_col12.clone(),
                program_component_output_limb_13_col13.clone(),
                program_component_output_limb_14_col14.clone(),
                program_component_output_limb_15_col15.clone(),
                program_component_output_limb_16_col16.clone(),
                program_component_output_limb_17_col17.clone(),
                program_component_output_limb_18_col18.clone(),
                program_component_output_limb_19_col19.clone(),
                program_component_output_limb_20_col20.clone(),
                program_component_output_limb_21_col21.clone(),
                program_component_output_limb_22_col22.clone(),
                program_component_output_limb_23_col23.clone(),
                program_component_output_limb_24_col24.clone(),
                program_component_output_limb_25_col25.clone(),
                program_component_output_limb_26_col26.clone(),
                program_component_output_limb_27_col27.clone(),
                program_component_output_limb_28_col28.clone(),
            ],
        ));

        MemVerifyCond::evaluate(
            [
                (E::F::from(M31::from(self.claim.verify_program_segment_start)) + seq.clone()),
                program_component_output_limb_0_col0.clone(),
                program_component_output_limb_1_col1.clone(),
                program_component_output_limb_2_col2.clone(),
                program_component_output_limb_3_col3.clone(),
                program_component_output_limb_4_col4.clone(),
                program_component_output_limb_5_col5.clone(),
                program_component_output_limb_6_col6.clone(),
                program_component_output_limb_7_col7.clone(),
                program_component_output_limb_8_col8.clone(),
                program_component_output_limb_9_col9.clone(),
                program_component_output_limb_10_col10.clone(),
                program_component_output_limb_11_col11.clone(),
                program_component_output_limb_12_col12.clone(),
                program_component_output_limb_13_col13.clone(),
                program_component_output_limb_14_col14.clone(),
                program_component_output_limb_15_col15.clone(),
                program_component_output_limb_16_col16.clone(),
                program_component_output_limb_17_col17.clone(),
                program_component_output_limb_18_col18.clone(),
                program_component_output_limb_19_col19.clone(),
                program_component_output_limb_20_col20.clone(),
                program_component_output_limb_21_col21.clone(),
                program_component_output_limb_22_col22.clone(),
                program_component_output_limb_23_col23.clone(),
                program_component_output_limb_24_col24.clone(),
                program_component_output_limb_25_col25.clone(),
                program_component_output_limb_26_col26.clone(),
                program_component_output_limb_27_col27.clone(),
                program_component_output_limb_28_col28.clone(),
            ],
            address_id_col29.clone(),
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
