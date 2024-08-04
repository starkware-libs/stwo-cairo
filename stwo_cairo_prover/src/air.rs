use std::iter::zip;

use itertools::{chain, Itertools};
use stwo_prover::core::air::{Component, ComponentProver};
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::channel::{Blake2sChannel, Channel};
use stwo_prover::core::fields::qm31::QM31;
use stwo_prover::core::pcs::{CommitmentSchemeProver, CommitmentSchemeVerifier};
use stwo_prover::core::poly::circle::{CanonicCoset, PolyOps};
use stwo_prover::core::prover::{prove, verify, StarkProof, LOG_BLOWUP_FACTOR};
use stwo_prover::core::vcs::blake2_hash::Blake2sHasher;
use stwo_prover::core::vcs::hasher::Hasher;

use crate::opcode::OpcodeInteractionElements;
use crate::rc::RcElements;
use crate::ret::{
    RetInput, RetOpcodeProver, RetOpcodeStatement0, RetOpcodeStatement1,
    RetOpcodeStatementComponent,
};

// TODO: Channel::mix_felts() should accept M31, not QM31.

pub struct CairoProof {
    stmt0: CairoStatement0,
    stmt1: CairoStatement1,
    stark_proof: StarkProof,
}

pub struct CairoStatement0 {
    ret: Vec<RetOpcodeStatement0>,
    // ...
}
impl CairoStatement0 {
    // TODO: mix_felts should accept M31, not QM31.
    fn serialize(&self) -> Vec<QM31> {
        todo! {}
    }
    fn log_sizes(&self) -> Vec<u32> {
        todo!()
    }
}

pub struct CairoStatement1 {
    ret: Vec<RetOpcodeStatement1>,
    // ...
}
impl CairoStatement1 {
    fn serialize(&self) -> Vec<QM31> {
        todo! {}
    }
    fn log_sizes(&self) -> Vec<u32> {
        todo!()
    }
}

// Externally provided inputs.
pub struct CairoInput {
    // Opcudes.
    ret: Vec<RetInput>,
    // Builtins.

    // Memory.

    // Tables. (rc, xor, ...)
}

pub struct OpcodeGenContext {
    // Table accumulators.
    // Memory.
}
impl OpcodeGenContext {
    fn new() -> Self {
        Self {}
    }
}

fn prove_cairo(input: CairoInput) -> CairoProof {
    // Setup protocol.
    let log_max_rows = 20;
    let twiddles = SimdBackend::precompute_twiddles(
        CanonicCoset::new(log_max_rows + 1 + LOG_BLOWUP_FACTOR)
            .circle_domain()
            .half_coset,
    );
    let commitment_scheme = &mut CommitmentSchemeProver::new(LOG_BLOWUP_FACTOR, &twiddles);
    let channel = &mut Blake2sChannel::new(Blake2sHasher::hash(&[]));

    // Prepare generation.
    let mut tree_builder = commitment_scheme.tree_builder();
    let mut ctx = OpcodeGenContext::new();

    // Generate opcode traces.
    let ret_provers = RetOpcodeProver::new(input.ret)
        .into_iter()
        .map(|r| r.write_trace(&mut tree_builder, &mut ctx))
        .collect_vec();

    // Generate table traces by consuming ctx.

    // Commit on statement and trace.
    let ret_stmts0 = ret_provers.iter().map(|r| r.stmt().clone()).collect();
    let stmt0 = CairoStatement0 { ret: ret_stmts0 };
    channel.mix_felts(&stmt0.serialize());
    tree_builder.commit(channel);

    // Draw interaction elements.
    let rc_els = RcElements::draw(channel);
    let opcode_elements = OpcodeInteractionElements { rc_els };
    let mut tree_builder = commitment_scheme.tree_builder();

    // Generate opcode interaction traces.
    let ret_provers = ret_provers
        .into_iter()
        .map(|r| r.write_interaction_trace(&mut tree_builder, &opcode_elements))
        .collect_vec();

    // Generate table interaction traces.

    // Commit on statement and trace.
    let ret_stmts = ret_provers.iter().map(|r| r.stmt().clone()).collect();
    let stmt1 = CairoStatement1 { ret: ret_stmts };
    channel.mix_felts(&stmt0.serialize());
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
    let stark_proof = prove(&components, channel, commitment_scheme).unwrap();

    CairoProof {
        stmt0,
        stmt1,
        stark_proof,
    }
}

pub fn verify_cairo(proof: CairoProof) {
    // Setup protocol.
    let commitment_scheme = &mut CommitmentSchemeVerifier::new();
    let channel = &mut Blake2sChannel::new(Blake2sHasher::hash(&[]));

    // Add statement to the channel
    channel.mix_felts(&proof.stmt0.serialize());
    // TODO: Move commitment outside StarkProof.
    commitment_scheme.commit(
        proof.stark_proof.commitments[0],
        &proof.stmt0.log_sizes(),
        channel,
    );

    // Draw interaction elements.
    let rc_els = RcElements::draw(channel);
    let opcode_elements = OpcodeInteractionElements { rc_els };

    // Add statement to the channel
    channel.mix_felts(&proof.stmt1.serialize());
    commitment_scheme.commit(
        proof.stark_proof.commitments[1],
        &proof.stmt1.log_sizes(),
        channel,
    );

    // Verify stark.
    let ret_components = zip(proof.stmt0.ret, proof.stmt1.ret)
        .map(|(stmt0, stmt1)| {
            RetOpcodeStatementComponent::new(stmt0, stmt1, opcode_elements.clone())
        })
        .collect_vec();

    let components: Vec<&dyn Component> =
        chain![ret_components.iter().map(|c| c as &dyn Component)].collect_vec();
    verify(&components, channel, commitment_scheme, proof.stark_proof).unwrap();
}
