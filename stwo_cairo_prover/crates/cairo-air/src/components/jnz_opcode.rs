use crate::components::prelude::*;
use crate::components::subroutines::decode_instruction_de75a::DecodeInstructionDe75A;
use crate::components::subroutines::read_positive_num_bits_252::ReadPositiveNumBits252;

pub const N_TRACE_COLUMNS: usize = 37;
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
    pub claim: Claim,
    pub verify_instruction_lookup_elements: relations::VerifyInstruction,
    pub memory_address_to_id_lookup_elements: relations::MemoryAddressToId,
    pub memory_id_to_big_lookup_elements: relations::MemoryIdToBig,
    pub opcodes_lookup_elements: relations::Opcodes,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
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

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
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
        let input_pc_col0 = eval.next_trace_mask();
        let input_ap_col1 = eval.next_trace_mask();
        let input_fp_col2 = eval.next_trace_mask();
        let offset0_col3 = eval.next_trace_mask();
        let dst_base_fp_col4 = eval.next_trace_mask();
        let ap_update_add_1_col5 = eval.next_trace_mask();
        let mem_dst_base_col6 = eval.next_trace_mask();
        let dst_id_col7 = eval.next_trace_mask();
        let dst_limb_0_col8 = eval.next_trace_mask();
        let dst_limb_1_col9 = eval.next_trace_mask();
        let dst_limb_2_col10 = eval.next_trace_mask();
        let dst_limb_3_col11 = eval.next_trace_mask();
        let dst_limb_4_col12 = eval.next_trace_mask();
        let dst_limb_5_col13 = eval.next_trace_mask();
        let dst_limb_6_col14 = eval.next_trace_mask();
        let dst_limb_7_col15 = eval.next_trace_mask();
        let dst_limb_8_col16 = eval.next_trace_mask();
        let dst_limb_9_col17 = eval.next_trace_mask();
        let dst_limb_10_col18 = eval.next_trace_mask();
        let dst_limb_11_col19 = eval.next_trace_mask();
        let dst_limb_12_col20 = eval.next_trace_mask();
        let dst_limb_13_col21 = eval.next_trace_mask();
        let dst_limb_14_col22 = eval.next_trace_mask();
        let dst_limb_15_col23 = eval.next_trace_mask();
        let dst_limb_16_col24 = eval.next_trace_mask();
        let dst_limb_17_col25 = eval.next_trace_mask();
        let dst_limb_18_col26 = eval.next_trace_mask();
        let dst_limb_19_col27 = eval.next_trace_mask();
        let dst_limb_20_col28 = eval.next_trace_mask();
        let dst_limb_21_col29 = eval.next_trace_mask();
        let dst_limb_22_col30 = eval.next_trace_mask();
        let dst_limb_23_col31 = eval.next_trace_mask();
        let dst_limb_24_col32 = eval.next_trace_mask();
        let dst_limb_25_col33 = eval.next_trace_mask();
        let dst_limb_26_col34 = eval.next_trace_mask();
        let dst_limb_27_col35 = eval.next_trace_mask();
        let enabler = eval.next_trace_mask();

        eval.add_constraint(enabler.clone() * enabler.clone() - enabler.clone());

        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [decode_instruction_de75a_output_tmp_e1597_5_offset0] =
            DecodeInstructionDe75A::evaluate(
                [input_pc_col0.clone()],
                offset0_col3.clone(),
                dst_base_fp_col4.clone(),
                ap_update_add_1_col5.clone(),
                &self.verify_instruction_lookup_elements,
                &mut eval,
            );
        // mem_dst_base.
        eval.add_constraint(
            (mem_dst_base_col6.clone()
                - ((dst_base_fp_col4.clone() * input_fp_col2.clone())
                    + ((M31_1.clone() - dst_base_fp_col4.clone()) * input_ap_col1.clone()))),
        );
        ReadPositiveNumBits252::evaluate(
            [(mem_dst_base_col6.clone()
                + decode_instruction_de75a_output_tmp_e1597_5_offset0.clone())],
            dst_id_col7.clone(),
            dst_limb_0_col8.clone(),
            dst_limb_1_col9.clone(),
            dst_limb_2_col10.clone(),
            dst_limb_3_col11.clone(),
            dst_limb_4_col12.clone(),
            dst_limb_5_col13.clone(),
            dst_limb_6_col14.clone(),
            dst_limb_7_col15.clone(),
            dst_limb_8_col16.clone(),
            dst_limb_9_col17.clone(),
            dst_limb_10_col18.clone(),
            dst_limb_11_col19.clone(),
            dst_limb_12_col20.clone(),
            dst_limb_13_col21.clone(),
            dst_limb_14_col22.clone(),
            dst_limb_15_col23.clone(),
            dst_limb_16_col24.clone(),
            dst_limb_17_col25.clone(),
            dst_limb_18_col26.clone(),
            dst_limb_19_col27.clone(),
            dst_limb_20_col28.clone(),
            dst_limb_21_col29.clone(),
            dst_limb_22_col30.clone(),
            dst_limb_23_col31.clone(),
            dst_limb_24_col32.clone(),
            dst_limb_25_col33.clone(),
            dst_limb_26_col34.clone(),
            dst_limb_27_col35.clone(),
            &self.memory_address_to_id_lookup_elements,
            &self.memory_id_to_big_lookup_elements,
            &mut eval,
        );
        // dst equals 0.
        eval.add_constraint(
            (((((((((((((((((((((((((((dst_limb_0_col8.clone()
                + dst_limb_1_col9.clone())
                + dst_limb_2_col10.clone())
                + dst_limb_3_col11.clone())
                + dst_limb_4_col12.clone())
                + dst_limb_5_col13.clone())
                + dst_limb_6_col14.clone())
                + dst_limb_7_col15.clone())
                + dst_limb_8_col16.clone())
                + dst_limb_9_col17.clone())
                + dst_limb_10_col18.clone())
                + dst_limb_11_col19.clone())
                + dst_limb_12_col20.clone())
                + dst_limb_13_col21.clone())
                + dst_limb_14_col22.clone())
                + dst_limb_15_col23.clone())
                + dst_limb_16_col24.clone())
                + dst_limb_17_col25.clone())
                + dst_limb_18_col26.clone())
                + dst_limb_19_col27.clone())
                + dst_limb_20_col28.clone())
                + dst_limb_21_col29.clone())
                + dst_limb_22_col30.clone())
                + dst_limb_23_col31.clone())
                + dst_limb_24_col32.clone())
                + dst_limb_25_col33.clone())
                + dst_limb_26_col34.clone())
                + dst_limb_27_col35.clone()),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.opcodes_lookup_elements,
            E::EF::from(enabler.clone()),
            &[
                input_pc_col0.clone(),
                input_ap_col1.clone(),
                input_fp_col2.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.opcodes_lookup_elements,
            -E::EF::from(enabler.clone()),
            &[
                (input_pc_col0.clone() + M31_2.clone()),
                (input_ap_col1.clone() + ap_update_add_1_col5.clone()),
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
    use stwo_prover::constraint_framework::expr::ExprEvaluator;
    use stwo_prover::core::fields::qm31::QM31;

    use super::*;
    use crate::components::constraints_regression_test_values::JNZ_OPCODE;

    #[test]
    fn jnz_opcode_constraints_regression() {
        let mut rng = SmallRng::seed_from_u64(0);
        let eval = Eval {
            claim: Claim { log_size: 4 },
            verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
            memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
            memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
            opcodes_lookup_elements: relations::Opcodes::dummy(),
        };
        let expr_eval = eval.evaluate(ExprEvaluator::new());
        let assignment = expr_eval.random_assignment();

        let mut sum = QM31::zero();
        for c in expr_eval.constraints {
            sum += c.assign(&assignment) * rng.gen::<QM31>();
        }

        assert_eq!(sum, JNZ_OPCODE);
    }
}
