#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use num_traits::One;
use stwo_prover::constraint_framework::logup::{LogupAtRow, LookupElements};
use stwo_prover::constraint_framework::{EvalAtRow, FrameworkComponent};
use stwo_prover::core::backend::simd::m31::PackedM31;
use stwo_prover::core::channel::Channel;
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::fields::qm31::SecureField;
use stwo_prover::core::fields::secure_column::SECURE_EXTENSION_DEGREE;
use stwo_prover::core::pcs::TreeVec;

use crate::cairo_air::VmStateLookupElements;
use crate::components::memory::MemoryLookupElements;

pub struct Component {
    pub claim: Claim,
    pub interaction_claim: InteractionClaim,
    pub self_lookup_elements: VmStateLookupElements,
    pub memory_lookup_elements: MemoryLookupElements,
}
impl Component {
    pub fn new(
        claim: Claim,
        interaction_claim: InteractionClaim,
        self_lookup_elements: VmStateLookupElements,
        memory_lookup_elements: MemoryLookupElements,
    ) -> Self {
        Self {
            claim,
            interaction_claim,
            self_lookup_elements,
            memory_lookup_elements,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Claim {
    pub log_size: u32,
    pub n_calls: usize,
}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let interaction_0_log_sizes = vec![self.log_size; 8];
        let interaction_1_log_sizes = vec![self.log_size; SECURE_EXTENSION_DEGREE * 2];
        TreeVec::new(vec![interaction_0_log_sizes, interaction_1_log_sizes])
    }

    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_nonce(self.log_size as u64);
        channel.mix_nonce(self.n_calls as u64);
    }
}

#[derive(Copy, Clone)]
pub struct InteractionClaim {
    pub claimed_sum: SecureField,
}
impl InteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_felts(&[self.claimed_sum]);
    }
}

impl FrameworkComponent for Component {
    fn log_size(&self) -> u32 {
        self.claim.log_size
    }

    fn max_constraint_log_degree_bound(&self) -> u32 {
        self.log_size() + 1
    }

    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    fn evaluate<E: EvalAtRow>(&self, mut eval: E) -> E {
        let mut logup =
            LogupAtRow::<1, E>::new(1, self.interaction_claim.claimed_sum, self.claim.log_size);
        let trace_row: [_; 8] = std::array::from_fn(|_| eval.next_trace_mask());
        logup.push_lookup(
            &mut eval,
            E::EF::one(),
            &[
                trace_row[0],
                E::F::from(M31::from(511)),
                E::F::from(M31::from(447)),
                E::F::from(M31::from(511)),
                E::F::from(M31::from(47)),
                E::F::from(M31::from(0)),
                E::F::from(M31::from(60)),
                E::F::from(M31::from(16)),
                E::F::from(M31::from(0)),
                E::F::from(M31::from(0)),
                E::F::from(M31::from(0)),
                E::F::from(M31::from(0)),
                E::F::from(M31::from(0)),
                E::F::from(M31::from(0)),
                E::F::from(M31::from(0)),
                E::F::from(M31::from(0)),
                E::F::from(M31::from(0)),
                E::F::from(M31::from(0)),
                E::F::from(M31::from(0)),
                E::F::from(M31::from(0)),
                E::F::from(M31::from(0)),
                E::F::from(M31::from(0)),
                E::F::from(M31::from(0)),
                E::F::from(M31::from(0)),
                E::F::from(M31::from(0)),
                E::F::from(M31::from(0)),
                E::F::from(M31::from(0)),
                E::F::from(M31::from(0)),
                E::F::from(M31::from(0)),
            ],
            &self.memory_lookup_elements,
        );
        eval.add_constraint((trace_row[3] * (trace_row[3] - E::F::from(M31::from(1)))));
        eval.add_constraint((trace_row[4] * (trace_row[4] - E::F::from(M31::from(1)))));
        eval.add_constraint((trace_row[4] * (trace_row[3] - E::F::from(M31::from(1)))));
        logup.push_lookup(
            &mut eval,
            E::EF::one(),
            &[
                trace_row[0] + E::F::from(M31::from(1)),
                trace_row[5],
                trace_row[6],
                trace_row[7],
                (trace_row[4] * E::F::from(M31::from(511))),
                (trace_row[4] * E::F::from(M31::from(511))),
                (trace_row[4] * E::F::from(M31::from(511))),
                (trace_row[4] * E::F::from(M31::from(511))),
                (trace_row[4] * E::F::from(M31::from(511))),
                (trace_row[4] * E::F::from(M31::from(511))),
                (trace_row[4] * E::F::from(M31::from(511))),
                (trace_row[4] * E::F::from(M31::from(511))),
                (trace_row[4] * E::F::from(M31::from(511))),
                (trace_row[4] * E::F::from(M31::from(511))),
                (trace_row[4] * E::F::from(M31::from(511))),
                (trace_row[4] * E::F::from(M31::from(511))),
                (trace_row[4] * E::F::from(M31::from(511))),
                (trace_row[4] * E::F::from(M31::from(511))),
                (trace_row[4] * E::F::from(M31::from(511))),
                (trace_row[4] * E::F::from(M31::from(511))),
                (trace_row[4] * E::F::from(M31::from(511))),
                (trace_row[4] * E::F::from(M31::from(511))),
                ((E::F::from(M31::from(136)) * trace_row[3]) - trace_row[4]),
                E::F::from(M31::from(0)),
                E::F::from(M31::from(0)),
                E::F::from(M31::from(0)),
                E::F::from(M31::from(0)),
                E::F::from(M31::from(0)),
                (trace_row[3] * E::F::from(M31::from(256))),
            ],
            &self.memory_lookup_elements,
        );

        // logup.push_lookup(
        //     &mut eval,
        //     -E::EF::one(),
        //     &[
        //         trace_row[0],
        //         trace_row[1],
        //         trace_row[2],
        //         (trace_row[0] + E::F::from(M31::from(2))),
        //         (trace_row[1]
        //             + ((((trace_row[9] * E::F::from(M31::from(262144)))
        //                 + ((trace_row[8] * E::F::from(M31::from(512))) + trace_row[7]))
        //                 - trace_row[5])
        //                 - (E::F::from(M31::from(134217728)) * trace_row[6]))),
        //         trace_row[2],
        //     ],
        //     &self.self_lookup_elements,
        // );
        logup.finalize(&mut eval);

        eval
    }
}
