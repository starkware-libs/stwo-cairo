use num_traits::{One, Zero};
use stwo_prover::constraint_framework::logup::LogupAtRow;
use stwo_prover::constraint_framework::{EvalAtRow, FrameworkComponent, FrameworkEval};
use stwo_prover::core::channel::Channel;
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::fields::qm31::SecureField;
use stwo_prover::core::fields::secure_column::SECURE_EXTENSION_DEGREE;
use stwo_prover::core::lookups::utils::Fraction;
use stwo_prover::core::pcs::TreeVec;

use crate::components::memory::component::N_M31_IN_FELT252;
use crate::components::memory::MemoryLookupElements;

pub const RET_N_TRACE_CELLS: usize = 7;
pub const RET_INSTRUCTION: [u32; N_M31_IN_FELT252] = [
    510, 447, 511, 495, 511, 91, 130, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
pub type RetOpcodeComponent = FrameworkComponent<RetOpcodeEval>;

#[derive(Clone)]
pub struct RetOpcodeEval {
    pub log_size: u32,
    pub lookup_elements: MemoryLookupElements,
    pub claimed_sum: SecureField,
}
impl RetOpcodeEval {
    pub fn new(
        ret_claim: RetOpcodeClaim,
        memory_lookup_elements: MemoryLookupElements,
        interaction_claim: RetOpcodeInteractionClaim,
    ) -> Self {
        Self {
            log_size: ret_claim.n_rets.next_power_of_two().ilog2(),
            lookup_elements: memory_lookup_elements,
            claimed_sum: interaction_claim.claimed_sum,
        }
    }
}

impl FrameworkEval for RetOpcodeEval {
    fn log_size(&self) -> u32 {
        self.log_size
    }

    fn max_constraint_log_degree_bound(&self) -> u32 {
        self.log_size + 1
    }

    fn evaluate<E: EvalAtRow>(&self, mut eval: E) -> E {
        let mut logup = LogupAtRow::<E>::new(1, self.claimed_sum, self.log_size);

        // PC Column
        let mut values = [E::F::zero(); N_M31_IN_FELT252 + 1];
        values[0] = eval.next_trace_mask();
        for i in 0..N_M31_IN_FELT252 {
            values[i + 1] = E::F::from(M31::from(RET_INSTRUCTION[i]));
        }
        let frac = Fraction::new(E::EF::one(), self.lookup_elements.combine(&values));
        logup.write_frac(&mut eval, frac);
        for i in 0..N_M31_IN_FELT252 {
            values[i + 1] = E::F::from(M31::from(0));
        }

        // TODO(Ohad): Add AP to the VM logup constraint.
        let _ap = eval.next_trace_mask();
        let fp = eval.next_trace_mask();

        // FP - 1
        let fp_minus_one_0 = eval.next_trace_mask();
        let fp_minus_one_1 = eval.next_trace_mask();
        values[0] = fp - E::F::one();
        values[1] = fp_minus_one_0;
        values[2] = fp_minus_one_1;
        let frac = Fraction::new(E::EF::one(), self.lookup_elements.combine(&values));
        logup.write_frac(&mut eval, frac);

        // FP - 2
        let fp_minus_two_0 = eval.next_trace_mask();
        let fp_minus_two_1 = eval.next_trace_mask();
        values[0] = fp - E::F::from(M31::from(2));
        values[1] = fp_minus_two_0;
        values[2] = fp_minus_two_1;
        let frac = Fraction::new(E::EF::one(), self.lookup_elements.combine(&values));
        logup.write_frac(&mut eval, frac);

        logup.finalize(&mut eval);
        eval
    }
}

#[derive(Clone)]
pub struct RetOpcodeClaim {
    pub n_rets: usize,
}
impl RetOpcodeClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_u64(self.n_rets as u64);
    }

    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let log_size = self.n_rets.next_power_of_two().ilog2();
        let interaction_0_log_sizes = vec![log_size; RET_N_TRACE_CELLS];
        let interaction_1_log_sizes = vec![log_size; SECURE_EXTENSION_DEGREE * 3];
        TreeVec::new(vec![interaction_0_log_sizes, interaction_1_log_sizes])
    }
}

#[derive(Clone)]
pub struct RetOpcodeInteractionClaim {
    pub log_size: u32,
    pub claimed_sum: SecureField,
}
impl RetOpcodeInteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_felts(&[self.claimed_sum]);
    }
}
