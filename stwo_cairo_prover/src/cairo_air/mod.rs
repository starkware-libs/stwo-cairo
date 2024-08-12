use std::cmp::max;

use itertools::{chain, Itertools};
use num_traits::Zero;
use stwo_prover::core::air::{Component, ComponentProver};
use stwo_prover::core::backend::simd::m31::{PackedM31, LOG_N_LANES};
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::channel::{Blake2sChannel, Channel};
use stwo_prover::core::fields::m31::BaseField;
use stwo_prover::core::fields::qm31::SecureField;
use stwo_prover::core::fields::IntoSlice;
use stwo_prover::core::pcs::{CommitmentSchemeProver, CommitmentSchemeVerifier, TreeVec};
use stwo_prover::core::poly::circle::{CanonicCoset, PolyOps};
use stwo_prover::core::prover::{prove, verify, StarkProof, LOG_BLOWUP_FACTOR};
use stwo_prover::core::vcs::blake2_hash::Blake2sHasher;
use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleHasher;
use stwo_prover::core::vcs::ops::MerkleHasher;
use stwo_prover::core::InteractionElements;

use crate::components::memory::component::{
    MemoryClaim, MemoryComponent, MemoryInteractionClaim, LOG_MEMORY_ADDRESS_BOUND,
};
use crate::components::memory::prover::MemoryClaimProver;
use crate::components::memory::{MemoryLookupElements, MemoryValues};
use crate::components::ret_opcode::component::{
    RetOpcodeClaim, RetOpcodeComponent, RetOpcodeInteractionClaim,
};
use crate::components::ret_opcode::prover::RetOpcodeClaimProver;

pub struct CairoInput {
    // Opcodes.
    pub ret: Vec<[PackedM31; 3]>,
    // Builtins.

    // Memory.
    pub memory: MemoryValues,
    // Tables..?
}

pub struct CairoProof<H: MerkleHasher> {
    pub claim: CairoClaim,
    pub interaction_claim: CairoInteractionClaim,
    pub stark_proof: StarkProof<H>,
}

pub struct CairoClaim {
    pub ret: Vec<RetOpcodeClaim>,
    pub memory: MemoryClaim,
    // ...
}
impl CairoClaim {
    pub fn mix_into(&self, _channel: &mut impl Channel) {
        self.ret.iter().for_each(|c| c.mix_into(_channel));
    }
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        // Interaction 0.
        let ret_log_sizes = self
            .ret
            .iter()
            .flat_map(|c| c.log_sizes()[0].clone())
            .collect_vec();
        let memory_log_size = self.memory.log_sizes()[0].clone();
        let interaction_0_log_sizes = chain(memory_log_size, ret_log_sizes).collect();

        // Interaction 1.
        let ret_log_sizes = self
            .ret
            .iter()
            .flat_map(|c| c.log_sizes()[1].clone())
            .collect_vec();
        let memory_log_size = self.memory.log_sizes()[1].clone();
        let interaction_1_log_sizes = chain(memory_log_size, ret_log_sizes).collect();

        TreeVec::new(vec![interaction_0_log_sizes, interaction_1_log_sizes])
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

pub struct CairoComponentGenerator {
    ret: Vec<RetOpcodeComponent>,
    memory: MemoryComponent, //..
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

pub fn prove_cairo(input: CairoInput) -> CairoProof<Blake2sMerkleHasher> {
    // log_n_rows will not be known at this point, TODO(Ohad): figure out log_max_rows.
    let ret_log_n_rows = input.ret.len().ilog2() + LOG_N_LANES;
    let log_max_rows = max(LOG_MEMORY_ADDRESS_BOUND, ret_log_n_rows);

    let twiddles = SimdBackend::precompute_twiddles(
        CanonicCoset::new(log_max_rows + LOG_BLOWUP_FACTOR + 2)
            .circle_domain()
            .half_coset,
    );

    // Setup protocol.
    let channel = &mut Blake2sChannel::new(Blake2sHasher::hash(&[]));
    let commitment_scheme = &mut CommitmentSchemeProver::new(LOG_BLOWUP_FACTOR, &twiddles);

    // TODO: Table interaction.

    // Base trace.
    // TODO(Ohad): change to OpcodeClaimProvers, and integrate padding.
    let ret_trace_generator = RetOpcodeClaimProver { inputs: input.ret };
    let mut memory_trace_generator = MemoryClaimProver::from_info(input.memory);
    let mut tree_builder = commitment_scheme.tree_builder();
    let (ret_claim, ret_interaction_prover) =
        ret_trace_generator.write_trace(&mut tree_builder, &mut memory_trace_generator);
    let (memory_claim, memory_interaction_prover) =
        memory_trace_generator.write_trace(&mut tree_builder);

    let cairo_claim = CairoClaim {
        ret: vec![ret_claim],
        memory: memory_claim.clone(),
    };
    cairo_claim.mix_into(channel);
    tree_builder.commit(channel);

    // Draw interaction elements.
    let interaction_elements = CairoInteractionElements::draw(channel);

    // Interaction trace.
    let mut tree_builder = commitment_scheme.tree_builder();
    let ret_interaction_claim = ret_interaction_prover
        .write_interaction_trace(&mut tree_builder, &interaction_elements.memory_lookup);
    let memory_interaction_claim = memory_interaction_prover
        .write_interaction_trace(&mut tree_builder, &interaction_elements.memory_lookup);

    debug_assert_eq!(
        memory_interaction_claim.claimed_sum + ret_interaction_claim.memory_claimed_sum,
        SecureField::zero()
    );
    let cairo_interaction_claim = CairoInteractionClaim {
        ret: vec![ret_interaction_claim.clone()],
        memory: memory_interaction_claim.clone(),
    };
    cairo_interaction_claim.mix_into(channel);
    tree_builder.commit(channel);

    // Component provers.
    let component_builder = CairoComponentGenerator::new(
        &cairo_claim,
        &interaction_elements,
        &cairo_interaction_claim,
    );
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
        claim: cairo_claim,
        interaction_claim: cairo_interaction_claim,
        stark_proof: proof,
    }
}

pub fn verify_cairo(
    CairoProof {
        claim: cairo_claim,
        interaction_claim,
        stark_proof,
    }: CairoProof<Blake2sMerkleHasher>,
) {
    // Verify.
    let channel = &mut Blake2sChannel::new(Blake2sHasher::hash(BaseField::into_slice(&[])));
    let commitment_scheme_verifier = &mut CommitmentSchemeVerifier::<Blake2sMerkleHasher>::new();

    cairo_claim.mix_into(channel);
    commitment_scheme_verifier.commit(
        stark_proof.commitments[0],
        &cairo_claim.log_sizes()[0],
        channel,
    );
    let lookup_elements = CairoInteractionElements::draw(channel);
    assert_eq!(
        interaction_claim.memory.claimed_sum + interaction_claim.ret[0].memory_claimed_sum,
        SecureField::zero()
    );
    interaction_claim.mix_into(channel);
    commitment_scheme_verifier.commit(
        stark_proof.commitments[1],
        &cairo_claim.log_sizes()[1],
        channel,
    );

    let component_generator =
        CairoComponentGenerator::new(&cairo_claim, &lookup_elements, &interaction_claim);
    let components = component_generator.components();

    verify(
        &components,
        channel,
        &InteractionElements::default(),
        commitment_scheme_verifier,
        stark_proof,
    )
    .unwrap();
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use num_traits::{One, Zero};
    use stwo_prover::core::backend::simd::m31::PackedM31;
    use stwo_prover::core::fields::m31::M31;

    use crate::cairo_air::{prove_cairo, verify_cairo, CairoInput};
    use crate::components::memory::component::N_M31_IN_FELT252;
    use crate::components::memory::MemoryValues;

    pub fn test_simd_ret_inputs() -> Vec<[PackedM31; 3]> {
        (0..8)
            .map(|i| {
                [
                    PackedM31::broadcast(M31::from_u32_unchecked(i)),
                    PackedM31::broadcast(M31::from_u32_unchecked(2)),
                    PackedM31::broadcast(M31::from_u32_unchecked(2)),
                ]
            })
            .collect_vec()
    }

    pub fn generate_test_simd_ret_memory() -> MemoryValues {
        let mut value = [PackedM31::zero(); N_M31_IN_FELT252];
        value[0] = PackedM31::one();
        MemoryValues {
            values: vec![value; 8],
        }
    }

    #[test]
    fn test_cairo_air() {
        let ret_inputs = test_simd_ret_inputs();
        let memory_info = generate_test_simd_ret_memory();
        let cairo_input = CairoInput {
            ret: ret_inputs,
            memory: memory_info,
        };

        let cairo_proof = prove_cairo(cairo_input);

        verify_cairo(cairo_proof)
    }
}
