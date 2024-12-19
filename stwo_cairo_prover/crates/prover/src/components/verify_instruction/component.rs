#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use num_traits::{One, Zero};
use serde::{Deserialize, Serialize};
use starknet_ff::FieldElement;
use stwo_cairo_serialize::CairoSerialize;
use stwo_prover::constraint_framework::logup::{
    ClaimedPrefixSum, LogupAtRow, LogupSums, LookupElements,
};
use stwo_prover::constraint_framework::preprocessed_columns::PreprocessedColumn;
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
    pub range_check_4_3_lookup_elements: relations::RangeCheck_4_3,
    pub range_check_7_2_5_lookup_elements: relations::RangeCheck_7_2_5,
    pub verify_instruction_lookup_elements: relations::VerifyInstruction,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct Claim {
    pub log_size: u32,
}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        const N_LOOKUPS: usize = 5;
        let trace_log_sizes = vec![self.log_size; 29];
        let interaction_log_sizes =
            vec![self.log_size; SECURE_EXTENSION_DEGREE * N_LOOKUPS.div_ceil(2)];
        let preprocessed_log_sizes = vec![self.log_size];
        TreeVec::new(vec![
            preprocessed_log_sizes,
            trace_log_sizes,
            interaction_log_sizes,
        ])
    }

    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_u64(self.log_size as u64);
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
        let M31_1 = E::F::from(M31::from(1));
        let M31_128 = E::F::from(M31::from(128));
        let M31_16 = E::F::from(M31::from(16));
        let M31_2 = E::F::from(M31::from(2));
        let M31_2048 = E::F::from(M31::from(2048));
        let M31_256 = E::F::from(M31::from(256));
        let M31_32 = E::F::from(M31::from(32));
        let M31_4 = E::F::from(M31::from(4));
        let M31_512 = E::F::from(M31::from(512));
        let M31_64 = E::F::from(M31::from(64));
        let M31_8 = E::F::from(M31::from(8));
        let M31_8192 = E::F::from(M31::from(8192));
        let input_col0 = eval.next_trace_mask();
        let input_col1 = eval.next_trace_mask();
        let input_col2 = eval.next_trace_mask();
        let input_col3 = eval.next_trace_mask();
        let input_col4 = eval.next_trace_mask();
        let input_col5 = eval.next_trace_mask();
        let input_col6 = eval.next_trace_mask();
        let input_col7 = eval.next_trace_mask();
        let input_col8 = eval.next_trace_mask();
        let input_col9 = eval.next_trace_mask();
        let input_col10 = eval.next_trace_mask();
        let input_col11 = eval.next_trace_mask();
        let input_col12 = eval.next_trace_mask();
        let input_col13 = eval.next_trace_mask();
        let input_col14 = eval.next_trace_mask();
        let input_col15 = eval.next_trace_mask();
        let input_col16 = eval.next_trace_mask();
        let input_col17 = eval.next_trace_mask();
        let input_col18 = eval.next_trace_mask();
        let offset0_low_col19 = eval.next_trace_mask();
        let offset0_mid_col20 = eval.next_trace_mask();
        let offset1_low_col21 = eval.next_trace_mask();
        let offset1_mid_col22 = eval.next_trace_mask();
        let offset1_high_col23 = eval.next_trace_mask();
        let offset2_low_col24 = eval.next_trace_mask();
        let offset2_mid_col25 = eval.next_trace_mask();
        let offset2_high_col26 = eval.next_trace_mask();
        let instruction_id_col27 = eval.next_trace_mask();
        let mult = eval.next_trace_mask();

        // encode_offsets.

        // Reconstructed offset0 is correct.
        eval.add_constraint(
            ((offset0_low_col19.clone() + (offset0_mid_col20.clone() * M31_512.clone()))
                - input_col1.clone()),
        );
        // Reconstructed offset1 is correct.
        eval.add_constraint(
            (((offset1_low_col21.clone() + (offset1_mid_col22.clone() * M31_4.clone()))
                + (offset1_high_col23.clone() * M31_2048.clone()))
                - input_col2.clone()),
        );
        // Reconstructed offset2 is correct.
        eval.add_constraint(
            (((offset2_low_col24.clone() + (offset2_mid_col25.clone() * M31_16.clone()))
                + (offset2_high_col26.clone() * M31_8192.clone()))
                - input_col3.clone()),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_7_2_5_lookup_elements,
            E::EF::one(),
            &[
                offset0_mid_col20.clone(),
                offset1_low_col21.clone(),
                offset1_high_col23.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_4_3_lookup_elements,
            E::EF::one(),
            &[offset2_low_col24.clone(), offset2_high_col26.clone()],
        ));

        // encode_flags.

        // Flag dst_base_fp is a bit.
        eval.add_constraint((input_col4.clone() * (M31_1.clone() - input_col4.clone())));
        // Flag op0_base_fp is a bit.
        eval.add_constraint((input_col5.clone() * (M31_1.clone() - input_col5.clone())));
        // Flag op_1_imm is a bit.
        eval.add_constraint((input_col6.clone() * (M31_1.clone() - input_col6.clone())));
        // Flag op_1_base_fp is a bit.
        eval.add_constraint((input_col7.clone() * (M31_1.clone() - input_col7.clone())));
        // Flag op_1_base_ap is a bit.
        eval.add_constraint((input_col8.clone() * (M31_1.clone() - input_col8.clone())));
        // Flag res_add is a bit.
        eval.add_constraint((input_col9.clone() * (M31_1.clone() - input_col9.clone())));
        // Flag res_mul is a bit.
        eval.add_constraint((input_col10.clone() * (M31_1.clone() - input_col10.clone())));
        // Flag pc_update_jump is a bit.
        eval.add_constraint((input_col11.clone() * (M31_1.clone() - input_col11.clone())));
        // Flag pc_update_jump_rel is a bit.
        eval.add_constraint((input_col12.clone() * (M31_1.clone() - input_col12.clone())));
        // Flag pc_update_jnz is a bit.
        eval.add_constraint((input_col13.clone() * (M31_1.clone() - input_col13.clone())));
        // Flag ap_update_add is a bit.
        eval.add_constraint((input_col14.clone() * (M31_1.clone() - input_col14.clone())));
        // Flag ap_update_add_1 is a bit.
        eval.add_constraint((input_col15.clone() * (M31_1.clone() - input_col15.clone())));
        // Flag opcode_call is a bit.
        eval.add_constraint((input_col16.clone() * (M31_1.clone() - input_col16.clone())));
        // Flag opcode_ret is a bit.
        eval.add_constraint((input_col17.clone() * (M31_1.clone() - input_col17.clone())));
        // Flag opcode_assert_eq is a bit.
        eval.add_constraint((input_col18.clone() * (M31_1.clone() - input_col18.clone())));

        // mem_verify.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[input_col0.clone(), instruction_id_col27.clone()],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                instruction_id_col27.clone(),
                offset0_low_col19.clone(),
                (offset0_mid_col20.clone() + (offset1_low_col21.clone() * M31_128.clone())),
                offset1_mid_col22.clone(),
                (offset1_high_col23.clone() + (offset2_low_col24.clone() * M31_32.clone())),
                offset2_mid_col25.clone(),
                (offset2_high_col26.clone()
                    + ((((((M31_0.clone() + (input_col4.clone() * M31_8.clone()))
                        + (input_col5.clone() * M31_16.clone()))
                        + (input_col6.clone() * M31_32.clone()))
                        + (input_col7.clone() * M31_64.clone()))
                        + (input_col8.clone() * M31_128.clone()))
                        + (input_col9.clone() * M31_256.clone()))),
                (((((((((M31_0.clone() + (input_col10.clone() * M31_1.clone()))
                    + (input_col11.clone() * M31_2.clone()))
                    + (input_col12.clone() * M31_4.clone()))
                    + (input_col13.clone() * M31_8.clone()))
                    + (input_col14.clone() * M31_16.clone()))
                    + (input_col15.clone() * M31_32.clone()))
                    + (input_col16.clone() * M31_64.clone()))
                    + (input_col17.clone() * M31_128.clone()))
                    + (input_col18.clone() * M31_256.clone())),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.verify_instruction_lookup_elements,
            E::EF::from(-mult),
            &[
                input_col0.clone(),
                input_col1.clone(),
                input_col2.clone(),
                input_col3.clone(),
                input_col4.clone(),
                input_col5.clone(),
                input_col6.clone(),
                input_col7.clone(),
                input_col8.clone(),
                input_col9.clone(),
                input_col10.clone(),
                input_col11.clone(),
                input_col12.clone(),
                input_col13.clone(),
                input_col14.clone(),
                input_col15.clone(),
                input_col16.clone(),
                input_col17.clone(),
                input_col18.clone(),
            ],
        ));

        eval.finalize_logup_in_pairs();
        eval
    }
}
