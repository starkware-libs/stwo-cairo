use components::CairoComponent;
use components::add_ap_opcode::{
    ClaimImpl as AddApOpcodeClaimImpl, InteractionClaimImpl as AddApOpcodeInteractionClaimImpl,
};
use components::add_ap_opcode_imm::{
    ClaimImpl as AddApOpcodeImmClaimImpl,
    InteractionClaimImpl as AddApOpcodeImmInteractionClaimImpl,
};
use components::add_ap_opcode_op_1_base_fp::{
    ClaimImpl as AddApOpcodeOp1BaseFpClaimImpl,
    InteractionClaimImpl as AddApOpcodeOp1BaseFpInteractionClaimImpl,
};
use components::add_opcode::{
    ClaimImpl as AddOpcodeClaimImpl, InteractionClaimImpl as AddOpcodeInteractionClaimImpl,
};
use components::add_opcode_imm::{
    ClaimImpl as AddOpcodeImmClaimImpl, InteractionClaimImpl as AddOpcodeImmInteractionClaimImpl,
};
use components::add_opcode_small::{
    ClaimImpl as AddOpcodeSmallClaimImpl,
    InteractionClaimImpl as AddOpcodeSmallInteractionClaimImpl,
};
use components::add_opcode_small_imm::{
    ClaimImpl as AddOpcodeSmallImmClaimImpl,
    InteractionClaimImpl as AddOpcodeSmallImmInteractionClaimImpl,
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
use components::call_opcode::{
    ClaimImpl as CallOpcodeClaimImpl, InteractionClaimImpl as CallOpcodeInteractionClaimImpl,
};
use components::call_opcode_op_1_base_fp::{
    ClaimImpl as CallOpcodeOp1BaseFpClaimImpl,
    InteractionClaimImpl as CallOpcodeOp1BaseFpInteractionClaimImpl,
};
use components::call_opcode_rel::{
    ClaimImpl as CallOpcodeRelClaimImpl, InteractionClaimImpl as CallOpcodeRelInteractionClaimImpl,
};
use components::generic_opcode::{
    ClaimImpl as GenericOpcodeClaimImpl, InteractionClaimImpl as GenericOpcodeInteractionClaimImpl,
};
use components::jnz_opcode::{
    ClaimImpl as JnzOpcodeClaimImpl, InteractionClaimImpl as JnzOpcodeInteractionClaimImpl,
};
use components::jnz_opcode_dst_base_fp::{
    ClaimImpl as JnzOpcodeDstBaseFpClaimImpl,
    InteractionClaimImpl as JnzOpcodeDstBaseFpInteractionClaimImpl,
};
use components::jnz_opcode_taken::{
    ClaimImpl as JnzOpcodeTakenClaimImpl,
    InteractionClaimImpl as JnzOpcodeTakenInteractionClaimImpl,
};
use components::jnz_opcode_taken_dst_base_fp::{
    ClaimImpl as JnzOpcodeTakenDstBaseFpClaimImpl,
    InteractionClaimImpl as JnzOpcodeTakenDstBaseFpInteractionClaimImpl,
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
    InteractionClaimImpl as MemoryAddressToIdInteractionClaimImpl,
};
use components::memory_id_to_big::{
    ClaimImpl as MemoryIdToBigClaimImpl, InteractionClaimImpl as MemoryIdToBigInteractionClaimImpl,
};
use components::mul_opcode::{
    ClaimImpl as MulOpcodeClaimImpl, InteractionClaimImpl as MulOpcodeInteractionClaimImpl,
};
use components::mul_opcode_imm::{
    ClaimImpl as MulOpcodeImmClaimImpl, InteractionClaimImpl as MulOpcodeImmInteractionClaimImpl,
};
use components::mul_opcode_small::{
    ClaimImpl as MulOpcodeSmallClaimImpl,
    InteractionClaimImpl as MulOpcodeSmallInteractionClaimImpl,
};
use components::mul_opcode_small_imm::{
    ClaimImpl as MulOpcodeSmallImmClaimImpl,
    InteractionClaimImpl as MulOpcodeSmallImmInteractionClaimImpl,
};
use components::range_check_builtin_bits_128::{
    ClaimImpl as RangeCheckBuiltinBits128ClaimImpl,
    InteractionClaimImpl as RangeCheckBuiltinBits128InteractionClaimImpl,
};
use components::range_check_builtin_bits_96::{
    ClaimImpl as RangeCheckBuiltinBits96ClaimImpl,
    InteractionClaimImpl as RangeCheckBuiltinBits96InteractionClaimImpl,
};
use components::range_check_vector::{
    ClaimImpl as RangeCheckClaimImpl, InteractionClaimImpl as RangeCheckInteractionClaimImpl,
};
use components::ret_opcode::{
    ClaimImpl as RetOpcodeClaimImpl, InteractionClaimImpl as RetOpcodeInteractionClaimImpl,
};
use components::verify_bitwise_xor_9::{
    ClaimImpl as VerifyBitwiseXor9ClaimImpl,
    InteractionClaimImpl as VerifyBitwiseXor9InteractionClaimImpl,
};
use components::verify_instruction::{
    ClaimImpl as VerifyInstructionClaimImpl,
    InteractionClaimImpl as VerifyInstructionInteractionClaimImpl,
};
use core::num::traits::Zero;
use core::num::traits::one::One;
use stwo_constraint_framework::{
    LookupElements, LookupElementsImpl, PreprocessedColumn, PreprocessedColumnImpl,
    PreprocessedColumnKey, PreprocessedColumnSet, PreprocessedColumnTrait, PreprocessedMaskValues,
    PreprocessedMaskValuesImpl,
};
use stwo_verifier_core::channel::{Channel, ChannelImpl, ChannelTrait};
use stwo_verifier_core::circle::CirclePoint;
use stwo_verifier_core::fields::Invertible;
use stwo_verifier_core::fields::m31::{M31, m31};
use stwo_verifier_core::fields::qm31::{QM31, qm31_const};
use stwo_verifier_core::fri::FriConfig;
use stwo_verifier_core::pcs::verifier::CommitmentSchemeVerifierImpl;
use stwo_verifier_core::pcs::{PcsConfig, PcsConfigTrait};
use stwo_verifier_core::utils::{ArrayImpl, OptionImpl, pow2};
use stwo_verifier_core::verifier::{Air, StarkProof, VerificationError, verify};
use stwo_verifier_core::{ColumnArray, ColumnSpan, TreeArray, TreeSpan};

pub mod components;
pub mod utils;

const SECURITY_BITS: u32 = 96;

const PREPROCESSED_COLUMNS_LOG_SIZES: [u32; 19] = [
    22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4,
];

pub const ADD_MOD_MEMORY_CELLS: usize = 7;
pub const BITWISE_MEMORY_CELLS: usize = 5;
pub const EC_OP_MEMORY_CELLS: usize = 7;
pub const ECDSA_MEMORY_CELLS: usize = 2;
pub const KECCAK_MEMORY_CELLS: usize = 16;
pub const MUL_MOD_MEMORY_CELLS: usize = 7;
pub const PEDERSEN_MEMORY_CELLS: usize = 3;
pub const POSEIDON_MEMORY_CELLS: usize = 6;
pub const RANGE_CHECK_MEMORY_CELLS: usize = 1;

// NOTE: Order matters!
const PREPROCESSED_COLUMNS: [PreprocessedColumn; 170] = [
    PreprocessedColumn::Seq(24), //
    PreprocessedColumn::Seq(23), //
    PreprocessedColumn::PedersenPoints(0), //
    PreprocessedColumn::PedersenPoints(1), //
    PreprocessedColumn::PedersenPoints(2), //
    PreprocessedColumn::PedersenPoints(3), //
    PreprocessedColumn::PedersenPoints(4), //
    PreprocessedColumn::PedersenPoints(5), //
    PreprocessedColumn::PedersenPoints(6), //
    PreprocessedColumn::PedersenPoints(7), //
    PreprocessedColumn::PedersenPoints(8), //
    PreprocessedColumn::PedersenPoints(9), //
    PreprocessedColumn::PedersenPoints(10), //
    PreprocessedColumn::PedersenPoints(11), //
    PreprocessedColumn::PedersenPoints(12), //
    PreprocessedColumn::PedersenPoints(13), //
    PreprocessedColumn::PedersenPoints(14), //
    PreprocessedColumn::PedersenPoints(15), //
    PreprocessedColumn::PedersenPoints(16), //
    PreprocessedColumn::PedersenPoints(17), //
    PreprocessedColumn::PedersenPoints(18), //
    PreprocessedColumn::PedersenPoints(19), //
    PreprocessedColumn::PedersenPoints(20), //
    PreprocessedColumn::PedersenPoints(21), //
    PreprocessedColumn::PedersenPoints(22), //
    PreprocessedColumn::PedersenPoints(23), //
    PreprocessedColumn::PedersenPoints(24), //
    PreprocessedColumn::PedersenPoints(25), //
    PreprocessedColumn::PedersenPoints(26), //
    PreprocessedColumn::PedersenPoints(27), //
    PreprocessedColumn::PedersenPoints(28), //
    PreprocessedColumn::PedersenPoints(29), //
    PreprocessedColumn::PedersenPoints(30), //
    PreprocessedColumn::PedersenPoints(31), //
    PreprocessedColumn::PedersenPoints(32), //
    PreprocessedColumn::PedersenPoints(33), //
    PreprocessedColumn::PedersenPoints(34), //
    PreprocessedColumn::PedersenPoints(35), //
    PreprocessedColumn::PedersenPoints(36), //
    PreprocessedColumn::PedersenPoints(37), //
    PreprocessedColumn::PedersenPoints(38), //
    PreprocessedColumn::PedersenPoints(39), //
    PreprocessedColumn::PedersenPoints(40), //
    PreprocessedColumn::PedersenPoints(41), //
    PreprocessedColumn::PedersenPoints(42), //
    PreprocessedColumn::PedersenPoints(43), //
    PreprocessedColumn::PedersenPoints(44), //
    PreprocessedColumn::PedersenPoints(45), //
    PreprocessedColumn::PedersenPoints(46), //
    PreprocessedColumn::PedersenPoints(47), //
    PreprocessedColumn::PedersenPoints(48), //
    PreprocessedColumn::PedersenPoints(49), //
    PreprocessedColumn::PedersenPoints(50), //
    PreprocessedColumn::PedersenPoints(51), //
    PreprocessedColumn::PedersenPoints(52), //
    PreprocessedColumn::PedersenPoints(53), //
    PreprocessedColumn::PedersenPoints(54), //
    PreprocessedColumn::PedersenPoints(55), //
    PreprocessedColumn::Seq(22), //
    PreprocessedColumn::Seq(21), //
    PreprocessedColumn::Seq(20), //
    PreprocessedColumn::Xor((10, 0)), //
    PreprocessedColumn::Xor((10, 1)), //
    PreprocessedColumn::Xor((10, 2)), //
    PreprocessedColumn::Seq(19), //
    PreprocessedColumn::RangeCheck(([19, 0, 0, 0, 0], 0)), //
    PreprocessedColumn::Seq(18), //
    PreprocessedColumn::Xor((9, 0)), //
    PreprocessedColumn::Xor((9, 1)), //
    PreprocessedColumn::Xor((9, 2)), //
    PreprocessedColumn::RangeCheck(([18, 0, 0, 0, 0], 0)), //
    PreprocessedColumn::RangeCheck(([9, 9, 0, 0, 0], 0)), //
    PreprocessedColumn::RangeCheck(([9, 9, 0, 0, 0], 1)), //
    PreprocessedColumn::RangeCheck(([3, 6, 6, 3, 0], 0)), //
    PreprocessedColumn::RangeCheck(([3, 6, 6, 3, 0], 1)), //
    PreprocessedColumn::RangeCheck(([3, 6, 6, 3, 0], 2)), //
    PreprocessedColumn::RangeCheck(([3, 6, 6, 3, 0], 3)), //
    PreprocessedColumn::Seq(17), //
    PreprocessedColumn::Seq(16), //
    PreprocessedColumn::Xor((8, 0)), //
    PreprocessedColumn::Xor((8, 1)), //
    PreprocessedColumn::Xor((8, 2)), //
    PreprocessedColumn::RangeCheck(([4, 4, 4, 4, 0], 0)), //
    PreprocessedColumn::RangeCheck(([4, 4, 4, 4, 0], 1)), //
    PreprocessedColumn::RangeCheck(([4, 4, 4, 4, 0], 2)), //
    PreprocessedColumn::RangeCheck(([4, 4, 4, 4, 0], 3)), //
    PreprocessedColumn::Seq(15), //
    PreprocessedColumn::RangeCheck(([3, 3, 3, 3, 3], 0)), //
    PreprocessedColumn::RangeCheck(([3, 3, 3, 3, 3], 1)), //
    PreprocessedColumn::RangeCheck(([3, 3, 3, 3, 3], 2)), //
    PreprocessedColumn::RangeCheck(([3, 3, 3, 3, 3], 3)), //
    PreprocessedColumn::RangeCheck(([3, 3, 3, 3, 3], 4)), //
    PreprocessedColumn::Seq(14), //
    PreprocessedColumn::Xor((7, 0)), //
    PreprocessedColumn::Xor((7, 1)), //
    PreprocessedColumn::Xor((7, 2)), //
    PreprocessedColumn::RangeCheck(([7, 2, 5, 0, 0], 0)), //
    PreprocessedColumn::RangeCheck(([7, 2, 5, 0, 0], 1)), //
    PreprocessedColumn::RangeCheck(([7, 2, 5, 0, 0], 2)), //
    PreprocessedColumn::Seq(13), //
    PreprocessedColumn::Seq(12), //
    PreprocessedColumn::RangeCheck(([12, 0, 0, 0, 0], 0)), //
    PreprocessedColumn::Seq(11), //
    PreprocessedColumn::RangeCheck(([11, 0, 0, 0, 0], 0)), //
    PreprocessedColumn::Seq(10), //
    PreprocessedColumn::Seq(9), //
    PreprocessedColumn::RangeCheck(([3, 6, 0, 0, 0], 0)), //
    PreprocessedColumn::RangeCheck(([3, 6, 0, 0, 0], 1)), //
    PreprocessedColumn::RangeCheck(([5, 4, 0, 0, 0], 0)), //
    PreprocessedColumn::RangeCheck(([5, 4, 0, 0, 0], 1)), //
    PreprocessedColumn::Seq(8), //
    PreprocessedColumn::Xor((4, 0)), //
    PreprocessedColumn::Xor((4, 1)), //
    PreprocessedColumn::Xor((4, 2)), //
    PreprocessedColumn::RangeCheck(([8, 0, 0, 0, 0], 0)), //
    PreprocessedColumn::RangeCheck(([4, 4, 0, 0, 0], 0)), //
    PreprocessedColumn::RangeCheck(([4, 4, 0, 0, 0], 1)), //
    PreprocessedColumn::Seq(7), //
    PreprocessedColumn::RangeCheck(([4, 3, 0, 0, 0], 0)), //
    PreprocessedColumn::RangeCheck(([4, 3, 0, 0, 0], 1)), //
    PreprocessedColumn::Seq(6), //
    PreprocessedColumn::RangeCheck(([6, 0, 0, 0, 0], 0)), //
    PreprocessedColumn::PoseidonRoundKeys(0), //
    PreprocessedColumn::PoseidonRoundKeys(1), //
    PreprocessedColumn::PoseidonRoundKeys(2), //
    PreprocessedColumn::PoseidonRoundKeys(3), //
    PreprocessedColumn::PoseidonRoundKeys(4), //
    PreprocessedColumn::PoseidonRoundKeys(5), //
    PreprocessedColumn::PoseidonRoundKeys(6), //
    PreprocessedColumn::PoseidonRoundKeys(7), //
    PreprocessedColumn::PoseidonRoundKeys(8), //
    PreprocessedColumn::PoseidonRoundKeys(9), //
    PreprocessedColumn::PoseidonRoundKeys(10), //
    PreprocessedColumn::PoseidonRoundKeys(11), //
    PreprocessedColumn::PoseidonRoundKeys(12), //
    PreprocessedColumn::PoseidonRoundKeys(13), //
    PreprocessedColumn::PoseidonRoundKeys(14), //
    PreprocessedColumn::PoseidonRoundKeys(15), //
    PreprocessedColumn::PoseidonRoundKeys(16), //
    PreprocessedColumn::PoseidonRoundKeys(17), //
    PreprocessedColumn::PoseidonRoundKeys(18), //
    PreprocessedColumn::PoseidonRoundKeys(19), //
    PreprocessedColumn::PoseidonRoundKeys(20), //
    PreprocessedColumn::PoseidonRoundKeys(21), //
    PreprocessedColumn::PoseidonRoundKeys(22), //
    PreprocessedColumn::PoseidonRoundKeys(23), //
    PreprocessedColumn::PoseidonRoundKeys(24), //
    PreprocessedColumn::PoseidonRoundKeys(25), //
    PreprocessedColumn::PoseidonRoundKeys(26), //
    PreprocessedColumn::PoseidonRoundKeys(27), //
    PreprocessedColumn::PoseidonRoundKeys(28), //
    PreprocessedColumn::PoseidonRoundKeys(29), //
    PreprocessedColumn::Seq(5), //
    PreprocessedColumn::Seq(4), //
    PreprocessedColumn::BlakeSigma(0), //
    PreprocessedColumn::BlakeSigma(1), //
    PreprocessedColumn::BlakeSigma(2), //
    PreprocessedColumn::BlakeSigma(3), //
    PreprocessedColumn::BlakeSigma(4), //
    PreprocessedColumn::BlakeSigma(5), //
    PreprocessedColumn::BlakeSigma(6), //
    PreprocessedColumn::BlakeSigma(7), //
    PreprocessedColumn::BlakeSigma(8), //
    PreprocessedColumn::BlakeSigma(9), //
    PreprocessedColumn::BlakeSigma(10), //
    PreprocessedColumn::BlakeSigma(11), //
    PreprocessedColumn::BlakeSigma(12), //
    PreprocessedColumn::BlakeSigma(13), //
    PreprocessedColumn::BlakeSigma(14), //
    PreprocessedColumn::BlakeSigma(15) //
];

type MemoryAddressToIdElements = LookupElements<2>;

type MemoryIdToBigElements = LookupElements<29>;

type OpcodeElements = LookupElements<3>;

type RangeCheck3BitElements = LookupElements<1>;

type RangeCheck6BitElements = LookupElements<1>;

type RangeCheck3Bit6BitElements = LookupElements<2>;

type RangeCheck11BitElements = LookupElements<1>;

type RangeCheck12BitElements = LookupElements<1>;

type RangeCheck18BitElements = LookupElements<1>;

type RangeCheck19BitElements = LookupElements<1>;

type RangeCheck9Bit9BitElements = LookupElements<2>;

type RangeCheck4Bit3BitElements = LookupElements<2>;

type RangeCheck7Bit2Bit5BitElements = LookupElements<3>;

type RangeCheck3Bit6Bit6Bit3BitElements = LookupElements<4>;

type VerifyInstructionElements = LookupElements<29>;

type VerifyBitwiseXor9BitElements = LookupElements<3>;


#[derive(Drop, Serde)]
pub struct CairoProof {
    pub claim: CairoClaim,
    pub interaction_claim: CairoInteractionClaim,
    pub stark_proof: StarkProof,
}

pub fn verify_cairo(proof: CairoProof) -> Result<(), CairoVerificationError> {
    let CairoProof { claim, interaction_claim, stark_proof } = proof;

    // Verify.
    let pcs_config = stark_proof.commitment_scheme_proof.config;

    verify_claim(@claim);

    let mut channel: Channel = Default::default();
    pcs_config.mix_into(ref channel);
    let mut commitment_scheme = CommitmentSchemeVerifierImpl::new(pcs_config);

    let log_sizes = claim.log_sizes();

    // Preproccessed trace.
    commitment_scheme
        .commit(
            stark_proof.commitment_scheme_proof.commitments[0].clone(), *log_sizes[0], ref channel,
        );
    claim.mix_into(ref channel);

    commitment_scheme
        .commit(
            stark_proof.commitment_scheme_proof.commitments[1].clone(), *log_sizes[1], ref channel,
        );
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
    sum += interaction_claim.builtins.sum();
    sum += *interaction_claim.memory_address_to_id.claimed_sum;
    sum += *interaction_claim.memory_id_to_value.big_claimed_sum;
    sum += *interaction_claim.memory_id_to_value.small_claimed_sum;
    sum += interaction_claim.range_checks.sum();
    sum += *interaction_claim.verify_bitwise_xor_9.claimed_sum;

    sum
}

#[derive(Drop)]
struct RangeChecksInteractionElements {
    pub rc_6: RangeCheck6BitElements,
    pub rc_11: RangeCheck11BitElements,
    pub rc_12: RangeCheck12BitElements,
    pub rc_18: RangeCheck18BitElements,
    pub rc_19: RangeCheck19BitElements,
    pub rc_3_6: RangeCheck3Bit6BitElements,
    pub rc_4_3: RangeCheck4Bit3BitElements,
    pub rc_9_9: RangeCheck9Bit9BitElements,
    pub rc_7_2_5: RangeCheck7Bit2Bit5BitElements,
    pub rc_3_6_6_3: RangeCheck3Bit6Bit6Bit3BitElements,
}

#[generate_trait]
impl RangeChecksInteractionElementsImpl of RangeChecksInteractionElementsTrait {
    fn draw(ref channel: Channel) -> RangeChecksInteractionElements {
        RangeChecksInteractionElements {
            rc_6: LookupElementsImpl::draw(ref channel),
            rc_11: LookupElementsImpl::draw(ref channel),
            rc_12: LookupElementsImpl::draw(ref channel),
            rc_18: LookupElementsImpl::draw(ref channel),
            rc_19: LookupElementsImpl::draw(ref channel),
            rc_3_6: LookupElementsImpl::draw(ref channel),
            rc_4_3: LookupElementsImpl::draw(ref channel),
            rc_9_9: LookupElementsImpl::draw(ref channel),
            rc_7_2_5: LookupElementsImpl::draw(ref channel),
            rc_3_6_6_3: LookupElementsImpl::draw(ref channel),
        }
    }
}

#[derive(Drop)]
struct CairoInteractionElements {
    pub opcodes: OpcodeElements,
    pub verify_instruction: VerifyInstructionElements,
    pub memory_address_to_id: MemoryAddressToIdElements,
    pub memory_id_to_value: MemoryIdToBigElements,
    pub range_checks: RangeChecksInteractionElements,
    pub verify_bitwise_xor_9: VerifyBitwiseXor9BitElements,
}

#[generate_trait]
impl CairoInteractionElementsImpl of CairoInteractionElementsTrait {
    fn draw(ref channel: Channel) -> CairoInteractionElements {
        CairoInteractionElements {
            opcodes: LookupElementsImpl::draw(ref channel),
            verify_instruction: LookupElementsImpl::draw(ref channel),
            memory_address_to_id: LookupElementsImpl::draw(ref channel),
            memory_id_to_value: LookupElementsImpl::draw(ref channel),
            range_checks: RangeChecksInteractionElementsImpl::draw(ref channel),
            verify_bitwise_xor_9: LookupElementsImpl::draw(ref channel),
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
    pub bitwise_builtin: Option<components::bitwise_builtin::Claim>,
    pub range_check_96_builtin: Option<components::range_check_builtin_bits_96::Claim>,
    pub range_check_128_builtin: Option<components::range_check_builtin_bits_128::Claim>,
}

#[generate_trait]
impl BuiltinsClaimImpl of BuiltinsClaimTrait {
    fn mix_into(self: @BuiltinsClaim, ref channel: Channel) {
        if let Some(claim) = self.bitwise_builtin {
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

        if let Some(claim) = self.bitwise_builtin {
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
}

#[derive(Drop, Serde, Copy)]
pub struct BuiltinsInteractionClaim {
    pub bitwise_builtin: Option<components::bitwise_builtin::InteractionClaim>,
    pub range_check_96_builtin: Option<components::range_check_builtin_bits_96::InteractionClaim>,
    pub range_check_128_builtin: Option<components::range_check_builtin_bits_128::InteractionClaim>,
}

#[generate_trait]
impl BuiltinsInteractionClaimImpl of BuiltinsInteractionClaimTrait {
    fn mix_into(self: @BuiltinsInteractionClaim, ref channel: Channel) {
        if let Some(claim) = self.bitwise_builtin {
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

        if let Some(claim) = self.bitwise_builtin {
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
    pub rc_6: components::range_check_vector::Claim,
    pub rc_11: components::range_check_vector::Claim,
    pub rc_12: components::range_check_vector::Claim,
    pub rc_18: components::range_check_vector::Claim,
    pub rc_19: components::range_check_vector::Claim,
    pub rc_3_6: components::range_check_vector::Claim,
    pub rc_4_3: components::range_check_vector::Claim,
    pub rc_9_9: components::range_check_vector::Claim,
    pub rc_7_2_5: components::range_check_vector::Claim,
    pub rc_3_6_6_3: components::range_check_vector::Claim,
}

#[generate_trait]
impl RangeChecksClaimImpl of RangeChecksClaimTrait {
    fn mix_into(self: @RangeChecksClaim, ref channel: Channel) {
        self.rc_6.mix_into(ref channel);
        self.rc_11.mix_into(ref channel);
        self.rc_12.mix_into(ref channel);
        self.rc_18.mix_into(ref channel);
        self.rc_19.mix_into(ref channel);
        self.rc_3_6.mix_into(ref channel);
        self.rc_4_3.mix_into(ref channel);
        self.rc_9_9.mix_into(ref channel);
        self.rc_7_2_5.mix_into(ref channel);
        self.rc_3_6_6_3.mix_into(ref channel);
    }

    fn log_sizes(self: @RangeChecksClaim) -> TreeArray<Span<u32>> {
        utils::tree_array_concat_cols(
            array![
                self.rc_6.log_sizes(), self.rc_11.log_sizes(), self.rc_12.log_sizes(),
                self.rc_18.log_sizes(), self.rc_19.log_sizes(), self.rc_3_6.log_sizes(),
                self.rc_4_3.log_sizes(), self.rc_9_9.log_sizes(), self.rc_7_2_5.log_sizes(),
                self.rc_3_6_6_3.log_sizes(),
            ],
        )
    }
}


#[derive(Drop, Serde, Clone)]
pub struct RangeChecksInteractionClaim {
    pub rc_6: components::range_check_vector::InteractionClaim,
    pub rc_11: components::range_check_vector::InteractionClaim,
    pub rc_12: components::range_check_vector::InteractionClaim,
    pub rc_18: components::range_check_vector::InteractionClaim,
    pub rc_19: components::range_check_vector::InteractionClaim,
    pub rc_3_6: components::range_check_vector::InteractionClaim,
    pub rc_4_3: components::range_check_vector::InteractionClaim,
    pub rc_9_9: components::range_check_vector::InteractionClaim,
    pub rc_7_2_5: components::range_check_vector::InteractionClaim,
    pub rc_3_6_6_3: components::range_check_vector::InteractionClaim,
}

#[generate_trait]
impl RangeChecksInteractionClaimImpl of RangeChecksInteractionClaimTrait {
    fn mix_into(self: @RangeChecksInteractionClaim, ref channel: Channel) {
        self.rc_6.mix_into(ref channel);
        self.rc_11.mix_into(ref channel);
        self.rc_12.mix_into(ref channel);
        self.rc_18.mix_into(ref channel);
        self.rc_19.mix_into(ref channel);
        self.rc_3_6.mix_into(ref channel);
        self.rc_4_3.mix_into(ref channel);
        self.rc_9_9.mix_into(ref channel);
        self.rc_7_2_5.mix_into(ref channel);
        self.rc_3_6_6_3.mix_into(ref channel);
    }

    fn sum(self: @RangeChecksInteractionClaim) -> QM31 {
        let mut sum = Zero::zero();
        sum += *self.rc_6.claimed_sum;
        sum += *self.rc_11.claimed_sum;
        sum += *self.rc_12.claimed_sum;
        sum += *self.rc_18.claimed_sum;
        sum += *self.rc_19.claimed_sum;
        sum += *self.rc_3_6.claimed_sum;
        sum += *self.rc_4_3.claimed_sum;
        sum += *self.rc_9_9.claimed_sum;
        sum += *self.rc_7_2_5.claimed_sum;
        sum += *self.rc_3_6_6_3.claimed_sum;
        sum
    }
}

#[derive(Drop, Serde)]
pub struct CairoClaim {
    pub public_data: PublicData,
    pub opcodes: OpcodeClaim,
    pub verify_instruction: components::verify_instruction::Claim,
    pub builtins: BuiltinsClaim,
    pub memory_address_to_id: components::memory_address_to_id::Claim,
    pub memory_id_to_value: components::memory_id_to_big::Claim,
    pub range_checks: RangeChecksClaim,
    pub verify_bitwise_xor_9: components::verify_bitwise_xor_9::Claim,
    // ...
}

#[generate_trait]
impl CairoClaimImpl of CairoClaimTrait {
    fn log_sizes(self: @CairoClaim) -> TreeArray<Span<u32>> {
        let range_check_log_sizes = self.range_checks.log_sizes();

        let mut aggregated_log_sizes = utils::tree_array_concat_cols(
            array![
                self.opcodes.log_sizes(), self.verify_instruction.log_sizes(),
                self.builtins.log_sizes(), self.memory_address_to_id.log_sizes(),
                self.memory_id_to_value.log_sizes(), self.range_checks.log_sizes(),
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
        self.builtins.mix_into(ref channel);
        self.memory_address_to_id.mix_into(ref channel);
        self.memory_id_to_value.mix_into(ref channel);
        self.range_checks.mix_into(ref channel);
        self.verify_bitwise_xor_9.mix_into(ref channel);
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

    verify_program(program);

    assert!((*initial_pc).is_one());
    assert!(*initial_pc + m31(2) < *initial_ap);
    assert!(initial_fp == final_fp);
    assert!(initial_fp == initial_ap);
    assert!(*final_pc == m31(5));
    assert!(initial_ap <= final_ap);
}

fn verify_builtins(builtins_claim: @BuiltinsClaim, segment_ranges: @PublicSegmentRanges) {
    // Check that non-supported builtins aren't used.
    assert!(segment_ranges.ec_op.start_ptr.value == segment_ranges.ec_op.stop_ptr.value);
    assert!(segment_ranges.ecdsa.start_ptr.value == segment_ranges.ecdsa.stop_ptr.value);
    assert!(segment_ranges.keccak.start_ptr.value == segment_ranges.keccak.stop_ptr.value);

    // Output builtin.
    assert!(segment_ranges.output.stop_ptr.value <= @pow2(31));
    assert!(segment_ranges.output.start_ptr.value <= segment_ranges.output.stop_ptr.value);

    // All other supported builtins.
    check_builtin(
        builtins_claim
            .range_check_128_builtin
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
        builtins_claim
            .range_check_96_builtin
            .map(
                |
                    claim,
                | BuiltinClaim {
                    segment_start: claim.range_check_builtin_segment_start,
                    log_size: claim.log_size,
                },
            ),
        *segment_ranges.range_check_96,
        RANGE_CHECK_MEMORY_CELLS,
    );
    check_builtin(
        builtins_claim
            .bitwise_builtin
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
    // TODO(alonf): Soundness - check all the builtins after they are merged.
}

fn check_builtin(builtin_claim: Option<BuiltinClaim>, segment_range: SegmentRange, n_cells: usize) {
    if segment_range.is_empty() {
        return;
    }

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

fn verify_program(program: @MemorySection) {
    let (_, program_value_0) = program[0];
    let (_, program_value_1) = program[1];
    let (_, program_value_2) = program[2];
    let (_, program_value_4) = program[4];
    let (_, program_value_5) = program[5];
    assert!(program_value_0 == @[0x7fff7fff, 0x4078001, 0, 0, 0, 0, 0, 0]); // ap += N_BUILTINS.
    assert!(program_value_1 == @[11, 0, 0, 0, 0, 0, 0, 0]); // Imm of last instruction (N_BUILTINS).
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
    pub builtins: BuiltinsInteractionClaim,
    pub memory_address_to_id: components::memory_address_to_id::InteractionClaim,
    pub memory_id_to_value: components::memory_id_to_big::InteractionClaim,
    pub range_checks: RangeChecksInteractionClaim,
    pub verify_bitwise_xor_9: components::verify_bitwise_xor_9::InteractionClaim,
}

#[generate_trait]
impl CairoInteractionClaimImpl of CairoInteractionClaimTrace {
    fn mix_into(self: @CairoInteractionClaim, ref channel: Channel) {
        self.opcodes.mix_into(ref channel);
        self.verify_instruction.mix_into(ref channel);
        self.builtins.mix_into(ref channel);
        self.memory_address_to_id.mix_into(ref channel);
        self.memory_id_to_value.mix_into(ref channel);
        self.range_checks.mix_into(ref channel);
        self.verify_bitwise_xor_9.mix_into(ref channel);
    }
}

#[derive(Drop, Serde)]
pub struct OpcodeInteractionClaim {
    add: Array<components::add_opcode::InteractionClaim>,
    add_imm: Array<components::add_opcode_imm::InteractionClaim>,
    add_small: Array<components::add_opcode_small::InteractionClaim>,
    add_small_imm: Array<components::add_opcode_small_imm::InteractionClaim>,
    add_ap: Array<components::add_ap_opcode::InteractionClaim>,
    add_ap_op_1_base_fp: Array<components::add_ap_opcode_op_1_base_fp::InteractionClaim>,
    add_ap_imm: Array<components::add_ap_opcode_imm::InteractionClaim>,
    assert_eq: Array<components::assert_eq_opcode::InteractionClaim>,
    assert_eq_imm: Array<components::assert_eq_opcode_imm::InteractionClaim>,
    assert_eq_double_deref: Array<components::assert_eq_opcode_double_deref::InteractionClaim>,
    call: Array<components::call_opcode::InteractionClaim>,
    call_op_1_base_fp: Array<components::call_opcode_op_1_base_fp::InteractionClaim>,
    call_rel: Array<components::call_opcode_rel::InteractionClaim>,
    generic: Array<components::generic_opcode::InteractionClaim>,
    jnz: Array<components::jnz_opcode::InteractionClaim>,
    jnz_dst_base_fp: Array<components::jnz_opcode_dst_base_fp::InteractionClaim>,
    jnz_taken: Array<components::jnz_opcode_taken::InteractionClaim>,
    jnz_taken_dst_base_fp: Array<components::jnz_opcode_taken_dst_base_fp::InteractionClaim>,
    jump: Array<components::jump_opcode::InteractionClaim>,
    jump_double_deref: Array<components::jump_opcode_double_deref::InteractionClaim>,
    jump_rel: Array<components::jump_opcode_rel::InteractionClaim>,
    jump_rel_imm: Array<components::jump_opcode_rel_imm::InteractionClaim>,
    mul: Array<components::mul_opcode::InteractionClaim>,
    mul_imm: Array<components::mul_opcode_imm::InteractionClaim>,
    mul_small: Array<components::mul_opcode_small::InteractionClaim>,
    mul_small_imm: Array<components::mul_opcode_small_imm::InteractionClaim>,
    ret: Array<components::ret_opcode::InteractionClaim>,
}

#[generate_trait]
impl OpcodeInteractionClaimImpl of OpcodeInteractionClaimTrait {
    fn mix_into(self: @OpcodeInteractionClaim, ref channel: Channel) {
        for interaction_claim in self.add.span() {
            interaction_claim.mix_into(ref channel);
        }

        for interaction_claim in self.add_imm.span() {
            interaction_claim.mix_into(ref channel);
        }

        for interaction_claim in self.add_small.span() {
            interaction_claim.mix_into(ref channel);
        }

        for interaction_claim in self.add_small_imm.span() {
            interaction_claim.mix_into(ref channel);
        }

        for interaction_claim in self.add_ap.span() {
            interaction_claim.mix_into(ref channel);
        }

        for interaction_claim in self.add_ap_op_1_base_fp.span() {
            interaction_claim.mix_into(ref channel);
        }

        for interaction_claim in self.add_ap_imm.span() {
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

        for interaction_claim in self.call.span() {
            interaction_claim.mix_into(ref channel);
        }

        for interaction_claim in self.call_op_1_base_fp.span() {
            interaction_claim.mix_into(ref channel);
        }

        for interaction_claim in self.call_rel.span() {
            interaction_claim.mix_into(ref channel);
        }

        for interaction_claim in self.generic.span() {
            interaction_claim.mix_into(ref channel);
        }

        for interaction_claim in self.jnz.span() {
            interaction_claim.mix_into(ref channel);
        }

        for interaction_claim in self.jnz_dst_base_fp.span() {
            interaction_claim.mix_into(ref channel);
        }

        for interaction_claim in self.jnz_taken.span() {
            interaction_claim.mix_into(ref channel);
        }

        for interaction_claim in self.jnz_taken_dst_base_fp.span() {
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

        for interaction_claim in self.mul_imm.span() {
            interaction_claim.mix_into(ref channel);
        }

        for interaction_claim in self.mul_small.span() {
            interaction_claim.mix_into(ref channel);
        }

        for interaction_claim in self.mul_small_imm.span() {
            interaction_claim.mix_into(ref channel);
        }

        for interaction_claim in self.ret.span() {
            interaction_claim.mix_into(ref channel);
        };
    }

    fn sum(self: @OpcodeInteractionClaim) -> QM31 {
        let mut sum = Zero::zero();

        for interaction_claim in self.add.span() {
            sum += *interaction_claim.claimed_sum;
        }

        for interaction_claim in self.add_imm.span() {
            sum += *interaction_claim.claimed_sum;
        }

        for interaction_claim in self.add_small.span() {
            sum += *interaction_claim.claimed_sum;
        }

        for interaction_claim in self.add_small_imm.span() {
            sum += *interaction_claim.claimed_sum;
        }

        for interaction_claim in self.add_ap.span() {
            sum += *interaction_claim.claimed_sum;
        }

        for interaction_claim in self.add_ap_op_1_base_fp.span() {
            sum += *interaction_claim.claimed_sum;
        }

        for interaction_claim in self.add_ap_imm.span() {
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

        for interaction_claim in self.call.span() {
            sum += *interaction_claim.claimed_sum;
        }

        for interaction_claim in self.call_op_1_base_fp.span() {
            sum += *interaction_claim.claimed_sum;
        }

        for interaction_claim in self.call_rel.span() {
            sum += *interaction_claim.claimed_sum;
        }

        for interaction_claim in self.generic.span() {
            sum += *interaction_claim.claimed_sum;
        }

        for interaction_claim in self.jnz.span() {
            sum += *interaction_claim.claimed_sum;
        }

        for interaction_claim in self.jnz_dst_base_fp.span() {
            sum += *interaction_claim.claimed_sum;
        }

        for interaction_claim in self.jnz_taken.span() {
            sum += *interaction_claim.claimed_sum;
        }

        for interaction_claim in self.jnz_taken_dst_base_fp.span() {
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

        for interaction_claim in self.mul_imm.span() {
            sum += *interaction_claim.claimed_sum;
        }

        for interaction_claim in self.mul_small.span() {
            sum += *interaction_claim.claimed_sum;
        }

        for interaction_claim in self.mul_small_imm.span() {
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
        } = *self;
        output.mix_into(ref channel);
        pedersen.mix_into(ref channel);
        range_check_128.mix_into(ref channel);
        ecdsa.mix_into(ref channel);
        bitwise.mix_into(ref channel);
        ec_op.mix_into(ref channel);
        keccak.mix_into(ref channel);
        poseidon.mix_into(ref channel);
        range_check_96.mix_into(ref channel);
        add_mod.mix_into(ref channel);
        mul_mod.mix_into(ref channel);
    }
}

/// A contiguous memory section.
pub type MemorySection = Array<PubMemoryValue>;

// TODO(alonf): Perform all public data validations.
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

        // Segment ranges.
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
        } = self.public_segments;

        i = 0;
        for segment in [
            output, pedersen, range_check_128, ecdsa, bitwise, ec_op, keccak, poseidon,
            range_check_96, add_mod, mul_mod,
        ]
            .span() {
            entries
                .append(
                    (
                        initial_ap + i,
                        **segment.start_ptr.id,
                        [**segment.start_ptr.value, 0, 0, 0, 0, 0, 0, 0],
                    ),
                );
            entries
                .append(
                    (
                        final_ap - 11 + i,
                        **segment.stop_ptr.id,
                        [**segment.stop_ptr.value, 0, 0, 0, 0, 0, 0, 0],
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
        channel.mix_u64(output.len().into());
        for (id, value) in output.span() {
            channel.mix_u64((*id).into());
            // Mix each element of the array individually
            for val_element in (*value).span() {
                channel.mix_u64((*val_element).into());
            }
        }

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
    pub add_imm: Array<components::add_opcode_imm::Claim>,
    pub add_small: Array<components::add_opcode_small::Claim>,
    pub add_small_imm: Array<components::add_opcode_small_imm::Claim>,
    pub add_ap: Array<components::add_ap_opcode::Claim>,
    pub add_ap_op_1_base_fp: Array<components::add_ap_opcode_op_1_base_fp::Claim>,
    pub add_ap_imm: Array<components::add_ap_opcode_imm::Claim>,
    pub assert_eq: Array<components::assert_eq_opcode::Claim>,
    pub assert_eq_imm: Array<components::assert_eq_opcode_imm::Claim>,
    pub assert_eq_double_deref: Array<components::assert_eq_opcode_double_deref::Claim>,
    pub call: Array<components::call_opcode::Claim>,
    pub call_op_1_base_fp: Array<components::call_opcode_op_1_base_fp::Claim>,
    pub call_rel: Array<components::call_opcode_rel::Claim>,
    pub generic: Array<components::generic_opcode::Claim>,
    pub jnz: Array<components::jnz_opcode::Claim>,
    pub jnz_dst_base_fp: Array<components::jnz_opcode_dst_base_fp::Claim>,
    pub jnz_taken: Array<components::jnz_opcode_taken::Claim>,
    pub jnz_taken_dst_base_fp: Array<components::jnz_opcode_taken_dst_base_fp::Claim>,
    pub jump: Array<components::jump_opcode::Claim>,
    pub jump_double_deref: Array<components::jump_opcode_double_deref::Claim>,
    pub jump_rel: Array<components::jump_opcode_rel::Claim>,
    pub jump_rel_imm: Array<components::jump_opcode_rel_imm::Claim>,
    pub mul: Array<components::mul_opcode::Claim>,
    pub mul_imm: Array<components::mul_opcode_imm::Claim>,
    pub mul_small: Array<components::mul_opcode_small::Claim>,
    pub mul_small_imm: Array<components::mul_opcode_small_imm::Claim>,
    pub ret: Array<components::ret_opcode::Claim>,
}

#[generate_trait]
impl OpcodeClaimImpl of OpcodeClaimTrait {
    fn mix_into(self: @OpcodeClaim, ref channel: Channel) {
        for claim in self.add.span() {
            claim.mix_into(ref channel);
        }

        for claim in self.add_imm.span() {
            claim.mix_into(ref channel);
        }

        for claim in self.add_small.span() {
            claim.mix_into(ref channel);
        }

        for claim in self.add_small_imm.span() {
            claim.mix_into(ref channel);
        }

        for claim in self.add_ap.span() {
            claim.mix_into(ref channel);
        }

        for claim in self.add_ap_op_1_base_fp.span() {
            claim.mix_into(ref channel);
        }

        for claim in self.add_ap_imm.span() {
            claim.mix_into(ref channel);
        }

        for claim in self.assert_eq.span() {
            claim.mix_into(ref channel);
        }

        for claim in self.assert_eq_imm.span() {
            claim.mix_into(ref channel);
        }

        for claim in self.assert_eq_double_deref.span() {
            claim.mix_into(ref channel);
        }

        for claim in self.call.span() {
            claim.mix_into(ref channel);
        }

        for claim in self.call_op_1_base_fp.span() {
            claim.mix_into(ref channel);
        }

        for claim in self.call_rel.span() {
            claim.mix_into(ref channel);
        }

        for claim in self.generic.span() {
            claim.mix_into(ref channel);
        }

        for claim in self.jnz.span() {
            claim.mix_into(ref channel);
        }

        for claim in self.jnz_dst_base_fp.span() {
            claim.mix_into(ref channel);
        }

        for claim in self.jnz_taken.span() {
            claim.mix_into(ref channel);
        }

        for claim in self.jnz_taken_dst_base_fp.span() {
            claim.mix_into(ref channel);
        }

        for claim in self.jump.span() {
            claim.mix_into(ref channel);
        }

        for claim in self.jump_double_deref.span() {
            claim.mix_into(ref channel);
        }

        for claim in self.jump_rel.span() {
            claim.mix_into(ref channel);
        }

        for claim in self.jump_rel_imm.span() {
            claim.mix_into(ref channel);
        }

        for claim in self.mul.span() {
            claim.mix_into(ref channel);
        }

        for claim in self.mul_imm.span() {
            claim.mix_into(ref channel);
        }

        for claim in self.mul_small.span() {
            claim.mix_into(ref channel);
        }

        for claim in self.mul_small_imm.span() {
            claim.mix_into(ref channel);
        }

        for claim in self.ret.span() {
            claim.mix_into(ref channel);
        }
    }

    fn log_sizes(self: @OpcodeClaim) -> TreeArray<Span<u32>> {
        let mut log_sizes = array![];

        for claim in self.add.span() {
            log_sizes.append(claim.log_sizes());
        }

        for claim in self.add_imm.span() {
            log_sizes.append(claim.log_sizes());
        }

        for claim in self.add_small.span() {
            log_sizes.append(claim.log_sizes());
        }

        for claim in self.add_small_imm.span() {
            log_sizes.append(claim.log_sizes());
        }

        for claim in self.add_ap.span() {
            log_sizes.append(claim.log_sizes());
        }

        for claim in self.add_ap_op_1_base_fp.span() {
            log_sizes.append(claim.log_sizes());
        }

        for claim in self.add_ap_imm.span() {
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

        for claim in self.call.span() {
            log_sizes.append(claim.log_sizes());
        }

        for claim in self.call_op_1_base_fp.span() {
            log_sizes.append(claim.log_sizes());
        }

        for claim in self.call_rel.span() {
            log_sizes.append(claim.log_sizes());
        }

        for claim in self.generic.span() {
            log_sizes.append(claim.log_sizes());
        }

        for claim in self.jnz.span() {
            log_sizes.append(claim.log_sizes());
        }

        for claim in self.jnz_dst_base_fp.span() {
            log_sizes.append(claim.log_sizes());
        }

        for claim in self.jnz_taken.span() {
            log_sizes.append(claim.log_sizes());
        }

        for claim in self.jnz_taken_dst_base_fp.span() {
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

        for claim in self.mul_imm.span() {
            log_sizes.append(claim.log_sizes());
        }

        for claim in self.mul_small.span() {
            log_sizes.append(claim.log_sizes());
        }

        for claim in self.mul_small_imm.span() {
            log_sizes.append(claim.log_sizes());
        }

        for claim in self.ret.span() {
            log_sizes.append(claim.log_sizes());
        }

        utils::tree_array_concat_cols(log_sizes)
    }
}

#[derive(Drop, Debug)]
pub enum CairoVerificationError {
    InvalidLogupSum,
    InvalidClaim,
    Stark: VerificationError,
}

#[derive(Drop)]
pub struct CairoAir {
    opcodes: OpcodeComponents,
    verify_instruction: components::verify_instruction::Component,
    builtins: BuiltinComponents,
    memory_address_to_id: components::memory_address_to_id::Component,
    memory_id_to_value: (
        components::memory_id_to_big::BigComponent, components::memory_id_to_big::SmallComponent,
    ),
    range_checks: RangeChecksComponents,
    verify_bitwise_xor_9: components::verify_bitwise_xor_9::Component,
}

#[generate_trait]
impl CairoAirNewImpl of CairoAirNewTrait {
    fn new(
        cairo_claim: @CairoClaim,
        interaction_elements: @CairoInteractionElements,
        interaction_claim: @CairoInteractionClaim,
    ) -> CairoAir {
        let opcode_components = OpcodeComponentsImpl::new(
            cairo_claim.opcodes, interaction_elements, interaction_claim.opcodes,
        );

        let builtins_components = BuiltinComponentsImpl::new(
            cairo_claim.builtins, interaction_elements, interaction_claim.builtins,
        );

        let verifyinstruction_component = components::verify_instruction::Component {
            claim: *cairo_claim.verify_instruction,
            interaction_claim: *interaction_claim.verify_instruction,
            memoryaddresstoid_lookup_elements: interaction_elements.memory_address_to_id.clone(),
            memoryidtobig_lookup_elements: interaction_elements.memory_id_to_value.clone(),
            rangecheck_4_3_lookup_elements: interaction_elements.range_checks.rc_4_3.clone(),
            range_check_7_2_5_lookup_elements: interaction_elements.range_checks.rc_7_2_5.clone(),
            verifyinstruction_lookup_elements: interaction_elements.verify_instruction.clone(),
        };

        let memory_address_to_id_component = components::memory_address_to_id::Component {
            claim: *cairo_claim.memory_address_to_id,
            interaction_claim: *interaction_claim.memory_address_to_id,
            lookup_elements: interaction_elements.memory_address_to_id.clone(),
        };

        let memory_id_to_value_component = components::memory_id_to_big::BigComponent {
            log_n_rows: *cairo_claim.memory_id_to_value.big_log_size,
            interaction_claim: *interaction_claim.memory_id_to_value,
            lookup_elements: interaction_elements.memory_id_to_value.clone(),
            range_9_9_lookup_elements: interaction_elements.range_checks.rc_9_9.clone(),
        };

        let small_memory_id_to_value_component = components::memory_id_to_big::SmallComponent {
            log_n_rows: *cairo_claim.memory_id_to_value.small_log_size,
            interaction_claim: *interaction_claim.memory_id_to_value,
            lookup_elements: interaction_elements.memory_id_to_value.clone(),
            range_9_9_lookup_elements: interaction_elements.range_checks.rc_9_9.clone(),
        };

        let range_checks_components = RangeChecksComponentsImpl::new(
            cairo_claim.range_checks, interaction_elements, interaction_claim.range_checks,
        );

        let verify_bitwise_xor_9_component = components::verify_bitwise_xor_9::Component {
            claim: *cairo_claim.verify_bitwise_xor_9,
            interaction_claim: *interaction_claim.verify_bitwise_xor_9,
            verify_bitwise_xor_9_lookup_elements: interaction_elements.verify_bitwise_xor_9.clone(),
        };

        CairoAir {
            opcodes: opcode_components,
            verify_instruction: verifyinstruction_component,
            builtins: builtins_components,
            memory_address_to_id: memory_address_to_id_component,
            memory_id_to_value: (memory_id_to_value_component, small_memory_id_to_value_component),
            range_checks: range_checks_components,
            verify_bitwise_xor_9: verify_bitwise_xor_9_component,
        }
    }
}

impl CairoAirImpl of Air<CairoAir> {
    fn composition_log_degree_bound(self: @CairoAir) -> u32 {
        let mut max_degree = self.opcodes.max_constraint_log_degree_bound();
        max_degree =
            core::cmp::max(max_degree, self.verify_instruction.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.builtins.max_constraint_log_degree_bound());
        max_degree =
            core::cmp::max(max_degree, self.memory_address_to_id.max_constraint_log_degree_bound());
        let (memory_id_to_value_big, memory_id_to_value_small) = self.memory_id_to_value;
        max_degree =
            core::cmp::max(max_degree, memory_id_to_value_big.max_constraint_log_degree_bound());
        max_degree =
            core::cmp::max(max_degree, memory_id_to_value_small.max_constraint_log_degree_bound());
        max_degree =
            core::cmp::max(max_degree, self.range_checks.max_constraint_log_degree_bound());
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
        memory_id_to_value_big
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
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
        memory_id_to_value_big
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
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
    rc_6: components::range_check_vector::Rc6BitComponent,
    rc_11: components::range_check_vector::Rc11BitComponent,
    rc_12: components::range_check_vector::Rc12BitComponent,
    rc_18: components::range_check_vector::Rc18BitComponent,
    rc_19: components::range_check_vector::Rc19BitComponent,
    rc_3_6: components::range_check_vector::Rc3Bit6BitComponent,
    rc_4_3: components::range_check_vector::Rc4Bit3BitComponent,
    rc_9_9: components::range_check_vector::Rc9Bit9BitComponent,
    rc_7_2_5: components::range_check_vector::Rc7Bit2Bit5BitComponent,
    rc_3_6_6_3: components::range_check_vector::Rc3Bit6Bit6Bit3BitComponent,
}

#[generate_trait]
impl RangeChecksComponentsImpl of RangeChecksComponentsTrait {
    fn new(
        claim: @RangeChecksClaim,
        interaction_elements: @CairoInteractionElements,
        interaction_claim: @RangeChecksInteractionClaim,
    ) -> RangeChecksComponents {
        RangeChecksComponents {
            rc_6: components::range_check_vector::Rc6BitComponent {
                interaction_claim: *interaction_claim.rc_6,
                lookup_elements: interaction_elements.range_checks.rc_6.clone(),
            },
            rc_11: components::range_check_vector::Rc11BitComponent {
                interaction_claim: *interaction_claim.rc_11,
                lookup_elements: interaction_elements.range_checks.rc_11.clone(),
            },
            rc_12: components::range_check_vector::Rc12BitComponent {
                interaction_claim: *interaction_claim.rc_12,
                lookup_elements: interaction_elements.range_checks.rc_12.clone(),
            },
            rc_18: components::range_check_vector::Rc18BitComponent {
                interaction_claim: *interaction_claim.rc_18,
                lookup_elements: interaction_elements.range_checks.rc_18.clone(),
            },
            rc_19: components::range_check_vector::Rc19BitComponent {
                interaction_claim: *interaction_claim.rc_19,
                lookup_elements: interaction_elements.range_checks.rc_19.clone(),
            },
            rc_3_6: components::range_check_vector::Rc3Bit6BitComponent {
                interaction_claim: *interaction_claim.rc_3_6,
                lookup_elements: interaction_elements.range_checks.rc_3_6.clone(),
            },
            rc_4_3: components::range_check_vector::Rc4Bit3BitComponent {
                interaction_claim: *interaction_claim.rc_4_3,
                lookup_elements: interaction_elements.range_checks.rc_4_3.clone(),
            },
            rc_9_9: components::range_check_vector::Rc9Bit9BitComponent {
                interaction_claim: *interaction_claim.rc_9_9,
                lookup_elements: interaction_elements.range_checks.rc_9_9.clone(),
            },
            rc_7_2_5: components::range_check_vector::Rc7Bit2Bit5BitComponent {
                interaction_claim: *interaction_claim.rc_7_2_5,
                lookup_elements: interaction_elements.range_checks.rc_7_2_5.clone(),
            },
            rc_3_6_6_3: components::range_check_vector::Rc3Bit6Bit6Bit3BitComponent {
                interaction_claim: *interaction_claim.rc_3_6_6_3,
                lookup_elements: interaction_elements.range_checks.rc_3_6_6_3.clone(),
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
            .rc_19
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .rc_3_6
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
            .rc_9_9
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
    }

    fn max_constraint_log_degree_bound(self: @RangeChecksComponents) -> u32 {
        let mut max_degree = 0;
        max_degree = core::cmp::max(max_degree, self.rc_6.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_11.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_12.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_18.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_19.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_3_6.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_4_3.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_9_9.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_7_2_5.max_constraint_log_degree_bound());
        max_degree = core::cmp::max(max_degree, self.rc_3_6_6_3.max_constraint_log_degree_bound());
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
            .rc_3_6
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
    }
}


#[derive(Drop)]
pub struct BuiltinComponents {
    pub bitwise_builtin: Option<components::bitwise_builtin::Component>,
    pub range_check_96_builtin: Option<components::range_check_builtin_bits_96::Component>,
    pub range_check_128_builtin: Option<components::range_check_builtin_bits_128::Component>,
}

#[generate_trait]
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
            bitwise_builtin: bitwise_builtin_component,
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
        if let Option::Some(component) = self.bitwise_builtin.as_snap() {
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

        if let Option::Some(component) = self.bitwise_builtin.as_snap() {
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

#[derive(Drop)]
pub struct OpcodeComponents {
    add: Array<components::add_opcode::Component>,
    add_imm: Array<components::add_opcode_imm::Component>,
    add_small: Array<components::add_opcode_small::Component>,
    add_small_imm: Array<components::add_opcode_small_imm::Component>,
    add_ap: Array<components::add_ap_opcode::Component>,
    add_ap_op_1_base_fp: Array<components::add_ap_opcode_op_1_base_fp::Component>,
    add_ap_imm: Array<components::add_ap_opcode_imm::Component>,
    assert_eq: Array<components::assert_eq_opcode::Component>,
    assert_eq_imm: Array<components::assert_eq_opcode_imm::Component>,
    assert_eq_double_deref: Array<components::assert_eq_opcode_double_deref::Component>,
    call: Array<components::call_opcode::Component>,
    call_op_1_base_fp: Array<components::call_opcode_op_1_base_fp::Component>,
    call_rel: Array<components::call_opcode_rel::Component>,
    generic: Array<components::generic_opcode::Component>,
    jnz: Array<components::jnz_opcode::Component>,
    jnz_dst_base_fp: Array<components::jnz_opcode_dst_base_fp::Component>,
    jnz_taken: Array<components::jnz_opcode_taken::Component>,
    jnz_taken_dst_base_fp: Array<components::jnz_opcode_taken_dst_base_fp::Component>,
    jump: Array<components::jump_opcode::Component>,
    jump_double_deref: Array<components::jump_opcode_double_deref::Component>,
    jump_rel: Array<components::jump_opcode_rel::Component>,
    jump_rel_imm: Array<components::jump_opcode_rel_imm::Component>,
    mul: Array<components::mul_opcode::Component>,
    mul_imm: Array<components::mul_opcode_imm::Component>,
    mul_small: Array<components::mul_opcode_small::Component>,
    mul_small_imm: Array<components::mul_opcode_small_imm::Component>,
    ret: Array<components::ret_opcode::Component>,
}

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

        // Add Imm components
        let mut add_imm_components = array![];
        let mut add_imm_claims = claim.add_imm.span();
        let mut add_imm_interaction_claims = interaction_claim.add_imm.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (add_imm_claims.pop_front(), add_imm_interaction_claims.pop_front()) {
            add_imm_components
                .append(
                    components::add_opcode_imm::Component {
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
        assert!(add_imm_claims.is_empty());
        assert!(add_imm_interaction_claims.is_empty());

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

        // Add Small Imm components
        let mut add_small_imm_components = array![];
        let mut add_small_imm_claims = claim.add_small_imm.span();
        let mut add_small_imm_interaction_claims = interaction_claim.add_small_imm.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (add_small_imm_claims.pop_front(), add_small_imm_interaction_claims.pop_front()) {
            add_small_imm_components
                .append(
                    components::add_opcode_small_imm::Component {
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
        assert!(add_small_imm_claims.is_empty());
        assert!(add_small_imm_interaction_claims.is_empty());

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
                    },
                );
        }
        assert!(add_ap_claims.is_empty());
        assert!(add_ap_interaction_claims.is_empty());

        // Add AP Op1 Base FP components
        let mut add_ap_op_1_base_fp_components = array![];
        let mut add_ap_op_1_base_fp_claims = claim.add_ap_op_1_base_fp.span();
        let mut add_ap_op_1_base_fp_interaction_claims = interaction_claim
            .add_ap_op_1_base_fp
            .span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (
                add_ap_op_1_base_fp_claims.pop_front(),
                add_ap_op_1_base_fp_interaction_claims.pop_front(),
            ) {
            add_ap_op_1_base_fp_components
                .append(
                    components::add_ap_opcode_op_1_base_fp::Component {
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
        assert!(add_ap_op_1_base_fp_claims.is_empty());
        assert!(add_ap_op_1_base_fp_interaction_claims.is_empty());

        // Add AP Imm components
        let mut add_ap_imm_components = array![];
        let mut add_ap_imm_claims = claim.add_ap_imm.span();
        let mut add_ap_imm_interaction_claims = interaction_claim.add_ap_imm.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (add_ap_imm_claims.pop_front(), add_ap_imm_interaction_claims.pop_front()) {
            add_ap_imm_components
                .append(
                    components::add_ap_opcode_imm::Component {
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
        assert!(add_ap_imm_claims.is_empty());
        assert!(add_ap_imm_interaction_claims.is_empty());

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

        // Call Op1 Base FP components
        let mut call_op_1_base_fp_components = array![];
        let mut call_op_1_base_fp_claims = claim.call_op_1_base_fp.span();
        let mut call_op_1_base_fp_interaction_claims = interaction_claim.call_op_1_base_fp.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (
                call_op_1_base_fp_claims.pop_front(),
                call_op_1_base_fp_interaction_claims.pop_front(),
            ) {
            call_op_1_base_fp_components
                .append(
                    components::call_opcode_op_1_base_fp::Component {
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
        assert!(call_op_1_base_fp_claims.is_empty());
        assert!(call_op_1_base_fp_interaction_claims.is_empty());

        // Call Rel components
        let mut call_rel_components = array![];
        let mut call_rel_claims = claim.call_rel.span();
        let mut call_rel_interaction_claims = interaction_claim.call_rel.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (call_rel_claims.pop_front(), call_rel_interaction_claims.pop_front()) {
            call_rel_components
                .append(
                    components::call_opcode_rel::Component {
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
        assert!(call_rel_claims.is_empty());
        assert!(call_rel_interaction_claims.is_empty());

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
                        range_check_9_9_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9
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

        // Jnz Dst Base FP components
        let mut jnz_dst_base_fp_components = array![];
        let mut jnz_dst_base_fp_claims = claim.jnz_dst_base_fp.span();
        let mut jnz_dst_base_fp_interaction_claims = interaction_claim.jnz_dst_base_fp.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (jnz_dst_base_fp_claims.pop_front(), jnz_dst_base_fp_interaction_claims.pop_front()) {
            jnz_dst_base_fp_components
                .append(
                    components::jnz_opcode_dst_base_fp::Component {
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
        assert!(jnz_dst_base_fp_claims.is_empty());
        assert!(jnz_dst_base_fp_interaction_claims.is_empty());

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

        // Jnz Taken Dst Base FP components
        let mut jnz_taken_dst_base_fp_components = array![];
        let mut jnz_taken_dst_base_fp_claims = claim.jnz_taken_dst_base_fp.span();
        let mut jnz_taken_dst_base_fp_interaction_claims = interaction_claim
            .jnz_taken_dst_base_fp
            .span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (
                jnz_taken_dst_base_fp_claims.pop_front(),
                jnz_taken_dst_base_fp_interaction_claims.pop_front(),
            ) {
            jnz_taken_dst_base_fp_components
                .append(
                    components::jnz_opcode_taken_dst_base_fp::Component {
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
        assert!(jnz_taken_dst_base_fp_claims.is_empty());
        assert!(jnz_taken_dst_base_fp_interaction_claims.is_empty());

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
                    },
                );
        }
        assert!(mul_claims.is_empty());
        assert!(mul_interaction_claims.is_empty());

        // Mul Imm components
        let mut mul_imm_components = array![];
        let mut mul_imm_claims = claim.mul_imm.span();
        let mut mul_imm_interaction_claims = interaction_claim.mul_imm.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (mul_imm_claims.pop_front(), mul_imm_interaction_claims.pop_front()) {
            mul_imm_components
                .append(
                    components::mul_opcode_imm::Component {
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
                    },
                );
        }
        assert!(mul_imm_claims.is_empty());
        assert!(mul_imm_interaction_claims.is_empty());

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

        // Mul Small Imm components
        let mut mul_small_imm_components = array![];
        let mut mul_small_imm_claims = claim.mul_small_imm.span();
        let mut mul_small_imm_interaction_claims = interaction_claim.mul_small_imm.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (mul_small_imm_claims.pop_front(), mul_small_imm_interaction_claims.pop_front()) {
            mul_small_imm_components
                .append(
                    components::mul_opcode_small_imm::Component {
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
            add_imm: add_imm_components,
            add_small: add_small_components,
            add_small_imm: add_small_imm_components,
            add_ap: add_ap_components,
            add_ap_op_1_base_fp: add_ap_op_1_base_fp_components,
            add_ap_imm: add_ap_imm_components,
            assert_eq: assert_eq_components,
            assert_eq_imm: assert_eq_imm_components,
            assert_eq_double_deref: assert_eq_double_deref_components,
            call: call_components,
            call_op_1_base_fp: call_op_1_base_fp_components,
            call_rel: call_rel_components,
            generic: generic_components,
            jnz: jnz_components,
            jnz_dst_base_fp: jnz_dst_base_fp_components,
            jnz_taken: jnz_taken_components,
            jnz_taken_dst_base_fp: jnz_taken_dst_base_fp_components,
            jump: jump_components,
            jump_double_deref: jump_double_deref_components,
            jump_rel: jump_rel_components,
            jump_rel_imm: jump_rel_imm_components,
            mul: mul_components,
            mul_imm: mul_imm_components,
            mul_small: mul_small_components,
            mul_small_imm: mul_small_imm_components,
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

        for component in self.add_imm.span() {
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

        for component in self.add_small_imm.span() {
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

        for component in self.add_ap_op_1_base_fp.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.add_ap_imm.span() {
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

        for component in self.call.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.call_op_1_base_fp.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.call_rel.span() {
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

        for component in self.jnz_dst_base_fp.span() {
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

        for component in self.jnz_taken_dst_base_fp.span() {
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

        for component in self.mul_imm.span() {
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

        for component in self.mul_small_imm.span() {
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

        for component in self.add_imm.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.add_small.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.add_small_imm.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.add_ap.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.add_ap_op_1_base_fp.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.add_ap_imm.span() {
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

        for component in self.call.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.call_op_1_base_fp.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.call_rel.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.generic.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.jnz.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.jnz_dst_base_fp.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.jnz_taken.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.jnz_taken_dst_base_fp.span() {
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

        for component in self.mul_imm.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.mul_small.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.mul_small_imm.span() {
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
        for component in self.add_imm.span() {
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
        for component in self.add_small_imm.span() {
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
        for component in self.add_ap_op_1_base_fp.span() {
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
        for component in self.add_ap_imm.span() {
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
        for component in self.call_op_1_base_fp.span() {
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

        for component in self.call_rel.span() {
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
        for component in self.jnz_dst_base_fp.span() {
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

        for component in self.jnz_taken_dst_base_fp.span() {
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

        for component in self.mul_imm.span() {
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

        for component in self.mul_small_imm.span() {
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
    };
    #[test]
    #[cairofmt::skip]
    fn test_public_data_logup_sum() {
        let mut public_data_felts = array![
            52,0,2147450879,67600385,0,0,0,0,0,0,1,11,0,0,0,0,0,0,0,2,2147581952,285507585,0,0,0,0,
            0,0,3,30,0,0,0,0,0,0,0,4,2147450879,17268737,0,0,0,0,0,0,5,0,0,0,0,0,0,0,0,0,2147450879,
            67600385,0,0,0,0,0,0,6,1,0,0,0,0,0,0,0,7,2147450878,546013183,0,0,0,0,0,0,8,2147450877,
            34045953,0,0,0,0,0,0,9,4,0,0,0,0,0,0,0,10,2147450880,1208647676,0,0,0,0,0,0,7,
            2147450878,546013183,0,0,0,0,0,0,11,2147450880,1208385537,0,0,0,0,0,0,12,3,0,0,0,0,0,0,
            0,13,2147254271,1073905664,0,0,0,0,0,0,11,2147450880,1208385537,0,0,0,0,0,0,14,6,0,0,0,
            0,0,0,0,15,2147254271,1073905665,0,0,0,0,0,0,16,2147254272,1208123395,0,0,0,0,0,0,17,
            2147450879,1074167809,0,0,0,0,0,0,18,5,0,0,0,0,0,0,0,19,2147254272,1208123396,0,0,0,0,0,
            0,17,2147450879,1074167809,0,0,0,0,0,0,20,7,0,0,0,0,0,0,0,21,2147254272,1210482689,0,0,
            0,0,0,0,18,5,0,0,0,0,0,0,0,22,2147319808,1210482689,0,0,0,0,0,0,1073741824,0,0,0,0,0,0,
            17,134217728,2,2147581952,285507585,0,0,0,0,0,0,1073741825,4294967277,4294967295,
            4294967295,4294967295,4294967295,4294967295,16,134217728,7,2147450878,546013183,0,0,0,0,
            0,0,0,2147450879,67600385,0,0,0,0,0,0,6,1,0,0,0,0,0,0,0,23,2147450880,1074233345,0,0,0,
            0,0,0,24,50,0,0,0,0,0,0,0,25,2147450880,1208647671,0,0,0,0,0,0,26,2147450880,1208647680,
            0,0,0,0,0,0,2,2147581952,285507585,0,0,0,0,0,0,1073741826,4294967268,4294967295,
            4294967295,4294967295,4294967295,4294967295,16,134217728,27,2147450880,1208647667,0,0,0,
            0,0,0,28,2147450880,1208647668,0,0,0,0,0,0,29,2147450880,1208647669,0,0,0,0,0,0,30,
            2147450880,1208647670,0,0,0,0,0,0,31,2147450880,1209171963,0,0,0,0,0,0,32,2147450880,
            1208647672,0,0,0,0,0,0,33,2147450880,1208647673,0,0,0,0,0,0,34,2147450880,1208647674,0,
            0,0,0,0,0,35,2147450880,1208647675,0,0,0,0,0,0,10,2147450880,1208647676,0,0,0,0,0,0,36,
            2147450880,1208647677,0,0,0,0,0,0,7,2147450878,546013183,0,0,0,0,0,0,38,485,38,485,38,
            485,38,485,39,869,39,869,40,4965,40,4965,41,4997,188,5247,42,15237,42,15237,43,15461,43,
            15461,44,15717,44,15717,45,16485,45,16485,46,20581,46,20581,47,22373,47,22373,0,2,37,55,
            0,0,0,0,0,0,0,5,0,0,0,0,0,0,0,0,1,55,55,5,485,55
        ].span();
        let public_data: PublicData = Serde::deserialize(ref public_data_felts).unwrap();
        let dummy_lookup_elements = dummy_interaction_lookup_elements();

        let sum = public_data.logup_sum(@dummy_lookup_elements);

        assert_eq!(sum, qm31_const::<1953467177, 1393200374, 79713755, 621084348>());
    }

    fn dummy_interaction_lookup_elements() -> CairoInteractionElements {
        CairoInteractionElements {
            opcodes: LookupElementsDummyImpl::dummy(),
            verify_instruction: LookupElementsDummyImpl::dummy(),
            memory_address_to_id: LookupElementsDummyImpl::dummy(),
            memory_id_to_value: LookupElementsDummyImpl::dummy(),
            range_checks: RangeChecksInteractionElements {
                rc_6: LookupElementsDummyImpl::dummy(),
                rc_11: LookupElementsDummyImpl::dummy(),
                rc_12: LookupElementsDummyImpl::dummy(),
                rc_18: LookupElementsDummyImpl::dummy(),
                rc_19: LookupElementsDummyImpl::dummy(),
                rc_3_6: LookupElementsDummyImpl::dummy(),
                rc_4_3: LookupElementsDummyImpl::dummy(),
                rc_9_9: LookupElementsDummyImpl::dummy(),
                rc_7_2_5: LookupElementsDummyImpl::dummy(),
                rc_3_6_6_3: LookupElementsDummyImpl::dummy(),
            },
            verify_bitwise_xor_9: LookupElementsDummyImpl::dummy(),
        }
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
}
