use num_traits::{One, Zero};
use paste::paste;
use stwo_cairo_adapter::builtins::{
    ADD_MOD_MEMORY_CELLS, BITWISE_MEMORY_CELLS, MUL_MOD_MEMORY_CELLS, PEDERSEN_MEMORY_CELLS,
    POSEIDON_MEMORY_CELLS, RANGE_CHECK_MEMORY_CELLS,
};
use stwo_cairo_common::memory::LOG_MEMORY_ADDRESS_BOUND;
use stwo_cairo_common::prover_types::cpu::CasmState;
use stwo_prover::constraint_framework::PREPROCESSED_TRACE_IDX;
use stwo_prover::core::channel::{Channel, MerkleChannel};
use stwo_prover::core::fields::m31::BaseField;
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

fn verify_claim(claim: &CairoClaim) {
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
                pc: final_pc,
                ap: final_ap,
                fp: final_fp,
            },
    } = &claim.public_data;

    verify_builtins(&claim.builtins, public_segments);

    verify_program(program);

    assert_eq!(*initial_pc, BaseField::one());
    assert!(
        *initial_pc + BaseField::from(2) < *initial_ap,
        "Initial pc + 2 must be less than initial ap, but got initial_pc: {}, initial_ap: {}",
        initial_pc,
        initial_ap
    );
    assert_eq!(initial_fp, final_fp);
    assert_eq!(initial_fp, initial_ap);
    assert_eq!(*final_pc, BaseField::from(5));
    assert!(initial_ap <= final_ap);
}

struct BuiltinClaim {
    segment_start: u32,
    log_size: u32,
}

fn verify_builtins(builtins_claim: &BuiltinsClaim, segment_ranges: &PublicSegmentRanges) {
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

    // Output builtin.
    assert!(segment_ranges.output.stop_ptr.value < 1 << 31);
    assert!(segment_ranges.output.start_ptr.value <= segment_ranges.output.stop_ptr.value);

    // Macro for calling `check_builtin` on all builtins except both range_check builtins.
    macro_rules! check_builtin_generic {
        ($name:ident) => {
            paste! {
                check_builtin(
                    builtins_claim.[<$name _builtin>]
                        .map(|claim| BuiltinClaim {
                            segment_start: claim.[<$name _builtin_segment_start>],
                            log_size: claim.log_size,
                        }),
                    segment_ranges.$name,
                    stringify!($name),
                    [<$name:upper _MEMORY_CELLS>]
                );
            }
        };
    }

    // All other supported builtins.
    check_builtin(
        builtins_claim
            .range_check_128_builtin
            .map(|claim| BuiltinClaim {
                segment_start: claim.range_check_builtin_bits_128_segment_start,
                log_size: claim.log_size,
            }),
        segment_ranges.range_check_128,
        "range_check_128",
        RANGE_CHECK_MEMORY_CELLS,
    );
    check_builtin(
        builtins_claim
            .range_check_96_builtin
            .map(|claim| BuiltinClaim {
                segment_start: claim.range_check_builtin_bits_96_segment_start,
                log_size: claim.log_size,
            }),
        segment_ranges.range_check_96,
        "range_check_96",
        RANGE_CHECK_MEMORY_CELLS,
    );
    check_builtin_generic!(bitwise);
    check_builtin_generic!(add_mod);
    check_builtin_generic!(mul_mod);
    check_builtin_generic!(pedersen);
    check_builtin_generic!(poseidon);
}

fn verify_program(program: &MemorySection) {
    assert_eq!(program[0].1, [0x7fff7fff, 0x4078001, 0, 0, 0, 0, 0, 0]); // ap += N_BUILTINS.
    assert_eq!(program[1].1, [11, 0, 0, 0, 0, 0, 0, 0]); // Imm of last instruction (N_BUILTINS).
    assert_eq!(program[2].1, [0x80018000, 0x11048001, 0, 0, 0, 0, 0, 0]); // Instruction: call rel ?
    assert_eq!(program[4].1, [0x7fff7fff, 0x1078001, 0, 0, 0, 0, 0, 0]); // Instruction: jmp rel 0.
    assert_eq!(program[5].1, [0, 0, 0, 0, 0, 0, 0, 0]); // Imm of last instruction (jmp rel 0).
}

fn check_builtin(
    builtin_claim: Option<BuiltinClaim>,
    segment_range: SegmentRange,
    name: &str,
    n_cells: usize,
) {
    if segment_range.is_empty() {
        return;
    }
    // If segment range is non-empty, claim must be Some.
    let BuiltinClaim {
        segment_start,
        log_size,
    } = builtin_claim.unwrap_or_else(|| {
        panic!(
            "Missing {} builtin claim despite non-empty segment range {:?}",
            name, segment_range
        )
    });

    let segment_end = segment_start + (1 << log_size) * n_cells as u32;
    let start_ptr = segment_range.start_ptr.value;
    let stop_ptr = segment_range.stop_ptr.value;
    assert!(
        (stop_ptr - start_ptr) % n_cells as u32 == 0,
        "Builtin segment range must divisible by {} cells, but got start_ptr: {}, stop_ptr: {}",
        n_cells,
        start_ptr,
        stop_ptr
    );

    // Check that segment_start == start_ptr <= stop_ptr <= segment_end < 2**31.
    assert_eq!(
        start_ptr, segment_start,
        "Builtin segment start doesn't match claim"
    );
    assert!(
        start_ptr <= stop_ptr,
        "Range start should be less than or equal to range stop"
    );
    assert!(
        stop_ptr <= segment_end,
        "Builtin stop pointer must be within the builtin segment"
    );
    assert!(
        segment_end < 1 << 31,
        "segment_end must be less than 2^31, but got {}",
        segment_end
    );
}

/// Logup security is defined by the `QM31` space (~124 bits) + `INTERACTION_POW_BITS` -
/// log2(number of relation terms).
/// E.g. assuming a 100-bit security target, the witness may contain up to
/// 1 << (24 + INTERACTION_POW_BITS) relation terms.
pub const INTERACTION_POW_BITS: u32 = 24;

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

    verify_claim(&claim);

    let channel = &mut MC::C::default();
    let commitment_scheme_verifier = &mut CommitmentSchemeVerifier::<MC>::new(pcs_config);

    let mut log_sizes = claim.log_sizes();
    log_sizes[PREPROCESSED_TRACE_IDX] = preprocessed_trace.to_preprocessed_trace().log_sizes();

    // Preproccessed trace.
    commitment_scheme_verifier.commit(stark_proof.commitments[0], &log_sizes[0], channel);

    claim.mix_into(channel);
    commitment_scheme_verifier.commit(stark_proof.commitments[1], &log_sizes[1], channel);

    // Proof of work.
    channel.mix_u64(interaction_pow);
    if channel.trailing_zeros() < INTERACTION_POW_BITS {
        return Err(CairoVerificationError::ProofOfWork);
    }

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
    #[error("Proof of work verification failed.")]
    ProofOfWork,
}
