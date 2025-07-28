use components::memory_address_to_id::{
    InteractionClaimImpl as MemoryAddressToIdInteractionClaimImpl, LOG_MEMORY_ADDRESS_TO_ID_SPLIT,
    MEMORY_ADDRESS_TO_ID_SPLIT,
};
use components::memory_id_to_big::InteractionClaimImpl as MemoryIdToBigInteractionClaimImpl;

#[cfg(feature: "poseidon252_verifier")]
mod poseidon252_verifier_uses {
    pub use core::poseidon::poseidon_hash_span;
    pub use stwo_cairo_air::utils::deconstruct_f252;
}
#[cfg(feature: "poseidon252_verifier")]
use poseidon252_verifier_uses::*;


#[cfg(not(feature: "poseidon252_verifier"))]
mod blake2s_verifier_uses {
    pub use core::blake::{blake2s_compress, blake2s_finalize};
    pub use stwo_verifier_core::channel::blake2s::BLAKE2S_256_INITIAL_STATE;
}
#[cfg(not(feature: "poseidon252_verifier"))]
use blake2s_verifier_uses::*;
use core::box::BoxImpl;
use core::dict::{Felt252Dict, Felt252DictEntryTrait, Felt252DictTrait, SquashedFelt252DictTrait};
use core::num::traits::Zero;
use core::num::traits::one::One;
use stwo_cairo_air::utils::construct_f252;
use stwo_constraint_framework::{
    LookupElements, LookupElementsImpl, PreprocessedColumnImpl, PreprocessedColumnKey,
    PreprocessedColumnTrait, PreprocessedMaskValuesImpl,
};
use stwo_verifier_core::channel::{Channel, ChannelImpl, ChannelTrait};
use stwo_verifier_core::fields::Invertible;
use stwo_verifier_core::fields::m31::{M31, P_U32};
use stwo_verifier_core::fields::qm31::{QM31, QM31Trait, qm31_const};
use stwo_verifier_core::pcs::PcsConfigTrait;
use stwo_verifier_core::pcs::verifier::CommitmentSchemeVerifierImpl;
use stwo_verifier_core::utils::{ArrayImpl, OptionImpl, pow2};
#[cfg(feature: "outputs_packing")]
use stwo_verifier_core::vcs::blake2s_hasher::Blake2sHash;
use stwo_verifier_core::verifier::{StarkProof, verify};


pub mod cairo_air;
use cairo_air::*;

pub mod cairo_component;
pub mod components;


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
}

/// The output of a verification.
#[cfg(not(feature: "outputs_packing"))]
#[derive(Drop, Serde)]
pub struct VerificationOutput {
    pub program_hash: felt252,
    pub output: Array<felt252>,
}
#[cfg(feature: "outputs_packing")]
#[derive(Drop, Serde)]
pub struct VerificationOutput {
    pub program_hash: felt252,
    pub output: Blake2sHash,
}

/// Given a proof, returns the output of the verifier.
#[cfg(not(feature: "outputs_packing"))]
pub fn get_verification_output(proof: @CairoProof) -> VerificationOutput {
    // Note: the blake hash yields a 256-bit integer, the given program hash is taken modulo the
    // f252 prime to yield a felt.
    let program_hash = construct_f252(
        hash_memory_section(proof.claim.public_data.public_memory.program),
    );

    let mut output = array![];
    for entry in proof.claim.public_data.public_memory.output {
        let (_, val) = entry;
        output.append(construct_f252(BoxTrait::new(*val)));
    }

    VerificationOutput { program_hash, output }
}

#[cfg(feature: "outputs_packing")]
pub fn get_verification_output(proof: @CairoProof) -> VerificationOutput {
    // Note: the blake hash yields a 256-bit integer, the given program hash is taken modulo the
    // f252 prime to yield a felt.
    let program_hash = construct_f252(
        hash_memory_section(proof.claim.public_data.public_memory.program),
    );

    let output_hash = Blake2sHash {
        hash: hash_memory_section(proof.claim.public_data.public_memory.output),
    };

    VerificationOutput { program_hash, output: output_hash }
}

pub fn verify_cairo(proof: CairoProof) {
    let CairoProof { claim, interaction_pow, interaction_claim, stark_proof } = proof;

    // Verify.
    let pcs_config = stark_proof.commitment_scheme_proof.config;

    verify_claim(@claim);

    let mut channel: Channel = Default::default();
    pcs_config.mix_into(ref channel);
    let mut commitment_scheme = CommitmentSchemeVerifierImpl::new(pcs_config);

    let log_sizes = claim.log_sizes();

    // Preprocessed trace.
    let expected_preprocessed_root = preprocessed_root(pcs_config.fri_config.log_blowup_factor);
    let preprocessed_root = stark_proof.commitment_scheme_proof.commitments[0].clone();
    assert!(preprocessed_root == expected_preprocessed_root);
    commitment_scheme.commit(preprocessed_root, *log_sizes[0], ref channel);
    claim.mix_into(ref channel);

    commitment_scheme
        .commit(
            stark_proof.commitment_scheme_proof.commitments[1].clone(), *log_sizes[1], ref channel,
        );
    assert!(
        channel.mix_and_check_pow_nonce(INTERACTION_POW_BITS, interaction_pow),
        "{}",
        CairoVerificationError::InteractionProofOfWork,
    );

    let interaction_elements = CairoInteractionElementsImpl::draw(ref channel);
    assert!(
        lookup_sum(@claim, @interaction_elements, @interaction_claim).is_zero(),
        "{}",
        CairoVerificationError::InvalidLogupSum,
    );

    interaction_claim.mix_into(ref channel);
    commitment_scheme
        .commit(
            stark_proof.commitment_scheme_proof.commitments[2].clone(), *log_sizes[2], ref channel,
        );

    let cairo_air = CairoAirNewImpl::new(@claim, @interaction_elements, @interaction_claim);
    verify(cairo_air, ref channel, stark_proof, commitment_scheme, SECURITY_BITS);
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
    sum += interaction_claim.opcodes.sum();
    sum += *interaction_claim.verify_instruction.claimed_sum;
    sum += interaction_claim.blake_context.sum();
    sum += interaction_claim.builtins.sum();
    sum += interaction_claim.pedersen_context.sum();
    sum += interaction_claim.poseidon_context.sum();
    sum += *interaction_claim.memory_address_to_id.claimed_sum;
    sum += interaction_claim.memory_id_to_value.sum();
    sum += interaction_claim.range_checks.sum();
    sum += *interaction_claim.verify_bitwise_xor_4.claimed_sum;
    sum += *interaction_claim.verify_bitwise_xor_7.claimed_sum;
    sum += *interaction_claim.verify_bitwise_xor_8.claimed_sum;
    sum += *interaction_claim.verify_bitwise_xor_9.claimed_sum;
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
            program, public_segments, output: _output, safe_call: _safe_call,
            }, initial_state: CasmState {
            pc: initial_pc, ap: initial_ap, fp: initial_fp,
            }, final_state: CasmState {
            pc: final_pc, ap: final_ap, fp: final_fp,
        },
    } = claim.public_data;

    verify_builtins(claim.builtins, public_segments);

    // Currently only bootloader context is supported.
    // TODO: Change that fact post MVP.
    public_segments.assert_bootloader_context();
    verify_program(program, public_segments);

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

    // When using address_to_id relation, it is assumed that address < 2^27.
    // To verify that, one needs to check that the size of the address_to_id component <=
    // 2^(27 - log2(MEMORY_ADDRESS_TO_ID_SPLIT)), beacuse the component is split to
    // MEMORY_ADDRESS_TO_ID_SPLIT addresses in each row of the component.
    assert!(pow2(LOG_MEMORY_ADDRESS_TO_ID_SPLIT) == MEMORY_ADDRESS_TO_ID_SPLIT);
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

    // Large value IDs reside in [2^30..P).
    // Check that IDs in (ID -> Value) do not overflow P.
    let mut n_unique_large_values = 0;
    for log_size in claim.memory_id_to_value.big_log_sizes.span() {
        n_unique_large_values += pow2(*log_size);
    }
    let large_id_offset = pow2(30);
    let largest_id = n_unique_large_values + large_id_offset - 1;
    assert!(largest_id < P_U32);
}

fn verify_builtins(builtins_claim: @BuiltinsClaim, segment_ranges: @PublicSegmentRanges) {
    // Check that non-supported builtins aren't used.
    if let Some(ec_op) = segment_ranges.ec_op {
        assert!(ec_op.start_ptr.value == ec_op.stop_ptr.value);
    }
    if let Some(ecdsa) = segment_ranges.ecdsa {
        assert!(ecdsa.start_ptr.value == ecdsa.stop_ptr.value);
    }
    if let Some(keccak) = segment_ranges.keccak {
        assert!(keccak.start_ptr.value == keccak.stop_ptr.value);
    }

    // Output builtin.
    assert!(segment_ranges.output.stop_ptr.value <= @pow2(31));
    assert!(segment_ranges.output.start_ptr.value <= segment_ranges.output.stop_ptr.value);

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
        *segment_ranges.range_check_128,
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
        *segment_ranges.range_check_96,
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
        *segment_ranges.bitwise,
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
        *segment_ranges.add_mod,
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
        *segment_ranges.mul_mod,
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
        *segment_ranges.pedersen,
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
        *segment_ranges.poseidon,
        POSEIDON_MEMORY_CELLS,
    );
}

fn check_builtin(
    builtin_claim: Option<BuiltinClaim>, segment_range: Option<SegmentRange>, n_cells: usize,
) {
    let segment_range = match segment_range {
        None => { return; },
        Some(segment_range) => {
            if segment_range.is_empty() {
                return;
            }
            segment_range
        },
    };

    let BuiltinClaim { segment_start, log_size } = builtin_claim.unwrap();

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

fn verify_program(program: @MemorySection, public_segments: @PublicSegmentRanges) {
    let (_, program_value_0) = program[0];
    let (_, program_value_1) = program[1];
    let (_, program_value_2) = program[2];
    let (_, program_value_4) = program[4];
    let (_, program_value_5) = program[5];

    let n_builtins = public_segments.present_segments().len();
    assert!(program_value_0 == @[0x7fff7fff, 0x4078001, 0, 0, 0, 0, 0, 0]); // ap += N_BUILTINS.
    assert!(
        program_value_1 == @[n_builtins, 0, 0, 0, 0, 0, 0, 0],
    ); // Imm of last instruction (N_BUILTINS).
    assert!(
        program_value_2 == @[0x80018000, 0x11048001, 0, 0, 0, 0, 0, 0],
    ); // Instruction: call rel ?
    assert!(
        program_value_4 == @[0x7fff7fff, 0x1078001, 0, 0, 0, 0, 0, 0],
    ); // Instruction: jmp rel 0.
    assert!(program_value_5 == @[0, 0, 0, 0, 0, 0, 0, 0]); // Imm of last instruction (jmp rel 0).
}


// TODO(alonf) Change all the obscure types and structs to a meaninful struct system for the memory.
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

// TODO(alonf): Change this into a struct. Remove Pub prefix.
// (id, value)
pub type PubMemoryValue = (u32, [u32; 8]);

// TODO(alonf): Change this into a struct. Remove Pub prefix.
// (address, id, value)
pub type PubMemoryEntry = (u32, u32, [u32; 8]);

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
    pub pedersen: Option<SegmentRange>,
    pub range_check_128: Option<SegmentRange>,
    pub ecdsa: Option<SegmentRange>,
    pub bitwise: Option<SegmentRange>,
    pub ec_op: Option<SegmentRange>,
    pub keccak: Option<SegmentRange>,
    pub poseidon: Option<SegmentRange>,
    pub range_check_96: Option<SegmentRange>,
    pub add_mod: Option<SegmentRange>,
    pub mul_mod: Option<SegmentRange>,
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

        segments.append(self.output);
        if let Some(pedersen) = self.pedersen {
            segments.append(pedersen);
        }
        if let Some(range_check_128) = self.range_check_128 {
            segments.append(range_check_128);
        }
        if let Some(ecdsa) = self.ecdsa {
            segments.append(ecdsa);
        }
        if let Some(bitwise) = self.bitwise {
            segments.append(bitwise);
        }
        if let Some(ec_op) = self.ec_op {
            segments.append(ec_op);
        }
        if let Some(keccak) = self.keccak {
            segments.append(keccak);
        }
        if let Some(poseidon) = self.poseidon {
            segments.append(poseidon);
        }
        if let Some(range_check_96) = self.range_check_96 {
            segments.append(range_check_96);
        }
        if let Some(add_mod) = self.add_mod {
            segments.append(add_mod);
        }
        if let Some(mul_mod) = self.mul_mod {
            segments.append(mul_mod);
        }
        segments
    }

    fn assert_bootloader_context(self: @PublicSegmentRanges) {
        let n_builtins = self.present_segments().len();
        assert!(n_builtins == 11);
    }
}

/// A contiguous memory section.
pub type MemorySection = Array<PubMemoryValue>;

/// Returns the hash of the memory section.
/// Note: this function ignores the ids and therefore assumes that the section is sorted.
// TODO(Gali): Unite this function with the one in verifier_core in a separate crate.
#[cfg(not(feature: "poseidon252_verifier"))]
pub fn hash_memory_section(section: @MemorySection) -> Box<[u32; 8]> {
    let mut state = BoxTrait::new(BLAKE2S_256_INITIAL_STATE);
    let mut byte_count = 0;

    let mut buffer = array![];
    for entry in section {
        // Compress whenever the buffer reaches capacity.
        if let Some(msg) = buffer.span().try_into() {
            state = blake2s_compress(state, byte_count, *msg);
            buffer = array![];
        }

        // Append current value to the buffer without its id.
        let (_, val) = *entry;
        buffer.append_span(val.span());
        byte_count += 32;
    }

    // Pad buffer to blake hash message size.
    for _ in buffer.len()..16 {
        buffer.append(0);
    }

    // Finalize hash.
    blake2s_finalize(state, byte_count, *buffer.span().try_into().unwrap())
}

/// Returns the hash of the memory section.
/// Note: this function ignores the ids and therefore assumes that the section is sorted.
#[cfg(feature: "poseidon252_verifier")]
pub fn hash_memory_section(section: @MemorySection) -> Box<[u32; 8]> {
    let mut felts = array![];
    for entry in section {
        let (_, val) = *entry;
        felts.append(construct_f252(BoxTrait::new(val)));
    }
    deconstruct_f252(poseidon_hash_span(felts.span()))
}

#[derive(Serde, Drop)]
pub struct PublicMemory {
    pub program: MemorySection,
    pub public_segments: PublicSegmentRanges,
    pub output: MemorySection,
    pub safe_call: MemorySection,
}

#[generate_trait]
pub impl PublicMemoryImpl of PublicMemoryTrait {
    fn get_entries(
        self: @PublicMemory, initial_pc: u32, initial_ap: u32, final_ap: u32,
    ) -> Array<PubMemoryEntry> {
        let mut entries = array![];

        // Program.
        let mut i: u32 = 0;
        for (id, value) in self.program.span() {
            entries.append((initial_pc + i, *id, *value));
            i += 1;
        }

        // Output.
        i = 0;
        for (id, value) in self.output.span() {
            entries.append((final_ap + i, *id, *value));
            i += 1;
        }

        // Safe call.
        let (id, value) = self.safe_call[0];
        entries.append((initial_ap - 2, *id, *value));
        let (id, value) = self.safe_call[1];
        entries.append((initial_ap - 1, *id, *value));

        let present_segments = self.public_segments.present_segments();
        let n_segments = present_segments.len();
        i = 0;
        for segment in present_segments {
            entries
                .append(
                    (
                        initial_ap + i,
                        *segment.start_ptr.id,
                        [*segment.start_ptr.value, 0, 0, 0, 0, 0, 0, 0],
                    ),
                );
            entries
                .append(
                    (
                        final_ap - n_segments + i,
                        *segment.stop_ptr.id,
                        [*segment.stop_ptr.value, 0, 0, 0, 0, 0, 0, 0],
                    ),
                );
            i += 1;
        }

        entries
    }
    fn mix_into(self: @PublicMemory, ref channel: Channel) {
        let PublicMemory { program, public_segments, output, safe_call } = self;

        // Program is the bootloader and doesn't need to be mixed into the channel.
        let _ = program;

        // Mix public segments.
        public_segments.mix_into(ref channel);

        // Mix output memory section.
        channel.mix_memory_section(output);

        // Mix safe_call memory section.
        channel.mix_u64(safe_call.len().into());
        for (id, value) in safe_call.span() {
            channel.mix_u64((*id).into());
            // Mix each element of the array individually
            for val_element in (*value).span() {
                channel.mix_u64((*val_element).into());
            }
        }
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
        sum += sum_public_memory_entries(@public_memory_entries, lookup_elements);

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
    entries: @Array<PubMemoryEntry>, lookup_elements: @CairoInteractionElements,
) -> QM31 {
    let mut sum = Zero::zero();
    let id_to_value_alpha = *lookup_elements.memory_id_to_value.alpha;
    let id_to_value_z = *lookup_elements.memory_id_to_value.z;
    let addr_to_id_alpha = *lookup_elements.memory_address_to_id.alpha;
    let addr_to_id_z = *lookup_elements.memory_address_to_id.z;

    for entry in entries.span() {
        let (addr, id, val) = *entry;

        let addr_m31: M31 = addr.try_into().unwrap();
        let addr_qm31 = addr_m31.into();
        let id_m31: M31 = id.try_into().unwrap();
        let id_qm31 = id_m31.into();
        let addr_to_id = (addr_qm31 + id_qm31 * addr_to_id_alpha - addr_to_id_z).inverse();

        // Use handwritten implementation of combine_id_to_value to improve performance.
        let mut combine_sum = combine::combine_felt252(val, id_to_value_alpha);
        combine_sum = combine_sum * id_to_value_alpha + id_m31.into() - id_to_value_z;
        let id_to_value = combine_sum.inverse();

        sum += addr_to_id + id_to_value;
    }

    sum
}

#[cfg(not(feature: "qm31_opcode"))]
// An alternative implementation that uses batch inverse, for the case that we don't have an opcode
// for it.
fn sum_public_memory_entries(
    entries: @Array<PubMemoryEntry>, lookup_elements: @CairoInteractionElements,
) -> QM31 {
    // Gather values to be inverted and summed.
    let mut values: Array<QM31> = array![];

    let id_to_value_alpha_powers: Box<[QM31; 29]> = *(lookup_elements
        .memory_id_to_value
        .alpha_powers
        .span()
        .try_into()
        .unwrap());
    let id_to_value_z = *lookup_elements.memory_id_to_value.z;
    let addr_to_id_alpha = *lookup_elements.memory_address_to_id.alpha;
    let addr_to_id_z = *lookup_elements.memory_address_to_id.z;

    for entry in entries.span() {
        let (addr, id, val) = *entry;

        let addr_m31: M31 = addr.try_into().unwrap();
        let addr_qm31: QM31 = addr_m31.into();
        let id_m31: M31 = id.try_into().unwrap();
        let addr_to_id = addr_qm31 + addr_to_id_alpha.mul_m31(id_m31) - addr_to_id_z;
        values.append(addr_to_id);

        // Use handwritten implementation of combine_id_to_value to improve performance.
        let combined_limbs = combine::combine_felt252(val, id_to_value_alpha_powers);
        let id_to_value = combined_limbs + id_m31.into() - id_to_value_z;
        values.append(id_to_value);
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

