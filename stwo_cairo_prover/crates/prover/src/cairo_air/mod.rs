use itertools::{chain, Itertools};
use num_traits::Zero;
use serde::{Deserialize, Serialize};
use stwo_prover::constraint_framework::TraceLocationAllocator;
use stwo_prover::core::air::{Component, ComponentProver};
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::channel::{Blake2sChannel, Channel};
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::fields::qm31::{SecureField, QM31};
use stwo_prover::core::fields::FieldExpOps;
use stwo_prover::core::pcs::{
    CommitmentSchemeProver, CommitmentSchemeVerifier, PcsConfig, TreeVec,
};
use stwo_prover::core::poly::circle::{CanonicCoset, PolyOps};
use stwo_prover::core::prover::{prove, verify, ProvingError, StarkProof, VerificationError};
use stwo_prover::core::vcs::blake2_merkle::{Blake2sMerkleChannel, Blake2sMerkleHasher};
use stwo_prover::core::vcs::ops::MerkleHasher;
use thiserror::Error;

use crate::components::memory::component::{
    MemoryClaim, MemoryComponent, MemoryEval, MemoryInteractionClaim,
};
use crate::components::memory::prover::MemoryClaimProver;
use crate::components::memory::MemoryLookupElements;
use crate::components::range_check_builtin::component::{
    RangeCheckBuiltinClaim, RangeCheckBuiltinComponent, RangeCheckBuiltinEval,
    RangeCheckBuiltinInteractionClaim,
};
use crate::components::range_check_builtin::prover::RangeCheckBuiltinClaimProver;
use crate::components::range_check_unit::component::{
    RangeCheckClaim, RangeCheckInteractionClaim, RangeCheckUnitComponent, RangeCheckUnitEval,
};
use crate::components::range_check_unit::component_prover::RangeCheckClaimProver;
use crate::components::range_check_unit::RangeCheckElements;
use crate::components::ret_opcode::component::{
    RetOpcodeClaim, RetOpcodeComponent, RetOpcodeEval, RetOpcodeInteractionClaim,
};
use crate::components::ret_opcode::prover::RetOpcodeClaimProver;
use crate::felt::split_f252;
use crate::input::instructions::VmState;
use crate::input::CairoInput;

const RC9_LOG_MAX: u32 = 9;
const RC9_LOG_REPS: u32 = 1;
const RC9_LOG_HEIGHT: u32 = RC9_LOG_MAX - RC9_LOG_REPS;
const RC9_REPS: usize = 1 << RC9_LOG_REPS;

#[derive(Serialize, Deserialize)]
pub struct CairoProof<H: MerkleHasher> {
    pub claim: CairoClaim,
    pub interaction_claim: CairoInteractionClaim,
    pub stark_proof: StarkProof<H>,
}

#[derive(Serialize, Deserialize)]
pub struct CairoClaim {
    // Common claim values.
    pub public_memory: Vec<(u32, [u32; 8])>,
    pub initial_state: VmState,
    pub final_state: VmState,

    pub ret: Vec<RetOpcodeClaim>,
    pub range_check_builtin: RangeCheckBuiltinClaim,
    pub memory: MemoryClaim,
    pub range_check9: RangeCheckClaim<RC9_REPS>,
    // ...
}

impl CairoClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        // TODO(spapini): Add common values.
        self.ret.iter().for_each(|c| c.mix_into(channel));
        self.range_check_builtin.mix_into(channel);
        self.memory.mix_into(channel);
    }

    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        TreeVec::concat_cols(chain!(
            self.ret.iter().map(|c| c.log_sizes()),
            [self.range_check_builtin.log_sizes()],
            [self.memory.log_sizes()],
            [self.range_check9.log_sizes()]
        ))
    }
}

pub struct CairoInteractionElements {
    memory_lookup: MemoryLookupElements,
    range9_lookup: RangeCheckElements,
    // ...
}
impl CairoInteractionElements {
    pub fn draw(channel: &mut impl Channel) -> CairoInteractionElements {
        CairoInteractionElements {
            memory_lookup: MemoryLookupElements::draw(channel),
            range9_lookup: RangeCheckElements::draw(channel),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CairoInteractionClaim {
    pub ret: Vec<RetOpcodeInteractionClaim>,
    pub range_check_builtin: RangeCheckBuiltinInteractionClaim,
    pub memory: MemoryInteractionClaim,
    pub range_check9: RangeCheckInteractionClaim<RC9_REPS>,
    // ...
}

impl CairoInteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        self.ret.iter().for_each(|c| c.mix_into(channel));
        self.range_check_builtin.mix_into(channel);
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
    sum += interaction_claim.range_check9.claimed_sum;
    sum += interaction_claim.ret[0].claimed_sum;
    sum += interaction_claim.range_check_builtin.claimed_sum;
    sum += interaction_claim.memory.claimed_sum;
    sum == SecureField::zero()
}

pub struct CairoComponents {
    ret: Vec<RetOpcodeComponent>,
    range_check_builtin: RangeCheckBuiltinComponent,
    memory: MemoryComponent,
    range_check9: RangeCheckUnitComponent<RC9_REPS>,
    // ...
}

impl CairoComponents {
    pub fn new(
        cairo_claim: &CairoClaim,
        interaction_elements: &CairoInteractionElements,
        interaction_claim: &CairoInteractionClaim,
    ) -> Self {
        let tree_span_provider = &mut TraceLocationAllocator::default();

        let ret_components = cairo_claim
            .ret
            .iter()
            .zip(interaction_claim.ret.iter())
            .map(|(claim, interaction_claim)| {
                RetOpcodeComponent::new(
                    tree_span_provider,
                    RetOpcodeEval::new(
                        claim.clone(),
                        interaction_elements.memory_lookup.clone(),
                        interaction_claim.clone(),
                    ),
                )
            })
            .collect_vec();
        let range_check_builtin_component = RangeCheckBuiltinComponent::new(
            tree_span_provider,
            RangeCheckBuiltinEval::new(
                cairo_claim.range_check_builtin.clone(),
                interaction_elements.memory_lookup.clone(),
                interaction_claim.range_check_builtin.clone(),
            ),
        );
        let memory_component = MemoryComponent::new(
            tree_span_provider,
            MemoryEval::new(
                cairo_claim.memory.clone(),
                interaction_elements.memory_lookup.clone(),
                interaction_elements.range9_lookup.clone(),
                interaction_claim.memory.clone(),
            ),
        );
        let range_check9_component = RangeCheckUnitComponent::new(
            tree_span_provider,
            RangeCheckUnitEval {
                log_n_rows: RC9_LOG_HEIGHT,
                lookup_elements: interaction_elements.range9_lookup.clone(),
                claimed_sum: interaction_claim.range_check9.claimed_sum,
            },
        );
        Self {
            ret: ret_components,
            range_check_builtin: range_check_builtin_component,
            memory: memory_component,
            range_check9: range_check9_component,
        }
    }

    pub fn provers(&self) -> Vec<&dyn ComponentProver<SimdBackend>> {
        let mut vec: Vec<&dyn ComponentProver<SimdBackend>> = vec![];
        for ret in self.ret.iter() {
            vec.push(ret);
        }
        vec.push(&self.range_check_builtin);
        vec.push(&self.memory);
        vec.push(&self.range_check9);
        vec
    }

    pub fn components(&self) -> Vec<&dyn Component> {
        let mut vec: Vec<&dyn Component> = vec![];
        for ret in self.ret.iter() {
            vec.push(ret);
        }
        vec.push(&self.range_check_builtin);
        vec.push(&self.memory);
        vec.push(&self.range_check9);
        vec
    }
}

const LOG_MAX_ROWS: u32 = 20;
pub fn prove_cairo(input: CairoInput) -> Result<CairoProof<Blake2sMerkleHasher>, ProvingError> {
    let config = PcsConfig::default();
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
    // TODO(Ohad): change to OpcodeClaimProvers, and integrate padding.
    let ret_trace_generator = RetOpcodeClaimProver::new(input.instructions.ret);
    let range_check_builtin_trace_generator =
        RangeCheckBuiltinClaimProver::new(input.range_check_builtin);
    let mut memory_trace_generator = MemoryClaimProver::new(input.mem);
    let mut range_check9_trace_generator = RangeCheckClaimProver::<RC9_LOG_HEIGHT, RC9_REPS> {
        multiplicities: input
            .range_check9
            .to_2d_simd_vec::<RC9_LOG_HEIGHT, RC9_REPS>(),
    };

    // Add public memory.
    for addr in &input.public_mem_addresses {
        memory_trace_generator.add_inputs(M31::from_u32_unchecked(*addr));
    }

    let mut tree_builder = commitment_scheme.tree_builder();

    let (ret_claim, ret_interaction_prover) =
        ret_trace_generator.write_trace(&mut tree_builder, &mut memory_trace_generator);
    let (range_check_builtin_claim, range_check_builtin_interaction_prover) =
        range_check_builtin_trace_generator
            .write_trace(&mut tree_builder, &mut memory_trace_generator);
    let (memory_claim, memory_interaction_prover) =
        memory_trace_generator.write_trace(&mut tree_builder);
    let (range_check9_claim, range_check9_interaction_prover) =
        range_check9_trace_generator.write_trace(&mut tree_builder);

    // Commit to the claim and the trace.
    let claim = CairoClaim {
        public_memory,
        initial_state: input.instructions.initial_state,
        final_state: input.instructions.final_state,
        ret: vec![ret_claim],
        range_check_builtin: range_check_builtin_claim.clone(),
        memory: memory_claim.clone(),
        range_check9: range_check9_claim.clone(),
    };
    claim.mix_into(channel);
    tree_builder.commit(channel);

    // Draw interaction elements.
    let interaction_elements = CairoInteractionElements::draw(channel);

    // Interaction trace.
    let mut tree_builder = commitment_scheme.tree_builder();
    let ret_interaction_claim = ret_interaction_prover
        .write_interaction_trace(&mut tree_builder, &interaction_elements.memory_lookup);
    let range_check_builtin_interaction_claim = range_check_builtin_interaction_prover
        .write_interaction_trace(&mut tree_builder, &interaction_elements.memory_lookup);
    let memory_interaction_claim = memory_interaction_prover.write_interaction_trace(
        &mut tree_builder,
        &interaction_elements.memory_lookup,
        &interaction_elements.range9_lookup,
    );

    let range_check9_interaction_claim = range_check9_interaction_prover
        .write_interaction_trace(&mut tree_builder, &interaction_elements.range9_lookup);

    // Commit to the interaction claim and the interaction trace.
    let interaction_claim = CairoInteractionClaim {
        ret: vec![ret_interaction_claim.clone()],
        range_check_builtin: range_check_builtin_interaction_claim.clone(),
        memory: memory_interaction_claim.clone(),
        range_check9: range_check9_interaction_claim.clone(),
    };
    debug_assert!(lookup_sum_valid(
        &claim,
        &interaction_elements,
        &interaction_claim
    ));
    interaction_claim.mix_into(channel);
    tree_builder.commit(channel);

    // Component provers.
    let component_builder = CairoComponents::new(&claim, &interaction_elements, &interaction_claim);
    let components = component_builder.provers();

    // Prove stark.
    let proof = prove::<SimdBackend, _>(&components, channel, commitment_scheme)?;

    Ok(CairoProof {
        claim,
        interaction_claim,
        stark_proof: proof,
    })
}

pub fn verify_cairo(
    CairoProof {
        claim,
        interaction_claim,
        stark_proof,
    }: CairoProof<Blake2sMerkleHasher>,
) -> Result<(), CairoVerificationError> {
    // Verify.
    let config = PcsConfig::default();
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
        &components,
        channel,
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
    use crate::input::vm_import::tests::small_cairo_input;

    fn test_input() -> CairoInput {
        let u128_max = u128::MAX;
        let instructions = casm! {
            // TODO(AlonH): Add actual range check segment.
            // Manually writing range check builtin segment of size 40 to memory.
            [ap] = u128_max, ap++;
            [ap + 38] = 1, ap++;
            ap += 38;

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
    fn test_basic_cairo_air() {
        let cairo_proof = prove_cairo(test_input()).unwrap();
        verify_cairo(cairo_proof).unwrap();
    }

    #[test]
    fn test_full_cairo_air() {
        let cairo_proof = prove_cairo(small_cairo_input()).unwrap();
        verify_cairo(cairo_proof).unwrap();
    }
}
