// This file was created by the AIR team.

use num_traits::Zero;
use serde::{Deserialize, Serialize};
use stwo::core::channel::Channel;
use stwo::core::fields::qm31::QM31;
use stwo_cairo_serialize::{CairoDeserialize, CairoSerialize};

use crate::air::{
    accumulate_relation_memory, accumulate_relation_uses, PublicData, RelationUsesDict,
};
use crate::claims::prelude::SecureField;
use crate::components::*;
use crate::relations::CommonLookupElements;

#[derive(Clone, Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct CairoClaim {
    pub public_data: PublicData,
    pub blake_compress_opcode: Option<blake_compress_opcode::Claim>,
    pub triple_xor_32: Option<triple_xor_32::Claim>,
    pub blake_round: Option<blake_round::Claim>,
    pub blake_g: Option<blake_g::Claim>,
    pub verify_bitwise_xor_7: Option<verify_bitwise_xor_7::Claim>,
    pub verify_bitwise_xor_4: Option<verify_bitwise_xor_4::Claim>,
    pub verify_bitwise_xor_12: Option<verify_bitwise_xor_12::Claim>,
    pub blake_round_sigma: Option<blake_round_sigma::Claim>,
    pub qm_31_add_mul_opcode: Option<qm_31_add_mul_opcode::Claim>,
    pub ret_opcode: Option<ret_opcode::Claim>,
    pub mul_opcode: Option<mul_opcode::Claim>,
    pub mul_opcode_small: Option<mul_opcode_small::Claim>,
    pub jump_opcode_abs: Option<jump_opcode_abs::Claim>,
    pub jump_opcode_double_deref: Option<jump_opcode_double_deref::Claim>,
    pub jump_opcode_rel: Option<jump_opcode_rel::Claim>,
    pub jump_opcode_rel_imm: Option<jump_opcode_rel_imm::Claim>,
    pub jnz_opcode_non_taken: Option<jnz_opcode_non_taken::Claim>,
    pub jnz_opcode_taken: Option<jnz_opcode_taken::Claim>,
    pub call_opcode_rel_imm: Option<call_opcode_rel_imm::Claim>,
    pub call_opcode_abs: Option<call_opcode_abs::Claim>,
    pub assert_eq_opcode_imm: Option<assert_eq_opcode_imm::Claim>,
    pub assert_eq_opcode_double_deref: Option<assert_eq_opcode_double_deref::Claim>,
    pub assert_eq_opcode: Option<assert_eq_opcode::Claim>,
    pub add_opcode: Option<add_opcode::Claim>,
    pub add_opcode_small: Option<add_opcode_small::Claim>,
    pub add_ap_opcode: Option<add_ap_opcode::Claim>,
    pub generic_opcode: Option<generic_opcode::Claim>,
    pub range_check_11: Option<range_check_11::Claim>,
    pub verify_instruction: Option<verify_instruction::Claim>,
    pub range_check_4_3: Option<range_check_4_3::Claim>,
    pub range_check_7_2_5: Option<range_check_7_2_5::Claim>,
    pub pedersen_builtin: Option<pedersen_builtin::Claim>,
    pub pedersen_aggregator_window_bits_18: Option<pedersen_aggregator_window_bits_18::Claim>,
    pub partial_ec_mul_window_bits_18: Option<partial_ec_mul_window_bits_18::Claim>,
    pub pedersen_points_table_window_bits_18: Option<pedersen_points_table_window_bits_18::Claim>,
    pub range_check_8: Option<range_check_8::Claim>,
    pub poseidon_builtin: Option<poseidon_builtin::Claim>,
    pub poseidon_aggregator: Option<poseidon_aggregator::Claim>,
    pub poseidon_3_partial_rounds_chain: Option<poseidon_3_partial_rounds_chain::Claim>,
    pub range_check_4_4: Option<range_check_4_4::Claim>,
    pub range_check_4_4_4_4: Option<range_check_4_4_4_4::Claim>,
    pub range_check_252_width_27: Option<range_check_252_width_27::Claim>,
    pub poseidon_full_round_chain: Option<poseidon_full_round_chain::Claim>,
    pub range_check_3_3_3_3_3: Option<range_check_3_3_3_3_3::Claim>,
    pub poseidon_round_keys: Option<poseidon_round_keys::Claim>,
    pub cube_252: Option<cube_252::Claim>,
    pub range_check_20: Option<range_check_20::Claim>,
    pub mul_mod_builtin: Option<mul_mod_builtin::Claim>,
    pub range_check_18: Option<range_check_18::Claim>,
    pub range_check_3_6_6_3: Option<range_check_3_6_6_3::Claim>,
    pub range_check_12: Option<range_check_12::Claim>,
    pub add_mod_builtin: Option<add_mod_builtin::Claim>,
    pub range_check96_builtin: Option<range_check96_builtin::Claim>,
    pub range_check_6: Option<range_check_6::Claim>,
    pub range_check_builtin: Option<range_check_builtin::Claim>,
    pub bitwise_builtin: Option<bitwise_builtin::Claim>,
    pub verify_bitwise_xor_8: Option<verify_bitwise_xor_8::Claim>,
    pub verify_bitwise_xor_9: Option<verify_bitwise_xor_9::Claim>,
    pub memory_id_to_big: Option<memory_id_to_big::Claim>,
    pub range_check_9_9: Option<range_check_9_9::Claim>,
    pub memory_address_to_id: Option<memory_address_to_id::Claim>,
}

impl CairoClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        self.public_data.mix_into(channel);
        channel.mix_u64(self.blake_compress_opcode.is_some() as u64);
        self.blake_compress_opcode
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.triple_xor_32.is_some() as u64);
        self.triple_xor_32.as_ref().inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.blake_round.is_some() as u64);
        self.blake_round.as_ref().inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.blake_g.is_some() as u64);
        self.blake_g.as_ref().inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.verify_bitwise_xor_7.is_some() as u64);
        self.verify_bitwise_xor_7
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.verify_bitwise_xor_4.is_some() as u64);
        self.verify_bitwise_xor_4
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.verify_bitwise_xor_12.is_some() as u64);
        self.verify_bitwise_xor_12
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.blake_round_sigma.is_some() as u64);
        self.blake_round_sigma
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.qm_31_add_mul_opcode.is_some() as u64);
        self.qm_31_add_mul_opcode
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.ret_opcode.is_some() as u64);
        self.ret_opcode.as_ref().inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.mul_opcode.is_some() as u64);
        self.mul_opcode.as_ref().inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.mul_opcode_small.is_some() as u64);
        self.mul_opcode_small
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.jump_opcode_abs.is_some() as u64);
        self.jump_opcode_abs
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.jump_opcode_double_deref.is_some() as u64);
        self.jump_opcode_double_deref
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.jump_opcode_rel.is_some() as u64);
        self.jump_opcode_rel
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.jump_opcode_rel_imm.is_some() as u64);
        self.jump_opcode_rel_imm
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.jnz_opcode_non_taken.is_some() as u64);
        self.jnz_opcode_non_taken
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.jnz_opcode_taken.is_some() as u64);
        self.jnz_opcode_taken
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.call_opcode_rel_imm.is_some() as u64);
        self.call_opcode_rel_imm
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.call_opcode_abs.is_some() as u64);
        self.call_opcode_abs
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.assert_eq_opcode_imm.is_some() as u64);
        self.assert_eq_opcode_imm
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.assert_eq_opcode_double_deref.is_some() as u64);
        self.assert_eq_opcode_double_deref
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.assert_eq_opcode.is_some() as u64);
        self.assert_eq_opcode
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.add_opcode.is_some() as u64);
        self.add_opcode.as_ref().inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.add_opcode_small.is_some() as u64);
        self.add_opcode_small
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.add_ap_opcode.is_some() as u64);
        self.add_ap_opcode.as_ref().inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.generic_opcode.is_some() as u64);
        self.generic_opcode
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.range_check_11.is_some() as u64);
        self.range_check_11
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.verify_instruction.is_some() as u64);
        self.verify_instruction
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.range_check_4_3.is_some() as u64);
        self.range_check_4_3
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.range_check_7_2_5.is_some() as u64);
        self.range_check_7_2_5
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.pedersen_builtin.is_some() as u64);
        self.pedersen_builtin
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.pedersen_aggregator_window_bits_18.is_some() as u64);
        self.pedersen_aggregator_window_bits_18
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.partial_ec_mul_window_bits_18.is_some() as u64);
        self.partial_ec_mul_window_bits_18
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.pedersen_points_table_window_bits_18.is_some() as u64);
        self.pedersen_points_table_window_bits_18
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.range_check_8.is_some() as u64);
        self.range_check_8.as_ref().inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.poseidon_builtin.is_some() as u64);
        self.poseidon_builtin
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.poseidon_aggregator.is_some() as u64);
        self.poseidon_aggregator
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.poseidon_3_partial_rounds_chain.is_some() as u64);
        self.poseidon_3_partial_rounds_chain
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.range_check_4_4.is_some() as u64);
        self.range_check_4_4
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.range_check_4_4_4_4.is_some() as u64);
        self.range_check_4_4_4_4
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.range_check_252_width_27.is_some() as u64);
        self.range_check_252_width_27
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.poseidon_full_round_chain.is_some() as u64);
        self.poseidon_full_round_chain
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.range_check_3_3_3_3_3.is_some() as u64);
        self.range_check_3_3_3_3_3
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.poseidon_round_keys.is_some() as u64);
        self.poseidon_round_keys
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.cube_252.is_some() as u64);
        self.cube_252.as_ref().inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.range_check_20.is_some() as u64);
        self.range_check_20
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.mul_mod_builtin.is_some() as u64);
        self.mul_mod_builtin
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.range_check_18.is_some() as u64);
        self.range_check_18
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.range_check_3_6_6_3.is_some() as u64);
        self.range_check_3_6_6_3
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.range_check_12.is_some() as u64);
        self.range_check_12
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.add_mod_builtin.is_some() as u64);
        self.add_mod_builtin
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.range_check96_builtin.is_some() as u64);
        self.range_check96_builtin
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.range_check_6.is_some() as u64);
        self.range_check_6.as_ref().inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.range_check_builtin.is_some() as u64);
        self.range_check_builtin
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.bitwise_builtin.is_some() as u64);
        self.bitwise_builtin
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.verify_bitwise_xor_8.is_some() as u64);
        self.verify_bitwise_xor_8
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.verify_bitwise_xor_9.is_some() as u64);
        self.verify_bitwise_xor_9
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.memory_id_to_big.is_some() as u64);
        self.memory_id_to_big
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.range_check_9_9.is_some() as u64);
        self.range_check_9_9
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        channel.mix_u64(self.memory_address_to_id.is_some() as u64);
        self.memory_address_to_id
            .as_ref()
            .inspect(|c| c.mix_into(channel));
    }

    pub fn accumulate_relation_uses(&self, relation_uses: &mut RelationUsesDict) {
        self.blake_compress_opcode.as_ref().inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                blake_compress_opcode::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        self.triple_xor_32.as_ref().inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                triple_xor_32::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        self.blake_round.as_ref().inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                blake_round::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        self.blake_g.as_ref().inspect(|c| {
            accumulate_relation_uses(relation_uses, blake_g::RELATION_USES_PER_ROW, c.log_size)
        });
        self.qm_31_add_mul_opcode.as_ref().inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                qm_31_add_mul_opcode::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        self.ret_opcode.as_ref().inspect(|c| {
            accumulate_relation_uses(relation_uses, ret_opcode::RELATION_USES_PER_ROW, c.log_size)
        });
        self.mul_opcode.as_ref().inspect(|c| {
            accumulate_relation_uses(relation_uses, mul_opcode::RELATION_USES_PER_ROW, c.log_size)
        });
        self.mul_opcode_small.as_ref().inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                mul_opcode_small::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        self.jump_opcode_abs.as_ref().inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                jump_opcode_abs::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        self.jump_opcode_double_deref.as_ref().inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                jump_opcode_double_deref::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        self.jump_opcode_rel.as_ref().inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                jump_opcode_rel::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        self.jump_opcode_rel_imm.as_ref().inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                jump_opcode_rel_imm::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        self.jnz_opcode_non_taken.as_ref().inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                jnz_opcode_non_taken::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        self.jnz_opcode_taken.as_ref().inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                jnz_opcode_taken::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        self.call_opcode_rel_imm.as_ref().inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                call_opcode_rel_imm::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        self.call_opcode_abs.as_ref().inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                call_opcode_abs::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        self.assert_eq_opcode_imm.as_ref().inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                assert_eq_opcode_imm::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        self.assert_eq_opcode_double_deref.as_ref().inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                assert_eq_opcode_double_deref::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        self.assert_eq_opcode.as_ref().inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                assert_eq_opcode::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        self.add_opcode.as_ref().inspect(|c| {
            accumulate_relation_uses(relation_uses, add_opcode::RELATION_USES_PER_ROW, c.log_size)
        });
        self.add_opcode_small.as_ref().inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                add_opcode_small::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        self.add_ap_opcode.as_ref().inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                add_ap_opcode::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        self.generic_opcode.as_ref().inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                generic_opcode::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        self.verify_instruction.as_ref().inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                verify_instruction::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        self.pedersen_builtin.as_ref().inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                pedersen_builtin::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        self.pedersen_aggregator_window_bits_18
            .as_ref()
            .inspect(|c| {
                accumulate_relation_uses(
                    relation_uses,
                    pedersen_aggregator_window_bits_18::RELATION_USES_PER_ROW,
                    c.log_size,
                )
            });
        self.partial_ec_mul_window_bits_18.as_ref().inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                partial_ec_mul_window_bits_18::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        self.poseidon_builtin.as_ref().inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                poseidon_builtin::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        self.poseidon_aggregator.as_ref().inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                poseidon_aggregator::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        self.poseidon_3_partial_rounds_chain.as_ref().inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                poseidon_3_partial_rounds_chain::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        self.range_check_252_width_27.as_ref().inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                range_check_252_width_27::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        self.poseidon_full_round_chain.as_ref().inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                poseidon_full_round_chain::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        self.cube_252.as_ref().inspect(|c| {
            accumulate_relation_uses(relation_uses, cube_252::RELATION_USES_PER_ROW, c.log_size)
        });
        self.mul_mod_builtin.as_ref().inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                mul_mod_builtin::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        self.add_mod_builtin.as_ref().inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                add_mod_builtin::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        self.range_check96_builtin.as_ref().inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                range_check96_builtin::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        self.range_check_builtin.as_ref().inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                range_check_builtin::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        self.bitwise_builtin.as_ref().inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                bitwise_builtin::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        accumulate_relation_memory(relation_uses, &self.memory_id_to_big);
        self.memory_address_to_id.as_ref().inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                memory_address_to_id::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
    }
}

#[derive(Clone, Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct CairoInteractionClaim {
    pub blake_compress_opcode: Option<blake_compress_opcode::InteractionClaim>,
    pub triple_xor_32: Option<triple_xor_32::InteractionClaim>,
    pub blake_round: Option<blake_round::InteractionClaim>,
    pub blake_g: Option<blake_g::InteractionClaim>,
    pub verify_bitwise_xor_7: Option<verify_bitwise_xor_7::InteractionClaim>,
    pub verify_bitwise_xor_4: Option<verify_bitwise_xor_4::InteractionClaim>,
    pub verify_bitwise_xor_12: Option<verify_bitwise_xor_12::InteractionClaim>,
    pub blake_round_sigma: Option<blake_round_sigma::InteractionClaim>,
    pub qm_31_add_mul_opcode: Option<qm_31_add_mul_opcode::InteractionClaim>,
    pub ret_opcode: Option<ret_opcode::InteractionClaim>,
    pub mul_opcode: Option<mul_opcode::InteractionClaim>,
    pub mul_opcode_small: Option<mul_opcode_small::InteractionClaim>,
    pub jump_opcode_abs: Option<jump_opcode_abs::InteractionClaim>,
    pub jump_opcode_double_deref: Option<jump_opcode_double_deref::InteractionClaim>,
    pub jump_opcode_rel: Option<jump_opcode_rel::InteractionClaim>,
    pub jump_opcode_rel_imm: Option<jump_opcode_rel_imm::InteractionClaim>,
    pub jnz_opcode_non_taken: Option<jnz_opcode_non_taken::InteractionClaim>,
    pub jnz_opcode_taken: Option<jnz_opcode_taken::InteractionClaim>,
    pub call_opcode_rel_imm: Option<call_opcode_rel_imm::InteractionClaim>,
    pub call_opcode_abs: Option<call_opcode_abs::InteractionClaim>,
    pub assert_eq_opcode_imm: Option<assert_eq_opcode_imm::InteractionClaim>,
    pub assert_eq_opcode_double_deref: Option<assert_eq_opcode_double_deref::InteractionClaim>,
    pub assert_eq_opcode: Option<assert_eq_opcode::InteractionClaim>,
    pub add_opcode: Option<add_opcode::InteractionClaim>,
    pub add_opcode_small: Option<add_opcode_small::InteractionClaim>,
    pub add_ap_opcode: Option<add_ap_opcode::InteractionClaim>,
    pub generic_opcode: Option<generic_opcode::InteractionClaim>,
    pub range_check_11: Option<range_check_11::InteractionClaim>,
    pub verify_instruction: Option<verify_instruction::InteractionClaim>,
    pub range_check_4_3: Option<range_check_4_3::InteractionClaim>,
    pub range_check_7_2_5: Option<range_check_7_2_5::InteractionClaim>,
    pub pedersen_builtin: Option<pedersen_builtin::InteractionClaim>,
    pub pedersen_aggregator_window_bits_18:
        Option<pedersen_aggregator_window_bits_18::InteractionClaim>,
    pub partial_ec_mul_window_bits_18: Option<partial_ec_mul_window_bits_18::InteractionClaim>,
    pub pedersen_points_table_window_bits_18:
        Option<pedersen_points_table_window_bits_18::InteractionClaim>,
    pub range_check_8: Option<range_check_8::InteractionClaim>,
    pub poseidon_builtin: Option<poseidon_builtin::InteractionClaim>,
    pub poseidon_aggregator: Option<poseidon_aggregator::InteractionClaim>,
    pub poseidon_3_partial_rounds_chain: Option<poseidon_3_partial_rounds_chain::InteractionClaim>,
    pub range_check_4_4: Option<range_check_4_4::InteractionClaim>,
    pub range_check_4_4_4_4: Option<range_check_4_4_4_4::InteractionClaim>,
    pub range_check_252_width_27: Option<range_check_252_width_27::InteractionClaim>,
    pub poseidon_full_round_chain: Option<poseidon_full_round_chain::InteractionClaim>,
    pub range_check_3_3_3_3_3: Option<range_check_3_3_3_3_3::InteractionClaim>,
    pub poseidon_round_keys: Option<poseidon_round_keys::InteractionClaim>,
    pub cube_252: Option<cube_252::InteractionClaim>,
    pub range_check_20: Option<range_check_20::InteractionClaim>,
    pub mul_mod_builtin: Option<mul_mod_builtin::InteractionClaim>,
    pub range_check_18: Option<range_check_18::InteractionClaim>,
    pub range_check_3_6_6_3: Option<range_check_3_6_6_3::InteractionClaim>,
    pub range_check_12: Option<range_check_12::InteractionClaim>,
    pub add_mod_builtin: Option<add_mod_builtin::InteractionClaim>,
    pub range_check96_builtin: Option<range_check96_builtin::InteractionClaim>,
    pub range_check_6: Option<range_check_6::InteractionClaim>,
    pub range_check_builtin: Option<range_check_builtin::InteractionClaim>,
    pub bitwise_builtin: Option<bitwise_builtin::InteractionClaim>,
    pub verify_bitwise_xor_8: Option<verify_bitwise_xor_8::InteractionClaim>,
    pub verify_bitwise_xor_9: Option<verify_bitwise_xor_9::InteractionClaim>,
    pub memory_id_to_big: Option<memory_id_to_big::InteractionClaim>,
    pub range_check_9_9: Option<range_check_9_9::InteractionClaim>,
    pub memory_address_to_id: Option<memory_address_to_id::InteractionClaim>,
}

impl CairoInteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        self.blake_compress_opcode
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        self.triple_xor_32.as_ref().inspect(|c| c.mix_into(channel));
        self.blake_round.as_ref().inspect(|c| c.mix_into(channel));
        self.blake_g.as_ref().inspect(|c| c.mix_into(channel));
        self.verify_bitwise_xor_7
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        self.verify_bitwise_xor_4
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        self.verify_bitwise_xor_12
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        self.blake_round_sigma
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        self.qm_31_add_mul_opcode
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        self.ret_opcode.as_ref().inspect(|c| c.mix_into(channel));
        self.mul_opcode.as_ref().inspect(|c| c.mix_into(channel));
        self.mul_opcode_small
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        self.jump_opcode_abs
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        self.jump_opcode_double_deref
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        self.jump_opcode_rel
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        self.jump_opcode_rel_imm
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        self.jnz_opcode_non_taken
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        self.jnz_opcode_taken
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        self.call_opcode_rel_imm
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        self.call_opcode_abs
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        self.assert_eq_opcode_imm
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        self.assert_eq_opcode_double_deref
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        self.assert_eq_opcode
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        self.add_opcode.as_ref().inspect(|c| c.mix_into(channel));
        self.add_opcode_small
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        self.add_ap_opcode.as_ref().inspect(|c| c.mix_into(channel));
        self.generic_opcode
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        self.range_check_11
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        self.verify_instruction
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        self.range_check_4_3
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        self.range_check_7_2_5
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        self.pedersen_builtin
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        self.pedersen_aggregator_window_bits_18
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        self.partial_ec_mul_window_bits_18
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        self.pedersen_points_table_window_bits_18
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        self.range_check_8.as_ref().inspect(|c| c.mix_into(channel));
        self.poseidon_builtin
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        self.poseidon_aggregator
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        self.poseidon_3_partial_rounds_chain
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        self.range_check_4_4
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        self.range_check_4_4_4_4
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        self.range_check_252_width_27
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        self.poseidon_full_round_chain
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        self.range_check_3_3_3_3_3
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        self.poseidon_round_keys
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        self.cube_252.as_ref().inspect(|c| c.mix_into(channel));
        self.range_check_20
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        self.mul_mod_builtin
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        self.range_check_18
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        self.range_check_3_6_6_3
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        self.range_check_12
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        self.add_mod_builtin
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        self.range_check96_builtin
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        self.range_check_6.as_ref().inspect(|c| c.mix_into(channel));
        self.range_check_builtin
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        self.bitwise_builtin
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        self.verify_bitwise_xor_8
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        self.verify_bitwise_xor_9
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        self.memory_id_to_big
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        self.range_check_9_9
            .as_ref()
            .inspect(|c| c.mix_into(channel));
        self.memory_address_to_id
            .as_ref()
            .inspect(|c| c.mix_into(channel));
    }
}

pub fn lookup_sum(
    claim: &CairoClaim,
    common_lookup_elements: &CommonLookupElements,
    interaction_claim: &CairoInteractionClaim,
) -> SecureField {
    let mut sum = QM31::zero();
    sum += claim.public_data.logup_sum(common_lookup_elements);
    interaction_claim
        .blake_compress_opcode
        .as_ref()
        .inspect(|ic| {
            sum += ic.claimed_sum;
        });
    interaction_claim.triple_xor_32.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim.blake_round.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim.blake_g.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim
        .verify_bitwise_xor_7
        .as_ref()
        .inspect(|ic| {
            sum += ic.claimed_sum;
        });
    interaction_claim
        .verify_bitwise_xor_4
        .as_ref()
        .inspect(|ic| {
            sum += ic.claimed_sum;
        });
    interaction_claim
        .verify_bitwise_xor_12
        .as_ref()
        .inspect(|ic| {
            sum += ic.claimed_sum;
        });
    interaction_claim.blake_round_sigma.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim
        .qm_31_add_mul_opcode
        .as_ref()
        .inspect(|ic| {
            sum += ic.claimed_sum;
        });
    interaction_claim.ret_opcode.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim.mul_opcode.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim.mul_opcode_small.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim.jump_opcode_abs.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim
        .jump_opcode_double_deref
        .as_ref()
        .inspect(|ic| {
            sum += ic.claimed_sum;
        });
    interaction_claim.jump_opcode_rel.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim
        .jump_opcode_rel_imm
        .as_ref()
        .inspect(|ic| {
            sum += ic.claimed_sum;
        });
    interaction_claim
        .jnz_opcode_non_taken
        .as_ref()
        .inspect(|ic| {
            sum += ic.claimed_sum;
        });
    interaction_claim.jnz_opcode_taken.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim
        .call_opcode_rel_imm
        .as_ref()
        .inspect(|ic| {
            sum += ic.claimed_sum;
        });
    interaction_claim.call_opcode_abs.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim
        .assert_eq_opcode_imm
        .as_ref()
        .inspect(|ic| {
            sum += ic.claimed_sum;
        });
    interaction_claim
        .assert_eq_opcode_double_deref
        .as_ref()
        .inspect(|ic| {
            sum += ic.claimed_sum;
        });
    interaction_claim.assert_eq_opcode.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim.add_opcode.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim.add_opcode_small.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim.add_ap_opcode.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim.generic_opcode.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim.range_check_11.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim.verify_instruction.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim.range_check_4_3.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim.range_check_7_2_5.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim.pedersen_builtin.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim
        .pedersen_aggregator_window_bits_18
        .as_ref()
        .inspect(|ic| {
            sum += ic.claimed_sum;
        });
    interaction_claim
        .partial_ec_mul_window_bits_18
        .as_ref()
        .inspect(|ic| {
            sum += ic.claimed_sum;
        });
    interaction_claim
        .pedersen_points_table_window_bits_18
        .as_ref()
        .inspect(|ic| {
            sum += ic.claimed_sum;
        });
    interaction_claim.range_check_8.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim.poseidon_builtin.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim
        .poseidon_aggregator
        .as_ref()
        .inspect(|ic| {
            sum += ic.claimed_sum;
        });
    interaction_claim
        .poseidon_3_partial_rounds_chain
        .as_ref()
        .inspect(|ic| {
            sum += ic.claimed_sum;
        });
    interaction_claim.range_check_4_4.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim
        .range_check_4_4_4_4
        .as_ref()
        .inspect(|ic| {
            sum += ic.claimed_sum;
        });
    interaction_claim
        .range_check_252_width_27
        .as_ref()
        .inspect(|ic| {
            sum += ic.claimed_sum;
        });
    interaction_claim
        .poseidon_full_round_chain
        .as_ref()
        .inspect(|ic| {
            sum += ic.claimed_sum;
        });
    interaction_claim
        .range_check_3_3_3_3_3
        .as_ref()
        .inspect(|ic| {
            sum += ic.claimed_sum;
        });
    interaction_claim
        .poseidon_round_keys
        .as_ref()
        .inspect(|ic| {
            sum += ic.claimed_sum;
        });
    interaction_claim.cube_252.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim.range_check_20.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim.mul_mod_builtin.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim.range_check_18.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim
        .range_check_3_6_6_3
        .as_ref()
        .inspect(|ic| {
            sum += ic.claimed_sum;
        });
    interaction_claim.range_check_12.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim.add_mod_builtin.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim
        .range_check96_builtin
        .as_ref()
        .inspect(|ic| {
            sum += ic.claimed_sum;
        });
    interaction_claim.range_check_6.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim
        .range_check_builtin
        .as_ref()
        .inspect(|ic| {
            sum += ic.claimed_sum;
        });
    interaction_claim.bitwise_builtin.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim
        .verify_bitwise_xor_8
        .as_ref()
        .inspect(|ic| {
            sum += ic.claimed_sum;
        });
    interaction_claim
        .verify_bitwise_xor_9
        .as_ref()
        .inspect(|ic| {
            sum += ic.claimed_sum;
        });
    interaction_claim.memory_id_to_big.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim.range_check_9_9.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim
        .memory_address_to_id
        .as_ref()
        .inspect(|ic| {
            sum += ic.claimed_sum;
        });
    sum
}
