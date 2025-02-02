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
use components::verify_instruction::{
    ClaimImpl as VerifyInstructionClaimImpl,
    InteractionClaimImpl as VerifyInstructionInteractionClaimImpl,
};
use core::num::traits::Zero;
use stwo_constraint_framework::{
    LookupElements, LookupElementsImpl, PreprocessedColumn, PreprocessedColumnKey,
    PreprocessedColumnSet, PreprocessedMaskValues, PreprocessedMaskValuesImpl,
};
use stwo_verifier_core::channel::{Channel, ChannelImpl};
use stwo_verifier_core::circle::CirclePoint;
use stwo_verifier_core::fields::Invertible;
use stwo_verifier_core::fields::m31::M31;
use stwo_verifier_core::fields::qm31::{QM31, QM31Zero};
use stwo_verifier_core::fri::FriConfig;
use stwo_verifier_core::pcs::PcsConfig;
use stwo_verifier_core::pcs::verifier::CommitmentSchemeVerifierImpl;
use stwo_verifier_core::utils::ArrayImpl;
use stwo_verifier_core::verifier::{Air, StarkProof, VerificationError, verify};
use stwo_verifier_core::{ColumnArray, ColumnSpan, TreeArray};

pub mod components;
pub mod utils;

const PREPROCESSED_COLUMNS_LOG_SIZES: [u32; 19] = [
    22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4,
];

// (Address, Id, Value)
pub type PublicMemory = Array<(u32, u32, [u32; 8])>;

type MemoryAddressToIdElements = LookupElements<2>;

type MemoryIdToBigElements = LookupElements<29>;

type OpcodeElements = LookupElements<3>;

type RangeCheck3BitElements = LookupElements<1>;

type RangeCheck6BitElements = LookupElements<1>;

type RangeCheck18BitElements = LookupElements<1>;

type RangeCheck19BitElements = LookupElements<1>;

type RangeCheck9Bit9BitElements = LookupElements<2>;

type RangeCheck4Bit3BitElements = LookupElements<2>;

type RangeCheck7Bit2Bit5BitElements = LookupElements<3>;

type VerifyInstructionElements = LookupElements<29>;


#[derive(Drop, Serde)]
pub struct CairoProof {
    pub claim: CairoClaim,
    pub interaction_claim: CairoInteractionClaim,
    pub stark_proof: StarkProof,
}

pub fn verify_cairo(proof: CairoProof) -> Result<(), CairoVerificationError> {
    let CairoProof { claim, interaction_claim, stark_proof } = proof;

    // Verify.
    let config = PcsConfig {
        pow_bits: 0,
        fri_config: FriConfig {
            log_blowup_factor: 1, log_last_layer_degree_bound: 2, n_queries: 15,
        },
    };
    let mut channel = ChannelImpl::new(0);
    let mut commitment_scheme = CommitmentSchemeVerifierImpl::new(config);

    let log_sizes = claim.log_sizes();

    // Preproccessed trace.
    commitment_scheme
        .commit(*stark_proof.commitment_scheme_proof.commitments[0], *log_sizes[0], ref channel);
    claim.mix_into(ref channel);

    commitment_scheme
        .commit(*stark_proof.commitment_scheme_proof.commitments[1], *log_sizes[1], ref channel);
    let interaction_elements = CairoInteractionElementsImpl::draw(ref channel);

    if lookup_sum(@claim, @interaction_elements, @interaction_claim).is_non_zero() {
        return Result::Err(CairoVerificationError::InvalidLogupSum);
    }

    interaction_claim.mix_into(ref channel);
    commitment_scheme
        .commit(*stark_proof.commitment_scheme_proof.commitments[2], *log_sizes[2], ref channel);

    let cairo_air = CairoAirNewImpl::new(@claim, @interaction_elements, @interaction_claim);
    if let Result::Err(err) = verify(cairo_air, ref channel, stark_proof, ref commitment_scheme) {
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

    sum
}

#[derive(Drop)]
struct CairoInteractionElements {
    pub opcodes: OpcodeElements,
    pub verify_instruction: VerifyInstructionElements,
    pub memory_address_to_id: MemoryAddressToIdElements,
    pub memory_id_to_value: MemoryIdToBigElements,
    pub range_check_19: RangeCheck19BitElements,
    pub range_check_9_9: RangeCheck9Bit9BitElements,
    pub range_check_7_2_5: RangeCheck7Bit2Bit5BitElements,
    pub range_check_4_3: RangeCheck4Bit3BitElements,
}

#[generate_trait]
impl CairoInteractionElementsImpl of CairoInteractionElementsTrait {
    fn draw(ref channel: Channel) -> CairoInteractionElements {
        CairoInteractionElements {
            opcodes: LookupElementsImpl::draw(ref channel),
            verify_instruction: LookupElementsImpl::draw(ref channel),
            memory_address_to_id: LookupElementsImpl::draw(ref channel),
            memory_id_to_value: LookupElementsImpl::draw(ref channel),
            range_check_19: LookupElementsImpl::draw(ref channel),
            range_check_9_9: LookupElementsImpl::draw(ref channel),
            range_check_7_2_5: LookupElementsImpl::draw(ref channel),
            range_check_4_3: LookupElementsImpl::draw(ref channel),
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
    pub range_check_128_builtin: Option<components::range_check_builtin_bits_128::Claim>,
    pub range_check_96_builtin: Option<components::range_check_builtin_bits_96::Claim>,
}

#[generate_trait]
impl BuiltinsClaimImpl of BuiltinsClaimTrait {
    fn mix_into(self: @BuiltinsClaim, ref channel: Channel) {
        if let Some(claim) = self.range_check_128_builtin {
            claim.mix_into(ref channel);
        }
        if let Some(claim) = self.range_check_96_builtin {
            claim.mix_into(ref channel);
        }
    }

    fn log_sizes(self: @BuiltinsClaim) -> TreeArray<Span<u32>> {
        let mut log_sizes = array![];

        if let Some(claim) = self.range_check_128_builtin {
            log_sizes.append(claim.log_sizes());
        }

        if let Some(claim) = self.range_check_96_builtin {
            log_sizes.append(claim.log_sizes());
        }

        utils::tree_array_concat_cols(log_sizes)
    }
}

#[derive(Drop, Serde, Copy)]
pub struct BuiltinsInteractionClaim {
    pub range_check_128_builtin: Option<components::range_check_builtin_bits_128::InteractionClaim>,
    pub range_check_96_builtin: Option<components::range_check_builtin_bits_96::InteractionClaim>,
}

#[generate_trait]
impl BuiltinsInteractionClaimImpl of BuiltinsInteractionClaimTrait {
    fn mix_into(self: @BuiltinsInteractionClaim, ref channel: Channel) {
        if let Some(claim) = self.range_check_128_builtin {
            claim.mix_into(ref channel);
        }
        if let Some(claim) = self.range_check_96_builtin {
            claim.mix_into(ref channel);
        }
    }

    fn sum(self: @BuiltinsInteractionClaim) -> QM31 {
        let mut sum = QM31Zero::zero();

        if let Some(claim) = self.range_check_128_builtin {
            sum += *claim.claimed_sum;
        }

        if let Some(claim) = self.range_check_96_builtin {
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
                self.rc_6.log_sizes(),
                self.rc_11.log_sizes(),
                self.rc_12.log_sizes(),
                self.rc_18.log_sizes(),
                self.rc_19.log_sizes(),
                self.rc_3_6.log_sizes(),
                self.rc_4_3.log_sizes(),
                self.rc_9_9.log_sizes(),
                self.rc_7_2_5.log_sizes(),
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
        let mut sum = QM31Zero::zero();
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
    // ...
}

#[generate_trait]
impl CairoClaimImpl of CairoClaimTrait {
    fn log_sizes(self: @CairoClaim) -> TreeArray<Span<u32>> {
        let mut aggregated_log_sizes = utils::tree_array_concat_cols(
            array![
                self.opcodes.log_sizes(),
                self.verify_instruction.log_sizes(),
                self.builtins.log_sizes(),
                self.memory_address_to_id.log_sizes(),
                self.memory_id_to_value.log_sizes(),
                self.range_checks.log_sizes(),
            ],
        );

        // Overwrite the preprocessed trace log sizes.
        let _invalid_preprocessed_trace_log_sizes = aggregated_log_sizes.pop_front();

        let mut preprocessed_trace_log_sizes = array![];
        for log_size in PREPROCESSED_COLUMNS_LOG_SIZES.span() {
            preprocessed_trace_log_sizes.append(*log_size); // IsFirst column
            preprocessed_trace_log_sizes.append(*log_size); // Seq column
        }

        let trace_log_sizes = aggregated_log_sizes.pop_front().unwrap();
        let interaction_log_sizes = aggregated_log_sizes.pop_front().unwrap();
        assert!(aggregated_log_sizes.is_empty());

        array![preprocessed_trace_log_sizes.span(), trace_log_sizes, interaction_log_sizes]
    }

    fn mix_into(self: @CairoClaim, ref channel: Channel) {
        self.opcodes.mix_into(ref channel);
        self.verify_instruction.mix_into(ref channel);
        self.builtins.mix_into(ref channel);
        self.memory_address_to_id.mix_into(ref channel);
        self.memory_id_to_value.mix_into(ref channel);
        self.range_checks.mix_into(ref channel);
    }
}

#[derive(Drop, Serde)]
pub struct CairoInteractionClaim {
    pub opcodes: OpcodeInteractionClaim,
    pub verify_instruction: components::verify_instruction::InteractionClaim,
    pub builtins: BuiltinsInteractionClaim,
    pub memory_address_to_id: components::memory_address_to_id::InteractionClaim,
    pub memory_id_to_value: components::memory_id_to_big::InteractionClaim,
    pub range_checks: RangeChecksInteractionClaim,
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
    }
}

#[derive(Drop, Serde)]
pub struct OpcodeInteractionClaim {
    // add: Array<PlaceHolder>,
    add: Array<components::add_opcode::InteractionClaim>,
    // add_imm: Array<PlaceHolder>,
    add_imm: Array<components::add_opcode_imm::InteractionClaim>,
    // add_small: Array<PlaceHolder>,
    add_small: Array<components::add_opcode_small::InteractionClaim>,
    // add_small_imm: Array<PlaceHolder>,
    add_small_imm: Array<components::add_opcode_small_imm::InteractionClaim>,
    // add_ap: Array<PlaceHolder>,
    add_ap: Array<components::add_ap_opcode::InteractionClaim>,
    // add_ap_op_1_base_fp: Array<PlaceHolder>,
    add_ap_op_1_base_fp: Array<components::add_ap_opcode_op_1_base_fp::InteractionClaim>,
    // add_ap_imm: Array<PlaceHolder>,
    add_ap_imm: Array<components::add_ap_opcode_imm::InteractionClaim>,
    // assert_eq: Array<PlaceHolder>,
    assert_eq: Array<components::assert_eq_opcode::InteractionClaim>,
    // assert_eq_imm: Array<PlaceHolder>,
    assert_eq_imm: Array<components::assert_eq_opcode_imm::InteractionClaim>,
    // assert_eq_double_deref: Array<PlaceHolder>,
    assert_eq_double_deref: Array<components::assert_eq_opcode_double_deref::InteractionClaim>,
    // call: Array<PlaceHolder>,
    call: Array<components::call_opcode::InteractionClaim>,
    // call_op_1_base_fp: Array<PlaceHolder>,
    call_op_1_base_fp: Array<components::call_opcode_op_1_base_fp::InteractionClaim>,
    // call_rel: Array<PlaceHolder>,
    call_rel: Array<components::call_opcode_rel::InteractionClaim>,
    // generic: Array<PlaceHolder>,
    generic: Array<components::generic_opcode::InteractionClaim>,
    // jnz: Array<PlaceHolder>,
    jnz: Array<components::jnz_opcode::InteractionClaim>,
    // jnz_dst_base_fp: Array<PlaceHolder>,
    jnz_dst_base_fp: Array<components::jnz_opcode_dst_base_fp::InteractionClaim>,
    // jnz_taken: Array<PlaceHolder>,
    jnz_taken: Array<components::jnz_opcode_taken::InteractionClaim>,
    // jnz_taken_dst_base_fp: Array<PlaceHolder>,
    jnz_taken_dst_base_fp: Array<components::jnz_opcode_taken_dst_base_fp::InteractionClaim>,
    // jump: Array<PlaceHolder>,
    jump: Array<components::jump_opcode::InteractionClaim>,
    // jump_double_deref: Array<PlaceHolder>,
    jump_double_deref: Array<components::jump_opcode_double_deref::InteractionClaim>,
    // jump_rel: Array<PlaceHolder>,
    jump_rel: Array<components::jump_opcode_rel::InteractionClaim>,
    // jump_rel_imm: Array<PlaceHolder>,
    jump_rel_imm: Array<components::jump_opcode_rel_imm::InteractionClaim>,
    // mul: Array<PlaceHolder>,
    mul: Array<components::mul_opcode::InteractionClaim>,
    // mul_imm: Array<PlaceHolder>,
    mul_imm: Array<components::mul_opcode_imm::InteractionClaim>,
    // ret: Array<PlaceHolder>,
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

        for interaction_claim in self.ret.span() {
            interaction_claim.mix_into(ref channel);
        };
    }

    fn sum(self: @OpcodeInteractionClaim) -> QM31 {
        let mut sum = QM31Zero::zero();

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

        for interaction_claim in self.ret.span() {
            sum += *interaction_claim.claimed_sum;
        }

        sum
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
        let mut sum = QM31Zero::zero();

        // TODO(andrew): Consider batch inverse here.
        for entry in self.public_memory.span() {
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
}

#[derive(Drop, Serde, Copy)]
pub struct CasmState {
    pub pc: M31,
    pub ap: M31,
    pub fp: M31,
}

type PlaceHolder = u32;

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

        // for claim in self.generic.span() {
        //     log_sizes.append(claim.log_sizes());
        // };

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

        // for claim in self.mul.span() {
        //     log_sizes.append(claim.log_sizes());
        // };

        // for claim in self.mul_imm.span() {
        //     log_sizes.append(claim.log_sizes());
        // };

        for claim in self.ret.span() {
            log_sizes.append(claim.log_sizes());
        }

        utils::tree_array_concat_cols(log_sizes)
    }
}

#[derive(Drop, Debug)]
pub enum CairoVerificationError {
    InvalidLogupSum,
    Stark: VerificationError,
}

#[derive(Drop)]
pub struct CairoAir {
    opcodes: OpcodeComponents,
    verify_instruction: components::verify_instruction::Component,
    memory_address_to_id: components::memory_address_to_id::Component,
    memory_id_to_value: (
        components::memory_id_to_big::BigComponent, components::memory_id_to_big::SmallComponent,
    ),
    range_check_19: components::range_check_vector::Rc19BitComponent,
    range_check_9_9: components::range_check_vector::Rc9Bit9BitComponent,
    range_check_7_2_5: components::range_check_vector::Rc7Bit2Bit5BitComponent,
    range_check_4_3: components::range_check_vector::Rc4Bit3BitComponent,
    // ...
    preprocessed_columns: Array<PreprocessedColumn>,
}

#[generate_trait]
impl CairoAirNewImpl of CairoAirNewTrait {
    fn new(
        cairo_claim: @CairoClaim,
        interaction_elements: @CairoInteractionElements,
        interaction_claim: @CairoInteractionClaim,
    ) -> CairoAir {
        let mut preprocessed_columns = array![];

        // TODO: This could be a constant.
        for is_first_log_size in PREPROCESSED_COLUMNS_LOG_SIZES.span() {
            preprocessed_columns.append(PreprocessedColumn::IsFirst(*is_first_log_size));
            preprocessed_columns.append(PreprocessedColumn::Seq(*is_first_log_size));
        }

        let opcode_components = OpcodeComponentsImpl::new(
            cairo_claim.opcodes, interaction_elements, interaction_claim.opcodes,
        );

        let verifyinstruction_component = components::verify_instruction::Component {
            claim: *cairo_claim.verify_instruction,
            interaction_claim: *interaction_claim.verify_instruction,
            memoryaddresstoid_lookup_elements: interaction_elements.memory_address_to_id.clone(),
            memoryidtobig_lookup_elements: interaction_elements.memory_id_to_value.clone(),
            rangecheck_4_3_lookup_elements: interaction_elements.range_check_4_3.clone(),
            range_check_7_2_5_lookup_elements: interaction_elements.range_check_7_2_5.clone(),
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
            range_9_9_lookup_elements: interaction_elements.range_check_9_9.clone(),
        };

        let small_memory_id_to_value_component = components::memory_id_to_big::SmallComponent {
            log_n_rows: *cairo_claim.memory_id_to_value.small_log_size,
            interaction_claim: *interaction_claim.memory_id_to_value,
            lookup_elements: interaction_elements.memory_id_to_value.clone(),
            range_9_9_lookup_elements: interaction_elements.range_check_9_9.clone(),
        };

        let range_check_19_component = components::range_check_vector::Rc19BitComponent {
            lookup_elements: interaction_elements.range_check_19.clone(),
            interaction_claim: interaction_claim.range_checks.rc_19.clone(),
        };

        let range_check_9_9_component = components::range_check_vector::Rc9Bit9BitComponent {
            lookup_elements: interaction_elements.range_check_9_9.clone(),
            interaction_claim: interaction_claim.range_checks.rc_9_9.clone(),
        };

        let range_check_7_2_5_component = components::range_check_vector::Rc7Bit2Bit5BitComponent {
            lookup_elements: interaction_elements.range_check_7_2_5.clone(),
            interaction_claim: interaction_claim.range_checks.rc_7_2_5.clone(),
        };

        let range_check_4_3_component = components::range_check_vector::Rc4Bit3BitComponent {
            lookup_elements: interaction_elements.range_check_4_3.clone(),
            interaction_claim: interaction_claim.range_checks.rc_4_3.clone(),
        };

        CairoAir {
            opcodes: opcode_components,
            verify_instruction: verifyinstruction_component,
            memory_address_to_id: memory_address_to_id_component,
            memory_id_to_value: (memory_id_to_value_component, small_memory_id_to_value_component),
            range_check_19: range_check_19_component,
            range_check_9_9: range_check_9_9_component,
            range_check_7_2_5: range_check_7_2_5_component,
            range_check_4_3: range_check_4_3_component,
            preprocessed_columns,
        }
    }
}

impl CairoAirImpl of Air<CairoAir> {
    fn composition_log_degree_bound(self: @CairoAir) -> u32 {
        let mut max_degree = self.opcodes.max_constraint_log_degree_bound();
        max_degree =
            core::cmp::max(max_degree, self.verify_instruction.max_constraint_log_degree_bound());
        max_degree =
            core::cmp::max(max_degree, self.memory_address_to_id.max_constraint_log_degree_bound());
        let (memory_id_to_value_big, memory_id_to_value_small) = self.memory_id_to_value;
        max_degree =
            core::cmp::max(max_degree, memory_id_to_value_big.max_constraint_log_degree_bound());
        max_degree =
            core::cmp::max(max_degree, memory_id_to_value_small.max_constraint_log_degree_bound());
        max_degree =
            core::cmp::max(max_degree, self.range_check_19.max_constraint_log_degree_bound());
        max_degree =
            core::cmp::max(max_degree, self.range_check_9_9.max_constraint_log_degree_bound());
        max_degree =
            core::cmp::max(max_degree, self.range_check_7_2_5.max_constraint_log_degree_bound());
        max_degree =
            core::cmp::max(max_degree, self.range_check_4_3.max_constraint_log_degree_bound());
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
            .range_check_19
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .range_check_9_9
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .range_check_7_2_5
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );
        self
            .range_check_4_3
            .mask_points(
                ref preprocessed_column_set,
                ref trace_mask_points,
                ref interaction_trace_mask_points,
                point,
            );

        let preprocessed_trace_mask_points = preprocessed_trace_mask_points(
            preprocessed_column_set,
        );

        array![preprocessed_trace_mask_points, trace_mask_points, interaction_trace_mask_points]
    }

    fn eval_composition_polynomial_at_point(
        self: @CairoAir,
        point: CirclePoint<QM31>,
        mask_values: @TreeArray<ColumnArray<Array<QM31>>>,
        random_coeff: QM31,
    ) -> QM31 {
        let mut sum = QM31Zero::zero();

        let mut preprocessed_mask_values = PreprocessedMaskValuesImpl::new(
            mask_values[0].span(), self.preprocessed_columns.span(),
        );

        let mut trace_mask_values = mask_values[1].span();
        let mut interaction_trace_mask_values = mask_values[2].span();

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
            .range_check_19
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .range_check_9_9
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .range_check_7_2_5
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        self
            .range_check_4_3
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
    preprocessed_column_set: PreprocessedColumnSet,
) -> ColumnArray<Array<CirclePoint<QM31>>> {
    let mut mask_points = array![];

    let PreprocessedColumnSet { values: original_values, mut contains } = preprocessed_column_set;

    for log_size in PREPROCESSED_COLUMNS_LOG_SIZES.span() {
        let is_first = PreprocessedColumn::IsFirst(*log_size);
        let seq = PreprocessedColumn::Seq(*log_size);

        // After adding the mask points, remove the item from the set.
        contains.insert(PreprocessedColumnKey::encode(@is_first), false);
        contains.insert(PreprocessedColumnKey::encode(@seq), false);
    }

    // Check all the original values have been handled.
    for value in original_values {
        let column_key = PreprocessedColumnKey::encode(@value);
        assert!(!contains.get(column_key));
    }

    mask_points
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
    // generic: Array<components::generic_opcode::Component>,
    generic: Array<PlaceHolder>,
    jnz: Array<components::jnz_opcode::Component>,
    jnz_dst_base_fp: Array<components::jnz_opcode_dst_base_fp::Component>,
    jnz_taken: Array<components::jnz_opcode_taken::Component>,
    jnz_taken_dst_base_fp: Array<components::jnz_opcode_taken_dst_base_fp::Component>,
    jump: Array<components::jump_opcode::Component>,
    jump_double_deref: Array<components::jump_opcode_double_deref::Component>,
    jump_rel: Array<components::jump_opcode_rel::Component>,
    jump_rel_imm: Array<components::jump_opcode_rel_imm::Component>,
    // mul: Array<components::mul_opcode::Component>,
    mul: Array<PlaceHolder>,
    // mul_imm: Array<components::mul_opcode_imm::Component>,
    mul_imm: Array<PlaceHolder>,
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
        // let mut generic_claims = claim.generic.span();
        // let mut generic_interaction_claims = interaction_claim.generic.span();
        // while let (Option::Some(claim), Option::Some(interaction_claim)) =
        //     (generic_claims.pop_front(), generic_interaction_claims.pop_front()) {
        //     generic_components
        //         .append(
        //             components::generic_opcode::Component {
        //                 claim: *claim,
        //                 interaction_claim: *interaction_claim,
        //                 memory_address_to_id_lookup_elements: interaction_elements
        //                     .memory_address_to_id
        //                     .clone(),
        //                 memory_id_to_big_lookup_elements: interaction_elements
        //                     .memory_id_to_value
        //                     .clone(),
        //                 opcodes_lookup_elements: interaction_elements.opcodes.clone(),
        //                 verify_instruction_lookup_elements: interaction_elements
        //                     .verify_instruction
        //                     .clone(),
        //                 range_check_19_lookup_elements:
        //                 interaction_elements.range_check_19.clone(),
        //                 range_check_9_9_lookup_elements: interaction_elements
        //                     .range_check_9_9
        //                     .clone(),
        //             },
        //         );
        // };
        // assert!(generic_claims.is_empty());
        // assert!(generic_interaction_claims.is_empty());

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
        // let mut mul_claims = claim.mul.span();
        // let mut mul_interaction_claims = interaction_claim.mul.span();
        // while let (Option::Some(claim), Option::Some(interaction_claim)) =
        //     (mul_claims.pop_front(), mul_interaction_claims.pop_front()) {
        //     mul_components
        //         .append(
        //             components::mul_opcode::Component {
        //                 claim: *claim,
        //                 interaction_claim: *interaction_claim,
        //                 memory_address_to_id_lookup_elements: interaction_elements
        //                     .memory_address_to_id
        //                     .clone(),
        //                 memory_id_to_big_lookup_elements: interaction_elements
        //                     .memory_id_to_value
        //                     .clone(),
        //                 opcodes_lookup_elements: interaction_elements.opcodes.clone(),
        //                 verify_instruction_lookup_elements: interaction_elements
        //                     .verify_instruction
        //                     .clone(),
        //                 range_check_19_lookup_elements:
        //                 interaction_elements.range_check_19.clone(),
        //             },
        //         );
        // };
        // assert!(mul_claims.is_empty());
        // assert!(mul_interaction_claims.is_empty());

        // Mul Imm components
        let mut mul_imm_components = array![];
        // let mut mul_imm_claims = claim.mul_imm.span();
        // let mut mul_imm_interaction_claims = interaction_claim.mul_imm.span();
        // while let (Option::Some(claim), Option::Some(interaction_claim)) =
        //     (mul_imm_claims.pop_front(), mul_imm_interaction_claims.pop_front()) {
        //     mul_imm_components
        //         .append(
        //             components::mul_opcode_imm::Component {
        //                 claim: *claim,
        //                 interaction_claim: *interaction_claim,
        //                 memory_address_to_id_lookup_elements: interaction_elements
        //                     .memory_address_to_id
        //                     .clone(),
        //                 memory_id_to_big_lookup_elements: interaction_elements
        //                     .memory_id_to_value
        //                     .clone(),
        //                 opcodes_lookup_elements: interaction_elements.opcodes.clone(),
        //                 verify_instruction_lookup_elements: interaction_elements
        //                     .verify_instruction
        //                     .clone(),
        //                 range_check_19_lookup_elements:
        //                 interaction_elements.range_check_19.clone(),
        //             },
        //         );
        // };
        // assert!(mul_imm_claims.is_empty());
        // assert!(mul_imm_interaction_claims.is_empty());

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

        // for component in self.generic.span() {
        //     component.mask_points(ref preprocessed_column_set, ref trace_mask_points, ref
        //     interaction_trace_mask_points, point);
        // };

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

        // for component in self.mul.span() {
        //     component.mask_points(ref preprocessed_column_set, ref trace_mask_points, ref
        //     interaction_trace_mask_points, point);
        // };

        // for component in self.mul_imm.span() {
        //     component.mask_points(ref preprocessed_column_set, ref trace_mask_points, ref
        //     interaction_trace_mask_points, point);
        // };

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

        // for component in self.generic.span() {
        //     max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        // };

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

        // for component in self.mul.span() {
        //     max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        // };

        // for component in self.mul_imm.span() {
        //     max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        // };

        for component in self.ret.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        max_degree
    }

    fn evaluate_constraints_at_point(
        self: @OpcodeComponents,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Array<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Array<QM31>>,
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

        // for component in self.generic.span() {
        //     component
        //         .evaluate_constraints_at_point(
        //             ref sum,
        //             ref preprocessed_mask_values,
        //             ref trace_mask_values,
        //             ref interaction_trace_mask_values,
        //             random_coeff,
        //             point,
        //         );
        // };

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

        // for component in self.mul.span() {
        //     component
        //         .evaluate_constraints_at_point(
        //             ref sum,
        //             ref preprocessed_mask_values,
        //             ref trace_mask_values,
        //             ref interaction_trace_mask_values,
        //             random_coeff,
        //             point,
        //         );
        // };

        // for component in self.mul_imm.span() {
        //     component
        //         .evaluate_constraints_at_point(
        //             ref sum,
        //             ref preprocessed_mask_values,
        //             ref trace_mask_values,
        //             ref interaction_trace_mask_values,
        //             random_coeff,
        //             point,
        //         );
        // };

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
