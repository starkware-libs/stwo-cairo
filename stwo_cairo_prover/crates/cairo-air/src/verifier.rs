use num_traits::Zero;
use stwo_cairo_adapter::builtins::{
    ADD_MOD_MEMORY_CELLS, BITWISE_MEMORY_CELLS, MUL_MOD_MEMORY_CELLS, PEDERSEN_MEMORY_CELLS,
    POSEIDON_MEMORY_CELLS, RANGE_CHECK_MEMORY_CELLS,
};
use stwo_cairo_common::memory::LOG_MEMORY_ADDRESS_BOUND;
use stwo_cairo_common::prover_types::cpu::CasmState;
use stwo_prover::constraint_framework::PREPROCESSED_TRACE_IDX;
use stwo_prover::core::channel::{Channel, MerkleChannel};
use stwo_prover::core::fields::qm31::SecureField;
use stwo_prover::core::pcs::{CommitmentSchemeVerifier, PcsConfig};
use stwo_prover::core::prover::{verify, VerificationError};
use thiserror::Error;

use crate::air::{
    lookup_sum, CairoClaim, CairoComponents, CairoInteractionElements, MemorySection, PublicData,
    PublicMemory, PublicSegmentRanges,
};
use crate::builtins_air::BuiltinsClaim;
use crate::components::memory_address_to_id::MEMORY_ADDRESS_TO_ID_SPLIT;
use crate::{CairoProof, PreProcessedTraceVariant};

fn verify_builtin_usage_boundaries(
    builtins_claim: &BuiltinsClaim,
    segment_ranges: &PublicSegmentRanges,
) {
    // Check that all non-supported builtins aren't used.
    assert_eq!(
        segment_ranges.ec_op.start_ptr.value,
        segment_ranges.ec_op.stop_ptr.value
    );
    assert_eq!(
        segment_ranges.ecdsa.start_ptr.value,
        segment_ranges.ecdsa.stop_ptr.value
    );
    assert_eq!(
        segment_ranges.keccak.start_ptr.value,
        segment_ranges.keccak.stop_ptr.value
    );

    // Check that output start and end pointer make sense.
    assert!(
        segment_ranges.output.stop_ptr.value <= 1 << 27,
        "Memory cannot reach beyond 2^27"
    );
    assert!(segment_ranges.output.start_ptr.value <= segment_ranges.output.stop_ptr.value);

    // `builtins_info` an iterable of tuples containing the builtin segment start, log size, number
    // of memory cells per builtin instance, and the builtin usage range.
    let builtins_info = [
        builtins_claim.range_check_128_builtin.map(|claim| {
            (
                claim.range_check_builtin_segment_start,
                claim.log_size,
                RANGE_CHECK_MEMORY_CELLS,
                segment_ranges.range_check_128,
            )
        }),
        builtins_claim.bitwise_builtin.map(|claim| {
            (
                claim.bitwise_builtin_segment_start,
                claim.log_size,
                BITWISE_MEMORY_CELLS,
                segment_ranges.bitwise,
            )
        }),
        builtins_claim.range_check_96_builtin.map(|claim| {
            (
                claim.range_check96_builtin_segment_start,
                claim.log_size,
                RANGE_CHECK_MEMORY_CELLS,
                segment_ranges.range_check_96,
            )
        }),
        builtins_claim.add_mod_builtin.map(|claim| {
            (
                claim.add_mod_builtin_segment_start,
                claim.log_size,
                ADD_MOD_MEMORY_CELLS,
                segment_ranges.add_mod,
            )
        }),
        builtins_claim.mul_mod_builtin.map(|claim| {
            (
                claim.mul_mod_builtin_segment_start,
                claim.log_size,
                MUL_MOD_MEMORY_CELLS,
                segment_ranges.mul_mod,
            )
        }),
        builtins_claim.pedersen_builtin.map(|claim| {
            (
                claim.pedersen_builtin_segment_start,
                claim.log_size,
                PEDERSEN_MEMORY_CELLS,
                segment_ranges.pedersen,
            )
        }),
        builtins_claim.poseidon_builtin.map(|claim| {
            (
                claim.poseidon_builtin_segment_start,
                claim.log_size,
                POSEIDON_MEMORY_CELLS,
                segment_ranges.poseidon,
            )
        }),
    ]
    .into_iter()
    .flatten();

    for (start, log_size, n_cells, range) in builtins_info {
        assert!(
            range.stop_ptr.value <= 1 << 27,
            "Memory cannot reach beyond 2^27"
        );
        assert!(range.start_ptr.value <= range.stop_ptr.value);
        assert_eq!(range.start_ptr.value, start);
        assert!(
            range.stop_ptr.value <= start + (1 << log_size) * n_cells as u32,
            "Builtin stop pointer must be within the builtin segment"
        );
    }
}

fn verify_program(program: &MemorySection) {
    assert_eq!(program[0].1, [0x7fff7fff, 0x4078001, 0, 0, 0, 0, 0, 0]); // Instruction: ap += N_BUILTINS.
    assert_eq!(program[1].1, [11, 0, 0, 0, 0, 0, 0, 0]); // Immediate of last instruction, should equal 11.
    assert_eq!(program[2].1, [0x80018000, 0x11048001, 0, 0, 0, 0, 0, 0]); // Instruction: call rel ?.
    assert_eq!(program[4].1, [0x7fff7fff, 0x1078001, 0, 0, 0, 0, 0, 0]); // Instruction: call rel ?.
    assert_eq!(program[5].1, [0, 0, 0, 0, 0, 0, 0, 0]); // Instruction: call rel ?.
}
fn verify_public_input(claim: &CairoClaim) {
    let PublicData {
        public_memory:
            PublicMemory {
                program,
                public_segments,
                output: _output,
                safe_call: _safe_call,
            },
        initial_state:
            CasmState {
                pc: initial_pc,
                ap: initial_ap,
                fp: _initial_fp,
            },
        final_state:
            CasmState {
                pc: final_pc,
                ap: final_ap,
                fp: _final_fp,
            },
    } = &claim.public_data;

    verify_builtin_usage_boundaries(&claim.builtins, public_segments);

    verify_program(program);

    assert_eq!(initial_pc.0, 1);
    assert_eq!(final_pc.0, 5);
    assert!(initial_ap <= final_ap);
}

pub fn verify_cairo<MC: MerkleChannel>(
    CairoProof {
        claim,
        interaction_pow,
        interaction_claim,
        stark_proof,
    }: CairoProof<MC::H>,
    pcs_config: PcsConfig,
    preprocessed_trace: PreProcessedTraceVariant,
) -> Result<(), CairoVerificationError> {
    // Auxiliary verifications.
    // Assert that ADDRESS->ID component does not overflow.
    assert!(
        (1 << claim.memory_address_to_id.log_size) * MEMORY_ADDRESS_TO_ID_SPLIT
            <= (1 << LOG_MEMORY_ADDRESS_BOUND)
    );

    verify_public_input(&claim);

    let channel = &mut MC::C::default();
    let commitment_scheme_verifier = &mut CommitmentSchemeVerifier::<MC>::new(pcs_config);

    let mut log_sizes = claim.log_sizes();
    log_sizes[PREPROCESSED_TRACE_IDX] = preprocessed_trace.to_preprocessed_trace().log_sizes();

    // Preproccessed trace.
    commitment_scheme_verifier.commit(stark_proof.commitments[0], &log_sizes[0], channel);

    claim.mix_into(channel);
    commitment_scheme_verifier.commit(stark_proof.commitments[1], &log_sizes[1], channel);
    channel.mix_u64(interaction_pow);
    let interaction_elements = CairoInteractionElements::draw(channel);

    // Verify lookup argument.
    if lookup_sum(&claim, &interaction_elements, &interaction_claim) != SecureField::zero() {
        return Err(CairoVerificationError::InvalidLogupSum);
    }
    interaction_claim.mix_into(channel);
    commitment_scheme_verifier.commit(stark_proof.commitments[2], &log_sizes[2], channel);

    let component_generator = CairoComponents::new(
        &claim,
        &interaction_elements,
        &interaction_claim,
        &preprocessed_trace.to_preprocessed_trace().ids(),
    );
    let components = component_generator.components();

    // Verify stark.
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
