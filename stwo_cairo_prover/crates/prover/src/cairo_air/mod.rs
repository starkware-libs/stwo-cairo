pub mod air;
use air::{
    lookup_sum, CairoClaim, CairoComponents, CairoInteractionClaim, CairoInteractionElements,
    CairoProof,
};
use itertools::Itertools;
use num_traits::Zero;
use stwo_prover::constraint_framework::preprocessed_columns::gen_is_first;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::channel::Blake2sChannel;
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::fields::qm31::SecureField;
use stwo_prover::core::pcs::{CommitmentSchemeProver, CommitmentSchemeVerifier, PcsConfig};
use stwo_prover::core::poly::circle::{CanonicCoset, PolyOps};
use stwo_prover::core::prover::{prove, verify, ProvingError, VerificationError};
use stwo_prover::core::vcs::blake2_merkle::{Blake2sMerkleChannel, Blake2sMerkleHasher};
use thiserror::Error;
use tracing::{span, Level};

use crate::components::memory::{addr_to_id, id_to_f252};
use crate::components::range_check_vector::{range_check_4_3, range_check_7_2_5, range_check_9_9};
use crate::components::{ret_opcode, verifyinstruction};
use crate::input::CairoInput;

const LOG_MAX_ROWS: u32 = 20;

const IS_FIRST_LOG_SIZES: [u32; 5] = [4, 6, 7, 14, 18];
pub fn prove_cairo(input: CairoInput) -> Result<CairoProof<Blake2sMerkleHasher>, ProvingError> {
    let _span = span!(Level::INFO, "prove_cairo").entered();
    let config = PcsConfig::default();
    let twiddles = SimdBackend::precompute_twiddles(
        CanonicCoset::new(LOG_MAX_ROWS + config.fri_config.log_blowup_factor + 2)
            .circle_domain()
            .half_coset,
    );

    // Setup protocol.
    let channel = &mut Blake2sChannel::default();
    let commitment_scheme = &mut CommitmentSchemeProver::new(config, &twiddles);

    // Preprocessed trace.
    let mut tree_builder = commitment_scheme.tree_builder();

    tree_builder.extend_evals(
        IS_FIRST_LOG_SIZES
            .iter()
            .cloned()
            .map(gen_is_first::<SimdBackend>),
    );
    tree_builder.commit(channel);

    // Extract public memory.
    let public_memory = input
        .public_mem_addresses
        .iter()
        .copied()
        .map(|addr| {
            let id = input.mem.get_raw_id(addr);
            (addr, id, input.mem.get(addr).as_u256())
        })
        .collect_vec();

    // TODO: Table interaction.

    // Base trace.
    // TODO(Ohad): change to OpcodeClaimProvers, and integrate padding.
    let ret_trace_generator = ret_opcode::ClaimGenerator::new(input.instructions.ret);
    let mut memory_addr_to_id_trace_generator = addr_to_id::ClaimGenerator::new(&input.mem);
    let mut memory_id_to_value_trace_generator = id_to_f252::ClaimGenerator::new(&input.mem);
    let mut range_check_9_9_trace_generator = range_check_9_9::ClaimGenerator::new();
    let mut verify_instruction_trace_generator = verifyinstruction::ClaimGenerator::default();
    let mut range_check_7_2_5_trace_generator = range_check_7_2_5::ClaimGenerator::new();
    let mut range_check_4_3_trace_generator = range_check_4_3::ClaimGenerator::new();

    // TODO(Ohad): Add public memory.
    for &addr in &input.public_mem_addresses {
        let id = memory_addr_to_id_trace_generator.ids[addr as usize];
        memory_addr_to_id_trace_generator.add_m31(M31::from_u32_unchecked(addr));
        memory_id_to_value_trace_generator.add_m31(M31::from_u32_unchecked(id));
    }

    let mut tree_builder = commitment_scheme.tree_builder();

    let (ret_claim, ret_interaction_prover) = ret_trace_generator.write_trace(
        &mut tree_builder,
        &mut memory_addr_to_id_trace_generator,
        &mut memory_id_to_value_trace_generator,
        &mut verify_instruction_trace_generator,
    );
    let (memory_addr_to_id_claim, memory_addr_to_id_interaction_prover) =
        memory_addr_to_id_trace_generator.write_trace(&mut tree_builder);
    let (memory_id_to_value_claim, memory_id_to_value_interaction_prover) =
        memory_id_to_value_trace_generator
            .write_trace(&mut tree_builder, &mut range_check_9_9_trace_generator);
    let (verify_instruction_claim, verify_instruction_interaction_generator) =
        verify_instruction_trace_generator.write_trace(
            &mut tree_builder,
            &mut memory_addr_to_id_trace_generator,
            &mut range_check_4_3_trace_generator,
            &mut range_check_7_2_5_trace_generator,
        );
    let (range_check9_9_claim, range_check9_9_interaction_prover) =
        range_check_9_9_trace_generator.write_trace(&mut tree_builder);
    let (range_check_7_2_5_claim, range_check_7_2_5_interaction_prover) =
        range_check_7_2_5_trace_generator.write_trace(&mut tree_builder);
    let (range_check_4_3_claim, range_check_4_3_interaction_prover) =
        range_check_4_3_trace_generator.write_trace(&mut tree_builder);
    // Commit to the claim and the trace.
    let claim = CairoClaim {
        public_memory,
        initial_state: input.instructions.initial_state,
        final_state: input.instructions.final_state,
        ret: vec![ret_claim],
        memory_addr_to_id: memory_addr_to_id_claim.clone(),
        memory_id_to_value: memory_id_to_value_claim.clone(),
        range_check9_9: range_check9_9_claim.clone(),
        range_check7_2_5: range_check_7_2_5_claim.clone(),
        range_check4_3: range_check_4_3_claim.clone(),
        verify_instruction: verify_instruction_claim,
    };
    claim.mix_into(channel);
    tree_builder.commit(channel);

    // Draw interaction elements.
    let interaction_elements = CairoInteractionElements::draw(channel);

    // Interaction trace.
    let mut tree_builder = commitment_scheme.tree_builder();
    let ret_interaction_claim = ret_interaction_prover.write_interaction_trace(
        &mut tree_builder,
        &interaction_elements.memory_addr_to_id_lookup,
        &interaction_elements.memory_id_to_value_lookup,
        &interaction_elements.verify_instruction_lookup,
        &interaction_elements.opcodes_lookup_elements,
    );
    let memory_addr_to_id_interaction_claim = memory_addr_to_id_interaction_prover
        .write_interaction_trace(
            &mut tree_builder,
            &interaction_elements.memory_addr_to_id_lookup,
        );
    let memory_id_to_value_interaction_claim = memory_id_to_value_interaction_prover
        .write_interaction_trace(
            &mut tree_builder,
            &interaction_elements.memory_id_to_value_lookup,
            &interaction_elements.range9_9_lookup,
        );
    let verifyinstruction_interaction_claim = verify_instruction_interaction_generator
        .write_interaction_trace(
            &mut tree_builder,
            &interaction_elements.memory_addr_to_id_lookup,
            &interaction_elements.memory_id_to_value_lookup,
            &interaction_elements.range4_3_lookup,
            &interaction_elements.range7_2_5_lookup,
            &interaction_elements.verify_instruction_lookup,
        );
    let range_check9_9_interaction_claim = range_check9_9_interaction_prover
        .write_interaction_trace(&mut tree_builder, &interaction_elements.range9_9_lookup);
    let range_check_7_2_5_interaction_claim = range_check_7_2_5_interaction_prover
        .write_interaction_trace(&mut tree_builder, &interaction_elements.range7_2_5_lookup);
    let range_check_4_3_interaction_claim = range_check_4_3_interaction_prover
        .write_interaction_trace(&mut tree_builder, &interaction_elements.range4_3_lookup);

    // Commit to the interaction claim and the interaction trace.
    let interaction_claim = CairoInteractionClaim {
        ret: vec![ret_interaction_claim],
        memory_addr_to_id: memory_addr_to_id_interaction_claim.clone(),
        memory_id_to_value: memory_id_to_value_interaction_claim.clone(),
        range_check9_9: range_check9_9_interaction_claim.clone(),
        range_check7_2_5: range_check_7_2_5_interaction_claim.clone(),
        range_check4_3: range_check_4_3_interaction_claim.clone(),
        verify_instruction: verifyinstruction_interaction_claim,
    };
    debug_assert_eq!(
        lookup_sum(&claim, &interaction_elements, &interaction_claim),
        SecureField::zero()
    );

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

    let log_sizes = claim.log_sizes();

    // Preproccessed trace.
    commitment_scheme_verifier.commit(stark_proof.commitments[0], &log_sizes[0], channel);

    claim.mix_into(channel);
    commitment_scheme_verifier.commit(stark_proof.commitments[1], &log_sizes[1], channel);
    let interaction_elements = CairoInteractionElements::draw(channel);
    if lookup_sum(&claim, &interaction_elements, &interaction_claim) != SecureField::zero() {
        return Err(CairoVerificationError::InvalidLogupSum);
    }
    interaction_claim.mix_into(channel);
    commitment_scheme_verifier.commit(stark_proof.commitments[2], &log_sizes[2], channel);

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

    #[ignore]
    #[test]
    fn test_full_cairo_air() {
        let cairo_proof = prove_cairo(small_cairo_input()).unwrap();
        verify_cairo(cairo_proof).unwrap();
    }
}
