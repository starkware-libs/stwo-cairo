use std::cmp::max;

use itertools::{chain, Itertools};
use stwo_prover::constraint_framework::logup::LookupElements;
use stwo_prover::core::air::{Component, ComponentProver};
use stwo_prover::core::backend::simd::m31::{PackedM31, LOG_N_LANES};
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::channel::{Blake2sChannel, Channel};
use stwo_prover::core::fields::m31::BaseField;
use stwo_prover::core::fields::IntoSlice;
use stwo_prover::core::pcs::{CommitmentSchemeProver, CommitmentSchemeVerifier};
use stwo_prover::core::poly::circle::{CanonicCoset, PolyOps};
use stwo_prover::core::prover::{prove, verify, StarkProof, LOG_BLOWUP_FACTOR};
use stwo_prover::core::vcs::blake2_hash::Blake2sHasher;
use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleHasher;
use stwo_prover::core::vcs::ops::MerkleHasher;
use stwo_prover::core::InteractionElements;

use crate::components::memory::component::{
    MemoryClaim, MemoryComponent, MemoryInteractionClaim, N_MEMORY_COLUMNS,
};
use crate::components::memory::simd_trace::{
    MemorySimdTraceGenerator, LOG_SIMD_MEMORY_ADDRESS_BOUND,
};
use crate::components::memory::{MemoryLookupElements, MemoryValues};
use crate::components::ret_opcode::component::{
    RetOpcodeClaim, RetOpcodeComponent, RetOpcodeInteractionClaim,
};
use crate::components::ret_opcode::simd_trace::RetOpcodeSimdTraceGenerator;

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
    pub fn log_sizes(&self) -> Vec<u32> {
        let ret_log_sizes = self.ret.iter().flat_map(|c| c.log_sizes()).collect_vec();
        let memory_log_size = vec![self.memory.log_address_bound; N_MEMORY_COLUMNS];
        chain(memory_log_size, ret_log_sizes).collect()
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
    pub fn log_sizes(&self) -> Vec<u32> {
        let ret_log_sizes = self.ret.iter().flat_map(|c| c.log_sizes()).collect_vec();
        let memory_log_size = vec![LOG_SIMD_MEMORY_ADDRESS_BOUND; 4];
        chain(memory_log_size, ret_log_sizes).collect()
    }
}

pub fn prove_cairo(input: CairoInput) -> CairoProof<Blake2sMerkleHasher> {
    // log_n_rows will not be known at this point, TODO(Ohad): figure out log_max_rows.
    let ret_log_n_rows = input.ret.len().ilog2() + LOG_N_LANES;
    let log_max_rows = max(LOG_SIMD_MEMORY_ADDRESS_BOUND, ret_log_n_rows);

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
    let ret_trace_generator = RetOpcodeSimdTraceGenerator { inputs: input.ret };
    let mut memory_trace_generator = MemorySimdTraceGenerator::from_info(input.memory);
    let mut tree_builder = commitment_scheme.tree_builder();
    let (ret_claim, ret_interaction_prover) =
        ret_trace_generator.write_trace(&mut tree_builder, &mut memory_trace_generator);
    let (memory_claim, memory_interaction_prover) =
        memory_trace_generator.write_trace(&mut tree_builder);

    let cairo_claim = CairoClaim {
        ret: vec![ret_claim],
        memory: memory_claim,
    };
    cairo_claim.mix_into(channel);
    tree_builder.commit(channel);

    // Draw interaction elements.
    let memory_lookup_elements = LookupElements::draw(channel);

    // Interaction trace.
    let mut tree_builder = commitment_scheme.tree_builder();
    let ret_interaction_claim =
        ret_interaction_prover.write_interaction_trace(&mut tree_builder, &memory_lookup_elements);
    let memory_interaction_claim = memory_interaction_prover
        .write_interaction_trace(&mut tree_builder, &memory_lookup_elements);
    debug_assert_eq!(
        memory_interaction_claim.claimed_sum,
        ret_interaction_claim.memory_claimed_sum
    );
    let cairo_interaction_claim = CairoInteractionClaim {
        ret: vec![ret_interaction_claim],
        memory: memory_interaction_claim,
    };
    cairo_interaction_claim.mix_into(channel);
    tree_builder.commit(channel);

    // Components
    let ret_component = RetOpcodeComponent {
        log_size: cairo_claim.ret[0].log_size,
        claimed_sum: cairo_interaction_claim.ret[0].memory_claimed_sum,
        lookup_elements: memory_lookup_elements.clone(),
    };
    let memory_component = MemoryComponent {
        log_n_rows: cairo_claim.memory.log_address_bound,
        claimed_sum: cairo_interaction_claim.memory.claimed_sum,
        lookup_elements: memory_lookup_elements.clone(),
    };
    let components: Vec<&dyn ComponentProver<SimdBackend>> =
        vec![&ret_component, &memory_component];

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
        &cairo_claim.log_sizes(),
        channel,
    );
    let lookup_elements = MemoryLookupElements::draw(channel);
    let ret_component = RetOpcodeComponent {
        log_size: cairo_claim.ret[0].log_size,
        claimed_sum: interaction_claim.ret[0].memory_claimed_sum,
        lookup_elements: lookup_elements.clone(),
    };
    let memory_component = MemoryComponent {
        log_n_rows: cairo_claim.memory.log_address_bound,
        claimed_sum: interaction_claim.memory.claimed_sum,
        lookup_elements,
    };
    let components: Vec<&dyn Component> = vec![&ret_component, &memory_component];

    interaction_claim.mix_into(channel);
    commitment_scheme_verifier.commit(
        stark_proof.commitments[1],
        &interaction_claim.log_sizes(),
        channel,
    );

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
