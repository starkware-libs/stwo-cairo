#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use num_traits::{One, Zero};
use serde::{Deserialize, Serialize};
use stwo_cairo_common::preprocessed_consts::poseidon::N_WORDS;
use stwo_cairo_serialize::CairoSerialize;
use stwo_prover::constraint_framework::logup::{LogupAtRow, LookupElements};
use stwo_prover::constraint_framework::{
    EvalAtRow, FrameworkComponent, FrameworkEval, RelationEntry,
};
use stwo_prover::core::backend::simd::m31::LOG_N_LANES;
use stwo_prover::core::channel::Channel;
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::fields::qm31::SecureField;
use stwo_prover::core::fields::secure_column::SECURE_EXTENSION_DEGREE;
use stwo_prover::core::pcs::TreeVec;

use crate::cairo_air::poseidon::const_columns::PoseidonRoundKeys;
use crate::cairo_air::preprocessed::{PreProcessedColumn, Seq};
use crate::cairo_air::relations;

pub struct Eval {
    pub claim: Claim,
    pub poseidon_round_keys_lookup_elements: relations::PoseidonRoundKeys,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct Claim {
    pub log_size: u32,
}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let trace_log_sizes = vec![self.log_size; 0];
        let interaction_log_sizes = vec![self.log_size; SECURE_EXTENSION_DEGREE];
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
        let poseidonroundkeys: [_; N_WORDS] =
            std::array::from_fn(|i| eval.get_preprocessed_column(PoseidonRoundKeys::new(i).id()));
        let seq = eval.get_preprocessed_column(Seq::new(self.log_size()).id());
        let multiplicity = eval.next_trace_mask();

        eval.add_to_relation(RelationEntry::new(
            &self.poseidon_round_keys_lookup_elements,
            -E::EF::from(multiplicity),
            &[
                seq.clone(),
                poseidonroundkeys[0].clone(),
                poseidonroundkeys[1].clone(),
                poseidonroundkeys[2].clone(),
                poseidonroundkeys[3].clone(),
                poseidonroundkeys[4].clone(),
                poseidonroundkeys[5].clone(),
                poseidonroundkeys[6].clone(),
                poseidonroundkeys[7].clone(),
                poseidonroundkeys[8].clone(),
                poseidonroundkeys[9].clone(),
                poseidonroundkeys[10].clone(),
                poseidonroundkeys[11].clone(),
                poseidonroundkeys[12].clone(),
                poseidonroundkeys[13].clone(),
                poseidonroundkeys[14].clone(),
                poseidonroundkeys[15].clone(),
                poseidonroundkeys[16].clone(),
                poseidonroundkeys[17].clone(),
                poseidonroundkeys[18].clone(),
                poseidonroundkeys[19].clone(),
                poseidonroundkeys[20].clone(),
                poseidonroundkeys[21].clone(),
                poseidonroundkeys[22].clone(),
                poseidonroundkeys[23].clone(),
                poseidonroundkeys[24].clone(),
                poseidonroundkeys[25].clone(),
                poseidonroundkeys[26].clone(),
                poseidonroundkeys[27].clone(),
                poseidonroundkeys[28].clone(),
                poseidonroundkeys[29].clone(),
            ],
        ));

        eval.finalize_logup_in_pairs();
        eval
    }
}
