use serde::{Deserialize, Serialize};
use stwo_prover::constraint_framework::logup::{LogupAtRow, LookupElements};
use stwo_prover::constraint_framework::preprocessed_columns::PreprocessedColumn;
use stwo_prover::constraint_framework::{EvalAtRow, FrameworkEval, INTERACTION_TRACE_IDX};
use stwo_prover::core::channel::Channel;
use stwo_prover::core::fields::qm31::SecureField;
use stwo_prover::core::fields::secure_column::SECURE_EXTENSION_DEGREE;
use stwo_prover::core::lookups::utils::Fraction;
use stwo_prover::core::pcs::TreeVec;

const N_MULTIPLICITY_COLUMNS: usize = 1;

#[derive(Clone)]
pub struct RangeCheckVectorEval<const N: usize> {
    pub log_ranges: [u32; N],
    pub lookup_elements: LookupElements<N>,
    pub claimed_sum: SecureField,
}

impl<const N: usize> FrameworkEval for RangeCheckVectorEval<N> {
    fn log_size(&self) -> u32 {
        self.log_ranges.iter().sum()
    }
    fn max_constraint_log_degree_bound(&self) -> u32 {
        self.log_size() + 1
    }
    fn evaluate<E: EvalAtRow>(&self, mut eval: E) -> E {
        let is_first = eval.get_preprocessed_column(PreprocessedColumn::IsFirst(self.log_size()));
        let mut logup =
            LogupAtRow::<E>::new(INTERACTION_TRACE_IDX, self.claimed_sum, None, is_first);
        let rc_values: [E::F; N] = std::array::from_fn(|_| eval.next_trace_mask());
        let multiplicity = eval.next_trace_mask();
        logup.write_frac(
            &mut eval,
            Fraction::new(
                E::EF::from(-multiplicity),
                self.lookup_elements.combine(&rc_values),
            ),
        );
        logup.finalize(&mut eval);

        eval
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct RangeCheckClaim {
    pub log_ranges: Vec<u32>,
}
impl RangeCheckClaim {
    fn log_size(&self) -> u32 {
        self.log_ranges.iter().sum()
    }

    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        TreeVec::new(vec![
            vec![self.log_size(); 1],
            vec![self.log_size(); self.log_ranges.len() + N_MULTIPLICITY_COLUMNS],
            vec![self.log_size(); SECURE_EXTENSION_DEGREE],
        ])
    }

    pub fn mix_into(&self, channel: &mut impl Channel) {
        for &log_range in self.log_ranges.iter() {
            channel.mix_u64(log_range as u64);
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct RangeCheckInteractionClaim {
    pub claimed_sum: SecureField,
}
impl RangeCheckInteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_felts(&[self.claimed_sum]);
    }
}
