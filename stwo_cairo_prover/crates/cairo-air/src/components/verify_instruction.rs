// This file was created by the AIR team.

use crate::components::prelude::*;
use crate::components::subroutines::encode_offsets::EncodeOffsets;
use crate::components::subroutines::mem_verify::MemVerify;

pub const N_TRACE_COLUMNS: usize = 17;
pub const RELATION_USES_PER_ROW: [RelationUse; 4] = [
    RelationUse {
        relation_id: "MemoryAddressToId",
        uses: 1,
    },
    RelationUse {
        relation_id: "MemoryIdToBig",
        uses: 1,
    },
    RelationUse {
        relation_id: "RangeCheck_4_3",
        uses: 1,
    },
    RelationUse {
        relation_id: "RangeCheck_7_2_5",
        uses: 1,
    },
];

pub struct Eval {
    pub claim: Claim,
    pub range_check_7_2_5_lookup_elements: relations::RangeCheck_7_2_5,
    pub range_check_4_3_lookup_elements: relations::RangeCheck_4_3,
    pub memory_address_to_id_lookup_elements: relations::MemoryAddressToId,
    pub memory_id_to_big_lookup_elements: relations::MemoryIdToBig,
    pub verify_instruction_lookup_elements: relations::VerifyInstruction,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct Claim {
    pub log_size: u32,
}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let trace_log_sizes = vec![self.log_size; N_TRACE_COLUMNS];
        let interaction_log_sizes = vec![self.log_size; SECURE_EXTENSION_DEGREE * 3];
        TreeVec::new(vec![vec![], trace_log_sizes, interaction_log_sizes])
    }

    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_u64(self.log_size as u64);
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
        let M31_0 = E::F::from(M31::from(0));
        let input_pc_col0 = eval.next_trace_mask();
        let input_offset0_col1 = eval.next_trace_mask();
        let input_offset1_col2 = eval.next_trace_mask();
        let input_offset2_col3 = eval.next_trace_mask();
        let input_inst_felt5_high_col4 = eval.next_trace_mask();
        let input_inst_felt6_col5 = eval.next_trace_mask();
        let input_opcode_extension_col6 = eval.next_trace_mask();
        let offset0_low_col7 = eval.next_trace_mask();
        let offset0_mid_col8 = eval.next_trace_mask();
        let offset1_low_col9 = eval.next_trace_mask();
        let offset1_mid_col10 = eval.next_trace_mask();
        let offset1_high_col11 = eval.next_trace_mask();
        let offset2_low_col12 = eval.next_trace_mask();
        let offset2_mid_col13 = eval.next_trace_mask();
        let offset2_high_col14 = eval.next_trace_mask();
        let instruction_id_col15 = eval.next_trace_mask();
        let multiplicity = eval.next_trace_mask();

        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [encode_offsets_output_tmp_16a4f_8_limb_1, encode_offsets_output_tmp_16a4f_8_limb_3] =
            EncodeOffsets::evaluate(
                [
                    input_offset0_col1.clone(),
                    input_offset1_col2.clone(),
                    input_offset2_col3.clone(),
                ],
                offset0_low_col7.clone(),
                offset0_mid_col8.clone(),
                offset1_low_col9.clone(),
                offset1_mid_col10.clone(),
                offset1_high_col11.clone(),
                offset2_low_col12.clone(),
                offset2_mid_col13.clone(),
                offset2_high_col14.clone(),
                &self.range_check_7_2_5_lookup_elements,
                &self.range_check_4_3_lookup_elements,
                &mut eval,
            );
        MemVerify::evaluate(
            [
                input_pc_col0.clone(),
                offset0_low_col7.clone(),
                encode_offsets_output_tmp_16a4f_8_limb_1.clone(),
                offset1_mid_col10.clone(),
                encode_offsets_output_tmp_16a4f_8_limb_3.clone(),
                offset2_mid_col13.clone(),
                (offset2_high_col14.clone() + input_inst_felt5_high_col4.clone()),
                input_inst_felt6_col5.clone(),
                input_opcode_extension_col6.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
            ],
            instruction_id_col15.clone(),
            &self.memory_address_to_id_lookup_elements,
            &self.memory_id_to_big_lookup_elements,
            &mut eval,
        );
        eval.add_to_relation(RelationEntry::new(
            &self.verify_instruction_lookup_elements,
            -E::EF::from(multiplicity),
            &[
                input_pc_col0.clone(),
                input_offset0_col1.clone(),
                input_offset1_col2.clone(),
                input_offset2_col3.clone(),
                input_inst_felt5_high_col4.clone(),
                input_inst_felt6_col5.clone(),
                input_opcode_extension_col6.clone(),
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
    use crate::components::constraints_regression_test_values::VERIFY_INSTRUCTION;

    #[test]
    fn verify_instruction_constraints_regression() {
        let mut rng = SmallRng::seed_from_u64(0);
        let eval = Eval {
            claim: Claim { log_size: 4 },
            range_check_7_2_5_lookup_elements: relations::RangeCheck_7_2_5::dummy(),
            range_check_4_3_lookup_elements: relations::RangeCheck_4_3::dummy(),
            memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
            memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
            verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
        };
        let expr_eval = eval.evaluate(ExprEvaluator::new());
        let assignment = expr_eval.random_assignment();

        let mut sum = QM31::zero();
        for c in expr_eval.constraints {
            sum += c.assign(&assignment) * rng.gen::<QM31>();
        }

        assert_eq!(sum, VERIFY_INSTRUCTION);
    }
}
