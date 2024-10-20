use num_traits::One;
use serde::{Deserialize, Serialize};
use stwo_prover::constraint_framework::logup::LogupAtRow;
use stwo_prover::constraint_framework::{EvalAtRow, FrameworkComponent, FrameworkEval};
use stwo_prover::core::channel::Channel;
use stwo_prover::core::fields::qm31::{SecureField, QM31};
use stwo_prover::core::fields::secure_column::SECURE_EXTENSION_DEGREE;
use stwo_prover::core::lookups::utils::Fraction;
use stwo_prover::core::pcs::TreeVec;

use super::IdToF252LookupElements;
use crate::components::range_check_unit::RangeCheckElements;

pub const MEMORY_ID_SIZE: usize = 1;
pub const N_M31_IN_FELT252: usize = 28;
pub const MULTIPLICITY_COLUMN_OFFSET: usize = N_M31_IN_FELT252 + MEMORY_ID_SIZE;
pub const N_MULTIPLICITY_COLUMNS: usize = 1;
// TODO(AlonH): Make memory size configurable.
pub const N_ID_TO_VALUE_COLUMNS: usize = MEMORY_ID_SIZE + N_M31_IN_FELT252 + N_MULTIPLICITY_COLUMNS;

pub type MemoryComponent = FrameworkComponent<MemoryEval>;

/// Addresses are continuous and start from 0.
/// Values are Felt252 stored as `N_M31_IN_FELT252` M31 values (each value containing 9 bits).
#[derive(Clone)]
pub struct MemoryEval {
    pub log_n_rows: u32,
    pub lookup_elements: IdToF252LookupElements,
    pub range9_lookup_elements: RangeCheckElements,
    pub claimed_sum: QM31,
}
impl MemoryEval {
    pub const fn n_columns(&self) -> usize {
        N_ID_TO_VALUE_COLUMNS
    }
    pub fn new(
        claim: MemoryClaim,
        lookup_elements: IdToF252LookupElements,
        range9_lookup_elements: RangeCheckElements,
        interaction_claim: MemoryInteractionClaim,
    ) -> Self {
        Self {
            log_n_rows: claim.log_size,
            lookup_elements,
            range9_lookup_elements,
            claimed_sum: interaction_claim.claimed_sum,
        }
    }
}

impl FrameworkEval for MemoryEval {
    fn log_size(&self) -> u32 {
        self.log_n_rows
    }

    fn max_constraint_log_degree_bound(&self) -> u32 {
        self.log_size() + 1
    }

    fn evaluate<E: EvalAtRow>(&self, mut eval: E) -> E {
        let [is_first] = eval.next_interaction_mask(2, [0]);
        let mut logup = LogupAtRow::<E>::new(1, self.claimed_sum, None, is_first);

        let id_and_value: [E::F; MEMORY_ID_SIZE + N_M31_IN_FELT252] =
            std::array::from_fn(|_| eval.next_trace_mask());
        let multiplicity = eval.next_trace_mask();
        let frac = Fraction::new(
            E::EF::from(-multiplicity),
            self.lookup_elements.combine(&id_and_value),
        );
        logup.write_frac(&mut eval, frac);

        // Range check elements.
        for value_limb in id_and_value.iter().skip(MEMORY_ID_SIZE) {
            let frac = Fraction::new(
                E::EF::one(),
                self.range9_lookup_elements.combine(&[value_limb.clone()]),
            );
            logup.write_frac(&mut eval, frac);
        }

        logup.finalize(&mut eval);

        eval
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MemoryClaim {
    pub log_size: u32,
}
impl MemoryClaim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let interaction_0_log_size = vec![self.log_size; N_ID_TO_VALUE_COLUMNS];
        let interaction_1_log_size =
            vec![self.log_size; SECURE_EXTENSION_DEGREE * (N_M31_IN_FELT252 + 1)];
        let fixed_column_log_sizes = vec![self.log_size];
        TreeVec::new(vec![
            interaction_0_log_size,
            interaction_1_log_size,
            fixed_column_log_sizes,
        ])
    }

    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_u64(self.log_size as u64);
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MemoryInteractionClaim {
    pub claimed_sum: SecureField,
}
impl MemoryInteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_felts(&[self.claimed_sum]);
    }
}
