use num_traits::{One, Zero};
use stwo_prover::constraint_framework::logup::{LogupAtRow, LookupElements};
use stwo_prover::constraint_framework::{EvalAtRow, FrameworkComponent};
use stwo_prover::core::channel::Channel;
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::fields::qm31::SecureField;

use crate::components::memory::component::{N_M31_IN_FELT252, N_MEMORY_COLUMNS};

pub const RET_N_TRACE_CELLS: usize = 7;

#[derive(Clone)]
pub struct RetOpcodeComponent {
    pub log_size: u32,
    pub claimed_sum: SecureField,
    pub lookup_elements: LookupElements<N_MEMORY_COLUMNS>,
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

        // No constraint on AP (YET).
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

pub struct RetOpcodeClaim {
    pub log_size: u32,
    pub n_rets: usize,
}
impl RetOpcodeClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_nonce(self.log_size as u64);
        channel.mix_nonce(self.n_rets as u64);
    }

    pub fn log_sizes(&self) -> Vec<u32> {
        vec![self.log_size; RET_N_TRACE_CELLS]
    }
}

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
        channel.mix_nonce(self.log_size as u64);
        // self.ret_claimed_sum.mix_into(channel);
        channel.mix_felts(&[self.memory_claimed_sum]);
    }
}
