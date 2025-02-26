use crate::components::prelude::constraint_eval::*;

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
        let trace_log_sizes = vec![self.log_size; 17];
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
        let M31_128 = E::F::from(M31::from(128));
        let M31_16 = E::F::from(M31::from(16));
        let M31_2048 = E::F::from(M31::from(2048));
        let M31_32 = E::F::from(M31::from(32));
        let M31_4 = E::F::from(M31::from(4));
        let M31_512 = E::F::from(M31::from(512));
        let M31_8192 = E::F::from(M31::from(8192));
        let input_limb_0_col0 = eval.next_trace_mask();
        let input_limb_1_col1 = eval.next_trace_mask();
        let input_limb_2_col2 = eval.next_trace_mask();
        let input_limb_3_col3 = eval.next_trace_mask();
        let input_limb_4_col4 = eval.next_trace_mask();
        let input_limb_5_col5 = eval.next_trace_mask();
        let input_limb_6_col6 = eval.next_trace_mask();
        let offset0_low_col7 = eval.next_trace_mask();
        let offset0_mid_col8 = eval.next_trace_mask();
        let offset1_low_col9 = eval.next_trace_mask();
        let offset1_mid_col10 = eval.next_trace_mask();
        let offset1_high_col11 = eval.next_trace_mask();
        let offset2_low_col12 = eval.next_trace_mask();
        let offset2_mid_col13 = eval.next_trace_mask();
        let offset2_high_col14 = eval.next_trace_mask();
        let instruction_id_col15 = eval.next_trace_mask();
        let mult = eval.next_trace_mask();

        // Encode Offsets.

        // Reconstructed offset0 is correct.
        eval.add_constraint(
            ((offset0_low_col7.clone() + (offset0_mid_col8.clone() * M31_512.clone()))
                - input_limb_1_col1.clone()),
        );
        // Reconstructed offset1 is correct.
        eval.add_constraint(
            (((offset1_low_col9.clone() + (offset1_mid_col10.clone() * M31_4.clone()))
                + (offset1_high_col11.clone() * M31_2048.clone()))
                - input_limb_2_col2.clone()),
        );
        // Reconstructed offset2 is correct.
        eval.add_constraint(
            (((offset2_low_col12.clone() + (offset2_mid_col13.clone() * M31_16.clone()))
                + (offset2_high_col14.clone() * M31_8192.clone()))
                - input_limb_3_col3.clone()),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_7_2_5_lookup_elements,
            E::EF::one(),
            &[
                offset0_mid_col8.clone(),
                offset1_low_col9.clone(),
                offset1_high_col11.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_4_3_lookup_elements,
            E::EF::one(),
            &[offset2_low_col12.clone(), offset2_high_col14.clone()],
        ));

        // Mem Verify.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[input_limb_0_col0.clone(), instruction_id_col15.clone()],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                instruction_id_col15.clone(),
                offset0_low_col7.clone(),
                (offset0_mid_col8.clone() + (offset1_low_col9.clone() * M31_128.clone())),
                offset1_mid_col10.clone(),
                (offset1_high_col11.clone() + (offset2_low_col12.clone() * M31_32.clone())),
                offset2_mid_col13.clone(),
                (offset2_high_col14.clone() + input_limb_4_col4.clone()),
                input_limb_5_col5.clone(),
                input_limb_6_col6.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.verify_instruction_lookup_elements,
            E::EF::from(-mult),
            &[
                input_limb_0_col0.clone(),
                input_limb_1_col1.clone(),
                input_limb_2_col2.clone(),
                input_limb_3_col3.clone(),
                input_limb_4_col4.clone(),
                input_limb_5_col5.clone(),
                input_limb_6_col6.clone(),
            ],
        ));

        eval.finalize_logup_in_pairs();
        eval
    }
}
