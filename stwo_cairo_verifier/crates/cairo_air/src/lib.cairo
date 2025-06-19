use components::CairoComponent;
use components::add_ap_opcode::{
    ClaimImpl as AddApOpcodeClaimImpl, InteractionClaimImpl as AddApOpcodeInteractionClaimImpl,
};
use components::add_mod_builtin::{
    ClaimImpl as AddModBuiltinClaimImpl, InteractionClaimImpl as AddModBuiltinInteractionClaimImpl,
};
use components::add_opcode::{
    ClaimImpl as AddOpcodeClaimImpl, InteractionClaimImpl as AddOpcodeInteractionClaimImpl,
};
use components::add_opcode_small::{
    ClaimImpl as AddOpcodeSmallClaimImpl,
    InteractionClaimImpl as AddOpcodeSmallInteractionClaimImpl,
};
use components::assert_eq_opcode::{
    ClaimImpl as AssertEqOpcodeClaimImpl,
    InteractionClaimImpl as AssertEqOpcodeInteractionClaimImpl,
};
use components::assert_eq_opcode_double_deref::{
    ClaimImpl as AssertEqOpcodeDoubleDerefClaimImpl,
    InteractionClaimImpl as AssertEqOpcodeDoubleDerefInteractionClaimImpl,
};
use components::assert_eq_opcode_imm::{
    ClaimImpl as AssertEqOpcodeImmClaimImpl,
    InteractionClaimImpl as AssertEqOpcodeImmInteractionClaimImpl,
};
use components::bitwise_builtin::{
    ClaimImpl as BitwiseBuiltinClaimImpl,
    InteractionClaimImpl as BitwiseBuiltinInteractionClaimImpl,
};
use components::blake_compress_opcode::{
    ClaimImpl as BlakeCompressOpcodeClaimImpl,
    InteractionClaimImpl as BlakeCompressOpcodeInteractionClaimImpl,
};
use components::blake_g::{
    ClaimImpl as BlakeGClaimImpl, InteractionClaimImpl as BlakeGInteractionClaimImpl,
};
use components::blake_round::{
    ClaimImpl as BlakeRoundClaimImpl, InteractionClaimImpl as BlakeRoundInteractionClaimImpl,
};
use components::blake_round_sigma::{
    ClaimImpl as BlakeRoundSigmaClaimImpl,
    InteractionClaimImpl as BlakeRoundSigmaInteractionClaimImpl,
};
use components::call_opcode::{
    ClaimImpl as CallOpcodeClaimImpl, InteractionClaimImpl as CallOpcodeInteractionClaimImpl,
};
use components::call_opcode_rel_imm::{
    ClaimImpl as CallOpcodeRelClaimImpl, InteractionClaimImpl as CallOpcodeRelInteractionClaimImpl,
};
use components::cube_252::{
    ClaimImpl as Cube252ClaimImpl, InteractionClaimImpl as Cube252InteractionClaimImpl,
};
use components::generic_opcode::{
    ClaimImpl as GenericOpcodeClaimImpl, InteractionClaimImpl as GenericOpcodeInteractionClaimImpl,
};
use components::jnz_opcode::{
    ClaimImpl as JnzOpcodeClaimImpl, InteractionClaimImpl as JnzOpcodeInteractionClaimImpl,
};
use components::jnz_opcode_taken::{
    ClaimImpl as JnzOpcodeTakenClaimImpl,
    InteractionClaimImpl as JnzOpcodeTakenInteractionClaimImpl,
};
use components::jump_opcode::{
    ClaimImpl as JumpOpcodeClaimImpl, InteractionClaimImpl as JumpOpcodeInteractionClaimImpl,
};
use components::jump_opcode_double_deref::{
    ClaimImpl as JumpOpcodeDoubleDerefClaimImpl,
    InteractionClaimImpl as JumpOpcodeDoubleDerefInteractionClaimImpl,
};
use components::jump_opcode_rel::{
    ClaimImpl as JumpOpcodeRelClaimImpl, InteractionClaimImpl as JumpOpcodeRelInteractionClaimImpl,
};
use components::jump_opcode_rel_imm::{
    ClaimImpl as JumpOpcodeRelImmClaimImpl,
    InteractionClaimImpl as JumpOpcodeRelImmInteractionClaimImpl,
};
use components::memory_address_to_id::{
    ClaimImpl as MemoryAddressToIdClaimImpl,
    InteractionClaimImpl as MemoryAddressToIdInteractionClaimImpl, LOG_MEMORY_ADDRESS_TO_ID_SPLIT,
    MEMORY_ADDRESS_TO_ID_SPLIT,
};
use components::memory_id_to_big::{
    ClaimImpl as MemoryIdToBigClaimImpl, InteractionClaimImpl as MemoryIdToBigInteractionClaimImpl,
};
use components::mul_mod_builtin::{
    ClaimImpl as MulModBuiltinClaimImpl, InteractionClaimImpl as MulModBuiltinInteractionClaimImpl,
};
use components::mul_opcode::{
    ClaimImpl as MulOpcodeClaimImpl, InteractionClaimImpl as MulOpcodeInteractionClaimImpl,
};
use components::mul_opcode_small::{
    ClaimImpl as MulOpcodeSmallClaimImpl,
    InteractionClaimImpl as MulOpcodeSmallInteractionClaimImpl,
};
use components::partial_ec_mul::{
    ClaimImpl as PartialEcMulClaimImpl, InteractionClaimImpl as PartialEcMulInteractionClaimImpl,
};
use components::pedersen_builtin::{
    ClaimImpl as PedersenBuiltinClaimImpl,
    InteractionClaimImpl as PedersenBuiltinInteractionClaimImpl,
};
use components::pedersen_points_table::{
    ClaimImpl as PedersenPointsTableClaimImpl,
    InteractionClaimImpl as PedersenPointsTableInteractionClaimImpl,
};
use components::poseidon_3_partial_rounds_chain::{
    ClaimImpl as Poseidon3PartialRoundsChainClaimImpl,
    InteractionClaimImpl as Poseidon3PartialRoundsChainInteractionClaimImpl,
};
use components::poseidon_builtin::{
    ClaimImpl as PoseidonBuiltinClaimImpl,
    InteractionClaimImpl as PoseidonBuiltinInteractionClaimImpl,
};
use components::poseidon_full_round_chain::{
    ClaimImpl as PoseidonFullRoundChainClaimImpl,
    InteractionClaimImpl as PoseidonFullRoundChainInteractionClaimImpl,
};
use components::poseidon_round_keys::{
    ClaimImpl as PoseidonRoundKeysClaimImpl,
    InteractionClaimImpl as PoseidonRoundKeysInteractionClaimImpl,
};
use components::qm_31_add_mul_opcode::{
    ClaimImpl as Qm31AddMulOpcodeClaimImpl,
    InteractionClaimImpl as Qm31AddMulOpcodeInteractionClaimImpl,
};
use components::range_check_11::{
    ClaimImpl as RangeCheck_11ClaimImpl, InteractionClaimImpl as RangeCheck_11InteractionClaimImpl,
};
use components::range_check_12::{
    ClaimImpl as RangeCheck_12ClaimImpl, InteractionClaimImpl as RangeCheck_12InteractionClaimImpl,
};
use components::range_check_18::{
    ClaimImpl as RangeCheck_18ClaimImpl, InteractionClaimImpl as RangeCheck_18InteractionClaimImpl,
};
use components::range_check_18_b::{
    ClaimImpl as RangeCheck_18BClaimImpl,
    InteractionClaimImpl as RangeCheck_18BInteractionClaimImpl,
};
use components::range_check_19::{
    ClaimImpl as RangeCheck_19ClaimImpl, InteractionClaimImpl as RangeCheck_19InteractionClaimImpl,
};
use components::range_check_19_b::{
    ClaimImpl as RangeCheck_19BClaimImpl,
    InteractionClaimImpl as RangeCheck_19BInteractionClaimImpl,
};
use components::range_check_19_c::{
    ClaimImpl as RangeCheck_19CClaimImpl,
    InteractionClaimImpl as RangeCheck_19CInteractionClaimImpl,
};
use components::range_check_19_d::{
    ClaimImpl as RangeCheck_19DClaimImpl,
    InteractionClaimImpl as RangeCheck_19DInteractionClaimImpl,
};
use components::range_check_19_e::{
    ClaimImpl as RangeCheck_19EClaimImpl,
    InteractionClaimImpl as RangeCheck_19EInteractionClaimImpl,
};
use components::range_check_19_f::{
    ClaimImpl as RangeCheck_19FClaimImpl,
    InteractionClaimImpl as RangeCheck_19FInteractionClaimImpl,
};
use components::range_check_19_g::{
    ClaimImpl as RangeCheck_19GClaimImpl,
    InteractionClaimImpl as RangeCheck_19GInteractionClaimImpl,
};
use components::range_check_19_h::{
    ClaimImpl as RangeCheck_19HClaimImpl,
    InteractionClaimImpl as RangeCheck_19HInteractionClaimImpl,
};
use components::range_check_3_3_3_3_3::{
    ClaimImpl as RangeCheck_3_3_3_3_3ClaimImpl,
    InteractionClaimImpl as RangeCheck_3_3_3_3_3InteractionClaimImpl,
};
use components::range_check_3_6_6_3::{
    ClaimImpl as RangeCheck_3_6_6_3ClaimImpl,
    InteractionClaimImpl as RangeCheck_3_6_6_3InteractionClaimImpl,
};
use components::range_check_4_3::{
    ClaimImpl as RangeCheck_4_3ClaimImpl,
    InteractionClaimImpl as RangeCheck_4_3InteractionClaimImpl,
};
use components::range_check_4_4::{
    ClaimImpl as RangeCheck_4_4ClaimImpl,
    InteractionClaimImpl as RangeCheck_4_4InteractionClaimImpl,
};
use components::range_check_4_4_4_4::{
    ClaimImpl as RangeCheck_4_4_4_4ClaimImpl,
    InteractionClaimImpl as RangeCheck_4_4_4_4InteractionClaimImpl,
};
use components::range_check_5_4::{
    ClaimImpl as RangeCheck_5_4ClaimImpl,
    InteractionClaimImpl as RangeCheck_5_4InteractionClaimImpl,
};
use components::range_check_6::{
    ClaimImpl as RangeCheck_6ClaimImpl, InteractionClaimImpl as RangeCheck_6InteractionClaimImpl,
};
use components::range_check_7_2_5::{
    ClaimImpl as RangeCheck_7_2_5ClaimImpl,
    InteractionClaimImpl as RangeCheck_7_2_5InteractionClaimImpl,
};
use components::range_check_8::{
    ClaimImpl as RangeCheck_8ClaimImpl, InteractionClaimImpl as RangeCheck_8InteractionClaimImpl,
};
use components::range_check_9_9::{
    ClaimImpl as RangeCheck_9_9ClaimImpl,
    InteractionClaimImpl as RangeCheck_9_9InteractionClaimImpl,
};
use components::range_check_9_9_b::{
    ClaimImpl as RangeCheck_9_9BClaimImpl,
    InteractionClaimImpl as RangeCheck_9_9BInteractionClaimImpl,
};
use components::range_check_9_9_c::{
    ClaimImpl as RangeCheck_9_9CClaimImpl,
    InteractionClaimImpl as RangeCheck_9_9CInteractionClaimImpl,
};
use components::range_check_9_9_d::{
    ClaimImpl as RangeCheck_9_9DClaimImpl,
    InteractionClaimImpl as RangeCheck_9_9DInteractionClaimImpl,
};
use components::range_check_9_9_e::{
    ClaimImpl as RangeCheck_9_9EClaimImpl,
    InteractionClaimImpl as RangeCheck_9_9EInteractionClaimImpl,
};
use components::range_check_9_9_f::{
    ClaimImpl as RangeCheck_9_9FClaimImpl,
    InteractionClaimImpl as RangeCheck_9_9FInteractionClaimImpl,
};
use components::range_check_9_9_g::{
    ClaimImpl as RangeCheck_9_9GClaimImpl,
    InteractionClaimImpl as RangeCheck_9_9GInteractionClaimImpl,
};
use components::range_check_9_9_h::{
    ClaimImpl as RangeCheck_9_9HClaimImpl,
    InteractionClaimImpl as RangeCheck_9_9HInteractionClaimImpl,
};
use components::range_check_builtin_bits_128::{
    ClaimImpl as RangeCheckBuiltinBits128ClaimImpl,
    InteractionClaimImpl as RangeCheckBuiltinBits128InteractionClaimImpl,
};
use components::range_check_builtin_bits_96::{
    ClaimImpl as RangeCheckBuiltinBits96ClaimImpl,
    InteractionClaimImpl as RangeCheckBuiltinBits96InteractionClaimImpl,
};
use components::range_check_felt_252_width_27::{
    ClaimImpl as RangeCheckFelt252Width27ClaimImpl,
    InteractionClaimImpl as RangeCheckFelt252Width27InteractionClaimImpl,
};
use components::ret_opcode::{
    ClaimImpl as RetOpcodeClaimImpl, InteractionClaimImpl as RetOpcodeInteractionClaimImpl,
};
use components::triple_xor_32::{
    ClaimImpl as TripleXor32ClaimImpl, InteractionClaimImpl as TripleXor32InteractionClaimImpl,
};
use components::verify_bitwise_xor_12::{
    ClaimImpl as VerifyBitwiseXor12ClaimImpl,
    InteractionClaimImpl as VerifyBitwiseXor12InteractionClaimImpl,
};
use components::verify_bitwise_xor_4::{
    ClaimImpl as VerifyBitwiseXor4ClaimImpl,
    InteractionClaimImpl as VerifyBitwiseXor4InteractionClaimImpl,
};
use components::verify_bitwise_xor_7::{
    ClaimImpl as VerifyBitwiseXor7ClaimImpl,
    InteractionClaimImpl as VerifyBitwiseXor7InteractionClaimImpl,
};
use components::verify_bitwise_xor_8::{
    ClaimImpl as VerifyBitwiseXor8ClaimImpl,
    InteractionClaimImpl as VerifyBitwiseXor8InteractionClaimImpl,
};
use components::verify_bitwise_xor_9::{
    ClaimImpl as VerifyBitwiseXor9ClaimImpl,
    InteractionClaimImpl as VerifyBitwiseXor9InteractionClaimImpl,
};
use components::verify_instruction::{
    ClaimImpl as VerifyInstructionClaimImpl,
    InteractionClaimImpl as VerifyInstructionInteractionClaimImpl,
};
#[cfg(not(feature: "poseidon252_verifier"))]
use core::blake::{blake2s_compress, blake2s_finalize};
use core::box::BoxImpl;
use core::dict::{Felt252Dict, Felt252DictEntryTrait, Felt252DictTrait, SquashedFelt252DictTrait};
use core::num::traits::Zero;
use core::num::traits::one::One;
#[cfg(feature: "poseidon252_verifier")]
use core::poseidon::poseidon_hash_span;
use stwo_cairo_air::utils::{construct_f252, deconstruct_f252};
use stwo_constraint_framework::{
    LookupElements, LookupElementsImpl, PreprocessedColumn, PreprocessedColumnImpl,
    PreprocessedColumnKey, PreprocessedColumnSet, PreprocessedColumnTrait, PreprocessedMaskValues,
    PreprocessedMaskValuesImpl,
};
#[cfg(not(feature: "poseidon252_verifier"))]
use stwo_verifier_core::channel::blake2s::BLAKE2S_256_INITIAL_STATE;
use stwo_verifier_core::channel::{Channel, ChannelImpl, ChannelTrait};
use stwo_verifier_core::circle::CirclePoint;
use stwo_verifier_core::fields::Invertible;
use stwo_verifier_core::fields::m31::{M31, P_U32, m31};
use stwo_verifier_core::fields::qm31::{QM31, qm31_const};
use stwo_verifier_core::fri::FriConfig;
use stwo_verifier_core::pcs::verifier::CommitmentSchemeVerifierImpl;
use stwo_verifier_core::pcs::{PcsConfig, PcsConfigTrait};
use stwo_verifier_core::utils::{ArrayImpl, OptionImpl, pow2};
use stwo_verifier_core::verifier::{Air, StarkProof, VerificationError, verify};
use stwo_verifier_core::{ColumnArray, ColumnSpan, Hash, TreeArray, TreeSpan};
pub mod components;
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
// IMPORTANT: This function must exactly match the output and ordering of the prover preprocessed
// trace declaration. If the function changes, this array must be updated to stay in sync.
// https://github.com/starkware-libs/stwo-cairo/blame/175026d/stwo_cairo_prover/crates/cairo-air/src/preprocessed.rs#L42
#[cairofmt::skip]
#[cfg(not(feature: "poseidon252_verifier"))]
const PREPROCESSED_COLUMNS: [PreprocessedColumn; 162] = [
    PreprocessedColumn::Seq(24),
    PreprocessedColumn::Seq(23),
    PreprocessedColumn::PedersenPoints(0),
    PreprocessedColumn::PedersenPoints(1),
    PreprocessedColumn::PedersenPoints(2),
    PreprocessedColumn::PedersenPoints(3),
    PreprocessedColumn::PedersenPoints(4),
    PreprocessedColumn::PedersenPoints(5),
    PreprocessedColumn::PedersenPoints(6),
    PreprocessedColumn::PedersenPoints(7),
    PreprocessedColumn::PedersenPoints(8),
    PreprocessedColumn::PedersenPoints(9),
    PreprocessedColumn::PedersenPoints(10),
    PreprocessedColumn::PedersenPoints(11),
    PreprocessedColumn::PedersenPoints(12),
    PreprocessedColumn::PedersenPoints(13),
    PreprocessedColumn::PedersenPoints(14),
    PreprocessedColumn::PedersenPoints(15),
    PreprocessedColumn::PedersenPoints(16),
    PreprocessedColumn::PedersenPoints(17),
    PreprocessedColumn::PedersenPoints(18),
    PreprocessedColumn::PedersenPoints(19),
    PreprocessedColumn::PedersenPoints(20),
    PreprocessedColumn::PedersenPoints(21),
    PreprocessedColumn::PedersenPoints(22),
    PreprocessedColumn::PedersenPoints(23),
    PreprocessedColumn::PedersenPoints(24),
    PreprocessedColumn::PedersenPoints(25),
    PreprocessedColumn::PedersenPoints(26),
    PreprocessedColumn::PedersenPoints(27),
    PreprocessedColumn::PedersenPoints(28),
    PreprocessedColumn::PedersenPoints(29),
    PreprocessedColumn::PedersenPoints(30),
    PreprocessedColumn::PedersenPoints(31),
    PreprocessedColumn::PedersenPoints(32),
    PreprocessedColumn::PedersenPoints(33),
    PreprocessedColumn::PedersenPoints(34),
    PreprocessedColumn::PedersenPoints(35),
    PreprocessedColumn::PedersenPoints(36),
    PreprocessedColumn::PedersenPoints(37),
    PreprocessedColumn::PedersenPoints(38),
    PreprocessedColumn::PedersenPoints(39),
    PreprocessedColumn::PedersenPoints(40),
    PreprocessedColumn::PedersenPoints(41),
    PreprocessedColumn::PedersenPoints(42),
    PreprocessedColumn::PedersenPoints(43),
    PreprocessedColumn::PedersenPoints(44),
    PreprocessedColumn::PedersenPoints(45),
    PreprocessedColumn::PedersenPoints(46),
    PreprocessedColumn::PedersenPoints(47),
    PreprocessedColumn::PedersenPoints(48),
    PreprocessedColumn::PedersenPoints(49),
    PreprocessedColumn::PedersenPoints(50),
    PreprocessedColumn::PedersenPoints(51),
    PreprocessedColumn::PedersenPoints(52),
    PreprocessedColumn::PedersenPoints(53),
    PreprocessedColumn::PedersenPoints(54),
    PreprocessedColumn::PedersenPoints(55),
    PreprocessedColumn::Seq(22),
    PreprocessedColumn::Seq(21),
    PreprocessedColumn::Seq(20),
    PreprocessedColumn::BitwiseXor((10, 0)),
    PreprocessedColumn::BitwiseXor((10, 1)),
    PreprocessedColumn::BitwiseXor((10, 2)),
    PreprocessedColumn::Seq(19),
    PreprocessedColumn::Seq(18),
    PreprocessedColumn::BitwiseXor((9, 0)),
    PreprocessedColumn::BitwiseXor((9, 1)),
    PreprocessedColumn::BitwiseXor((9, 2)),
    PreprocessedColumn::RangeCheck2(([9, 9], 0)),
    PreprocessedColumn::RangeCheck2(([9, 9], 1)),
    PreprocessedColumn::RangeCheck4(([3, 6, 6, 3], 0)),
    PreprocessedColumn::RangeCheck4(([3, 6, 6, 3], 1)),
    PreprocessedColumn::RangeCheck4(([3, 6, 6, 3], 2)),
    PreprocessedColumn::RangeCheck4(([3, 6, 6, 3], 3)),
    PreprocessedColumn::Seq(17),
    PreprocessedColumn::Seq(16),
    PreprocessedColumn::BitwiseXor((8, 0)),
    PreprocessedColumn::BitwiseXor((8, 1)),
    PreprocessedColumn::BitwiseXor((8, 2)),
    PreprocessedColumn::RangeCheck4(([4, 4, 4, 4], 0)),
    PreprocessedColumn::RangeCheck4(([4, 4, 4, 4], 1)),
    PreprocessedColumn::RangeCheck4(([4, 4, 4, 4], 2)),
    PreprocessedColumn::RangeCheck4(([4, 4, 4, 4], 3)),
    PreprocessedColumn::Seq(15),
    PreprocessedColumn::RangeCheck5(([3, 3, 3, 3, 3], 0)),
    PreprocessedColumn::RangeCheck5(([3, 3, 3, 3, 3], 1)),
    PreprocessedColumn::RangeCheck5(([3, 3, 3, 3, 3], 2)),
    PreprocessedColumn::RangeCheck5(([3, 3, 3, 3, 3], 3)),
    PreprocessedColumn::RangeCheck5(([3, 3, 3, 3, 3], 4)),
    PreprocessedColumn::Seq(14),
    PreprocessedColumn::BitwiseXor((7, 0)),
    PreprocessedColumn::BitwiseXor((7, 1)),
    PreprocessedColumn::BitwiseXor((7, 2)),
    PreprocessedColumn::RangeCheck3(([7, 2, 5], 0)),
    PreprocessedColumn::RangeCheck3(([7, 2, 5], 1)),
    PreprocessedColumn::RangeCheck3(([7, 2, 5], 2)),
    PreprocessedColumn::Seq(13),
    PreprocessedColumn::Seq(12),
    PreprocessedColumn::Seq(11),
    PreprocessedColumn::Seq(10),
    PreprocessedColumn::Seq(9),
    PreprocessedColumn::RangeCheck2(([5, 4], 0)),
    PreprocessedColumn::RangeCheck2(([5, 4], 1)),
    PreprocessedColumn::Seq(8),
    PreprocessedColumn::BitwiseXor((4, 0)),
    PreprocessedColumn::BitwiseXor((4, 1)),
    PreprocessedColumn::BitwiseXor((4, 2)),
    PreprocessedColumn::RangeCheck2(([4, 4], 0)),
    PreprocessedColumn::RangeCheck2(([4, 4], 1)),
    PreprocessedColumn::Seq(7),
    PreprocessedColumn::RangeCheck2(([4, 3], 0)),
    PreprocessedColumn::RangeCheck2(([4, 3], 1)),
    PreprocessedColumn::Seq(6),
    PreprocessedColumn::PoseidonRoundKeys(0),
    PreprocessedColumn::PoseidonRoundKeys(1),
    PreprocessedColumn::PoseidonRoundKeys(2),
    PreprocessedColumn::PoseidonRoundKeys(3),
    PreprocessedColumn::PoseidonRoundKeys(4),
    PreprocessedColumn::PoseidonRoundKeys(5),
    PreprocessedColumn::PoseidonRoundKeys(6),
    PreprocessedColumn::PoseidonRoundKeys(7),
    PreprocessedColumn::PoseidonRoundKeys(8),
    PreprocessedColumn::PoseidonRoundKeys(9),
    PreprocessedColumn::PoseidonRoundKeys(10),
    PreprocessedColumn::PoseidonRoundKeys(11),
    PreprocessedColumn::PoseidonRoundKeys(12),
    PreprocessedColumn::PoseidonRoundKeys(13),
    PreprocessedColumn::PoseidonRoundKeys(14),
    PreprocessedColumn::PoseidonRoundKeys(15),
    PreprocessedColumn::PoseidonRoundKeys(16),
    PreprocessedColumn::PoseidonRoundKeys(17),
    PreprocessedColumn::PoseidonRoundKeys(18),
    PreprocessedColumn::PoseidonRoundKeys(19),
    PreprocessedColumn::PoseidonRoundKeys(20),
    PreprocessedColumn::PoseidonRoundKeys(21),
    PreprocessedColumn::PoseidonRoundKeys(22),
    PreprocessedColumn::PoseidonRoundKeys(23),
    PreprocessedColumn::PoseidonRoundKeys(24),
    PreprocessedColumn::PoseidonRoundKeys(25),
    PreprocessedColumn::PoseidonRoundKeys(26),
    PreprocessedColumn::PoseidonRoundKeys(27),
    PreprocessedColumn::PoseidonRoundKeys(28),
    PreprocessedColumn::PoseidonRoundKeys(29),
    PreprocessedColumn::Seq(5),
    PreprocessedColumn::Seq(4),
    PreprocessedColumn::BlakeSigma(0),
    PreprocessedColumn::BlakeSigma(1),
    PreprocessedColumn::BlakeSigma(2),
    PreprocessedColumn::BlakeSigma(3),
    PreprocessedColumn::BlakeSigma(4),
    PreprocessedColumn::BlakeSigma(5),
    PreprocessedColumn::BlakeSigma(6),
    PreprocessedColumn::BlakeSigma(7),
    PreprocessedColumn::BlakeSigma(8),
    PreprocessedColumn::BlakeSigma(9),
    PreprocessedColumn::BlakeSigma(10),
    PreprocessedColumn::BlakeSigma(11),
    PreprocessedColumn::BlakeSigma(12),
    PreprocessedColumn::BlakeSigma(13),
    PreprocessedColumn::BlakeSigma(14),
    PreprocessedColumn::BlakeSigma(15)
];
// IMPORTANT: This function must exactly match the output and ordering of the prover preprocessed
// trace declaration. If the function changes, this array must be updated to stay in sync.
// https://github.com/starkware-libs/stwo-cairo/blame/175026d/stwo_cairo_prover/crates/cairo-air/src/preprocessed.rs#L42
#[cairofmt::skip]
#[cfg(feature: "poseidon252_verifier")]
const PREPROCESSED_COLUMNS: [PreprocessedColumn; 106] = [
    PreprocessedColumn::Seq(24),
    PreprocessedColumn::Seq(23),
    PreprocessedColumn::Seq(22),
    PreprocessedColumn::Seq(21),
    PreprocessedColumn::Seq(20),
    PreprocessedColumn::BitwiseXor((10, 0)),
    PreprocessedColumn::BitwiseXor((10, 1)),
    PreprocessedColumn::BitwiseXor((10, 2)),
    PreprocessedColumn::Seq(19),
    PreprocessedColumn::Seq(18),
    PreprocessedColumn::BitwiseXor((9, 0)),
    PreprocessedColumn::BitwiseXor((9, 1)),
    PreprocessedColumn::BitwiseXor((9, 2)),
    PreprocessedColumn::RangeCheck2(([9, 9], 0)),
    PreprocessedColumn::RangeCheck2(([9, 9], 1)),
    PreprocessedColumn::RangeCheck4(([3, 6, 6, 3], 0)),
    PreprocessedColumn::RangeCheck4(([3, 6, 6, 3], 1)),
    PreprocessedColumn::RangeCheck4(([3, 6, 6, 3], 2)),
    PreprocessedColumn::RangeCheck4(([3, 6, 6, 3], 3)),
    PreprocessedColumn::Seq(17),
    PreprocessedColumn::Seq(16),
    PreprocessedColumn::BitwiseXor((8, 0)),
    PreprocessedColumn::BitwiseXor((8, 1)),
    PreprocessedColumn::BitwiseXor((8, 2)),
    PreprocessedColumn::RangeCheck4(([4, 4, 4, 4], 0)),
    PreprocessedColumn::RangeCheck4(([4, 4, 4, 4], 1)),
    PreprocessedColumn::RangeCheck4(([4, 4, 4, 4], 2)),
    PreprocessedColumn::RangeCheck4(([4, 4, 4, 4], 3)),
    PreprocessedColumn::Seq(15),
    PreprocessedColumn::RangeCheck5(([3, 3, 3, 3, 3], 0)),
    PreprocessedColumn::RangeCheck5(([3, 3, 3, 3, 3], 1)),
    PreprocessedColumn::RangeCheck5(([3, 3, 3, 3, 3], 2)),
    PreprocessedColumn::RangeCheck5(([3, 3, 3, 3, 3], 3)),
    PreprocessedColumn::RangeCheck5(([3, 3, 3, 3, 3], 4)),
    PreprocessedColumn::Seq(14),
    PreprocessedColumn::BitwiseXor((7, 0)),
    PreprocessedColumn::BitwiseXor((7, 1)),
    PreprocessedColumn::BitwiseXor((7, 2)),
    PreprocessedColumn::RangeCheck3(([7, 2, 5], 0)),
    PreprocessedColumn::RangeCheck3(([7, 2, 5], 1)),
    PreprocessedColumn::RangeCheck3(([7, 2, 5], 2)),
    PreprocessedColumn::Seq(13),
    PreprocessedColumn::Seq(12),
    PreprocessedColumn::Seq(11),
    PreprocessedColumn::Seq(10),
    PreprocessedColumn::Seq(9),
    PreprocessedColumn::RangeCheck2(([5, 4], 0)),
    PreprocessedColumn::RangeCheck2(([5, 4], 1)),
    PreprocessedColumn::Seq(8),
    PreprocessedColumn::BitwiseXor((4, 0)),
    PreprocessedColumn::BitwiseXor((4, 1)),
    PreprocessedColumn::BitwiseXor((4, 2)),
    PreprocessedColumn::RangeCheck2(([4, 4], 0)),
    PreprocessedColumn::RangeCheck2(([4, 4], 1)),
    PreprocessedColumn::Seq(7),
    PreprocessedColumn::RangeCheck2(([4, 3], 0)),
    PreprocessedColumn::RangeCheck2(([4, 3], 1)),
    PreprocessedColumn::Seq(6),
    PreprocessedColumn::PoseidonRoundKeys(0),
    PreprocessedColumn::PoseidonRoundKeys(1),
    PreprocessedColumn::PoseidonRoundKeys(2),
    PreprocessedColumn::PoseidonRoundKeys(3),
    PreprocessedColumn::PoseidonRoundKeys(4),
    PreprocessedColumn::PoseidonRoundKeys(5),
    PreprocessedColumn::PoseidonRoundKeys(6),
    PreprocessedColumn::PoseidonRoundKeys(7),
    PreprocessedColumn::PoseidonRoundKeys(8),
    PreprocessedColumn::PoseidonRoundKeys(9),
    PreprocessedColumn::PoseidonRoundKeys(10),
    PreprocessedColumn::PoseidonRoundKeys(11),
    PreprocessedColumn::PoseidonRoundKeys(12),
    PreprocessedColumn::PoseidonRoundKeys(13),
    PreprocessedColumn::PoseidonRoundKeys(14),
    PreprocessedColumn::PoseidonRoundKeys(15),
    PreprocessedColumn::PoseidonRoundKeys(16),
    PreprocessedColumn::PoseidonRoundKeys(17),
    PreprocessedColumn::PoseidonRoundKeys(18),
    PreprocessedColumn::PoseidonRoundKeys(19),
    PreprocessedColumn::PoseidonRoundKeys(20),
    PreprocessedColumn::PoseidonRoundKeys(21),
    PreprocessedColumn::PoseidonRoundKeys(22),
    PreprocessedColumn::PoseidonRoundKeys(23),
    PreprocessedColumn::PoseidonRoundKeys(24),
    PreprocessedColumn::PoseidonRoundKeys(25),
    PreprocessedColumn::PoseidonRoundKeys(26),
    PreprocessedColumn::PoseidonRoundKeys(27),
    PreprocessedColumn::PoseidonRoundKeys(28),
    PreprocessedColumn::PoseidonRoundKeys(29),
    PreprocessedColumn::Seq(5),
    PreprocessedColumn::Seq(4),
    PreprocessedColumn::BlakeSigma(0),
    PreprocessedColumn::BlakeSigma(1),
    PreprocessedColumn::BlakeSigma(2),
    PreprocessedColumn::BlakeSigma(3),
    PreprocessedColumn::BlakeSigma(4),
    PreprocessedColumn::BlakeSigma(5),
    PreprocessedColumn::BlakeSigma(6),
    PreprocessedColumn::BlakeSigma(7),
    PreprocessedColumn::BlakeSigma(8),
    PreprocessedColumn::BlakeSigma(9),
    PreprocessedColumn::BlakeSigma(10),
    PreprocessedColumn::BlakeSigma(11),
    PreprocessedColumn::BlakeSigma(12),
    PreprocessedColumn::BlakeSigma(13),
    PreprocessedColumn::BlakeSigma(14),
    PreprocessedColumn::BlakeSigma(15)
];

type Cube252Elements = LookupElements<20>;

type MemoryAddressToIdElements = LookupElements<2>;

type MemoryIdToBigElements = LookupElements<29>;

type OpcodesElements = LookupElements<3>;

type PartialEcMulElements = LookupElements<73>;

type PedersenPointsTableElements = LookupElements<57>;

type PoseidonFullRoundChainElements = LookupElements<32>;

type Poseidon3PartialRoundsChainElements = LookupElements<42>;

type PoseidonRoundKeysElements = LookupElements<31>;

type BlakeGElements = LookupElements<20>;

type BlakeRoundElements = LookupElements<35>;

type BlakeRoundSigmaElements = LookupElements<17>;

type TripleXor32Elements = LookupElements<8>;

type RangeCheck_6Elements = LookupElements<1>;

type RangeCheck_8Elements = LookupElements<1>;

type RangeCheck_11Elements = LookupElements<1>;

type RangeCheck_12Elements = LookupElements<1>;

type RangeCheck_18Elements = LookupElements<1>;
type RangeCheck_18_BElements = LookupElements<1>;

type RangeCheck_19Elements = LookupElements<1>;
type RangeCheck_19_BElements = LookupElements<1>;
type RangeCheck_19_CElements = LookupElements<1>;
type RangeCheck_19_DElements = LookupElements<1>;
type RangeCheck_19_EElements = LookupElements<1>;
type RangeCheck_19_FElements = LookupElements<1>;
type RangeCheck_19_GElements = LookupElements<1>;
type RangeCheck_19_HElements = LookupElements<1>;

type RangeCheck_9_9Elements = LookupElements<2>;
type RangeCheck_9_9_BElements = LookupElements<2>;
type RangeCheck_9_9_CElements = LookupElements<2>;
type RangeCheck_9_9_DElements = LookupElements<2>;
type RangeCheck_9_9_EElements = LookupElements<2>;
type RangeCheck_9_9_FElements = LookupElements<2>;
type RangeCheck_9_9_GElements = LookupElements<2>;
type RangeCheck_9_9_HElements = LookupElements<2>;

type RangeCheck_4_3Elements = LookupElements<2>;

type RangeCheck_4_4Elements = LookupElements<2>;

type RangeCheck_5_4Elements = LookupElements<2>;

type RangeCheck_7_2_5Elements = LookupElements<3>;

type RangeCheck_3_6_6_3Elements = LookupElements<4>;

type RangeCheck_4_4_4_4Elements = LookupElements<4>;

type RangeCheck_3_3_3_3_3Elements = LookupElements<5>;

type RangeCheckFelt252Width27Elements = LookupElements<10>;

type VerifyInstructionElements = LookupElements<7>;

type VerifyBitwiseXor_4Elements = LookupElements<3>;

type VerifyBitwiseXor_7Elements = LookupElements<3>;

type VerifyBitwiseXor_8Elements = LookupElements<3>;

type VerifyBitwiseXor_9Elements = LookupElements<3>;

type VerifyBitwiseXor_12Elements = LookupElements<3>;

// A dict from relation_id, which is a string encoded as a felt252, to the number of uses of the
// corresponding relation.
type RelationUsesDict = Felt252Dict<u64>;

// A tuple of (relation_id, uses).
type RelationUse = (felt252, u32);

/// Returns PreProcessedTrace::canonical root for the given blowup factor.
#[cfg(not(feature: "poseidon252_verifier"))]
fn preprocessed_root(log_blowup_factor: u32) -> Hash {
    match log_blowup_factor - 1 {
        0 => Hash {
            hash: BoxImpl::new(
                [
                    0x8a2202ef, 0x477c9959, 0x79655388, 0x958a3409, 0x87ec09fd, 0x7034f8ab,
                    0x1e720385, 0x70f00ad4,
                ],
            ),
        },
        1 => Hash {
            hash: BoxImpl::new(
                [
                    0x1966f0a8, 0xa0059272, 0x9eca2f06, 0x82791af7, 0x9a2c1522, 0x2fbdff33,
                    0x5553d795, 0x3fc5a18a,
                ],
            ),
        },
        2 => Hash {
            hash: BoxImpl::new(
                [
                    0x1d553a98, 0x78da025b, 0x87686d83, 0xce0aa49a, 0x9c5752d8, 0xc3954c47,
                    0xc80ca41a, 0xc757f720,
                ],
            ),
        },
        3 => Hash {
            hash: BoxImpl::new(
                [
                    0x6bd0149a, 0x786401f3, 0x98edb866, 0x53b8113b, 0xa18ef714, 0x155b1183,
                    0x19d8fff5, 0x9e792495,
                ],
            ),
        },
        4 => Hash {
            hash: BoxImpl::new(
                [
                    0x1bfe4fde, 0xeddf6d4b, 0x2bf346c4, 0x8332fe5f, 0x43ce2525, 0x55611509,
                    0xe13c2956, 0x66aeb325,
                ],
            ),
        },
        _ => panic!("invalid blowup factor"),
    }
}

/// Returns PreProcessedTrace::canonical_without_pedersen root for the given blowup factor.
#[cfg(feature: "poseidon252_verifier")]
fn preprocessed_root(log_blowup_factor: u32) -> Hash {
    match log_blowup_factor - 1 {
        0 => 0x053be12b3503460f6657c5a46ed1b56719f91e45530a869327b39b7b8a20e0e6,
        1 => 0x009c3ff379acfd021edc6424e1db4efeaf89dcc33454f3dc4ebe4cab8eedf6b9,
        2 => 0x03d4e5bd52271fa8f9e62faa88edbb7139e6d8c53f03da60245ec4630be6aa57,
        3 => 0x01d88ea9df444050fb2e8694c58bece0f152c7901b24e2c4814374e12dc88159,
        4 => 0x048d6171d793484bb412ad431d3fdd875dfd00cfa1e3e95d4e5d38bd0e4add50,
        _ => panic!("invalid blowup factor"),
    }
}

#[derive(Drop, Serde)]
pub struct CairoProof {
    pub claim: CairoClaim,
    pub interaction_pow: u64,
    pub interaction_claim: CairoInteractionClaim,
    pub stark_proof: StarkProof,
}

/// The output of a verification.
#[derive(Drop, Serde)]
pub struct VerificationOutput {
    pub program_hash: felt252,
    pub output: Array<felt252>,
}

/// Given a proof, returns the output of the verifier.
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

pub fn verify_cairo(proof: CairoProof) -> Result<(), CairoVerificationError> {
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

    if !channel.mix_and_check_pow_nonce(INTERACTION_POW_BITS, interaction_pow) {
        return Err(CairoVerificationError::InteractionProofOfWork);
    }

    let interaction_elements = CairoInteractionElementsImpl::draw(ref channel);

    if lookup_sum(@claim, @interaction_elements, @interaction_claim).is_non_zero() {
        return Result::Err(CairoVerificationError::InvalidLogupSum);
    }

    interaction_claim.mix_into(ref channel);
    commitment_scheme
        .commit(
            stark_proof.commitment_scheme_proof.commitments[2].clone(), *log_sizes[2], ref channel,
        );

    let cairo_air = CairoAirNewImpl::new(@claim, @interaction_elements, @interaction_claim);
    if let Result::Err(err) =
        verify(cairo_air, ref channel, stark_proof, commitment_scheme, SECURITY_BITS) {
        return Result::Err(CairoVerificationError::Stark(err));
    }

    Result::Ok(())
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

#[derive(Drop)]
struct RangeChecksInteractionElements {
    pub rc_6: RangeCheck_6Elements,
    pub rc_8: RangeCheck_8Elements,
    pub rc_11: RangeCheck_11Elements,
    pub rc_12: RangeCheck_12Elements,
    pub rc_18: RangeCheck_18Elements,
    pub rc_18_b: RangeCheck_18Elements,
    pub rc_19: RangeCheck_19Elements,
    pub rc_19_b: RangeCheck_19Elements,
    pub rc_19_c: RangeCheck_19Elements,
    pub rc_19_d: RangeCheck_19Elements,
    pub rc_19_e: RangeCheck_19Elements,
    pub rc_19_f: RangeCheck_19Elements,
    pub rc_19_g: RangeCheck_19Elements,
    pub rc_19_h: RangeCheck_19Elements,
    pub rc_4_3: RangeCheck_4_3Elements,
    pub rc_4_4: RangeCheck_4_4Elements,
    pub rc_5_4: RangeCheck_5_4Elements,
    pub rc_9_9: RangeCheck_9_9Elements,
    pub rc_9_9_b: RangeCheck_9_9Elements,
    pub rc_9_9_c: RangeCheck_9_9Elements,
    pub rc_9_9_d: RangeCheck_9_9Elements,
    pub rc_9_9_e: RangeCheck_9_9Elements,
    pub rc_9_9_f: RangeCheck_9_9Elements,
    pub rc_9_9_g: RangeCheck_9_9Elements,
    pub rc_9_9_h: RangeCheck_9_9Elements,
    pub rc_7_2_5: RangeCheck_7_2_5Elements,
    pub rc_3_6_6_3: RangeCheck_3_6_6_3Elements,
    pub rc_4_4_4_4: RangeCheck_4_4_4_4Elements,
    pub rc_3_3_3_3_3: RangeCheck_3_3_3_3_3Elements,
}

#[generate_trait]
impl RangeChecksInteractionElementsImpl of RangeChecksInteractionElementsTrait {
    fn draw(ref channel: Channel) -> RangeChecksInteractionElements {
        RangeChecksInteractionElements {
            rc_6: LookupElementsImpl::draw(ref channel),
            rc_8: LookupElementsImpl::draw(ref channel),
            rc_11: LookupElementsImpl::draw(ref channel),
            rc_12: LookupElementsImpl::draw(ref channel),
            rc_18: LookupElementsImpl::draw(ref channel),
            rc_18_b: LookupElementsImpl::draw(ref channel),
            rc_19: LookupElementsImpl::draw(ref channel),
            rc_19_b: LookupElementsImpl::draw(ref channel),
            rc_19_c: LookupElementsImpl::draw(ref channel),
            rc_19_d: LookupElementsImpl::draw(ref channel),
            rc_19_e: LookupElementsImpl::draw(ref channel),
            rc_19_f: LookupElementsImpl::draw(ref channel),
            rc_19_g: LookupElementsImpl::draw(ref channel),
            rc_19_h: LookupElementsImpl::draw(ref channel),
            rc_4_3: LookupElementsImpl::draw(ref channel),
            rc_4_4: LookupElementsImpl::draw(ref channel),
            rc_5_4: LookupElementsImpl::draw(ref channel),
            rc_9_9: LookupElementsImpl::draw(ref channel),
            rc_9_9_b: LookupElementsImpl::draw(ref channel),
            rc_9_9_c: LookupElementsImpl::draw(ref channel),
            rc_9_9_d: LookupElementsImpl::draw(ref channel),
            rc_9_9_e: LookupElementsImpl::draw(ref channel),
            rc_9_9_f: LookupElementsImpl::draw(ref channel),
            rc_9_9_g: LookupElementsImpl::draw(ref channel),
            rc_9_9_h: LookupElementsImpl::draw(ref channel),
            rc_7_2_5: LookupElementsImpl::draw(ref channel),
            rc_3_6_6_3: LookupElementsImpl::draw(ref channel),
            rc_4_4_4_4: LookupElementsImpl::draw(ref channel),
            rc_3_3_3_3_3: LookupElementsImpl::draw(ref channel),
        }
    }
}

#[derive(Drop)]
struct CairoInteractionElements {
    pub opcodes: OpcodesElements,
    pub verify_instruction: VerifyInstructionElements,
    pub blake_round: BlakeRoundElements,
    pub blake_g: BlakeGElements,
    pub blake_round_sigma: BlakeRoundSigmaElements,
    pub triple_xor_32: TripleXor32Elements,
    pub partial_ec_mul: PartialEcMulElements,
    pub pedersen_points_table: PedersenPointsTableElements,
    pub poseidon_full_round_chain: PoseidonFullRoundChainElements,
    pub poseidon_3_partial_rounds_chain: Poseidon3PartialRoundsChainElements,
    pub cube_252: Cube252Elements,
    pub poseidon_round_keys: PoseidonRoundKeysElements,
    pub range_check_felt_252_width_27: RangeCheckFelt252Width27Elements,
    pub memory_address_to_id: MemoryAddressToIdElements,
    pub memory_id_to_value: MemoryIdToBigElements,
    pub range_checks: RangeChecksInteractionElements,
    pub verify_bitwise_xor_4: VerifyBitwiseXor_4Elements,
    pub verify_bitwise_xor_7: VerifyBitwiseXor_7Elements,
    pub verify_bitwise_xor_8: VerifyBitwiseXor_8Elements,
    pub verify_bitwise_xor_9: VerifyBitwiseXor_9Elements,
    pub verify_bitwise_xor_12: VerifyBitwiseXor_12Elements,
}

#[generate_trait]
impl CairoInteractionElementsImpl of CairoInteractionElementsTrait {
    fn draw(ref channel: Channel) -> CairoInteractionElements {
        CairoInteractionElements {
            opcodes: LookupElementsImpl::draw(ref channel),
            verify_instruction: LookupElementsImpl::draw(ref channel),
            blake_round: LookupElementsImpl::draw(ref channel),
            blake_g: LookupElementsImpl::draw(ref channel),
            blake_round_sigma: LookupElementsImpl::draw(ref channel),
            triple_xor_32: LookupElementsImpl::draw(ref channel),
            poseidon_3_partial_rounds_chain: LookupElementsImpl::draw(ref channel),
            poseidon_full_round_chain: LookupElementsImpl::draw(ref channel),
            cube_252: LookupElementsImpl::draw(ref channel),
            poseidon_round_keys: LookupElementsImpl::draw(ref channel),
            range_check_felt_252_width_27: LookupElementsImpl::draw(ref channel),
            partial_ec_mul: LookupElementsImpl::draw(ref channel),
            pedersen_points_table: LookupElementsImpl::draw(ref channel),
            memory_address_to_id: LookupElementsImpl::draw(ref channel),
            memory_id_to_value: LookupElementsImpl::draw(ref channel),
            range_checks: RangeChecksInteractionElementsImpl::draw(ref channel),
            verify_bitwise_xor_4: LookupElementsImpl::draw(ref channel),
            verify_bitwise_xor_7: LookupElementsImpl::draw(ref channel),
            verify_bitwise_xor_8: LookupElementsImpl::draw(ref channel),
            verify_bitwise_xor_9: LookupElementsImpl::draw(ref channel),
            verify_bitwise_xor_12: LookupElementsImpl::draw(ref channel),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Claim {
    pub log_size: u32,
    pub range_check_builtin_segment_start: u32,
}

#[derive(Drop, Serde, Copy)]
pub struct BuiltinsClaim {
    pub add_mod_builtin: Option<components::add_mod_builtin::Claim>,
    pub bitwise_builtin: Option<components::bitwise_builtin::Claim>,
    pub mul_mod_builtin: Option<components::mul_mod_builtin::Claim>,
    pub pedersen_builtin: Option<components::pedersen_builtin::Claim>,
    pub poseidon_builtin: Option<components::poseidon_builtin::Claim>,
    pub range_check_96_builtin: Option<components::range_check_builtin_bits_96::Claim>,
    pub range_check_128_builtin: Option<components::range_check_builtin_bits_128::Claim>,
}

#[generate_trait]
impl BuiltinsClaimImpl of BuiltinsClaimTrait {
    fn mix_into(self: @BuiltinsClaim, ref channel: Channel) {
        if let Some(claim) = self.add_mod_builtin {
            claim.mix_into(ref channel);
        }
        if let Some(claim) = self.bitwise_builtin {
            claim.mix_into(ref channel);
        }
        if let Some(claim) = self.mul_mod_builtin {
            claim.mix_into(ref channel);
        }
        if let Some(claim) = self.pedersen_builtin {
            claim.mix_into(ref channel);
        }
        if let Some(claim) = self.poseidon_builtin {
            claim.mix_into(ref channel);
        }
        if let Some(claim) = self.range_check_96_builtin {
            claim.mix_into(ref channel);
        }
        if let Some(claim) = self.range_check_128_builtin {
            claim.mix_into(ref channel);
        }
    }

    fn log_sizes(self: @BuiltinsClaim) -> TreeArray<Span<u32>> {
        let mut log_sizes = array![];

        if let Some(claim) = self.add_mod_builtin {
            log_sizes.append(claim.log_sizes());
        }

        if let Some(claim) = self.bitwise_builtin {
            log_sizes.append(claim.log_sizes());
        }

        if let Some(claim) = self.mul_mod_builtin {
            log_sizes.append(claim.log_sizes());
        }

        if let Some(claim) = self.pedersen_builtin {
            log_sizes.append(claim.log_sizes());
        }

        if let Some(claim) = self.poseidon_builtin {
            log_sizes.append(claim.log_sizes());
        }

        if let Some(claim) = self.range_check_96_builtin {
            log_sizes.append(claim.log_sizes());
        }

        if let Some(claim) = self.range_check_128_builtin {
            log_sizes.append(claim.log_sizes());
        }

        utils::tree_array_concat_cols(log_sizes)
    }

    fn accumulate_relation_uses(self: @BuiltinsClaim, ref relation_uses: RelationUsesDict) {
        let BuiltinsClaim {
            add_mod_builtin,
            bitwise_builtin,
            mul_mod_builtin,
            pedersen_builtin,
            poseidon_builtin,
            range_check_96_builtin,
            range_check_128_builtin,
        } = self;

        if let Some(claim) = add_mod_builtin {
            accumulate_relation_uses(
                ref relation_uses,
                components::add_mod_builtin::RELATION_USES_PER_ROW.span(),
                *claim.log_size,
            );
        }
        if let Some(claim) = bitwise_builtin {
            accumulate_relation_uses(
                ref relation_uses,
                components::bitwise_builtin::RELATION_USES_PER_ROW.span(),
                *claim.log_size,
            );
        }
        if let Some(claim) = mul_mod_builtin {
            accumulate_relation_uses(
                ref relation_uses,
                components::mul_mod_builtin::RELATION_USES_PER_ROW.span(),
                *claim.log_size,
            );
        }
        if let Some(claim) = pedersen_builtin {
            accumulate_relation_uses(
                ref relation_uses,
                components::pedersen_builtin::RELATION_USES_PER_ROW.span(),
                *claim.log_size,
            );
        }
        if let Some(claim) = poseidon_builtin {
            accumulate_relation_uses(
                ref relation_uses,
                components::poseidon_builtin::RELATION_USES_PER_ROW.span(),
                *claim.log_size,
            );
        }
        if let Some(claim) = range_check_96_builtin {
            accumulate_relation_uses(
                ref relation_uses,
                components::range_check_builtin_bits_96::RELATION_USES_PER_ROW.span(),
                *claim.log_size,
            );
        }
        if let Some(claim) = range_check_128_builtin {
            accumulate_relation_uses(
                ref relation_uses,
                components::range_check_builtin_bits_128::RELATION_USES_PER_ROW.span(),
                *claim.log_size,
            );
        }
    }
}

#[derive(Drop, Serde, Copy)]
pub struct BuiltinsInteractionClaim {
    pub add_mod_builtin: Option<components::add_mod_builtin::InteractionClaim>,
    pub bitwise_builtin: Option<components::bitwise_builtin::InteractionClaim>,
    pub mul_mod_builtin: Option<components::mul_mod_builtin::InteractionClaim>,
    pub pedersen_builtin: Option<components::pedersen_builtin::InteractionClaim>,
    pub poseidon_builtin: Option<components::poseidon_builtin::InteractionClaim>,
    pub range_check_96_builtin: Option<components::range_check_builtin_bits_96::InteractionClaim>,
    pub range_check_128_builtin: Option<components::range_check_builtin_bits_128::InteractionClaim>,
}

#[generate_trait]
impl BuiltinsInteractionClaimImpl of BuiltinsInteractionClaimTrait {
    fn mix_into(self: @BuiltinsInteractionClaim, ref channel: Channel) {
        if let Some(claim) = self.add_mod_builtin {
            claim.mix_into(ref channel);
        }
        if let Some(claim) = self.bitwise_builtin {
            claim.mix_into(ref channel);
        }
        if let Some(claim) = self.mul_mod_builtin {
            claim.mix_into(ref channel);
        }
        if let Some(claim) = self.pedersen_builtin {
            claim.mix_into(ref channel);
        }
        if let Some(claim) = self.poseidon_builtin {
            claim.mix_into(ref channel);
        }
        if let Some(claim) = self.range_check_96_builtin {
            claim.mix_into(ref channel);
        }
        if let Some(claim) = self.range_check_128_builtin {
            claim.mix_into(ref channel);
        }
    }

    fn sum(self: @BuiltinsInteractionClaim) -> QM31 {
        let mut sum = Zero::zero();

        if let Some(claim) = self.add_mod_builtin {
            sum += *claim.claimed_sum;
        }

        if let Some(claim) = self.bitwise_builtin {
            sum += *claim.claimed_sum;
        }

        if let Some(claim) = self.mul_mod_builtin {
            sum += *claim.claimed_sum;
        }

        if let Some(claim) = self.pedersen_builtin {
            sum += *claim.claimed_sum;
        }

        if let Some(claim) = self.poseidon_builtin {
            sum += *claim.claimed_sum;
        }

        if let Some(claim) = self.range_check_96_builtin {
            sum += *claim.claimed_sum;
        }

        if let Some(claim) = self.range_check_128_builtin {
            sum += *claim.claimed_sum;
        }

        sum
    }
}

#[derive(Drop, Serde, Clone)]
pub struct RangeChecksClaim {
    pub rc_6: components::range_check_6::Claim,
    pub rc_8: components::range_check_8::Claim,
    pub rc_11: components::range_check_11::Claim,
    pub rc_12: components::range_check_12::Claim,
    pub rc_18: components::range_check_18::Claim,
    pub rc_18_b: components::range_check_18_b::Claim,
    pub rc_19: components::range_check_19::Claim,
    pub rc_19_b: components::range_check_19_b::Claim,
    pub rc_19_c: components::range_check_19_c::Claim,
    pub rc_19_d: components::range_check_19_d::Claim,
    pub rc_19_e: components::range_check_19_e::Claim,
    pub rc_19_f: components::range_check_19_f::Claim,
    pub rc_19_g: components::range_check_19_g::Claim,
    pub rc_19_h: components::range_check_19_h::Claim,
    pub rc_4_3: components::range_check_4_3::Claim,
    pub rc_4_4: components::range_check_4_4::Claim,
    pub rc_5_4: components::range_check_5_4::Claim,
    pub rc_9_9: components::range_check_9_9::Claim,
    pub rc_9_9_b: components::range_check_9_9_b::Claim,
    pub rc_9_9_c: components::range_check_9_9_c::Claim,
    pub rc_9_9_d: components::range_check_9_9_d::Claim,
    pub rc_9_9_e: components::range_check_9_9_e::Claim,
    pub rc_9_9_f: components::range_check_9_9_f::Claim,
    pub rc_9_9_g: components::range_check_9_9_g::Claim,
    pub rc_9_9_h: components::range_check_9_9_h::Claim,
    pub rc_7_2_5: components::range_check_7_2_5::Claim,
    pub rc_3_6_6_3: components::range_check_3_6_6_3::Claim,
    pub rc_4_4_4_4: components::range_check_4_4_4_4::Claim,
    pub rc_3_3_3_3_3: components::range_check_3_3_3_3_3::Claim,
}

#[generate_trait]
impl RangeChecksClaimImpl of RangeChecksClaimTrait {
    fn mix_into(self: @RangeChecksClaim, ref channel: Channel) {
        self.rc_6.mix_into(ref channel);
        self.rc_8.mix_into(ref channel);
        self.rc_11.mix_into(ref channel);
        self.rc_12.mix_into(ref channel);
        self.rc_18.mix_into(ref channel);
        self.rc_18_b.mix_into(ref channel);
        self.rc_19.mix_into(ref channel);
        self.rc_19_b.mix_into(ref channel);
        self.rc_19_c.mix_into(ref channel);
        self.rc_19_d.mix_into(ref channel);
        self.rc_19_e.mix_into(ref channel);
        self.rc_19_f.mix_into(ref channel);
        self.rc_19_g.mix_into(ref channel);
        self.rc_19_h.mix_into(ref channel);
        self.rc_4_3.mix_into(ref channel);
        self.rc_4_4.mix_into(ref channel);
        self.rc_5_4.mix_into(ref channel);
        self.rc_9_9.mix_into(ref channel);
        self.rc_9_9_b.mix_into(ref channel);
        self.rc_9_9_c.mix_into(ref channel);
        self.rc_9_9_d.mix_into(ref channel);
        self.rc_9_9_e.mix_into(ref channel);
        self.rc_9_9_f.mix_into(ref channel);
        self.rc_9_9_g.mix_into(ref channel);
        self.rc_9_9_h.mix_into(ref channel);
        self.rc_7_2_5.mix_into(ref channel);
        self.rc_3_6_6_3.mix_into(ref channel);
        self.rc_4_4_4_4.mix_into(ref channel);
        self.rc_3_3_3_3_3.mix_into(ref channel);
    }

    fn log_sizes(self: @RangeChecksClaim) -> TreeArray<Span<u32>> {
        utils::tree_array_concat_cols(
            array![
                self.rc_6.log_sizes(), self.rc_8.log_sizes(), self.rc_11.log_sizes(),
                self.rc_12.log_sizes(), self.rc_18.log_sizes(), self.rc_18_b.log_sizes(),
                self.rc_19.log_sizes(), self.rc_19_b.log_sizes(), self.rc_19_c.log_sizes(),
                self.rc_19_d.log_sizes(), self.rc_19_e.log_sizes(), self.rc_19_f.log_sizes(),
                self.rc_19_g.log_sizes(), self.rc_19_h.log_sizes(), self.rc_4_3.log_sizes(),
                self.rc_4_4.log_sizes(), self.rc_5_4.log_sizes(), self.rc_9_9.log_sizes(),
                self.rc_9_9_b.log_sizes(), self.rc_9_9_c.log_sizes(), self.rc_9_9_d.log_sizes(),
                self.rc_9_9_e.log_sizes(), self.rc_9_9_f.log_sizes(), self.rc_9_9_g.log_sizes(),
                self.rc_9_9_h.log_sizes(), self.rc_7_2_5.log_sizes(), self.rc_3_6_6_3.log_sizes(),
                self.rc_4_4_4_4.log_sizes(), self.rc_3_3_3_3_3.log_sizes(),
            ],
        )
    }
}


#[derive(Drop, Serde, Clone)]
pub struct RangeChecksInteractionClaim {
    pub rc_6: components::range_check_6::InteractionClaim,
    pub rc_8: components::range_check_8::InteractionClaim,
    pub rc_11: components::range_check_11::InteractionClaim,
    pub rc_12: components::range_check_12::InteractionClaim,
    pub rc_18: components::range_check_18::InteractionClaim,
    pub rc_18_b: components::range_check_18_b::InteractionClaim,
    pub rc_19: components::range_check_19::InteractionClaim,
    pub rc_19_b: components::range_check_19_b::InteractionClaim,
    pub rc_19_c: components::range_check_19_c::InteractionClaim,
    pub rc_19_d: components::range_check_19_d::InteractionClaim,
    pub rc_19_e: components::range_check_19_e::InteractionClaim,
    pub rc_19_f: components::range_check_19_f::InteractionClaim,
    pub rc_19_g: components::range_check_19_g::InteractionClaim,
    pub rc_19_h: components::range_check_19_h::InteractionClaim,
    pub rc_4_3: components::range_check_4_3::InteractionClaim,
    pub rc_4_4: components::range_check_4_4::InteractionClaim,
    pub rc_5_4: components::range_check_5_4::InteractionClaim,
    pub rc_9_9: components::range_check_9_9::InteractionClaim,
    pub rc_9_9_b: components::range_check_9_9_b::InteractionClaim,
    pub rc_9_9_c: components::range_check_9_9_c::InteractionClaim,
    pub rc_9_9_d: components::range_check_9_9_d::InteractionClaim,
    pub rc_9_9_e: components::range_check_9_9_e::InteractionClaim,
    pub rc_9_9_f: components::range_check_9_9_f::InteractionClaim,
    pub rc_9_9_g: components::range_check_9_9_g::InteractionClaim,
    pub rc_9_9_h: components::range_check_9_9_h::InteractionClaim,
    pub rc_7_2_5: components::range_check_7_2_5::InteractionClaim,
    pub rc_3_6_6_3: components::range_check_3_6_6_3::InteractionClaim,
    pub rc_4_4_4_4: components::range_check_4_4_4_4::InteractionClaim,
    pub rc_3_3_3_3_3: components::range_check_3_3_3_3_3::InteractionClaim,
}

#[generate_trait]
impl RangeChecksInteractionClaimImpl of RangeChecksInteractionClaimTrait {
    fn mix_into(self: @RangeChecksInteractionClaim, ref channel: Channel) {
        self.rc_6.mix_into(ref channel);
        self.rc_8.mix_into(ref channel);
        self.rc_11.mix_into(ref channel);
        self.rc_12.mix_into(ref channel);
        self.rc_18.mix_into(ref channel);
        self.rc_18_b.mix_into(ref channel);
        self.rc_19.mix_into(ref channel);
        self.rc_19_b.mix_into(ref channel);
        self.rc_19_c.mix_into(ref channel);
        self.rc_19_d.mix_into(ref channel);
        self.rc_19_e.mix_into(ref channel);
        self.rc_19_f.mix_into(ref channel);
        self.rc_19_g.mix_into(ref channel);
        self.rc_19_h.mix_into(ref channel);
        self.rc_4_3.mix_into(ref channel);
        self.rc_4_4.mix_into(ref channel);
        self.rc_5_4.mix_into(ref channel);
        self.rc_9_9.mix_into(ref channel);
        self.rc_9_9_b.mix_into(ref channel);
        self.rc_9_9_c.mix_into(ref channel);
        self.rc_9_9_d.mix_into(ref channel);
        self.rc_9_9_e.mix_into(ref channel);
        self.rc_9_9_f.mix_into(ref channel);
        self.rc_9_9_g.mix_into(ref channel);
        self.rc_9_9_h.mix_into(ref channel);
        self.rc_7_2_5.mix_into(ref channel);
        self.rc_3_6_6_3.mix_into(ref channel);
        self.rc_4_4_4_4.mix_into(ref channel);
        self.rc_3_3_3_3_3.mix_into(ref channel);
    }

    fn sum(self: @RangeChecksInteractionClaim) -> QM31 {
        let mut sum = Zero::zero();
        sum += *self.rc_6.claimed_sum;
        sum += *self.rc_8.claimed_sum;
        sum += *self.rc_11.claimed_sum;
        sum += *self.rc_12.claimed_sum;
        sum += *self.rc_18.claimed_sum;
        sum += *self.rc_18_b.claimed_sum;
        sum += *self.rc_19.claimed_sum;
        sum += *self.rc_19_b.claimed_sum;
        sum += *self.rc_19_c.claimed_sum;
        sum += *self.rc_19_d.claimed_sum;
        sum += *self.rc_19_e.claimed_sum;
        sum += *self.rc_19_f.claimed_sum;
        sum += *self.rc_19_g.claimed_sum;
        sum += *self.rc_19_h.claimed_sum;
        sum += *self.rc_4_3.claimed_sum;
        sum += *self.rc_4_4.claimed_sum;
        sum += *self.rc_5_4.claimed_sum;
        sum += *self.rc_9_9.claimed_sum;
        sum += *self.rc_9_9_b.claimed_sum;
        sum += *self.rc_9_9_c.claimed_sum;
        sum += *self.rc_9_9_d.claimed_sum;
        sum += *self.rc_9_9_e.claimed_sum;
        sum += *self.rc_9_9_f.claimed_sum;
        sum += *self.rc_9_9_g.claimed_sum;
        sum += *self.rc_9_9_h.claimed_sum;
        sum += *self.rc_7_2_5.claimed_sum;
        sum += *self.rc_3_6_6_3.claimed_sum;
        sum += *self.rc_4_4_4_4.claimed_sum;
        sum += *self.rc_3_3_3_3_3.claimed_sum;
        sum
    }
}

#[derive(Drop, Serde)]
struct PedersenClaim {
    pub partial_ec_mul: components::partial_ec_mul::Claim,
    pub pedersen_points_table: components::pedersen_points_table::Claim,
}

#[generate_trait]
impl PedersenClaimImpl of PedersenClaimTrait {
    fn mix_into(self: @PedersenClaim, ref channel: Channel) {
        self.partial_ec_mul.mix_into(ref channel);
        self.pedersen_points_table.mix_into(ref channel);
    }

    fn log_sizes(self: @PedersenClaim) -> TreeArray<Span<u32>> {
        utils::tree_array_concat_cols(
            array![self.partial_ec_mul.log_sizes(), self.pedersen_points_table.log_sizes()],
        )
    }

    fn accumulate_relation_uses(self: @PedersenClaim, ref relation_uses: RelationUsesDict) {
        let PedersenClaim { partial_ec_mul, pedersen_points_table: _ } = self;

        // NOTE: The following components do not USE relations:
        // - pedersen_points_table

        accumulate_relation_uses(
            ref relation_uses,
            components::partial_ec_mul::RELATION_USES_PER_ROW.span(),
            *partial_ec_mul.log_size,
        );
    }
}

#[derive(Drop, Serde)]
struct PedersenInteractionClaim {
    pub partial_ec_mul: components::partial_ec_mul::InteractionClaim,
    pub pedersen_points_table: components::pedersen_points_table::InteractionClaim,
}

#[generate_trait]
impl PedersenInteractionClaimImpl of PedersenInteractionClaimTrait {
    fn mix_into(self: @PedersenInteractionClaim, ref channel: Channel) {
        self.partial_ec_mul.mix_into(ref channel);
        self.pedersen_points_table.mix_into(ref channel);
    }

    fn sum(self: @PedersenInteractionClaim) -> QM31 {
        let mut sum = Zero::zero();
        sum += *self.partial_ec_mul.claimed_sum;
        sum += *self.pedersen_points_table.claimed_sum;
        sum
    }
}

#[derive(Drop, Serde)]
struct PedersenContextClaim {
    pub claim: Option<PedersenClaim>,
}

#[generate_trait]
impl PedersenContextClaimImpl of PedersenContextClaimTrait {
    fn mix_into(self: @PedersenContextClaim, ref channel: Channel) {
        if let Option::Some(claim) = self.claim {
            claim.mix_into(ref channel);
        }
    }

    fn log_sizes(self: @PedersenContextClaim) -> TreeArray<Span<u32>> {
        if let Option::Some(claim) = self.claim {
            claim.log_sizes()
        } else {
            array![]
        }
    }

    fn accumulate_relation_uses(self: @PedersenContextClaim, ref relation_uses: RelationUsesDict) {
        if let Some(claim) = self.claim {
            claim.accumulate_relation_uses(ref relation_uses);
        }
    }
}

#[derive(Drop, Serde)]
struct PedersenContextInteractionClaim {
    pub interaction_claim: Option<PedersenInteractionClaim>,
}

#[generate_trait]
impl PedersenContextInteractionClaimImpl of PedersenContextInteractionClaimTrait {
    fn mix_into(self: @PedersenContextInteractionClaim, ref channel: Channel) {
        if let Some(interaction_claim) = self.interaction_claim {
            interaction_claim.mix_into(ref channel);
        }
    }

    fn sum(self: @PedersenContextInteractionClaim) -> QM31 {
        if let Some(interaction_claim) = self.interaction_claim {
            interaction_claim.sum()
        } else {
            Zero::zero()
        }
    }
}

#[derive(Drop, Serde)]
struct PoseidonClaim {
    pub poseidon_3_partial_rounds_chain: components::poseidon_3_partial_rounds_chain::Claim,
    pub poseidon_full_round_chain: components::poseidon_full_round_chain::Claim,
    pub cube_252: components::cube_252::Claim,
    pub poseidon_round_keys: components::poseidon_round_keys::Claim,
    pub range_check_felt_252_width_27: components::range_check_felt_252_width_27::Claim,
}

#[generate_trait]
impl PoseidonClaimImpl of PoseidonClaimTrait {
    fn mix_into(self: @PoseidonClaim, ref channel: Channel) {
        self.poseidon_3_partial_rounds_chain.mix_into(ref channel);
        self.poseidon_full_round_chain.mix_into(ref channel);
        self.cube_252.mix_into(ref channel);
        self.poseidon_round_keys.mix_into(ref channel);
        self.range_check_felt_252_width_27.mix_into(ref channel);
    }

    fn log_sizes(self: @PoseidonClaim) -> TreeArray<Span<u32>> {
        utils::tree_array_concat_cols(
            array![
                self.poseidon_3_partial_rounds_chain.log_sizes(),
                self.poseidon_full_round_chain.log_sizes(), self.cube_252.log_sizes(),
                self.poseidon_round_keys.log_sizes(),
                self.range_check_felt_252_width_27.log_sizes(),
            ],
        )
    }

    fn accumulate_relation_uses(self: @PoseidonClaim, ref relation_uses: RelationUsesDict) {
        let PoseidonClaim {
            poseidon_3_partial_rounds_chain,
            poseidon_full_round_chain,
            cube_252,
            poseidon_round_keys: _,
            range_check_felt_252_width_27,
        } = self;

        // NOTE: The following components do not USE relations:
        // - poseidon_round_keys

        accumulate_relation_uses(
            ref relation_uses,
            components::poseidon_3_partial_rounds_chain::RELATION_USES_PER_ROW.span(),
            *poseidon_3_partial_rounds_chain.log_size,
        );
        accumulate_relation_uses(
            ref relation_uses,
            components::poseidon_full_round_chain::RELATION_USES_PER_ROW.span(),
            *poseidon_full_round_chain.log_size,
        );
        accumulate_relation_uses(
            ref relation_uses,
            components::cube_252::RELATION_USES_PER_ROW.span(),
            *cube_252.log_size,
        );
        accumulate_relation_uses(
            ref relation_uses,
            components::range_check_felt_252_width_27::RELATION_USES_PER_ROW.span(),
            *range_check_felt_252_width_27.log_size,
        );
    }
}

#[derive(Drop, Serde)]
struct PoseidonInteractionClaim {
    pub poseidon_3_partial_rounds_chain: components::poseidon_3_partial_rounds_chain::InteractionClaim,
    pub poseidon_full_round_chain: components::poseidon_full_round_chain::InteractionClaim,
    pub cube_252: components::cube_252::InteractionClaim,
    pub poseidon_round_keys: components::poseidon_round_keys::InteractionClaim,
    pub range_check_felt_252_width_27: components::range_check_felt_252_width_27::InteractionClaim,
}

#[generate_trait]
impl PoseidonInteractionClaimImpl of PoseidonInteractionClaimTrait {
    fn mix_into(self: @PoseidonInteractionClaim, ref channel: Channel) {
        self.poseidon_3_partial_rounds_chain.mix_into(ref channel);
        self.poseidon_full_round_chain.mix_into(ref channel);
        self.cube_252.mix_into(ref channel);
        self.poseidon_round_keys.mix_into(ref channel);
        self.range_check_felt_252_width_27.mix_into(ref channel);
    }

    fn sum(self: @PoseidonInteractionClaim) -> QM31 {
        let mut sum = Zero::zero();
        sum += *self.poseidon_3_partial_rounds_chain.claimed_sum;
        sum += *self.poseidon_full_round_chain.claimed_sum;
        sum += *self.cube_252.claimed_sum;
        sum += *self.poseidon_round_keys.claimed_sum;
        sum += *self.range_check_felt_252_width_27.claimed_sum;
        sum
    }
}

#[derive(Drop, Serde)]
struct PoseidonContextClaim {
    pub claim: Option<PoseidonClaim>,
}

#[generate_trait]
impl PoseidonContextClaimImpl of PoseidonContextClaimTrait {
    fn mix_into(self: @PoseidonContextClaim, ref channel: Channel) {
        if let Option::Some(claim) = self.claim {
            claim.mix_into(ref channel);
        }
    }

    fn log_sizes(self: @PoseidonContextClaim) -> TreeArray<Span<u32>> {
        if let Option::Some(claim) = self.claim {
            claim.log_sizes()
        } else {
            array![]
        }
    }

    fn accumulate_relation_uses(self: @PoseidonContextClaim, ref relation_uses: RelationUsesDict) {
        if let Some(claim) = self.claim {
            claim.accumulate_relation_uses(ref relation_uses);
        }
    }
}

#[derive(Drop, Serde)]
struct PoseidonContextInteractionClaim {
    pub interaction_claim: Option<PoseidonInteractionClaim>,
}

#[generate_trait]
impl PoseidonContextInteractionClaimImpl of PoseidonContextInteractionClaimTrait {
    fn mix_into(self: @PoseidonContextInteractionClaim, ref channel: Channel) {
        if let Some(interaction_claim) = self.interaction_claim {
            interaction_claim.mix_into(ref channel);
        }
    }

    fn sum(self: @PoseidonContextInteractionClaim) -> QM31 {
        if let Some(interaction_claim) = self.interaction_claim {
            interaction_claim.sum()
        } else {
            Zero::zero()
        }
    }
}

#[derive(Drop, Serde)]
struct BlakeClaim {
    pub blake_round: components::blake_round::Claim,
    pub blake_g: components::blake_g::Claim,
    pub blake_round_sigma: components::blake_round_sigma::Claim,
    pub triple_xor_32: components::triple_xor_32::Claim,
    pub verify_bitwise_xor_12: components::verify_bitwise_xor_12::Claim,
}

#[generate_trait]
impl BlakeClaimImpl of BlakeClaimTrait {
    fn mix_into(self: @BlakeClaim, ref channel: Channel) {
        self.blake_round.mix_into(ref channel);
        self.blake_g.mix_into(ref channel);
        self.blake_round_sigma.mix_into(ref channel);
        self.triple_xor_32.mix_into(ref channel);
        self.verify_bitwise_xor_12.mix_into(ref channel);
    }

    fn log_sizes(self: @BlakeClaim) -> TreeArray<Span<u32>> {
        utils::tree_array_concat_cols(
            array![
                self.blake_round.log_sizes(), self.blake_g.log_sizes(),
                self.blake_round_sigma.log_sizes(), self.triple_xor_32.log_sizes(),
                self.verify_bitwise_xor_12.log_sizes(),
            ],
        )
    }

    fn accumulate_relation_uses(self: @BlakeClaim, ref relation_uses: RelationUsesDict) {
        let BlakeClaim {
            blake_round, blake_g, blake_round_sigma: _, triple_xor_32, verify_bitwise_xor_12: _,
        } = self;
        // NOTE: The following components do not USE relations:
        // - blake_sigma
        // - verify_bitwise_xor_12

        accumulate_relation_uses(
            ref relation_uses,
            components::blake_round::RELATION_USES_PER_ROW.span(),
            *blake_round.log_size,
        );
        accumulate_relation_uses(
            ref relation_uses, components::blake_g::RELATION_USES_PER_ROW.span(), *blake_g.log_size,
        );
        accumulate_relation_uses(
            ref relation_uses,
            components::triple_xor_32::RELATION_USES_PER_ROW.span(),
            *triple_xor_32.log_size,
        );
    }
}

#[derive(Drop, Serde)]
struct BlakeInteractionClaim {
    pub blake_round: components::blake_round::InteractionClaim,
    pub blake_g: components::blake_g::InteractionClaim,
    pub blake_round_sigma: components::blake_round_sigma::InteractionClaim,
    pub triple_xor_32: components::triple_xor_32::InteractionClaim,
    pub verify_bitwise_xor_12: components::verify_bitwise_xor_12::InteractionClaim,
}

#[generate_trait]
impl BlakeInteractionClaimImpl of BlakeInteractionClaimTrait {
    fn mix_into(self: @BlakeInteractionClaim, ref channel: Channel) {
        self.blake_round.mix_into(ref channel);
        self.blake_g.mix_into(ref channel);
        self.blake_round_sigma.mix_into(ref channel);
        self.triple_xor_32.mix_into(ref channel);
        self.verify_bitwise_xor_12.mix_into(ref channel);
    }

    fn sum(self: @BlakeInteractionClaim) -> QM31 {
        let mut sum = Zero::zero();
        sum += *self.blake_round.claimed_sum;
        sum += *self.blake_g.claimed_sum;
        sum += *self.blake_round_sigma.claimed_sum;
        sum += *self.triple_xor_32.claimed_sum;
        sum += *self.verify_bitwise_xor_12.claimed_sum;
        sum
    }
}

#[derive(Drop, Serde)]
struct BlakeContextClaim {
    pub claim: Option<BlakeClaim>,
}

#[generate_trait]
impl BlakeContextClaimImpl of BlakeContextClaimTrait {
    fn mix_into(self: @BlakeContextClaim, ref channel: Channel) {
        if let Some(claim) = self.claim {
            claim.mix_into(ref channel);
        }
    }

    fn log_sizes(self: @BlakeContextClaim) -> TreeArray<Span<u32>> {
        if let Some(claim) = self.claim {
            claim.log_sizes()
        } else {
            array![]
        }
    }

    fn accumulate_relation_uses(self: @BlakeContextClaim, ref relation_uses: RelationUsesDict) {
        if let Some(claim) = self.claim {
            claim.accumulate_relation_uses(ref relation_uses);
        }
    }
}

#[derive(Drop, Serde)]
pub struct CairoClaim {
    pub public_data: PublicData,
    pub opcodes: OpcodeClaim,
    pub verify_instruction: components::verify_instruction::Claim,
    pub blake_context: BlakeContextClaim,
    pub builtins: BuiltinsClaim,
    pub pedersen_context: PedersenContextClaim,
    pub poseidon_context: PoseidonContextClaim,
    pub memory_address_to_id: components::memory_address_to_id::Claim,
    pub memory_id_to_value: components::memory_id_to_big::Claim,
    pub range_checks: RangeChecksClaim,
    pub verify_bitwise_xor_4: components::verify_bitwise_xor_4::Claim,
    pub verify_bitwise_xor_7: components::verify_bitwise_xor_7::Claim,
    pub verify_bitwise_xor_8: components::verify_bitwise_xor_8::Claim,
    pub verify_bitwise_xor_9: components::verify_bitwise_xor_9::Claim,
    // ...
}

#[generate_trait]
impl CairoClaimImpl of CairoClaimTrait {
    fn log_sizes(self: @CairoClaim) -> TreeArray<Span<u32>> {
        let mut aggregated_log_sizes = utils::tree_array_concat_cols(
            array![
                self.opcodes.log_sizes(), self.verify_instruction.log_sizes(),
                self.blake_context.log_sizes(), self.builtins.log_sizes(),
                self.pedersen_context.log_sizes(), self.poseidon_context.log_sizes(),
                self.memory_address_to_id.log_sizes(), self.memory_id_to_value.log_sizes(),
                self.range_checks.log_sizes(), self.verify_bitwise_xor_4.log_sizes(),
                self.verify_bitwise_xor_7.log_sizes(), self.verify_bitwise_xor_8.log_sizes(),
                self.verify_bitwise_xor_9.log_sizes(),
            ],
        );

        // Overwrite the preprocessed trace log sizes.
        let _invalid_preprocessed_trace_log_sizes = aggregated_log_sizes.pop_front();

        let mut preprocessed_trace_log_sizes = array![];

        for preprocessed_column in PREPROCESSED_COLUMNS.span() {
            preprocessed_trace_log_sizes.append(preprocessed_column.log_size());
        }

        let trace_log_sizes = aggregated_log_sizes.pop_front().unwrap();
        let interaction_log_sizes = aggregated_log_sizes.pop_front().unwrap();
        assert!(aggregated_log_sizes.is_empty());

        array![preprocessed_trace_log_sizes.span(), trace_log_sizes, interaction_log_sizes]
    }

    fn mix_into(self: @CairoClaim, ref channel: Channel) {
        self.public_data.mix_into(ref channel);
        self.opcodes.mix_into(ref channel);
        self.verify_instruction.mix_into(ref channel);
        self.blake_context.mix_into(ref channel);
        self.builtins.mix_into(ref channel);
        self.pedersen_context.mix_into(ref channel);
        self.poseidon_context.mix_into(ref channel);
        self.memory_address_to_id.mix_into(ref channel);
        self.memory_id_to_value.mix_into(ref channel);
        self.range_checks.mix_into(ref channel);
        self.verify_bitwise_xor_4.mix_into(ref channel);
        self.verify_bitwise_xor_7.mix_into(ref channel);
        self.verify_bitwise_xor_8.mix_into(ref channel);
        self.verify_bitwise_xor_9.mix_into(ref channel);
    }

    fn accumulate_relation_uses(self: @CairoClaim, ref relation_uses: RelationUsesDict) {
        let CairoClaim {
            public_data: _,
            opcodes,
            verify_instruction,
            blake_context,
            builtins,
            pedersen_context,
            poseidon_context,
            memory_address_to_id: _,
            memory_id_to_value,
            range_checks: _,
            verify_bitwise_xor_4: _,
            verify_bitwise_xor_7: _,
            verify_bitwise_xor_8: _,
            verify_bitwise_xor_9: _,
        } = self;
        // NOTE: The following components do not USE relations:
        // - range_checks
        // - verify_bitwise_xor_*
        // - memory_address_to_id

        opcodes.accumulate_relation_uses(ref relation_uses);
        blake_context.accumulate_relation_uses(ref relation_uses);
        builtins.accumulate_relation_uses(ref relation_uses);
        pedersen_context.accumulate_relation_uses(ref relation_uses);
        poseidon_context.accumulate_relation_uses(ref relation_uses);

        accumulate_relation_uses(
            ref relation_uses,
            components::verify_instruction::RELATION_USES_PER_ROW.span(),
            *verify_instruction.log_size,
        );
        for log_size in memory_id_to_value.big_log_sizes.span() {
            accumulate_relation_uses(
                ref relation_uses,
                components::memory_id_to_big::RELATION_USES_PER_ROW_BIG.span(),
                *log_size,
            );
        }
        accumulate_relation_uses(
            ref relation_uses,
            components::memory_id_to_big::RELATION_USES_PER_ROW_SMALL.span(),
            *memory_id_to_value.small_log_size,
        );
    }
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


#[derive(Drop, Serde)]
pub struct CairoInteractionClaim {
    pub opcodes: OpcodeInteractionClaim,
    pub verify_instruction: components::verify_instruction::InteractionClaim,
    pub blake_context: BlakeContextInteractionClaim,
    pub builtins: BuiltinsInteractionClaim,
    pub pedersen_context: PedersenContextInteractionClaim,
    pub poseidon_context: PoseidonContextInteractionClaim,
    pub memory_address_to_id: components::memory_address_to_id::InteractionClaim,
    pub memory_id_to_value: components::memory_id_to_big::InteractionClaim,
    pub range_checks: RangeChecksInteractionClaim,
    pub verify_bitwise_xor_4: components::verify_bitwise_xor_4::InteractionClaim,
    pub verify_bitwise_xor_7: components::verify_bitwise_xor_7::InteractionClaim,
    pub verify_bitwise_xor_8: components::verify_bitwise_xor_8::InteractionClaim,
    pub verify_bitwise_xor_9: components::verify_bitwise_xor_9::InteractionClaim,
}

#[generate_trait]
impl CairoInteractionClaimImpl of CairoInteractionClaimTrace {
    fn mix_into(self: @CairoInteractionClaim, ref channel: Channel) {
        self.opcodes.mix_into(ref channel);
        self.verify_instruction.mix_into(ref channel);
        self.blake_context.mix_into(ref channel);
        self.builtins.mix_into(ref channel);
        self.pedersen_context.mix_into(ref channel);
        self.poseidon_context.mix_into(ref channel);
        self.memory_address_to_id.mix_into(ref channel);
        self.memory_id_to_value.mix_into(ref channel);
        self.range_checks.mix_into(ref channel);
        self.verify_bitwise_xor_4.mix_into(ref channel);
        self.verify_bitwise_xor_7.mix_into(ref channel);
        self.verify_bitwise_xor_8.mix_into(ref channel);
        self.verify_bitwise_xor_9.mix_into(ref channel);
    }
}

#[derive(Drop, Serde)]
struct BlakeContextInteractionClaim {
    pub interaction_claim: Option<BlakeInteractionClaim>,
}

#[generate_trait]
impl BlakeContextInteractionClaimImpl of BlakeContextInteractionClaimTrait {
    fn mix_into(self: @BlakeContextInteractionClaim, ref channel: Channel) {
        if let Some(interaction_claim) = self.interaction_claim {
            interaction_claim.mix_into(ref channel);
        }
    }

    fn sum(self: @BlakeContextInteractionClaim) -> QM31 {
        if let Some(interaction_claim) = self.interaction_claim {
            interaction_claim.sum()
        } else {
            Zero::zero()
        }
    }
}

#[derive(Drop, Serde)]
pub struct OpcodeInteractionClaim {
    add: Array<components::add_opcode::InteractionClaim>,
    add_small: Array<components::add_opcode_small::InteractionClaim>,
    add_ap: Array<components::add_ap_opcode::InteractionClaim>,
    assert_eq: Array<components::assert_eq_opcode::InteractionClaim>,
    assert_eq_imm: Array<components::assert_eq_opcode_imm::InteractionClaim>,
    assert_eq_double_deref: Array<components::assert_eq_opcode_double_deref::InteractionClaim>,
    blake: Array<components::blake_compress_opcode::InteractionClaim>,
    call: Array<components::call_opcode::InteractionClaim>,
    call_rel_imm: Array<components::call_opcode_rel_imm::InteractionClaim>,
    generic: Array<components::generic_opcode::InteractionClaim>,
    jnz: Array<components::jnz_opcode::InteractionClaim>,
    jnz_taken: Array<components::jnz_opcode_taken::InteractionClaim>,
    jump: Array<components::jump_opcode::InteractionClaim>,
    jump_double_deref: Array<components::jump_opcode_double_deref::InteractionClaim>,
    jump_rel: Array<components::jump_opcode_rel::InteractionClaim>,
    jump_rel_imm: Array<components::jump_opcode_rel_imm::InteractionClaim>,
    mul: Array<components::mul_opcode::InteractionClaim>,
    mul_small: Array<components::mul_opcode_small::InteractionClaim>,
    qm31: Array<components::qm_31_add_mul_opcode::InteractionClaim>,
    ret: Array<components::ret_opcode::InteractionClaim>,
}

#[generate_trait]
impl OpcodeInteractionClaimImpl of OpcodeInteractionClaimTrait {
    fn mix_into(self: @OpcodeInteractionClaim, ref channel: Channel) {
        for interaction_claim in self.add.span() {
            interaction_claim.mix_into(ref channel);
        }

        for interaction_claim in self.add_small.span() {
            interaction_claim.mix_into(ref channel);
        }

        for interaction_claim in self.add_ap.span() {
            interaction_claim.mix_into(ref channel);
        }

        for interaction_claim in self.assert_eq.span() {
            interaction_claim.mix_into(ref channel);
        }

        for interaction_claim in self.assert_eq_imm.span() {
            interaction_claim.mix_into(ref channel);
        }

        for interaction_claim in self.assert_eq_double_deref.span() {
            interaction_claim.mix_into(ref channel);
        }

        for interaction_claim in self.blake.span() {
            interaction_claim.mix_into(ref channel);
        }

        for interaction_claim in self.call.span() {
            interaction_claim.mix_into(ref channel);
        }

        for interaction_claim in self.call_rel_imm.span() {
            interaction_claim.mix_into(ref channel);
        }

        for interaction_claim in self.generic.span() {
            interaction_claim.mix_into(ref channel);
        }

        for interaction_claim in self.jnz.span() {
            interaction_claim.mix_into(ref channel);
        }

        for interaction_claim in self.jnz_taken.span() {
            interaction_claim.mix_into(ref channel);
        }

        for interaction_claim in self.jump.span() {
            interaction_claim.mix_into(ref channel);
        }

        for interaction_claim in self.jump_double_deref.span() {
            interaction_claim.mix_into(ref channel);
        }

        for interaction_claim in self.jump_rel.span() {
            interaction_claim.mix_into(ref channel);
        }

        for interaction_claim in self.jump_rel_imm.span() {
            interaction_claim.mix_into(ref channel);
        }

        for interaction_claim in self.mul.span() {
            interaction_claim.mix_into(ref channel);
        }

        for interaction_claim in self.mul_small.span() {
            interaction_claim.mix_into(ref channel);
        }

        for interaction_claim in self.qm31.span() {
            interaction_claim.mix_into(ref channel);
        }

        for interaction_claim in self.ret.span() {
            interaction_claim.mix_into(ref channel);
        }
    }

    fn sum(self: @OpcodeInteractionClaim) -> QM31 {
        let mut sum = Zero::zero();

        for interaction_claim in self.add.span() {
            sum += *interaction_claim.claimed_sum;
        }

        for interaction_claim in self.add_small.span() {
            sum += *interaction_claim.claimed_sum;
        }

        for interaction_claim in self.add_ap.span() {
            sum += *interaction_claim.claimed_sum;
        }

        for interaction_claim in self.assert_eq.span() {
            sum += *interaction_claim.claimed_sum;
        }

        for interaction_claim in self.assert_eq_imm.span() {
            sum += *interaction_claim.claimed_sum;
        }

        for interaction_claim in self.assert_eq_double_deref.span() {
            sum += *interaction_claim.claimed_sum;
        }

        for interaction_claim in self.blake.span() {
            sum += *interaction_claim.claimed_sum;
        }

        for interaction_claim in self.call.span() {
            sum += *interaction_claim.claimed_sum;
        }

        for interaction_claim in self.call_rel_imm.span() {
            sum += *interaction_claim.claimed_sum;
        }

        for interaction_claim in self.generic.span() {
            sum += *interaction_claim.claimed_sum;
        }

        for interaction_claim in self.jnz.span() {
            sum += *interaction_claim.claimed_sum;
        }

        for interaction_claim in self.jnz_taken.span() {
            sum += *interaction_claim.claimed_sum;
        }

        for interaction_claim in self.jump.span() {
            sum += *interaction_claim.claimed_sum;
        }

        for interaction_claim in self.jump_double_deref.span() {
            sum += *interaction_claim.claimed_sum;
        }

        for interaction_claim in self.jump_rel.span() {
            sum += *interaction_claim.claimed_sum;
        }

        for interaction_claim in self.jump_rel_imm.span() {
            sum += *interaction_claim.claimed_sum;
        }

        for interaction_claim in self.mul.span() {
            sum += *interaction_claim.claimed_sum;
        }

        for interaction_claim in self.mul_small.span() {
            sum += *interaction_claim.claimed_sum;
        }

        for interaction_claim in self.qm31.span() {
            sum += *interaction_claim.claimed_sum;
        }

        for interaction_claim in self.ret.span() {
            sum += *interaction_claim.claimed_sum;
        }

        sum
    }
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
        let mut flat_output = array![];
        for entry in output.span() {
            let (_, val) = entry;
            flat_output.append_span((*val).span());
        }
        channel.mix_u32s(flat_output.span());

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


#[derive(Drop, Serde)]
pub struct PublicData {
    pub public_memory: PublicMemory,
    pub initial_state: CasmState,
    pub final_state: CasmState,
}

#[generate_trait]
impl PublicDataImpl of PublicDataTrait {
    fn logup_sum(self: @PublicData, lookup_elements: @CairoInteractionElements) -> QM31 {
        // TODO(Ohad): Optimized inverse.
        let mut sum = Zero::zero();

        // TODO(andrew): Consider batch inverse here.
        for entry in self
            .public_memory
            .get_entries(
                initial_pc: (*self.initial_state.pc).into(),
                initial_ap: (*self.initial_state.ap).into(),
                final_ap: (*self.final_state.ap).into(),
            )
            .span() {
            let (addr, id, val) = *entry;

            let addr_m31 = addr.try_into().unwrap();
            let id_m31 = id.try_into().unwrap();
            let addr_to_id = lookup_elements
                .memory_address_to_id
                .combine([addr_m31, id_m31])
                .inverse();

            let mut elements = array![id_m31];
            elements.append_span(utils::split_f252(val).span());
            let id_to_value = lookup_elements
                .memory_id_to_value
                .combine((*elements.span().try_into().unwrap()).unbox())
                .inverse();

            sum += addr_to_id + id_to_value;
        }

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

#[derive(Drop, Serde)]
pub struct OpcodeClaim {
    pub add: Array<components::add_opcode::Claim>,
    pub add_small: Array<components::add_opcode_small::Claim>,
    pub add_ap: Array<components::add_ap_opcode::Claim>,
    pub assert_eq: Array<components::assert_eq_opcode::Claim>,
    pub assert_eq_imm: Array<components::assert_eq_opcode_imm::Claim>,
    pub assert_eq_double_deref: Array<components::assert_eq_opcode_double_deref::Claim>,
    pub blake: Array<components::blake_compress_opcode::Claim>,
    pub call: Array<components::call_opcode::Claim>,
    pub call_rel_imm: Array<components::call_opcode_rel_imm::Claim>,
    pub generic: Array<components::generic_opcode::Claim>,
    pub jnz: Array<components::jnz_opcode::Claim>,
    pub jnz_taken: Array<components::jnz_opcode_taken::Claim>,
    pub jump: Array<components::jump_opcode::Claim>,
    pub jump_double_deref: Array<components::jump_opcode_double_deref::Claim>,
    pub jump_rel: Array<components::jump_opcode_rel::Claim>,
    pub jump_rel_imm: Array<components::jump_opcode_rel_imm::Claim>,
    pub mul: Array<components::mul_opcode::Claim>,
    pub mul_small: Array<components::mul_opcode_small::Claim>,
    pub qm31: Array<components::qm_31_add_mul_opcode::Claim>,
    pub ret: Array<components::ret_opcode::Claim>,
}

#[generate_trait]
impl OpcodeClaimImpl of OpcodeClaimTrait {
    fn mix_into(self: @OpcodeClaim, ref channel: Channel) {
        channel.mix_u64(self.add.len().into());
        for claim in self.add.span() {
            claim.mix_into(ref channel);
        }

        channel.mix_u64(self.add_small.len().into());
        for claim in self.add_small.span() {
            claim.mix_into(ref channel);
        }

        channel.mix_u64(self.add_ap.len().into());
        for claim in self.add_ap.span() {
            claim.mix_into(ref channel);
        }

        channel.mix_u64(self.assert_eq.len().into());
        for claim in self.assert_eq.span() {
            claim.mix_into(ref channel);
        }

        channel.mix_u64(self.assert_eq_imm.len().into());
        for claim in self.assert_eq_imm.span() {
            claim.mix_into(ref channel);
        }

        channel.mix_u64(self.assert_eq_double_deref.len().into());
        for claim in self.assert_eq_double_deref.span() {
            claim.mix_into(ref channel);
        }

        channel.mix_u64(self.blake.len().into());
        for claim in self.blake.span() {
            claim.mix_into(ref channel);
        }

        channel.mix_u64(self.call.len().into());
        for claim in self.call.span() {
            claim.mix_into(ref channel);
        }

        channel.mix_u64(self.call_rel_imm.len().into());
        for claim in self.call_rel_imm.span() {
            claim.mix_into(ref channel);
        }

        channel.mix_u64(self.generic.len().into());
        for claim in self.generic.span() {
            claim.mix_into(ref channel);
        }

        channel.mix_u64(self.jnz.len().into());
        for claim in self.jnz.span() {
            claim.mix_into(ref channel);
        }

        channel.mix_u64(self.jnz_taken.len().into());
        for claim in self.jnz_taken.span() {
            claim.mix_into(ref channel);
        }

        channel.mix_u64(self.jump.len().into());
        for claim in self.jump.span() {
            claim.mix_into(ref channel);
        }

        channel.mix_u64(self.jump_double_deref.len().into());
        for claim in self.jump_double_deref.span() {
            claim.mix_into(ref channel);
        }

        channel.mix_u64(self.jump_rel.len().into());
        for claim in self.jump_rel.span() {
            claim.mix_into(ref channel);
        }

        channel.mix_u64(self.jump_rel_imm.len().into());
        for claim in self.jump_rel_imm.span() {
            claim.mix_into(ref channel);
        }

        channel.mix_u64(self.mul.len().into());
        for claim in self.mul.span() {
            claim.mix_into(ref channel);
        }

        channel.mix_u64(self.mul_small.len().into());
        for claim in self.mul_small.span() {
            claim.mix_into(ref channel);
        }

        channel.mix_u64(self.qm31.len().into());
        for claim in self.qm31.span() {
            claim.mix_into(ref channel);
        }

        channel.mix_u64(self.ret.len().into());
        for claim in self.ret.span() {
            claim.mix_into(ref channel);
        }
    }

    fn log_sizes(self: @OpcodeClaim) -> TreeArray<Span<u32>> {
        let mut log_sizes = array![];

        for claim in self.add.span() {
            log_sizes.append(claim.log_sizes());
        }

        for claim in self.add_small.span() {
            log_sizes.append(claim.log_sizes());
        }

        for claim in self.add_ap.span() {
            log_sizes.append(claim.log_sizes());
        }

        for claim in self.assert_eq.span() {
            log_sizes.append(claim.log_sizes());
        }

        for claim in self.assert_eq_imm.span() {
            log_sizes.append(claim.log_sizes());
        }

        for claim in self.assert_eq_double_deref.span() {
            log_sizes.append(claim.log_sizes());
        }

        for claim in self.blake.span() {
            log_sizes.append(claim.log_sizes());
        }

        for claim in self.call.span() {
            log_sizes.append(claim.log_sizes());
        }

        for claim in self.call_rel_imm.span() {
            log_sizes.append(claim.log_sizes());
        }

        for claim in self.generic.span() {
            log_sizes.append(claim.log_sizes());
        }

        for claim in self.jnz.span() {
            log_sizes.append(claim.log_sizes());
        }

        for claim in self.jnz_taken.span() {
            log_sizes.append(claim.log_sizes());
        }

        for claim in self.jump.span() {
            log_sizes.append(claim.log_sizes());
        }

        for claim in self.jump_double_deref.span() {
            log_sizes.append(claim.log_sizes());
        }

        for claim in self.jump_rel.span() {
            log_sizes.append(claim.log_sizes());
        }

        for claim in self.jump_rel_imm.span() {
            log_sizes.append(claim.log_sizes());
        }

        for claim in self.mul.span() {
            log_sizes.append(claim.log_sizes());
        }

        for claim in self.mul_small.span() {
            log_sizes.append(claim.log_sizes());
        }

        for claim in self.qm31.span() {
            log_sizes.append(claim.log_sizes());
        }

        for claim in self.ret.span() {
            log_sizes.append(claim.log_sizes());
        }

        utils::tree_array_concat_cols(log_sizes)
    }

    fn accumulate_relation_uses(self: @OpcodeClaim, ref relation_uses: RelationUsesDict) {
        let OpcodeClaim {
            add,
            add_small,
            add_ap,
            assert_eq,
            assert_eq_imm,
            assert_eq_double_deref,
            blake,
            call,
            call_rel_imm,
            generic,
            jnz,
            jnz_taken,
            jump,
            jump_double_deref,
            jump_rel,
            jump_rel_imm,
            mul,
            mul_small,
            qm31,
            ret,
        } = self;
        for claim in add.span() {
            accumulate_relation_uses(
                ref relation_uses,
                components::add_opcode::RELATION_USES_PER_ROW.span(),
                *claim.log_size,
            );
        }

        for claim in add_small.span() {
            accumulate_relation_uses(
                ref relation_uses,
                components::add_opcode_small::RELATION_USES_PER_ROW.span(),
                *claim.log_size,
            );
        }

        for claim in add_ap.span() {
            accumulate_relation_uses(
                ref relation_uses,
                components::add_ap_opcode::RELATION_USES_PER_ROW.span(),
                *claim.log_size,
            );
        }

        for claim in assert_eq.span() {
            accumulate_relation_uses(
                ref relation_uses,
                components::assert_eq_opcode::RELATION_USES_PER_ROW.span(),
                *claim.log_size,
            );
        }

        for claim in assert_eq_imm.span() {
            accumulate_relation_uses(
                ref relation_uses,
                components::assert_eq_opcode_imm::RELATION_USES_PER_ROW.span(),
                *claim.log_size,
            );
        }

        for claim in assert_eq_double_deref.span() {
            accumulate_relation_uses(
                ref relation_uses,
                components::assert_eq_opcode_double_deref::RELATION_USES_PER_ROW.span(),
                *claim.log_size,
            );
        }

        for claim in blake.span() {
            accumulate_relation_uses(
                ref relation_uses,
                components::blake_compress_opcode::RELATION_USES_PER_ROW.span(),
                *claim.log_size,
            );
        }

        for claim in call.span() {
            accumulate_relation_uses(
                ref relation_uses,
                components::call_opcode::RELATION_USES_PER_ROW.span(),
                *claim.log_size,
            );
        }

        for claim in call_rel_imm.span() {
            accumulate_relation_uses(
                ref relation_uses,
                components::call_opcode_rel_imm::RELATION_USES_PER_ROW.span(),
                *claim.log_size,
            );
        }

        for claim in generic.span() {
            accumulate_relation_uses(
                ref relation_uses,
                components::generic_opcode::RELATION_USES_PER_ROW.span(),
                *claim.log_size,
            );
        }

        for claim in jnz.span() {
            accumulate_relation_uses(
                ref relation_uses,
                components::jnz_opcode::RELATION_USES_PER_ROW.span(),
                *claim.log_size,
            );
        }

        for claim in jnz_taken.span() {
            accumulate_relation_uses(
                ref relation_uses,
                components::jnz_opcode_taken::RELATION_USES_PER_ROW.span(),
                *claim.log_size,
            );
        }

        for claim in jump.span() {
            accumulate_relation_uses(
                ref relation_uses,
                components::jump_opcode::RELATION_USES_PER_ROW.span(),
                *claim.log_size,
            );
        }

        for claim in jump_double_deref.span() {
            accumulate_relation_uses(
                ref relation_uses,
                components::jump_opcode_double_deref::RELATION_USES_PER_ROW.span(),
                *claim.log_size,
            );
        }

        for claim in jump_rel.span() {
            accumulate_relation_uses(
                ref relation_uses,
                components::jump_opcode_rel::RELATION_USES_PER_ROW.span(),
                *claim.log_size,
            );
        }

        for claim in jump_rel_imm.span() {
            accumulate_relation_uses(
                ref relation_uses,
                components::jump_opcode_rel_imm::RELATION_USES_PER_ROW.span(),
                *claim.log_size,
            );
        }

        for claim in mul.span() {
            accumulate_relation_uses(
                ref relation_uses,
                components::mul_opcode::RELATION_USES_PER_ROW.span(),
                *claim.log_size,
            );
        }

        for claim in mul_small.span() {
            accumulate_relation_uses(
                ref relation_uses,
                components::mul_opcode_small::RELATION_USES_PER_ROW.span(),
                *claim.log_size,
            );
        }

        for claim in qm31.span() {
            accumulate_relation_uses(
                ref relation_uses,
                components::qm_31_add_mul_opcode::RELATION_USES_PER_ROW.span(),
                *claim.log_size,
            );
        }

        for claim in ret.span() {
            accumulate_relation_uses(
                ref relation_uses,
                components::ret_opcode::RELATION_USES_PER_ROW.span(),
                *claim.log_size,
            );
        }
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
    InvalidClaim,
    Stark: VerificationError,
}

#[derive(Drop)]
#[cfg(not(feature: "poseidon252_verifier"))]
pub struct CairoAir {
    opcodes: OpcodeComponents,
    verify_instruction: components::verify_instruction::Component,
    blake_context: BlakeContextComponents,
    builtins: BuiltinComponents,
    pedersen_context: PedersenContextComponents,
    poseidon_context: PoseidonContextComponents,
    memory_address_to_id: components::memory_address_to_id::Component,
    memory_id_to_value: (
        Array<components::memory_id_to_big::BigComponent>,
        components::memory_id_to_big::SmallComponent,
    ),
    range_checks: RangeChecksComponents,
    verify_bitwise_xor_4: components::verify_bitwise_xor_4::Component,
    verify_bitwise_xor_7: components::verify_bitwise_xor_7::Component,
    verify_bitwise_xor_8: components::verify_bitwise_xor_8::Component,
    verify_bitwise_xor_9: components::verify_bitwise_xor_9::Component,
}

#[generate_trait]
#[cfg(not(feature: "poseidon252_verifier"))]
impl CairoAirNewImpl of CairoAirNewTrait {
    fn new(
        cairo_claim: @CairoClaim,
        interaction_elements: @CairoInteractionElements,
        interaction_claim: @CairoInteractionClaim,
    ) -> CairoAir {
        let opcode_components = OpcodeComponentsImpl::new(
            cairo_claim.opcodes, interaction_elements, interaction_claim.opcodes,
        );

        let blake_context_component = BlakeContextComponentsImpl::new(
            cairo_claim.blake_context, interaction_elements, interaction_claim.blake_context,
        );

        let builtins_components = BuiltinComponentsImpl::new(
            cairo_claim.builtins, interaction_elements, interaction_claim.builtins,
        );

        let pedersen_context_components = PedersenContextComponentsImpl::new(
            cairo_claim.pedersen_context, interaction_elements, interaction_claim.pedersen_context,
        );

        let poseidon_context_components = PoseidonContextComponentsImpl::new(
            cairo_claim.poseidon_context, interaction_elements, interaction_claim.poseidon_context,
        );

        let verifyinstruction_component = components::verify_instruction::Component {
            claim: *cairo_claim.verify_instruction,
            interaction_claim: *interaction_claim.verify_instruction,
            memory_address_to_id_lookup_elements: interaction_elements.memory_address_to_id.clone(),
            memory_id_to_big_lookup_elements: interaction_elements.memory_id_to_value.clone(),
            range_check_4_3_lookup_elements: interaction_elements.range_checks.rc_4_3.clone(),
            range_check_7_2_5_lookup_elements: interaction_elements.range_checks.rc_7_2_5.clone(),
            verify_instruction_lookup_elements: interaction_elements.verify_instruction.clone(),
        };

        let memory_address_to_id_component = components::memory_address_to_id::Component {
            claim: *cairo_claim.memory_address_to_id,
            interaction_claim: *interaction_claim.memory_address_to_id,
            lookup_elements: interaction_elements.memory_address_to_id.clone(),
        };

        assert!(
            cairo_claim
                .memory_id_to_value
                .big_log_sizes
                .len() == interaction_claim
                .memory_id_to_value
                .big_claimed_sums
                .len(),
        );
        let mut memory_id_to_value_components = array![];
        let mut offset = 0;
        for i in 0..cairo_claim.memory_id_to_value.big_log_sizes.len() {
            let log_size = *cairo_claim.memory_id_to_value.big_log_sizes[i];
            let claimed_sum = *interaction_claim.memory_id_to_value.big_claimed_sums[i];
            memory_id_to_value_components
                .append(
                    components::memory_id_to_big::BigComponent {
                        log_n_rows: log_size,
                        offset: offset,
                        claimed_sum: claimed_sum,
                        lookup_elements: interaction_elements.memory_id_to_value.clone(),
                        range_9_9_lookup_elements: interaction_elements.range_checks.rc_9_9.clone(),
                        range_9_9_b_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9_b
                            .clone(),
                        range_9_9_c_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9_c
                            .clone(),
                        range_9_9_d_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9_d
                            .clone(),
                        range_9_9_e_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9_e
                            .clone(),
                        range_9_9_f_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9_f
                            .clone(),
                        range_9_9_g_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9_g
                            .clone(),
                        range_9_9_h_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9_h
                            .clone(),
                    },
                );
            offset = offset + pow2(log_size);
        }

        let small_memory_id_to_value_component = components::memory_id_to_big::SmallComponent {
            log_n_rows: *cairo_claim.memory_id_to_value.small_log_size,
            claimed_sum: *interaction_claim.memory_id_to_value.small_claimed_sum,
            lookup_elements: interaction_elements.memory_id_to_value.clone(),
            range_9_9_lookup_elements: interaction_elements.range_checks.rc_9_9.clone(),
            range_9_9_b_lookup_elements: interaction_elements.range_checks.rc_9_9_b.clone(),
            range_9_9_c_lookup_elements: interaction_elements.range_checks.rc_9_9_c.clone(),
            range_9_9_d_lookup_elements: interaction_elements.range_checks.rc_9_9_d.clone(),
        };

        let range_checks_components = RangeChecksComponentsImpl::new(
            cairo_claim.range_checks, interaction_elements, interaction_claim.range_checks,
        );

        let verify_bitwise_xor_4_component = components::verify_bitwise_xor_4::Component {
            claim: *cairo_claim.verify_bitwise_xor_4,
            interaction_claim: *interaction_claim.verify_bitwise_xor_4,
            verify_bitwise_xor_4_lookup_elements: interaction_elements.verify_bitwise_xor_4.clone(),
        };

        let verify_bitwise_xor_7_component = components::verify_bitwise_xor_7::Component {
            claim: *cairo_claim.verify_bitwise_xor_7,
            interaction_claim: *interaction_claim.verify_bitwise_xor_7,
            verify_bitwise_xor_7_lookup_elements: interaction_elements.verify_bitwise_xor_7.clone(),
        };

        let verify_bitwise_xor_8_component = components::verify_bitwise_xor_8::Component {
            claim: *cairo_claim.verify_bitwise_xor_8,
            interaction_claim: *interaction_claim.verify_bitwise_xor_8,
            verify_bitwise_xor_8_lookup_elements: interaction_elements.verify_bitwise_xor_8.clone(),
        };

        let verify_bitwise_xor_9_component = components::verify_bitwise_xor_9::Component {
            claim: *cairo_claim.verify_bitwise_xor_9,
            interaction_claim: *interaction_claim.verify_bitwise_xor_9,
            verify_bitwise_xor_9_lookup_elements: interaction_elements.verify_bitwise_xor_9.clone(),
        };

        CairoAir {
            opcodes: opcode_components,
            verify_instruction: verifyinstruction_component,
            blake_context: blake_context_component,
            builtins: builtins_components,
            pedersen_context: pedersen_context_components,
            poseidon_context: poseidon_context_components,
            memory_address_to_id: memory_address_to_id_component,
            memory_id_to_value: (memory_id_to_value_components, small_memory_id_to_value_component),
            range_checks: range_checks_components,
            verify_bitwise_xor_4: verify_bitwise_xor_4_component,
            verify_bitwise_xor_7: verify_bitwise_xor_7_component,
            verify_bitwise_xor_8: verify_bitwise_xor_8_component,
            verify_bitwise_xor_9: verify_bitwise_xor_9_component,
        }
    }
}

#[cfg(not(feature: "poseidon252_verifier"))]
impl CairoAirImpl of Air<CairoAir> {
    fn composition_log_degree_bound(self: @CairoAir) -> u32 {
        let mut max_degree = self.opcodes.max_constraint_log_degree_bound();
        max_degree =
            core::cmp::max(max_degree, self.verify_instruction.max_constraint_log_degree_bound());
        max_degree =
            core::cmp::max(max_degree, self.blake_context.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.builtins.max_constraint_log_degree_bound());
        max_degree =
            core::cmp::max(max_degree, self.pedersen_context.max_constraint_log_degree_bound());
        max_degree =
            core::cmp::max(max_degree, self.poseidon_context.max_constraint_log_degree_bound());
        max_degree =
            core::cmp::max(max_degree, self.memory_address_to_id.max_constraint_log_degree_bound());
        let (memory_id_to_value_big, memory_id_to_value_small) = self.memory_id_to_value;
        for memory_id_to_value_big_component in memory_id_to_value_big.span() {
            max_degree =
                core::cmp::max(
                    max_degree, memory_id_to_value_big_component.max_constraint_log_degree_bound(),
                );
        }
        max_degree =
            core::cmp::max(max_degree, memory_id_to_value_small.max_constraint_log_degree_bound());
        max_degree =
            core::cmp::max(max_degree, self.range_checks.max_constraint_log_degree_bound());
        max_degree =
            core::cmp::max(max_degree, self.verify_bitwise_xor_4.max_constraint_log_degree_bound());
        max_degree =
            core::cmp::max(max_degree, self.verify_bitwise_xor_7.max_constraint_log_degree_bound());
        max_degree =
            core::cmp::max(max_degree, self.verify_bitwise_xor_8.max_constraint_log_degree_bound());
        max_degree =
            core::cmp::max(max_degree, self.verify_bitwise_xor_9.max_constraint_log_degree_bound());
        max_degree
    }

    fn mask_points(
        self: @CairoAir, point: CirclePoint<QM31>,
    ) -> TreeArray<ColumnArray<Array<CirclePoint<QM31>>>> {
        let mut preprocessed_column_set: PreprocessedColumnSet = Default::default();
        let mut trace_mask_points = array![];
        let mut interaction_trace_mask_points = array![];
        self
            .opcodes
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .verify_instruction
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .blake_context
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .builtins
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .pedersen_context
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .poseidon_context
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .memory_address_to_id
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );

        let (memory_id_to_value_big, memory_id_to_value_small) = self.memory_id_to_value;
        for memory_id_to_value_big_component in memory_id_to_value_big.span() {
            memory_id_to_value_big_component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }
        memory_id_to_value_small
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .range_checks
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .verify_bitwise_xor_4
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .verify_bitwise_xor_7
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .verify_bitwise_xor_8
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .verify_bitwise_xor_9
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );

        let preprocessed_trace_mask_points = preprocessed_trace_mask_points(
            preprocessed_column_set, point,
        );

        array![preprocessed_trace_mask_points, trace_mask_points, interaction_trace_mask_points]
    }

    fn eval_composition_polynomial_at_point(
        self: @CairoAir,
        point: CirclePoint<QM31>,
        mask_values: TreeSpan<ColumnSpan<Span<QM31>>>,
        random_coeff: QM31,
    ) -> QM31 {
        let mut sum = Zero::zero();

        let [
            preprocessed_mask_values,
            mut trace_mask_values,
            mut interaction_trace_mask_values,
            _composition_trace_mask_values,
        ]: [ColumnSpan<Span<QM31>>; 4] =
            (*mask_values
            .try_into()
            .unwrap())
            .unbox();

        let mut preprocessed_mask_values = PreprocessedMaskValuesImpl::new(
            preprocessed_mask_values, PREPROCESSED_COLUMNS.span(),
        );

        self
            .opcodes
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .verify_instruction
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .blake_context
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .builtins
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .pedersen_context
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .poseidon_context
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .memory_address_to_id
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        let (memory_id_to_value_big, memory_id_to_value_small) = self.memory_id_to_value;
        for memory_id_to_value_big_component in memory_id_to_value_big.span() {
            memory_id_to_value_big_component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }
        memory_id_to_value_small
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .range_checks
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .verify_bitwise_xor_4
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .verify_bitwise_xor_7
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .verify_bitwise_xor_8
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .verify_bitwise_xor_9
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        sum
    }
}

#[derive(Drop)]
#[cfg(feature: "poseidon252_verifier")]
pub struct CairoAir {
    opcodes: OpcodeComponents,
    verify_instruction: components::verify_instruction::Component,
    blake_context: BlakeContextComponents,
    builtins: BuiltinComponents,
    memory_address_to_id: components::memory_address_to_id::Component,
    memory_id_to_value: (
        Array<components::memory_id_to_big::BigComponent>,
        components::memory_id_to_big::SmallComponent,
    ),
    range_checks: RangeChecksComponents,
    verify_bitwise_xor_4: components::verify_bitwise_xor_4::Component,
    verify_bitwise_xor_7: components::verify_bitwise_xor_7::Component,
    verify_bitwise_xor_8: components::verify_bitwise_xor_8::Component,
    verify_bitwise_xor_9: components::verify_bitwise_xor_9::Component,
}

#[generate_trait]
#[cfg(feature: "poseidon252_verifier")]
impl CairoAirNewImpl of CairoAirNewTrait {
    fn new(
        cairo_claim: @CairoClaim,
        interaction_elements: @CairoInteractionElements,
        interaction_claim: @CairoInteractionClaim,
    ) -> CairoAir {
        let opcode_components = OpcodeComponentsImpl::new(
            cairo_claim.opcodes, interaction_elements, interaction_claim.opcodes,
        );

        let blake_context_component = BlakeContextComponentsImpl::new(
            cairo_claim.blake_context, interaction_elements, interaction_claim.blake_context,
        );

        let builtins_components = BuiltinComponentsImpl::new(
            cairo_claim.builtins, interaction_elements, interaction_claim.builtins,
        );

        let verifyinstruction_component = components::verify_instruction::Component {
            claim: *cairo_claim.verify_instruction,
            interaction_claim: *interaction_claim.verify_instruction,
            memory_address_to_id_lookup_elements: interaction_elements.memory_address_to_id.clone(),
            memory_id_to_big_lookup_elements: interaction_elements.memory_id_to_value.clone(),
            range_check_4_3_lookup_elements: interaction_elements.range_checks.rc_4_3.clone(),
            range_check_7_2_5_lookup_elements: interaction_elements.range_checks.rc_7_2_5.clone(),
            verify_instruction_lookup_elements: interaction_elements.verify_instruction.clone(),
        };

        let memory_address_to_id_component = components::memory_address_to_id::Component {
            claim: *cairo_claim.memory_address_to_id,
            interaction_claim: *interaction_claim.memory_address_to_id,
            lookup_elements: interaction_elements.memory_address_to_id.clone(),
        };

        assert!(
            cairo_claim
                .memory_id_to_value
                .big_log_sizes
                .len() == interaction_claim
                .memory_id_to_value
                .big_claimed_sums
                .len(),
        );
        let mut memory_id_to_value_components = array![];
        let mut offset = 0;
        for i in 0..cairo_claim.memory_id_to_value.big_log_sizes.len() {
            let log_size = *cairo_claim.memory_id_to_value.big_log_sizes[i];
            let claimed_sum = *interaction_claim.memory_id_to_value.big_claimed_sums[i];
            memory_id_to_value_components
                .append(
                    components::memory_id_to_big::BigComponent {
                        log_n_rows: log_size,
                        offset: offset,
                        claimed_sum: claimed_sum,
                        lookup_elements: interaction_elements.memory_id_to_value.clone(),
                        range_9_9_lookup_elements: interaction_elements.range_checks.rc_9_9.clone(),
                        range_9_9_b_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9_b
                            .clone(),
                        range_9_9_c_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9_c
                            .clone(),
                        range_9_9_d_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9_d
                            .clone(),
                        range_9_9_e_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9_e
                            .clone(),
                        range_9_9_f_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9_f
                            .clone(),
                        range_9_9_g_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9_g
                            .clone(),
                        range_9_9_h_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9_h
                            .clone(),
                    },
                );
            offset = offset + pow2(log_size);
        }

        let small_memory_id_to_value_component = components::memory_id_to_big::SmallComponent {
            log_n_rows: *cairo_claim.memory_id_to_value.small_log_size,
            claimed_sum: *interaction_claim.memory_id_to_value.small_claimed_sum,
            lookup_elements: interaction_elements.memory_id_to_value.clone(),
            range_9_9_lookup_elements: interaction_elements.range_checks.rc_9_9.clone(),
            range_9_9_b_lookup_elements: interaction_elements.range_checks.rc_9_9_b.clone(),
            range_9_9_c_lookup_elements: interaction_elements.range_checks.rc_9_9_c.clone(),
            range_9_9_d_lookup_elements: interaction_elements.range_checks.rc_9_9_d.clone(),
        };

        let range_checks_components = RangeChecksComponentsImpl::new(
            cairo_claim.range_checks, interaction_elements, interaction_claim.range_checks,
        );

        let verify_bitwise_xor_4_component = components::verify_bitwise_xor_4::Component {
            claim: *cairo_claim.verify_bitwise_xor_4,
            interaction_claim: *interaction_claim.verify_bitwise_xor_4,
            verify_bitwise_xor_4_lookup_elements: interaction_elements.verify_bitwise_xor_4.clone(),
        };

        let verify_bitwise_xor_7_component = components::verify_bitwise_xor_7::Component {
            claim: *cairo_claim.verify_bitwise_xor_7,
            interaction_claim: *interaction_claim.verify_bitwise_xor_7,
            verify_bitwise_xor_7_lookup_elements: interaction_elements.verify_bitwise_xor_7.clone(),
        };

        let verify_bitwise_xor_8_component = components::verify_bitwise_xor_8::Component {
            claim: *cairo_claim.verify_bitwise_xor_8,
            interaction_claim: *interaction_claim.verify_bitwise_xor_8,
            verify_bitwise_xor_8_lookup_elements: interaction_elements.verify_bitwise_xor_8.clone(),
        };

        let verify_bitwise_xor_9_component = components::verify_bitwise_xor_9::Component {
            claim: *cairo_claim.verify_bitwise_xor_9,
            interaction_claim: *interaction_claim.verify_bitwise_xor_9,
            verify_bitwise_xor_9_lookup_elements: interaction_elements.verify_bitwise_xor_9.clone(),
        };

        CairoAir {
            opcodes: opcode_components,
            verify_instruction: verifyinstruction_component,
            blake_context: blake_context_component,
            builtins: builtins_components,
            memory_address_to_id: memory_address_to_id_component,
            memory_id_to_value: (memory_id_to_value_components, small_memory_id_to_value_component),
            range_checks: range_checks_components,
            verify_bitwise_xor_4: verify_bitwise_xor_4_component,
            verify_bitwise_xor_7: verify_bitwise_xor_7_component,
            verify_bitwise_xor_8: verify_bitwise_xor_8_component,
            verify_bitwise_xor_9: verify_bitwise_xor_9_component,
        }
    }
}

#[cfg(feature: "poseidon252_verifier")]
impl CairoAirImpl of Air<CairoAir> {
    fn composition_log_degree_bound(self: @CairoAir) -> u32 {
        let mut max_degree = self.opcodes.max_constraint_log_degree_bound();
        max_degree =
            core::cmp::max(max_degree, self.verify_instruction.max_constraint_log_degree_bound());
        max_degree =
            core::cmp::max(max_degree, self.blake_context.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.builtins.max_constraint_log_degree_bound());
        max_degree =
            core::cmp::max(max_degree, self.memory_address_to_id.max_constraint_log_degree_bound());
        let (memory_id_to_value_big, memory_id_to_value_small) = self.memory_id_to_value;
        for memory_id_to_value_big_component in memory_id_to_value_big.span() {
            max_degree =
                core::cmp::max(
                    max_degree, memory_id_to_value_big_component.max_constraint_log_degree_bound(),
                );
        }
        max_degree =
            core::cmp::max(max_degree, memory_id_to_value_small.max_constraint_log_degree_bound());
        max_degree =
            core::cmp::max(max_degree, self.range_checks.max_constraint_log_degree_bound());
        max_degree =
            core::cmp::max(max_degree, self.verify_bitwise_xor_4.max_constraint_log_degree_bound());
        max_degree =
            core::cmp::max(max_degree, self.verify_bitwise_xor_7.max_constraint_log_degree_bound());
        max_degree =
            core::cmp::max(max_degree, self.verify_bitwise_xor_8.max_constraint_log_degree_bound());
        max_degree =
            core::cmp::max(max_degree, self.verify_bitwise_xor_9.max_constraint_log_degree_bound());
        max_degree
    }

    fn mask_points(
        self: @CairoAir, point: CirclePoint<QM31>,
    ) -> TreeArray<ColumnArray<Array<CirclePoint<QM31>>>> {
        let mut preprocessed_column_set: PreprocessedColumnSet = Default::default();
        let mut trace_mask_points = array![];
        let mut interaction_trace_mask_points = array![];
        self
            .opcodes
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .verify_instruction
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .blake_context
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .builtins
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .memory_address_to_id
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );

        let (memory_id_to_value_big, memory_id_to_value_small) = self.memory_id_to_value;
        for memory_id_to_value_big_component in memory_id_to_value_big.span() {
            memory_id_to_value_big_component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }
        memory_id_to_value_small
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .range_checks
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .verify_bitwise_xor_4
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .verify_bitwise_xor_7
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .verify_bitwise_xor_8
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .verify_bitwise_xor_9
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );

        let preprocessed_trace_mask_points = preprocessed_trace_mask_points(
            preprocessed_column_set, point,
        );

        array![preprocessed_trace_mask_points, trace_mask_points, interaction_trace_mask_points]
    }

    fn eval_composition_polynomial_at_point(
        self: @CairoAir,
        point: CirclePoint<QM31>,
        mask_values: TreeSpan<ColumnSpan<Span<QM31>>>,
        random_coeff: QM31,
    ) -> QM31 {
        let mut sum = Zero::zero();

        let [
            preprocessed_mask_values,
            mut trace_mask_values,
            mut interaction_trace_mask_values,
            _composition_trace_mask_values,
        ]: [ColumnSpan<Span<QM31>>; 4] =
            (*mask_values
            .try_into()
            .unwrap())
            .unbox();

        let mut preprocessed_mask_values = PreprocessedMaskValuesImpl::new(
            preprocessed_mask_values, PREPROCESSED_COLUMNS.span(),
        );

        self
            .opcodes
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .verify_instruction
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .blake_context
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .builtins
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .memory_address_to_id
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        let (memory_id_to_value_big, memory_id_to_value_small) = self.memory_id_to_value;
        for memory_id_to_value_big_component in memory_id_to_value_big.span() {
            memory_id_to_value_big_component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }
        memory_id_to_value_small
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .range_checks
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .verify_bitwise_xor_4
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .verify_bitwise_xor_7
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .verify_bitwise_xor_8
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .verify_bitwise_xor_9
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        sum
    }
}

fn preprocessed_trace_mask_points(
    preprocessed_column_set: PreprocessedColumnSet, point: CirclePoint<QM31>,
) -> ColumnArray<Array<CirclePoint<QM31>>> {
    let mut mask_points = array![];

    let PreprocessedColumnSet { values: original_values, mut contains } = preprocessed_column_set;

    for preprocessed_column in PREPROCESSED_COLUMNS.span() {
        let preprocessed_column_key = PreprocessedColumnKey::encode(preprocessed_column);

        if contains.get(preprocessed_column_key) {
            mask_points.append(array![point]);
            // Remove the item from the set.
            contains.insert(preprocessed_column_key, false);
        } else {
            mask_points.append(array![]);
        }
    }

    // Sanity check all the original values have been handled.
    for value in original_values {
        let column_key = PreprocessedColumnKey::encode(@value);
        assert!(!contains.get(column_key));
    }

    mask_points
}

#[derive(Drop)]
pub struct RangeChecksComponents {
    rc_6: components::range_check_6::Component,
    rc_8: components::range_check_8::Component,
    rc_11: components::range_check_11::Component,
    rc_12: components::range_check_12::Component,
    rc_18: components::range_check_18::Component,
    rc_18_b: components::range_check_18_b::Component,
    rc_19: components::range_check_19::Component,
    rc_19_b: components::range_check_19_b::Component,
    rc_19_c: components::range_check_19_c::Component,
    rc_19_d: components::range_check_19_d::Component,
    rc_19_e: components::range_check_19_e::Component,
    rc_19_f: components::range_check_19_f::Component,
    rc_19_g: components::range_check_19_g::Component,
    rc_19_h: components::range_check_19_h::Component,
    rc_4_3: components::range_check_4_3::Component,
    rc_4_4: components::range_check_4_4::Component,
    rc_5_4: components::range_check_5_4::Component,
    rc_9_9: components::range_check_9_9::Component,
    rc_9_9_b: components::range_check_9_9_b::Component,
    rc_9_9_c: components::range_check_9_9_c::Component,
    rc_9_9_d: components::range_check_9_9_d::Component,
    rc_9_9_e: components::range_check_9_9_e::Component,
    rc_9_9_f: components::range_check_9_9_f::Component,
    rc_9_9_g: components::range_check_9_9_g::Component,
    rc_9_9_h: components::range_check_9_9_h::Component,
    rc_7_2_5: components::range_check_7_2_5::Component,
    rc_3_6_6_3: components::range_check_3_6_6_3::Component,
    rc_4_4_4_4: components::range_check_4_4_4_4::Component,
    rc_3_3_3_3_3: components::range_check_3_3_3_3_3::Component,
}

#[generate_trait]
impl RangeChecksComponentsImpl of RangeChecksComponentsTrait {
    fn new(
        claim: @RangeChecksClaim,
        interaction_elements: @CairoInteractionElements,
        interaction_claim: @RangeChecksInteractionClaim,
    ) -> RangeChecksComponents {
        RangeChecksComponents {
            rc_6: components::range_check_6::Component {
                claim: *claim.rc_6,
                interaction_claim: *interaction_claim.rc_6,
                range_check_6_lookup_elements: interaction_elements.range_checks.rc_6.clone(),
            },
            rc_8: components::range_check_8::Component {
                claim: *claim.rc_8,
                interaction_claim: *interaction_claim.rc_8,
                range_check_8_lookup_elements: interaction_elements.range_checks.rc_8.clone(),
            },
            rc_11: components::range_check_11::Component {
                claim: *claim.rc_11,
                interaction_claim: *interaction_claim.rc_11,
                range_check_11_lookup_elements: interaction_elements.range_checks.rc_11.clone(),
            },
            rc_12: components::range_check_12::Component {
                claim: *claim.rc_12,
                interaction_claim: *interaction_claim.rc_12,
                range_check_12_lookup_elements: interaction_elements.range_checks.rc_12.clone(),
            },
            rc_18: components::range_check_18::Component {
                claim: *claim.rc_18,
                interaction_claim: *interaction_claim.rc_18,
                range_check_18_lookup_elements: interaction_elements.range_checks.rc_18.clone(),
            },
            rc_18_b: components::range_check_18_b::Component {
                claim: *claim.rc_18_b,
                interaction_claim: *interaction_claim.rc_18_b,
                range_check_18_b_lookup_elements: interaction_elements.range_checks.rc_18_b.clone(),
            },
            rc_19: components::range_check_19::Component {
                claim: *claim.rc_19,
                interaction_claim: *interaction_claim.rc_19,
                range_check_19_lookup_elements: interaction_elements.range_checks.rc_19.clone(),
            },
            rc_19_b: components::range_check_19_b::Component {
                claim: *claim.rc_19_b,
                interaction_claim: *interaction_claim.rc_19_b,
                range_check_19_b_lookup_elements: interaction_elements.range_checks.rc_19_b.clone(),
            },
            rc_19_c: components::range_check_19_c::Component {
                claim: *claim.rc_19_c,
                interaction_claim: *interaction_claim.rc_19_c,
                range_check_19_c_lookup_elements: interaction_elements.range_checks.rc_19_c.clone(),
            },
            rc_19_d: components::range_check_19_d::Component {
                claim: *claim.rc_19_d,
                interaction_claim: *interaction_claim.rc_19_d,
                range_check_19_d_lookup_elements: interaction_elements.range_checks.rc_19_d.clone(),
            },
            rc_19_e: components::range_check_19_e::Component {
                claim: *claim.rc_19_e,
                interaction_claim: *interaction_claim.rc_19_e,
                range_check_19_e_lookup_elements: interaction_elements.range_checks.rc_19_e.clone(),
            },
            rc_19_f: components::range_check_19_f::Component {
                claim: *claim.rc_19_f,
                interaction_claim: *interaction_claim.rc_19_f,
                range_check_19_f_lookup_elements: interaction_elements.range_checks.rc_19_f.clone(),
            },
            rc_19_g: components::range_check_19_g::Component {
                claim: *claim.rc_19_g,
                interaction_claim: *interaction_claim.rc_19_g,
                range_check_19_g_lookup_elements: interaction_elements.range_checks.rc_19_g.clone(),
            },
            rc_19_h: components::range_check_19_h::Component {
                claim: *claim.rc_19_h,
                interaction_claim: *interaction_claim.rc_19_h,
                range_check_19_h_lookup_elements: interaction_elements.range_checks.rc_19_h.clone(),
            },
            rc_4_3: components::range_check_4_3::Component {
                claim: *claim.rc_4_3,
                interaction_claim: *interaction_claim.rc_4_3,
                range_check_4_3_lookup_elements: interaction_elements.range_checks.rc_4_3.clone(),
            },
            rc_4_4: components::range_check_4_4::Component {
                claim: *claim.rc_4_4,
                interaction_claim: *interaction_claim.rc_4_4,
                range_check_4_4_lookup_elements: interaction_elements.range_checks.rc_4_4.clone(),
            },
            rc_5_4: components::range_check_5_4::Component {
                claim: *claim.rc_5_4,
                interaction_claim: *interaction_claim.rc_5_4,
                range_check_5_4_lookup_elements: interaction_elements.range_checks.rc_5_4.clone(),
            },
            rc_9_9: components::range_check_9_9::Component {
                claim: *claim.rc_9_9,
                interaction_claim: *interaction_claim.rc_9_9,
                range_check_9_9_lookup_elements: interaction_elements.range_checks.rc_9_9.clone(),
            },
            rc_9_9_b: components::range_check_9_9_b::Component {
                claim: *claim.rc_9_9_b,
                interaction_claim: *interaction_claim.rc_9_9_b,
                range_check_9_9_b_lookup_elements: interaction_elements
                    .range_checks
                    .rc_9_9_b
                    .clone(),
            },
            rc_9_9_c: components::range_check_9_9_c::Component {
                claim: *claim.rc_9_9_c,
                interaction_claim: *interaction_claim.rc_9_9_c,
                range_check_9_9_c_lookup_elements: interaction_elements
                    .range_checks
                    .rc_9_9_c
                    .clone(),
            },
            rc_9_9_d: components::range_check_9_9_d::Component {
                claim: *claim.rc_9_9_d,
                interaction_claim: *interaction_claim.rc_9_9_d,
                range_check_9_9_d_lookup_elements: interaction_elements
                    .range_checks
                    .rc_9_9_d
                    .clone(),
            },
            rc_9_9_e: components::range_check_9_9_e::Component {
                claim: *claim.rc_9_9_e,
                interaction_claim: *interaction_claim.rc_9_9_e,
                range_check_9_9_e_lookup_elements: interaction_elements
                    .range_checks
                    .rc_9_9_e
                    .clone(),
            },
            rc_9_9_f: components::range_check_9_9_f::Component {
                claim: *claim.rc_9_9_f,
                interaction_claim: *interaction_claim.rc_9_9_f,
                range_check_9_9_f_lookup_elements: interaction_elements
                    .range_checks
                    .rc_9_9_f
                    .clone(),
            },
            rc_9_9_g: components::range_check_9_9_g::Component {
                claim: *claim.rc_9_9_g,
                interaction_claim: *interaction_claim.rc_9_9_g,
                range_check_9_9_g_lookup_elements: interaction_elements
                    .range_checks
                    .rc_9_9_g
                    .clone(),
            },
            rc_9_9_h: components::range_check_9_9_h::Component {
                claim: *claim.rc_9_9_h,
                interaction_claim: *interaction_claim.rc_9_9_h,
                range_check_9_9_h_lookup_elements: interaction_elements
                    .range_checks
                    .rc_9_9_h
                    .clone(),
            },
            rc_7_2_5: components::range_check_7_2_5::Component {
                claim: *claim.rc_7_2_5,
                interaction_claim: *interaction_claim.rc_7_2_5,
                range_check_7_2_5_lookup_elements: interaction_elements
                    .range_checks
                    .rc_7_2_5
                    .clone(),
            },
            rc_3_6_6_3: components::range_check_3_6_6_3::Component {
                claim: *claim.rc_3_6_6_3,
                interaction_claim: *interaction_claim.rc_3_6_6_3,
                range_check_3_6_6_3_lookup_elements: interaction_elements
                    .range_checks
                    .rc_3_6_6_3
                    .clone(),
            },
            rc_4_4_4_4: components::range_check_4_4_4_4::Component {
                claim: *claim.rc_4_4_4_4,
                interaction_claim: *interaction_claim.rc_4_4_4_4,
                range_check_4_4_4_4_lookup_elements: interaction_elements
                    .range_checks
                    .rc_4_4_4_4
                    .clone(),
            },
            rc_3_3_3_3_3: components::range_check_3_3_3_3_3::Component {
                claim: *claim.rc_3_3_3_3_3,
                interaction_claim: *interaction_claim.rc_3_3_3_3_3,
                range_check_3_3_3_3_3_lookup_elements: interaction_elements
                    .range_checks
                    .rc_3_3_3_3_3
                    .clone(),
            },
        }
    }

    fn mask_points(
        self: @RangeChecksComponents,
        ref preprocessed_column_set: PreprocessedColumnSet,
        ref trace_mask_points: Array<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: Array<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        self
            .rc_6
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .rc_8
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .rc_11
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .rc_12
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .rc_18
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .rc_18_b
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .rc_19
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .rc_19_b
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .rc_19_c
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .rc_19_d
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .rc_19_e
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .rc_19_f
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .rc_19_g
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .rc_19_h
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .rc_4_3
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .rc_4_4
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .rc_5_4
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .rc_9_9
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .rc_9_9_b
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .rc_9_9_c
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .rc_9_9_d
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .rc_9_9_e
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .rc_9_9_f
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .rc_9_9_g
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .rc_9_9_h
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .rc_7_2_5
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .rc_3_6_6_3
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .rc_4_4_4_4
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .rc_3_3_3_3_3
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
    }

    fn max_constraint_log_degree_bound(self: @RangeChecksComponents) -> u32 {
        let mut max_degree = 0;
        max_degree = core::cmp::max(max_degree, self.rc_6.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_8.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_11.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_12.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_18.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_18_b.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_19.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_19_b.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_19_c.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_19_d.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_19_e.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_19_f.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_19_g.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_19_h.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_4_3.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_4_4.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_5_4.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_9_9.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_9_9_b.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_9_9_c.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_9_9_d.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_9_9_e.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_9_9_f.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_9_9_g.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_9_9_h.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_7_2_5.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_3_6_6_3.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_4_4_4_4.max_constraint_log_degree_bound());
        max_degree =
            core::cmp::max(max_degree, self.rc_3_3_3_3_3.max_constraint_log_degree_bound());
        max_degree
    }

    fn evaluate_constraints_at_point(
        self: @RangeChecksComponents,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
        point: CirclePoint<QM31>,
    ) {
        self
            .rc_6
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .rc_8
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .rc_11
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .rc_12
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .rc_18
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .rc_18_b
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .rc_19
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .rc_19_b
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .rc_19_c
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .rc_19_d
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .rc_19_e
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .rc_19_f
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .rc_19_g
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .rc_19_h
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .rc_4_3
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .rc_4_4
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .rc_5_4
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .rc_9_9
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .rc_9_9_b
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .rc_9_9_c
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .rc_9_9_d
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .rc_9_9_e
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .rc_9_9_f
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .rc_9_9_g
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .rc_9_9_h
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .rc_7_2_5
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .rc_3_6_6_3
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .rc_4_4_4_4
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .rc_3_3_3_3_3
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
    }
}

#[cfg(not(feature: "poseidon252_verifier"))]
#[derive(Drop)]
pub struct PedersenContextComponents {
    components: Option<PedersenComponents>,
}

#[generate_trait]
#[cfg(not(feature: "poseidon252_verifier"))]
impl PedersenContextComponentsImpl of PedersenContextComponentsTrait {
    fn new(
        claim: @PedersenContextClaim,
        interaction_elements: @CairoInteractionElements,
        interaction_claim: @PedersenContextInteractionClaim,
    ) -> PedersenContextComponents {
        if let Some(claim) = claim.claim {
            PedersenContextComponents {
                components: Some(
                    PedersenComponentsImpl::new(
                        claim,
                        interaction_elements,
                        interaction_claim.interaction_claim.as_snap().unwrap(),
                    ),
                ),
            }
        } else {
            PedersenContextComponents { components: None }
        }
    }

    fn max_constraint_log_degree_bound(self: @PedersenContextComponents) -> u32 {
        if let Option::Some(components) = self.components {
            components.max_constraint_log_degree_bound()
        } else {
            0
        }
    }

    fn mask_points(
        self: @PedersenContextComponents,
        ref preprocessed_column_set: PreprocessedColumnSet,
        ref trace_mask_points: Array<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: Array<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        if let Option::Some(components) = self.components {
            components
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }
    }

    fn evaluate_constraints_at_point(
        self: @PedersenContextComponents,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
        point: CirclePoint<QM31>,
    ) {
        if let Option::Some(components) = self.components {
            components
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }
    }
}

#[cfg(not(feature: "poseidon252_verifier"))]
#[derive(Drop)]
struct PedersenComponents {
    pub partial_ec_mul: components::partial_ec_mul::Component,
    pub pedersen_points_table: components::pedersen_points_table::Component,
}

#[cfg(not(feature: "poseidon252_verifier"))]
#[generate_trait]
impl PedersenComponentsImpl of PedersenComponentsTrait {
    fn new(
        claim: @PedersenClaim,
        interaction_elements: @CairoInteractionElements,
        interaction_claim: @PedersenInteractionClaim,
    ) -> PedersenComponents {
        let partial_ec_mul_component = components::partial_ec_mul::Component {
            claim: *claim.partial_ec_mul,
            interaction_claim: *interaction_claim.partial_ec_mul,
            partial_ec_mul_lookup_elements: interaction_elements.partial_ec_mul.clone(),
            pedersen_points_table_lookup_elements: interaction_elements
                .pedersen_points_table
                .clone(),
            range_check_19_lookup_elements: interaction_elements.range_checks.rc_19.clone(),
            range_check_19_b_lookup_elements: interaction_elements.range_checks.rc_19_b.clone(),
            range_check_19_c_lookup_elements: interaction_elements.range_checks.rc_19_c.clone(),
            range_check_19_d_lookup_elements: interaction_elements.range_checks.rc_19_d.clone(),
            range_check_19_e_lookup_elements: interaction_elements.range_checks.rc_19_e.clone(),
            range_check_19_f_lookup_elements: interaction_elements.range_checks.rc_19_f.clone(),
            range_check_19_g_lookup_elements: interaction_elements.range_checks.rc_19_g.clone(),
            range_check_19_h_lookup_elements: interaction_elements.range_checks.rc_19_h.clone(),
            range_check_9_9_lookup_elements: interaction_elements.range_checks.rc_9_9.clone(),
            range_check_9_9_b_lookup_elements: interaction_elements.range_checks.rc_9_9_b.clone(),
            range_check_9_9_c_lookup_elements: interaction_elements.range_checks.rc_9_9_c.clone(),
            range_check_9_9_d_lookup_elements: interaction_elements.range_checks.rc_9_9_d.clone(),
            range_check_9_9_e_lookup_elements: interaction_elements.range_checks.rc_9_9_e.clone(),
            range_check_9_9_f_lookup_elements: interaction_elements.range_checks.rc_9_9_f.clone(),
            range_check_9_9_g_lookup_elements: interaction_elements.range_checks.rc_9_9_g.clone(),
            range_check_9_9_h_lookup_elements: interaction_elements.range_checks.rc_9_9_h.clone(),
        };

        let pedersen_points_table_component = components::pedersen_points_table::Component {
            claim: *claim.pedersen_points_table,
            interaction_claim: *interaction_claim.pedersen_points_table,
            pedersen_points_table_lookup_elements: interaction_elements
                .pedersen_points_table
                .clone(),
        };

        PedersenComponents {
            partial_ec_mul: partial_ec_mul_component,
            pedersen_points_table: pedersen_points_table_component,
        }
    }

    fn mask_points(
        self: @PedersenComponents,
        ref preprocessed_column_set: PreprocessedColumnSet,
        ref trace_mask_points: Array<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: Array<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        self
            .partial_ec_mul
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .pedersen_points_table
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
    }

    fn evaluate_constraints_at_point(
        self: @PedersenComponents,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
        point: CirclePoint<QM31>,
    ) {
        self
            .partial_ec_mul
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .pedersen_points_table
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
    }

    fn max_constraint_log_degree_bound(self: @PedersenComponents) -> u32 {
        let mut max_degree = 0;
        max_degree =
            core::cmp::max(max_degree, self.partial_ec_mul.max_constraint_log_degree_bound());
        max_degree =
            core::cmp::max(
                max_degree, self.pedersen_points_table.max_constraint_log_degree_bound(),
            );
        max_degree
    }
}

#[cfg(not(feature: "poseidon252_verifier"))]
#[derive(Drop)]
pub struct PoseidonContextComponents {
    components: Option<PoseidonComponents>,
}

#[generate_trait]
#[cfg(not(feature: "poseidon252_verifier"))]
impl PoseidonContextComponentsImpl of PoseidonContextComponentsTrait {
    fn new(
        claim: @PoseidonContextClaim,
        interaction_elements: @CairoInteractionElements,
        interaction_claim: @PoseidonContextInteractionClaim,
    ) -> PoseidonContextComponents {
        if let Some(claim) = claim.claim {
            PoseidonContextComponents {
                components: Some(
                    PoseidonComponentsImpl::new(
                        claim,
                        interaction_elements,
                        interaction_claim.interaction_claim.as_snap().unwrap(),
                    ),
                ),
            }
        } else {
            PoseidonContextComponents { components: None }
        }
    }

    fn max_constraint_log_degree_bound(self: @PoseidonContextComponents) -> u32 {
        if let Option::Some(components) = self.components {
            components.max_constraint_log_degree_bound()
        } else {
            0
        }
    }

    fn mask_points(
        self: @PoseidonContextComponents,
        ref preprocessed_column_set: PreprocessedColumnSet,
        ref trace_mask_points: Array<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: Array<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        if let Option::Some(components) = self.components {
            components
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }
    }

    fn evaluate_constraints_at_point(
        self: @PoseidonContextComponents,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
        point: CirclePoint<QM31>,
    ) {
        if let Option::Some(components) = self.components {
            components
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }
    }
}

#[cfg(not(feature: "poseidon252_verifier"))]
#[derive(Drop)]
struct PoseidonComponents {
    pub poseidon_3_partial_rounds_chain: components::poseidon_3_partial_rounds_chain::Component,
    pub poseidon_full_round_chain: components::poseidon_full_round_chain::Component,
    pub cube_252: components::cube_252::Component,
    pub poseidon_round_keys: components::poseidon_round_keys::Component,
    pub range_check_felt_252_width_27: components::range_check_felt_252_width_27::Component,
}

#[cfg(not(feature: "poseidon252_verifier"))]
#[generate_trait]
impl PoseidonComponentsImpl of PoseidonComponentsTrait {
    fn new(
        claim: @PoseidonClaim,
        interaction_elements: @CairoInteractionElements,
        interaction_claim: @PoseidonInteractionClaim,
    ) -> PoseidonComponents {
        let poseidon_3_partial_rounds_chain_component =
            components::poseidon_3_partial_rounds_chain::Component {
            claim: *claim.poseidon_3_partial_rounds_chain,
            interaction_claim: *interaction_claim.poseidon_3_partial_rounds_chain,
            range_check_4_4_lookup_elements: interaction_elements.range_checks.rc_4_4.clone(),
            range_check_4_4_4_4_lookup_elements: interaction_elements
                .range_checks
                .rc_4_4_4_4
                .clone(),
            poseidon_3_partial_rounds_chain_lookup_elements: interaction_elements
                .poseidon_3_partial_rounds_chain
                .clone(),
            range_check_felt_252_width_27_lookup_elements: interaction_elements
                .range_check_felt_252_width_27
                .clone(),
            cube_252_lookup_elements: interaction_elements.cube_252.clone(),
            poseidon_round_keys_lookup_elements: interaction_elements.poseidon_round_keys.clone(),
        };

        let poseidon_full_round_chain_component = components::poseidon_full_round_chain::Component {
            claim: *claim.poseidon_full_round_chain,
            interaction_claim: *interaction_claim.poseidon_full_round_chain,
            cube_252_lookup_elements: interaction_elements.cube_252.clone(),
            range_check_3_3_3_3_3_lookup_elements: interaction_elements
                .range_checks
                .rc_3_3_3_3_3
                .clone(),
            poseidon_full_round_chain_lookup_elements: interaction_elements
                .poseidon_full_round_chain
                .clone(),
            poseidon_round_keys_lookup_elements: interaction_elements.poseidon_round_keys.clone(),
        };

        let cube_252_component = components::cube_252::Component {
            claim: *claim.cube_252,
            interaction_claim: *interaction_claim.cube_252,
            cube_252_lookup_elements: interaction_elements.cube_252.clone(),
            range_check_19_lookup_elements: interaction_elements.range_checks.rc_19.clone(),
            range_check_19_b_lookup_elements: interaction_elements.range_checks.rc_19_b.clone(),
            range_check_19_c_lookup_elements: interaction_elements.range_checks.rc_19_c.clone(),
            range_check_19_d_lookup_elements: interaction_elements.range_checks.rc_19_d.clone(),
            range_check_19_e_lookup_elements: interaction_elements.range_checks.rc_19_e.clone(),
            range_check_19_f_lookup_elements: interaction_elements.range_checks.rc_19_f.clone(),
            range_check_19_g_lookup_elements: interaction_elements.range_checks.rc_19_g.clone(),
            range_check_19_h_lookup_elements: interaction_elements.range_checks.rc_19_h.clone(),
            range_check_9_9_lookup_elements: interaction_elements.range_checks.rc_9_9.clone(),
            range_check_9_9_b_lookup_elements: interaction_elements.range_checks.rc_9_9_b.clone(),
            range_check_9_9_c_lookup_elements: interaction_elements.range_checks.rc_9_9_c.clone(),
            range_check_9_9_d_lookup_elements: interaction_elements.range_checks.rc_9_9_d.clone(),
            range_check_9_9_e_lookup_elements: interaction_elements.range_checks.rc_9_9_e.clone(),
            range_check_9_9_f_lookup_elements: interaction_elements.range_checks.rc_9_9_f.clone(),
            range_check_9_9_g_lookup_elements: interaction_elements.range_checks.rc_9_9_g.clone(),
            range_check_9_9_h_lookup_elements: interaction_elements.range_checks.rc_9_9_h.clone(),
        };

        let poseidon_round_keys_component = components::poseidon_round_keys::Component {
            claim: *claim.poseidon_round_keys,
            interaction_claim: *interaction_claim.poseidon_round_keys,
            poseidon_round_keys_lookup_elements: interaction_elements.poseidon_round_keys.clone(),
        };

        let range_check_felt_252_width_27_component =
            components::range_check_felt_252_width_27::Component {
            claim: *claim.range_check_felt_252_width_27,
            interaction_claim: *interaction_claim.range_check_felt_252_width_27,
            range_check_18_lookup_elements: interaction_elements.range_checks.rc_18.clone(),
            range_check_18_b_lookup_elements: interaction_elements.range_checks.rc_18_b.clone(),
            range_check_9_9_lookup_elements: interaction_elements.range_checks.rc_9_9.clone(),
            range_check_9_9_b_lookup_elements: interaction_elements.range_checks.rc_9_9_b.clone(),
            range_check_9_9_c_lookup_elements: interaction_elements.range_checks.rc_9_9_c.clone(),
            range_check_9_9_d_lookup_elements: interaction_elements.range_checks.rc_9_9_d.clone(),
            range_check_9_9_e_lookup_elements: interaction_elements.range_checks.rc_9_9_e.clone(),
            range_check_felt_252_width_27_lookup_elements: interaction_elements
                .range_check_felt_252_width_27
                .clone(),
        };

        PoseidonComponents {
            poseidon_3_partial_rounds_chain: poseidon_3_partial_rounds_chain_component,
            poseidon_full_round_chain: poseidon_full_round_chain_component,
            cube_252: cube_252_component,
            poseidon_round_keys: poseidon_round_keys_component,
            range_check_felt_252_width_27: range_check_felt_252_width_27_component,
        }
    }

    fn mask_points(
        self: @PoseidonComponents,
        ref preprocessed_column_set: PreprocessedColumnSet,
        ref trace_mask_points: Array<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: Array<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        self
            .poseidon_3_partial_rounds_chain
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .poseidon_full_round_chain
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .cube_252
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .poseidon_round_keys
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .range_check_felt_252_width_27
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
    }

    fn evaluate_constraints_at_point(
        self: @PoseidonComponents,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
        point: CirclePoint<QM31>,
    ) {
        self
            .poseidon_3_partial_rounds_chain
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .poseidon_full_round_chain
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .cube_252
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .poseidon_round_keys
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .range_check_felt_252_width_27
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
    }

    fn max_constraint_log_degree_bound(self: @PoseidonComponents) -> u32 {
        let mut max_degree = 0;
        max_degree =
            core::cmp::max(
                max_degree, self.poseidon_3_partial_rounds_chain.max_constraint_log_degree_bound(),
            );
        max_degree =
            core::cmp::max(
                max_degree, self.poseidon_full_round_chain.max_constraint_log_degree_bound(),
            );
        max_degree = core::cmp::max(max_degree, self.cube_252.max_constraint_log_degree_bound());
        max_degree =
            core::cmp::max(max_degree, self.poseidon_round_keys.max_constraint_log_degree_bound());
        max_degree =
            core::cmp::max(
                max_degree, self.range_check_felt_252_width_27.max_constraint_log_degree_bound(),
            );
        max_degree
    }
}

#[derive(Drop)]
pub struct BlakeContextComponents {
    components: Option<BlakeComponents>,
}

#[generate_trait]
impl BlakeContextComponentsImpl of BlakeContextComponentsTrait {
    fn new(
        claim: @BlakeContextClaim,
        interaction_elements: @CairoInteractionElements,
        interaction_claim: @BlakeContextInteractionClaim,
    ) -> BlakeContextComponents {
        if let Some(claim) = claim.claim {
            BlakeContextComponents {
                components: Some(
                    BlakeComponentsImpl::new(
                        claim,
                        interaction_elements,
                        interaction_claim.interaction_claim.as_snap().unwrap(),
                    ),
                ),
            }
        } else {
            BlakeContextComponents { components: None }
        }
    }

    fn max_constraint_log_degree_bound(self: @BlakeContextComponents) -> u32 {
        if let Option::Some(components) = self.components {
            components.max_constraint_log_degree_bound()
        } else {
            0
        }
    }

    fn mask_points(
        self: @BlakeContextComponents,
        ref preprocessed_column_set: PreprocessedColumnSet,
        ref trace_mask_points: Array<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: Array<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        if let Option::Some(components) = self.components {
            components
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }
    }

    fn evaluate_constraints_at_point(
        self: @BlakeContextComponents,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
        point: CirclePoint<QM31>,
    ) {
        if let Option::Some(components) = self.components {
            components
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }
    }
}

#[derive(Drop)]
struct BlakeComponents {
    pub blake_round: components::blake_round::Component,
    pub blake_g: components::blake_g::Component,
    pub blake_round_sigma: components::blake_round_sigma::Component,
    pub triple_xor_32: components::triple_xor_32::Component,
    pub verify_bitwise_xor_12: components::verify_bitwise_xor_12::Component,
}

#[generate_trait]
impl BlakeComponentsImpl of BlakeComponentsTrait {
    fn new(
        claim: @BlakeClaim,
        interaction_elements: @CairoInteractionElements,
        interaction_claim: @BlakeInteractionClaim,
    ) -> BlakeComponents {
        let blake_round_component = components::blake_round::Component {
            claim: *claim.blake_round,
            interaction_claim: *interaction_claim.blake_round,
            blake_round_lookup_elements: interaction_elements.blake_round.clone(),
            blake_g_lookup_elements: interaction_elements.blake_g.clone(),
            blake_round_sigma_lookup_elements: interaction_elements.blake_round_sigma.clone(),
            memory_address_to_id_lookup_elements: interaction_elements.memory_address_to_id.clone(),
            memory_id_to_big_lookup_elements: interaction_elements.memory_id_to_value.clone(),
            range_check_7_2_5_lookup_elements: interaction_elements.range_checks.rc_7_2_5.clone(),
        };

        let blake_g_component = components::blake_g::Component {
            claim: *claim.blake_g,
            interaction_claim: *interaction_claim.blake_g,
            blake_g_lookup_elements: interaction_elements.blake_g.clone(),
            verify_bitwise_xor_12_lookup_elements: interaction_elements
                .verify_bitwise_xor_12
                .clone(),
            verify_bitwise_xor_4_lookup_elements: interaction_elements.verify_bitwise_xor_4.clone(),
            verify_bitwise_xor_7_lookup_elements: interaction_elements.verify_bitwise_xor_7.clone(),
            verify_bitwise_xor_8_lookup_elements: interaction_elements.verify_bitwise_xor_8.clone(),
            verify_bitwise_xor_9_lookup_elements: interaction_elements.verify_bitwise_xor_9.clone(),
        };

        let blake_round_sigma_component = components::blake_round_sigma::Component {
            claim: *claim.blake_round_sigma,
            interaction_claim: *interaction_claim.blake_round_sigma,
            blake_round_sigma_lookup_elements: interaction_elements.blake_round_sigma.clone(),
        };

        let triple_xor_32_component = components::triple_xor_32::Component {
            claim: *claim.triple_xor_32,
            interaction_claim: *interaction_claim.triple_xor_32,
            verify_bitwise_xor_8_lookup_elements: interaction_elements.verify_bitwise_xor_8.clone(),
            triple_xor_32_lookup_elements: interaction_elements.triple_xor_32.clone(),
        };

        let verify_bitwise_xor_12_component = components::verify_bitwise_xor_12::Component {
            claim: *claim.verify_bitwise_xor_12,
            interaction_claim: *interaction_claim.verify_bitwise_xor_12,
            verify_bitwise_xor_12_lookup_elements: interaction_elements
                .verify_bitwise_xor_12
                .clone(),
        };

        BlakeComponents {
            blake_round: blake_round_component,
            blake_g: blake_g_component,
            blake_round_sigma: blake_round_sigma_component,
            triple_xor_32: triple_xor_32_component,
            verify_bitwise_xor_12: verify_bitwise_xor_12_component,
        }
    }

    fn mask_points(
        self: @BlakeComponents,
        ref preprocessed_column_set: PreprocessedColumnSet,
        ref trace_mask_points: Array<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: Array<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        self
            .blake_round
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .blake_g
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .blake_round_sigma
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .triple_xor_32
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .verify_bitwise_xor_12
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
    }

    fn evaluate_constraints_at_point(
        self: @BlakeComponents,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
        point: CirclePoint<QM31>,
    ) {
        self
            .blake_round
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .blake_g
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .blake_round_sigma
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .triple_xor_32
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .verify_bitwise_xor_12
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
    }

    fn max_constraint_log_degree_bound(self: @BlakeComponents) -> u32 {
        let mut max_degree = 0;
        max_degree = core::cmp::max(max_degree, self.blake_round.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.blake_g.max_constraint_log_degree_bound());
        max_degree =
            core::cmp::max(max_degree, self.blake_round_sigma.max_constraint_log_degree_bound());
        max_degree =
            core::cmp::max(max_degree, self.triple_xor_32.max_constraint_log_degree_bound());
        max_degree =
            core::cmp::max(
                max_degree, self.verify_bitwise_xor_12.max_constraint_log_degree_bound(),
            );
        max_degree
    }
}

#[derive(Drop)]
#[cfg(not(feature: "poseidon252_verifier"))]
pub struct BuiltinComponents {
    pub add_mod_builtin: Option<components::add_mod_builtin::Component>,
    pub bitwise_builtin: Option<components::bitwise_builtin::Component>,
    pub mul_mod_builtin: Option<components::mul_mod_builtin::Component>,
    pub pedersen_builtin: Option<components::pedersen_builtin::Component>,
    pub poseidon_builtin: Option<components::poseidon_builtin::Component>,
    pub range_check_96_builtin: Option<components::range_check_builtin_bits_96::Component>,
    pub range_check_128_builtin: Option<components::range_check_builtin_bits_128::Component>,
}

#[derive(Drop)]
#[cfg(feature: "poseidon252_verifier")]
pub struct BuiltinComponents {
    pub bitwise_builtin: Option<components::bitwise_builtin::Component>,
    pub range_check_128_builtin: Option<components::range_check_builtin_bits_128::Component>,
}

#[generate_trait]
#[cfg(not(feature: "poseidon252_verifier"))]
impl BuiltinComponentsImpl of BuiltinComponentsTrait {
    fn new(
        claim: @BuiltinsClaim,
        interaction_elements: @CairoInteractionElements,
        interaction_claim: @BuiltinsInteractionClaim,
    ) -> BuiltinComponents {
        let mut add_mod_builtin_component = Option::None;
        if let Option::Some(claim) = claim.add_mod_builtin {
            add_mod_builtin_component =
                Option::Some(
                    components::add_mod_builtin::Component {
                        claim: *claim,
                        interaction_claim: (*interaction_claim.add_mod_builtin).unwrap(),
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                    },
                );
        }

        let mut bitwise_builtin_component = Option::None;
        if let Option::Some(claim) = claim.bitwise_builtin {
            bitwise_builtin_component =
                Option::Some(
                    components::bitwise_builtin::Component {
                        claim: *claim,
                        interaction_claim: (*interaction_claim.bitwise_builtin).unwrap(),
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        verify_bitwise_xor_9_lookup_elements: interaction_elements
                            .verify_bitwise_xor_9
                            .clone(),
                    },
                );
        }

        let mut mul_mod_builtin_component = Option::None;
        if let Option::Some(claim) = claim.mul_mod_builtin {
            mul_mod_builtin_component =
                Option::Some(
                    components::mul_mod_builtin::Component {
                        claim: *claim,
                        interaction_claim: (*interaction_claim.mul_mod_builtin).unwrap(),
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        range_check_12_lookup_elements: interaction_elements
                            .range_checks
                            .rc_12
                            .clone(),
                        range_check_18_lookup_elements: interaction_elements
                            .range_checks
                            .rc_18
                            .clone(),
                        range_check_3_6_6_3_lookup_elements: interaction_elements
                            .range_checks
                            .rc_3_6_6_3
                            .clone(),
                    },
                );
        }

        let mut pedersen_builtin_component = Option::None;
        if let Option::Some(claim) = claim.pedersen_builtin {
            pedersen_builtin_component =
                Option::Some(
                    components::pedersen_builtin::Component {
                        claim: *claim,
                        interaction_claim: (*interaction_claim.pedersen_builtin).unwrap(),
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        range_check_8_lookup_elements: interaction_elements
                            .range_checks
                            .rc_8
                            .clone(),
                        range_check_5_4_lookup_elements: interaction_elements
                            .range_checks
                            .rc_5_4
                            .clone(),
                        partial_ec_mul_lookup_elements: interaction_elements.partial_ec_mul.clone(),
                    },
                );
        }

        let mut poseidon_builtin_component = Option::None;
        if let Option::Some(claim) = claim.poseidon_builtin {
            poseidon_builtin_component =
                Option::Some(
                    components::poseidon_builtin::Component {
                        claim: *claim,
                        interaction_claim: (*interaction_claim.poseidon_builtin).unwrap(),
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        cube_252_lookup_elements: interaction_elements.cube_252.clone(),
                        poseidon_3_partial_rounds_chain_lookup_elements: interaction_elements
                            .poseidon_3_partial_rounds_chain
                            .clone(),
                        range_check_3_3_3_3_3_lookup_elements: interaction_elements
                            .range_checks
                            .rc_3_3_3_3_3
                            .clone(),
                        range_check_4_4_4_4_lookup_elements: interaction_elements
                            .range_checks
                            .rc_4_4_4_4
                            .clone(),
                        range_check_4_4_lookup_elements: interaction_elements
                            .range_checks
                            .rc_4_4
                            .clone(),
                        poseidon_full_round_chain_lookup_elements: interaction_elements
                            .poseidon_full_round_chain
                            .clone(),
                        range_check_felt_252_width_27_lookup_elements: interaction_elements
                            .range_check_felt_252_width_27
                            .clone(),
                    },
                );
        }

        let mut range_check_96_builtin_component = Option::None;
        if let Option::Some(claim) = claim.range_check_96_builtin {
            range_check_96_builtin_component =
                Option::Some(
                    components::range_check_builtin_bits_96::Component {
                        claim: *claim,
                        interaction_claim: (*interaction_claim.range_check_96_builtin).unwrap(),
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        range_check_6_lookup_elements: interaction_elements
                            .range_checks
                            .rc_6
                            .clone(),
                    },
                );
        }

        let mut range_check_128_builtin_component = Option::None;
        if let Option::Some(claim) = claim.range_check_128_builtin {
            range_check_128_builtin_component =
                Option::Some(
                    components::range_check_builtin_bits_128::Component {
                        claim: *claim,
                        interaction_claim: (*interaction_claim.range_check_128_builtin).unwrap(),
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                    },
                );
        }

        BuiltinComponents {
            add_mod_builtin: add_mod_builtin_component,
            bitwise_builtin: bitwise_builtin_component,
            mul_mod_builtin: mul_mod_builtin_component,
            pedersen_builtin: pedersen_builtin_component,
            poseidon_builtin: poseidon_builtin_component,
            range_check_96_builtin: range_check_96_builtin_component,
            range_check_128_builtin: range_check_128_builtin_component,
        }
    }

    fn mask_points(
        self: @BuiltinComponents,
        ref preprocessed_column_set: PreprocessedColumnSet,
        ref trace_mask_points: Array<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: Array<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        if let Option::Some(component) = self.add_mod_builtin.as_snap() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        if let Option::Some(component) = self.bitwise_builtin.as_snap() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        if let Option::Some(component) = self.mul_mod_builtin.as_snap() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        if let Option::Some(component) = self.pedersen_builtin.as_snap() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        if let Option::Some(component) = self.poseidon_builtin.as_snap() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        if let Option::Some(component) = self.range_check_96_builtin.as_snap() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        if let Option::Some(component) = self.range_check_128_builtin.as_snap() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }
    }

    fn max_constraint_log_degree_bound(self: @BuiltinComponents) -> u32 {
        let mut max_degree = 0;

        if let Option::Some(component) = self.add_mod_builtin.as_snap() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        if let Option::Some(component) = self.bitwise_builtin.as_snap() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        if let Option::Some(component) = self.mul_mod_builtin.as_snap() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        if let Option::Some(component) = self.pedersen_builtin.as_snap() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        if let Option::Some(component) = self.poseidon_builtin.as_snap() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        if let Option::Some(component) = self.range_check_96_builtin.as_snap() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        if let Option::Some(component) = self.range_check_128_builtin.as_snap() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        max_degree
    }


    fn evaluate_constraints_at_point(
        self: @BuiltinComponents,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
        point: CirclePoint<QM31>,
    ) {
        if let Option::Some(component) = self.add_mod_builtin.as_snap() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        if let Option::Some(component) = self.bitwise_builtin.as_snap() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        if let Option::Some(component) = self.mul_mod_builtin.as_snap() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        if let Option::Some(component) = self.pedersen_builtin.as_snap() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        if let Option::Some(component) = self.poseidon_builtin.as_snap() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        if let Option::Some(component) = self.range_check_96_builtin.as_snap() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        if let Option::Some(component) = self.range_check_128_builtin.as_snap() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }
    }
}

#[generate_trait]
#[cfg(feature: "poseidon252_verifier")]
impl BuiltinComponentsImpl of BuiltinComponentsTrait {
    fn new(
        claim: @BuiltinsClaim,
        interaction_elements: @CairoInteractionElements,
        interaction_claim: @BuiltinsInteractionClaim,
    ) -> BuiltinComponents {
        let mut bitwise_builtin_component = Option::None;
        if let Option::Some(claim) = claim.bitwise_builtin {
            bitwise_builtin_component =
                Option::Some(
                    components::bitwise_builtin::Component {
                        claim: *claim,
                        interaction_claim: (*interaction_claim.bitwise_builtin).unwrap(),
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        verify_bitwise_xor_9_lookup_elements: interaction_elements
                            .verify_bitwise_xor_9
                            .clone(),
                    },
                );
        }

        let mut range_check_128_builtin_component = Option::None;
        if let Option::Some(claim) = claim.range_check_128_builtin {
            range_check_128_builtin_component =
                Option::Some(
                    components::range_check_builtin_bits_128::Component {
                        claim: *claim,
                        interaction_claim: (*interaction_claim.range_check_128_builtin).unwrap(),
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                    },
                );
        }

        BuiltinComponents {
            bitwise_builtin: bitwise_builtin_component,
            range_check_128_builtin: range_check_128_builtin_component,
        }
    }

    fn mask_points(
        self: @BuiltinComponents,
        ref preprocessed_column_set: PreprocessedColumnSet,
        ref trace_mask_points: Array<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: Array<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        if let Option::Some(component) = self.bitwise_builtin.as_snap() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        if let Option::Some(component) = self.range_check_128_builtin.as_snap() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }
    }

    fn max_constraint_log_degree_bound(self: @BuiltinComponents) -> u32 {
        let mut max_degree = 0;

        if let Option::Some(component) = self.bitwise_builtin.as_snap() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        if let Option::Some(component) = self.range_check_128_builtin.as_snap() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        max_degree
    }


    fn evaluate_constraints_at_point(
        self: @BuiltinComponents,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
        point: CirclePoint<QM31>,
    ) {
        if let Option::Some(component) = self.bitwise_builtin.as_snap() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        if let Option::Some(component) = self.range_check_128_builtin.as_snap() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }
    }
}

#[cfg(not(feature: "poseidon252_verifier"))]
#[derive(Drop)]
pub struct OpcodeComponents {
    add: Array<components::add_opcode::Component>,
    add_small: Array<components::add_opcode_small::Component>,
    add_ap: Array<components::add_ap_opcode::Component>,
    assert_eq: Array<components::assert_eq_opcode::Component>,
    assert_eq_imm: Array<components::assert_eq_opcode_imm::Component>,
    assert_eq_double_deref: Array<components::assert_eq_opcode_double_deref::Component>,
    blake: Array<components::blake_compress_opcode::Component>,
    call: Array<components::call_opcode::Component>,
    call_rel_imm: Array<components::call_opcode_rel_imm::Component>,
    generic: Array<components::generic_opcode::Component>,
    jnz: Array<components::jnz_opcode::Component>,
    jnz_taken: Array<components::jnz_opcode_taken::Component>,
    jump: Array<components::jump_opcode::Component>,
    jump_double_deref: Array<components::jump_opcode_double_deref::Component>,
    jump_rel: Array<components::jump_opcode_rel::Component>,
    jump_rel_imm: Array<components::jump_opcode_rel_imm::Component>,
    mul: Array<components::mul_opcode::Component>,
    mul_small: Array<components::mul_opcode_small::Component>,
    qm31: Array<components::qm_31_add_mul_opcode::Component>,
    ret: Array<components::ret_opcode::Component>,
}

#[cfg(feature: "poseidon252_verifier")]
#[derive(Drop)]
pub struct OpcodeComponents {
    add: Array<components::add_opcode::Component>,
    add_small: Array<components::add_opcode_small::Component>,
    add_ap: Array<components::add_ap_opcode::Component>,
    assert_eq: Array<components::assert_eq_opcode::Component>,
    assert_eq_imm: Array<components::assert_eq_opcode_imm::Component>,
    assert_eq_double_deref: Array<components::assert_eq_opcode_double_deref::Component>,
    blake: Array<components::blake_compress_opcode::Component>,
    call: Array<components::call_opcode::Component>,
    call_rel_imm: Array<components::call_opcode_rel_imm::Component>,
    jnz: Array<components::jnz_opcode::Component>,
    jnz_taken: Array<components::jnz_opcode_taken::Component>,
    jump: Array<components::jump_opcode::Component>,
    jump_double_deref: Array<components::jump_opcode_double_deref::Component>,
    jump_rel: Array<components::jump_opcode_rel::Component>,
    jump_rel_imm: Array<components::jump_opcode_rel_imm::Component>,
    mul: Array<components::mul_opcode::Component>,
    mul_small: Array<components::mul_opcode_small::Component>,
    qm31: Array<components::qm_31_add_mul_opcode::Component>,
    ret: Array<components::ret_opcode::Component>,
}

#[cfg(not(feature: "poseidon252_verifier"))]
#[generate_trait]
impl OpcodeComponentsImpl of OpcodeComponentsTrait {
    fn new(
        claim: @OpcodeClaim,
        interaction_elements: @CairoInteractionElements,
        interaction_claim: @OpcodeInteractionClaim,
    ) -> OpcodeComponents {
        // Add components
        let mut add_components = array![];
        let mut add_claims = claim.add.span();
        let mut add_interaction_claims = interaction_claim.add.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (add_claims.pop_front(), add_interaction_claims.pop_front()) {
            add_components
                .append(
                    components::add_opcode::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                );
        }
        assert!(add_claims.is_empty());
        assert!(add_interaction_claims.is_empty());

        // Add Small components
        let mut add_small_components = array![];
        let mut add_small_claims = claim.add_small.span();
        let mut add_small_interaction_claims = interaction_claim.add_small.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (add_small_claims.pop_front(), add_small_interaction_claims.pop_front()) {
            add_small_components
                .append(
                    components::add_opcode_small::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                );
        }
        assert!(add_small_claims.is_empty());
        assert!(add_small_interaction_claims.is_empty());

        // Add AP components
        let mut add_ap_components = array![];
        let mut add_ap_claims = claim.add_ap.span();
        let mut add_ap_interaction_claims = interaction_claim.add_ap.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (add_ap_claims.pop_front(), add_ap_interaction_claims.pop_front()) {
            add_ap_components
                .append(
                    components::add_ap_opcode::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                        range_check_19_lookup_elements: interaction_elements
                            .range_checks
                            .rc_19
                            .clone(),
                        range_check_8_lookup_elements: interaction_elements
                            .range_checks
                            .rc_8
                            .clone(),
                    },
                );
        }
        assert!(add_ap_claims.is_empty());
        assert!(add_ap_interaction_claims.is_empty());

        // Assert Eq components
        let mut assert_eq_components = array![];
        let mut assert_eq_claims = claim.assert_eq.span();
        let mut assert_eq_interaction_claims = interaction_claim.assert_eq.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (assert_eq_claims.pop_front(), assert_eq_interaction_claims.pop_front()) {
            assert_eq_components
                .append(
                    components::assert_eq_opcode::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                );
        }
        assert!(assert_eq_claims.is_empty());
        assert!(assert_eq_interaction_claims.is_empty());

        // Assert Eq Imm components
        let mut assert_eq_imm_components = array![];
        let mut assert_eq_imm_claims = claim.assert_eq_imm.span();
        let mut assert_eq_imm_interaction_claims = interaction_claim.assert_eq_imm.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (assert_eq_imm_claims.pop_front(), assert_eq_imm_interaction_claims.pop_front()) {
            assert_eq_imm_components
                .append(
                    components::assert_eq_opcode_imm::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                );
        }
        assert!(assert_eq_imm_claims.is_empty());
        assert!(assert_eq_imm_interaction_claims.is_empty());

        // Assert Eq Double Deref components
        let mut assert_eq_double_deref_components = array![];
        let mut assert_eq_double_deref_claims = claim.assert_eq_double_deref.span();
        let mut assert_eq_double_deref_interaction_claims = interaction_claim
            .assert_eq_double_deref
            .span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (
                assert_eq_double_deref_claims.pop_front(),
                assert_eq_double_deref_interaction_claims.pop_front(),
            ) {
            assert_eq_double_deref_components
                .append(
                    components::assert_eq_opcode_double_deref::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                );
        }
        assert!(assert_eq_double_deref_claims.is_empty());
        assert!(assert_eq_double_deref_interaction_claims.is_empty());

        let mut blake_components = array![];
        let mut blake_claims = claim.blake.span();
        let mut blake_interaction_claims = interaction_claim.blake.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (blake_claims.pop_front(), blake_interaction_claims.pop_front()) {
            blake_components
                .append(
                    components::blake_compress_opcode::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        range_check_7_2_5_lookup_elements: interaction_elements
                            .range_checks
                            .rc_7_2_5
                            .clone(),
                        triple_xor_32_lookup_elements: interaction_elements.triple_xor_32.clone(),
                        verify_bitwise_xor_8_lookup_elements: interaction_elements
                            .verify_bitwise_xor_8
                            .clone(),
                        blake_round_lookup_elements: interaction_elements.blake_round.clone(),
                    },
                );
        }
        assert!(blake_claims.is_empty());
        assert!(blake_interaction_claims.is_empty());

        // Call components
        let mut call_components = array![];
        let mut call_claims = claim.call.span();
        let mut call_interaction_claims = interaction_claim.call.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (call_claims.pop_front(), call_interaction_claims.pop_front()) {
            call_components
                .append(
                    components::call_opcode::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                );
        }
        assert!(call_claims.is_empty());
        assert!(call_interaction_claims.is_empty());

        // Call Rel_imm components
        let mut call_rel_imm_components = array![];
        let mut call_rel_imm_claims = claim.call_rel_imm.span();
        let mut call_rel_imm_interaction_claims = interaction_claim.call_rel_imm.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (call_rel_imm_claims.pop_front(), call_rel_imm_interaction_claims.pop_front()) {
            call_rel_imm_components
                .append(
                    components::call_opcode_rel_imm::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                );
        }
        assert!(call_rel_imm_claims.is_empty());
        assert!(call_rel_imm_interaction_claims.is_empty());

        // Generic components
        let mut generic_components = array![];
        let mut generic_claims = claim.generic.span();
        let mut generic_interaction_claims = interaction_claim.generic.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (generic_claims.pop_front(), generic_interaction_claims.pop_front()) {
            generic_components
                .append(
                    components::generic_opcode::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                        range_check_19_lookup_elements: interaction_elements
                            .range_checks
                            .rc_19
                            .clone(),
                        range_check_19_b_lookup_elements: interaction_elements
                            .range_checks
                            .rc_19_b
                            .clone(),
                        range_check_19_c_lookup_elements: interaction_elements
                            .range_checks
                            .rc_19_c
                            .clone(),
                        range_check_19_d_lookup_elements: interaction_elements
                            .range_checks
                            .rc_19_d
                            .clone(),
                        range_check_19_e_lookup_elements: interaction_elements
                            .range_checks
                            .rc_19_e
                            .clone(),
                        range_check_19_f_lookup_elements: interaction_elements
                            .range_checks
                            .rc_19_f
                            .clone(),
                        range_check_19_g_lookup_elements: interaction_elements
                            .range_checks
                            .rc_19_g
                            .clone(),
                        range_check_19_h_lookup_elements: interaction_elements
                            .range_checks
                            .rc_19_h
                            .clone(),
                        range_check_9_9_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9
                            .clone(),
                        range_check_9_9_b_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9_b
                            .clone(),
                        range_check_9_9_c_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9_c
                            .clone(),
                        range_check_9_9_d_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9_d
                            .clone(),
                        range_check_9_9_e_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9_e
                            .clone(),
                        range_check_9_9_f_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9_f
                            .clone(),
                        range_check_9_9_g_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9_g
                            .clone(),
                        range_check_9_9_h_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9_h
                            .clone(),
                    },
                );
        }
        assert!(generic_claims.is_empty());
        assert!(generic_interaction_claims.is_empty());

        // Jnz components
        let mut jnz_components = array![];
        let mut jnz_claims = claim.jnz.span();
        let mut jnz_interaction_claims = interaction_claim.jnz.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (jnz_claims.pop_front(), jnz_interaction_claims.pop_front()) {
            jnz_components
                .append(
                    components::jnz_opcode::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                );
        }
        assert!(jnz_claims.is_empty());
        assert!(jnz_interaction_claims.is_empty());

        // Jnz Taken components
        let mut jnz_taken_components = array![];
        let mut jnz_taken_claims = claim.jnz_taken.span();
        let mut jnz_taken_interaction_claims = interaction_claim.jnz_taken.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (jnz_taken_claims.pop_front(), jnz_taken_interaction_claims.pop_front()) {
            jnz_taken_components
                .append(
                    components::jnz_opcode_taken::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                );
        }
        assert!(jnz_taken_claims.is_empty());
        assert!(jnz_taken_interaction_claims.is_empty());

        // Jump components
        let mut jump_components = array![];
        let mut jump_claims = claim.jump.span();
        let mut jump_interaction_claims = interaction_claim.jump.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (jump_claims.pop_front(), jump_interaction_claims.pop_front()) {
            jump_components
                .append(
                    components::jump_opcode::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                );
        }
        assert!(jump_claims.is_empty());
        assert!(jump_interaction_claims.is_empty());

        // Jump Double Deref components
        let mut jump_double_deref_components = array![];
        let mut jump_double_deref_claims = claim.jump_double_deref.span();
        let mut jump_double_deref_interaction_claims = interaction_claim.jump_double_deref.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (
                jump_double_deref_claims.pop_front(),
                jump_double_deref_interaction_claims.pop_front(),
            ) {
            jump_double_deref_components
                .append(
                    components::jump_opcode_double_deref::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                );
        }
        assert!(jump_double_deref_claims.is_empty());
        assert!(jump_double_deref_interaction_claims.is_empty());

        // Jump Rel components
        let mut jump_rel_components = array![];
        let mut jump_rel_claims = claim.jump_rel.span();
        let mut jump_rel_interaction_claims = interaction_claim.jump_rel.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (jump_rel_claims.pop_front(), jump_rel_interaction_claims.pop_front()) {
            jump_rel_components
                .append(
                    components::jump_opcode_rel::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                );
        }
        assert!(jump_rel_claims.is_empty());
        assert!(jump_rel_interaction_claims.is_empty());

        // Jump Rel Imm components
        let mut jump_rel_imm_components = array![];
        let mut jump_rel_imm_claims = claim.jump_rel_imm.span();
        let mut jump_rel_imm_interaction_claims = interaction_claim.jump_rel_imm.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (jump_rel_imm_claims.pop_front(), jump_rel_imm_interaction_claims.pop_front()) {
            jump_rel_imm_components
                .append(
                    components::jump_opcode_rel_imm::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                );
        }
        assert!(jump_rel_imm_claims.is_empty());
        assert!(jump_rel_imm_interaction_claims.is_empty());

        // Mul components
        let mut mul_components = array![];
        let mut mul_claims = claim.mul.span();
        let mut mul_interaction_claims = interaction_claim.mul.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (mul_claims.pop_front(), mul_interaction_claims.pop_front()) {
            mul_components
                .append(
                    components::mul_opcode::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                        range_check_19_lookup_elements: interaction_elements
                            .range_checks
                            .rc_19
                            .clone(),
                        range_check_19_b_lookup_elements: interaction_elements
                            .range_checks
                            .rc_19_b
                            .clone(),
                        range_check_19_c_lookup_elements: interaction_elements
                            .range_checks
                            .rc_19_c
                            .clone(),
                        range_check_19_d_lookup_elements: interaction_elements
                            .range_checks
                            .rc_19_d
                            .clone(),
                        range_check_19_e_lookup_elements: interaction_elements
                            .range_checks
                            .rc_19_e
                            .clone(),
                        range_check_19_f_lookup_elements: interaction_elements
                            .range_checks
                            .rc_19_f
                            .clone(),
                        range_check_19_g_lookup_elements: interaction_elements
                            .range_checks
                            .rc_19_g
                            .clone(),
                        range_check_19_h_lookup_elements: interaction_elements
                            .range_checks
                            .rc_19_h
                            .clone(),
                    },
                );
        }
        assert!(mul_claims.is_empty());
        assert!(mul_interaction_claims.is_empty());

        // Mul Small components
        let mut mul_small_components = array![];
        let mut mul_small_claims = claim.mul_small.span();
        let mut mul_small_interaction_claims = interaction_claim.mul_small.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (mul_small_claims.pop_front(), mul_small_interaction_claims.pop_front()) {
            mul_small_components
                .append(
                    components::mul_opcode_small::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                        range_check_11_lookup_elements: interaction_elements
                            .range_checks
                            .rc_11
                            .clone(),
                    },
                );
        }
        assert!(mul_small_claims.is_empty());
        assert!(mul_small_interaction_claims.is_empty());

        // QM31 components
        let mut qm31_components = array![];
        let mut qm31_claims = claim.qm31.span();
        let mut qm31_interaction_claims = interaction_claim.qm31.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (qm31_claims.pop_front(), qm31_interaction_claims.pop_front()) {
            qm31_components
                .append(
                    components::qm_31_add_mul_opcode::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                        range_check_4_4_4_4_lookup_elements: interaction_elements
                            .range_checks
                            .rc_4_4_4_4
                            .clone(),
                    },
                );
        }
        assert!(qm31_claims.is_empty());
        assert!(qm31_interaction_claims.is_empty());

        // Ret components
        let mut ret_components = array![];
        let mut ret_claims = claim.ret.span();
        let mut ret_interaction_claims = interaction_claim.ret.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (ret_claims.pop_front(), ret_interaction_claims.pop_front()) {
            ret_components
                .append(
                    components::ret_opcode::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                );
        }
        assert!(ret_claims.is_empty());
        assert!(ret_interaction_claims.is_empty());

        OpcodeComponents {
            add: add_components,
            add_small: add_small_components,
            add_ap: add_ap_components,
            assert_eq: assert_eq_components,
            assert_eq_imm: assert_eq_imm_components,
            assert_eq_double_deref: assert_eq_double_deref_components,
            blake: blake_components,
            call: call_components,
            call_rel_imm: call_rel_imm_components,
            generic: generic_components,
            jnz: jnz_components,
            jnz_taken: jnz_taken_components,
            jump: jump_components,
            jump_double_deref: jump_double_deref_components,
            jump_rel: jump_rel_components,
            jump_rel_imm: jump_rel_imm_components,
            mul: mul_components,
            mul_small: mul_small_components,
            qm31: qm31_components,
            ret: ret_components,
        }
    }

    fn mask_points(
        self: @OpcodeComponents,
        ref preprocessed_column_set: PreprocessedColumnSet,
        ref trace_mask_points: Array<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: Array<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        for component in self.add.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.add_small.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.add_ap.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.assert_eq.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.assert_eq_imm.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.assert_eq_double_deref.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.blake.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.call.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.call_rel_imm.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.generic.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.jnz.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.jnz_taken.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.jump.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.jump_double_deref.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.jump_rel.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.jump_rel_imm.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.mul.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.mul_small.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.qm31.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.ret.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        };
    }

    fn max_constraint_log_degree_bound(self: @OpcodeComponents) -> u32 {
        let mut max_degree = 0;

        for component in self.add.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.add_small.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.add_ap.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.assert_eq.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.assert_eq_imm.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.assert_eq_double_deref.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.blake.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.call.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.call_rel_imm.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.generic.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.jnz.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.jnz_taken.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.jump.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.jump_double_deref.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.jump_rel.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.jump_rel_imm.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.mul.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.mul_small.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.qm31.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.ret.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        max_degree
    }

    fn evaluate_constraints_at_point(
        self: @OpcodeComponents,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
        point: CirclePoint<QM31>,
    ) {
        for component in self.add.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }
        for component in self.add_small.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }
        for component in self.add_ap.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }
        for component in self.assert_eq.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }
        for component in self.assert_eq_imm.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }
        for component in self.assert_eq_double_deref.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        for component in self.blake.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        for component in self.call.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        for component in self.call_rel_imm.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        for component in self.generic.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        for component in self.jnz.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        for component in self.jnz_taken.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        for component in self.jump.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        for component in self.jump_double_deref.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        for component in self.jump_rel.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        for component in self.jump_rel_imm.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        for component in self.mul.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }
        for component in self.mul_small.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        for component in self.qm31.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        for component in self.ret.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        };
    }
}

#[cfg(feature: "poseidon252_verifier")]
#[generate_trait]
impl OpcodeComponentsImpl of OpcodeComponentsTrait {
    fn new(
        claim: @OpcodeClaim,
        interaction_elements: @CairoInteractionElements,
        interaction_claim: @OpcodeInteractionClaim,
    ) -> OpcodeComponents {
        // Add components
        let mut add_components = array![];
        let mut add_claims = claim.add.span();
        let mut add_interaction_claims = interaction_claim.add.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (add_claims.pop_front(), add_interaction_claims.pop_front()) {
            add_components
                .append(
                    components::add_opcode::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                );
        }
        assert!(add_claims.is_empty());
        assert!(add_interaction_claims.is_empty());

        // Add Small components
        let mut add_small_components = array![];
        let mut add_small_claims = claim.add_small.span();
        let mut add_small_interaction_claims = interaction_claim.add_small.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (add_small_claims.pop_front(), add_small_interaction_claims.pop_front()) {
            add_small_components
                .append(
                    components::add_opcode_small::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                );
        }
        assert!(add_small_claims.is_empty());
        assert!(add_small_interaction_claims.is_empty());

        // Add AP components
        let mut add_ap_components = array![];
        let mut add_ap_claims = claim.add_ap.span();
        let mut add_ap_interaction_claims = interaction_claim.add_ap.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (add_ap_claims.pop_front(), add_ap_interaction_claims.pop_front()) {
            add_ap_components
                .append(
                    components::add_ap_opcode::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                        range_check_19_lookup_elements: interaction_elements
                            .range_checks
                            .rc_19
                            .clone(),
                        range_check_8_lookup_elements: interaction_elements
                            .range_checks
                            .rc_8
                            .clone(),
                    },
                );
        }
        assert!(add_ap_claims.is_empty());
        assert!(add_ap_interaction_claims.is_empty());

        // Assert Eq components
        let mut assert_eq_components = array![];
        let mut assert_eq_claims = claim.assert_eq.span();
        let mut assert_eq_interaction_claims = interaction_claim.assert_eq.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (assert_eq_claims.pop_front(), assert_eq_interaction_claims.pop_front()) {
            assert_eq_components
                .append(
                    components::assert_eq_opcode::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                );
        }
        assert!(assert_eq_claims.is_empty());
        assert!(assert_eq_interaction_claims.is_empty());

        // Assert Eq Imm components
        let mut assert_eq_imm_components = array![];
        let mut assert_eq_imm_claims = claim.assert_eq_imm.span();
        let mut assert_eq_imm_interaction_claims = interaction_claim.assert_eq_imm.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (assert_eq_imm_claims.pop_front(), assert_eq_imm_interaction_claims.pop_front()) {
            assert_eq_imm_components
                .append(
                    components::assert_eq_opcode_imm::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                );
        }
        assert!(assert_eq_imm_claims.is_empty());
        assert!(assert_eq_imm_interaction_claims.is_empty());

        // Assert Eq Double Deref components
        let mut assert_eq_double_deref_components = array![];
        let mut assert_eq_double_deref_claims = claim.assert_eq_double_deref.span();
        let mut assert_eq_double_deref_interaction_claims = interaction_claim
            .assert_eq_double_deref
            .span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (
                assert_eq_double_deref_claims.pop_front(),
                assert_eq_double_deref_interaction_claims.pop_front(),
            ) {
            assert_eq_double_deref_components
                .append(
                    components::assert_eq_opcode_double_deref::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                );
        }
        assert!(assert_eq_double_deref_claims.is_empty());
        assert!(assert_eq_double_deref_interaction_claims.is_empty());

        let mut blake_components = array![];
        let mut blake_claims = claim.blake.span();
        let mut blake_interaction_claims = interaction_claim.blake.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (blake_claims.pop_front(), blake_interaction_claims.pop_front()) {
            blake_components
                .append(
                    components::blake_compress_opcode::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        range_check_7_2_5_lookup_elements: interaction_elements
                            .range_checks
                            .rc_7_2_5
                            .clone(),
                        triple_xor_32_lookup_elements: interaction_elements.triple_xor_32.clone(),
                        verify_bitwise_xor_8_lookup_elements: interaction_elements
                            .verify_bitwise_xor_8
                            .clone(),
                        blake_round_lookup_elements: interaction_elements.blake_round.clone(),
                    },
                );
        }
        assert!(blake_claims.is_empty());
        assert!(blake_interaction_claims.is_empty());

        // Call components
        let mut call_components = array![];
        let mut call_claims = claim.call.span();
        let mut call_interaction_claims = interaction_claim.call.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (call_claims.pop_front(), call_interaction_claims.pop_front()) {
            call_components
                .append(
                    components::call_opcode::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                );
        }
        assert!(call_claims.is_empty());
        assert!(call_interaction_claims.is_empty());

        // Call Rel_imm components
        let mut call_rel_imm_components = array![];
        let mut call_rel_imm_claims = claim.call_rel_imm.span();
        let mut call_rel_imm_interaction_claims = interaction_claim.call_rel_imm.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (call_rel_imm_claims.pop_front(), call_rel_imm_interaction_claims.pop_front()) {
            call_rel_imm_components
                .append(
                    components::call_opcode_rel_imm::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                );
        }
        assert!(call_rel_imm_claims.is_empty());
        assert!(call_rel_imm_interaction_claims.is_empty());

        // Jnz components
        let mut jnz_components = array![];
        let mut jnz_claims = claim.jnz.span();
        let mut jnz_interaction_claims = interaction_claim.jnz.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (jnz_claims.pop_front(), jnz_interaction_claims.pop_front()) {
            jnz_components
                .append(
                    components::jnz_opcode::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                );
        }
        assert!(jnz_claims.is_empty());
        assert!(jnz_interaction_claims.is_empty());

        // Jnz Taken components
        let mut jnz_taken_components = array![];
        let mut jnz_taken_claims = claim.jnz_taken.span();
        let mut jnz_taken_interaction_claims = interaction_claim.jnz_taken.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (jnz_taken_claims.pop_front(), jnz_taken_interaction_claims.pop_front()) {
            jnz_taken_components
                .append(
                    components::jnz_opcode_taken::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                );
        }
        assert!(jnz_taken_claims.is_empty());
        assert!(jnz_taken_interaction_claims.is_empty());

        // Jump components
        let mut jump_components = array![];
        let mut jump_claims = claim.jump.span();
        let mut jump_interaction_claims = interaction_claim.jump.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (jump_claims.pop_front(), jump_interaction_claims.pop_front()) {
            jump_components
                .append(
                    components::jump_opcode::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                );
        }
        assert!(jump_claims.is_empty());
        assert!(jump_interaction_claims.is_empty());

        // Jump Double Deref components
        let mut jump_double_deref_components = array![];
        let mut jump_double_deref_claims = claim.jump_double_deref.span();
        let mut jump_double_deref_interaction_claims = interaction_claim.jump_double_deref.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (
                jump_double_deref_claims.pop_front(),
                jump_double_deref_interaction_claims.pop_front(),
            ) {
            jump_double_deref_components
                .append(
                    components::jump_opcode_double_deref::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                );
        }
        assert!(jump_double_deref_claims.is_empty());
        assert!(jump_double_deref_interaction_claims.is_empty());

        // Jump Rel components
        let mut jump_rel_components = array![];
        let mut jump_rel_claims = claim.jump_rel.span();
        let mut jump_rel_interaction_claims = interaction_claim.jump_rel.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (jump_rel_claims.pop_front(), jump_rel_interaction_claims.pop_front()) {
            jump_rel_components
                .append(
                    components::jump_opcode_rel::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                );
        }
        assert!(jump_rel_claims.is_empty());
        assert!(jump_rel_interaction_claims.is_empty());

        // Jump Rel Imm components
        let mut jump_rel_imm_components = array![];
        let mut jump_rel_imm_claims = claim.jump_rel_imm.span();
        let mut jump_rel_imm_interaction_claims = interaction_claim.jump_rel_imm.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (jump_rel_imm_claims.pop_front(), jump_rel_imm_interaction_claims.pop_front()) {
            jump_rel_imm_components
                .append(
                    components::jump_opcode_rel_imm::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                );
        }
        assert!(jump_rel_imm_claims.is_empty());
        assert!(jump_rel_imm_interaction_claims.is_empty());

        // Mul components
        let mut mul_components = array![];
        let mut mul_claims = claim.mul.span();
        let mut mul_interaction_claims = interaction_claim.mul.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (mul_claims.pop_front(), mul_interaction_claims.pop_front()) {
            mul_components
                .append(
                    components::mul_opcode::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                        range_check_19_lookup_elements: interaction_elements
                            .range_checks
                            .rc_19
                            .clone(),
                        range_check_19_b_lookup_elements: interaction_elements
                            .range_checks
                            .rc_19_b
                            .clone(),
                        range_check_19_c_lookup_elements: interaction_elements
                            .range_checks
                            .rc_19_c
                            .clone(),
                        range_check_19_d_lookup_elements: interaction_elements
                            .range_checks
                            .rc_19_d
                            .clone(),
                        range_check_19_e_lookup_elements: interaction_elements
                            .range_checks
                            .rc_19_e
                            .clone(),
                        range_check_19_f_lookup_elements: interaction_elements
                            .range_checks
                            .rc_19_f
                            .clone(),
                        range_check_19_g_lookup_elements: interaction_elements
                            .range_checks
                            .rc_19_g
                            .clone(),
                        range_check_19_h_lookup_elements: interaction_elements
                            .range_checks
                            .rc_19_h
                            .clone(),
                    },
                );
        }
        assert!(mul_claims.is_empty());
        assert!(mul_interaction_claims.is_empty());

        // Mul Small components
        let mut mul_small_components = array![];
        let mut mul_small_claims = claim.mul_small.span();
        let mut mul_small_interaction_claims = interaction_claim.mul_small.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (mul_small_claims.pop_front(), mul_small_interaction_claims.pop_front()) {
            mul_small_components
                .append(
                    components::mul_opcode_small::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                        range_check_11_lookup_elements: interaction_elements
                            .range_checks
                            .rc_11
                            .clone(),
                    },
                );
        }
        assert!(mul_small_claims.is_empty());
        assert!(mul_small_interaction_claims.is_empty());

        // QM31 components
        let mut qm31_components = array![];
        let mut qm31_claims = claim.qm31.span();
        let mut qm31_interaction_claims = interaction_claim.qm31.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (qm31_claims.pop_front(), qm31_interaction_claims.pop_front()) {
            qm31_components
                .append(
                    components::qm_31_add_mul_opcode::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                        range_check_4_4_4_4_lookup_elements: interaction_elements
                            .range_checks
                            .rc_4_4_4_4
                            .clone(),
                    },
                );
        }
        assert!(qm31_claims.is_empty());
        assert!(qm31_interaction_claims.is_empty());

        // Ret components
        let mut ret_components = array![];
        let mut ret_claims = claim.ret.span();
        let mut ret_interaction_claims = interaction_claim.ret.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (ret_claims.pop_front(), ret_interaction_claims.pop_front()) {
            ret_components
                .append(
                    components::ret_opcode::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                );
        }
        assert!(ret_claims.is_empty());
        assert!(ret_interaction_claims.is_empty());

        OpcodeComponents {
            add: add_components,
            add_small: add_small_components,
            add_ap: add_ap_components,
            assert_eq: assert_eq_components,
            assert_eq_imm: assert_eq_imm_components,
            assert_eq_double_deref: assert_eq_double_deref_components,
            blake: blake_components,
            call: call_components,
            call_rel_imm: call_rel_imm_components,
            jnz: jnz_components,
            jnz_taken: jnz_taken_components,
            jump: jump_components,
            jump_double_deref: jump_double_deref_components,
            jump_rel: jump_rel_components,
            jump_rel_imm: jump_rel_imm_components,
            mul: mul_components,
            mul_small: mul_small_components,
            qm31: qm31_components,
            ret: ret_components,
        }
    }

    fn mask_points(
        self: @OpcodeComponents,
        ref preprocessed_column_set: PreprocessedColumnSet,
        ref trace_mask_points: Array<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: Array<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        for component in self.add.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.add_small.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.add_ap.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.assert_eq.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.assert_eq_imm.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.assert_eq_double_deref.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.blake.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.call.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.call_rel_imm.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.jnz.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.jnz_taken.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.jump.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.jump_double_deref.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.jump_rel.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.jump_rel_imm.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.mul.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.mul_small.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.qm31.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.ret.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        };
    }

    fn max_constraint_log_degree_bound(self: @OpcodeComponents) -> u32 {
        let mut max_degree = 0;

        for component in self.add.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.add_small.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.add_ap.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.assert_eq.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.assert_eq_imm.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.assert_eq_double_deref.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.blake.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.call.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.call_rel_imm.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.jnz.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.jnz_taken.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.jump.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.jump_double_deref.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.jump_rel.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.jump_rel_imm.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.mul.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.mul_small.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.qm31.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.ret.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        max_degree
    }

    fn evaluate_constraints_at_point(
        self: @OpcodeComponents,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
        point: CirclePoint<QM31>,
    ) {
        for component in self.add.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }
        for component in self.add_small.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }
        for component in self.add_ap.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }
        for component in self.assert_eq.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }
        for component in self.assert_eq_imm.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }
        for component in self.assert_eq_double_deref.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        for component in self.blake.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        for component in self.call.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        for component in self.call_rel_imm.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        for component in self.jnz.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        for component in self.jnz_taken.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        for component in self.jump.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        for component in self.jump_double_deref.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        for component in self.jump_rel.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        for component in self.jump_rel_imm.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        for component in self.mul.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }
        for component in self.mul_small.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        for component in self.qm31.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        for component in self.ret.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        };
    }
}

#[cfg(test)]
mod tests {
    use core::num::traits::one::One;
    use stwo_constraint_framework::LookupElements;
    use stwo_verifier_core::fields::qm31::qm31_const;
    use stwo_verifier_core::utils::ArrayImpl;
    use super::{
        CairoInteractionElements, PublicData, PublicDataImpl, RangeChecksInteractionElements,
        RelationUsesDict, accumulate_relation_uses, hash_memory_section,
    };
    #[test]
    #[cairofmt::skip]
    fn test_public_data_logup_sum() {
        let mut public_data_felts = array![
            0, 228, 2520, 228, 2520, 0, 228, 2520, 228, 2520, 0, 228, 
            2520, 228, 2520, 0, 5, 0, 5, 0, 0, 228, 2520, 228, 
            2520, 0, 5, 0, 5, 0, 0, 5, 0, 5, 0, 0, 228, 2520, 228, 2520, 
            0, 228, 2520, 228, 2520, 0, 228, 2520, 228, 2520,
            0, 228, 2520, 228, 2520, 0, 2, 227, 1336, 0, 0, 0, 0, 0, 
            0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1336, 1336, 5, 2520, 1336]
        .span();
        let public_data: PublicData = Serde::deserialize(ref public_data_felts).unwrap();
        let dummy_lookup_elements = dummy_interaction_lookup_elements();

        let sum = public_data.logup_sum(@dummy_lookup_elements);

        assert_eq!(sum, qm31_const::<971792689, 636659210, 1237675822, 245392094>());
    }

    fn dummy_interaction_lookup_elements() -> CairoInteractionElements {
        CairoInteractionElements {
            opcodes: LookupElementsDummyImpl::dummy(),
            verify_instruction: LookupElementsDummyImpl::dummy(),
            blake_round: LookupElementsDummyImpl::dummy(),
            blake_g: LookupElementsDummyImpl::dummy(),
            blake_round_sigma: LookupElementsDummyImpl::dummy(),
            triple_xor_32: LookupElementsDummyImpl::dummy(),
            poseidon_3_partial_rounds_chain: LookupElementsDummyImpl::dummy(),
            poseidon_full_round_chain: LookupElementsDummyImpl::dummy(),
            cube_252: LookupElementsDummyImpl::dummy(),
            poseidon_round_keys: LookupElementsDummyImpl::dummy(),
            range_check_felt_252_width_27: LookupElementsDummyImpl::dummy(),
            partial_ec_mul: LookupElementsDummyImpl::dummy(),
            pedersen_points_table: LookupElementsDummyImpl::dummy(),
            memory_address_to_id: LookupElementsDummyImpl::dummy(),
            memory_id_to_value: LookupElementsDummyImpl::dummy(),
            range_checks: RangeChecksInteractionElements {
                rc_6: LookupElementsDummyImpl::dummy(),
                rc_8: LookupElementsDummyImpl::dummy(),
                rc_11: LookupElementsDummyImpl::dummy(),
                rc_12: LookupElementsDummyImpl::dummy(),
                rc_18: LookupElementsDummyImpl::dummy(),
                rc_18_b: LookupElementsDummyImpl::dummy(),
                rc_19: LookupElementsDummyImpl::dummy(),
                rc_19_b: LookupElementsDummyImpl::dummy(),
                rc_19_c: LookupElementsDummyImpl::dummy(),
                rc_19_d: LookupElementsDummyImpl::dummy(),
                rc_19_e: LookupElementsDummyImpl::dummy(),
                rc_19_f: LookupElementsDummyImpl::dummy(),
                rc_19_g: LookupElementsDummyImpl::dummy(),
                rc_19_h: LookupElementsDummyImpl::dummy(),
                rc_4_3: LookupElementsDummyImpl::dummy(),
                rc_4_4: LookupElementsDummyImpl::dummy(),
                rc_5_4: LookupElementsDummyImpl::dummy(),
                rc_9_9: LookupElementsDummyImpl::dummy(),
                rc_9_9_b: LookupElementsDummyImpl::dummy(),
                rc_9_9_c: LookupElementsDummyImpl::dummy(),
                rc_9_9_d: LookupElementsDummyImpl::dummy(),
                rc_9_9_e: LookupElementsDummyImpl::dummy(),
                rc_9_9_f: LookupElementsDummyImpl::dummy(),
                rc_9_9_g: LookupElementsDummyImpl::dummy(),
                rc_9_9_h: LookupElementsDummyImpl::dummy(),
                rc_7_2_5: LookupElementsDummyImpl::dummy(),
                rc_3_6_6_3: LookupElementsDummyImpl::dummy(),
                rc_4_4_4_4: LookupElementsDummyImpl::dummy(),
                rc_3_3_3_3_3: LookupElementsDummyImpl::dummy(),
            },
            verify_bitwise_xor_4: LookupElementsDummyImpl::dummy(),
            verify_bitwise_xor_7: LookupElementsDummyImpl::dummy(),
            verify_bitwise_xor_8: LookupElementsDummyImpl::dummy(),
            verify_bitwise_xor_9: LookupElementsDummyImpl::dummy(),
            verify_bitwise_xor_12: LookupElementsDummyImpl::dummy(),
        }
    }

    #[cfg(not(feature: "poseidon252_verifier"))]
    #[test]
    fn test_hash_memory_section() {
        let section = array![
            (0, [1, 2, 3, 4, 5, 6, 7, 8]), (0, [2, 3, 4, 5, 6, 7, 8, 9]),
            (0, [3, 4, 5, 6, 7, 8, 9, 10]),
        ];

        assert_eq!(
            hash_memory_section(@section).unbox(),
            [
                3098114871, 843612567, 2372208999, 1823639248, 1136624132, 2551058277, 1389013608,
                1207876589,
            ],
        );
    }

    #[cfg(feature: "poseidon252_verifier")]
    #[test]
    fn test_hash_memory_section() {
        let section = array![
            (0, [1, 2, 3, 4, 5, 6, 7, 8]), (0, [2, 3, 4, 5, 6, 7, 8, 9]),
            (0, [3, 4, 5, 6, 7, 8, 9, 10]),
        ];

        assert_eq!(
            hash_memory_section(@section).unbox(),
            [
                2433336977, 2153250057, 881002283, 2835163344, 2300811583, 376217666, 1436681392,
                91789842,
            ],
        );
    }

    #[generate_trait]
    impl LookupElementsDummyImpl<const N: usize> of LookupElementsDummyTrait<N> {
        fn dummy() -> LookupElements<N> {
            LookupElements::<
                N,
            > {
                z: qm31_const::<1, 2, 3, 4>(),
                alpha: One::one(),
                alpha_powers: ArrayImpl::new_repeated(N, One::one()),
            }
        }
    }

    #[test]
    fn test_accumulate_relation_uses() {
        let mut relation_uses: RelationUsesDict = Default::default();
        relation_uses.insert('relation_1', 4);
        relation_uses.insert('relation_2', 10);
        let log_size = 2;
        let relation_uses_per_row = [('relation_1', 2), ('relation_2', 4)];

        accumulate_relation_uses(ref relation_uses, relation_uses_per_row.span(), log_size);

        assert_eq!(relation_uses.get('relation_1'), 12);
        assert_eq!(relation_uses.get('relation_2'), 26);
    }
}
