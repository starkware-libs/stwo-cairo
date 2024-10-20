use serde::{Deserialize, Serialize};
use stwo_prover::constraint_framework::logup::LogupAtRow;
use stwo_prover::constraint_framework::{EvalAtRow, FrameworkEval};
use stwo_prover::core::channel::Channel;
use stwo_prover::core::fields::qm31::{SecureField, QM31};
use stwo_prover::core::fields::secure_column::SECURE_EXTENSION_DEGREE;
use stwo_prover::core::lookups::utils::Fraction;
use stwo_prover::core::pcs::TreeVec;

use super::AddrToIdLookupElements;
pub const N_ADDR_TO_ID_COLUMNS: usize = 3;

// TODO(ShaharS): Break to repititions in order to batch the logup.
#[derive(Clone)]
pub struct AddrToIdEval {
    pub log_n_rows: u32,
    pub lookup_elements: AddrToIdLookupElements,
    pub claimed_sum: QM31,
}
impl AddrToIdEval {
    // TODO(ShaharS): use Seq column for address, and also use repititions.
    pub const fn n_columns(&self) -> usize {
        N_ADDR_TO_ID_COLUMNS
    }

    pub fn new(
        claim: AddrToIdClaim,
        lookup_elements: AddrToIdLookupElements,
        interaction_claim: AddrToIdInteractionClaim,
    ) -> Self {
        Self {
            log_n_rows: claim.log_size,
            lookup_elements,
            claimed_sum: interaction_claim.claimed_sum,
        }
    }
}

impl FrameworkEval for AddrToIdEval {
    fn log_size(&self) -> u32 {
        self.log_n_rows
    }

    fn max_constraint_log_degree_bound(&self) -> u32 {
        self.log_size() + 1
    }

    fn evaluate<E: EvalAtRow>(&self, mut eval: E) -> E {
        let [is_first] = eval.next_interaction_mask(2, [0]);
        let mut logup = LogupAtRow::<E>::new(1, self.claimed_sum, None, is_first);

        let address_and_id: [E::F; 2] = std::array::from_fn(|_| eval.next_trace_mask());
        let multiplicity = eval.next_trace_mask();
        let frac = Fraction::new(
            E::EF::from(-multiplicity),
            self.lookup_elements.combine(&address_and_id),
        );
        logup.write_frac(&mut eval, frac);
        logup.finalize(&mut eval);

        eval
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AddrToIdClaim {
    pub log_size: u32,
}
impl AddrToIdClaim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let interaction_0_log_size = vec![self.log_size; N_ADDR_TO_ID_COLUMNS];
        let interaction_1_log_size = vec![self.log_size; SECURE_EXTENSION_DEGREE];
        TreeVec::new(vec![interaction_0_log_size, interaction_1_log_size])
    }

    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_u64(self.log_size as u64);
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AddrToIdInteractionClaim {
    pub claimed_sum: SecureField,
}
impl AddrToIdInteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_felts(&[self.claimed_sum]);
    }
}
