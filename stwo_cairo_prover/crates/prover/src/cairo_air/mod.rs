use itertools::{chain, Itertools};
use num_traits::Zero;
use stwo_prover::core::air::{Component, ComponentProver};
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::channel::{Blake2sChannel, Channel};
use stwo_prover::core::fields::m31::{BaseField, M31};
use stwo_prover::core::fields::qm31::{SecureField, QM31};
use stwo_prover::core::fields::{FieldExpOps, IntoSlice};
use stwo_prover::core::pcs::{CommitmentSchemeProver, CommitmentSchemeVerifier, TreeVec};
use stwo_prover::core::poly::circle::{CanonicCoset, PolyOps};
use stwo_prover::core::prover::{prove, verify, StarkProof, VerificationError, LOG_BLOWUP_FACTOR};
use stwo_prover::core::vcs::blake2_hash::Blake2sHasher;
use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleHasher;
use stwo_prover::core::vcs::ops::MerkleHasher;
use stwo_prover::core::InteractionElements;
use thiserror::Error;

use crate::components::memory::component::{MemoryClaim, MemoryComponent, MemoryInteractionClaim};
use crate::components::memory::prover::MemoryClaimProver;
use crate::components::memory::MemoryLookupElements;
use crate::components::ret_opcode::component::{
    RetOpcodeClaim, RetOpcodeComponent, RetOpcodeInteractionClaim,
};
use crate::components::ret_opcode::prover::RetOpcodeClaimProver;
use crate::input::instructions::VmState;
use crate::input::{CairoInput, SegmentAddrs};
use crate::felt::split_f252;

pub struct CairoProof<H: MerkleHasher> {
    pub claim: CairoClaim,
    pub interaction_claim: CairoInteractionClaim,
    pub stark_proof: StarkProof<H>,
}

pub struct CairoClaim {
    // Common claim values.
    pub public_memory: Vec<(u32, [u32; 8])>,
    pub initial_state: VmState,
    pub final_state: VmState,
    pub range_check: SegmentAddrs,

    pub ret: Vec<RetOpcodeClaim>,
    pub memory: MemoryClaim,
    // ...
}
impl CairoClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        // TODO(spapini): Add common values.
        self.ret.iter().for_each(|c| c.mix_into(channel));
        self.memory.mix_into(channel);
    }

    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        TreeVec::concat_cols(chain!(
            self.ret.iter().map(|c| c.log_sizes()),
            [self.memory.log_sizes()]
        ))
    }
}

pub struct CairoInteractionElements {
    memory_lookup: MemoryLookupElements,
    // ...
}
impl CairoInteractionElements {
    pub fn draw(channel: &mut impl Channel) -> CairoInteractionElements {
        CairoInteractionElements {
            memory_lookup: MemoryLookupElements::draw(channel),
        }
    }
}

pub struct CairoInteractionClaim {
    pub ret: Vec<RetOpcodeInteractionClaim>,
    pub memory: MemoryInteractionClaim,
    // ...
}
impl CairoInteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        self.ret.iter().for_each(|c| c.mix_into(channel));
        self.memory.mix_into(channel);
    }
}

pub fn lookup_sum_valid(
    claim: &CairoClaim,
    elements: &CairoInteractionElements,
    interaction_claim: &CairoInteractionClaim,
) -> bool {
    let mut sum = QM31::zero();
    // Public memory.
    // TODO(spapini): Optimized inverse.
    sum += claim
        .public_memory
        .iter()
        .map(|(addr, val)| {
            elements
                .memory_lookup
                .combine::<M31, QM31>(
                    &[
                        [M31::from_u32_unchecked(*addr)].as_slice(),
                        split_f252(*val).as_slice(),
                    ]
                    .concat(),
                )
                .inverse()
        })
        .sum::<SecureField>();
    // TODO: include initial and final state.
    sum += interaction_claim.memory.claimed_sum;
    sum += interaction_claim.ret[0].claimed_sum;
    sum == SecureField::zero()
}

pub struct CairoComponentGenerator {
    ret: Vec<RetOpcodeComponent>,
    memory: MemoryComponent,
    // ...
}

impl CairoComponentGenerator {
    pub fn new(
        cairo_claim: &CairoClaim,
        interaction_elements: &CairoInteractionElements,
        interaction_claim: &CairoInteractionClaim,
    ) -> Self {
        let ret_components = cairo_claim
            .ret
            .iter()
            .zip(interaction_claim.ret.iter())
            .map(|(claim, interaction_claim)| {
                RetOpcodeComponent::new(
                    claim.clone(),
                    interaction_elements.memory_lookup.clone(),
                    interaction_claim.clone(),
                )
            })
            .collect_vec();
        let memory_component = MemoryComponent::new(
            cairo_claim.memory.clone(),
            interaction_elements.memory_lookup.clone(),
            interaction_claim.memory.clone(),
        );
        Self {
            ret: ret_components,
            memory: memory_component,
        }
    }

    pub fn provers(&self) -> Vec<&dyn ComponentProver<SimdBackend>> {
        let mut vec: Vec<&dyn ComponentProver<SimdBackend>> = vec![];
        for ret in self.ret.iter() {
            vec.push(ret);
        }
        vec.push(&self.memory);
        vec
    }

    pub fn components(&self) -> Vec<&dyn Component> {
        let mut vec: Vec<&dyn Component> = vec![];
        for ret in self.ret.iter() {
            vec.push(ret);
        }
        vec.push(&self.memory);
        vec
    }
}

const LOG_MAX_ROWS: u32 = 20;
pub fn prove_cairo(input: CairoInput) -> CairoProof<Blake2sMerkleHasher> {
    let twiddles = SimdBackend::precompute_twiddles(
        CanonicCoset::new(LOG_MAX_ROWS + LOG_BLOWUP_FACTOR + 2)
            .circle_domain()
            .half_coset,
    );

    // Setup protocol.
    let channel = &mut Blake2sChannel::new(Blake2sHasher::hash(&[]));
    let commitment_scheme = &mut CommitmentSchemeProver::new(LOG_BLOWUP_FACTOR, &twiddles);

    // Extract public memory.
    let public_memory = input
        .public_mem_addresses
        .iter()
        .copied()
        .map(|a| (a, input.mem.get(a).as_u256()))
        .collect_vec();

    // TODO: Table interaction.

    // Base trace.
    // TODO(Ohad): change to OpcodeClaimProvers, and integrate padding.
    let ret_trace_generator = RetOpcodeClaimProver::new(input.instructions.ret);
    let mut memory_trace_generator = MemoryClaimProver::new(input.mem);

    // Add public memory.
    for addr in &input.public_mem_addresses {
        memory_trace_generator.add_inputs(M31::from_u32_unchecked(*addr));
    }

    let mut tree_builder = commitment_scheme.tree_builder();
    let (ret_claim, ret_interaction_prover) =
        ret_trace_generator.write_trace(&mut tree_builder, &mut memory_trace_generator);
    let (memory_claim, memory_interaction_prover) =
        memory_trace_generator.write_trace(&mut tree_builder);

    // Commit to the claim and the trace.
    let claim = CairoClaim {
        public_memory,
        initial_state: input.instructions.initial_state,
        final_state: input.instructions.final_state,
        range_check: input.range_check,
        ret: vec![ret_claim],
        memory: memory_claim.clone(),
    };
    claim.mix_into(channel);
    tree_builder.commit(channel);

    // Draw interaction elements.
    let interaction_elements = CairoInteractionElements::draw(channel);

    // Interaction trace.
    let mut tree_builder = commitment_scheme.tree_builder();
    let ret_interaction_claim = ret_interaction_prover
        .write_interaction_trace(&mut tree_builder, &interaction_elements.memory_lookup);
    let memory_interaction_claim = memory_interaction_prover
        .write_interaction_trace(&mut tree_builder, &interaction_elements.memory_lookup);

    // Commit to the interaction claim and the interaction trace.
    let interaction_claim = CairoInteractionClaim {
        ret: vec![ret_interaction_claim.clone()],
        memory: memory_interaction_claim.clone(),
    };
    debug_assert!(lookup_sum_valid(
        &claim,
        &interaction_elements,
        &interaction_claim
    ));
    interaction_claim.mix_into(channel);
    tree_builder.commit(channel);

    // Component provers.
    let component_builder =
        CairoComponentGenerator::new(&claim, &interaction_elements, &interaction_claim);
    let components = component_builder.provers();

    // Prove stark.
    let proof = prove::<SimdBackend, _, _>(
        &components,
        channel,
        &InteractionElements::default(),
        commitment_scheme,
    )
    .unwrap();

    CairoProof {
        claim,
        interaction_claim,
        stark_proof: proof,
    }
}

pub fn verify_cairo(
    CairoProof {
        claim,
        interaction_claim,
        stark_proof,
    }: CairoProof<Blake2sMerkleHasher>,
) -> Result<(), CairoVerificationError> {
    // Verify.
    let channel = &mut Blake2sChannel::new(Blake2sHasher::hash(BaseField::into_slice(&[])));
    let commitment_scheme_verifier = &mut CommitmentSchemeVerifier::<Blake2sMerkleHasher>::new();

    claim.mix_into(channel);
    commitment_scheme_verifier.commit(stark_proof.commitments[0], &claim.log_sizes()[0], channel);
    let interaction_elements = CairoInteractionElements::draw(channel);
    if !lookup_sum_valid(&claim, &interaction_elements, &interaction_claim) {
        return Err(CairoVerificationError::InvalidLogupSum);
    }
    interaction_claim.mix_into(channel);
    commitment_scheme_verifier.commit(stark_proof.commitments[1], &claim.log_sizes()[1], channel);

    let component_generator =
        CairoComponentGenerator::new(&claim, &interaction_elements, &interaction_claim);
    let components = component_generator.components();

    verify(
        &components,
        channel,
        &InteractionElements::default(),
        commitment_scheme_verifier,
        stark_proof,
    )
    .map_err(CairoVerificationError::Stark)
}

#[derive(Error, Debug)]
pub enum CairoVerificationError {
    #[error("Invalid logup sum")]
    InvalidLogupSum,
    #[error("Stark verification error: {0}")]
    Stark(#[from] VerificationError),
}

#[cfg(test)]
mod tests {
    use cairo_lang_casm::casm;

    use crate::cairo_air::{prove_cairo, verify_cairo, CairoInput};
    use crate::input::plain::input_from_plain_casm;

    fn test_input() -> CairoInput {
        let instructions = casm! {
            [ap] = 10, ap++;
            call rel 4;
            jmp rel 11;

            jmp rel 4 if [fp-3] != 0;
            jmp rel 6;
            [ap] = [fp-3] + (-1), ap++;
            call rel (-6);
            ret;
        }
        .instructions;

        input_from_plain_casm(instructions)
    }

    #[test]
    fn test_cairo_air() {
        let cairo_proof = prove_cairo(test_input());

        verify_cairo(cairo_proof).unwrap();
    }
}
