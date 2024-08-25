use itertools::{chain, Itertools};
use num_traits::Zero;
use stwo_prover::core::air::{Component, ComponentProver};
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::channel::{Blake2sChannel, Channel};
use stwo_prover::core::fields::qm31::{SecureField, QM31};
use stwo_prover::core::pcs::{
    CommitmentSchemeProver, CommitmentSchemeVerifier, PcsConfig, TreeVec,
};
use stwo_prover::core::poly::circle::{CanonicCoset, PolyOps};
use stwo_prover::core::prover::{prove, verify, StarkProof, VerificationError};
use stwo_prover::core::vcs::blake2_merkle::{Blake2sMerkleChannel, Blake2sMerkleHasher};
use stwo_prover::core::vcs::ops::MerkleHasher;
use stwo_prover::core::InteractionElements;
use thiserror::Error;

use crate::components::memory::{AddrToIdClaim, AddrToIdComponent, AddrToIdProverBuilder};
use crate::components::opcode::{CpuRangeProvers, OpcodeElements, OpcodeGenContext};
use crate::components::range_check::RangeProver;
use crate::components::ret_opcode::{RetOpcode, RetProver};
use crate::components::{StandardClaim, StandardComponent, StandardInteractionClaim};
use crate::input::instructions::VmState;
use crate::input::{CairoInput, SegmentAddrs};

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

    // Opcodes.
    pub ret: Vec<StandardClaim<RetOpcode>>,

    // Memory.
    pub addr_to_id: AddrToIdClaim,
}
impl CairoClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        // TODO(spapini): Add common values.
        self.ret.iter().for_each(|c| c.mix_into(channel));
        self.addr_to_id.mix_into(channel);
    }

    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        TreeVec::concat_cols(chain!(
            self.ret.iter().map(|c| c.log_sizes()),
            [self.addr_to_id.log_sizes()]
        ))
    }
}

pub struct CairoInteractionElements {
    opcode: OpcodeElements,
    // ...
}
impl CairoInteractionElements {
    pub fn draw(channel: &mut impl Channel) -> CairoInteractionElements {
        CairoInteractionElements {
            opcode: OpcodeElements::draw(channel),
        }
    }
}

pub struct CairoInteractionClaim {
    pub ret: Vec<StandardInteractionClaim>,
    pub addr_to_id: StandardInteractionClaim,
}
impl CairoInteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        self.ret.iter().for_each(|c| c.mix_into(channel));
        self.addr_to_id.mix_into(channel);
    }
}

pub fn lookup_sum_valid(
    _claim: &CairoClaim,
    _elements: &CairoInteractionElements,
    interaction_claim: &CairoInteractionClaim,
) -> bool {
    let mut sum = QM31::zero();
    // // Public memory.
    // // TODO(spapini): Optimized inverse.
    // sum += claim
    //     .public_memory
    //     .iter()
    //     .map(|(addr, val)| {
    //         elements
    //             .memory_lookup
    //             .combine::<M31, QM31>(
    //                 &[
    //                     [M31::from_u32_unchecked(*addr)].as_slice(),
    //                     split_f252(*val).as_slice(),
    //                 ]
    //                 .concat(),
    //             )
    //             .inverse()
    //     })
    //     .sum::<SecureField>();
    // TODO: include initial and final state.
    sum += interaction_claim.addr_to_id.claimed_sum;
    sum += interaction_claim.ret[0].claimed_sum;
    sum == SecureField::zero()
}

pub struct CairoComponentGenerator {
    ret: Vec<StandardComponent<RetOpcode>>,
    addr_to_id: AddrToIdComponent,
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
                StandardComponent::new(
                    claim.clone(),
                    interaction_elements.opcode.clone(),
                    interaction_claim.clone(),
                )
            })
            .collect_vec();
        let addr_to_id = AddrToIdComponent::new(
            cairo_claim.addr_to_id.clone(),
            interaction_elements.opcode.memory_elements.clone(),
            interaction_claim.addr_to_id.clone(),
        );
        Self {
            ret: ret_components,
            addr_to_id,
        }
    }

    pub fn provers(&self) -> Vec<&dyn ComponentProver<SimdBackend>> {
        let mut vec: Vec<&dyn ComponentProver<SimdBackend>> = vec![];
        for ret in self.ret.iter() {
            vec.push(ret);
        }
        vec.push(&self.addr_to_id);
        vec
    }

    pub fn components(&self) -> Vec<&dyn Component> {
        let mut vec: Vec<&dyn Component> = vec![];
        for ret in self.ret.iter() {
            vec.push(ret);
        }
        vec.push(&self.addr_to_id);
        vec
    }
}

const LOG_MAX_ROWS: u32 = 20;
pub fn prove_cairo(config: PcsConfig, input: CairoInput) -> CairoProof<Blake2sMerkleHasher> {
    let twiddles = SimdBackend::precompute_twiddles(
        CanonicCoset::new(LOG_MAX_ROWS + config.fri_config.log_blowup_factor + 2)
            .circle_domain()
            .half_coset,
    );

    // Setup protocol.
    let channel = &mut Blake2sChannel::default();
    let commitment_scheme = &mut CommitmentSchemeProver::new(config, &twiddles);

    // Extract public memory.
    let public_memory = input
        .public_mem_addresses
        .iter()
        .copied()
        .map(|a| (a, input.mem.get(a).as_u256()))
        .collect_vec();

    // TODO: Table interaction.

    // Base trace.
    let ret_prover = RetProver::new(input.instructions.ret.into_iter())
        .pop()
        .unwrap();
    let mut addr_to_id = AddrToIdProverBuilder::new(input.mem.address_to_id);
    let opcode_ctx = &mut OpcodeGenContext {
        addr_to_id: &mut addr_to_id,
        range: CpuRangeProvers {
            range2: &mut RangeProver,
            range3: &mut RangeProver,
        },
    };

    // Add public memory.
    for addr in &input.public_mem_addresses {
        opcode_ctx.addr_to_id.add_inputs(*addr);
    }

    let mut tree_builder = commitment_scheme.tree_builder();
    let (ret_claim, ret) = ret_prover.write_trace(&mut tree_builder, opcode_ctx);
    let addr_to_id = addr_to_id.build();
    let (memory_claim, addr_to_id) = addr_to_id.write_trace(&mut tree_builder, &mut ());

    // Commit to the claim and the trace.
    let claim = CairoClaim {
        public_memory,
        initial_state: input.instructions.initial_state,
        final_state: input.instructions.final_state,
        range_check: input.range_check,
        ret: vec![ret_claim],
        addr_to_id: memory_claim.clone(),
    };
    claim.mix_into(channel);
    tree_builder.commit(channel);

    // Draw interaction elements.
    let interaction_elements = CairoInteractionElements::draw(channel);

    // Interaction trace.
    let mut tree_builder = commitment_scheme.tree_builder();
    let ret_interaction_claim =
        ret.write_interaction_trace(&mut tree_builder, &interaction_elements.opcode);
    let addr_to_id_interaction_claim = addr_to_id.write_interaction_trace(
        &mut tree_builder,
        &interaction_elements.opcode.memory_elements.addr_to_id,
    );

    // Commit to the interaction claim and the interaction trace.
    let interaction_claim = CairoInteractionClaim {
        ret: vec![ret_interaction_claim.clone()],
        addr_to_id: addr_to_id_interaction_claim.clone(),
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
    let proof = prove::<SimdBackend, _>(
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
    config: PcsConfig,
    CairoProof {
        claim,
        interaction_claim,
        stark_proof,
    }: CairoProof<Blake2sMerkleHasher>,
) -> Result<(), CairoVerificationError> {
    // Verify.
    let channel = &mut Blake2sChannel::default();
    let commitment_scheme_verifier =
        &mut CommitmentSchemeVerifier::<Blake2sMerkleChannel>::new(config);

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
    use stwo_prover::core::pcs::PcsConfig;

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
        let config = PcsConfig::default();
        let cairo_proof = prove_cairo(config, test_input());

        verify_cairo(config, cairo_proof).unwrap();
    }
}
