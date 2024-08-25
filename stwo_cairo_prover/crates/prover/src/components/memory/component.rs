use std::usize;

use stwo_prover::constraint_framework::logup::LogupAtRow;
use stwo_prover::constraint_framework::{EvalAtRow, FrameworkComponent};
use stwo_prover::core::channel::Channel;
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::fields::qm31::QM31;
use stwo_prover::core::fields::secure_column::SECURE_EXTENSION_DEGREE;
use stwo_prover::core::pcs::TreeVec;

use super::MemoryElements;
use crate::components::StandardInteractionClaim;

pub const N_IDS_PER_LINE: usize = 4;

#[derive(Clone)]
pub struct AddrToIdComponent {
    pub log_size: u32,
    pub mem_elements: MemoryElements,
    pub claimed_sum: QM31,
}
impl AddrToIdComponent {
    pub fn new(
        claim: AddrToIdClaim,
        mem_elements: MemoryElements,
        interaction_claim: StandardInteractionClaim,
    ) -> Self {
        Self {
            log_size: claim.log_size,
            mem_elements,
            claimed_sum: interaction_claim.claimed_sum,
        }
    }
}

impl FrameworkComponent for AddrToIdComponent {
    fn log_size(&self) -> u32 {
        self.log_size
    }

    fn max_constraint_log_degree_bound(&self) -> u32 {
        self.log_size() + 1
    }

    fn evaluate<E: EvalAtRow>(&self, mut eval: E) -> E {
        let mut logup = LogupAtRow::<1, E>::new(1, self.claimed_sum, self.log_size());

        let [base_addr] = eval.next_interaction_mask(2, [0]);
        for i in 0..N_IDS_PER_LINE {
            let id = eval.next_trace_mask();
            let mult = eval.next_trace_mask();
            logup.push_lookup(
                &mut eval,
                (-mult).into(),
                &[
                    base_addr * M31::from(N_IDS_PER_LINE)
                        + E::F::from(M31::from_u32_unchecked(i as u32)),
                    id,
                ],
                &self.mem_elements.addr_to_id,
            );
        }
        logup.finalize(&mut eval);

        eval
    }
}

#[derive(Clone)]
pub struct AddrToIdClaim {
    pub log_size: u32,
}
impl AddrToIdClaim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        assert_eq!(N_IDS_PER_LINE % 2, 0);
        TreeVec::new(vec![
            vec![self.log_size; N_IDS_PER_LINE * 2],
            vec![self.log_size; SECURE_EXTENSION_DEGREE * N_IDS_PER_LINE / 2],
            vec![self.log_size; 1],
        ])
    }

    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_nonce(self.log_size as u64);
    }
}
