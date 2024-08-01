use num_traits::{One, Zero};
use stwo_prover::constraint_framework::logup::{LogupAtRow, LookupElements};
use stwo_prover::constraint_framework::{EvalAtRow, FrameworkComponent};
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::fields::qm31::SecureField;

const RANGE_CHECK_BITS: usize = 128;
pub const N_BITS_PER_FELT: usize = 9;
pub const ADDRESS_FELTS: usize = 1;
pub const N_VALUES_FELTS: usize = RANGE_CHECK_BITS.div_ceil(N_BITS_PER_FELT);

pub struct RangeCheck128BuiltinEval<'a, E: EvalAtRow> {
    pub eval: E,
    pub logup: LogupAtRow<2, E>,
    pub initial_memory_address: E::F,
    pub memory_lookup_elements: &'a LookupElements,
    pub rcunit2_lookup_elements: &'a LookupElements,
}

impl<'a, E: EvalAtRow> RangeCheck128BuiltinEval<'a, E> {
    pub fn eval(mut self) -> E {
        let mut values = [E::F::zero(); N_VALUES_FELTS + 1];

        // Memory address.
        // TODO: Use a constant column instead of taking the next_trace_mask().
        values[0] = self.initial_memory_address + self.eval.next_trace_mask();

        // Memory values.
        #[allow(clippy::needless_range_loop)]
        for i in ADDRESS_FELTS..N_VALUES_FELTS + ADDRESS_FELTS {
            values[i] = self.eval.next_trace_mask();
        }

        // Compute lookup for memory.
        self.logup.push_lookup(
            &mut self.eval,
            E::EF::one(),
            &values,
            *self.memory_lookup_elements,
        );

        // Compute lookup for last element range check.
        self.logup.push_lookup(
            &mut self.eval,
            E::EF::one(),
            &[values[N_VALUES_FELTS]],
            *self.rcunit2_lookup_elements,
        );

        self.logup.finalize(&mut self.eval);
        self.eval
    }
}

pub struct RangeCheck128BuiltinComponent {
    pub log_size: u32,
    pub initial_memory_address: M31,
    pub memory_lookup_elements: LookupElements,
    pub rcunit2_lookup_elements: LookupElements,
    pub claimed_sum: SecureField,
}

impl FrameworkComponent for RangeCheck128BuiltinComponent {
    fn log_size(&self) -> u32 {
        self.log_size
    }
    fn max_constraint_log_degree_bound(&self) -> u32 {
        self.log_size + 1
    }
    fn evaluate<E: EvalAtRow>(&self, mut eval: E) -> E {
        let [is_first] = eval.next_interaction_mask(2, [0]);
        let rc_builtin_eval = RangeCheck128BuiltinEval {
            eval,
            initial_memory_address: E::F::from(self.initial_memory_address),
            memory_lookup_elements: &self.memory_lookup_elements,
            rcunit2_lookup_elements: &self.rcunit2_lookup_elements,
            logup: LogupAtRow::new(1, self.claimed_sum, is_first),
        };
        rc_builtin_eval.eval()
    }
}
