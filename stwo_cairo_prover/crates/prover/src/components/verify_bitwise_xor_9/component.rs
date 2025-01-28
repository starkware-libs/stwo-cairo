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

use crate::cairo_air::preprocessed::{BitwiseXor, PreProcessedColumn};
use crate::relations;

pub struct Eval {
    pub claim: Claim,
    pub verify_bitwise_xor_9_lookup_elements: relations::VerifyBitwiseXor_9,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct Claim {
    pub log_size: u32,
}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let log_size = self.log_size;
        let trace_log_sizes = vec![log_size; 1];
        let interaction_log_sizes = vec![log_size; SECURE_EXTENSION_DEGREE];
        let preprocessed_log_sizes = vec![log_size];
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
        let BitwiseXor0 = eval
            .get_preprocessed_column(PreProcessedColumn::BitwiseXor(BitwiseXor::new(9, 0)).id());
        let BitwiseXor1 = eval
            .get_preprocessed_column(PreProcessedColumn::BitwiseXor(BitwiseXor::new(9, 1)).id());
        let BitwiseXor2 = eval
            .get_preprocessed_column(PreProcessedColumn::BitwiseXor(BitwiseXor::new(9, 2)).id());
        let multiplicity = eval.next_trace_mask();

        eval.add_to_relation(RelationEntry::new(
            &self.verify_bitwise_xor_9_lookup_elements,
            -E::EF::from(multiplicity),
            &[BitwiseXor0, BitwiseXor1, BitwiseXor2],
        ));

        eval.finalize_logup_in_pairs();
        eval
    }
}
