// This file was created by the AIR team.

use crate::components::prelude::*;
use crate::components::subroutines::decode_instruction_7ebc4::DecodeInstruction7Ebc4;
use crate::components::subroutines::read_small::ReadSmall;

pub const N_TRACE_COLUMNS: usize = 13;
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
        let ap_update_add_1_col3 = eval.next_trace_mask();
        let next_pc_id_col4 = eval.next_trace_mask();
        let msb_col5 = eval.next_trace_mask();
        let mid_limbs_set_col6 = eval.next_trace_mask();
        let next_pc_limb_0_col7 = eval.next_trace_mask();
        let next_pc_limb_1_col8 = eval.next_trace_mask();
        let next_pc_limb_2_col9 = eval.next_trace_mask();
        let remainder_bits_col10 = eval.next_trace_mask();
        let partial_limb_msb_col11 = eval.next_trace_mask();
        let enabler = eval.next_trace_mask();

        eval.add_constraint(enabler.clone() * enabler.clone() - enabler.clone());

        DecodeInstruction7Ebc4::evaluate(
            [input_pc_col0.clone()],
            ap_update_add_1_col3.clone(),
            &self.common_lookup_elements,
            &mut eval,
        );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [read_small_output_tmp_81a39_13_limb_0] = ReadSmall::evaluate(
            [(input_pc_col0.clone() + M31_1.clone())],
            next_pc_id_col4.clone(),
            msb_col5.clone(),
            mid_limbs_set_col6.clone(),
            next_pc_limb_0_col7.clone(),
            next_pc_limb_1_col8.clone(),
            next_pc_limb_2_col9.clone(),
            remainder_bits_col10.clone(),
            partial_limb_msb_col11.clone(),
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
                (input_pc_col0.clone() + read_small_output_tmp_81a39_13_limb_0.clone()),
                (input_ap_col1.clone() + ap_update_add_1_col3.clone()),
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
    use crate::components::constraints_regression_test_values::JUMP_OPCODE_REL_IMM;

    #[test]
    fn jump_opcode_rel_imm_constraints_regression() {
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

        JUMP_OPCODE_REL_IMM.assert_debug_eq(&sum);
    }
}
