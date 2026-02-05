// This file was created by the AIR team.

use stwo_cairo_air::claim::ClaimTrait;
use stwo_cairo_air::{PublicData, PublicDataImpl, RelationUsesDict};
use stwo_constraint_framework::CommonLookupElements;
use stwo_verifier_core::TreeArray;
use stwo_verifier_core::channel::Channel;
use stwo_verifier_core::fields::qm31::QM31;
use stwo_verifier_core::utils::OptionImpl;
use crate::cairo_air::override_preprocessed_trace_log_sizes;
use crate::components::add_ap_opcode::InteractionClaimImpl as AddApOpcodeInteractionClaimImpl;
use crate::components::add_mod_builtin::InteractionClaimImpl as AddModBuiltinInteractionClaimImpl;
use crate::components::add_opcode::InteractionClaimImpl as AddOpcodeInteractionClaimImpl;
use crate::components::add_opcode_small::InteractionClaimImpl as AddOpcodeSmallInteractionClaimImpl;
use crate::components::assert_eq_opcode::InteractionClaimImpl as AssertEqOpcodeInteractionClaimImpl;
use crate::components::assert_eq_opcode_double_deref::InteractionClaimImpl as AssertEqOpcodeDoubleDerefInteractionClaimImpl;
use crate::components::assert_eq_opcode_imm::InteractionClaimImpl as AssertEqOpcodeImmInteractionClaimImpl;
use crate::components::bitwise_builtin::InteractionClaimImpl as BitwiseBuiltinInteractionClaimImpl;
use crate::components::blake_compress_opcode::InteractionClaimImpl as BlakeCompressOpcodeInteractionClaimImpl;
use crate::components::blake_g::InteractionClaimImpl as BlakeGInteractionClaimImpl;
use crate::components::blake_round::InteractionClaimImpl as BlakeRoundInteractionClaimImpl;
use crate::components::blake_round_sigma::InteractionClaimImpl as BlakeRoundSigmaInteractionClaimImpl;
use crate::components::call_opcode_abs::InteractionClaimImpl as CallOpcodeAbsInteractionClaimImpl;
use crate::components::call_opcode_rel_imm::InteractionClaimImpl as CallOpcodeRelImmInteractionClaimImpl;
use crate::components::cube_252::InteractionClaimImpl as Cube252InteractionClaimImpl;
use crate::components::generic_opcode::InteractionClaimImpl as GenericOpcodeInteractionClaimImpl;
use crate::components::jnz_opcode_non_taken::InteractionClaimImpl as JnzOpcodeNonTakenInteractionClaimImpl;
use crate::components::jnz_opcode_taken::InteractionClaimImpl as JnzOpcodeTakenInteractionClaimImpl;
use crate::components::jump_opcode_abs::InteractionClaimImpl as JumpOpcodeAbsInteractionClaimImpl;
use crate::components::jump_opcode_double_deref::InteractionClaimImpl as JumpOpcodeDoubleDerefInteractionClaimImpl;
use crate::components::jump_opcode_rel::InteractionClaimImpl as JumpOpcodeRelInteractionClaimImpl;
use crate::components::jump_opcode_rel_imm::InteractionClaimImpl as JumpOpcodeRelImmInteractionClaimImpl;
use crate::components::memory_address_to_id::InteractionClaimImpl as MemoryAddressToIdInteractionClaimImpl;
use crate::components::memory_id_to_big::InteractionClaimImpl as MemoryIdToBigInteractionClaimImpl;
use crate::components::mul_mod_builtin::InteractionClaimImpl as MulModBuiltinInteractionClaimImpl;
use crate::components::mul_opcode::InteractionClaimImpl as MulOpcodeInteractionClaimImpl;
use crate::components::mul_opcode_small::InteractionClaimImpl as MulOpcodeSmallInteractionClaimImpl;
use crate::components::partial_ec_mul_window_bits_18::InteractionClaimImpl as PartialEcMulWindowBits18InteractionClaimImpl;
use crate::components::pedersen_aggregator_window_bits_18::InteractionClaimImpl as PedersenAggregatorWindowBits18InteractionClaimImpl;
use crate::components::pedersen_builtin::InteractionClaimImpl as PedersenBuiltinInteractionClaimImpl;
use crate::components::pedersen_points_table_window_bits_18::InteractionClaimImpl as PedersenPointsTableWindowBits18InteractionClaimImpl;
use crate::components::poseidon_3_partial_rounds_chain::InteractionClaimImpl as Poseidon3PartialRoundsChainInteractionClaimImpl;
use crate::components::poseidon_aggregator::InteractionClaimImpl as PoseidonAggregatorInteractionClaimImpl;
use crate::components::poseidon_builtin::InteractionClaimImpl as PoseidonBuiltinInteractionClaimImpl;
use crate::components::poseidon_full_round_chain::InteractionClaimImpl as PoseidonFullRoundChainInteractionClaimImpl;
use crate::components::poseidon_round_keys::InteractionClaimImpl as PoseidonRoundKeysInteractionClaimImpl;
use crate::components::qm_31_add_mul_opcode::InteractionClaimImpl as Qm31AddMulOpcodeInteractionClaimImpl;
use crate::components::range_check96_builtin::InteractionClaimImpl as RangeCheck96BuiltinInteractionClaimImpl;
use crate::components::range_check_11::InteractionClaimImpl as RangeCheck11InteractionClaimImpl;
use crate::components::range_check_12::InteractionClaimImpl as RangeCheck12InteractionClaimImpl;
use crate::components::range_check_18::InteractionClaimImpl as RangeCheck18InteractionClaimImpl;
use crate::components::range_check_20::InteractionClaimImpl as RangeCheck20InteractionClaimImpl;
use crate::components::range_check_252_width_27::InteractionClaimImpl as RangeCheck252Width27InteractionClaimImpl;
use crate::components::range_check_3_3_3_3_3::InteractionClaimImpl as RangeCheck33333InteractionClaimImpl;
use crate::components::range_check_3_6_6_3::InteractionClaimImpl as RangeCheck3663InteractionClaimImpl;
use crate::components::range_check_4_3::InteractionClaimImpl as RangeCheck43InteractionClaimImpl;
use crate::components::range_check_4_4::InteractionClaimImpl as RangeCheck44InteractionClaimImpl;
use crate::components::range_check_4_4_4_4::InteractionClaimImpl as RangeCheck4444InteractionClaimImpl;
use crate::components::range_check_6::InteractionClaimImpl as RangeCheck6InteractionClaimImpl;
use crate::components::range_check_7_2_5::InteractionClaimImpl as RangeCheck725InteractionClaimImpl;
use crate::components::range_check_8::InteractionClaimImpl as RangeCheck8InteractionClaimImpl;
use crate::components::range_check_9_9::InteractionClaimImpl as RangeCheck99InteractionClaimImpl;
use crate::components::range_check_builtin::InteractionClaimImpl as RangeCheckBuiltinInteractionClaimImpl;
use crate::components::ret_opcode::InteractionClaimImpl as RetOpcodeInteractionClaimImpl;
use crate::components::triple_xor_32::InteractionClaimImpl as TripleXor32InteractionClaimImpl;
use crate::components::verify_bitwise_xor_12::InteractionClaimImpl as VerifyBitwiseXor12InteractionClaimImpl;
use crate::components::verify_bitwise_xor_4::InteractionClaimImpl as VerifyBitwiseXor4InteractionClaimImpl;
use crate::components::verify_bitwise_xor_7::InteractionClaimImpl as VerifyBitwiseXor7InteractionClaimImpl;
use crate::components::verify_bitwise_xor_8::InteractionClaimImpl as VerifyBitwiseXor8InteractionClaimImpl;
use crate::components::verify_bitwise_xor_9::InteractionClaimImpl as VerifyBitwiseXor9InteractionClaimImpl;
use crate::components::verify_instruction::InteractionClaimImpl as VerifyInstructionInteractionClaimImpl;
use crate::utils::tree_array_concat_cols;
use crate::{ChannelTrait, PublicDataTrait, components};
use super::claim::{FlatClaim, FlatClaimTrait, flatten_interaction_claim};

#[derive(Drop, Serde)]
pub struct CairoClaim {
    pub public_data: PublicData,
    pub add_opcode: Option<components::add_opcode::Claim>,
    pub add_opcode_small: Option<components::add_opcode_small::Claim>,
    pub add_ap_opcode: Option<components::add_ap_opcode::Claim>,
    pub assert_eq_opcode: Option<components::assert_eq_opcode::Claim>,
    pub assert_eq_opcode_imm: Option<components::assert_eq_opcode_imm::Claim>,
    pub assert_eq_opcode_double_deref: Option<components::assert_eq_opcode_double_deref::Claim>,
    pub blake_compress_opcode: Option<components::blake_compress_opcode::Claim>,
    pub call_opcode_abs: Option<components::call_opcode_abs::Claim>,
    pub call_opcode_rel_imm: Option<components::call_opcode_rel_imm::Claim>,
    pub generic_opcode: Option<components::generic_opcode::Claim>,
    pub jnz_opcode_non_taken: Option<components::jnz_opcode_non_taken::Claim>,
    pub jnz_opcode_taken: Option<components::jnz_opcode_taken::Claim>,
    pub jump_opcode_abs: Option<components::jump_opcode_abs::Claim>,
    pub jump_opcode_double_deref: Option<components::jump_opcode_double_deref::Claim>,
    pub jump_opcode_rel: Option<components::jump_opcode_rel::Claim>,
    pub jump_opcode_rel_imm: Option<components::jump_opcode_rel_imm::Claim>,
    pub mul_opcode: Option<components::mul_opcode::Claim>,
    pub mul_opcode_small: Option<components::mul_opcode_small::Claim>,
    pub qm_31_add_mul_opcode: Option<components::qm_31_add_mul_opcode::Claim>,
    pub ret_opcode: Option<components::ret_opcode::Claim>,
    pub verify_instruction: Option<components::verify_instruction::Claim>,
    pub blake_round: Option<components::blake_round::Claim>,
    pub blake_g: Option<components::blake_g::Claim>,
    pub blake_round_sigma: Option<components::blake_round_sigma::Claim>,
    pub triple_xor_32: Option<components::triple_xor_32::Claim>,
    pub verify_bitwise_xor_12: Option<components::verify_bitwise_xor_12::Claim>,
    pub add_mod_builtin: Option<components::add_mod_builtin::Claim>,
    pub bitwise_builtin: Option<components::bitwise_builtin::Claim>,
    pub mul_mod_builtin: Option<components::mul_mod_builtin::Claim>,
    pub pedersen_builtin: Option<components::pedersen_builtin::Claim>,
    pub poseidon_builtin: Option<components::poseidon_builtin::Claim>,
    pub range_check96_builtin: Option<components::range_check96_builtin::Claim>,
    pub range_check_builtin: Option<components::range_check_builtin::Claim>,
    pub pedersen_aggregator_window_bits_18: Option<
        components::pedersen_aggregator_window_bits_18::Claim,
    >,
    pub partial_ec_mul_window_bits_18: Option<components::partial_ec_mul_window_bits_18::Claim>,
    pub pedersen_points_table_window_bits_18: Option<
        components::pedersen_points_table_window_bits_18::Claim,
    >,
    pub poseidon_aggregator: Option<components::poseidon_aggregator::Claim>,
    pub poseidon_3_partial_rounds_chain: Option<components::poseidon_3_partial_rounds_chain::Claim>,
    pub poseidon_full_round_chain: Option<components::poseidon_full_round_chain::Claim>,
    pub cube_252: Option<components::cube_252::Claim>,
    pub poseidon_round_keys: Option<components::poseidon_round_keys::Claim>,
    pub range_check_252_width_27: Option<components::range_check_252_width_27::Claim>,
    pub memory_address_to_id: Option<components::memory_address_to_id::Claim>,
    pub memory_id_to_big: Option<components::memory_id_to_big::Claim>,
    pub range_check_6: Option<components::range_check_6::Claim>,
    pub range_check_8: Option<components::range_check_8::Claim>,
    pub range_check_11: Option<components::range_check_11::Claim>,
    pub range_check_12: Option<components::range_check_12::Claim>,
    pub range_check_18: Option<components::range_check_18::Claim>,
    pub range_check_20: Option<components::range_check_20::Claim>,
    pub range_check_4_3: Option<components::range_check_4_3::Claim>,
    pub range_check_4_4: Option<components::range_check_4_4::Claim>,
    pub range_check_9_9: Option<components::range_check_9_9::Claim>,
    pub range_check_7_2_5: Option<components::range_check_7_2_5::Claim>,
    pub range_check_3_6_6_3: Option<components::range_check_3_6_6_3::Claim>,
    pub range_check_4_4_4_4: Option<components::range_check_4_4_4_4::Claim>,
    pub range_check_3_3_3_3_3: Option<components::range_check_3_3_3_3_3::Claim>,
    pub verify_bitwise_xor_4: Option<components::verify_bitwise_xor_4::Claim>,
    pub verify_bitwise_xor_7: Option<components::verify_bitwise_xor_7::Claim>,
    pub verify_bitwise_xor_8: Option<components::verify_bitwise_xor_8::Claim>,
    pub verify_bitwise_xor_9: Option<components::verify_bitwise_xor_9::Claim>,
}

pub impl CairoClaimImpl of ClaimTrait<CairoClaim> {
    fn log_sizes(self: @CairoClaim) -> TreeArray<Span<u32>> {
        let mut log_sizes_list = array![];
        if let Some(claim) = self.add_opcode {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.add_opcode_small {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.add_ap_opcode {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.assert_eq_opcode {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.assert_eq_opcode_imm {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.assert_eq_opcode_double_deref {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.blake_compress_opcode {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.call_opcode_abs {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.call_opcode_rel_imm {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.generic_opcode {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.jnz_opcode_non_taken {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.jnz_opcode_taken {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.jump_opcode_abs {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.jump_opcode_double_deref {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.jump_opcode_rel {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.jump_opcode_rel_imm {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.mul_opcode {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.mul_opcode_small {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.qm_31_add_mul_opcode {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.ret_opcode {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.verify_instruction {
            log_sizes_list.append(claim.log_sizes());
        }
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
        if let Some(claim) = self.range_check96_builtin {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.range_check_builtin {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.pedersen_aggregator_window_bits_18 {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.partial_ec_mul_window_bits_18 {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.pedersen_points_table_window_bits_18 {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.poseidon_aggregator {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.poseidon_3_partial_rounds_chain {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.poseidon_full_round_chain {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.cube_252 {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.poseidon_round_keys {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.range_check_252_width_27 {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.memory_address_to_id {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.memory_id_to_big {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.range_check_6 {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.range_check_8 {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.range_check_11 {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.range_check_12 {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.range_check_18 {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.range_check_20 {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.range_check_4_3 {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.range_check_4_4 {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.range_check_9_9 {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.range_check_7_2_5 {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.range_check_3_6_6_3 {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.range_check_4_4_4_4 {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.range_check_3_3_3_3_3 {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.verify_bitwise_xor_4 {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.verify_bitwise_xor_7 {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.verify_bitwise_xor_8 {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.verify_bitwise_xor_9 {
            log_sizes_list.append(claim.log_sizes());
        }
        let aggregated_log_sizes = tree_array_concat_cols(log_sizes_list);
        override_preprocessed_trace_log_sizes(aggregated_log_sizes)
    }

    fn mix_into(self: @CairoClaim, ref channel: Channel) {
        let claim: FlatClaim = FlatClaimTrait::from_cairo_claim(self);
        claim.mix_into(ref channel);
    }

    fn accumulate_relation_uses(self: @CairoClaim, ref relation_uses: RelationUsesDict) {
        if let Some(claim) = self.add_opcode {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = self.add_opcode_small {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = self.add_ap_opcode {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = self.assert_eq_opcode {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = self.assert_eq_opcode_imm {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = self.assert_eq_opcode_double_deref {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = self.blake_compress_opcode {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = self.call_opcode_abs {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = self.call_opcode_rel_imm {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = self.generic_opcode {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = self.jnz_opcode_non_taken {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = self.jnz_opcode_taken {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = self.jump_opcode_abs {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = self.jump_opcode_double_deref {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = self.jump_opcode_rel {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = self.jump_opcode_rel_imm {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = self.mul_opcode {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = self.mul_opcode_small {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = self.qm_31_add_mul_opcode {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = self.ret_opcode {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = self.verify_instruction {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = self.blake_round {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = self.blake_g {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = self.triple_xor_32 {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = self.add_mod_builtin {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = self.bitwise_builtin {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = self.mul_mod_builtin {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = self.pedersen_builtin {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = self.poseidon_builtin {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = self.range_check96_builtin {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = self.range_check_builtin {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = self.pedersen_aggregator_window_bits_18 {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = self.partial_ec_mul_window_bits_18 {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = self.poseidon_aggregator {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = self.poseidon_3_partial_rounds_chain {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = self.poseidon_full_round_chain {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = self.cube_252 {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = self.range_check_252_width_27 {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = self.memory_address_to_id {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = self.memory_id_to_big {
            claim.accumulate_relation_uses(ref relation_uses);
        }
    }
}

#[derive(Drop, Serde)]
pub struct CairoInteractionClaim {
    pub add_opcode: Option<components::add_opcode::InteractionClaim>,
    pub add_opcode_small: Option<components::add_opcode_small::InteractionClaim>,
    pub add_ap_opcode: Option<components::add_ap_opcode::InteractionClaim>,
    pub assert_eq_opcode: Option<components::assert_eq_opcode::InteractionClaim>,
    pub assert_eq_opcode_imm: Option<components::assert_eq_opcode_imm::InteractionClaim>,
    pub assert_eq_opcode_double_deref: Option<
        components::assert_eq_opcode_double_deref::InteractionClaim,
    >,
    pub blake_compress_opcode: Option<components::blake_compress_opcode::InteractionClaim>,
    pub call_opcode_abs: Option<components::call_opcode_abs::InteractionClaim>,
    pub call_opcode_rel_imm: Option<components::call_opcode_rel_imm::InteractionClaim>,
    pub generic_opcode: Option<components::generic_opcode::InteractionClaim>,
    pub jnz_opcode_non_taken: Option<components::jnz_opcode_non_taken::InteractionClaim>,
    pub jnz_opcode_taken: Option<components::jnz_opcode_taken::InteractionClaim>,
    pub jump_opcode_abs: Option<components::jump_opcode_abs::InteractionClaim>,
    pub jump_opcode_double_deref: Option<components::jump_opcode_double_deref::InteractionClaim>,
    pub jump_opcode_rel: Option<components::jump_opcode_rel::InteractionClaim>,
    pub jump_opcode_rel_imm: Option<components::jump_opcode_rel_imm::InteractionClaim>,
    pub mul_opcode: Option<components::mul_opcode::InteractionClaim>,
    pub mul_opcode_small: Option<components::mul_opcode_small::InteractionClaim>,
    pub qm_31_add_mul_opcode: Option<components::qm_31_add_mul_opcode::InteractionClaim>,
    pub ret_opcode: Option<components::ret_opcode::InteractionClaim>,
    pub verify_instruction: Option<components::verify_instruction::InteractionClaim>,
    pub blake_round: Option<components::blake_round::InteractionClaim>,
    pub blake_g: Option<components::blake_g::InteractionClaim>,
    pub blake_round_sigma: Option<components::blake_round_sigma::InteractionClaim>,
    pub triple_xor_32: Option<components::triple_xor_32::InteractionClaim>,
    pub verify_bitwise_xor_12: Option<components::verify_bitwise_xor_12::InteractionClaim>,
    pub add_mod_builtin: Option<components::add_mod_builtin::InteractionClaim>,
    pub bitwise_builtin: Option<components::bitwise_builtin::InteractionClaim>,
    pub mul_mod_builtin: Option<components::mul_mod_builtin::InteractionClaim>,
    pub pedersen_builtin: Option<components::pedersen_builtin::InteractionClaim>,
    pub poseidon_builtin: Option<components::poseidon_builtin::InteractionClaim>,
    pub range_check96_builtin: Option<components::range_check96_builtin::InteractionClaim>,
    pub range_check_builtin: Option<components::range_check_builtin::InteractionClaim>,
    pub pedersen_aggregator_window_bits_18: Option<
        components::pedersen_aggregator_window_bits_18::InteractionClaim,
    >,
    pub partial_ec_mul_window_bits_18: Option<
        components::partial_ec_mul_window_bits_18::InteractionClaim,
    >,
    pub pedersen_points_table_window_bits_18: Option<
        components::pedersen_points_table_window_bits_18::InteractionClaim,
    >,
    pub poseidon_aggregator: Option<components::poseidon_aggregator::InteractionClaim>,
    pub poseidon_3_partial_rounds_chain: Option<
        components::poseidon_3_partial_rounds_chain::InteractionClaim,
    >,
    pub poseidon_full_round_chain: Option<components::poseidon_full_round_chain::InteractionClaim>,
    pub cube_252: Option<components::cube_252::InteractionClaim>,
    pub poseidon_round_keys: Option<components::poseidon_round_keys::InteractionClaim>,
    pub range_check_252_width_27: Option<components::range_check_252_width_27::InteractionClaim>,
    pub memory_address_to_id: Option<components::memory_address_to_id::InteractionClaim>,
    pub memory_id_to_big: Option<components::memory_id_to_big::InteractionClaim>,
    pub range_check_6: Option<components::range_check_6::InteractionClaim>,
    pub range_check_8: Option<components::range_check_8::InteractionClaim>,
    pub range_check_11: Option<components::range_check_11::InteractionClaim>,
    pub range_check_12: Option<components::range_check_12::InteractionClaim>,
    pub range_check_18: Option<components::range_check_18::InteractionClaim>,
    pub range_check_20: Option<components::range_check_20::InteractionClaim>,
    pub range_check_4_3: Option<components::range_check_4_3::InteractionClaim>,
    pub range_check_4_4: Option<components::range_check_4_4::InteractionClaim>,
    pub range_check_9_9: Option<components::range_check_9_9::InteractionClaim>,
    pub range_check_7_2_5: Option<components::range_check_7_2_5::InteractionClaim>,
    pub range_check_3_6_6_3: Option<components::range_check_3_6_6_3::InteractionClaim>,
    pub range_check_4_4_4_4: Option<components::range_check_4_4_4_4::InteractionClaim>,
    pub range_check_3_3_3_3_3: Option<components::range_check_3_3_3_3_3::InteractionClaim>,
    pub verify_bitwise_xor_4: Option<components::verify_bitwise_xor_4::InteractionClaim>,
    pub verify_bitwise_xor_7: Option<components::verify_bitwise_xor_7::InteractionClaim>,
    pub verify_bitwise_xor_8: Option<components::verify_bitwise_xor_8::InteractionClaim>,
    pub verify_bitwise_xor_9: Option<components::verify_bitwise_xor_9::InteractionClaim>,
}

#[generate_trait]
pub impl CairoInteractionClaimImpl of CairoInteractionClaimTrace {
    fn mix_into(self: @CairoInteractionClaim, ref channel: Channel) {
        let claim = flatten_interaction_claim(self);
        channel.mix_felts(claim);
    }
}

pub fn lookup_sum(
    claim: @CairoClaim, elements: @CommonLookupElements, interaction_claim: @CairoInteractionClaim,
) -> QM31 {
    let mut sum = claim.public_data.logup_sum(elements);
    if let Some(interaction_claim) = interaction_claim.add_opcode {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.add_opcode_small {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.add_ap_opcode {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.assert_eq_opcode {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.assert_eq_opcode_imm {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.assert_eq_opcode_double_deref {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.blake_compress_opcode {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.call_opcode_abs {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.call_opcode_rel_imm {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.generic_opcode {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.jnz_opcode_non_taken {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.jnz_opcode_taken {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.jump_opcode_abs {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.jump_opcode_double_deref {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.jump_opcode_rel {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.jump_opcode_rel_imm {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.mul_opcode {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.mul_opcode_small {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.qm_31_add_mul_opcode {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.ret_opcode {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.verify_instruction {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.blake_round {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.blake_g {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.blake_round_sigma {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.triple_xor_32 {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.verify_bitwise_xor_12 {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.add_mod_builtin {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.bitwise_builtin {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.mul_mod_builtin {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.pedersen_builtin {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.poseidon_builtin {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.range_check96_builtin {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.range_check_builtin {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.pedersen_aggregator_window_bits_18 {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.partial_ec_mul_window_bits_18 {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.pedersen_points_table_window_bits_18 {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.poseidon_aggregator {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.poseidon_3_partial_rounds_chain {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.poseidon_full_round_chain {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.cube_252 {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.poseidon_round_keys {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.range_check_252_width_27 {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.memory_address_to_id {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.memory_id_to_big {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.range_check_6 {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.range_check_8 {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.range_check_11 {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.range_check_12 {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.range_check_18 {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.range_check_20 {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.range_check_4_3 {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.range_check_4_4 {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.range_check_9_9 {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.range_check_7_2_5 {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.range_check_3_6_6_3 {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.range_check_4_4_4_4 {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.range_check_3_3_3_3_3 {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.verify_bitwise_xor_4 {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.verify_bitwise_xor_7 {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.verify_bitwise_xor_8 {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = interaction_claim.verify_bitwise_xor_9 {
        sum += *interaction_claim.claimed_sum;
    }
    sum
}
