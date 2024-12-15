use serde::{Deserialize, Serialize};
use stwo_prover::constraint_framework::{
    EvalAtRow, FrameworkComponent, FrameworkEval, RelationEntry,
};
use stwo_prover::core::channel::Channel;
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::fields::qm31::SecureField;
use stwo_prover::core::fields::secure_column::SECURE_EXTENSION_DEGREE;
use stwo_prover::core::pcs::TreeVec;

use crate::relations;

// TODO(Ohad): Address should be a preprocessed `seq`.
pub const N_ADDR_COLUMNS: usize = 1;

// Split the (ID , Multiplicity) columns to shorter chunks. This is done to improve the performance
// during The merkle commitment and FRI, as this component is usually the tallest in the Cairo AIR.
// TODO(Ohad): Change split to 8 after seq is implemented.
pub(super) const LOG_SPLIT_SIZE: u32 = 2;
pub(super) const SPLIT_SIZE: usize = 1 << LOG_SPLIT_SIZE;
pub(super) const N_ID_AND_MULT_COLUMNS_PER_CHUNK: usize = 2;
pub(super) const N_TRACE_COLUMNS: usize =
    N_ADDR_COLUMNS + SPLIT_SIZE * N_ID_AND_MULT_COLUMNS_PER_CHUNK;

pub type Component = FrameworkComponent<Eval>;

#[derive(Clone)]
pub struct Eval {
    // The log size of the component after split.
    pub log_size: u32,
    pub lookup_elements: relations::MemoryAddressToId,
}
impl Eval {
    // TODO(ShaharS): use Seq column for address, and also use repititions.
    pub const fn n_columns(&self) -> usize {
        N_TRACE_COLUMNS
    }

    pub fn new(claim: Claim, lookup_elements: relations::MemoryAddressToId) -> Self {
        Self {
            log_size: claim.log_size,
            lookup_elements,
        }
    }
}

impl FrameworkEval for Eval {
    fn log_size(&self) -> u32 {
        self.log_size
    }

    fn max_constraint_log_degree_bound(&self) -> u32 {
        self.log_size() + 1
    }

    fn evaluate<E: EvalAtRow>(&self, mut eval: E) -> E {
        let address = eval.next_trace_mask();
        for i in 0..SPLIT_SIZE {
            let id = eval.next_trace_mask();
            let multiplicity = eval.next_trace_mask();
            let address = address.clone() + E::F::from(M31((i * (1 << self.log_size())) as u32));
            eval.add_to_relation(RelationEntry::new(
                &self.lookup_elements,
                E::EF::from(-multiplicity),
                &[address, id],
            ));
        }

        eval.finalize_logup_in_pairs();
        eval
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Claim {
    pub log_size: u32,
}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let preprocessed_log_sizes = vec![self.log_size];
        let trace_log_sizes = vec![self.log_size; N_TRACE_COLUMNS];
        let interaction_log_sizes =
            vec![self.log_size; SECURE_EXTENSION_DEGREE * SPLIT_SIZE.div_ceil(2)];
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

#[derive(Clone, Serialize, Deserialize)]
pub struct InteractionClaim {
    pub claimed_sum: SecureField,
}
impl InteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_felts(&[self.claimed_sum]);
    }
}
