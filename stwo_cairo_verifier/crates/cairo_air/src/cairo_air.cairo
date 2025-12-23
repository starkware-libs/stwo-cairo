use components::add_ap_opcode::InteractionClaimImpl as AddApOpcodeInteractionClaimImpl;
use components::add_mod_builtin::InteractionClaimImpl as AddModBuiltinInteractionClaimImpl;
use components::add_opcode::InteractionClaimImpl as AddOpcodeInteractionClaimImpl;
use components::add_opcode_small::InteractionClaimImpl as AddOpcodeSmallInteractionClaimImpl;
use components::assert_eq_opcode::InteractionClaimImpl as AssertEqOpcodeInteractionClaimImpl;
use components::assert_eq_opcode_double_deref::InteractionClaimImpl as AssertEqOpcodeDoubleDerefInteractionClaimImpl;
use components::assert_eq_opcode_imm::InteractionClaimImpl as AssertEqOpcodeImmInteractionClaimImpl;
use components::bitwise_builtin::InteractionClaimImpl as BitwiseBuiltinInteractionClaimImpl;
use components::blake_compress_opcode::InteractionClaimImpl as BlakeCompressOpcodeInteractionClaimImpl;
use components::blake_g::InteractionClaimImpl as BlakeGInteractionClaimImpl;
use components::blake_round::InteractionClaimImpl as BlakeRoundInteractionClaimImpl;
use components::blake_round_sigma::InteractionClaimImpl as BlakeRoundSigmaInteractionClaimImpl;
use components::call_opcode_abs::InteractionClaimImpl as CallOpcodeAbsInteractionClaimImpl;
use components::call_opcode_rel_imm::InteractionClaimImpl as CallOpcodeRelImmInteractionClaimImpl;
use components::generic_opcode::InteractionClaimImpl as GenericOpcodeInteractionClaimImpl;
use components::jnz_opcode_non_taken::InteractionClaimImpl as JnzOpcodeNonTakenInteractionClaimImpl;
use components::jnz_opcode_taken::InteractionClaimImpl as JnzOpcodeTakenInteractionClaimImpl;
use components::jump_opcode_abs::InteractionClaimImpl as JumpOpcodeAbsInteractionClaimImpl;
use components::jump_opcode_double_deref::InteractionClaimImpl as JumpOpcodeDoubleDerefInteractionClaimImpl;
use components::jump_opcode_rel::InteractionClaimImpl as JumpOpcodeRelInteractionClaimImpl;
use components::jump_opcode_rel_imm::InteractionClaimImpl as JumpOpcodeRelImmInteractionClaimImpl;
use components::memory_address_to_id::InteractionClaimImpl as MemoryAddressToIdInteractionClaimImpl;
use components::memory_id_to_big::{
    InteractionClaimImpl as MemoryIdToBigInteractionClaimImpl, LARGE_MEMORY_VALUE_ID_BASE,
};
use components::mul_mod_builtin::InteractionClaimImpl as MulModBuiltinInteractionClaimImpl;
use components::mul_opcode::InteractionClaimImpl as MulOpcodeInteractionClaimImpl;
use components::mul_opcode_small::InteractionClaimImpl as MulOpcodeSmallInteractionClaimImpl;
use components::pedersen_builtin::InteractionClaimImpl as PedersenBuiltinInteractionClaimImpl;
use components::poseidon_builtin::InteractionClaimImpl as PoseidonBuiltinInteractionClaimImpl;
use components::qm_31_add_mul_opcode::InteractionClaimImpl as Qm31AddMulOpcodeInteractionClaimImpl;
use components::range_check_builtin_bits_128::InteractionClaimImpl as RangeCheckBuiltinBits128InteractionClaimImpl;
use components::range_check_builtin_bits_96::InteractionClaimImpl as RangeCheckBuiltinBits96InteractionClaimImpl;
use components::ret_opcode::InteractionClaimImpl as RetOpcodeInteractionClaimImpl;
use components::triple_xor_32::InteractionClaimImpl as TripleXor32InteractionClaimImpl;
use components::verify_bitwise_xor_12::InteractionClaimImpl as VerifyBitwiseXor12InteractionClaimImpl;
use components::verify_bitwise_xor_4::InteractionClaimImpl as VerifyBitwiseXor4InteractionClaimImpl;
use components::verify_bitwise_xor_7::InteractionClaimImpl as VerifyBitwiseXor7InteractionClaimImpl;
use components::verify_bitwise_xor_8::InteractionClaimImpl as VerifyBitwiseXor8InteractionClaimImpl;
use components::verify_bitwise_xor_9::InteractionClaimImpl as VerifyBitwiseXor9InteractionClaimImpl;
use components::verify_instruction::InteractionClaimImpl as VerifyInstructionInteractionClaimImpl;
use core::box::BoxImpl;
use core::num::traits::Zero;
use stwo_cairo_air::cairo_component::CairoComponent;
use stwo_cairo_air::cairo_interaction_elements::CairoInteractionElements;
use stwo_cairo_air::claim::ClaimTrait;
use stwo_cairo_air::components::{
    add_ap_opcode, add_mod_builtin, add_opcode, add_opcode_small, assert_eq_opcode,
    assert_eq_opcode_double_deref, assert_eq_opcode_imm, bitwise_builtin, blake_compress_opcode,
    blake_g, blake_round, blake_round_sigma, call_opcode_abs, call_opcode_rel_imm, generic_opcode,
    jnz_opcode_non_taken, jnz_opcode_taken, jump_opcode_abs, jump_opcode_double_deref,
    jump_opcode_rel, jump_opcode_rel_imm, mul_mod_builtin, mul_opcode, mul_opcode_small,
    pedersen_builtin, poseidon_builtin, qm_31_add_mul_opcode, range_check_builtin_bits_128,
    range_check_builtin_bits_96, ret_opcode, triple_xor_32, verify_bitwise_xor_12,
};
use crate::P_U32;

#[cfg(not(feature: "poseidon252_verifier"))]
pub mod pedersen_context_imports {
    pub use stwo_cairo_air::pedersen::{PedersenContextComponents, PedersenContextComponentsImpl};
}
#[cfg(not(feature: "poseidon252_verifier"))]
use pedersen_context_imports::*;
#[cfg(or(not(feature: "poseidon252_verifier"), feature: "poseidon_outputs_packing"))]
pub mod poseidon_context_imports {
    pub use stwo_cairo_air::poseidon::{PoseidonContextComponents, PoseidonContextComponentsImpl};
}
#[cfg(or(not(feature: "poseidon252_verifier"), feature: "poseidon_outputs_packing"))]
use poseidon_context_imports::*;
use stwo_cairo_air::pedersen::{
    PedersenContextClaim, PedersenContextInteractionClaim, PedersenContextInteractionClaimImpl,
};
use stwo_cairo_air::poseidon::{
    PoseidonContextClaim, PoseidonContextInteractionClaim, PoseidonContextInteractionClaimImpl,
};
use stwo_cairo_air::preprocessed_columns::PREPROCESSED_COLUMN_LOG_SIZE;
use stwo_cairo_air::range_checks::{
    RangeChecksClaim, RangeChecksComponents, RangeChecksComponentsImpl, RangeChecksInteractionClaim,
    RangeChecksInteractionClaimImpl, RangeChecksInteractionElementsImpl,
};
use stwo_cairo_air::{PublicData, PublicDataImpl, RelationUsesDict, components, utils};
use stwo_constraint_framework::{
    LookupElements, LookupElementsImpl, PreprocessedMaskValues, PreprocessedMaskValuesImpl,
};
use stwo_verifier_core::channel::{Channel, ChannelTrait};
use stwo_verifier_core::circle::CirclePoint;
use stwo_verifier_core::fields::qm31::QM31;
use stwo_verifier_core::pcs::verifier::CommitmentSchemeVerifierImpl;
use stwo_verifier_core::utils::{ArrayImpl, OptionImpl, pow2};
use stwo_verifier_core::verifier::Air;
use stwo_verifier_core::{ColumnSpan, TreeArray, TreeSpan};


pub type Cube252Elements = LookupElements<20>;

pub type MemoryAddressToIdElements = LookupElements<2>;

pub type MemoryIdToBigElements = LookupElements<29>;

pub type OpcodesElements = LookupElements<3>;

pub type PedersenAggregatorElements = LookupElements<3>;

pub type PartialEcMulElements = LookupElements<72>;

pub type PedersenPointsTableElements = LookupElements<57>;

pub type PoseidonFullRoundChainElements = LookupElements<32>;

pub type Poseidon3PartialRoundsChainElements = LookupElements<42>;

pub type PoseidonAggregatorElements = LookupElements<6>;

pub type PoseidonRoundKeysElements = LookupElements<31>;

pub type BlakeGElements = LookupElements<20>;

pub type BlakeRoundElements = LookupElements<35>;

pub type BlakeRoundSigmaElements = LookupElements<17>;

pub type TripleXor32Elements = LookupElements<8>;

pub type RangeCheck252Width27Elements = LookupElements<10>;

pub type VerifyInstructionElements = LookupElements<7>;

pub type VerifyBitwiseXor_4Elements = LookupElements<3>;

pub type VerifyBitwiseXor_7Elements = LookupElements<3>;

pub type VerifyBitwiseXor_8Elements = LookupElements<3>;

pub type VerifyBitwiseXor_8_BElements = LookupElements<3>;

pub type VerifyBitwiseXor_9Elements = LookupElements<3>;

pub type VerifyBitwiseXor_12Elements = LookupElements<3>;

/// Validates that every `mask_value` provided in the proof (in `sampled_values`) is used by at
/// least one component.
///
/// Since `eval_composition_polynomial_at_point` is responsible for validating the *structure*
/// of `sampled_values` in the proof, it needs to ensure that all sampled preprocessed
/// mask values are actually used. Otherwise, the prover would have the freedom to
/// send a sample of a column even if it is unused, adding another term to the FRI quotients.
///
/// Additionally, there is a sanity check that the columns in the trace and interaction-trace were
/// consumed by the components.
/// This is not strictly necessary as the verifier generates the column indices on its own and only
/// access samples of columns for which it knows about.
fn validate_mask_usage(
    preprocessed_mask_values: PreprocessedMaskValues,
    trace_mask_values: ColumnSpan<Span<QM31>>,
    interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
) {
    preprocessed_mask_values.validate_usage();
    assert!(trace_mask_values.is_empty());
    assert!(interaction_trace_mask_values.is_empty());
}

#[derive(Drop, Serde)]
pub struct CairoClaim {
    pub public_data: PublicData,
    pub add: Option<add_opcode::Claim>,
    pub add_small: Option<add_opcode_small::Claim>,
    pub add_ap: Option<add_ap_opcode::Claim>,
    pub assert_eq: Option<assert_eq_opcode::Claim>,
    pub assert_eq_imm: Option<assert_eq_opcode_imm::Claim>,
    pub assert_eq_double_deref: Option<assert_eq_opcode_double_deref::Claim>,
    pub blake: Option<blake_compress_opcode::Claim>,
    pub call: Option<call_opcode_abs::Claim>,
    pub call_rel_imm: Option<call_opcode_rel_imm::Claim>,
    pub generic: Option<generic_opcode::Claim>,
    pub jnz: Option<jnz_opcode_non_taken::Claim>,
    pub jnz_taken: Option<jnz_opcode_taken::Claim>,
    pub jump: Option<jump_opcode_abs::Claim>,
    pub jump_double_deref: Option<jump_opcode_double_deref::Claim>,
    pub jump_rel: Option<jump_opcode_rel::Claim>,
    pub jump_rel_imm: Option<jump_opcode_rel_imm::Claim>,
    pub mul: Option<mul_opcode::Claim>,
    pub mul_small: Option<mul_opcode_small::Claim>,
    pub qm31: Option<qm_31_add_mul_opcode::Claim>,
    pub ret: Option<ret_opcode::Claim>,
    pub verify_instruction: components::verify_instruction::Claim,
    pub blake_round: Option<components::blake_round::Claim>,
    pub blake_g: Option<components::blake_g::Claim>,
    pub blake_round_sigma: Option<components::blake_round_sigma::Claim>,
    pub triple_xor_32: Option<components::triple_xor_32::Claim>,
    pub verify_bitwise_xor_12: Option<components::verify_bitwise_xor_12::Claim>,
    pub add_mod_builtin: Option<add_mod_builtin::Claim>,
    pub bitwise_builtin: Option<bitwise_builtin::Claim>,
    pub mul_mod_builtin: Option<mul_mod_builtin::Claim>,
    pub pedersen_builtin: Option<pedersen_builtin::Claim>,
    pub poseidon_builtin: Option<poseidon_builtin::Claim>,
    pub range_check_96_builtin: Option<range_check_builtin_bits_96::Claim>,
    pub range_check_128_builtin: Option<range_check_builtin_bits_128::Claim>,
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

pub impl CairoClaimImpl of ClaimTrait<CairoClaim> {
    fn log_sizes(self: @CairoClaim) -> TreeArray<Span<u32>> {
        let mut log_sizes_list = array![];
        if let Some(claim) = self.add {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.add_small {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.add_ap {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.assert_eq {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.assert_eq_imm {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.assert_eq_double_deref {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.blake {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.call {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.call_rel_imm {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.generic {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.jnz {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.jnz_taken {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.jump {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.jump_double_deref {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.jump_rel {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.jump_rel_imm {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.mul {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.mul_small {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.qm31 {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.ret {
            log_sizes_list.append(claim.log_sizes());
        }

        log_sizes_list.append(self.verify_instruction.log_sizes());

        if let Some(claim) = self.blake_round {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.blake_g {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.blake_round_sigma {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.triple_xor_32 {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.verify_bitwise_xor_12 {
            log_sizes_list.append(claim.log_sizes());
        }

        if let Some(claim) = self.add_mod_builtin {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.bitwise_builtin {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.mul_mod_builtin {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.pedersen_builtin {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.poseidon_builtin {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.range_check_96_builtin {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.range_check_128_builtin {
            log_sizes_list.append(claim.log_sizes());
        }
        log_sizes_list.append(self.pedersen_context.log_sizes());
        log_sizes_list.append(self.poseidon_context.log_sizes());
        log_sizes_list.append(self.memory_address_to_id.log_sizes());
        log_sizes_list.append(self.memory_id_to_value.log_sizes());
        log_sizes_list.append(self.range_checks.log_sizes());
        log_sizes_list.append(self.verify_bitwise_xor_4.log_sizes());
        log_sizes_list.append(self.verify_bitwise_xor_7.log_sizes());
        log_sizes_list.append(self.verify_bitwise_xor_8.log_sizes());
        log_sizes_list.append(self.verify_bitwise_xor_9.log_sizes());

        let mut aggregated_log_sizes = utils::tree_array_concat_cols(log_sizes_list);

        // Override the preprocessed trace log sizes, since they come from a global setting
        // rather than computed by concatenating preprocessed log sizes of the individual
        // components.
        // TODO(ilya): consider removing the generation of `_invalid_preprocessed_trace_log_sizes`.
        let boxed_triplet: Box<[Span<u32>; 3]> = *aggregated_log_sizes.span().try_into().unwrap();
        let [_invalid_preprocessed_trace_log_sizes, trace_log_sizes, interaction_log_sizes] =
            boxed_triplet
            .unbox();

        array![PREPROCESSED_COLUMN_LOG_SIZE.span(), trace_log_sizes, interaction_log_sizes]
    }

    fn mix_into(self: @CairoClaim, ref channel: Channel) {
        let CairoClaim {
            public_data,
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
            verify_instruction,
            blake_round,
            blake_g,
            blake_round_sigma,
            triple_xor_32,
            verify_bitwise_xor_12,
            add_mod_builtin,
            bitwise_builtin,
            mul_mod_builtin,
            pedersen_builtin,
            poseidon_builtin,
            range_check_96_builtin,
            range_check_128_builtin,
            pedersen_context,
            poseidon_context,
            memory_address_to_id,
            memory_id_to_value,
            range_checks,
            verify_bitwise_xor_4,
            verify_bitwise_xor_7,
            verify_bitwise_xor_8,
            verify_bitwise_xor_9,
        } = self;

        public_data.mix_into(ref channel);

        if let Some(claim) = add {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }

        if let Some(claim) = add_small {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }

        if let Some(claim) = add_ap {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }

        if let Some(claim) = assert_eq {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }

        if let Some(claim) = assert_eq_imm {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }

        if let Some(claim) = assert_eq_double_deref {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }

        if let Some(claim) = blake {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }

        if let Some(claim) = call {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }

        if let Some(claim) = call_rel_imm {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }

        if let Some(claim) = generic {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }

        if let Some(claim) = jnz {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }

        if let Some(claim) = jnz_taken {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }

        if let Some(claim) = jump {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }

        if let Some(claim) = jump_double_deref {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }

        if let Some(claim) = jump_rel {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }

        if let Some(claim) = jump_rel_imm {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }

        if let Some(claim) = mul {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }

        if let Some(claim) = mul_small {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }

        if let Some(claim) = qm31 {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }

        if let Some(claim) = ret {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }

        verify_instruction.mix_into(ref channel);
        if let Some(claim) = blake_round {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }
        if let Some(claim) = blake_g {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }
        if let Some(claim) = blake_round_sigma {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }
        if let Some(claim) = triple_xor_32 {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }
        if let Some(claim) = verify_bitwise_xor_12 {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }
        if let Some(claim) = add_mod_builtin {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }
        if let Some(claim) = bitwise_builtin {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }
        if let Some(claim) = mul_mod_builtin {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }
        if let Some(claim) = pedersen_builtin {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }
        if let Some(claim) = poseidon_builtin {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }
        if let Some(claim) = range_check_96_builtin {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }
        if let Some(claim) = range_check_128_builtin {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }
        pedersen_context.mix_into(ref channel);
        poseidon_context.mix_into(ref channel);
        memory_address_to_id.mix_into(ref channel);
        memory_id_to_value.mix_into(ref channel);
        range_checks.mix_into(ref channel);
        verify_bitwise_xor_4.mix_into(ref channel);
        verify_bitwise_xor_7.mix_into(ref channel);
        verify_bitwise_xor_8.mix_into(ref channel);
        verify_bitwise_xor_9.mix_into(ref channel);
    }

    fn accumulate_relation_uses(self: @CairoClaim, ref relation_uses: RelationUsesDict) {
        let CairoClaim {
            public_data: _,
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
            verify_instruction,
            blake_round,
            blake_g,
            blake_round_sigma: _,
            triple_xor_32,
            verify_bitwise_xor_12: _,
            add_mod_builtin,
            bitwise_builtin,
            mul_mod_builtin,
            pedersen_builtin,
            poseidon_builtin,
            range_check_96_builtin,
            range_check_128_builtin,
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

        if let Some(claim) = add {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = add_small {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = add_ap {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = assert_eq {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = assert_eq_imm {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = assert_eq_double_deref {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = blake {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = call {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = call_rel_imm {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = generic {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = jnz {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = jnz_taken {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = jump {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = jump_double_deref {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = jump_rel {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = jump_rel_imm {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = mul {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = mul_small {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = qm31 {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = ret {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = blake_round {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = blake_g {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = triple_xor_32 {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = add_mod_builtin {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = bitwise_builtin {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = mul_mod_builtin {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = pedersen_builtin {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = poseidon_builtin {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = range_check_96_builtin {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = range_check_128_builtin {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        pedersen_context.accumulate_relation_uses(ref relation_uses);
        poseidon_context.accumulate_relation_uses(ref relation_uses);
        verify_instruction.accumulate_relation_uses(ref relation_uses);
        memory_id_to_value.accumulate_relation_uses(ref relation_uses);
    }
}


#[derive(Drop, Serde)]
pub struct CairoInteractionClaim {
    pub add: Option<add_opcode::InteractionClaim>,
    pub add_small: Option<add_opcode_small::InteractionClaim>,
    pub add_ap: Option<add_ap_opcode::InteractionClaim>,
    pub assert_eq: Option<assert_eq_opcode::InteractionClaim>,
    pub assert_eq_imm: Option<assert_eq_opcode_imm::InteractionClaim>,
    pub assert_eq_double_deref: Option<assert_eq_opcode_double_deref::InteractionClaim>,
    pub blake: Option<blake_compress_opcode::InteractionClaim>,
    pub call: Option<call_opcode_abs::InteractionClaim>,
    pub call_rel_imm: Option<call_opcode_rel_imm::InteractionClaim>,
    pub generic: Option<generic_opcode::InteractionClaim>,
    pub jnz: Option<jnz_opcode_non_taken::InteractionClaim>,
    pub jnz_taken: Option<jnz_opcode_taken::InteractionClaim>,
    pub jump: Option<jump_opcode_abs::InteractionClaim>,
    pub jump_double_deref: Option<jump_opcode_double_deref::InteractionClaim>,
    pub jump_rel: Option<jump_opcode_rel::InteractionClaim>,
    pub jump_rel_imm: Option<jump_opcode_rel_imm::InteractionClaim>,
    pub mul: Option<mul_opcode::InteractionClaim>,
    pub mul_small: Option<mul_opcode_small::InteractionClaim>,
    pub qm31: Option<qm_31_add_mul_opcode::InteractionClaim>,
    pub ret: Option<ret_opcode::InteractionClaim>,
    pub verify_instruction: components::verify_instruction::InteractionClaim,
    pub blake_round: Option<components::blake_round::InteractionClaim>,
    pub blake_g: Option<components::blake_g::InteractionClaim>,
    pub blake_round_sigma: Option<components::blake_round_sigma::InteractionClaim>,
    pub triple_xor_32: Option<components::triple_xor_32::InteractionClaim>,
    pub verify_bitwise_xor_12: Option<components::verify_bitwise_xor_12::InteractionClaim>,
    pub add_mod_builtin: Option<add_mod_builtin::InteractionClaim>,
    pub bitwise_builtin: Option<bitwise_builtin::InteractionClaim>,
    pub mul_mod_builtin: Option<mul_mod_builtin::InteractionClaim>,
    pub pedersen_builtin: Option<pedersen_builtin::InteractionClaim>,
    pub poseidon_builtin: Option<poseidon_builtin::InteractionClaim>,
    pub range_check_96_builtin: Option<range_check_builtin_bits_96::InteractionClaim>,
    pub range_check_128_builtin: Option<range_check_builtin_bits_128::InteractionClaim>,
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
pub impl CairoInteractionClaimImpl of CairoInteractionClaimTrace {
    fn mix_into(self: @CairoInteractionClaim, ref channel: Channel) {
        let CairoInteractionClaim {
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
            verify_instruction,
            blake_round,
            blake_g,
            blake_round_sigma,
            triple_xor_32,
            verify_bitwise_xor_12,
            add_mod_builtin,
            bitwise_builtin,
            mul_mod_builtin,
            pedersen_builtin,
            poseidon_builtin,
            range_check_96_builtin,
            range_check_128_builtin,
            pedersen_context,
            poseidon_context,
            memory_address_to_id,
            memory_id_to_value,
            range_checks,
            verify_bitwise_xor_4,
            verify_bitwise_xor_7,
            verify_bitwise_xor_8,
            verify_bitwise_xor_9,
        } = self;

        if let Some(interaction_claim) = add {
            interaction_claim.mix_into(ref channel);
        }
        if let Some(interaction_claim) = add_small {
            interaction_claim.mix_into(ref channel);
        }
        if let Some(interaction_claim) = add_ap {
            interaction_claim.mix_into(ref channel);
        }
        if let Some(interaction_claim) = assert_eq {
            interaction_claim.mix_into(ref channel);
        }
        if let Some(interaction_claim) = assert_eq_imm {
            interaction_claim.mix_into(ref channel);
        }
        if let Some(interaction_claim) = assert_eq_double_deref {
            interaction_claim.mix_into(ref channel);
        }
        if let Some(interaction_claim) = blake {
            interaction_claim.mix_into(ref channel);
        }
        if let Some(interaction_claim) = call {
            interaction_claim.mix_into(ref channel);
        }
        if let Some(interaction_claim) = call_rel_imm {
            interaction_claim.mix_into(ref channel);
        }
        if let Some(interaction_claim) = generic {
            interaction_claim.mix_into(ref channel);
        }
        if let Some(interaction_claim) = jnz {
            interaction_claim.mix_into(ref channel);
        }
        if let Some(interaction_claim) = jnz_taken {
            interaction_claim.mix_into(ref channel);
        }
        if let Some(interaction_claim) = jump {
            interaction_claim.mix_into(ref channel);
        }
        if let Some(interaction_claim) = jump_double_deref {
            interaction_claim.mix_into(ref channel);
        }
        if let Some(interaction_claim) = jump_rel {
            interaction_claim.mix_into(ref channel);
        }
        if let Some(interaction_claim) = jump_rel_imm {
            interaction_claim.mix_into(ref channel);
        }
        if let Some(interaction_claim) = mul {
            interaction_claim.mix_into(ref channel);
        }
        if let Some(interaction_claim) = mul_small {
            interaction_claim.mix_into(ref channel);
        }
        if let Some(interaction_claim) = qm31 {
            interaction_claim.mix_into(ref channel);
        }
        if let Some(interaction_claim) = ret {
            interaction_claim.mix_into(ref channel);
        }
        verify_instruction.mix_into(ref channel);
        if let Some(claim) = blake_round {
            claim.mix_into(ref channel);
        }
        if let Some(claim) = blake_g {
            claim.mix_into(ref channel);
        }
        if let Some(claim) = blake_round_sigma {
            claim.mix_into(ref channel);
        }
        if let Some(claim) = triple_xor_32 {
            claim.mix_into(ref channel);
        }
        if let Some(claim) = verify_bitwise_xor_12 {
            claim.mix_into(ref channel);
        }
        if let Some(claim) = add_mod_builtin {
            claim.mix_into(ref channel);
        }
        if let Some(claim) = bitwise_builtin {
            claim.mix_into(ref channel);
        }
        if let Some(claim) = mul_mod_builtin {
            claim.mix_into(ref channel);
        }
        if let Some(claim) = pedersen_builtin {
            claim.mix_into(ref channel);
        }
        if let Some(claim) = poseidon_builtin {
            claim.mix_into(ref channel);
        }
        if let Some(claim) = range_check_96_builtin {
            claim.mix_into(ref channel);
        }
        if let Some(claim) = range_check_128_builtin {
            claim.mix_into(ref channel);
        }
        pedersen_context.mix_into(ref channel);
        poseidon_context.mix_into(ref channel);
        memory_address_to_id.mix_into(ref channel);
        memory_id_to_value.mix_into(ref channel);
        range_checks.mix_into(ref channel);
        verify_bitwise_xor_4.mix_into(ref channel);
        verify_bitwise_xor_7.mix_into(ref channel);
        verify_bitwise_xor_8.mix_into(ref channel);
        verify_bitwise_xor_9.mix_into(ref channel);
    }
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
        verify_instruction,
        blake_round,
        blake_g,
        blake_round_sigma,
        triple_xor_32,
        verify_bitwise_xor_12,
        add_mod_builtin,
        bitwise_builtin,
        mul_mod_builtin,
        pedersen_builtin,
        poseidon_builtin,
        range_check_96_builtin,
        range_check_128_builtin,
        pedersen_context,
        poseidon_context,
        memory_address_to_id,
        memory_id_to_value,
        range_checks,
        verify_bitwise_xor_4,
        verify_bitwise_xor_7,
        verify_bitwise_xor_8,
        verify_bitwise_xor_9,
    } = interaction_claim;

    if let Some(interaction_claim) = add {
        assert!(claim.add.is_some(), "add interaction claim is present but add claim isn't");
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = add_small {
        assert!(
            claim.add_small.is_some(),
            "add_small interaction claim is present but add_small claim isn't",
        );
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = add_ap {
        assert!(
            claim.add_ap.is_some(), "add_ap interaction claim is present but add_ap claim isn't",
        );
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = assert_eq {
        assert!(
            claim.assert_eq.is_some(),
            "assert_eq interaction claim is present but assert_eq claim isn't",
        );
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = assert_eq_imm {
        assert!(
            claim.assert_eq_imm.is_some(),
            "assert_eq_imm interaction claim is present but assert_eq_imm claim isn't",
        );
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = assert_eq_double_deref {
        assert!(
            claim.assert_eq_double_deref.is_some(),
            "assert_eq_double_deref interaction claim is present but assert_eq_double_deref claim isn't",
        );
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = blake {
        assert!(claim.blake.is_some(), "blake interaction claim is present but blake claim isn't");
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = call {
        assert!(claim.call.is_some(), "call interaction claim is present but call claim isn't");
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = call_rel_imm {
        assert!(
            claim.call_rel_imm.is_some(),
            "call_rel_imm interaction claim is present but call_rel_imm claim isn't",
        );
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = generic {
        assert!(
            claim.generic.is_some(), "generic interaction claim is present but generic claim isn't",
        );
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = jnz {
        assert!(claim.jnz.is_some(), "jnz interaction claim is present but jnz claim isn't");
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = jnz_taken {
        assert!(
            claim.jnz_taken.is_some(),
            "jnz_taken interaction claim is present but jnz_taken claim isn't",
        );
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = jump {
        assert!(claim.jump.is_some(), "jump interaction claim is present but jump claim isn't");
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = jump_double_deref {
        assert!(
            claim.jump_double_deref.is_some(),
            "jump_double_deref interaction claim is present but jump_double_deref claim isn't",
        );
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = jump_rel {
        assert!(
            claim.jump_rel.is_some(),
            "jump_rel interaction claim is present but jump_rel claim isn't",
        );
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = jump_rel_imm {
        assert!(
            claim.jump_rel_imm.is_some(),
            "jump_rel_imm interaction claim is present but jump_rel_imm claim isn't",
        );
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = mul {
        assert!(claim.mul.is_some(), "mul interaction claim is present but mul claim isn't");
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = mul_small {
        assert!(
            claim.mul_small.is_some(),
            "mul_small interaction claim is present but mul_small claim isn't",
        );
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = qm31 {
        assert!(claim.qm31.is_some(), "qm31 interaction claim is present but qm31 claim isn't");
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = ret {
        assert!(claim.ret.is_some(), "ret interaction claim is present but ret claim isn't");
        sum += *interaction_claim.claimed_sum;
    }

    sum += *verify_instruction.claimed_sum;
    if let Some(interaction_claim) = blake_round {
        assert!(
            claim.blake_round.is_some(),
            "blake_round interaction claim is present but blake_round claim isn't",
        );
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = blake_g {
        assert!(
            claim.blake_g.is_some(), "blake_g interaction claim is present but blake_g claim isn't",
        );
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = blake_round_sigma {
        assert!(
            claim.blake_round_sigma.is_some(),
            "blake_round_sigma interaction claim is present but blake_round_sigma claim isn't",
        );
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = triple_xor_32 {
        assert!(
            claim.triple_xor_32.is_some(),
            "triple_xor_32 interaction claim is present but triple_xor_32 claim isn't",
        );
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = verify_bitwise_xor_12 {
        assert!(
            claim.verify_bitwise_xor_12.is_some(),
            "verify_bitwise_xor_12 interaction claim is present but verify_bitwise_xor_12 claim isn't",
        );
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = add_mod_builtin {
        assert!(
            claim.add_mod_builtin.is_some(),
            "add_mod_builtin interaction claim is present but add_mod_builtin claim isn't",
        );
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = bitwise_builtin {
        assert!(
            claim.bitwise_builtin.is_some(),
            "bitwise_builtin interaction claim is present but bitwise_builtin claim isn't",
        );
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = mul_mod_builtin {
        assert!(
            claim.mul_mod_builtin.is_some(),
            "mul_mod_builtin interaction claim is present but mul_mod_builtin claim isn't",
        );
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = pedersen_builtin {
        assert!(
            claim.pedersen_builtin.is_some(),
            "pedersen_builtin interaction claim is present but pedersen_builtin claim isn't",
        );
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = poseidon_builtin {
        assert!(
            claim.poseidon_builtin.is_some(),
            "poseidon_builtin interaction claim is present but poseidon_builtin claim isn't",
        );
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = range_check_96_builtin {
        assert!(
            claim.range_check_96_builtin.is_some(),
            "range_check_96_builtin interaction claim is present but range_check_96_builtin claim isn't",
        );
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = range_check_128_builtin {
        assert!(
            claim.range_check_128_builtin.is_some(),
            "range_check_128_builtin interaction claim is present but range_check_128_builtin claim isn't",
        );
        sum += *interaction_claim.claimed_sum;
    }
    sum += pedersen_context.sum();
    sum += poseidon_context.sum();
    sum += *memory_address_to_id.claimed_sum;
    sum += memory_id_to_value.sum();
    sum += range_checks.sum();
    sum += *verify_bitwise_xor_4.claimed_sum;
    sum += *verify_bitwise_xor_7.claimed_sum;
    sum += *verify_bitwise_xor_8.claimed_sum;
    sum += *verify_bitwise_xor_9.claimed_sum;
    sum
}

#[derive(Drop)]
#[cfg(not(feature: "poseidon252_verifier"))]
pub struct CairoAir {
    add: Option<add_opcode::Component>,
    add_small: Option<add_opcode_small::Component>,
    add_ap: Option<add_ap_opcode::Component>,
    assert_eq: Option<assert_eq_opcode::Component>,
    assert_eq_imm: Option<assert_eq_opcode_imm::Component>,
    assert_eq_double_deref: Option<assert_eq_opcode_double_deref::Component>,
    blake: Option<blake_compress_opcode::Component>,
    call: Option<call_opcode_abs::Component>,
    call_rel_imm: Option<call_opcode_rel_imm::Component>,
    generic: Option<generic_opcode::Component>,
    jnz: Option<jnz_opcode_non_taken::Component>,
    jnz_taken: Option<jnz_opcode_taken::Component>,
    jump: Option<jump_opcode_abs::Component>,
    jump_double_deref: Option<jump_opcode_double_deref::Component>,
    jump_rel: Option<jump_opcode_rel::Component>,
    jump_rel_imm: Option<jump_opcode_rel_imm::Component>,
    mul: Option<mul_opcode::Component>,
    mul_small: Option<mul_opcode_small::Component>,
    qm31: Option<qm_31_add_mul_opcode::Component>,
    ret: Option<ret_opcode::Component>,
    verify_instruction: components::verify_instruction::Component,
    blake_round: Option<components::blake_round::Component>,
    blake_g: Option<components::blake_g::Component>,
    blake_round_sigma: Option<components::blake_round_sigma::Component>,
    triple_xor_32: Option<components::triple_xor_32::Component>,
    verify_bitwise_xor_12: Option<components::verify_bitwise_xor_12::Component>,
    add_mod_builtin: Option<add_mod_builtin::Component>,
    bitwise_builtin: Option<bitwise_builtin::Component>,
    mul_mod_builtin: Option<mul_mod_builtin::Component>,
    pedersen_builtin: Option<pedersen_builtin::Component>,
    poseidon_builtin: Option<poseidon_builtin::Component>,
    range_check_96_builtin: Option<range_check_builtin_bits_96::Component>,
    range_check_128_builtin: Option<range_check_builtin_bits_128::Component>,
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
    /// The degree bound of the cairo air.
    composition_log_degree_bound: u32,
}

#[generate_trait]
#[cfg(not(feature: "poseidon252_verifier"))]
pub impl CairoAirNewImpl of CairoAirNewTrait {
    fn new(
        cairo_claim: @CairoClaim,
        interaction_elements: @CairoInteractionElements,
        interaction_claim: @CairoInteractionClaim,
        composition_log_degree_bound: u32,
    ) -> CairoAir {
        let CairoClaim {
            add: add_claims,
            add_small: add_small_claims,
            add_ap: add_ap_claims,
            assert_eq: assert_eq_claims,
            assert_eq_imm: assert_eq_imm_claims,
            assert_eq_double_deref: assert_eq_double_deref_claims,
            blake: blake_claims,
            call: call_claims,
            call_rel_imm: call_rel_imm_claims,
            generic: generic_claims,
            jnz: jnz_claims,
            jnz_taken: jnz_taken_claims,
            jump: jump_claims,
            jump_double_deref: jump_double_deref_claims,
            jump_rel: jump_rel_claims,
            jump_rel_imm: jump_rel_imm_claims,
            mul: mul_claims,
            mul_small: mul_small_claims,
            qm31: qm31_claims,
            ret: ret_claims,
            ..,
        } = cairo_claim;

        let CairoInteractionClaim {
            add: add_interaction_claims,
            add_small: add_small_interaction_claims,
            add_ap: add_ap_interaction_claims,
            assert_eq: assert_eq_interaction_claims,
            assert_eq_imm: assert_eq_imm_interaction_claims,
            assert_eq_double_deref: assert_eq_double_deref_interaction_claims,
            blake: blake_interaction_claims,
            call: call_interaction_claims,
            call_rel_imm: call_rel_imm_interaction_claims,
            generic: generic_interaction_claims,
            jnz: jnz_interaction_claims,
            jnz_taken: jnz_taken_interaction_claims,
            jump: jump_interaction_claims,
            jump_double_deref: jump_double_deref_interaction_claims,
            jump_rel: jump_rel_interaction_claims,
            jump_rel_imm: jump_rel_imm_interaction_claims,
            mul: mul_interaction_claims,
            mul_small: mul_small_interaction_claims,
            qm31: qm31_interaction_claims,
            ret: ret_interaction_claims,
            ..,
        } = interaction_claim;

        // Add component
        let add_component = add_claims
            .map(
                |claim| {
                    let interaction_claim = add_interaction_claims.unwrap();
                    add_opcode::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );
        // Add Small component
        let add_small_component = add_small_claims
            .map(
                |claim| {
                    let interaction_claim = add_small_interaction_claims.unwrap();
                    add_opcode_small::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Add AP component
        let add_ap_component = add_ap_claims
            .map(
                |claim| {
                    let interaction_claim = add_ap_interaction_claims.unwrap();
                    add_ap_opcode::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Assert Eq component
        let assert_eq_component = assert_eq_claims
            .map(
                |claim| {
                    let interaction_claim = assert_eq_interaction_claims.unwrap();
                    assert_eq_opcode::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Assert Eq Imm component
        let assert_eq_imm_component = assert_eq_imm_claims
            .map(
                |claim| {
                    let interaction_claim = assert_eq_imm_interaction_claims.unwrap();
                    assert_eq_opcode_imm::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Assert Eq Double Deref component
        let assert_eq_double_deref_component = assert_eq_double_deref_claims
            .map(
                |claim| {
                    let interaction_claim = assert_eq_double_deref_interaction_claims.unwrap();
                    assert_eq_opcode_double_deref::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        let blake_component = blake_claims
            .map(
                |claim| {
                    let interaction_claim = blake_interaction_claims.unwrap();
                    blake_compress_opcode::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Call component
        let call_component = call_claims
            .map(
                |claim| {
                    let interaction_claim = call_interaction_claims.unwrap();
                    call_opcode_abs::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Call Rel_imm component
        let call_rel_imm_component = call_rel_imm_claims
            .map(
                |claim| {
                    let interaction_claim = call_rel_imm_interaction_claims.unwrap();
                    call_opcode_rel_imm::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Generic component
        let generic_component = generic_claims
            .map(
                |claim| {
                    let interaction_claim = generic_interaction_claims.unwrap();
                    generic_opcode::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Jnz component
        let jnz_component = jnz_claims
            .map(
                |claim| {
                    let interaction_claim = jnz_interaction_claims.unwrap();
                    jnz_opcode_non_taken::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Jnz Taken component
        let jnz_taken_component = jnz_taken_claims
            .map(
                |claim| {
                    let interaction_claim = jnz_taken_interaction_claims.unwrap();
                    jnz_opcode_taken::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Jump component
        let jump_component = jump_claims
            .map(
                |claim| {
                    let interaction_claim = jump_interaction_claims.unwrap();
                    jump_opcode_abs::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Jump Double Deref component
        let jump_double_deref_component = jump_double_deref_claims
            .map(
                |claim| {
                    let interaction_claim = jump_double_deref_interaction_claims.unwrap();
                    jump_opcode_double_deref::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Jump Rel component
        let jump_rel_component = jump_rel_claims
            .map(
                |claim| {
                    let interaction_claim = jump_rel_interaction_claims.unwrap();
                    jump_opcode_rel::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Jump Rel Imm component
        let jump_rel_imm_component = jump_rel_imm_claims
            .map(
                |claim| {
                    let interaction_claim = jump_rel_imm_interaction_claims.unwrap();
                    jump_opcode_rel_imm::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Mul component
        let mul_component = mul_claims
            .map(
                |claim| {
                    let interaction_claim = mul_interaction_claims.unwrap();
                    mul_opcode::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Mul Small component
        let mul_small_component = mul_small_claims
            .map(
                |claim| {
                    let interaction_claim = mul_small_interaction_claims.unwrap();
                    mul_opcode_small::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // QM31 component
        let qm31_component = qm31_claims
            .map(
                |claim| {
                    let interaction_claim = qm31_interaction_claims.unwrap();
                    qm_31_add_mul_opcode::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Ret component
        let ret_component = ret_claims
            .map(
                |claim| {
                    let interaction_claim = ret_interaction_claims.unwrap();
                    ret_opcode::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        let blake_round_component = cairo_claim
            .blake_round
            .map(
                |claim| {
                    let interaction_claim = interaction_claim.blake_round.unwrap();
                    blake_round::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        let blake_g_component = cairo_claim
            .blake_g
            .map(
                |claim| {
                    let interaction_claim = interaction_claim.blake_g.unwrap();
                    blake_g::NewComponentImpl::new(@claim, @interaction_claim, interaction_elements)
                },
            );

        let blake_round_sigma_component = cairo_claim
            .blake_round_sigma
            .map(
                |claim| {
                    let interaction_claim = interaction_claim.blake_round_sigma.unwrap();
                    blake_round_sigma::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        let triple_xor_32_component = cairo_claim
            .triple_xor_32
            .map(
                |claim| {
                    let interaction_claim = interaction_claim.triple_xor_32.unwrap();
                    triple_xor_32::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        let verify_bitwise_xor_12_component = cairo_claim
            .verify_bitwise_xor_12
            .map(
                |claim| {
                    let interaction_claim = interaction_claim.verify_bitwise_xor_12.unwrap();
                    verify_bitwise_xor_12::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        let add_mod_builtin_component = cairo_claim
            .add_mod_builtin
            .map(
                |claim| {
                    let interaction_claim = interaction_claim.add_mod_builtin.unwrap();
                    add_mod_builtin::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );
        let bitwise_builtin_component = cairo_claim
            .bitwise_builtin
            .map(
                |claim| {
                    let interaction_claim = interaction_claim.bitwise_builtin.unwrap();
                    bitwise_builtin::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );
        let mul_mod_builtin_component = cairo_claim
            .mul_mod_builtin
            .map(
                |claim| {
                    let interaction_claim = interaction_claim.mul_mod_builtin.unwrap();
                    mul_mod_builtin::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );
        let pedersen_builtin_component = cairo_claim
            .pedersen_builtin
            .map(
                |claim| {
                    let interaction_claim = interaction_claim.pedersen_builtin.unwrap();
                    pedersen_builtin::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );
        let poseidon_builtin_component = cairo_claim
            .poseidon_builtin
            .map(
                |claim| {
                    let interaction_claim = interaction_claim.poseidon_builtin.unwrap();
                    poseidon_builtin::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );
        let range_check_96_builtin_component = cairo_claim
            .range_check_96_builtin
            .map(
                |claim| {
                    let interaction_claim = interaction_claim.range_check_96_builtin.unwrap();
                    range_check_builtin_bits_96::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );
        let range_check_128_builtin_component = cairo_claim
            .range_check_128_builtin
            .map(
                |claim| {
                    let interaction_claim = interaction_claim.range_check_128_builtin.unwrap();
                    range_check_builtin_bits_128::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        let pedersen_context_components = PedersenContextComponentsImpl::new(
            cairo_claim.pedersen_context, interaction_elements, interaction_claim.pedersen_context,
        );

        let poseidon_context_components = PoseidonContextComponentsImpl::new(
            cairo_claim.poseidon_context, interaction_elements, interaction_claim.poseidon_context,
        );

        let verifyinstruction_component = components::verify_instruction::NewComponentImpl::new(
            cairo_claim.verify_instruction,
            interaction_claim.verify_instruction,
            interaction_elements,
        );

        let memory_address_to_id_component =
            components::memory_address_to_id::NewComponentImpl::new(
            cairo_claim.memory_address_to_id,
            interaction_claim.memory_address_to_id,
            interaction_elements,
        );

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
        let mut offset: u32 = LARGE_MEMORY_VALUE_ID_BASE;
        for i in 0..cairo_claim.memory_id_to_value.big_log_sizes.len() {
            let log_size = *cairo_claim.memory_id_to_value.big_log_sizes[i];
            let claimed_sum = *interaction_claim.memory_id_to_value.big_claimed_sums[i];
            memory_id_to_value_components
                .append(
                    components::memory_id_to_big::NewBigComponentImpl::new(
                        log_size, offset, claimed_sum, interaction_elements,
                    ),
                );
            offset = offset + pow2(log_size);
        }
        // Check that IDs in (ID -> Value) do not overflow P.
        assert!(offset <= P_U32);

        let small_memory_id_to_value_component =
            components::memory_id_to_big::NewSmallComponentImpl::new(
            *cairo_claim.memory_id_to_value.small_log_size,
            *interaction_claim.memory_id_to_value.small_claimed_sum,
            interaction_elements,
        );

        let range_checks_components = RangeChecksComponentsImpl::new(
            cairo_claim.range_checks, interaction_elements, interaction_claim.range_checks,
        );

        let verify_bitwise_xor_4_component =
            components::verify_bitwise_xor_4::NewComponentImpl::new(
            cairo_claim.verify_bitwise_xor_4,
            interaction_claim.verify_bitwise_xor_4,
            interaction_elements,
        );

        let verify_bitwise_xor_7_component =
            components::verify_bitwise_xor_7::NewComponentImpl::new(
            cairo_claim.verify_bitwise_xor_7,
            interaction_claim.verify_bitwise_xor_7,
            interaction_elements,
        );

        let verify_bitwise_xor_8_component =
            components::verify_bitwise_xor_8::NewComponentImpl::new(
            cairo_claim.verify_bitwise_xor_8,
            interaction_claim.verify_bitwise_xor_8,
            interaction_elements,
        );

        let verify_bitwise_xor_9_component =
            components::verify_bitwise_xor_9::NewComponentImpl::new(
            cairo_claim.verify_bitwise_xor_9,
            interaction_claim.verify_bitwise_xor_9,
            interaction_elements,
        );

        CairoAir {
            add: add_component,
            add_small: add_small_component,
            add_ap: add_ap_component,
            assert_eq: assert_eq_component,
            assert_eq_imm: assert_eq_imm_component,
            assert_eq_double_deref: assert_eq_double_deref_component,
            blake: blake_component,
            call: call_component,
            call_rel_imm: call_rel_imm_component,
            generic: generic_component,
            jnz: jnz_component,
            jnz_taken: jnz_taken_component,
            jump: jump_component,
            jump_double_deref: jump_double_deref_component,
            jump_rel: jump_rel_component,
            jump_rel_imm: jump_rel_imm_component,
            mul: mul_component,
            mul_small: mul_small_component,
            qm31: qm31_component,
            ret: ret_component,
            verify_instruction: verifyinstruction_component,
            blake_round: blake_round_component,
            blake_g: blake_g_component,
            blake_round_sigma: blake_round_sigma_component,
            triple_xor_32: triple_xor_32_component,
            verify_bitwise_xor_12: verify_bitwise_xor_12_component,
            add_mod_builtin: add_mod_builtin_component,
            bitwise_builtin: bitwise_builtin_component,
            mul_mod_builtin: mul_mod_builtin_component,
            pedersen_builtin: pedersen_builtin_component,
            poseidon_builtin: poseidon_builtin_component,
            range_check_96_builtin: range_check_96_builtin_component,
            range_check_128_builtin: range_check_128_builtin_component,
            pedersen_context: pedersen_context_components,
            poseidon_context: poseidon_context_components,
            memory_address_to_id: memory_address_to_id_component,
            memory_id_to_value: (memory_id_to_value_components, small_memory_id_to_value_component),
            range_checks: range_checks_components,
            verify_bitwise_xor_4: verify_bitwise_xor_4_component,
            verify_bitwise_xor_7: verify_bitwise_xor_7_component,
            verify_bitwise_xor_8: verify_bitwise_xor_8_component,
            verify_bitwise_xor_9: verify_bitwise_xor_9_component,
            composition_log_degree_bound,
        }
    }
}

#[cfg(not(feature: "poseidon252_verifier"))]
pub impl CairoAirImpl of Air<CairoAir> {
    fn composition_log_degree_bound(self: @CairoAir) -> u32 {
        *self.composition_log_degree_bound
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
            preprocessed_mask_values,
        );
        let CairoAir {
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
            verify_instruction,
            blake_round,
            blake_g,
            blake_round_sigma,
            triple_xor_32,
            verify_bitwise_xor_12,
            add_mod_builtin,
            bitwise_builtin,
            mul_mod_builtin,
            pedersen_builtin,
            poseidon_builtin,
            range_check_96_builtin,
            range_check_128_builtin,
            pedersen_context,
            poseidon_context,
            memory_address_to_id,
            memory_id_to_value,
            range_checks,
            verify_bitwise_xor_4,
            verify_bitwise_xor_7,
            verify_bitwise_xor_8,
            verify_bitwise_xor_9,
            composition_log_degree_bound: _,
        } = self;

        if let Some(component) = add {
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
        if let Some(component) = add_small {
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
        if let Some(component) = add_ap {
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
        if let Some(component) = assert_eq {
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
        if let Some(component) = assert_eq_imm {
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
        if let Some(component) = assert_eq_double_deref {
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
        if let Some(component) = blake {
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
        if let Some(component) = call {
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
        if let Some(component) = call_rel_imm {
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
        if let Some(component) = generic {
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
        if let Some(component) = jnz {
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
        if let Some(component) = jnz_taken {
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
        if let Some(component) = jump {
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
        if let Some(component) = jump_double_deref {
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
        if let Some(component) = jump_rel {
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
        if let Some(component) = jump_rel_imm {
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
        if let Some(component) = mul {
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
        if let Some(component) = mul_small {
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
        if let Some(component) = qm31 {
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
        if let Some(component) = ret {
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
        verify_instruction
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        if let Some(component) = blake_round {
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
        if let Some(component) = blake_g {
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
        if let Some(component) = blake_round_sigma {
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
        if let Some(component) = triple_xor_32 {
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
        if let Some(component) = verify_bitwise_xor_12 {
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
        if let Some(component) = add_mod_builtin {
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
        if let Some(component) = bitwise_builtin {
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
        if let Some(component) = mul_mod_builtin {
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
        if let Some(component) = pedersen_builtin {
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
        if let Some(component) = poseidon_builtin {
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
        if let Some(component) = range_check_96_builtin {
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
        if let Some(component) = range_check_128_builtin {
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
        pedersen_context
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        poseidon_context
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        memory_address_to_id
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        let (memory_id_to_value_big, memory_id_to_value_small) = memory_id_to_value;
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

        range_checks
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        verify_bitwise_xor_4
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        verify_bitwise_xor_7
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        verify_bitwise_xor_8
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        verify_bitwise_xor_9
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );

        validate_mask_usage(
            preprocessed_mask_values, trace_mask_values, interaction_trace_mask_values,
        );
        sum
    }
}

#[derive(Drop)]
#[cfg(and(feature: "poseidon252_verifier", not(feature: "poseidon_outputs_packing")))]
pub struct CairoAir {
    add: Option<add_opcode::Component>,
    add_small: Option<add_opcode_small::Component>,
    add_ap: Option<add_ap_opcode::Component>,
    assert_eq: Option<assert_eq_opcode::Component>,
    assert_eq_imm: Option<assert_eq_opcode_imm::Component>,
    assert_eq_double_deref: Option<assert_eq_opcode_double_deref::Component>,
    blake: Option<blake_compress_opcode::Component>,
    call: Option<call_opcode_abs::Component>,
    call_rel_imm: Option<call_opcode_rel_imm::Component>,
    jnz: Option<jnz_opcode_non_taken::Component>,
    jnz_taken: Option<jnz_opcode_taken::Component>,
    jump: Option<jump_opcode_abs::Component>,
    jump_double_deref: Option<jump_opcode_double_deref::Component>,
    jump_rel: Option<jump_opcode_rel::Component>,
    jump_rel_imm: Option<jump_opcode_rel_imm::Component>,
    mul: Option<mul_opcode::Component>,
    mul_small: Option<mul_opcode_small::Component>,
    qm31: Option<qm_31_add_mul_opcode::Component>,
    ret: Option<ret_opcode::Component>,
    verify_instruction: components::verify_instruction::Component,
    blake_round: Option<components::blake_round::Component>,
    blake_g: Option<components::blake_g::Component>,
    blake_round_sigma: Option<components::blake_round_sigma::Component>,
    triple_xor_32: Option<components::triple_xor_32::Component>,
    verify_bitwise_xor_12: Option<components::verify_bitwise_xor_12::Component>,
    bitwise_builtin: Option<bitwise_builtin::Component>,
    range_check_128_builtin: Option<range_check_builtin_bits_128::Component>,
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
    /// The degree bound of the cairo air.
    composition_log_degree_bound: u32,
}

#[generate_trait]
#[cfg(and(feature: "poseidon252_verifier", not(feature: "poseidon_outputs_packing")))]
pub impl CairoAirNewImpl of CairoAirNewTrait {
    fn new(
        cairo_claim: @CairoClaim,
        interaction_elements: @CairoInteractionElements,
        interaction_claim: @CairoInteractionClaim,
        composition_log_degree_bound: u32,
    ) -> CairoAir {
        let CairoClaim {
            add: add_claims,
            add_small: add_small_claims,
            add_ap: add_ap_claims,
            assert_eq: assert_eq_claims,
            assert_eq_imm: assert_eq_imm_claims,
            assert_eq_double_deref: assert_eq_double_deref_claims,
            blake: blake_claims,
            call: call_claims,
            call_rel_imm: call_rel_imm_claims,
            generic: generic_claims,
            jnz: jnz_claims,
            jnz_taken: jnz_taken_claims,
            jump: jump_claims,
            jump_double_deref: jump_double_deref_claims,
            jump_rel: jump_rel_claims,
            jump_rel_imm: jump_rel_imm_claims,
            mul: mul_claims,
            mul_small: mul_small_claims,
            qm31: qm31_claims,
            ret: ret_claims,
            ..,
        } = cairo_claim;
        assert!(generic_claims.is_none(), "The generic opcode is not supported.");

        let CairoInteractionClaim {
            add: add_interaction_claims,
            add_small: add_small_interaction_claims,
            add_ap: add_ap_interaction_claims,
            assert_eq: assert_eq_interaction_claims,
            assert_eq_imm: assert_eq_imm_interaction_claims,
            assert_eq_double_deref: assert_eq_double_deref_interaction_claims,
            blake: blake_interaction_claims,
            call: call_interaction_claims,
            call_rel_imm: call_rel_imm_interaction_claims,
            generic: generic_interaction_claims,
            jnz: jnz_interaction_claims,
            jnz_taken: jnz_taken_interaction_claims,
            jump: jump_interaction_claims,
            jump_double_deref: jump_double_deref_interaction_claims,
            jump_rel: jump_rel_interaction_claims,
            jump_rel_imm: jump_rel_imm_interaction_claims,
            mul: mul_interaction_claims,
            mul_small: mul_small_interaction_claims,
            qm31: qm31_interaction_claims,
            ret: ret_interaction_claims,
            ..,
        } = interaction_claim;
        assert!(generic_interaction_claims.is_none(), "The generic opcode is not supported.");

        // Add components
        let add_components = add_claims
            .map(
                |claim| {
                    let interaction_claim = add_interaction_claims.unwrap();
                    add_opcode::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Add Small components
        let add_small_components = add_small_claims
            .map(
                |claim| {
                    let interaction_claim = add_small_interaction_claims.unwrap();
                    add_opcode_small::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Add AP components
        let add_ap_components = add_ap_claims
            .map(
                |claim| {
                    let interaction_claim = add_ap_interaction_claims.unwrap();
                    add_ap_opcode::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Assert Eq components
        let assert_eq_components = assert_eq_claims
            .map(
                |claim| {
                    let interaction_claim = assert_eq_interaction_claims.unwrap();
                    assert_eq_opcode::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Assert Eq Imm components
        let assert_eq_imm_components = assert_eq_imm_claims
            .map(
                |claim| {
                    let interaction_claim = assert_eq_imm_interaction_claims.unwrap();
                    assert_eq_opcode_imm::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Assert Eq Double Deref components
        let assert_eq_double_deref_components = assert_eq_double_deref_claims
            .map(
                |claim| {
                    let interaction_claim = assert_eq_double_deref_interaction_claims.unwrap();
                    assert_eq_opcode_double_deref::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        let blake_components = blake_claims
            .map(
                |claim| {
                    let interaction_claim = blake_interaction_claims.unwrap();
                    blake_compress_opcode::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Call components
        let call_components = call_claims
            .map(
                |claim| {
                    let interaction_claim = call_interaction_claims.unwrap();
                    call_opcode_abs::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Call Rel_imm components
        let call_rel_imm_components = call_rel_imm_claims
            .map(
                |claim| {
                    let interaction_claim = call_rel_imm_interaction_claims.unwrap();
                    call_opcode_rel_imm::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Jnz components
        let jnz_components = jnz_claims
            .map(
                |claim| {
                    let interaction_claim = jnz_interaction_claims.unwrap();
                    jnz_opcode_non_taken::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Jnz Taken components
        let jnz_taken_components = jnz_taken_claims
            .map(
                |claim| {
                    let interaction_claim = jnz_taken_interaction_claims.unwrap();
                    jnz_opcode_taken::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Jump components
        let jump_components = jump_claims
            .map(
                |claim| {
                    let interaction_claim = jump_interaction_claims.unwrap();
                    jump_opcode_abs::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Jump Double Deref components
        let jump_double_deref_components = jump_double_deref_claims
            .map(
                |claim| {
                    let interaction_claim = jump_double_deref_interaction_claims.unwrap();
                    jump_opcode_double_deref::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Jump Rel components
        let jump_rel_components = jump_rel_claims
            .map(
                |claim| {
                    let interaction_claim = jump_rel_interaction_claims.unwrap();
                    jump_opcode_rel::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Jump Rel Imm components
        let jump_rel_imm_components = jump_rel_imm_claims
            .map(
                |claim| {
                    let interaction_claim = jump_rel_imm_interaction_claims.unwrap();
                    jump_opcode_rel_imm::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Mul components
        let mul_components = mul_claims
            .map(
                |claim| {
                    let interaction_claim = mul_interaction_claims.unwrap();
                    mul_opcode::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Mul Small components
        let mul_small_components = mul_small_claims
            .map(
                |claim| {
                    let interaction_claim = mul_small_interaction_claims.unwrap();
                    mul_opcode_small::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // QM31 components
        let qm31_components = qm31_claims
            .map(
                |claim| {
                    let interaction_claim = qm31_interaction_claims.unwrap();
                    qm_31_add_mul_opcode::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Ret components
        let ret_components = ret_claims
            .map(
                |claim| {
                    let interaction_claim = ret_interaction_claims.unwrap();
                    ret_opcode::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        let blake_round_component = cairo_claim
            .blake_round
            .map(
                |claim| {
                    let interaction_claim = interaction_claim.blake_round.unwrap();
                    blake_round::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        let blake_g_component = cairo_claim
            .blake_g
            .map(
                |claim| {
                    let interaction_claim = interaction_claim.blake_g.unwrap();
                    blake_g::NewComponentImpl::new(@claim, @interaction_claim, interaction_elements)
                },
            );

        let blake_round_sigma_component = cairo_claim
            .blake_round_sigma
            .map(
                |claim| {
                    let interaction_claim = interaction_claim.blake_round_sigma.unwrap();
                    blake_round_sigma::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        let triple_xor_32_component = cairo_claim
            .triple_xor_32
            .map(
                |claim| {
                    let interaction_claim = interaction_claim.triple_xor_32.unwrap();
                    triple_xor_32::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        let verify_bitwise_xor_12_component = cairo_claim
            .verify_bitwise_xor_12
            .map(
                |claim| {
                    let interaction_claim = interaction_claim.verify_bitwise_xor_12.unwrap();
                    verify_bitwise_xor_12::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        let bitwise_builtin_component = cairo_claim
            .bitwise_builtin
            .map(
                |claim| {
                    let interaction_claim = interaction_claim.bitwise_builtin.unwrap();
                    bitwise_builtin::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );
        let range_check_128_builtin_component = cairo_claim
            .range_check_128_builtin
            .map(
                |claim| {
                    let interaction_claim = interaction_claim.range_check_128_builtin.unwrap();
                    range_check_builtin_bits_128::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        let verifyinstruction_component = components::verify_instruction::NewComponentImpl::new(
            cairo_claim.verify_instruction,
            interaction_claim.verify_instruction,
            interaction_elements,
        );

        let memory_address_to_id_component =
            components::memory_address_to_id::NewComponentImpl::new(
            cairo_claim.memory_address_to_id,
            interaction_claim.memory_address_to_id,
            interaction_elements,
        );

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
        let mut offset: u32 = LARGE_MEMORY_VALUE_ID_BASE;
        for i in 0..cairo_claim.memory_id_to_value.big_log_sizes.len() {
            let log_size = *cairo_claim.memory_id_to_value.big_log_sizes[i];
            let claimed_sum = *interaction_claim.memory_id_to_value.big_claimed_sums[i];
            memory_id_to_value_components
                .append(
                    components::memory_id_to_big::NewBigComponentImpl::new(
                        log_size, offset, claimed_sum, interaction_elements,
                    ),
                );
            offset = offset + pow2(log_size);
        }
        // Check that IDs in (ID -> Value) do not overflow P.
        assert!(offset <= P_U32);

        let small_memory_id_to_value_component =
            components::memory_id_to_big::NewSmallComponentImpl::new(
            *cairo_claim.memory_id_to_value.small_log_size,
            *interaction_claim.memory_id_to_value.small_claimed_sum,
            interaction_elements,
        );

        let range_checks_components = RangeChecksComponentsImpl::new(
            cairo_claim.range_checks, interaction_elements, interaction_claim.range_checks,
        );

        let verify_bitwise_xor_4_component =
            components::verify_bitwise_xor_4::NewComponentImpl::new(
            cairo_claim.verify_bitwise_xor_4,
            interaction_claim.verify_bitwise_xor_4,
            interaction_elements,
        );

        let verify_bitwise_xor_7_component =
            components::verify_bitwise_xor_7::NewComponentImpl::new(
            cairo_claim.verify_bitwise_xor_7,
            interaction_claim.verify_bitwise_xor_7,
            interaction_elements,
        );

        let verify_bitwise_xor_8_component =
            components::verify_bitwise_xor_8::NewComponentImpl::new(
            cairo_claim.verify_bitwise_xor_8,
            interaction_claim.verify_bitwise_xor_8,
            interaction_elements,
        );

        let verify_bitwise_xor_9_component =
            components::verify_bitwise_xor_9::NewComponentImpl::new(
            cairo_claim.verify_bitwise_xor_9,
            interaction_claim.verify_bitwise_xor_9,
            interaction_elements,
        );

        CairoAir {
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
            verify_instruction: verifyinstruction_component,
            blake_round: blake_round_component,
            blake_g: blake_g_component,
            blake_round_sigma: blake_round_sigma_component,
            triple_xor_32: triple_xor_32_component,
            verify_bitwise_xor_12: verify_bitwise_xor_12_component,
            bitwise_builtin: bitwise_builtin_component,
            range_check_128_builtin: range_check_128_builtin_component,
            memory_address_to_id: memory_address_to_id_component,
            memory_id_to_value: (memory_id_to_value_components, small_memory_id_to_value_component),
            range_checks: range_checks_components,
            verify_bitwise_xor_4: verify_bitwise_xor_4_component,
            verify_bitwise_xor_7: verify_bitwise_xor_7_component,
            verify_bitwise_xor_8: verify_bitwise_xor_8_component,
            verify_bitwise_xor_9: verify_bitwise_xor_9_component,
            composition_log_degree_bound,
        }
    }
}

#[cfg(and(feature: "poseidon252_verifier", not(feature: "poseidon_outputs_packing")))]
pub impl CairoAirImpl of Air<CairoAir> {
    fn composition_log_degree_bound(self: @CairoAir) -> u32 {
        *self.composition_log_degree_bound
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
            preprocessed_mask_values,
        );

        let CairoAir {
            add,
            add_small,
            add_ap,
            assert_eq,
            assert_eq_imm,
            assert_eq_double_deref,
            blake,
            call,
            call_rel_imm,
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
            verify_instruction,
            blake_round,
            blake_g,
            blake_round_sigma,
            triple_xor_32,
            verify_bitwise_xor_12,
            bitwise_builtin,
            range_check_128_builtin,
            memory_address_to_id,
            memory_id_to_value,
            range_checks,
            verify_bitwise_xor_4,
            verify_bitwise_xor_7,
            verify_bitwise_xor_8,
            verify_bitwise_xor_9,
            composition_log_degree_bound: _,
        } = self;

        if let Some(component) = add {
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
        if let Some(component) = add_small {
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
        if let Some(component) = add_ap {
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
        if let Some(component) = assert_eq {
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
        if let Some(component) = assert_eq_imm {
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
        if let Some(component) = assert_eq_double_deref {
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
        if let Some(component) = blake {
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
        if let Some(component) = call {
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
        if let Some(component) = call_rel_imm {
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
        if let Some(component) = jnz {
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
        if let Some(component) = jnz_taken {
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
        if let Some(component) = jump {
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
        if let Some(component) = jump_double_deref {
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
        if let Some(component) = jump_rel {
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
        if let Some(component) = jump_rel_imm {
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
        if let Some(component) = mul {
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
        if let Some(component) = mul_small {
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
        if let Some(component) = qm31 {
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
        if let Some(component) = ret {
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
        verify_instruction
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        if let Some(component) = blake_round {
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
        if let Some(component) = blake_g {
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
        if let Some(component) = blake_round_sigma {
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
        if let Some(component) = triple_xor_32 {
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
        if let Some(component) = verify_bitwise_xor_12 {
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
        if let Some(component) = bitwise_builtin {
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
        if let Some(component) = range_check_128_builtin {
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
        memory_address_to_id
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        let (memory_id_to_value_big, memory_id_to_value_small) = memory_id_to_value;
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
        range_checks
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        verify_bitwise_xor_4
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        verify_bitwise_xor_7
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        verify_bitwise_xor_8
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        verify_bitwise_xor_9
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );

        validate_mask_usage(
            preprocessed_mask_values, trace_mask_values, interaction_trace_mask_values,
        );
        sum
    }
}


#[derive(Drop)]
#[cfg(and(feature: "poseidon252_verifier", feature: "poseidon_outputs_packing"))]
pub struct CairoAir {
    add: Option<add_opcode::Component>,
    add_small: Option<add_opcode_small::Component>,
    add_ap: Option<add_ap_opcode::Component>,
    assert_eq: Option<assert_eq_opcode::Component>,
    assert_eq_imm: Option<assert_eq_opcode_imm::Component>,
    assert_eq_double_deref: Option<assert_eq_opcode_double_deref::Component>,
    blake: Option<blake_compress_opcode::Component>,
    call: Option<call_opcode_abs::Component>,
    call_rel_imm: Option<call_opcode_rel_imm::Component>,
    jnz: Option<jnz_opcode_non_taken::Component>,
    jnz_taken: Option<jnz_opcode_taken::Component>,
    jump: Option<jump_opcode_abs::Component>,
    jump_double_deref: Option<jump_opcode_double_deref::Component>,
    jump_rel: Option<jump_opcode_rel::Component>,
    jump_rel_imm: Option<jump_opcode_rel_imm::Component>,
    mul: Option<mul_opcode::Component>,
    mul_small: Option<mul_opcode_small::Component>,
    qm31: Option<qm_31_add_mul_opcode::Component>,
    ret: Option<ret_opcode::Component>,
    verify_instruction: components::verify_instruction::Component,
    blake_round: Option<components::blake_round::Component>,
    blake_g: Option<components::blake_g::Component>,
    blake_round_sigma: Option<components::blake_round_sigma::Component>,
    triple_xor_32: Option<components::triple_xor_32::Component>,
    verify_bitwise_xor_12: Option<components::verify_bitwise_xor_12::Component>,
    bitwise_builtin: Option<bitwise_builtin::Component>,
    poseidon_builtin: Option<poseidon_builtin::Component>,
    range_check_128_builtin: Option<range_check_builtin_bits_128::Component>,
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
    /// The degree bound of the cairo air.
    composition_log_degree_bound: u32,
}

#[generate_trait]
#[cfg(and(feature: "poseidon252_verifier", feature: "poseidon_outputs_packing"))]
pub impl CairoAirNewImpl of CairoAirNewTrait {
    fn new(
        cairo_claim: @CairoClaim,
        interaction_elements: @CairoInteractionElements,
        interaction_claim: @CairoInteractionClaim,
        composition_log_degree_bound: u32,
    ) -> CairoAir {
        let CairoClaim {
            add: add_claims,
            add_small: add_small_claims,
            add_ap: add_ap_claims,
            assert_eq: assert_eq_claims,
            assert_eq_imm: assert_eq_imm_claims,
            assert_eq_double_deref: assert_eq_double_deref_claims,
            blake: blake_claims,
            call: call_claims,
            call_rel_imm: call_rel_imm_claims,
            generic: generic_claims,
            jnz: jnz_claims,
            jnz_taken: jnz_taken_claims,
            jump: jump_claims,
            jump_double_deref: jump_double_deref_claims,
            jump_rel: jump_rel_claims,
            jump_rel_imm: jump_rel_imm_claims,
            mul: mul_claims,
            mul_small: mul_small_claims,
            qm31: qm31_claims,
            ret: ret_claims,
            ..,
        } = cairo_claim;
        assert!(generic_claims.is_none(), "The generic opcode is not supported.");

        let CairoInteractionClaim {
            add: add_interaction_claims,
            add_small: add_small_interaction_claims,
            add_ap: add_ap_interaction_claims,
            assert_eq: assert_eq_interaction_claims,
            assert_eq_imm: assert_eq_imm_interaction_claims,
            assert_eq_double_deref: assert_eq_double_deref_interaction_claims,
            blake: blake_interaction_claims,
            call: call_interaction_claims,
            call_rel_imm: call_rel_imm_interaction_claims,
            generic: generic_interaction_claims,
            jnz: jnz_interaction_claims,
            jnz_taken: jnz_taken_interaction_claims,
            jump: jump_interaction_claims,
            jump_double_deref: jump_double_deref_interaction_claims,
            jump_rel: jump_rel_interaction_claims,
            jump_rel_imm: jump_rel_imm_interaction_claims,
            mul: mul_interaction_claims,
            mul_small: mul_small_interaction_claims,
            qm31: qm31_interaction_claims,
            ret: ret_interaction_claims,
            ..,
        } = interaction_claim;
        assert!(generic_interaction_claims.is_none(), "The generic opcode is not supported.");

        // Add components
        let add_components = add_claims
            .map(
                |claim| {
                    let interaction_claim = add_interaction_claims.unwrap();
                    add_opcode::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Add Small components
        let add_small_components = add_small_claims
            .map(
                |claim| {
                    let interaction_claim = add_small_interaction_claims.unwrap();
                    add_opcode_small::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Add AP components
        let add_ap_components = add_ap_claims
            .map(
                |claim| {
                    let interaction_claim = add_ap_interaction_claims.unwrap();
                    add_ap_opcode::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Assert Eq components
        let assert_eq_components = assert_eq_claims
            .map(
                |claim| {
                    let interaction_claim = assert_eq_interaction_claims.unwrap();
                    assert_eq_opcode::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Assert Eq Imm components
        let assert_eq_imm_components = assert_eq_imm_claims
            .map(
                |claim| {
                    let interaction_claim = assert_eq_imm_interaction_claims.unwrap();
                    assert_eq_opcode_imm::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Assert Eq Double Deref components
        let assert_eq_double_deref_components = assert_eq_double_deref_claims
            .map(
                |claim| {
                    let interaction_claim = assert_eq_double_deref_interaction_claims.unwrap();
                    assert_eq_opcode_double_deref::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        let blake_components = blake_claims
            .map(
                |claim| {
                    let interaction_claim = blake_interaction_claims.unwrap();
                    blake_compress_opcode::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Call components
        let call_components = call_claims
            .map(
                |claim| {
                    let interaction_claim = call_interaction_claims.unwrap();
                    call_opcode_abs::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Call Rel_imm components
        let call_rel_imm_components = call_rel_imm_claims
            .map(
                |claim| {
                    let interaction_claim = call_rel_imm_interaction_claims.unwrap();
                    call_opcode_rel_imm::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Jnz components
        let jnz_components = jnz_claims
            .map(
                |claim| {
                    let interaction_claim = jnz_interaction_claims.unwrap();
                    jnz_opcode_non_taken::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Jnz Taken components
        let jnz_taken_components = jnz_taken_claims
            .map(
                |claim| {
                    let interaction_claim = jnz_taken_interaction_claims.unwrap();
                    jnz_opcode_taken::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Jump components
        let jump_components = jump_claims
            .map(
                |claim| {
                    let interaction_claim = jump_interaction_claims.unwrap();
                    jump_opcode_abs::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Jump Double Deref components
        let jump_double_deref_components = jump_double_deref_claims
            .map(
                |claim| {
                    let interaction_claim = jump_double_deref_interaction_claims.unwrap();
                    jump_opcode_double_deref::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Jump Rel components
        let jump_rel_components = jump_rel_claims
            .map(
                |claim| {
                    let interaction_claim = jump_rel_interaction_claims.unwrap();
                    jump_opcode_rel::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Jump Rel Imm components
        let jump_rel_imm_components = jump_rel_imm_claims
            .map(
                |claim| {
                    let interaction_claim = jump_rel_imm_interaction_claims.unwrap();
                    jump_opcode_rel_imm::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Mul components
        let mul_components = mul_claims
            .map(
                |claim| {
                    let interaction_claim = mul_interaction_claims.unwrap();
                    mul_opcode::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Mul Small components
        let mul_small_components = mul_small_claims
            .map(
                |claim| {
                    let interaction_claim = mul_small_interaction_claims.unwrap();
                    mul_opcode_small::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // QM31 components
        let qm31_components = qm31_claims
            .map(
                |claim| {
                    let interaction_claim = qm31_interaction_claims.unwrap();
                    qm_31_add_mul_opcode::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Ret components
        let ret_components = ret_claims
            .map(
                |claim| {
                    let interaction_claim = ret_interaction_claims.unwrap();
                    ret_opcode::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        let blake_round_component = cairo_claim
            .blake_round
            .map(
                |claim| {
                    let interaction_claim = interaction_claim.blake_round.unwrap();
                    blake_round::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        let blake_g_component = cairo_claim
            .blake_g
            .map(
                |claim| {
                    let interaction_claim = interaction_claim.blake_g.unwrap();
                    blake_g::NewComponentImpl::new(@claim, @interaction_claim, interaction_elements)
                },
            );

        let blake_round_sigma_component = cairo_claim
            .blake_round_sigma
            .map(
                |claim| {
                    let interaction_claim = interaction_claim.blake_round_sigma.unwrap();
                    blake_round_sigma::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        let triple_xor_32_component = cairo_claim
            .triple_xor_32
            .map(
                |claim| {
                    let interaction_claim = interaction_claim.triple_xor_32.unwrap();
                    triple_xor_32::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        let verify_bitwise_xor_12_component = cairo_claim
            .verify_bitwise_xor_12
            .map(
                |claim| {
                    let interaction_claim = interaction_claim.verify_bitwise_xor_12.unwrap();
                    verify_bitwise_xor_12::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        let bitwise_builtin_component = cairo_claim
            .bitwise_builtin
            .map(
                |claim| {
                    let interaction_claim = interaction_claim.bitwise_builtin.unwrap();
                    bitwise_builtin::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );
        let poseidon_builtin_component = cairo_claim
            .poseidon_builtin
            .map(
                |claim| {
                    let interaction_claim = interaction_claim.poseidon_builtin.unwrap();
                    poseidon_builtin::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        let range_check_128_builtin_component = cairo_claim
            .range_check_128_builtin
            .map(
                |claim| {
                    let interaction_claim = interaction_claim.range_check_128_builtin.unwrap();
                    range_check_builtin_bits_128::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        let poseidon_context_components = PoseidonContextComponentsImpl::new(
            cairo_claim.poseidon_context, interaction_elements, interaction_claim.poseidon_context,
        );

        let verifyinstruction_component = components::verify_instruction::NewComponentImpl::new(
            cairo_claim.verify_instruction,
            interaction_claim.verify_instruction,
            interaction_elements,
        );

        let memory_address_to_id_component =
            components::memory_address_to_id::NewComponentImpl::new(
            cairo_claim.memory_address_to_id,
            interaction_claim.memory_address_to_id,
            interaction_elements,
        );

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
        let mut offset: u32 = LARGE_MEMORY_VALUE_ID_BASE;
        for i in 0..cairo_claim.memory_id_to_value.big_log_sizes.len() {
            let log_size = *cairo_claim.memory_id_to_value.big_log_sizes[i];
            let claimed_sum = *interaction_claim.memory_id_to_value.big_claimed_sums[i];
            memory_id_to_value_components
                .append(
                    components::memory_id_to_big::NewBigComponentImpl::new(
                        log_size, offset, claimed_sum, interaction_elements,
                    ),
                );
            offset = offset + pow2(log_size);
        }
        // Check that IDs in (ID -> Value) do not overflow P.
        assert!(offset <= P_U32);

        let small_memory_id_to_value_component =
            components::memory_id_to_big::NewSmallComponentImpl::new(
            *cairo_claim.memory_id_to_value.small_log_size,
            *interaction_claim.memory_id_to_value.small_claimed_sum,
            interaction_elements,
        );

        let range_checks_components = RangeChecksComponentsImpl::new(
            cairo_claim.range_checks, interaction_elements, interaction_claim.range_checks,
        );

        let verify_bitwise_xor_4_component =
            components::verify_bitwise_xor_4::NewComponentImpl::new(
            cairo_claim.verify_bitwise_xor_4,
            interaction_claim.verify_bitwise_xor_4,
            interaction_elements,
        );

        let verify_bitwise_xor_7_component =
            components::verify_bitwise_xor_7::NewComponentImpl::new(
            cairo_claim.verify_bitwise_xor_7,
            interaction_claim.verify_bitwise_xor_7,
            interaction_elements,
        );

        let verify_bitwise_xor_8_component =
            components::verify_bitwise_xor_8::NewComponentImpl::new(
            cairo_claim.verify_bitwise_xor_8,
            interaction_claim.verify_bitwise_xor_8,
            interaction_elements,
        );

        let verify_bitwise_xor_9_component =
            components::verify_bitwise_xor_9::NewComponentImpl::new(
            cairo_claim.verify_bitwise_xor_9,
            interaction_claim.verify_bitwise_xor_9,
            interaction_elements,
        );

        CairoAir {
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
            verify_instruction: verifyinstruction_component,
            blake_round: blake_round_component,
            blake_g: blake_g_component,
            blake_round_sigma: blake_round_sigma_component,
            triple_xor_32: triple_xor_32_component,
            verify_bitwise_xor_12: verify_bitwise_xor_12_component,
            bitwise_builtin: bitwise_builtin_component,
            poseidon_builtin: poseidon_builtin_component,
            range_check_128_builtin: range_check_128_builtin_component,
            poseidon_context: poseidon_context_components,
            memory_address_to_id: memory_address_to_id_component,
            memory_id_to_value: (memory_id_to_value_components, small_memory_id_to_value_component),
            range_checks: range_checks_components,
            verify_bitwise_xor_4: verify_bitwise_xor_4_component,
            verify_bitwise_xor_7: verify_bitwise_xor_7_component,
            verify_bitwise_xor_8: verify_bitwise_xor_8_component,
            verify_bitwise_xor_9: verify_bitwise_xor_9_component,
            composition_log_degree_bound,
        }
    }
}

#[cfg(and(feature: "poseidon252_verifier", feature: "poseidon_outputs_packing"))]
pub impl CairoAirImpl of Air<CairoAir> {
    fn composition_log_degree_bound(self: @CairoAir) -> u32 {
        *self.composition_log_degree_bound
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
            preprocessed_mask_values,
        );

        let CairoAir {
            add,
            add_small,
            add_ap,
            assert_eq,
            assert_eq_imm,
            assert_eq_double_deref,
            blake,
            call,
            call_rel_imm,
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
            verify_instruction,
            blake_round,
            blake_g,
            blake_round_sigma,
            triple_xor_32,
            verify_bitwise_xor_12,
            bitwise_builtin,
            poseidon_builtin,
            range_check_128_builtin,
            poseidon_context,
            memory_address_to_id,
            memory_id_to_value,
            range_checks,
            verify_bitwise_xor_4,
            verify_bitwise_xor_7,
            verify_bitwise_xor_8,
            verify_bitwise_xor_9,
            composition_log_degree_bound: _,
        } = self;

        if let Some(component) = add {
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
        if let Some(component) = add_small {
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
        if let Some(component) = add_ap {
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
        if let Some(component) = assert_eq {
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
        if let Some(component) = assert_eq_imm {
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
        if let Some(component) = assert_eq_double_deref {
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
        if let Some(component) = blake {
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
        if let Some(component) = call {
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
        if let Some(component) = call_rel_imm {
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
        if let Some(component) = jnz {
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
        if let Some(component) = jnz_taken {
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
        if let Some(component) = jump {
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
        if let Some(component) = jump_double_deref {
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
        if let Some(component) = jump_rel {
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
        if let Some(component) = jump_rel_imm {
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
        if let Some(component) = mul {
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
        if let Some(component) = mul_small {
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
        if let Some(component) = qm31 {
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
        if let Some(component) = ret {
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
        verify_instruction
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        if let Some(component) = blake_round {
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
        if let Some(component) = blake_g {
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
        if let Some(component) = blake_round_sigma {
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
        if let Some(component) = triple_xor_32 {
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
        if let Some(component) = verify_bitwise_xor_12 {
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
        if let Some(component) = bitwise_builtin {
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
        if let Some(component) = poseidon_builtin {
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
        if let Some(component) = range_check_128_builtin {
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
        poseidon_context
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        memory_address_to_id
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        let (memory_id_to_value_big, memory_id_to_value_small) = memory_id_to_value;
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
        range_checks
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        verify_bitwise_xor_4
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        verify_bitwise_xor_7
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        verify_bitwise_xor_8
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        verify_bitwise_xor_9
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );

        validate_mask_usage(
            preprocessed_mask_values, trace_mask_values, interaction_trace_mask_values,
        );
        sum
    }
}
