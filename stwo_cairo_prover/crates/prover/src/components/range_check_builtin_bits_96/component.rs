use component_utils::cairo_air::relations;

use crate::components::prelude::constraint_eval::*;

pub struct Eval {
    pub claim: Claim,
    pub memory_address_to_id_lookup_elements: relations::MemoryAddressToId,
    pub memory_id_to_big_lookup_elements: relations::MemoryIdToBig,
    pub range_check_6_lookup_elements: relations::RangeCheck_6,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct Claim {
    pub log_size: u32,
    pub range_check96_builtin_segment_start: u32,
}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let trace_log_sizes = vec![self.log_size; 12];
        let interaction_log_sizes = vec![self.log_size; SECURE_EXTENSION_DEGREE * 2];
        let preprocessed_log_sizes = vec![self.log_size];
        TreeVec::new(vec![
            preprocessed_log_sizes,
            trace_log_sizes,
            interaction_log_sizes,
        ])
    }

    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_u64(self.log_size as u64);
        channel.mix_u64(self.range_check96_builtin_segment_start as u64);
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
        let seq = eval.get_preprocessed_column(Seq::new(self.log_size()).id());
        let value_id_col0 = eval.next_trace_mask();
        let value_limb_0_col1 = eval.next_trace_mask();
        let value_limb_1_col2 = eval.next_trace_mask();
        let value_limb_2_col3 = eval.next_trace_mask();
        let value_limb_3_col4 = eval.next_trace_mask();
        let value_limb_4_col5 = eval.next_trace_mask();
        let value_limb_5_col6 = eval.next_trace_mask();
        let value_limb_6_col7 = eval.next_trace_mask();
        let value_limb_7_col8 = eval.next_trace_mask();
        let value_limb_8_col9 = eval.next_trace_mask();
        let value_limb_9_col10 = eval.next_trace_mask();
        let value_limb_10_col11 = eval.next_trace_mask();

        // Read Positive Num Bits 96.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (E::F::from(M31::from(self.claim.range_check96_builtin_segment_start))
                    + seq.clone()),
                value_id_col0.clone(),
            ],
        ));

        // Range Check Last Limb Bits In Ms Limb 6.

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_6_lookup_elements,
            E::EF::one(),
            &[value_limb_10_col11.clone()],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                value_id_col0.clone(),
                value_limb_0_col1.clone(),
                value_limb_1_col2.clone(),
                value_limb_2_col3.clone(),
                value_limb_3_col4.clone(),
                value_limb_4_col5.clone(),
                value_limb_5_col6.clone(),
                value_limb_6_col7.clone(),
                value_limb_7_col8.clone(),
                value_limb_8_col9.clone(),
                value_limb_9_col10.clone(),
                value_limb_10_col11.clone(),
            ],
        ));

        eval.finalize_logup_in_pairs();
        eval
    }
}
