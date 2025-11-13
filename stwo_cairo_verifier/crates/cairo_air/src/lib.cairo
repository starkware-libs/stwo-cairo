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
use core::num::traits::Zero;
use core::num::traits::one::One;
use stwo_constraint_framework::{LookupElements, LookupElementsImpl, PreprocessedMaskValuesImpl};
use stwo_verifier_core::channel::{Channel, ChannelImpl, ChannelTrait};
use stwo_verifier_core::fields::Invertible;
#[cfg(not(feature: "qm31_opcode"))]
use stwo_verifier_core::fields::m31::{AddM31Trait, MulByM31Trait};
use stwo_verifier_core::fields::m31::{M31, P_U32};
#[cfg(not(feature: "qm31_opcode"))]
use stwo_verifier_core::fields::qm31::{PackedUnreducedQM31, PackedUnreducedQM31Trait};
use stwo_verifier_core::fields::qm31::{QM31, qm31_const};
use stwo_verifier_core::pcs::PcsConfigTrait;
use stwo_verifier_core::pcs::verifier::{CommitmentSchemeVerifierImpl, get_trace_lde_log_size};
use stwo_verifier_core::utils::{ArrayImpl, OptionImpl, pow2};
use stwo_verifier_core::verifier::{StarkProof, verify};
use stwo_verifier_utils::{MemorySection, PubMemoryValue, construct_f252};
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

pub mod pedersen;
use pedersen::PedersenContextInteractionClaimImpl;

pub mod poseidon;
use poseidon::PoseidonContextInteractionClaimImpl;

pub mod blake;
use blake::BlakeContextInteractionClaimImpl;

pub mod builtins;
use builtins::{BuiltinsClaim, BuiltinsInteractionClaimImpl};

pub mod opcodes;
use opcodes::OpcodeInteractionClaimImpl;

pub mod range_checks;
use range_checks::RangeChecksInteractionClaimImpl;
pub use range_checks::range_check_elements::*;

pub mod preprocessed_columns;
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
    /// Optional salt used in the channel initialization.
    pub channel_salt: Option<u64>,
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
    if let Some(salt) = channel_salt {
        channel.mix_u64(salt);
    }
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

    let interaction_elements = CairoInteractionElementsImpl::draw(ref channel);
    assert!(
        lookup_sum(@claim, @interaction_elements, @interaction_claim).is_zero(),
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
    let cairo_air = CairoAirNewImpl::new(
        @claim, @interaction_elements, @interaction_claim, cairo_air_log_degree_bound,
    );
    verify(
        cairo_air,
        ref channel,
        stark_proof,
        commitment_scheme,
        SECURITY_BITS,
        composition_commitment,
    );
}


pub fn lookup_sum(
    claim: @CairoClaim,
    elements: @CairoInteractionElements,
    interaction_claim: @CairoInteractionClaim,
) -> QM31 {
    let mut sum = claim.public_data.logup_sum(elements);
    // If the table is padded, take the sum of the non-padded values.
    // Otherwise, the claimed_sum is the total_sum.
    // TODO(Ohad): hide this logic behind `InteractionClaim`, and only sum here.

    // TODO(Andrew): double check this is correct order.
    let CairoInteractionClaim {
        opcodes,
        verify_instruction,
        blake_context,
        builtins,
        pedersen_context,
        poseidon_context,
        memory_address_to_id,
        memory_id_to_value,
        range_checks,
        verify_bitwise_xor_4,
        verify_bitwise_xor_7,
        verify_bitwise_xor_8,
        verify_bitwise_xor_8_b,
        verify_bitwise_xor_9,
    } = interaction_claim;

    sum += opcodes.sum();
    sum += *verify_instruction.claimed_sum;
    sum += blake_context.sum();
    sum += builtins.sum();
    sum += pedersen_context.sum();
    sum += poseidon_context.sum();
    sum += *memory_address_to_id.claimed_sum;
    sum += memory_id_to_value.sum();
    sum += range_checks.sum();
    sum += *verify_bitwise_xor_4.claimed_sum;
    sum += *verify_bitwise_xor_7.claimed_sum;
    sum += *verify_bitwise_xor_8.claimed_sum;
    sum += *verify_bitwise_xor_8_b.claimed_sum;
    sum += *verify_bitwise_xor_9.claimed_sum;
    sum
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

    verify_builtins(claim.builtins, public_segments);
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
    // 27-bit address space (i.e., is less than 2**27).
    // Higher addresses are not supported by components that assume 27-bit addresses.
    assert!(*claim.memory_address_to_id.log_size <= 27_u32 - LOG_MEMORY_ADDRESS_TO_ID_SPLIT);

    // Count the number of uses of each relation.
    let mut relation_uses: RelationUsesDict = Default::default();
    claim.accumulate_relation_uses(ref relation_uses);

    // Make sure ap does not overflow P:
    // Check that the number of uses of the Opcodes relation is leq than 2^29. This bounds the
    // number of steps of the program by 2^29. An add_ap use can increase ap *to* at most 2^27-1,
    // and every other step can increase ap by at most 2. Therefore the most ap can increase to with
    // n_steps steps is 2^27-1 + 2 * (n_steps-1). This is less than P if n_steps <= 2^29.
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
fn verify_builtins(builtins_claim: @BuiltinsClaim, segment_ranges: @PublicSegmentRanges) {
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
    let BuiltinsClaim {
        range_check_128_builtin,
        range_check_96_builtin,
        bitwise_builtin,
        add_mod_builtin,
        mul_mod_builtin,
        pedersen_builtin,
        poseidon_builtin,
    } = builtins_claim;
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

#[derive(Serde, Drop)]
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


#[derive(Drop, Serde)]
pub struct PublicData {
    pub public_memory: PublicMemory,
    pub initial_state: CasmState,
    pub final_state: CasmState,
}

#[generate_trait]
impl PublicDataImpl of PublicDataTrait {
    fn logup_sum(self: @PublicData, lookup_elements: @CairoInteractionElements) -> QM31 {
        let mut sum = Zero::zero();

        let public_memory_entries = self
            .public_memory
            .get_entries(
                initial_pc: (*self.initial_state.pc).into(),
                initial_ap: (*self.initial_state.ap).into(),
                final_ap: (*self.final_state.ap).into(),
            );
        sum += sum_public_memory_entries(public_memory_entries, lookup_elements);

        // Yield initial state and use the final.
        let CasmState { pc, ap, fp } = *self.final_state;
        sum += lookup_elements.opcodes.combine([pc, ap, fp]).inverse();
        let CasmState { pc, ap, fp } = *self.initial_state;
        sum -= lookup_elements.opcodes.combine([pc, ap, fp]).inverse();

        sum
    }

    fn mix_into(self: @PublicData, ref channel: Channel) {
        self.public_memory.mix_into(ref channel);
        self.initial_state.mix_into(ref channel);
        self.final_state.mix_into(ref channel);
    }
}

#[cfg(feature: "qm31_opcode")]
fn sum_public_memory_entries(
    pub_memory_entries: PublicMemoryEntries, lookup_elements: @CairoInteractionElements,
) -> QM31 {
    let mut sum = Zero::zero();
    let id_to_value_alpha = *lookup_elements.memory_id_to_value.alpha;
    let id_to_value_z = *lookup_elements.memory_id_to_value.z;
    let addr_to_id_alpha = *lookup_elements.memory_address_to_id.alpha;
    let addr_to_id_z = *lookup_elements.memory_address_to_id.z;

    for PublicMemoryEntry { address, id, value } in pub_memory_entries {
        let addr_m31: M31 = address.try_into().unwrap();
        let addr_qm31 = addr_m31.into();
        let id_m31: M31 = id.try_into().unwrap();
        let id_qm31 = id_m31.into();
        let addr_to_id = (addr_qm31 + id_qm31 * addr_to_id_alpha - addr_to_id_z).inverse();

        // Use handwritten implementation of combine_id_to_value to improve performance.
        let mut combine_sum = combine::combine_felt252(value, id_to_value_alpha);
        combine_sum = combine_sum + id_m31.into() - id_to_value_z;
        let id_to_value = combine_sum.inverse();

        sum += addr_to_id + id_to_value;
    }

    sum
}

#[cfg(not(feature: "qm31_opcode"))]
// An alternative implementation that uses batch inverse, for the case that we don't have an opcode
// for it.
fn sum_public_memory_entries(
    pub_memory_entries: PublicMemoryEntries, lookup_elements: @CairoInteractionElements,
) -> QM31 {
    // Gather values to be inverted and summed.
    let mut values: Array<QM31> = array![];

    let mut alpha_powers = lookup_elements.memory_id_to_value.alpha_powers.span();
    // Remove the first element, which is 1.
    let _ = alpha_powers.pop_front();
    let packed_alpha_powers: Array<PackedUnreducedQM31> = alpha_powers
        .into_iter()
        .map(|alpha| -> PackedUnreducedQM31 {
            (*alpha).into()
        })
        .collect();
    let id_to_value_alpha_powers: Box<[PackedUnreducedQM31; 28]> = *(packed_alpha_powers
        .span()
        .try_into()
        .unwrap());

    let addr_to_id_alpha: PackedUnreducedQM31 = (*lookup_elements.memory_address_to_id.alpha)
        .into();
    let minus_id_to_value_z: PackedUnreducedQM31 = PackedUnreducedQM31Trait::large_zero()
        - (*lookup_elements.memory_id_to_value.z).into();
    let minus_addr_to_id_z: PackedUnreducedQM31 = PackedUnreducedQM31Trait::large_zero()
        - (*lookup_elements.memory_address_to_id.z).into();

    for PublicMemoryEntry { address, id, value } in pub_memory_entries {
        let addr_m31: M31 = address.try_into().unwrap();
        let id_m31: M31 = id.try_into().unwrap();
        let addr_to_id: PackedUnreducedQM31 = minus_addr_to_id_z
            + addr_to_id_alpha.mul_m31(id_m31).add_m31(addr_m31);
        values.append(addr_to_id.reduce());

        // Use handwritten implementation of combine_id_to_value to improve performance.
        let combined_limbs = combine::combine_felt252(value, id_to_value_alpha_powers);
        let id_to_value = minus_id_to_value_z + combined_limbs.add_m31(id_m31);
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

