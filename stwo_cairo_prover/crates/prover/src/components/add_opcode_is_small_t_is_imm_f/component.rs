#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use num_traits::{One, Zero};
use serde::{Deserialize, Serialize};
use stwo_cairo_serialize::CairoSerialize;
use stwo_prover::constraint_framework::logup::{LogupAtRow, LogupSums, LookupElements};
use stwo_prover::constraint_framework::{
    EvalAtRow, FrameworkComponent, FrameworkEval, RelationEntry,
};
use stwo_prover::core::backend::simd::m31::LOG_N_LANES;
use stwo_prover::core::channel::Channel;
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::fields::qm31::SecureField;
use stwo_prover::core::fields::secure_column::SECURE_EXTENSION_DEGREE;
use stwo_prover::core::pcs::TreeVec;

use crate::relations;

pub struct Eval {
    pub claim: Claim,
    pub memory_address_to_id_lookup_elements: relations::MemoryAddressToId,
    pub memory_id_to_big_lookup_elements: relations::MemoryIdToBig,
    pub opcodes_lookup_elements: relations::Opcodes,
    pub verify_instruction_lookup_elements: relations::VerifyInstruction,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct Claim {
    pub n_calls: usize,
}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let log_size = std::cmp::max(self.n_calls.next_power_of_two().ilog2(), LOG_N_LANES);
        let trace_log_sizes = vec![log_size; 32];
        let interaction_log_sizes = vec![log_size; SECURE_EXTENSION_DEGREE * 5];
        let preprocessed_log_sizes = vec![log_size];
        TreeVec::new(vec![
            preprocessed_log_sizes,
            trace_log_sizes,
            interaction_log_sizes,
        ])
    }

    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_u64(self.n_calls as u64);
    }
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct InteractionClaim {
    pub logup_sums: LogupSums,
}
impl InteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        let (total_sum, claimed_sum) = self.logup_sums;
        channel.mix_felts(&[total_sum]);
        if let Some(claimed_sum) = claimed_sum {
            channel.mix_felts(&[claimed_sum.0]);
            channel.mix_u64(claimed_sum.1 as u64);
        }
    }
}

pub type Component = FrameworkComponent<Eval>;

impl FrameworkEval for Eval {
    fn log_size(&self) -> u32 {
        std::cmp::max(self.claim.n_calls.next_power_of_two().ilog2(), LOG_N_LANES)
    }

    fn max_constraint_log_degree_bound(&self) -> u32 {
        self.log_size() + 1
    }

    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    fn evaluate<E: EvalAtRow>(&self, mut eval: E) -> E {
        let M31_0 = E::F::from(M31::from(0));
        let M31_1 = E::F::from(M31::from(1));
        let M31_134217728 = E::F::from(M31::from(134217728));
        let M31_136 = E::F::from(M31::from(136));
        let M31_256 = E::F::from(M31::from(256));
        let M31_262144 = E::F::from(M31::from(262144));
        let M31_32768 = E::F::from(M31::from(32768));
        let M31_511 = E::F::from(M31::from(511));
        let M31_512 = E::F::from(M31::from(512));
        let input_pc_col0 = eval.next_trace_mask();
        let input_ap_col1 = eval.next_trace_mask();
        let input_fp_col2 = eval.next_trace_mask();
        let offset0_col3 = eval.next_trace_mask();
        let offset1_col4 = eval.next_trace_mask();
        let offset2_col5 = eval.next_trace_mask();
        let dst_base_fp_col6 = eval.next_trace_mask();
        let op0_base_fp_col7 = eval.next_trace_mask();
        let op1_base_fp_col8 = eval.next_trace_mask();
        let op1_base_ap_col9 = eval.next_trace_mask();
        let ap_update_add_1_col10 = eval.next_trace_mask();
        let mem_dst_base_col11 = eval.next_trace_mask();
        let mem0_base_col12 = eval.next_trace_mask();
        let mem1_base_col13 = eval.next_trace_mask();
        let dst_id_col14 = eval.next_trace_mask();
        let msb_col15 = eval.next_trace_mask();
        let mid_limbs_set_col16 = eval.next_trace_mask();
        let dst_limb_0_col17 = eval.next_trace_mask();
        let dst_limb_1_col18 = eval.next_trace_mask();
        let dst_limb_2_col19 = eval.next_trace_mask();
        let op0_id_col20 = eval.next_trace_mask();
        let msb_col21 = eval.next_trace_mask();
        let mid_limbs_set_col22 = eval.next_trace_mask();
        let op0_limb_0_col23 = eval.next_trace_mask();
        let op0_limb_1_col24 = eval.next_trace_mask();
        let op0_limb_2_col25 = eval.next_trace_mask();
        let op1_id_col26 = eval.next_trace_mask();
        let msb_col27 = eval.next_trace_mask();
        let mid_limbs_set_col28 = eval.next_trace_mask();
        let op1_limb_0_col29 = eval.next_trace_mask();
        let op1_limb_1_col30 = eval.next_trace_mask();
        let op1_limb_2_col31 = eval.next_trace_mask();

        // Decode Instruction.

        eval.add_to_relation(RelationEntry::new(
            &self.verify_instruction_lookup_elements,
            E::EF::one(),
            &[
                input_pc_col0.clone(),
                offset0_col3.clone(),
                offset1_col4.clone(),
                offset2_col5.clone(),
                dst_base_fp_col6.clone(),
                op0_base_fp_col7.clone(),
                M31_0.clone(),
                op1_base_fp_col8.clone(),
                op1_base_ap_col9.clone(),
                M31_1.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                ap_update_add_1_col10.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_1.clone(),
            ],
        ));

        // mem_dst_base.
        eval.add_constraint(
            (mem_dst_base_col11.clone()
                - ((dst_base_fp_col6.clone() * input_fp_col2.clone())
                    + ((M31_1.clone() - dst_base_fp_col6.clone()) * input_ap_col1.clone()))),
        );
        // mem0_base.
        eval.add_constraint(
            (mem0_base_col12.clone()
                - ((op0_base_fp_col7.clone() * input_fp_col2.clone())
                    + ((M31_1.clone() - op0_base_fp_col7.clone()) * input_ap_col1.clone()))),
        );
        // Either flag op1_base_fp is on or flag op1_base_ap is on.
        eval.add_constraint(
            ((op1_base_fp_col8.clone() + op1_base_ap_col9.clone()) - M31_1.clone()),
        );
        // mem1_base.
        eval.add_constraint(
            (mem1_base_col13.clone()
                - ((op1_base_fp_col8.clone() * input_fp_col2.clone())
                    + (op1_base_ap_col9.clone() * input_ap_col1.clone()))),
        );

        // Read Small.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (mem_dst_base_col11.clone() + (offset0_col3.clone() - M31_32768.clone())),
                dst_id_col14.clone(),
            ],
        ));

        // Cond Decode Small Sign.

        // msb is a bit.
        eval.add_constraint((msb_col15.clone() * (msb_col15.clone() - M31_1.clone())));
        // mid_limbs_set is a bit.
        eval.add_constraint(
            (mid_limbs_set_col16.clone() * (mid_limbs_set_col16.clone() - M31_1.clone())),
        );
        // Cannot have msb equals 0 and mid_limbs_set equals 1.
        eval.add_constraint(
            ((M31_1.clone() * mid_limbs_set_col16.clone()) * (msb_col15.clone() - M31_1.clone())),
        );

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                dst_id_col14.clone(),
                dst_limb_0_col17.clone(),
                dst_limb_1_col18.clone(),
                dst_limb_2_col19.clone(),
                (mid_limbs_set_col16.clone() * M31_511.clone()),
                (mid_limbs_set_col16.clone() * M31_511.clone()),
                (mid_limbs_set_col16.clone() * M31_511.clone()),
                (mid_limbs_set_col16.clone() * M31_511.clone()),
                (mid_limbs_set_col16.clone() * M31_511.clone()),
                (mid_limbs_set_col16.clone() * M31_511.clone()),
                (mid_limbs_set_col16.clone() * M31_511.clone()),
                (mid_limbs_set_col16.clone() * M31_511.clone()),
                (mid_limbs_set_col16.clone() * M31_511.clone()),
                (mid_limbs_set_col16.clone() * M31_511.clone()),
                (mid_limbs_set_col16.clone() * M31_511.clone()),
                (mid_limbs_set_col16.clone() * M31_511.clone()),
                (mid_limbs_set_col16.clone() * M31_511.clone()),
                (mid_limbs_set_col16.clone() * M31_511.clone()),
                (mid_limbs_set_col16.clone() * M31_511.clone()),
                (mid_limbs_set_col16.clone() * M31_511.clone()),
                (mid_limbs_set_col16.clone() * M31_511.clone()),
                (mid_limbs_set_col16.clone() * M31_511.clone()),
                ((M31_136.clone() * msb_col15.clone()) - mid_limbs_set_col16.clone()),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                (msb_col15.clone() * M31_256.clone()),
            ],
        ));

        // Read Small.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (mem0_base_col12.clone() + (offset1_col4.clone() - M31_32768.clone())),
                op0_id_col20.clone(),
            ],
        ));

        // Cond Decode Small Sign.

        // msb is a bit.
        eval.add_constraint((msb_col21.clone() * (msb_col21.clone() - M31_1.clone())));
        // mid_limbs_set is a bit.
        eval.add_constraint(
            (mid_limbs_set_col22.clone() * (mid_limbs_set_col22.clone() - M31_1.clone())),
        );
        // Cannot have msb equals 0 and mid_limbs_set equals 1.
        eval.add_constraint(
            ((M31_1.clone() * mid_limbs_set_col22.clone()) * (msb_col21.clone() - M31_1.clone())),
        );

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                op0_id_col20.clone(),
                op0_limb_0_col23.clone(),
                op0_limb_1_col24.clone(),
                op0_limb_2_col25.clone(),
                (mid_limbs_set_col22.clone() * M31_511.clone()),
                (mid_limbs_set_col22.clone() * M31_511.clone()),
                (mid_limbs_set_col22.clone() * M31_511.clone()),
                (mid_limbs_set_col22.clone() * M31_511.clone()),
                (mid_limbs_set_col22.clone() * M31_511.clone()),
                (mid_limbs_set_col22.clone() * M31_511.clone()),
                (mid_limbs_set_col22.clone() * M31_511.clone()),
                (mid_limbs_set_col22.clone() * M31_511.clone()),
                (mid_limbs_set_col22.clone() * M31_511.clone()),
                (mid_limbs_set_col22.clone() * M31_511.clone()),
                (mid_limbs_set_col22.clone() * M31_511.clone()),
                (mid_limbs_set_col22.clone() * M31_511.clone()),
                (mid_limbs_set_col22.clone() * M31_511.clone()),
                (mid_limbs_set_col22.clone() * M31_511.clone()),
                (mid_limbs_set_col22.clone() * M31_511.clone()),
                (mid_limbs_set_col22.clone() * M31_511.clone()),
                (mid_limbs_set_col22.clone() * M31_511.clone()),
                (mid_limbs_set_col22.clone() * M31_511.clone()),
                ((M31_136.clone() * msb_col21.clone()) - mid_limbs_set_col22.clone()),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                (msb_col21.clone() * M31_256.clone()),
            ],
        ));

        // Read Small.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (mem1_base_col13.clone() + (offset2_col5.clone() - M31_32768.clone())),
                op1_id_col26.clone(),
            ],
        ));

        // Cond Decode Small Sign.

        // msb is a bit.
        eval.add_constraint((msb_col27.clone() * (msb_col27.clone() - M31_1.clone())));
        // mid_limbs_set is a bit.
        eval.add_constraint(
            (mid_limbs_set_col28.clone() * (mid_limbs_set_col28.clone() - M31_1.clone())),
        );
        // Cannot have msb equals 0 and mid_limbs_set equals 1.
        eval.add_constraint(
            ((M31_1.clone() * mid_limbs_set_col28.clone()) * (msb_col27.clone() - M31_1.clone())),
        );

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                op1_id_col26.clone(),
                op1_limb_0_col29.clone(),
                op1_limb_1_col30.clone(),
                op1_limb_2_col31.clone(),
                (mid_limbs_set_col28.clone() * M31_511.clone()),
                (mid_limbs_set_col28.clone() * M31_511.clone()),
                (mid_limbs_set_col28.clone() * M31_511.clone()),
                (mid_limbs_set_col28.clone() * M31_511.clone()),
                (mid_limbs_set_col28.clone() * M31_511.clone()),
                (mid_limbs_set_col28.clone() * M31_511.clone()),
                (mid_limbs_set_col28.clone() * M31_511.clone()),
                (mid_limbs_set_col28.clone() * M31_511.clone()),
                (mid_limbs_set_col28.clone() * M31_511.clone()),
                (mid_limbs_set_col28.clone() * M31_511.clone()),
                (mid_limbs_set_col28.clone() * M31_511.clone()),
                (mid_limbs_set_col28.clone() * M31_511.clone()),
                (mid_limbs_set_col28.clone() * M31_511.clone()),
                (mid_limbs_set_col28.clone() * M31_511.clone()),
                (mid_limbs_set_col28.clone() * M31_511.clone()),
                (mid_limbs_set_col28.clone() * M31_511.clone()),
                (mid_limbs_set_col28.clone() * M31_511.clone()),
                (mid_limbs_set_col28.clone() * M31_511.clone()),
                ((M31_136.clone() * msb_col27.clone()) - mid_limbs_set_col28.clone()),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                (msb_col27.clone() * M31_256.clone()),
            ],
        ));

        // dst equals op0 + op1.
        eval.add_constraint(
            (((((dst_limb_0_col17.clone() + (dst_limb_1_col18.clone() * M31_512.clone()))
                + (dst_limb_2_col19.clone() * M31_262144.clone()))
                - msb_col15.clone())
                - (M31_134217728.clone() * mid_limbs_set_col16.clone()))
                - (((((op0_limb_0_col23.clone()
                    + (op0_limb_1_col24.clone() * M31_512.clone()))
                    + (op0_limb_2_col25.clone() * M31_262144.clone()))
                    - msb_col21.clone())
                    - (M31_134217728.clone() * mid_limbs_set_col22.clone()))
                    + ((((op1_limb_0_col29.clone()
                        + (op1_limb_1_col30.clone() * M31_512.clone()))
                        + (op1_limb_2_col31.clone() * M31_262144.clone()))
                        - msb_col27.clone())
                        - (M31_134217728.clone() * mid_limbs_set_col28.clone())))),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.opcodes_lookup_elements,
            E::EF::one(),
            &[
                input_pc_col0.clone(),
                input_ap_col1.clone(),
                input_fp_col2.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.opcodes_lookup_elements,
            -E::EF::one(),
            &[
                (input_pc_col0.clone() + M31_1.clone()),
                (input_ap_col1.clone() + ap_update_add_1_col10.clone()),
                input_fp_col2.clone(),
            ],
        ));

        eval.finalize_logup_in_pairs();
        eval
    }
}
