use num_traits::{One, Zero};
use stwo_prover::constraint_framework::logup::LogupAtRow;
use stwo_prover::constraint_framework::{EvalAtRow, FrameworkComponent, FrameworkEval};
use stwo_prover::core::channel::Channel;
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::fields::qm31::SecureField;
use stwo_prover::core::fields::secure_column::SECURE_EXTENSION_DEGREE;
use stwo_prover::core::lookups::utils::Fraction;
use stwo_prover::core::pcs::TreeVec;

use crate::components::memory::{MemoryLookupElements, N_ADDRESS_FELTS, N_BITS_PER_FELT};
use crate::components::LOOKUP_INTERACTION_PHASE;
use crate::input::SegmentAddrs;

const RANGE_CHECK_BITS: usize = 128;
const N_INTERMEDIATE_COLUMNS: usize = 1;
pub const N_VALUES_FELTS: usize = RANGE_CHECK_BITS.div_ceil(N_BITS_PER_FELT);
pub const N_RANGE_CHECK_COLUMNS: usize = N_ADDRESS_FELTS + N_VALUES_FELTS + N_INTERMEDIATE_COLUMNS;
pub const LAST_VALUE_OFFSET: usize = N_ADDRESS_FELTS + N_VALUES_FELTS - 1;

pub type RangeCheckBuiltinComponent = FrameworkComponent<RangeCheckBuiltinEval>;

const _: () = assert!(
    RANGE_CHECK_BITS % N_BITS_PER_FELT == 2,
    "High non-zero element must be 2 bits"
);

pub struct RangeCheckBuiltinEval {
    pub log_size: u32,
    pub initial_memory_address: M31,
    pub memory_lookup_elements: MemoryLookupElements,
    pub claimed_sum: SecureField,
}

impl RangeCheckBuiltinEval {
    pub fn new(
        claim: RangeCheckBuiltinClaim,
        memory_lookup_elements: MemoryLookupElements,
        interaction_claim: RangeCheckBuiltinInteractionClaim,
    ) -> Self {
        let n_values = claim.memory_segment.end_addr - claim.memory_segment.begin_addr;
        let log_size = n_values.next_power_of_two().ilog2();
        Self {
            log_size,
            initial_memory_address: M31::from(claim.memory_segment.begin_addr),
            memory_lookup_elements,
            claimed_sum: interaction_claim.claimed_sum,
        }
    }
}

impl FrameworkEval for RangeCheckBuiltinEval {
    fn log_size(&self) -> u32 {
        self.log_size
    }

    fn max_constraint_log_degree_bound(&self) -> u32 {
        self.log_size + 1
    }

    fn evaluate<E: EvalAtRow>(&self, eval: E) -> E {
        let mut eval = eval;
        let mut values = [E::F::zero(); N_ADDRESS_FELTS + N_VALUES_FELTS];

        // Memory address.
        // TODO(ShaharS): Use a constant column instead of taking the next_trace_mask().
        values[0] = E::F::from(self.initial_memory_address) + eval.next_trace_mask();

        // Memory values.
        for value in values.iter_mut().skip(N_ADDRESS_FELTS) {
            *value = eval.next_trace_mask();
        }

        // Compute lookup for memory.
        let mut logup = LogupAtRow::new(LOOKUP_INTERACTION_PHASE, self.claimed_sum, self.log_size);
        let frac = Fraction::new(E::EF::one(), self.memory_lookup_elements.combine(&values));
        logup.write_frac(&mut eval, frac);

        // Add constraints for the last 2 bit value.
        let last_value_felt = values[N_ADDRESS_FELTS + N_VALUES_FELTS - 1];
        let intermediate_value = eval.next_trace_mask();
        eval.add_constraint(
            intermediate_value - (last_value_felt * (last_value_felt - E::F::one())),
        );
        eval.add_constraint(
            intermediate_value
                * (last_value_felt - E::F::from(M31::from(2)))
                * (last_value_felt - E::F::from(M31::from(3))),
        );

        logup.finalize(&mut eval);
        eval
    }
}

#[derive(Clone)]
pub struct RangeCheckBuiltinClaim {
    pub memory_segment: SegmentAddrs,
}

impl RangeCheckBuiltinClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_u64(self.memory_segment.begin_addr as u64);
        channel.mix_u64(self.memory_segment.end_addr as u64);
    }

    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let n_values = self.memory_segment.end_addr - self.memory_segment.begin_addr;
        let log_size = n_values.next_power_of_two().ilog2();
        let trace_log_sizes = vec![log_size; N_RANGE_CHECK_COLUMNS];
        let interaction_trace_log_sizes = vec![log_size; SECURE_EXTENSION_DEGREE];
        TreeVec::new(vec![trace_log_sizes, interaction_trace_log_sizes])
    }
}

#[derive(Clone)]
pub struct RangeCheckBuiltinInteractionClaim {
    pub log_size: u32,
    pub claimed_sum: SecureField,
}

impl RangeCheckBuiltinInteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_felts(&[self.claimed_sum]);
    }
}
