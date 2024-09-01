use num_traits::{One, Zero};
use stwo_prover::constraint_framework::logup::LogupAtRow;
use stwo_prover::constraint_framework::{EvalAtRow, FrameworkComponent};
use stwo_prover::core::channel::Channel;
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::fields::qm31::SecureField;
use stwo_prover::core::fields::secure_column::SECURE_EXTENSION_DEGREE;
use stwo_prover::core::pcs::TreeVec;

use crate::components::memory::{MemoryLookupElements, N_ADDRESS_FELTS, N_BITS_PER_FELT};
use crate::components::range_check_unit::RangeCheckElements;
use crate::components::LOOKUP_INTERACTION_PHASE;
use crate::input::SegmentAddrs;

const RANGE_CHECK_BITS: usize = 128;
pub const N_RANGE_CHECK_COLUMNS: usize = 16;
pub const N_VALUES_FELTS: usize = RANGE_CHECK_BITS.div_ceil(N_BITS_PER_FELT);
pub const LAST_VALUE_OFFSET: usize = N_ADDRESS_FELTS + N_VALUES_FELTS - 1;

pub struct RangeCheckBuiltinEval<'a, E: EvalAtRow> {
    pub eval: E,
    pub logup: LogupAtRow<2, E>,
    pub initial_memory_address: E::F,
    pub memory_lookup_elements: &'a MemoryLookupElements,
    pub range2_lookup_elements: &'a RangeCheckElements,
}
const _: () = assert!(
    RANGE_CHECK_BITS % N_BITS_PER_FELT == 2,
    "High non-zero element must be 2 bits"
);

impl<'a, E: EvalAtRow> RangeCheckBuiltinEval<'a, E> {
    pub fn eval(mut self) -> E {
        let mut values = [E::F::zero(); N_ADDRESS_FELTS + N_VALUES_FELTS];

        // Memory address.
        // TODO(ShaharS): Use a constant column instead of taking the next_trace_mask().
        values[0] = self.initial_memory_address + self.eval.next_trace_mask();

        // Memory values.
        for value in values.iter_mut().skip(N_ADDRESS_FELTS) {
            *value = self.eval.next_trace_mask();
        }

        // Compute lookup for memory.
        self.logup.push_lookup(
            &mut self.eval,
            E::EF::one(),
            &values,
            self.memory_lookup_elements,
        );

        // Compute lookup for last element range check.
        self.logup.push_lookup(
            &mut self.eval,
            E::EF::one(),
            &[values[N_VALUES_FELTS]],
            self.range2_lookup_elements,
        );

        self.logup.finalize(&mut self.eval);
        self.eval
    }
}

pub struct RangeCheckBuiltinComponent {
    pub log_size: u32,
    pub initial_memory_address: M31,
    pub memory_lookup_elements: MemoryLookupElements,
    pub range2_lookup_elements: RangeCheckElements,
    pub claimed_sum: SecureField,
}

impl FrameworkComponent for RangeCheckBuiltinComponent {
    fn log_size(&self) -> u32 {
        self.log_size
    }
    fn max_constraint_log_degree_bound(&self) -> u32 {
        self.log_size + 1
    }
    fn evaluate<E: EvalAtRow>(&self, eval: E) -> E {
        let rc_builtin_eval = RangeCheckBuiltinEval {
            eval,
            initial_memory_address: E::F::from(self.initial_memory_address),
            memory_lookup_elements: &self.memory_lookup_elements,
            range2_lookup_elements: &self.range2_lookup_elements,
            logup: LogupAtRow::new(LOOKUP_INTERACTION_PHASE, self.claimed_sum, self.log_size),
        };
        rc_builtin_eval.eval()
    }
}

#[derive(Clone)]
pub struct RangeCheckBuiltinClaim {
    pub memory_segment: SegmentAddrs,
}

impl RangeCheckBuiltinClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_nonce(self.memory_segment.begin_addr as u64);
        channel.mix_nonce(self.memory_segment.end_addr as u64);
    }

    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let n_values = self.memory_segment.end_addr - self.memory_segment.begin_addr;
        let log_size = n_values.next_power_of_two().ilog2();
        let trace_log_sizes = vec![log_size; N_ADDRESS_FELTS + N_VALUES_FELTS];
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
