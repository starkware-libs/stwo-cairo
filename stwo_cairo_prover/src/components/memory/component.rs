use stwo_prover::constraint_framework::logup::LogupAtRow;
use stwo_prover::constraint_framework::{EvalAtRow, FrameworkComponent};
use stwo_prover::core::channel::Channel;
use stwo_prover::core::fields::qm31::{SecureField, QM31};
use stwo_prover::core::fields::secure_column::SECURE_EXTENSION_DEGREE;
use stwo_prover::core::pcs::TreeVec;

use super::MemoryLookupElements;

pub const N_M31_IN_FELT252: usize = 28;
pub const MULTIPLICITY_COLUMN_OFFSET: usize = N_M31_IN_FELT252 + 1;
// TODO(AlonH): Make memory size configurable.
pub const N_MEMORY_COLUMNS: usize = N_M31_IN_FELT252 + 2;
pub const LOG_MEMORY_ADDRESS_BOUND: u32 = 7;
pub const MEMORY_ADDRESS_BOUND: usize = 1 << LOG_MEMORY_ADDRESS_BOUND;
/// Addresses are continuous and start from 0.
/// Values are Felt252 stored as `N_M31_IN_FELT252` M31 values (each value containing 9 bits).
#[derive(Clone)]
pub struct MemoryComponent {
    pub log_n_rows: u32,
    pub lookup_elements: MemoryLookupElements,
    pub claimed_sum: QM31,
}
impl MemoryComponent {
    pub const fn n_columns(&self) -> usize {
        N_M31_IN_FELT252 + 2
    }
    pub fn new(
        claim: MemoryClaim,
        lookup_elements: MemoryLookupElements,
        interaction_claim: MemoryInteractionClaim,
    ) -> Self {
        Self {
            log_n_rows: claim.log_address_bound,
            lookup_elements,
            claimed_sum: interaction_claim.claimed_sum,
        }
    }
}

impl FrameworkComponent for MemoryComponent {
    fn log_size(&self) -> u32 {
        self.log_n_rows
    }

    fn max_constraint_log_degree_bound(&self) -> u32 {
        self.log_size() + 1
    }

    fn evaluate<E: EvalAtRow>(&self, mut eval: E) -> E {
        let mut logup = LogupAtRow::<1, E>::new(1, self.claimed_sum, self.log_size());

        let address_and_value: [E::F; N_M31_IN_FELT252 + 1] =
            std::array::from_fn(|_| eval.next_trace_mask());
        let multiplicity = eval.next_trace_mask();
        logup.push_lookup(
            &mut eval,
            (-multiplicity).into(),
            &address_and_value,
            &self.lookup_elements,
        );
        // TODO(Ohad): add range check lookup constraint.
        logup.finalize(&mut eval);

        eval
    }
}

#[derive(Clone)]
pub struct MemoryClaim {
    pub log_address_bound: u32,
}
impl MemoryClaim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let interaction_0_log_size = vec![self.log_address_bound; N_M31_IN_FELT252 + 2];
        let interaction_1_log_size = vec![self.log_address_bound; SECURE_EXTENSION_DEGREE];
        TreeVec::new(vec![interaction_0_log_size, interaction_1_log_size])
    }
}

#[derive(Clone)]
pub struct MemoryInteractionClaim {
    pub claimed_sum: SecureField,
}
impl MemoryInteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_felts(&[self.claimed_sum]);
    }
}
