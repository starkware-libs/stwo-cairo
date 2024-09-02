use stwo_prover::constraint_framework::logup::LogupAtRow;
use stwo_prover::constraint_framework::{EvalAtRow, FrameworkEval};
use stwo_prover::core::channel::Channel;
use stwo_prover::core::fields::qm31::SecureField;
use stwo_prover::core::fields::secure_column::SECURE_EXTENSION_DEGREE;
use stwo_prover::core::pcs::TreeVec;

use super::RangeCheckElements;

#[derive(Clone)]
pub struct RangeCheckUnitComponent {
    pub log_n_rows: u32,
    pub lookup_elements: RangeCheckElements,
    pub claimed_sum: SecureField,
}

impl FrameworkEval for RangeCheckUnitComponent {
    fn log_size(&self) -> u32 {
        self.log_n_rows
    }
    fn max_constraint_log_degree_bound(&self) -> u32 {
        self.log_n_rows + 1
    }
    fn evaluate<E: EvalAtRow>(&self, mut eval: E) -> E {
        let mut logup = LogupAtRow::<1, E>::new(1, self.claimed_sum, self.log_size());
        let rc_value = eval.next_trace_mask();
        let multiplicity = eval.next_trace_mask();
        logup.push_lookup(
            &mut eval,
            (-multiplicity).into(),
            &[rc_value],
            &self.lookup_elements,
        );
        logup.finalize(&mut eval);

        eval
    }
}

#[derive(Clone)]
pub struct RangeCheckClaim {
    pub log_rc_max: u32,
}
impl RangeCheckClaim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        TreeVec::new(vec![
            vec![self.log_rc_max; 2],
            vec![self.log_rc_max; SECURE_EXTENSION_DEGREE],
        ])
    }

    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_nonce(self.log_rc_max as u64);
    }
}

#[derive(Clone)]
pub struct RangeCheckInteractionClaim {
    pub claimed_sum: SecureField,
}
impl RangeCheckInteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_felts(&[self.claimed_sum]);
    }
}
