use components::memory_address_to_id::{
    InteractionClaimImpl as MemoryAddressToIdInteractionClaimImpl, LOG_MEMORY_ADDRESS_TO_ID_SPLIT,
};
use components::memory_id_to_big::InteractionClaimImpl as MemoryIdToBigInteractionClaimImpl;
use stwo_verifier_core::Hash;
use stwo_verifier_utils::{PublicMemoryEntries, PublicMemoryEntriesTrait, PublicMemoryEntry};

#[cfg(feature: "poseidon252_verifier")]
mod poseidon252_verifier_uses {
    pub use core::poseidon::poseidon_hash_span;
    pub use stwo_verifier_utils::deconstruct_f252;
}
#[cfg(feature: "poseidon252_verifier")]
use poseidon252_verifier_uses::*;


#[cfg(not(feature: "poseidon252_verifier"))]
mod blake2s_verifier_uses {
    pub use core::blake::{blake2s_compress, blake2s_finalize};
    pub use stwo_verifier_utils::BLAKE2S_256_INITIAL_STATE;
}
#[cfg(not(feature: "poseidon252_verifier"))]
use blake2s_verifier_uses::*;
use core::box::BoxImpl;
use core::dict::{Felt252Dict, Felt252DictEntryTrait, Felt252DictTrait, SquashedFelt252DictTrait};
use core::iter::{Extend, Iterator};
use core::num::traits::Zero;
use core::num::traits::one::One;
use core::traits::TryInto;
use stwo_constraint_framework::{
    CommonLookupElements, LookupElementsImpl, PreprocessedMaskValuesImpl,
};
use stwo_verifier_core::channel::{Channel, ChannelImpl, ChannelTrait};
use stwo_verifier_core::fields::Invertible;
#[cfg(not(feature: "qm31_opcode"))]
use stwo_verifier_core::fields::m31::{AddM31Trait, MulByM31Trait};
use stwo_verifier_core::fields::m31::{M31, M31Trait, P_U32};
use stwo_verifier_core::fields::qm31::QM31;
#[cfg(not(feature: "qm31_opcode"))]
use stwo_verifier_core::fields::qm31::{PackedUnreducedQM31, PackedUnreducedQM31Trait};
use stwo_verifier_core::pcs::PcsConfigTrait;
use stwo_verifier_core::pcs::verifier::{CommitmentSchemeVerifierImpl, get_trace_lde_log_size};
use stwo_verifier_core::utils::{ArrayImpl, OptionImpl, pack_into_qm31s, pow2};
use stwo_verifier_core::verifier::{StarkProof, verify};
use stwo_verifier_utils::{MemorySection, PubMemoryValue, construct_f252};
use crate::components::memory_address_to_id::*;
use crate::components::memory_id_to_big::*;
use crate::utils::split;
#[cairofmt::skip]
mod hash_imports {
    // Program hash function
    // The program is always hashed using its channel hash function.
    #[cfg(not(feature: "poseidon252_verifier"))]
    pub use stwo_verifier_utils::blake2s::encode_and_hash_memory_section as encode_and_hash_program_memory_section;
    #[cfg(feature: "poseidon252_verifier")]
    pub use stwo_verifier_utils::poseidon252::encode_and_hash_memory_section as encode_and_hash_program_memory_section;

    // Outputs hash function
    #[cfg(feature: "blake_outputs_packing")]
    pub use stwo_verifier_utils::blake2s::encode_and_hash_memory_section as encode_and_hash_outputs_memory_section;
    #[cfg(feature: "poseidon_outputs_packing")]
    pub use stwo_verifier_utils::poseidon252::encode_and_hash_memory_section as encode_and_hash_outputs_memory_section;
}
use hash_imports::*;

pub mod claims;
use claims::{
    CairoClaim, CairoClaimImpl, CairoInteractionClaim, CairoInteractionClaimImpl, lookup_sum,
};
pub mod cairo_air;
use cairo_air::*;

pub mod cairo_component;
pub mod components;

// This module checks the validity of feature combinations. It's placed in this crate since it's the
// first to compile, allowing early detection of invalid feature configurations.
mod features_check;
mod prelude;


#[cfg(test)]
mod profiling;
#[cfg(test)]
mod test;
pub mod test_utils;
pub mod utils;

// Security constants.
pub const INTERACTION_POW_BITS: u32 = 24;
const SECURITY_BITS: u32 = 96;

pub const ADD_MOD_MEMORY_CELLS: usize = 7;
pub const BITWISE_MEMORY_CELLS: usize = 5;
pub const EC_OP_MEMORY_CELLS: usize = 7;
pub const ECDSA_MEMORY_CELLS: usize = 2;
pub const KECCAK_MEMORY_CELLS: usize = 16;
pub const MUL_MOD_MEMORY_CELLS: usize = 7;
pub const PEDERSEN_MEMORY_CELLS: usize = 3;
pub const POSEIDON_MEMORY_CELLS: usize = 6;
// This is for both the 128 and 96 bit range checks.
pub const RANGE_CHECK_MEMORY_CELLS: usize = 1;

pub mod blake;
pub mod builtins;
pub mod opcodes;
pub mod pedersen;
pub mod poseidon;
pub mod preprocessed_columns;
pub mod range_checks;
use preprocessed_columns::preprocessed_root;

pub mod claim;

// TODO(az-starkware): Once we upgrade Cairo version, move these to preprocessed_column.cairo
// using #[path = "..."]
#[cfg(not(feature: "poseidon252_verifier"))]
mod preprocessed_columns_canonical;
#[cfg(feature: "poseidon252_verifier")]
mod preprocessed_columns_without_pedersen;

// A dict from relation_id, which is a string encoded as a felt252, to the number of uses of the
// corresponding relation.
type RelationUsesDict = Felt252Dict<u64>;

// A tuple of (relation_id, uses).
type RelationUse = (felt252, u32);

#[derive(Drop, Serde)]
pub struct CairoProof {
    pub claim: CairoClaim,
    pub interaction_pow: u64,
    pub interaction_claim: CairoInteractionClaim,
    pub stark_proof: StarkProof,
    /// Salt used in the channel initialization.
    pub channel_salt: u32,
}

/// The output of a verification.
#[cfg(not(or(feature: "blake_outputs_packing", feature: "poseidon_outputs_packing")))]
#[derive(Drop, Serde)]
pub struct VerificationOutput {
    pub program_hash: felt252,
    pub output: Array<felt252>,
}
#[cfg(or(feature: "blake_outputs_packing", feature: "poseidon_outputs_packing"))]
#[derive(Drop, Serde)]
pub struct VerificationOutput {
    pub program_hash: felt252,
    pub output_hash: felt252,
}

/// Given a proof, returns the output of the verifier.
#[cfg(not(or(feature: "blake_outputs_packing", feature: "poseidon_outputs_packing")))]
pub fn get_verification_output(proof: @CairoProof) -> VerificationOutput {
    // Note: the blake hash yields a 256-bit integer, the given program hash is taken modulo the
    // f252 prime to yield a felt.
    let program_hash = construct_f252(
        encode_and_hash_program_memory_section(*proof.claim.public_data.public_memory.program),
    );

    let mut output = array![];
    for entry in proof.claim.public_data.public_memory.output {
        let (_, val) = entry;
        output.append(construct_f252(BoxTrait::new(*val)));
    }

    VerificationOutput { program_hash, output }
}

#[cfg(or(feature: "blake_outputs_packing", feature: "poseidon_outputs_packing"))]
pub fn get_verification_output(proof: @CairoProof) -> VerificationOutput {
    // Note: the blake hash yields a 256-bit integer, the given program hash is taken modulo the
    // f252 prime to yield a felt.
    let program_hash = construct_f252(
        encode_and_hash_program_memory_section(*proof.claim.public_data.public_memory.program),
    );

    let output_hash = construct_f252(
        encode_and_hash_outputs_memory_section(*proof.claim.public_data.public_memory.output),
    );

    VerificationOutput { program_hash, output_hash }
}

pub fn verify_cairo(proof: CairoProof) {
    let CairoProof { claim, interaction_pow, interaction_claim, stark_proof, channel_salt } = proof;

    // Verify.
    let pcs_config = stark_proof.commitment_scheme_proof.config;

    verify_claim(@claim);

    let mut channel: Channel = Default::default();
    // Mix channel salt. Note that we first reduce it modulo `M31::P`, then cast it as QM31.
    let channel_salt_as_felt: QM31 = M31Trait::reduce_u32(channel_salt).into();
    channel.mix_felts([channel_salt_as_felt].span());

    pcs_config.mix_into(ref channel);
    let mut commitment_scheme = CommitmentSchemeVerifierImpl::new();

    // Unpack commitments.
    let commitments: @Box<[Hash; 4]> = stark_proof
        .commitment_scheme_proof
        .commitments
        .try_into()
        .unwrap();
    let [
        preprocessed_commitment,
        trace_commitment,
        interaction_trace_commitment,
        composition_commitment,
    ] =
        commitments
        .unbox();

    // Unpack claim.log_sizes().
    let log_sizes: @Box<[Span<u32>; 3]> = claim.log_sizes().span().try_into().unwrap();
    let [preprocessed_log_sizes, trace_log_sizes, interaction_trace_log_sizes] = log_sizes.unbox();

    let log_blowup_factor = pcs_config.fri_config.log_blowup_factor;
    // Preprocessed trace.
    let expected_preprocessed_root = preprocessed_root(log_blowup_factor);
    assert!(preprocessed_commitment == expected_preprocessed_root);
    commitment_scheme
        .commit(preprocessed_commitment, preprocessed_log_sizes, ref channel, log_blowup_factor);
    claim.mix_into(ref channel);

    commitment_scheme.commit(trace_commitment, trace_log_sizes, ref channel, log_blowup_factor);
    assert!(
        channel.verify_pow_nonce(INTERACTION_POW_BITS, interaction_pow),
        "{}",
        CairoVerificationError::InteractionProofOfWork,
    );
    channel.mix_u64(interaction_pow);

    let common_lookup_elements = LookupElementsImpl::draw(ref channel);
    assert!(
        lookup_sum(@claim, @common_lookup_elements, @interaction_claim).is_zero(),
        "{}",
        CairoVerificationError::InvalidLogupSum,
    );

    interaction_claim.mix_into(ref channel);
    commitment_scheme
        .commit(
            interaction_trace_commitment,
            interaction_trace_log_sizes,
            ref channel,
            log_blowup_factor,
        );

    let trace_lde_log_size = get_trace_lde_log_size(@commitment_scheme.trees);
    let trace_log_size = trace_lde_log_size - pcs_config.fri_config.log_blowup_factor;
    // The maximal constraint degree is 2, so the degree bound for the cairo air is the degree bound
    // of the trace plus 1.
    let cairo_air_log_degree_bound = trace_log_size + 1;
    let cairo_air = CairoAirNewImpl::new(@claim, @common_lookup_elements, @interaction_claim);

    verify(
        stark_proof,
        cairo_air,
        cairo_air_log_degree_bound,
        composition_commitment,
        commitment_scheme,
        ref channel,
        SECURITY_BITS,
    );
}

/// Verifies the claim of the Cairo proof.
///
/// # Panics
///
/// Panics if the claim is invalid.
fn verify_claim(claim: @CairoClaim) {
    let PublicData {
        public_memory: PublicMemory {
            program, public_segments, output: _output, safe_call_ids: _safe_call_ids,
            }, initial_state: CasmState {
            pc: initial_pc, ap: initial_ap, fp: initial_fp,
            }, final_state: CasmState {
            pc: final_pc, ap: final_ap, fp: final_fp,
        },
    } = claim.public_data;

    verify_builtins(
        claim.range_check_builtin,
        claim.range_check96_builtin,
        claim.bitwise_builtin,
        claim.add_mod_builtin,
        claim.mul_mod_builtin,
        claim.pedersen_builtin,
        claim.pedersen_builtin_narrow_windows,
        claim.poseidon_builtin,
        public_segments,
    );
    verify_program(*program, public_segments);

    let initial_pc: u32 = (*initial_pc).into();
    let initial_ap: u32 = (*initial_ap).into();
    let initial_fp: u32 = (*initial_fp).into();
    let final_pc: u32 = (*final_pc).into();
    let final_ap: u32 = (*final_ap).into();
    let final_fp: u32 = (*final_fp).into();

    assert!(initial_pc.is_one());
    assert!(initial_pc + 2 < initial_ap);
    assert!(initial_fp == final_fp);
    assert!(initial_fp == initial_ap);
    assert!(final_pc == 5);
    assert!(initial_ap <= final_ap);

    // Sanity check: ensure that the maximum address in the address_to_id component fits within a
    // 29-bit address space (i.e., is less than 2**29).
    // Higher addresses are not supported by components that assume 29-bit addresses.
    assert!(
        (*claim.memory_address_to_id).unwrap().log_size <= 29_u32 - LOG_MEMORY_ADDRESS_TO_ID_SPLIT,
    );

    // Count the number of uses of each relation.
    let mut relation_uses: RelationUsesDict = Default::default();
    claim.accumulate_relation_uses(ref relation_uses);

    // Check that the number of uses of the Opcodes relation <= 2^29. This bounds the number of
    // steps of the program by 2^29. The bound on the number of steps allows us to bound the
    // registers by P-(2^29-1)-1 at the start of each opcode, and show that ap/fp/pc + rel_imm does
    // not overflow P as follows:
    // The register pc - bounded by verify_instraction.
    // The register ap - add_ap_opcode or generic_opcode can set ap to at most 2^29-1, call_opcode
    // can increase ap by 2 (but starts with ap < 2^29 because [ap] = fp), and every other step can
    // increase ap by at most 1. Therefore, after (n_steps - 1) > 1 steps, ap can be at most
    // 2^29-1 + (n_steps-2).
    // The register fp - call_opcode and generic_opcode can set fp to ap + 2, where ap is either a
    // valid address, or range checked, and hence < 2^29. ret_opcode and generic_opcode can set fp
    // to a valid address, which is again < 2^29.
    let opcodes_uses = relation_uses.get('Opcodes');
    assert!(opcodes_uses <= pow2(29).into());

    // Check that no relation has more than P-1 uses.
    let squashed_dict = relation_uses.squash();
    let entries = squashed_dict.into_entries();
    for entry in entries {
        let (_relation_id, _first_uses, last_uses) = entry;
        assert!(last_uses < P_U32.into(), "A relation has more than P-1 uses");
    }
}

/// Checks that the ranges given by `segment_ranges` are valid given the claim.
///
/// `segment_ranges` specifies the memory segments for each builtin used by the Cairo program.
/// `builtins_claim` describes the address ranges that are verified by the builtins AIR.
///
/// This function ensures that all builtin segments are fully contained within the address ranges
/// verified by the builtins AIR.
/// The builtins keccak, ec_op, and ecdsa, are not supported, and therefore it's checked that their
/// segments are empty.
fn verify_builtins(
    range_check_128_builtin: @Option<crate::components::range_check_builtin::Claim>,
    range_check_96_builtin: @Option<crate::components::range_check96_builtin::Claim>,
    bitwise_builtin: @Option<crate::components::bitwise_builtin::Claim>,
    add_mod_builtin: @Option<crate::components::add_mod_builtin::Claim>,
    mul_mod_builtin: @Option<crate::components::mul_mod_builtin::Claim>,
    pedersen_builtin: @Option<crate::components::pedersen_builtin::Claim>,
    pedersen_builtin_narrow_windows: @Option<
        crate::components::pedersen_builtin_narrow_windows::Claim,
    >,
    poseidon_builtin: @Option<crate::components::poseidon_builtin::Claim>,
    segment_ranges: @PublicSegmentRanges,
) {
    assert!(pedersen_builtin_narrow_windows.is_none());

    let PublicSegmentRanges {
        ec_op: ec_op_segment_range,
        ecdsa: ecdsa_segment_range,
        keccak: keccak_segment_range,
        output: output_segment_range,
        pedersen: pedersen_segment_range,
        range_check_128: range_check_128_segment_range,
        range_check_96: range_check_96_segment_range,
        bitwise: bitwise_segment_range,
        add_mod: add_mod_segment_range,
        mul_mod: mul_mod_segment_range,
        poseidon: poseidon_segment_range,
    } = segment_ranges;

    // Check that non-supported builtins aren't used.
    assert!(ec_op_segment_range.start_ptr.value == ec_op_segment_range.stop_ptr.value);
    assert!(ecdsa_segment_range.start_ptr.value == ecdsa_segment_range.stop_ptr.value);
    assert!(keccak_segment_range.start_ptr.value == keccak_segment_range.stop_ptr.value);

    // Output builtin.
    assert!(output_segment_range.stop_ptr.value <= @pow2(31));
    assert!(output_segment_range.start_ptr.value <= output_segment_range.stop_ptr.value);

    // All other supported builtins.
    check_builtin(
        range_check_128_builtin
            .map(
                |
                    claim,
                | BuiltinClaim {
                    segment_start: claim.range_check_builtin_segment_start,
                    log_size: claim.log_size,
                },
            ),
        *range_check_128_segment_range,
        RANGE_CHECK_MEMORY_CELLS,
    );
    check_builtin(
        range_check_96_builtin
            .map(
                |
                    claim,
                | BuiltinClaim {
                    segment_start: claim.range_check96_builtin_segment_start,
                    log_size: claim.log_size,
                },
            ),
        *range_check_96_segment_range,
        RANGE_CHECK_MEMORY_CELLS,
    );
    check_builtin(
        bitwise_builtin
            .map(
                |
                    claim,
                | BuiltinClaim {
                    segment_start: claim.bitwise_builtin_segment_start, log_size: claim.log_size,
                },
            ),
        *bitwise_segment_range,
        BITWISE_MEMORY_CELLS,
    );
    check_builtin(
        add_mod_builtin
            .map(
                |
                    claim,
                | BuiltinClaim {
                    segment_start: claim.add_mod_builtin_segment_start, log_size: claim.log_size,
                },
            ),
        *add_mod_segment_range,
        ADD_MOD_MEMORY_CELLS,
    );
    check_builtin(
        mul_mod_builtin
            .map(
                |
                    claim,
                | BuiltinClaim {
                    segment_start: claim.mul_mod_builtin_segment_start, log_size: claim.log_size,
                },
            ),
        *mul_mod_segment_range,
        MUL_MOD_MEMORY_CELLS,
    );
    check_builtin(
        pedersen_builtin
            .map(
                |
                    claim,
                | BuiltinClaim {
                    segment_start: claim.pedersen_builtin_segment_start, log_size: claim.log_size,
                },
            ),
        *pedersen_segment_range,
        PEDERSEN_MEMORY_CELLS,
    );
    check_builtin(
        poseidon_builtin
            .map(
                |
                    claim,
                | BuiltinClaim {
                    segment_start: claim.poseidon_builtin_segment_start, log_size: claim.log_size,
                },
            ),
        *poseidon_segment_range,
        POSEIDON_MEMORY_CELLS,
    );
}

fn check_builtin(builtin_claim: Option<BuiltinClaim>, segment_range: SegmentRange, n_cells: usize) {
    let Some(BuiltinClaim { segment_start, log_size }) = builtin_claim else {
        // If the builtin is disabled it can't be used by the program.
        assert!(segment_range.is_empty());
        return;
    };

    let segment_end = segment_start + pow2(log_size) * n_cells;
    let start_ptr = segment_range.start_ptr.value;
    let stop_ptr = segment_range.stop_ptr.value;
    assert!((stop_ptr - start_ptr) % n_cells == 0);

    // Check that segment_start == start_ptr <= stop_ptr <= segment_end <= 2**31.
    assert!(start_ptr == segment_start);
    assert!(start_ptr <= stop_ptr);
    assert!(stop_ptr <= segment_end);
    assert!(segment_end <= pow2(31));
}

#[derive(Drop)]
struct BuiltinClaim {
    segment_start: u32,
    log_size: u32,
}

/// Check the program memory section is of the expected format.
/// The program should start with:
///     0. ap += N_BUILTINS
///     1. N_BUILTINS (immediate of instruction 0)
///     2. call rel ? (call to main)
///     3. main address (unrestricted immediate of instruction 2)
///     4. jmp rel 0 (to continue execution when step padding)
///     5. 0 (immediate of instruction 4)
fn verify_program(mut program: MemorySection, public_segments: @PublicSegmentRanges) {
    let [
        (_, program_value_0),
        (_, program_value_1),
        (_, program_value_2),
        (_, _program_value_3),
        (_, program_value_4),
        (_, program_value_5),
    ]: [PubMemoryValue; 6] =
        (*program
        .multi_pop_front()
        .unwrap())
        .unbox();

    // ap += N_BUILTINS. Two felts.
    let n_builtins = public_segments.present_segments().len();
    assert!(program_value_0 == [0x7fff7fff, 0x4078001, 0, 0, 0, 0, 0, 0]); // ap += N_BUILTINS.
    // Imm of last instruction (N_BUILTINS).
    assert!(program_value_1 == [n_builtins, 0, 0, 0, 0, 0, 0, 0]);

    // Instruction: call rel program[3]. Two felts. (call to main). No restrictions on the imm.
    assert!(program_value_2 == [0x80018000, 0x11048001, 0, 0, 0, 0, 0, 0]);

    // Instruction: jmp rel 0. Two felts.
    assert!(program_value_4 == [0x7fff7fff, 0x1078001, 0, 0, 0, 0, 0, 0]);
    // Imm of last instruction (jmp rel 0).
    assert!(program_value_5 == [0, 0, 0, 0, 0, 0, 0, 0]);
}


// TODO(alonf) Change all the obscure types and structs to a meaningful struct system for the
// memory.
#[derive(Clone, Debug, Serde, Copy, Drop)]
pub struct MemorySmallValue {
    pub id: u32,
    pub value: u32,
}

#[generate_trait]
impl MemorySmallValueImpl of MemorySmallValueTrait {
    fn mix_into(self: @MemorySmallValue, ref channel: Channel) {
        channel.mix_u64((*self.id).into());
        channel.mix_u64((*self.value).into());
    }
}

#[derive(Debug, Serde, Copy, Drop)]
pub struct SegmentRange {
    pub start_ptr: MemorySmallValue,
    pub stop_ptr: MemorySmallValue,
}

#[generate_trait]
impl SegmentRangeImpl of SegmentRangeTrait {
    fn is_empty(self: @SegmentRange) -> bool {
        self.start_ptr.value == self.stop_ptr.value
    }
    fn mix_into(self: @SegmentRange, ref channel: Channel) {
        self.start_ptr.mix_into(ref channel);
        self.stop_ptr.mix_into(ref channel);
    }
}

#[derive(Clone, Debug, Serde, Copy, Drop)]
pub struct PublicSegmentRanges {
    pub output: SegmentRange,
    pub pedersen: SegmentRange,
    pub range_check_128: SegmentRange,
    pub ecdsa: SegmentRange,
    pub bitwise: SegmentRange,
    pub ec_op: SegmentRange,
    pub keccak: SegmentRange,
    pub poseidon: SegmentRange,
    pub range_check_96: SegmentRange,
    pub add_mod: SegmentRange,
    pub mul_mod: SegmentRange,
}

#[generate_trait]
impl PublicSegmentRangesImpl of PublicSegmentRangesTrait {
    fn mix_into(self: @PublicSegmentRanges, ref channel: Channel) {
        for segment in self.present_segments() {
            segment.mix_into(ref channel);
        }
    }

    fn present_segments(self: @PublicSegmentRanges) -> Array<@SegmentRange> {
        let mut segments = array![];
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
        } = self;

        segments.append(output);
        segments.append(pedersen);
        segments.append(range_check_128);
        segments.append(ecdsa);
        segments.append(bitwise);
        segments.append(ec_op);
        segments.append(keccak);
        segments.append(poseidon);
        segments.append(range_check_96);
        segments.append(add_mod);
        segments.append(mul_mod);
        segments
    }
}

#[derive(Copy, Serde, Drop)]
pub struct PublicMemory {
    pub program: MemorySection,
    pub public_segments: PublicSegmentRanges,
    pub output: MemorySection,
    pub safe_call_ids: [u32; 2],
}


#[generate_trait]
pub impl PublicMemoryImpl of PublicMemoryTrait {
    fn get_entries(
        self: @PublicMemory, initial_pc: u32, initial_ap: u32, final_ap: u32,
    ) -> PublicMemoryEntries {
        let mut pub_memory_entries = PublicMemoryEntriesTrait::empty();

        // The program is loaded to `initial_pc`.
        pub_memory_entries.add_memory_section(self.program, initial_pc);
        // Output was written to `final_ap`.
        pub_memory_entries.add_memory_section(self.output, final_ap);

        // The safe call area should be [initial_fp, 0] and initial_fp should be the same as
        // initial_ap.
        let [safe_call_id0, safe_call_id1] = self.safe_call_ids;
        pub_memory_entries
            .add_memory_entry(
                PublicMemoryEntry {
                    address: initial_ap - 2,
                    id: *safe_call_id0,
                    value: [initial_ap, 0, 0, 0, 0, 0, 0, 0],
                },
            );
        pub_memory_entries
            .add_memory_entry(
                PublicMemoryEntry {
                    address: initial_ap - 1, id: *safe_call_id1, value: [0, 0, 0, 0, 0, 0, 0, 0],
                },
            );

        let present_segments = self.public_segments.present_segments();
        let n_segments = present_segments.len();

        let mut args_ptr = initial_ap;
        let mut return_values_ptr = final_ap - n_segments;
        for segment in present_segments {
            pub_memory_entries
                .add_memory_entry(
                    PublicMemoryEntry {
                        address: args_ptr,
                        id: *segment.start_ptr.id,
                        value: [*segment.start_ptr.value, 0, 0, 0, 0, 0, 0, 0],
                    },
                );
            args_ptr += 1;

            pub_memory_entries
                .add_memory_entry(
                    PublicMemoryEntry {
                        address: return_values_ptr,
                        id: *segment.stop_ptr.id,
                        value: [*segment.stop_ptr.value, 0, 0, 0, 0, 0, 0, 0],
                    },
                );
            return_values_ptr += 1;
        }

        pub_memory_entries
    }
    fn mix_into(self: @PublicMemory, ref channel: Channel) {
        let PublicMemory { program, public_segments, output, safe_call_ids } = self;

        // Mix program memory section.
        channel.mix_memory_section(*program);

        // Mix public segments.
        public_segments.mix_into(ref channel);

        // Mix output memory section.
        channel.mix_memory_section(*output);

        // Mix safe_call_ids.
        let [safe_call_id_0, safe_call_id_1] = safe_call_ids;
        channel.mix_u64((*safe_call_id_0).into());
        channel.mix_u64((*safe_call_id_1).into());
    }
}


mod combine;


#[derive(Clone, Drop, Serde)]
pub struct PublicData {
    pub public_memory: PublicMemory,
    pub initial_state: CasmState,
    pub final_state: CasmState,
}

#[generate_trait]
impl PublicDataImpl of PublicDataTrait {
    fn logup_sum(self: @PublicData, common_lookup_elements: @crate::CommonLookupElements) -> QM31 {
        let mut sum = Zero::zero();

        let public_memory_entries = self
            .public_memory
            .get_entries(
                initial_pc: (*self.initial_state.pc).into(),
                initial_ap: (*self.initial_state.ap).into(),
                final_ap: (*self.final_state.ap).into(),
            );
        sum += sum_public_memory_entries(public_memory_entries, common_lookup_elements);

        // Yield initial state and use the final.
        let CasmState { pc, ap, fp } = *self.final_state;
        sum += common_lookup_elements.combine([OPCODES_RELATION_ID, pc, ap, fp].span()).inverse();
        let CasmState { pc, ap, fp } = *self.initial_state;
        sum -= common_lookup_elements.combine([OPCODES_RELATION_ID, pc, ap, fp].span()).inverse();

        sum
    }

    fn mix_into(self: @PublicData, ref channel: Channel) {
        let (public_claim, output_claim, program_claim) = self.pack_into_u32s();
        channel.mix_felts(pack_into_qm31s(public_claim));
        channel.mix_felts(pack_into_qm31s(output_claim));
        channel.mix_felts(pack_into_qm31s(program_claim));
    }

    /// Converts public data to [u32], where each u32 is at most 2^31 - 1.
    /// Returns the output and program values separately.
    fn pack_into_u32s(self: @PublicData) -> (Span<u32>, Span<u32>, Span<u32>) {
        let PublicData {
            initial_state: CasmState {
                pc: initial_pc, ap: initial_ap, fp: initial_fp,
                }, final_state: CasmState {
                pc: final_pc, ap: final_ap, fp: final_fp,
                }, public_memory: PublicMemory {
                public_segments, output, safe_call_ids, program,
            },
        } = self;

        let mut public_claim = array![];
        public_claim.append((*initial_pc).into());
        public_claim.append((*initial_ap).into());
        public_claim.append((*initial_fp).into());
        public_claim.append((*final_pc).into());
        public_claim.append((*final_ap).into());
        public_claim.append((*final_fp).into());
        let PublicSegmentRanges {
            output: output_ranges,
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
        } = public_segments;
        public_claim
            .extend(
                array![
                    *output_ranges.start_ptr.id, *output_ranges.start_ptr.value,
                    *output_ranges.stop_ptr.id, *output_ranges.stop_ptr.value,
                ],
            );
        public_claim
            .extend(
                array![
                    *pedersen.start_ptr.id, *pedersen.start_ptr.value, *pedersen.stop_ptr.id,
                    *pedersen.stop_ptr.value,
                ],
            );
        public_claim
            .extend(
                array![
                    *range_check_128.start_ptr.id, *range_check_128.start_ptr.value,
                    *range_check_128.stop_ptr.id, *range_check_128.stop_ptr.value,
                ],
            );
        public_claim
            .extend(
                array![
                    *ecdsa.start_ptr.id, *ecdsa.start_ptr.value, *ecdsa.stop_ptr.id,
                    *ecdsa.stop_ptr.value,
                ],
            );
        public_claim
            .extend(
                array![
                    *bitwise.start_ptr.id, *bitwise.start_ptr.value, *bitwise.stop_ptr.id,
                    *bitwise.stop_ptr.value,
                ],
            );
        public_claim
            .extend(
                array![
                    *ec_op.start_ptr.id, *ec_op.start_ptr.value, *ec_op.stop_ptr.id,
                    *ec_op.stop_ptr.value,
                ],
            );
        public_claim
            .extend(
                array![
                    *keccak.start_ptr.id, *keccak.start_ptr.value, *keccak.stop_ptr.id,
                    *keccak.stop_ptr.value,
                ],
            );
        public_claim
            .extend(
                array![
                    *poseidon.start_ptr.id, *poseidon.start_ptr.value, *poseidon.stop_ptr.id,
                    *poseidon.stop_ptr.value,
                ],
            );
        public_claim
            .extend(
                array![
                    *range_check_96.start_ptr.id, *range_check_96.start_ptr.value,
                    *range_check_96.stop_ptr.id, *range_check_96.stop_ptr.value,
                ],
            );
        public_claim
            .extend(
                array![
                    *add_mod.start_ptr.id, *add_mod.start_ptr.value, *add_mod.stop_ptr.id,
                    *add_mod.stop_ptr.value,
                ],
            );
        public_claim
            .extend(
                array![
                    *mul_mod.start_ptr.id, *mul_mod.start_ptr.value, *mul_mod.stop_ptr.id,
                    *mul_mod.stop_ptr.value,
                ],
            );
        let arr: Array<u32> = safe_call_ids.into_iter().map(|x| *x).collect();
        public_claim.extend(arr);
        for (id, _) in output {
            public_claim.append(*id);
        }
        for (id, _) in program {
            public_claim.append(*id);
        }

        // Collect output values.
        let mut output_claim = array![];
        for (_, value) in output {
            let fixed_arr: [u32; 8] = *value;
            let new_value: [u32; N_M31_IN_FELT252] = split(fixed_arr);
            let arr: Array<u32> = new_value.span().into_iter().map(|x| *x).collect();
            output_claim.extend(arr);
        }

        // Collect program values.
        let mut program_claim = array![];
        for (_, value) in program {
            let fixed_arr: [u32; 8] = *value;
            let new_value: [u32; N_M31_IN_FELT252] = split(fixed_arr);
            let arr: Array<u32> = new_value.span().into_iter().map(|x| *x).collect();
            program_claim.extend(arr);
        }

        (public_claim.span(), output_claim.span(), program_claim.span())
    }
}

#[cfg(feature: "qm31_opcode")]
fn sum_public_memory_entries(
    pub_memory_entries: PublicMemoryEntries, common_lookup_elements: @crate::CommonLookupElements,
) -> QM31 {
    let mut sum = Zero::zero();
    let common_z = *common_lookup_elements.z;
    let common_alpha = *common_lookup_elements.alpha_powers[1];
    let common_alpha2 = *common_lookup_elements.alpha_powers[2];

    for PublicMemoryEntry { address, id, value } in pub_memory_entries {
        let addr_m31: M31 = address.try_into().unwrap();
        let addr_qm31 = addr_m31.into();
        let id_m31: M31 = id.try_into().unwrap();
        let id_qm31 = id_m31.into();
        let addr_to_id = (MEMORY_ADDRESS_TO_ID_RELATION_ID.into()
            + addr_qm31 * common_alpha
            + id_qm31 * common_alpha2
            - common_z)
            .inverse();

        // Use handwritten implementation of combine_id_to_value to improve performance.
        let mut combine_sum = combine::combine_felt252(value, common_alpha);
        combine_sum = combine_sum * common_alpha
            + id_m31.into() * common_alpha
            + MEMORY_ID_TO_VALUE_RELATION_ID.into()
            - common_z;
        let id_to_value = combine_sum.inverse();

        sum += addr_to_id + id_to_value;
    }

    sum
}

#[cfg(not(feature: "qm31_opcode"))]
// An alternative implementation that uses batch inverse, for the case that we don't have an opcode
// for it.
fn sum_public_memory_entries(
    pub_memory_entries: PublicMemoryEntries, common_lookup_elements: @CommonLookupElements,
) -> QM31 {
    // Gather values to be inverted and summed.
    let mut values: Array<QM31> = array![];

    let mut alpha_powers = common_lookup_elements.alpha_powers.span();
    // Remove the first two elements (1 and alpha), so combine_felt252 below computes
    // sum_{i} value[i] * alpha**(i+2) as required for the id_to_value relation.
    let _ = alpha_powers.pop_front();
    let _ = alpha_powers.pop_front();
    let mut packed_alpha_powers: Span<PackedUnreducedQM31> = alpha_powers
        .into_iter()
        .map(|alpha| -> PackedUnreducedQM31 {
            (*alpha).into()
        })
        .collect::<Array<_>>()
        .span();
    let id_to_value_alpha_powers: Box<[PackedUnreducedQM31; 28]> = *(packed_alpha_powers
        .multi_pop_front()
        .unwrap());

    let common_alpha: PackedUnreducedQM31 = (*common_lookup_elements.alpha).into();
    let common_alpha2: PackedUnreducedQM31 = (*common_lookup_elements.alpha
        * *common_lookup_elements.alpha)
        .into();
    let minus_common_z: PackedUnreducedQM31 = PackedUnreducedQM31Trait::large_zero()
        - (*common_lookup_elements.z).into();

    for PublicMemoryEntry { address, id, value } in pub_memory_entries {
        let addr_m31: M31 = address.try_into().unwrap();
        let id_m31: M31 = id.try_into().unwrap();
        let addr_to_id: PackedUnreducedQM31 = (minus_common_z
            + common_alpha2.mul_m31(id_m31)
            + common_alpha.mul_m31(addr_m31))
            .add_m31(MEMORY_ADDRESS_TO_ID_RELATION_ID);
        values.append(addr_to_id.reduce());

        // Use handwritten implementation of combine_id_to_value to improve performance.
        let combined_limbs = combine::combine_felt252(value, id_to_value_alpha_powers);
        let id_to_value = (minus_common_z + combined_limbs + common_alpha.mul_m31(id_m31))
            .add_m31(MEMORY_ID_TO_VALUE_RELATION_ID);
        values.append(id_to_value.reduce());
    }

    utils::sum_inverses_qm31(@values)
}


#[derive(Drop, Serde, Copy)]
pub struct CasmState {
    pub pc: M31,
    pub ap: M31,
    pub fp: M31,
}

#[generate_trait]
impl CasmStateImpl of CasmStateTrait {
    fn mix_into(self: @CasmState, ref channel: Channel) {
        let pc_u32: u32 = (*self.pc).into();
        let ap_u32: u32 = (*self.ap).into();
        let fp_u32: u32 = (*self.fp).into();
        channel.mix_u64(pc_u32.into());
        channel.mix_u64(ap_u32.into());
        channel.mix_u64(fp_u32.into());
    }
}

pub fn accumulate_relation_uses(
    ref relation_uses: RelationUsesDict, relation_uses_per_row: Span<RelationUse>, log_size: u32,
) {
    let component_size = pow2(log_size);
    for relation_use in relation_uses_per_row {
        let (relation_id, uses) = *relation_use;
        let (entry, prev_uses) = relation_uses.entry(relation_id);
        relation_uses = entry.finalize(prev_uses + uses.into() * component_size.into());
    }
}

#[derive(Drop, Debug)]
pub enum CairoVerificationError {
    InteractionProofOfWork,
    InvalidLogupSum,
}

impl CairoVerificationErrorDisplay of core::fmt::Display<CairoVerificationError> {
    fn fmt(
        self: @CairoVerificationError, ref f: core::fmt::Formatter,
    ) -> Result<(), core::fmt::Error> {
        match self {
            CairoVerificationError::InteractionProofOfWork => write!(
                f, "Interaction Proof Of Work",
            ),
            CairoVerificationError::InvalidLogupSum => write!(f, "Logup sum is not zero"),
        }
    }
}
