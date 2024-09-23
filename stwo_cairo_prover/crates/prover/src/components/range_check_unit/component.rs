use num_traits::{One, Zero};
use stwo_prover::constraint_framework::logup::LogupAtRow;
use stwo_prover::constraint_framework::{EvalAtRow, FrameworkComponent, FrameworkEval};
use stwo_prover::core::channel::Channel;
use stwo_prover::core::fields::m31::BaseField;
use stwo_prover::core::fields::qm31::SecureField;
use stwo_prover::core::fields::secure_column::SECURE_EXTENSION_DEGREE;
use stwo_prover::core::lookups::utils::Fraction;
use stwo_prover::core::pcs::TreeVec;

use super::RangeCheckElements;

pub type RangeCheckUnitComponent<const N_REPETITIONS: usize> =
    FrameworkComponent<RangeCheckUnitEval<N_REPETITIONS>>;

#[derive(Clone)]
pub struct RangeCheckUnitEval<const N_REPETITIONS: usize> {
    pub log_n_rows: u32,
    pub lookup_elements: RangeCheckElements,
    pub claimed_sum: SecureField,
}

impl<const N_REPETITIONS: usize> FrameworkEval for RangeCheckUnitEval<N_REPETITIONS> {
    fn log_size(&self) -> u32 {
        self.log_n_rows
    }
    fn max_constraint_log_degree_bound(&self) -> u32 {
        self.log_n_rows + 1
    }
    fn evaluate<E: EvalAtRow>(&self, mut eval: E) -> E {
        let mut logup = LogupAtRow::<E>::new(1, self.claimed_sum, self.log_size());
        let rc_value = eval.next_trace_mask();
        let mut frac = Fraction::new(E::EF::zero(), E::EF::one());
        for repetition in 0..N_REPETITIONS {
            let multiplicity = eval.next_trace_mask();
            frac = frac
                + Fraction::new(
                    E::EF::from(-multiplicity),
                    self.lookup_elements.combine(&[rc_value
                        + E::F::from(BaseField::from_u32_unchecked(
                            (repetition as u32) << self.log_n_rows,
                        ))]),
                );
        }
        logup.write_frac(&mut eval, frac);
        logup.finalize(&mut eval);

        eval
    }
}

/// Range check unit component claim.
/// `log_rc_height` is the log of the number of rows in the range check table, meaning that that
/// range checked is 0..(N_REPETITIONS * 2^log_rc_height).
#[derive(Clone)]
pub struct RangeCheckClaim<const N_REPETITIONS: usize> {
    pub log_rc_height: u32,
}
impl<const N_REPETITIONS: usize> RangeCheckClaim<N_REPETITIONS> {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        TreeVec::new(vec![
            vec![self.log_rc_height; 1 + N_REPETITIONS],
            vec![self.log_rc_height; SECURE_EXTENSION_DEGREE],
        ])
    }

    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_u64(self.log_rc_height as u64);
        channel.mix_u64(N_REPETITIONS as u64);
    }
}

#[derive(Clone)]
pub struct RangeCheckInteractionClaim<const N_REPETITIONS: usize> {
    pub claimed_sum: SecureField,
}
impl<const N_REPETITIONS: usize> RangeCheckInteractionClaim<N_REPETITIONS> {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_felts(&[self.claimed_sum]);
    }
}
