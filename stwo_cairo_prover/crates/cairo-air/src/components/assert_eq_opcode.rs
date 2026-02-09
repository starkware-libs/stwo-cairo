// This file was created by the AIR team.

use crate::components::prelude::*;
use crate::components::subroutines::decode_instruction_fe864::DecodeInstructionFe864;
use crate::components::subroutines::mem_verify_equal::MemVerifyEqual;

pub const N_TRACE_COLUMNS: usize = 12;
pub const RELATION_USES_PER_ROW: [RelationUse; 3] = [
    RelationUse {
        relation_id: "MemoryAddressToId",
        uses: 2,
    },
    RelationUse {
        relation_id: "Opcodes",
        uses: 1,
    },
    RelationUse {
        relation_id: "VerifyInstruction",
        uses: 1,
    },
];

pub struct Eval {
    pub log_size: u32,
    pub common_lookup_elements: relations::CommonLookupElements,
}

pub fn log_sizes(log_size: u32) -> TreeVec<Vec<u32>> {
    let trace_log_sizes = vec![log_size; N_TRACE_COLUMNS];
    let interaction_log_sizes = vec![log_size; SECURE_EXTENSION_DEGREE * 3];
    TreeVec::new(vec![vec![], trace_log_sizes, interaction_log_sizes])
}

pub type Component = FrameworkComponent<Eval>;

impl FrameworkEval for Eval {
    fn log_size(&self) -> u32 {
        self.log_size
    }

    fn max_constraint_log_degree_bound(&self) -> u32 {
        self.log_size() + 1
    }

    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    fn evaluate<E: EvalAtRow>(&self, mut eval: E) -> E {
        let M31_1 = E::F::from(M31::from(1));
        let M31_428564188 = E::F::from(M31::from(428564188));
        let input_pc_col0 = eval.next_trace_mask();
        let input_ap_col1 = eval.next_trace_mask();
        let input_fp_col2 = eval.next_trace_mask();
        let offset0_col3 = eval.next_trace_mask();
        let offset2_col4 = eval.next_trace_mask();
        let dst_base_fp_col5 = eval.next_trace_mask();
        let op1_base_fp_col6 = eval.next_trace_mask();
        let ap_update_add_1_col7 = eval.next_trace_mask();
        let mem_dst_base_col8 = eval.next_trace_mask();
        let mem1_base_col9 = eval.next_trace_mask();
        let dst_id_col10 = eval.next_trace_mask();
        let enabler = eval.next_trace_mask();

        eval.add_constraint(enabler.clone() * enabler.clone() - enabler.clone());

        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [decode_instruction_fe864_output_tmp_d6f03_7_offset0, decode_instruction_fe864_output_tmp_d6f03_7_offset2, decode_instruction_fe864_output_tmp_d6f03_7_op1_base_ap] =
            DecodeInstructionFe864::evaluate(
                [input_pc_col0.clone()],
                offset0_col3.clone(),
                offset2_col4.clone(),
                dst_base_fp_col5.clone(),
                op1_base_fp_col6.clone(),
                ap_update_add_1_col7.clone(),
                &self.common_lookup_elements,
                &mut eval,
            );
        // mem_dst_base.
        eval.add_constraint(
            (mem_dst_base_col8.clone()
                - ((dst_base_fp_col5.clone() * input_fp_col2.clone())
                    + ((M31_1.clone() - dst_base_fp_col5.clone()) * input_ap_col1.clone()))),
        );
        // mem1_base.
        eval.add_constraint(
            (mem1_base_col9.clone()
                - ((op1_base_fp_col6.clone() * input_fp_col2.clone())
                    + (decode_instruction_fe864_output_tmp_d6f03_7_op1_base_ap.clone()
                        * input_ap_col1.clone()))),
        );
        MemVerifyEqual::evaluate(
            [
                (mem_dst_base_col8.clone()
                    + decode_instruction_fe864_output_tmp_d6f03_7_offset0.clone()),
                (mem1_base_col9.clone()
                    + decode_instruction_fe864_output_tmp_d6f03_7_offset2.clone()),
            ],
            dst_id_col10.clone(),
            &self.common_lookup_elements,
            &mut eval,
        );
        eval.add_to_relation(RelationEntry::new(
            &self.common_lookup_elements,
            E::EF::from(enabler.clone()),
            &[
                M31_428564188.clone(),
                input_pc_col0.clone(),
                input_ap_col1.clone(),
                input_fp_col2.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.common_lookup_elements,
            -E::EF::from(enabler.clone()),
            &[
                M31_428564188.clone(),
                (input_pc_col0.clone() + M31_1.clone()),
                (input_ap_col1.clone() + ap_update_add_1_col7.clone()),
                input_fp_col2.clone(),
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
    use crate::components::constraints_regression_test_values::ASSERT_EQ_OPCODE;

    #[test]
    fn assert_eq_opcode_constraints_regression() {
        let mut rng = SmallRng::seed_from_u64(0);
        let eval = Eval {
            log_size: 4,
            common_lookup_elements: relations::CommonLookupElements::dummy(),
        };
        let expr_eval = eval.evaluate(ExprEvaluator::new());
        let assignment = expr_eval.random_assignment();

        let mut sum = QM31::zero();
        for c in expr_eval.constraints {
            sum += c.assign(&assignment) * rng.gen::<QM31>();
        }

        ASSERT_EQ_OPCODE.assert_debug_eq(&sum);
    }
}
