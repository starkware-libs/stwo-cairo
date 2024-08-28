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
use tracing::{span, Level};

use crate::components::memory::{
    MemoryAndRangeElements, MemoryClaim, MemoryComponents, MemoryInteractionClaim,
    MemoryProverBuilder,
};
use crate::components::opcode::{
    CpuRangeProvers, OpcodeElements, OpcodeGenContext, OpcodesClaim, OpcodesComponents,
    OpcodesInteractionClaim, OpcodesProvers,
};
use crate::components::range_check::RangeProver;
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
    pub opcodes: OpcodesClaim,

    // Memory.
    pub mem: MemoryClaim,
}
impl CairoClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        // TODO(spapini): Add common values.
        self.mem.mix_into(channel);
        self.opcodes.mix_into(channel);
    }

    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        TreeVec::concat_cols([self.mem.log_sizes(), self.opcodes.log_sizes()].into_iter())
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
    pub opcodes: OpcodesInteractionClaim,
    pub mem: MemoryInteractionClaim,
}
impl CairoInteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        self.opcodes.mix_into(channel);
        self.mem.mix_into(channel);
    }
}

pub fn lookup_sum_valid(
    _claim: &CairoClaim,
    _elements: &CairoInteractionElements,
    interaction_claim: &CairoInteractionClaim,
) -> bool {
    let mut sum = QM31::zero();
    // Public memory.
    // TODO(spapini): Optimized inverse.
    // sum += claim
    //     .public_memory
    //     .iter()
    //     .map(|(addr, val)| {
    //         elements
    //             .opcode.memory_elements.addr_to_id
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
    sum += interaction_claim.mem.logup_sum();
    sum += interaction_claim.opcodes.logup_sum();
    sum == SecureField::zero()
}

pub struct CairoComponents {
    opcodes: OpcodesComponents,
    mem: MemoryComponents,
}

impl CairoComponents {
    pub fn new(
        cairo_claim: &CairoClaim,
        interaction_elements: &CairoInteractionElements,
        interaction_claim: &CairoInteractionClaim,
    ) -> Self {
        let opcodes = OpcodesComponents::new(
            &cairo_claim.opcodes,
            &interaction_elements.opcode,
            &interaction_claim.opcodes,
        );
        let mem_and_range_els = MemoryAndRangeElements {
            mem: interaction_elements.opcode.mem.clone(),
            range: interaction_elements.opcode.range.clone(),
        };
        let mem =
            MemoryComponents::new(&cairo_claim.mem, &mem_and_range_els, &interaction_claim.mem);
        Self { opcodes, mem }
    }

    pub fn provers(&self) -> impl Iterator<Item = &dyn ComponentProver<SimdBackend>> {
        chain![self.opcodes.provers(), self.mem.provers()]
    }

    pub fn components(&self) -> impl Iterator<Item = &dyn Component> {
        chain![self.opcodes.components(), self.mem.components()]
    }
}

const LOG_MAX_ROWS: u32 = 20;
pub fn prove_cairo(config: PcsConfig, input: CairoInput) -> CairoProof<Blake2sMerkleHasher> {
    let _span = span!(Level::INFO, "Proof").entered();
    let span = span!(Level::INFO, "Twiddles").entered();
    let twiddles = SimdBackend::precompute_twiddles(
        CanonicCoset::new(LOG_MAX_ROWS + config.fri_config.log_blowup_factor + 2)
            .circle_domain()
            .half_coset,
    );
    span.exit();

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
    let initial_state = input.instructions.initial_state;
    let final_state = input.instructions.final_state;

    // TODO: Table interaction.

    // Base trace.
    let span = span!(Level::INFO, "Trace gen").entered();
    let opcode_provers = OpcodesProvers::new(input.instructions);
    let mut tree_builder = commitment_scheme.tree_builder();

    // // Add public memory.
    // for addr in &input.public_mem_addresses {
    //     opcode_ctx.addr_to_id.add_inputs(*addr);
    // }
    let range_provers = CpuRangeProvers {
        range2: &mut RangeProver,
        range3: &mut RangeProver,
    };
    let mut mem = MemoryProverBuilder::new(&input.mem);
    let (opcodes_claim, opcodes_provers) = opcode_provers.generate(
        OpcodeGenContext {
            mem_builder: &mut mem,
            mem: &input.mem,
            range: range_provers,
        },
        &mut tree_builder,
    );
    let (memory_claim, mem) = mem.generate(input.mem, &mut tree_builder);

    // Commit to the claim and the trace.
    let claim = CairoClaim {
        public_memory,
        initial_state,
        final_state,
        range_check: input.range_check,
        opcodes: opcodes_claim,
        mem: memory_claim,
    };
    claim.mix_into(channel);
    span.exit();
    tree_builder.commit(channel);

    // Draw interaction elements.
    let interaction_elements = CairoInteractionElements::draw(channel);
    let memory_and_range_elements = MemoryAndRangeElements {
        mem: interaction_elements.opcode.mem.clone(),
        range: interaction_elements.opcode.range.clone(),
    };

    // Interaction trace.
    let span = span!(Level::INFO, "Interaction trace gen").entered();
    let mut tree_builder = commitment_scheme.tree_builder();
    let opcodes_interaction_claim =
        opcodes_provers.generate(&interaction_elements.opcode, &mut tree_builder);
    let mem_interaction_claim = mem.generate(&mut tree_builder, &memory_and_range_elements);

    // Commit to the interaction claim and the interaction trace.
    let interaction_claim = CairoInteractionClaim {
        opcodes: opcodes_interaction_claim,
        mem: mem_interaction_claim,
    };
    debug_assert!(
        lookup_sum_valid(&claim, &interaction_elements, &interaction_claim),
        "Lookups are invalid"
    );
    if true {
        panic!("Passed lookup sum check");
    }
    interaction_claim.mix_into(channel);
    span.exit();
    tree_builder.commit(channel);

    // Component provers.
    let component_builder = CairoComponents::new(&claim, &interaction_elements, &interaction_claim);
    let components = component_builder.provers();

    // // TODO: Remove. Only for debugging.
    // let trace_polys = commitment_scheme
    //     .trees
    //     .as_ref()
    //     .map(|t| t.polynomials.iter().cloned().collect_vec());
    // component_builder.assert_constraints(trace_polys);

    // Prove stark.
    let proof = prove::<SimdBackend, _>(
        &components.collect_vec(),
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
        CairoComponents::new(&claim, &interaction_elements, &interaction_claim);
    let components = component_generator.components();

    verify(
        &components.collect_vec(),
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

    #[test_log::test]
    fn test_cairo_air() {
        let config = PcsConfig::default();
        let cairo_proof = prove_cairo(config, test_input());

        verify_cairo(config, cairo_proof).unwrap();
    }
}
