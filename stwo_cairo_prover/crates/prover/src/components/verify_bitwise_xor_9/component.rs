use crate::components::prelude::constraint_eval::*;

pub const N_BITS: u32 = 9;
pub const LOG_SIZE: u32 = N_BITS * 2;

pub struct Eval {
    pub verify_bitwise_xor_9_lookup_elements: relations::VerifyBitwiseXor_9,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct Claim {}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let trace_log_sizes = vec![LOG_SIZE; 1];
        let interaction_log_sizes = vec![LOG_SIZE; SECURE_EXTENSION_DEGREE];
        let preprocessed_log_sizes = vec![LOG_SIZE];
        TreeVec::new(vec![
            preprocessed_log_sizes,
            trace_log_sizes,
            interaction_log_sizes,
        ])
    }

    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_u64(LOG_SIZE as u64);
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
        LOG_SIZE
    }

    fn max_constraint_log_degree_bound(&self) -> u32 {
        self.log_size() + 1
    }

    fn evaluate<E: EvalAtRow>(&self, mut eval: E) -> E {
        let xor_a = eval.get_preprocessed_column(BitwiseXor::new(N_BITS, 0).id());
        let xor_b = eval.get_preprocessed_column(BitwiseXor::new(N_BITS, 1).id());
        let xor_c = eval.get_preprocessed_column(BitwiseXor::new(N_BITS, 2).id());
        let multiplicity = eval.next_trace_mask();

        eval.add_to_relation(RelationEntry::new(
            &self.verify_bitwise_xor_9_lookup_elements,
            -E::EF::from(multiplicity),
            &[xor_a, xor_b, xor_c],
        ));

        eval.finalize_logup_in_pairs();
        eval
    }
}
