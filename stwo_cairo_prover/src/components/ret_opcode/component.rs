use num_traits::{One, Zero};
use stwo_prover::constraint_framework::logup::LogupAtRow;
use stwo_prover::constraint_framework::{EvalAtRow, FrameworkComponent};
use stwo_prover::core::channel::Channel;
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::fields::qm31::SecureField;
use stwo_prover::core::fields::secure_column::SECURE_EXTENSION_DEGREE;
use stwo_prover::core::pcs::TreeVec;

use crate::components::memory::component::N_M31_IN_FELT252;
use crate::components::memory::MemoryLookupElements;

pub const RET_N_TRACE_CELLS: usize = 7;

#[derive(Clone)]
pub struct RetOpcodeComponent {
    pub log_size: u32,
    pub lookup_elements: MemoryLookupElements,
    pub claimed_sum: SecureField,
}
impl RetOpcodeComponent {
    pub fn new(
        ret_claim: RetOpcodeClaim,
        memory_lookup_elements: MemoryLookupElements,
        interaction_claim: RetOpcodeInteractionClaim,
    ) -> Self {
        Self {
            log_size: ret_claim.log_size,
            lookup_elements: memory_lookup_elements,
            claimed_sum: interaction_claim.memory_claimed_sum,
        }
    }
}

impl FrameworkComponent for RetOpcodeComponent {
    fn log_size(&self) -> u32 {
        self.log_size
    }

    fn max_constraint_log_degree_bound(&self) -> u32 {
        self.log_size + 1
    }

    fn evaluate<E: EvalAtRow>(&self, mut eval: E) -> E {
        let mut logup = LogupAtRow::<1, E>::new(1, self.claimed_sum, self.log_size);

        // PC Column
        let mut values = [E::F::zero(); N_M31_IN_FELT252 + 1];
        values[0] = eval.next_trace_mask();
        values[1] = E::F::one();
        logup.push_lookup(&mut eval, E::EF::one(), &values, &self.lookup_elements);

        // TODO(Ohad): Add AP to the VM logup constraint.
        let _ap = eval.next_trace_mask();
        let fp = eval.next_trace_mask();

        // FP - 1
        let fp_minus_one_0 = eval.next_trace_mask();
        let fp_minus_one_1 = eval.next_trace_mask();
        values[0] = fp - E::F::one();
        values[1] = fp_minus_one_0;
        values[2] = fp_minus_one_1;
        logup.push_lookup(&mut eval, E::EF::one(), &values, &self.lookup_elements);

        // FP - 2
        let fp_minus_two_0 = eval.next_trace_mask();
        let fp_minus_two_1 = eval.next_trace_mask();
        values[0] = fp - E::F::from(M31::from(2));
        values[1] = fp_minus_two_0;
        values[2] = fp_minus_two_1;
        logup.push_lookup(&mut eval, E::EF::one(), &values, &self.lookup_elements);

        logup.finalize(&mut eval);
        eval
    }
}

#[derive(Clone)]
pub struct RetOpcodeClaim {
    pub log_size: u32,
    pub n_rets: usize,
}
impl RetOpcodeClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_nonce(self.log_size as u64);
        channel.mix_nonce(self.n_rets as u64);
    }

    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let interaction_0_log_sizes = vec![self.log_size; RET_N_TRACE_CELLS];
        let interaction_1_log_sizes = vec![self.log_size; SECURE_EXTENSION_DEGREE * 3];
        TreeVec::new(vec![interaction_0_log_sizes, interaction_1_log_sizes])
    }
}

#[derive(Clone)]
pub struct RetOpcodeInteractionClaim {
    // pub ret_claimed_sum: SecureField,
    pub log_size: u32,
    pub memory_claimed_sum: SecureField,
}
impl RetOpcodeInteractionClaim {
    pub fn log_sizes(&self) -> Vec<u32> {
        vec![self.log_size; 12]
    }
    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_felts(&[self.memory_claimed_sum]);
    }
}
