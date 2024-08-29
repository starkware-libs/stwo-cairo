use num_traits::One;
use stwo_prover::constraint_framework::logup::LogupAtRow;
use stwo_prover::constraint_framework::{EvalAtRow, FrameworkComponent};
use stwo_prover::core::channel::Channel;
use stwo_prover::core::fields::qm31::{SecureField, QM31};
use stwo_prover::core::fields::secure_column::SECURE_EXTENSION_DEGREE;
use stwo_prover::core::pcs::TreeVec;

use super::MemoryLookupElements;
use crate::components::range_check_unit::RangeCheckElements;

pub const N_M31_IN_FELT252: usize = 28;
pub const MULTIPLICITY_COLUMN_OFFSET: usize = N_M31_IN_FELT252 + 1;
// TODO(AlonH): Make memory size configurable.
pub const N_MEMORY_COLUMNS: usize = N_M31_IN_FELT252 + 2;
pub const LOG_MEMORY_ADDRESS_BOUND: u32 = 7;
pub const MEMORY_ADDRESS_BOUND: usize = 1 << LOG_MEMORY_ADDRESS_BOUND;

/// Addresses are continuous and start from 0.
/// Values are Felt252 stored as `N_M31_IN_FELT252` M31 values (each value containing 9 bits).
#[derive(Clone)]
pub struct MemoryComponent<const BATCH_SIZE: usize> {
    pub log_n_rows: u32,
    pub lookup_elements: MemoryLookupElements,
    pub range9_lookup_elements: RangeCheckElements,
    pub claimed_sum: QM31,
}
impl<const BATCH_SIZE: usize> MemoryComponent<BATCH_SIZE> {
    pub const fn n_columns(&self) -> usize {
        N_M31_IN_FELT252 + 2
    }
    pub fn new(
        claim: MemoryClaim<BATCH_SIZE>,
        lookup_elements: MemoryLookupElements,
        range9_lookup_elements: RangeCheckElements,
        interaction_claim: MemoryInteractionClaim,
    ) -> Self {
        Self {
            log_n_rows: claim.log_address_bound,
            lookup_elements,
            range9_lookup_elements,
            claimed_sum: interaction_claim.claimed_sum,
        }
    }
}

impl<const BATCH_SIZE: usize> FrameworkComponent for MemoryComponent<BATCH_SIZE> {
    fn log_size(&self) -> u32 {
        self.log_n_rows
    }

    fn max_constraint_log_degree_bound(&self) -> u32 {
        self.log_size() + 1
    }

    fn evaluate<E: EvalAtRow>(&self, mut eval: E) -> E {
        let mut logup = LogupAtRow::<BATCH_SIZE, E>::new(1, self.claimed_sum, self.log_size());

        let address_and_value: [E::F; N_M31_IN_FELT252 + 1] =
            std::array::from_fn(|_| eval.next_trace_mask());
        let multiplicity = eval.next_trace_mask();
        logup.push_lookup(
            &mut eval,
            (-multiplicity).into(),
            &address_and_value,
            &self.lookup_elements,
        );

        // Range check elements.
        for value in address_and_value[1..].iter() {
            logup.push_lookup(
                &mut eval,
                E::EF::one(),
                &[*value],
                &self.range9_lookup_elements,
            );
        }

        logup.finalize(&mut eval);

        eval
    }
}

#[derive(Clone)]
pub struct MemoryClaim<const BATCH_SIZE: usize> {
    pub log_address_bound: u32,
}
impl<const BATCH_SIZE: usize> MemoryClaim<BATCH_SIZE> {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let interaction_0_log_size = vec![self.log_address_bound; N_M31_IN_FELT252 + 2];
        // Memory lookup + N_M31_IN_FELT252 range checks.
        let n_logup_entries = N_M31_IN_FELT252 + 1;
        let n_logup_cols = (n_logup_entries + BATCH_SIZE - 1) / BATCH_SIZE;
        let interaction_1_log_size =
            vec![self.log_address_bound; n_logup_cols * SECURE_EXTENSION_DEGREE];
        TreeVec::new(vec![interaction_0_log_size, interaction_1_log_size])
    }

    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_nonce(self.log_address_bound as u64);
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
