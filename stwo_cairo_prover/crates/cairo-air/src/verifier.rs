use std::collections::HashMap;

use num_traits::{One, Zero};
use paste::paste;
use serde_json::to_string_pretty;
use starknet_ff::FieldElement as FieldElement252;
use stwo::core::channel::{Channel, MerkleChannel};
use stwo::core::fields::m31::BaseField;
use stwo::core::fields::qm31::SecureField;
use stwo::core::pcs::CommitmentSchemeVerifier;
use stwo::core::vcs::blake2_hash::Blake2sHash;
use stwo::core::vcs_lifted::blake2_merkle::{Blake2sM31MerkleChannel, Blake2sMerkleChannel};
use stwo::core::vcs_lifted::merkle_hasher::MerkleHasherLifted;
use stwo::core::vcs_lifted::poseidon252_merkle::Poseidon252MerkleChannel;
use stwo::core::verifier::{verify_ex, VerificationError};
use stwo_cairo_common::builtins::*;
use stwo_cairo_common::memory::{LARGE_MEMORY_VALUE_ID_BASE, LOG_MEMORY_ADDRESS_BOUND};
use stwo_cairo_common::preprocessed_columns::preprocessed_trace::PreProcessedTraceVariant;
use stwo_cairo_common::prover_types::cpu::{CasmState, PRIME};
use stwo_constraint_framework::PREPROCESSED_TRACE_IDX;
use thiserror::Error;
use tracing::{span, Level};

use crate::air::{MemorySection, PublicData, PublicMemory, PublicSegmentRanges, SegmentRange};
use crate::cairo_components::CairoComponents;
use crate::claims::{lookup_sum, CairoClaim};
use crate::components::memory_address_to_id::MEMORY_ADDRESS_TO_ID_SPLIT;
use crate::components::{
    add_mod_builtin, bitwise_builtin, ec_op_builtin, mul_mod_builtin, pedersen_builtin,
    pedersen_builtin_narrow_windows, poseidon_builtin, range_check96_builtin, range_check_builtin,
};
use crate::relations::CommonLookupElements;
use crate::CairoProofForRustVerifier;

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

    verify_builtins(
        &claim.add_mod_builtin,
        &claim.bitwise_builtin,
        &claim.mul_mod_builtin,
        &claim.pedersen_builtin,
        &claim.pedersen_builtin_narrow_windows,
        &claim.poseidon_builtin,
        &claim.range_check96_builtin,
        &claim.range_check_builtin,
        &claim.ec_op_builtin,
        public_segments,
    );

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
        .memory_id_to_big
        .as_ref()
        .unwrap()
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

#[allow(clippy::too_many_arguments)]
fn verify_builtins(
    add_mod_builtin_claim: &Option<add_mod_builtin::Claim>,
    bitwise_builtin_claim: &Option<bitwise_builtin::Claim>,
    mul_mod_builtin_claim: &Option<mul_mod_builtin::Claim>,
    pedersen_builtin_claim: &Option<pedersen_builtin::Claim>,
    pedersen_builtin_narrow_windows_claim: &Option<pedersen_builtin_narrow_windows::Claim>,
    poseidon_builtin_claim: &Option<poseidon_builtin::Claim>,
    range_check_96_builtin_claim: &Option<range_check96_builtin::Claim>,
    range_check_128_builtin_claim: &Option<range_check_builtin::Claim>,
    ec_op_builtin_claim: &Option<ec_op_builtin::Claim>,
    segment_ranges: &PublicSegmentRanges,
) {
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

    // Output builtin.
    assert!(output.stop_ptr.value < 1 << 31);
    assert!(output.start_ptr.value <= output.stop_ptr.value);

    // Macro for calling `check_builtin` on all builtins except both range_check builtins.
    macro_rules! check_builtin_generic {
        ($name:ident) => {
            paste! {
                check_builtin(
                    (*[<$name _builtin_claim>]).map(|claim| claim.log_size),
                    $name,
                    stringify!($name),
                    [<$name:upper _BUILTIN_MEMORY_CELLS>]
                );
            }
        };
    }

    // All other supported builtins.
    check_builtin(
        (*range_check_128_builtin_claim).map(|claim| claim.log_size),
        range_check_128,
        "range_check_128",
        RANGE_CHECK_BUILTIN_MEMORY_CELLS,
    );
    check_builtin(
        (*range_check_96_builtin_claim).map(|claim| claim.log_size),
        range_check_96,
        "range_check_96",
        RANGE_CHECK_96_BUILTIN_MEMORY_CELLS,
    );
    assert!(
        !(pedersen_builtin_claim.is_some() && pedersen_builtin_narrow_windows_claim.is_some()),
        "Both pedersen_builtin_claim and pedersen_builtin_narrow_windows_claim builtins cannot be used together");
    if let Some(claim) = pedersen_builtin_claim {
        check_builtin(
            Some(claim.log_size),
            pedersen,
            "pedersen",
            PEDERSEN_BUILTIN_MEMORY_CELLS,
        );
    } else {
        check_builtin(
            pedersen_builtin_narrow_windows_claim.map(|claim| claim.log_size),
            pedersen,
            "pedersen",
            PEDERSEN_BUILTIN_NARROW_WINDOWS_MEMORY_CELLS,
        );
    };
    check_builtin_generic!(bitwise);
    check_builtin_generic!(add_mod);
    check_builtin_generic!(mul_mod);
    check_builtin_generic!(poseidon);
    check_builtin_generic!(ec_op);
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
    log_size: Option<u32>,
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

    // If segment range is non-empty, log_size must be Some.
    let log_size = log_size.unwrap_or_else(|| {
        panic!("Missing {name} builtin claim despite non-empty segment range {segment_range:?}")
    });

    let start_ptr = segment_range.start_ptr.value;
    let stop_ptr = segment_range.stop_ptr.value;
    let segment_end = start_ptr + (1 << log_size) * n_cells as u32;
    assert!(
        (stop_ptr - start_ptr).is_multiple_of(n_cells as u32),
        "Builtin segment range must divisible by {n_cells} cells, but got start_ptr: {start_ptr}, stop_ptr: {stop_ptr}"
    );

    // Check that start_ptr <= stop_ptr <= segment_end < 2**31.
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

/// Static preprocessed-root policy for Rust verifier configurations.
///
/// `Exact` enforces Cairo-verifier-style binding by comparing the proof's preprocessed
/// commitment root against a verifier-known root before the root is committed to the channel.
/// `NoStaticRoot` is an explicit opt-out: the Rust verifier commits the proof-supplied
/// preprocessed root without this static equality check.
pub trait ExpectedPreprocessedRoot: MerkleChannel
where
    Self::H: MerkleHasherLifted,
{
    fn expected_preprocessed_root(
        variant: PreProcessedTraceVariant,
        log_blowup_factor: u32,
        lifting_log_size: Option<u32>,
    ) -> Result<
        ExpectedPreprocessedRootPolicy<<Self::H as MerkleHasherLifted>::Hash>,
        CairoVerificationError,
    >;
}

pub enum ExpectedPreprocessedRootPolicy<Hash> {
    /// This verifier configuration intentionally has no built-in static preprocessed root.
    /// Callers must not infer Cairo-style static root binding for modes that return this policy.
    NoStaticRoot,
    Exact(Hash),
}

impl ExpectedPreprocessedRoot for Blake2sMerkleChannel {
    fn expected_preprocessed_root(
        variant: PreProcessedTraceVariant,
        log_blowup_factor: u32,
        lifting_log_size: Option<u32>,
    ) -> Result<
        ExpectedPreprocessedRootPolicy<<Self::H as MerkleHasherLifted>::Hash>,
        CairoVerificationError,
    > {
        if variant != PreProcessedTraceVariant::Canonical {
            return Ok(ExpectedPreprocessedRootPolicy::NoStaticRoot);
        }
        if lifting_log_size.is_some() {
            return Err(CairoVerificationError::UnsupportedPreprocessedRootConfig);
        }
        let root = match log_blowup_factor {
            1 => [
                0x42228ea9, 0x35d2f53b, 0x360b1f98, 0x56ae39d9, 0x75e23bef, 0x32b0581c, 0x6e1e83b2,
                0x6403ba24,
            ],
            2 => [
                0x7094e904, 0x210a38ad, 0x126b6cee, 0x57097de8, 0x1c860da1, 0x91b704b1, 0xc6cf280a,
                0x8a211523,
            ],
            3 => [
                0x84cb79b9, 0xb050ad16, 0x69787584, 0xcf7f274f, 0x399792c8, 0xecf77fed, 0x458488fe,
                0xe27bbcac,
            ],
            4 => [
                0x803fe777, 0x1d9267a0, 0xe383c36d, 0x2b1b4bf0, 0x9a47969f, 0xcb683ef4, 0x598eca47,
                0x09db42f9,
            ],
            5 => [
                0xbfee7cd5, 0x429ca185, 0xa8d60ba7, 0x3856e072, 0xb88de2aa, 0x12ab5bc3, 0xde44271d,
                0x2500318a,
            ],
            _ => return Err(CairoVerificationError::UnsupportedPreprocessedRootConfig),
        };
        Ok(ExpectedPreprocessedRootPolicy::Exact(Blake2sHash(
            root.map(u32::to_le_bytes).concat().try_into().unwrap(),
        )))
    }
}

impl ExpectedPreprocessedRoot for Blake2sM31MerkleChannel {
    fn expected_preprocessed_root(
        _variant: PreProcessedTraceVariant,
        _log_blowup_factor: u32,
        _lifting_log_size: Option<u32>,
    ) -> Result<
        ExpectedPreprocessedRootPolicy<<Self::H as MerkleHasherLifted>::Hash>,
        CairoVerificationError,
    > {
        Ok(ExpectedPreprocessedRootPolicy::NoStaticRoot)
    }
}

impl ExpectedPreprocessedRoot for Poseidon252MerkleChannel {
    fn expected_preprocessed_root(
        variant: PreProcessedTraceVariant,
        log_blowup_factor: u32,
        lifting_log_size: Option<u32>,
    ) -> Result<
        ExpectedPreprocessedRootPolicy<<Self::H as MerkleHasherLifted>::Hash>,
        CairoVerificationError,
    > {
        if variant != PreProcessedTraceVariant::CanonicalWithoutPedersen {
            return Ok(ExpectedPreprocessedRootPolicy::NoStaticRoot);
        }
        if lifting_log_size.is_some() {
            return Err(CairoVerificationError::UnsupportedPreprocessedRootConfig);
        }
        let root = match log_blowup_factor {
            1 => "0x1fdacd6e29834987926672fcac7fda0577adce49999ad96fb0195e745e9ce0",
            2 => "0x14103a81a8417d83c41bc877194607103f9e1cb4cb188e5f21b75499709bbb6",
            3 => "0x20861f94b3cf37baf1939d20691d38630596bce389dcee5f224ef1f5682e70f",
            4 => "0x26f3a39ddf7042fefdd2595bf4739063e044845418839b2b1ec556fdc1aecf",
            5 => "0x2111b7db0f7fb2f629d493661207663b88609e5abebbe7e59c4f116dc3dac0",
            _ => return Err(CairoVerificationError::UnsupportedPreprocessedRootConfig),
        };
        FieldElement252::from_hex_be(root)
            .map(ExpectedPreprocessedRootPolicy::Exact)
            .map_err(|_| CairoVerificationError::UnsupportedPreprocessedRootConfig)
    }
}

/// Verifies a Cairo proof with the Rust verifier.
///
/// Static preprocessed-root binding is enforced only for configurations whose
/// [`ExpectedPreprocessedRoot`] implementation returns [`ExpectedPreprocessedRootPolicy::Exact`].
/// The built-in exact policies are Blake2s + `Canonical` and Poseidon252 +
/// `CanonicalWithoutPedersen`, for supported log blowup factors without lifting.
/// Configurations that return [`ExpectedPreprocessedRootPolicy::NoStaticRoot`] remain valid
/// verifier modes, but they do not enforce Cairo-style static preprocessed-root equality.
pub fn verify_cairo<MC: ExpectedPreprocessedRoot>(
    proof: CairoProofForRustVerifier<MC::H>,
) -> Result<(), CairoVerificationError> {
    verify_cairo_ex::<MC>(proof, false)
}

pub fn verify_cairo_ex<MC: ExpectedPreprocessedRoot>(
    CairoProofForRustVerifier {
        claim,
        interaction_pow,
        interaction_claim,
        stark_proof,
        channel_salt,
        preprocessed_trace_variant,
    }: CairoProofForRustVerifier<MC::H>,
    include_all_preprocessed_columns: bool,
) -> Result<(), CairoVerificationError> {
    let _span = span!(Level::INFO, "verify_cairo").entered();

    // Auxiliary verifications.
    // Assert that ADDRESS->ID component does not overflow.
    assert!(
        (1 << claim.memory_address_to_id.as_ref().unwrap().log_size) * MEMORY_ADDRESS_TO_ID_SPLIT
            <= (1 << LOG_MEMORY_ADDRESS_BOUND)
    );

    verify_claim(&claim);

    let channel = &mut MC::C::default();
    channel.mix_felts(&[channel_salt.into()]);

    let pcs_config = stark_proof.config;
    pcs_config.mix_into(channel);
    let commitment_scheme_verifier = &mut CommitmentSchemeVerifier::<MC>::new(pcs_config);

    let mut log_sizes = claim.log_sizes();
    log_sizes.insert(
        PREPROCESSED_TRACE_IDX,
        preprocessed_trace_variant
            .to_preprocessed_trace()
            .log_sizes(),
    );

    if let ExpectedPreprocessedRootPolicy::Exact(expected_preprocessed_root) =
        MC::expected_preprocessed_root(
            preprocessed_trace_variant,
            pcs_config.fri_config.log_blowup_factor,
            pcs_config.lifting_log_size,
        )?
    {
        if stark_proof.commitments[0] != expected_preprocessed_root {
            return Err(CairoVerificationError::InvalidPreprocessedRoot);
        }
    }

    // Preproccessed trace.
    commitment_scheme_verifier.commit(stark_proof.commitments[0], &log_sizes[0], channel);

    claim.mix_into::<MC>(channel);
    commitment_scheme_verifier.commit(stark_proof.commitments[1], &log_sizes[1], channel);

    // Proof of work.
    if !channel.verify_pow_nonce(INTERACTION_POW_BITS, interaction_pow) {
        return Err(CairoVerificationError::ProofOfWork);
    }
    channel.mix_u64(interaction_pow);
    let interaction_elements = CommonLookupElements::draw(channel);

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
        &preprocessed_trace_variant.to_preprocessed_trace().ids(),
    );
    let components = component_generator.components();

    // Verify stark.
    verify_ex(
        &components,
        channel,
        commitment_scheme_verifier,
        stark_proof,
        include_all_preprocessed_columns,
    )
    .map_err(CairoVerificationError::Stark)
}

#[derive(Error, Debug)]
pub enum CairoVerificationError {
    #[error("Invalid logup sum")]
    InvalidLogupSum,
    #[error("Invalid preprocessed commitment root")]
    InvalidPreprocessedRoot,
    #[error("Unsupported preprocessed root configuration for static root verifier")]
    UnsupportedPreprocessedRootConfig,
    #[error("Stark verification error: {0}")]
    Stark(#[from] VerificationError),
    #[error("Proof of work verification failed.")]
    ProofOfWork,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn expect_exact_blake2s_root(log_blowup_factor: u32) -> Blake2sHash {
        match Blake2sMerkleChannel::expected_preprocessed_root(
            PreProcessedTraceVariant::Canonical,
            log_blowup_factor,
            None,
        )
        .unwrap()
        {
            ExpectedPreprocessedRootPolicy::Exact(root) => root,
            ExpectedPreprocessedRootPolicy::NoStaticRoot => {
                panic!("expected a static Blake2s root")
            }
        }
    }

    #[test]
    fn blake2s_canonical_roots_are_defined_for_supported_blowup_factors() {
        for log_blowup_factor in 1..=5 {
            let root = expect_exact_blake2s_root(log_blowup_factor);
            assert_ne!(root, Blake2sHash([0; 32]));
        }
    }

    #[test]
    fn blake2s_canonical_root_policy_fails_closed_for_unsupported_config() {
        assert!(matches!(
            Blake2sMerkleChannel::expected_preprocessed_root(
                PreProcessedTraceVariant::Canonical,
                6,
                None,
            ),
            Err(CairoVerificationError::UnsupportedPreprocessedRootConfig)
        ));
        assert!(matches!(
            Blake2sMerkleChannel::expected_preprocessed_root(
                PreProcessedTraceVariant::Canonical,
                1,
                Some(26),
            ),
            Err(CairoVerificationError::UnsupportedPreprocessedRootConfig)
        ));
    }

    #[test]
    fn poseidon_canonical_without_pedersen_root_policy_fails_closed() {
        assert!(matches!(
            Poseidon252MerkleChannel::expected_preprocessed_root(
                PreProcessedTraceVariant::CanonicalWithoutPedersen,
                6,
                None,
            ),
            Err(CairoVerificationError::UnsupportedPreprocessedRootConfig)
        ));
        assert!(matches!(
            Poseidon252MerkleChannel::expected_preprocessed_root(
                PreProcessedTraceVariant::CanonicalWithoutPedersen,
                1,
                Some(26),
            ),
            Err(CairoVerificationError::UnsupportedPreprocessedRootConfig)
        ));
    }

    #[test]
    fn non_static_root_modes_remain_explicitly_unbound() {
        assert!(matches!(
            Blake2sMerkleChannel::expected_preprocessed_root(
                PreProcessedTraceVariant::CanonicalWithoutPedersen,
                1,
                None,
            )
            .unwrap(),
            ExpectedPreprocessedRootPolicy::NoStaticRoot
        ));
        assert!(matches!(
            Blake2sM31MerkleChannel::expected_preprocessed_root(
                PreProcessedTraceVariant::Canonical,
                6,
                Some(26),
            )
            .unwrap(),
            ExpectedPreprocessedRootPolicy::NoStaticRoot
        ));
    }
}
