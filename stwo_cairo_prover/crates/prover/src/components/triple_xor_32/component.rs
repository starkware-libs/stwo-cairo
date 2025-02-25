use crate::components::prelude::constraint_eval::*;

pub struct Eval {
    pub claim: Claim,
    pub triple_xor_32_lookup_elements: relations::TripleXor32,
    pub verify_bitwise_xor_8_lookup_elements: relations::VerifyBitwiseXor_8,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct Claim {
    pub log_size: u32,
}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let trace_log_sizes = vec![self.log_size; 20];
        let interaction_log_sizes = vec![self.log_size; SECURE_EXTENSION_DEGREE * 5];
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
        let M31_256 = E::F::from(M31::from(256));
        let input_limb_0_col0 = eval.next_trace_mask();
        let input_limb_1_col1 = eval.next_trace_mask();
        let input_limb_2_col2 = eval.next_trace_mask();
        let input_limb_3_col3 = eval.next_trace_mask();
        let input_limb_4_col4 = eval.next_trace_mask();
        let input_limb_5_col5 = eval.next_trace_mask();
        let ms_8_bits_col6 = eval.next_trace_mask();
        let ms_8_bits_col7 = eval.next_trace_mask();
        let ms_8_bits_col8 = eval.next_trace_mask();
        let ms_8_bits_col9 = eval.next_trace_mask();
        let ms_8_bits_col10 = eval.next_trace_mask();
        let ms_8_bits_col11 = eval.next_trace_mask();
        let xor_col12 = eval.next_trace_mask();
        let xor_col13 = eval.next_trace_mask();
        let xor_col14 = eval.next_trace_mask();
        let xor_col15 = eval.next_trace_mask();
        let xor_col16 = eval.next_trace_mask();
        let xor_col17 = eval.next_trace_mask();
        let xor_col18 = eval.next_trace_mask();
        let xor_col19 = eval.next_trace_mask();

        // Bitwise Xor Num Bits 8.

        eval.add_to_relation(RelationEntry::new(
            &self.verify_bitwise_xor_8_lookup_elements,
            E::EF::one(),
            &[
                (input_limb_0_col0.clone() - (ms_8_bits_col6.clone() * M31_256.clone())),
                (input_limb_2_col2.clone() - (ms_8_bits_col8.clone() * M31_256.clone())),
                xor_col12.clone(),
            ],
        ));

        // Bitwise Xor Num Bits 8.

        eval.add_to_relation(RelationEntry::new(
            &self.verify_bitwise_xor_8_lookup_elements,
            E::EF::one(),
            &[
                xor_col12.clone(),
                (input_limb_4_col4.clone() - (ms_8_bits_col10.clone() * M31_256.clone())),
                xor_col13.clone(),
            ],
        ));

        // Bitwise Xor Num Bits 8.

        eval.add_to_relation(RelationEntry::new(
            &self.verify_bitwise_xor_8_lookup_elements,
            E::EF::one(),
            &[
                ms_8_bits_col6.clone(),
                ms_8_bits_col8.clone(),
                xor_col14.clone(),
            ],
        ));

        // Bitwise Xor Num Bits 8.

        eval.add_to_relation(RelationEntry::new(
            &self.verify_bitwise_xor_8_lookup_elements,
            E::EF::one(),
            &[
                xor_col14.clone(),
                ms_8_bits_col10.clone(),
                xor_col15.clone(),
            ],
        ));

        // Bitwise Xor Num Bits 8.

        eval.add_to_relation(RelationEntry::new(
            &self.verify_bitwise_xor_8_lookup_elements,
            E::EF::one(),
            &[
                (input_limb_1_col1.clone() - (ms_8_bits_col7.clone() * M31_256.clone())),
                (input_limb_3_col3.clone() - (ms_8_bits_col9.clone() * M31_256.clone())),
                xor_col16.clone(),
            ],
        ));

        // Bitwise Xor Num Bits 8.

        eval.add_to_relation(RelationEntry::new(
            &self.verify_bitwise_xor_8_lookup_elements,
            E::EF::one(),
            &[
                xor_col16.clone(),
                (input_limb_5_col5.clone() - (ms_8_bits_col11.clone() * M31_256.clone())),
                xor_col17.clone(),
            ],
        ));

        // Bitwise Xor Num Bits 8.

        eval.add_to_relation(RelationEntry::new(
            &self.verify_bitwise_xor_8_lookup_elements,
            E::EF::one(),
            &[
                ms_8_bits_col7.clone(),
                ms_8_bits_col9.clone(),
                xor_col18.clone(),
            ],
        ));

        // Bitwise Xor Num Bits 8.

        eval.add_to_relation(RelationEntry::new(
            &self.verify_bitwise_xor_8_lookup_elements,
            E::EF::one(),
            &[
                xor_col18.clone(),
                ms_8_bits_col11.clone(),
                xor_col19.clone(),
            ],
        ));

        let triple_xor32_output_tmp_298db_15_limb_0 =
            eval.add_intermediate((xor_col13.clone() + (xor_col15.clone() * M31_256.clone())));
        let triple_xor32_output_tmp_298db_15_limb_1 =
            eval.add_intermediate((xor_col17.clone() + (xor_col19.clone() * M31_256.clone())));
        eval.add_to_relation(RelationEntry::new(
            &self.triple_xor_32_lookup_elements,
            -E::EF::one(),
            &[
                input_limb_0_col0.clone(),
                input_limb_1_col1.clone(),
                input_limb_2_col2.clone(),
                input_limb_3_col3.clone(),
                input_limb_4_col4.clone(),
                input_limb_5_col5.clone(),
                triple_xor32_output_tmp_298db_15_limb_0.clone(),
                triple_xor32_output_tmp_298db_15_limb_1.clone(),
            ],
        ));

        eval.finalize_logup_in_pairs();
        eval
    }
}
