use stwo_prover::constraint_framework::logup::LogupAtRow;
use stwo_prover::constraint_framework::{EvalAtRow, FrameworkComponent};
use stwo_prover::core::channel::Channel;
use stwo_prover::core::fields::qm31::SecureField;
use stwo_prover::core::fields::secure_column::SECURE_EXTENSION_DEGREE;
use stwo_prover::core::pcs::TreeVec;

use super::prover::N_REPETITIONS;
use super::{FVmState, Opcode, OpcodeElements};
use crate::components::{StandardInteractionClaim, StandardLookupData};

#[derive(Clone)]
pub struct OpcodeComponent<O: Opcode> {
    pub log_size: u32,
    pub opcode_elements: OpcodeElements,
    pub claimed_sum: SecureField,
    phantom: std::marker::PhantomData<O>,
}
impl<O: Opcode> OpcodeComponent<O> {
    pub fn new(
        ret_claim: OpcodeClaim<O>,
        opcode_elements: OpcodeElements,
        interaction_claim: StandardInteractionClaim,
    ) -> Self {
        Self {
            log_size: ret_claim.log_size,
            opcode_elements,
            claimed_sum: interaction_claim.claimed_sum,
            phantom: std::marker::PhantomData,
        }
    }
}

impl<O: Opcode> FrameworkComponent for OpcodeComponent<O> {
    fn log_size(&self) -> u32 {
        self.log_size
    }

    fn max_constraint_log_degree_bound(&self) -> u32 {
        self.log_size + 1
    }

    // TODO: repetitions.
    fn evaluate<E: EvalAtRow>(&self, mut eval: E) -> E {
        let mut logup = LogupAtRow::<2, E>::new(1, self.claimed_sum, self.log_size);

        for i in 0..N_REPETITIONS {
            let state = FVmState {
                pc: eval.next_trace_mask(),
                fp: eval.next_trace_mask(),
                ap: eval.next_trace_mask(),
            };

            let _state = O::evaluate(&mut eval, &mut logup, state, &self.opcode_elements);

            // TODO: State lookups. Do it inside the opcode impl.
        }

        logup.finalize(&mut eval);
        eval
    }
}

#[derive(Clone)]
pub struct OpcodeClaim<O: Opcode> {
    pub log_size: u32,
    pub n_instances: usize,
    pub phantom: std::marker::PhantomData<O>,
}
impl<O: Opcode> OpcodeClaim<O> {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_nonce(self.log_size as u64);
        channel.mix_nonce(self.n_instances as u64);
    }

    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let interaction_0_log_sizes = vec![self.log_size; O::n_columns() * N_REPETITIONS];
        let interaction_1_log_sizes =
            vec![
                self.log_size;
                SECURE_EXTENSION_DEGREE * O::LookupData::N_LOOKUPS * N_REPETITIONS / 2
            ];
        TreeVec::new(vec![interaction_0_log_sizes, interaction_1_log_sizes])
    }
}
