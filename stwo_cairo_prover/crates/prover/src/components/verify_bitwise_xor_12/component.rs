use crate::components::prelude::constraint_eval::*;

pub struct Eval {
    pub claim: Claim,
    pub verify_bitwise_xor_12_lookup_elements: relations::VerifyBitwiseXor_12,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct Claim {
    pub log_size: u32,
}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let trace_log_sizes = vec![self.log_size; 0];
        let interaction_log_sizes = vec![self.log_size; SECURE_EXTENSION_DEGREE];
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
        let bitwisexor = eval.get_preprocessed_column((BitwiseXor::new(12, 0)).id());
        let bitwisexor = eval.get_preprocessed_column((BitwiseXor::new(12, 1)).id());
        let bitwisexor = eval.get_preprocessed_column((BitwiseXor::new(12, 2)).id());
        eval.add_to_relation(RelationEntry::new(
            &self.verify_bitwise_xor_12_lookup_elements,
            -E::EF::one(),
            &[bitwisexor.clone(), bitwisexor.clone(), bitwisexor.clone()],
        ));

        eval.finalize_logup_in_pairs();
        eval
    }
}
