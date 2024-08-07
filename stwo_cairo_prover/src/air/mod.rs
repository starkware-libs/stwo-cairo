mod opcode;
mod ret;

use std::iter::zip;

use itertools::{chain, Itertools};
use opcode::OpcodeElements;
use ret::{
    RetInput, RetOpcodeClaim, RetOpcodeComponent, RetOpcodeInteractionClaim, RetOpcodeProver,
};
use stwo_prover::core::air::{Component, ComponentProver};
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::channel::{Blake2sChannel, Channel as _};
use stwo_prover::core::pcs::{CommitmentSchemeProver, CommitmentSchemeVerifier, TreeVec};
use stwo_prover::core::poly::circle::{CanonicCoset, PolyOps};
use stwo_prover::core::prover::{prove, verify, StarkProof, LOG_BLOWUP_FACTOR};
use stwo_prover::core::vcs::blake2_hash::Blake2sHasher;
use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleHasher;
use stwo_prover::core::InteractionElements;

use crate::components::range_check_unit::RangeElements;

type Channel = Blake2sChannel;
type MerkleHasher = Blake2sMerkleHasher;
// TODO: Channel::mix_felts() should accept M31, not QM31.

pub struct CairoProof {
    claim: CairoClaim,
    interaction_claim: CairoInteractionClaim,
    stark_proof: StarkProof<MerkleHasher>,
}

pub struct CairoClaim {
    ret: Vec<RetOpcodeClaim>,
    // ...
}
impl CairoClaim {
    // TODO: mix_felts should accept M31, not QM31.
    fn mix_into(&self, _channel: &mut Channel) {
        todo! {}
    }
    fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        todo!()
    }
}

pub struct CairoInteractionClaim {
    ret: Vec<RetOpcodeInteractionClaim>,
    // ...
}
impl CairoInteractionClaim {
    fn mix_into(&self, _channel: &mut Channel) {
        todo! {}
    }
}

// Externally provided inputs.
pub struct CairoInput {
    // Opcodes.
    ret: Vec<RetInput>,
    // Builtins.

    // Memory.

    // Tables. (rc, xor, ...)
}

struct OpcodeGenContext {
    // Table accumulators.
    // Memory.
}
impl OpcodeGenContext {
    fn new() -> Self {
        Self {}
    }
}

#[allow(dead_code)]
pub fn prove_cairo(input: CairoInput) -> CairoProof {
    // Setup protocol.
    let log_max_rows = 20;
    let twiddles = SimdBackend::precompute_twiddles(
        CanonicCoset::new(log_max_rows + 1 + LOG_BLOWUP_FACTOR)
            .circle_domain()
            .half_coset,
    );
    let commitment_scheme = &mut CommitmentSchemeProver::new(LOG_BLOWUP_FACTOR, &twiddles);
    let channel = &mut Channel::new(Blake2sHasher::hash(&[]));

    // Prepare generation.
    let mut tree_builder = commitment_scheme.tree_builder();
    let mut ctx = OpcodeGenContext::new();

    // Generate opcode traces.
    let ret_provers = RetOpcodeProver::new(input.ret)
        .into_iter()
        .map(|r| r.write_trace(&mut tree_builder, &mut ctx))
        .collect_vec();

    // Generate table traces by consuming ctx.

    // Commit on claim and trace.
    let ret_claims = ret_provers.iter().map(|r| r.claim().clone()).collect();
    let claim = CairoClaim { ret: ret_claims };
    claim.mix_into(channel);
    tree_builder.commit(channel);

    // Draw interaction elements.
    let opcode_elements = OpcodeElements::draw(channel);
    let mut tree_builder = commitment_scheme.tree_builder();

    // Generate opcode interaction traces.
    let ret_provers = ret_provers
        .into_iter()
        .map(|r| r.write_interaction_trace(&mut tree_builder, &opcode_elements))
        .collect_vec();

    // Generate table interaction traces.

    // Commit on interaction claim and trace.
    let ret_claims = ret_provers.iter().map(|r| r.claim().clone()).collect();
    let interaction_claim = CairoInteractionClaim { ret: ret_claims };
    interaction_claim.mix_into(channel);
    tree_builder.commit(channel);

    // Prepare components.
    let ret_components = ret_provers
        .into_iter()
        .map(|r| r.into_component())
        .collect_vec();

    // Prove stark.
    let components: Vec<&dyn ComponentProver<SimdBackend>> = chain![ret_components
        .iter()
        .map(|c| c as &dyn ComponentProver<SimdBackend>)]
    .collect_vec();
    let stark_proof = prove(
        &components,
        channel,
        &InteractionElements::default(),
        commitment_scheme,
    )
    .unwrap();

    CairoProof {
        claim,
        interaction_claim,
        stark_proof,
    }
}

#[allow(dead_code)]
pub fn verify_cairo(
    CairoProof {
        claim,
        interaction_claim,
        stark_proof,
    }: CairoProof,
) {
    // Setup protocol.
    let commitment_scheme = &mut CommitmentSchemeVerifier::new();
    let channel = &mut Blake2sChannel::new(Blake2sHasher::hash(&[]));

    // Add claim to the channel
    claim.mix_into(channel);
    let log_sizes = claim.log_sizes();
    // TODO: Move commitment outside StarkProof.
    commitment_scheme.commit(stark_proof.commitments[0], &log_sizes[0], channel);

    // Draw interaction elements.
    let rc_els = RangeElements::draw(channel);
    let opcode_elements = OpcodeElements {
        range_elements: rc_els,
    };

    // Add interaction claim to the channel
    interaction_claim.mix_into(channel);
    commitment_scheme.commit(stark_proof.commitments[1], &log_sizes[1], channel);

    // Verify stark.
    let ret_components = zip(claim.ret, interaction_claim.ret)
        .map(|(claim, interaction_claim)| {
            RetOpcodeComponent::new(claim, interaction_claim, opcode_elements.clone())
        })
        .collect_vec();

    let components: Vec<&dyn Component> =
        chain![ret_components.iter().map(|c| c as &dyn Component)].collect_vec();
    verify(
        &components,
        channel,
        &InteractionElements::default(),
        commitment_scheme,
        stark_proof,
    )
    .unwrap();
}
