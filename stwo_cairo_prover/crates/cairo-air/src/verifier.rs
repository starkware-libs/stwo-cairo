use std::collections::HashMap;

use num_traits::{One, Zero};
use paste::paste;
use serde_json::to_string_pretty;
use stwo::core::channel::{Channel, MerkleChannel};
use stwo::core::fields::m31::BaseField;
use stwo::core::fields::qm31::SecureField;
use stwo::core::pcs::CommitmentSchemeVerifier;
use stwo::core::verifier::{verify, VerificationError};
use stwo_cairo_common::builtins::{
    ADD_MOD_MEMORY_CELLS, BITWISE_MEMORY_CELLS, MUL_MOD_MEMORY_CELLS, PEDERSEN_MEMORY_CELLS,
    POSEIDON_MEMORY_CELLS, RANGE_CHECK_MEMORY_CELLS,
};
use stwo_cairo_common::memory::{LARGE_MEMORY_VALUE_ID_BASE, LOG_MEMORY_ADDRESS_BOUND};
use stwo_cairo_common::prover_types::cpu::{CasmState, PRIME};
use stwo_constraint_framework::PREPROCESSED_TRACE_IDX;
use thiserror::Error;
use tracing::{span, Level};

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
                safe_call_ids: _safe_call_ids,
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

    verify_program(program, public_segments);

    assert_eq!(*initial_pc, BaseField::one());
    assert!(
        *initial_pc + BaseField::from(2) < *initial_ap,
        "Initial pc + 2 must be less than initial ap, but got initial_pc: {initial_pc}, initial_ap: {initial_ap}"
    );
    assert_eq!(initial_fp, final_fp);
    assert_eq!(initial_fp, initial_ap);
    assert_eq!(*final_pc, BaseField::from(5));
    assert!(initial_ap <= final_ap);

    // Assert that each relation has strictly less than P uses.
    let mut relation_uses = HashMap::<&'static str, u64>::new();
    claim.accumulate_relation_uses(&mut relation_uses);
    check_relation_uses(&relation_uses);

    // Large value IDs reside in [LARGE_MEMORY_VALUE_ID_BASE..P).
    // Check that IDs in (ID -> Value) do not overflow P.
    let largest_id = claim
        .memory_id_to_value
        .big_log_sizes
        .iter()
        .map(|log_size| 1 << log_size)
        .sum::<u32>()
        - 1
        + LARGE_MEMORY_VALUE_ID_BASE;
    assert!(largest_id < PRIME);
}

fn check_relation_uses(relation_uses: &HashMap<&'static str, u64>) {
    let all_relation_uses_pretty = to_string_pretty(&relation_uses).unwrap();
    log::info!("All relation uses:\n{all_relation_uses_pretty}");

    let outstanding_relations = relation_uses
        .iter()
        .filter(|(_, &uses)| uses >= PRIME.into())
        .collect::<Vec<_>>();

    if !outstanding_relations.is_empty() {
        let outstanding_relations_pretty = to_string_pretty(&outstanding_relations).unwrap();
        panic!(
            "Found {} outstanding relations:\n{}",
            outstanding_relations.len(),
            outstanding_relations_pretty
        );
    }
}

#[derive(Clone)]
pub struct RelationUse {
    pub relation_id: &'static str,
    pub uses: u64,
}

struct BuiltinClaim {
    segment_start: u32,
    log_size: u32,
}

fn verify_builtins(builtins_claim: &BuiltinsClaim, segment_ranges: &PublicSegmentRanges) {
    let PublicSegmentRanges {
        output,
        pedersen,
        range_check_128,
        ecdsa,
        bitwise,
        ec_op,
        keccak,
        poseidon,
        range_check_96,
        add_mod,
        mul_mod,
    } = *segment_ranges;
    // Check that non-supported builtins aren't used.
    if let Some(ecdsa) = ecdsa {
        assert_eq!(
            ecdsa.start_ptr.value, ecdsa.stop_ptr.value,
            "ECDSA segment is not empty"
        );
    }
    if let Some(keccak) = keccak {
        assert_eq!(
            keccak.start_ptr.value, keccak.stop_ptr.value,
            "Keccak segment is not empty"
        );
    }
    if let Some(ec_op) = ec_op {
        assert_eq!(
            ec_op.start_ptr.value, ec_op.stop_ptr.value,
            "EC_OP segment is not empty"
        );
    }

    // Output builtin.
    assert!(output.stop_ptr.value < 1 << 31);
    assert!(output.start_ptr.value <= output.stop_ptr.value);

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
                    $name,
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
                segment_start: claim.range_check_builtin_segment_start,
                log_size: claim.log_size,
            }),
        range_check_128,
        "range_check_128",
        RANGE_CHECK_MEMORY_CELLS,
    );
    check_builtin(
        builtins_claim
            .range_check_96_builtin
            .map(|claim| BuiltinClaim {
                segment_start: claim.range_check96_builtin_segment_start,
                log_size: claim.log_size,
            }),
        range_check_96,
        "range_check_96",
        RANGE_CHECK_MEMORY_CELLS,
    );
    check_builtin_generic!(bitwise);
    check_builtin_generic!(add_mod);
    check_builtin_generic!(mul_mod);
    check_builtin_generic!(pedersen);
    check_builtin_generic!(poseidon);
}

fn verify_program(program: &MemorySection, public_segments: &PublicSegmentRanges) {
    // For information about how the compiler adds this code, see:
    // https://github.com/starkware-libs/cairo/blob/3babe0518abc8e4fc72f519fb515d6c752138f78/crates/cairo-lang-executable/src/executable.rs#L21-L25

    // First instruction: add_app_immediate (n_builtins).
    let n_builtins = public_segments.present_segments().len() as u32;
    assert_eq!(program[0].1, [0x7fff7fff, 0x4078001, 0, 0, 0, 0, 0, 0]); // add_ap_imm.
    assert_eq!(program[1].1, [n_builtins, 0, 0, 0, 0, 0, 0, 0]); // Imm.

    // Safe call.
    assert_eq!(program[2].1, [0x80018000, 0x11048001, 0, 0, 0, 0, 0, 0]); // Instruction: call rel ?
    assert_eq!(program[4].1, [0x7fff7fff, 0x1078001, 0, 0, 0, 0, 0, 0]); // Instruction: jmp rel 0.
    assert_eq!(program[5].1, [0, 0, 0, 0, 0, 0, 0, 0]); // Imm of last instruction (jmp rel 0).
}

fn check_builtin(
    builtin_claim: Option<BuiltinClaim>,
    segment_range: Option<SegmentRange>,
    name: &str,
    n_cells: usize,
) {
    let segment_range = match segment_range {
        None => return,
        Some(segment_range) => {
            if segment_range.is_empty() {
                return;
            }
            segment_range
        }
    };

    // If segment range is non-empty, claim must be Some.
    let BuiltinClaim {
        segment_start,
        log_size,
    } = builtin_claim.unwrap_or_else(|| {
        panic!("Missing {name} builtin claim despite non-empty segment range {segment_range:?}")
    });

    let segment_end = segment_start + (1 << log_size) * n_cells as u32;
    let start_ptr = segment_range.start_ptr.value;
    let stop_ptr = segment_range.stop_ptr.value;
    assert!(
        (stop_ptr - start_ptr).is_multiple_of(n_cells as u32),
        "Builtin segment range must divisible by {n_cells} cells, but got start_ptr: {start_ptr}, stop_ptr: {stop_ptr}"
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
        "segment_end must be less than 2^31, but got {segment_end}"
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
        channel_salt,
    }: CairoProof<MC::H>,
    preprocessed_trace: PreProcessedTraceVariant,
) -> Result<(), CairoVerificationError> {
    let _span = span!(Level::INFO, "verify_cairo").entered();

    // Auxiliary verifications.
    // Assert that ADDRESS->ID component does not overflow.
    assert!(
        (1 << claim.memory_address_to_id.log_size) * MEMORY_ADDRESS_TO_ID_SPLIT
            <= (1 << LOG_MEMORY_ADDRESS_BOUND)
    );

    verify_claim(&claim);

    let channel = &mut MC::C::default();
    if let Some(salt) = channel_salt {
        channel.mix_u64(salt);
    }
    let pcs_config = stark_proof.config;
    pcs_config.mix_into(channel);
    let commitment_scheme_verifier = &mut CommitmentSchemeVerifier::<MC>::new(pcs_config);

    let mut log_sizes = claim.log_sizes();
    log_sizes[PREPROCESSED_TRACE_IDX] = preprocessed_trace.to_preprocessed_trace().log_sizes();

    // Preproccessed trace.
    commitment_scheme_verifier.commit(stark_proof.commitments[0], &log_sizes[0], channel);

    claim.mix_into(channel);
    commitment_scheme_verifier.commit(stark_proof.commitments[1], &log_sizes[1], channel);

    // Proof of work.
    if !channel.verify_pow_nonce(INTERACTION_POW_BITS, interaction_pow) {
        return Err(CairoVerificationError::ProofOfWork);
    }
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
    #[error("Proof of work verification failed.")]
    ProofOfWork,
}
