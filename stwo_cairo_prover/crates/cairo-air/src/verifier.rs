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
    PublicMemory, PublicSegmentRanges, SegmentRange,
};
use crate::builtins_air::BuiltinsClaim;
use crate::components::memory_address_to_id::MEMORY_ADDRESS_TO_ID_SPLIT;
use crate::{CairoProof, PreProcessedTraceVariant};

struct GeneralBuiltinClaim {
    segment_start: u32,
    log_size: u32,
}

fn check_builtin(
    builtin_claim: Option<GeneralBuiltinClaim>,
    segment_range: SegmentRange,
    name: &str,
    n_cells: usize,
) {
    if segment_range.is_empty() {
        return;
    }
    // If segment range is non-empty, claim must be Some.
    let GeneralBuiltinClaim {
        segment_start,
        log_size,
    } = builtin_claim.unwrap_or_else(|| {
        panic!(
            "Missing {} builtin claim despite non-empty segment range {:?}",
            name, segment_range
        )
    });

    assert!(
        segment_range.stop_ptr.value <= 1 << 27,
        "Memory cannot reach beyond 2^27"
    );
    assert!(
        segment_range.start_ptr.value <= segment_range.stop_ptr.value,
        "Range start should be less than or equal to range stop"
    );
    assert_eq!(
        segment_range.start_ptr.value, segment_start,
        "Builtin segment start doesn't match claim"
    );
    assert!(
        segment_range.stop_ptr.value <= segment_start + (1 << log_size) * n_cells as u32,
        "Builtin stop pointer must be within the builtin segment"
    );
}

fn verify_builtin_usage_boundaries(
    builtins_claim: &BuiltinsClaim,
    segment_ranges: &PublicSegmentRanges,
) {
    // Check that non-supported builtins aren't used.
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

    // Check that output start and end pointers make sense.
    assert!(
        segment_ranges.output.stop_ptr.value <= 1 << 27,
        "Memory cannot reach beyond 2^27"
    );
    assert!(segment_ranges.output.start_ptr.value <= segment_ranges.output.stop_ptr.value);

    check_builtin(
        builtins_claim
            .range_check_128_builtin
            .map(|claim| GeneralBuiltinClaim {
                segment_start: claim.range_check_builtin_segment_start,
                log_size: claim.log_size,
            }),
        segment_ranges.range_check_128,
        "range_check_128",
        RANGE_CHECK_MEMORY_CELLS,
    );
    check_builtin(
        builtins_claim
            .bitwise_builtin
            .map(|claim| GeneralBuiltinClaim {
                segment_start: claim.bitwise_builtin_segment_start,
                log_size: claim.log_size,
            }),
        segment_ranges.bitwise,
        "bitwise",
        BITWISE_MEMORY_CELLS,
    );
    check_builtin(
        builtins_claim
            .range_check_96_builtin
            .map(|claim| GeneralBuiltinClaim {
                segment_start: claim.range_check96_builtin_segment_start,
                log_size: claim.log_size,
            }),
        segment_ranges.range_check_96,
        "range_check_96",
        RANGE_CHECK_MEMORY_CELLS,
    );
    check_builtin(
        builtins_claim
            .add_mod_builtin
            .map(|claim| GeneralBuiltinClaim {
                segment_start: claim.add_mod_builtin_segment_start,
                log_size: claim.log_size,
            }),
        segment_ranges.add_mod,
        "add_mod",
        ADD_MOD_MEMORY_CELLS,
    );
    check_builtin(
        builtins_claim
            .mul_mod_builtin
            .map(|claim| GeneralBuiltinClaim {
                segment_start: claim.mul_mod_builtin_segment_start,
                log_size: claim.log_size,
            }),
        segment_ranges.mul_mod,
        "mul_mod",
        MUL_MOD_MEMORY_CELLS,
    );
    check_builtin(
        builtins_claim
            .pedersen_builtin
            .map(|claim| GeneralBuiltinClaim {
                segment_start: claim.pedersen_builtin_segment_start,
                log_size: claim.log_size,
            }),
        segment_ranges.pedersen,
        "pedersen",
        PEDERSEN_MEMORY_CELLS,
    );
    check_builtin(
        builtins_claim
            .poseidon_builtin
            .map(|claim| GeneralBuiltinClaim {
                segment_start: claim.poseidon_builtin_segment_start,
                log_size: claim.log_size,
            }),
        segment_ranges.poseidon,
        "poseidon",
        POSEIDON_MEMORY_CELLS,
    );
}

fn verify_program(program: &MemorySection) {
    assert_eq!(program[0].1, [0x7fff7fff, 0x4078001, 0, 0, 0, 0, 0, 0]); // ap += N_BUILTINS.
    assert_eq!(program[1].1, [11, 0, 0, 0, 0, 0, 0, 0]); // Imm of last instructino (N_BUILTINS).
    assert_eq!(program[2].1, [0x80018000, 0x11048001, 0, 0, 0, 0, 0, 0]); // Instruction: call rel ?
    assert_eq!(program[4].1, [0x7fff7fff, 0x1078001, 0, 0, 0, 0, 0, 0]); // Instruction: jmp rel 0.
    assert_eq!(program[5].1, [0, 0, 0, 0, 0, 0, 0, 0]); // Imm of last instruction (jmp rel 0).
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
                fp: initial_fp,
            },
        final_state:
            CasmState {
                pc: _final_pc,
                ap: final_ap,
                fp: _final_fp,
            },
    } = &claim.public_data;

    verify_builtin_usage_boundaries(&claim.builtins, public_segments);

    verify_program(program);

    assert_eq!(initial_pc.0, 1);
    // TODO(alonf): Soundness issue, assert that final_pc is 5 once fixed in adapter.
    // assert_eq!(final_pc.0, 5);
    assert!(initial_ap <= final_ap);
    // TODO(alonf): Soundness issue, assert that initial_fp == final_fp once fixed in adapter.
    // assert_eq!(initial_fp, final_fp);
    assert_eq!(initial_fp, initial_ap);
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
