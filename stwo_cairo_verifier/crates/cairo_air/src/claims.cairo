// This file was created by the AIR team.

use stwo_cairo_air::{PublicData, PublicDataImpl, RelationUsesDict};
use stwo_constraint_framework::claim::ClaimTrait;
use stwo_constraint_framework::{CommonLookupElements, tree_array_concat_cols};
use stwo_verifier_core::TreeArray;
use stwo_verifier_core::channel::Channel;
use stwo_verifier_core::fields::qm31::QM31;
use stwo_verifier_core::utils::OptionImpl;
use crate::cairo_air::override_preprocessed_trace_log_sizes;
use crate::components::memory_address_to_id::MEMORY_ADDRESS_TO_ID_SPLIT;
use crate::components::{
    add_ap_opcode, add_mod_builtin, add_opcode, add_opcode_small, assert_eq_opcode,
    assert_eq_opcode_double_deref, assert_eq_opcode_imm, bitwise_builtin, blake_compress_opcode,
    blake_g, blake_round, blake_round_sigma, call_opcode_abs, call_opcode_rel_imm, cube_252,
    ec_op_builtin, generic_opcode, jnz_opcode_non_taken, jnz_opcode_taken, jump_opcode_abs,
    jump_opcode_double_deref, jump_opcode_rel, jump_opcode_rel_imm, memory_address_to_id,
    memory_id_to_big, memory_id_to_small, mul_mod_builtin, mul_opcode, mul_opcode_small,
    partial_ec_mul_generic, partial_ec_mul_window_bits_18, partial_ec_mul_window_bits_9,
    pedersen_aggregator_window_bits_18, pedersen_aggregator_window_bits_9, pedersen_builtin,
    pedersen_builtin_narrow_windows, pedersen_points_table_window_bits_18,
    pedersen_points_table_window_bits_9, poseidon_3_partial_rounds_chain, poseidon_aggregator,
    poseidon_builtin, poseidon_full_round_chain, poseidon_round_keys, qm_31_add_mul_opcode,
    range_check96_builtin, range_check_11, range_check_12, range_check_18, range_check_20,
    range_check_252_width_27, range_check_3_3_3_3_3, range_check_3_6_6_3, range_check_4_3,
    range_check_4_4, range_check_4_4_4_4, range_check_6, range_check_7_2_5, range_check_8,
    range_check_9_9, range_check_builtin, ret_opcode, triple_xor_32, verify_bitwise_xor_12,
    verify_bitwise_xor_4, verify_bitwise_xor_7, verify_bitwise_xor_8, verify_bitwise_xor_9,
    verify_instruction,
};
use crate::{ChannelTrait, PublicDataTrait, components};
use super::claim::{FlatClaim, FlatClaimTrait};

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
    pub pedersen_builtin_narrow_windows: Option<components::pedersen_builtin_narrow_windows::Claim>,
    pub poseidon_builtin: Option<components::poseidon_builtin::Claim>,
    pub range_check96_builtin: Option<components::range_check96_builtin::Claim>,
    pub range_check_builtin: Option<components::range_check_builtin::Claim>,
    pub ec_op_builtin: Option<components::ec_op_builtin::Claim>,
    pub partial_ec_mul_generic: Option<components::partial_ec_mul_generic::Claim>,
    pub pedersen_aggregator_window_bits_18: Option<
        components::pedersen_aggregator_window_bits_18::Claim,
    >,
    pub partial_ec_mul_window_bits_18: Option<components::partial_ec_mul_window_bits_18::Claim>,
    pub pedersen_points_table_window_bits_18: Option<
        components::pedersen_points_table_window_bits_18::Claim,
    >,
    pub pedersen_aggregator_window_bits_9: Option<
        components::pedersen_aggregator_window_bits_9::Claim,
    >,
    pub partial_ec_mul_window_bits_9: Option<components::partial_ec_mul_window_bits_9::Claim>,
    pub pedersen_points_table_window_bits_9: Option<
        components::pedersen_points_table_window_bits_9::Claim,
    >,
    pub poseidon_aggregator: Option<components::poseidon_aggregator::Claim>,
    pub poseidon_3_partial_rounds_chain: Option<components::poseidon_3_partial_rounds_chain::Claim>,
    pub poseidon_full_round_chain: Option<components::poseidon_full_round_chain::Claim>,
    pub cube_252: Option<components::cube_252::Claim>,
    pub poseidon_round_keys: Option<components::poseidon_round_keys::Claim>,
    pub range_check_252_width_27: Option<components::range_check_252_width_27::Claim>,
    pub memory_address_to_id: Option<components::memory_address_to_id::Claim>,
    pub memory_id_to_big: Option<components::memory_id_to_big::Claim>,
    pub memory_id_to_small: Option<components::memory_id_to_small::Claim>,
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
        if let Some(claim) = self.pedersen_builtin_narrow_windows {
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
        if let Some(claim) = self.ec_op_builtin {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.partial_ec_mul_generic {
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
        if let Some(claim) = self.pedersen_aggregator_window_bits_9 {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.partial_ec_mul_window_bits_9 {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.pedersen_points_table_window_bits_9 {
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
        if let Some(claim) = self.memory_id_to_small {
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
        let flat_claim = self.flatten_claim();
        flat_claim.mix_into(ref channel);
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
        if let Some(claim) = self.pedersen_builtin_narrow_windows {
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
        if let Some(claim) = self.ec_op_builtin {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = self.partial_ec_mul_generic {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = self.pedersen_aggregator_window_bits_18 {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = self.partial_ec_mul_window_bits_18 {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = self.pedersen_aggregator_window_bits_9 {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = self.partial_ec_mul_window_bits_9 {
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
        if let Some(claim) = self.memory_id_to_small {
            claim.accumulate_relation_uses(ref relation_uses);
        }
    }
}

#[generate_trait]
pub impl CairoClaimFlattenImpl of CairoClaimFlattenTrait {
    /// Create flat claim out of the given claim.
    fn flatten_claim(self: @CairoClaim) -> FlatClaim {
        let mut component_enable_bits = array![];
        let mut component_log_sizes = array![];

        if let Some(c) = self.add_opcode {
            component_log_sizes.append(*c.log_size);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(c) = self.add_opcode_small {
            component_log_sizes.append(*c.log_size);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(c) = self.add_ap_opcode {
            component_log_sizes.append(*c.log_size);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(c) = self.assert_eq_opcode {
            component_log_sizes.append(*c.log_size);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(c) = self.assert_eq_opcode_imm {
            component_log_sizes.append(*c.log_size);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(c) = self.assert_eq_opcode_double_deref {
            component_log_sizes.append(*c.log_size);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(c) = self.blake_compress_opcode {
            component_log_sizes.append(*c.log_size);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(c) = self.call_opcode_abs {
            component_log_sizes.append(*c.log_size);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(c) = self.call_opcode_rel_imm {
            component_log_sizes.append(*c.log_size);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(c) = self.generic_opcode {
            component_log_sizes.append(*c.log_size);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(c) = self.jnz_opcode_non_taken {
            component_log_sizes.append(*c.log_size);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(c) = self.jnz_opcode_taken {
            component_log_sizes.append(*c.log_size);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(c) = self.jump_opcode_abs {
            component_log_sizes.append(*c.log_size);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(c) = self.jump_opcode_double_deref {
            component_log_sizes.append(*c.log_size);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(c) = self.jump_opcode_rel {
            component_log_sizes.append(*c.log_size);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(c) = self.jump_opcode_rel_imm {
            component_log_sizes.append(*c.log_size);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(c) = self.mul_opcode {
            component_log_sizes.append(*c.log_size);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(c) = self.mul_opcode_small {
            component_log_sizes.append(*c.log_size);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(c) = self.qm_31_add_mul_opcode {
            component_log_sizes.append(*c.log_size);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(c) = self.ret_opcode {
            component_log_sizes.append(*c.log_size);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(c) = self.verify_instruction {
            component_log_sizes.append(*c.log_size);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(c) = self.blake_round {
            component_log_sizes.append(*c.log_size);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(c) = self.blake_g {
            component_log_sizes.append(*c.log_size);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(_c) = self.blake_round_sigma {
            component_log_sizes.append(blake_round_sigma::LOG_SIZE);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(c) = self.triple_xor_32 {
            component_log_sizes.append(*c.log_size);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(_c) = self.verify_bitwise_xor_12 {
            component_log_sizes.append(verify_bitwise_xor_12::LOG_SIZE);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(c) = self.add_mod_builtin {
            component_log_sizes.append(*c.log_size);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(c) = self.bitwise_builtin {
            component_log_sizes.append(*c.log_size);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(c) = self.mul_mod_builtin {
            component_log_sizes.append(*c.log_size);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(c) = self.pedersen_builtin {
            component_log_sizes.append(*c.log_size);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(c) = self.pedersen_builtin_narrow_windows {
            component_log_sizes.append(*c.log_size);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(c) = self.poseidon_builtin {
            component_log_sizes.append(*c.log_size);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(c) = self.range_check96_builtin {
            component_log_sizes.append(*c.log_size);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(c) = self.range_check_builtin {
            component_log_sizes.append(*c.log_size);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(c) = self.ec_op_builtin {
            component_log_sizes.append(*c.log_size);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(c) = self.partial_ec_mul_generic {
            component_log_sizes.append(*c.log_size);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(c) = self.pedersen_aggregator_window_bits_18 {
            component_log_sizes.append(*c.log_size);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(c) = self.partial_ec_mul_window_bits_18 {
            component_log_sizes.append(*c.log_size);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(_c) = self.pedersen_points_table_window_bits_18 {
            component_log_sizes.append(pedersen_points_table_window_bits_18::LOG_SIZE);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(c) = self.pedersen_aggregator_window_bits_9 {
            component_log_sizes.append(*c.log_size);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(c) = self.partial_ec_mul_window_bits_9 {
            component_log_sizes.append(*c.log_size);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(_c) = self.pedersen_points_table_window_bits_9 {
            component_log_sizes.append(pedersen_points_table_window_bits_9::LOG_SIZE);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(c) = self.poseidon_aggregator {
            component_log_sizes.append(*c.log_size);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(c) = self.poseidon_3_partial_rounds_chain {
            component_log_sizes.append(*c.log_size);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(c) = self.poseidon_full_round_chain {
            component_log_sizes.append(*c.log_size);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(c) = self.cube_252 {
            component_log_sizes.append(*c.log_size);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(_c) = self.poseidon_round_keys {
            component_log_sizes.append(poseidon_round_keys::LOG_SIZE);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(c) = self.range_check_252_width_27 {
            component_log_sizes.append(*c.log_size);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(c) = self.memory_address_to_id {
            component_log_sizes.append(*c.log_size);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        let memory_id_to_big::Claim { big_log_sizes } = self.memory_id_to_big.as_snap().unwrap();
        assert!(big_log_sizes.len() <= MEMORY_ADDRESS_TO_ID_SPLIT);
        for log_size in big_log_sizes {
            component_log_sizes.append(*log_size);
            component_enable_bits.append(true);
        }
        for _ in 0..(MEMORY_ADDRESS_TO_ID_SPLIT - big_log_sizes.len()) {
            component_enable_bits.append(false);
        }
        if let Some(c) = self.memory_id_to_small {
            component_log_sizes.append(*c.log_size);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(_c) = self.range_check_6 {
            component_log_sizes.append(range_check_6::LOG_SIZE);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(_c) = self.range_check_8 {
            component_log_sizes.append(range_check_8::LOG_SIZE);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(_c) = self.range_check_11 {
            component_log_sizes.append(range_check_11::LOG_SIZE);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(_c) = self.range_check_12 {
            component_log_sizes.append(range_check_12::LOG_SIZE);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(_c) = self.range_check_18 {
            component_log_sizes.append(range_check_18::LOG_SIZE);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(_c) = self.range_check_20 {
            component_log_sizes.append(range_check_20::LOG_SIZE);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(_c) = self.range_check_4_3 {
            component_log_sizes.append(range_check_4_3::LOG_SIZE);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(_c) = self.range_check_4_4 {
            component_log_sizes.append(range_check_4_4::LOG_SIZE);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(_c) = self.range_check_9_9 {
            component_log_sizes.append(range_check_9_9::LOG_SIZE);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(_c) = self.range_check_7_2_5 {
            component_log_sizes.append(range_check_7_2_5::LOG_SIZE);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(_c) = self.range_check_3_6_6_3 {
            component_log_sizes.append(range_check_3_6_6_3::LOG_SIZE);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(_c) = self.range_check_4_4_4_4 {
            component_log_sizes.append(range_check_4_4_4_4::LOG_SIZE);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(_c) = self.range_check_3_3_3_3_3 {
            component_log_sizes.append(range_check_3_3_3_3_3::LOG_SIZE);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(_c) = self.verify_bitwise_xor_4 {
            component_log_sizes.append(verify_bitwise_xor_4::LOG_SIZE);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(_c) = self.verify_bitwise_xor_7 {
            component_log_sizes.append(verify_bitwise_xor_7::LOG_SIZE);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(_c) = self.verify_bitwise_xor_8 {
            component_log_sizes.append(verify_bitwise_xor_8::LOG_SIZE);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }
        if let Some(_c) = self.verify_bitwise_xor_9 {
            component_log_sizes.append(verify_bitwise_xor_9::LOG_SIZE);
            component_enable_bits.append(true);
        } else {
            component_enable_bits.append(false);
        }

        FlatClaim {
            component_enable_bits: component_enable_bits.span(),
            component_log_sizes: component_log_sizes.span(),
            public_data: self.public_data.clone(),
        }
    }
}

#[derive(Drop, Serde)]
pub struct CairoInteractionClaim {
    pub claimed_sum: Vec<QM31>,
}

#[generate_trait]
pub impl CairoInteractionClaimImpl of CairoInteractionClaimTrace {
    fn mix_into(self: @CairoInteractionClaim, ref channel: Channel) {
        channel.mix_felts(self.claimed_sum.span());
    }

    /// Returns the flattened claimed sums for the logup argument, one per component.
    /// The order must match the order of components as they appear in [cairo_air::air::CairoAir].
    fn flatten_interaction_claim(self: @CairoInteractionClaim) -> Span<QM31> {
        self.claimed_sum.span()
    }
}

pub fn lookup_sum(
    claim: @CairoClaim, elements: @CommonLookupElements, interaction_claim: @CairoInteractionClaim,
) -> QM31 {
    let mut sum = claim.public_data.logup_sum(elements);
    for claimed_sum in interaction_claim.claimed_sum.span() {
        sum += *claimed_sum;
    }
    sum
}
