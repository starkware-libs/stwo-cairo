// This file was created by the AIR team.

use num_traits::Zero;
use serde::{Deserialize, Serialize};
use stwo::core::channel::Channel;
use stwo::core::fields::qm31::SecureField;
use stwo::core::pcs::TreeVec;
use stwo_cairo_serialize::{CairoDeserialize, CairoSerialize};

use super::flat_claims::FlatClaim;
use crate::air::{
    accumulate_relation_memory, accumulate_relation_uses, PublicData, RelationUsesDict,
};
use crate::components::memory_address_to_id::MEMORY_ADDRESS_TO_ID_SPLIT;
use crate::components::*;
use crate::relations::CommonLookupElements;

#[derive(Clone, Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct CairoClaim {
    pub public_data: PublicData,
    pub add_opcode: Option<add_opcode::Claim>,
    pub add_opcode_small: Option<add_opcode_small::Claim>,
    pub add_ap_opcode: Option<add_ap_opcode::Claim>,
    pub assert_eq_opcode: Option<assert_eq_opcode::Claim>,
    pub assert_eq_opcode_imm: Option<assert_eq_opcode_imm::Claim>,
    pub assert_eq_opcode_double_deref: Option<assert_eq_opcode_double_deref::Claim>,
    pub blake_compress_opcode: Option<blake_compress_opcode::Claim>,
    pub call_opcode_abs: Option<call_opcode_abs::Claim>,
    pub call_opcode_rel_imm: Option<call_opcode_rel_imm::Claim>,
    pub generic_opcode: Option<generic_opcode::Claim>,
    pub jnz_opcode_non_taken: Option<jnz_opcode_non_taken::Claim>,
    pub jnz_opcode_taken: Option<jnz_opcode_taken::Claim>,
    pub jump_opcode_abs: Option<jump_opcode_abs::Claim>,
    pub jump_opcode_double_deref: Option<jump_opcode_double_deref::Claim>,
    pub jump_opcode_rel: Option<jump_opcode_rel::Claim>,
    pub jump_opcode_rel_imm: Option<jump_opcode_rel_imm::Claim>,
    pub mul_opcode: Option<mul_opcode::Claim>,
    pub mul_opcode_small: Option<mul_opcode_small::Claim>,
    pub qm_31_add_mul_opcode: Option<qm_31_add_mul_opcode::Claim>,
    pub ret_opcode: Option<ret_opcode::Claim>,
    pub verify_instruction: Option<verify_instruction::Claim>,
    pub blake_round: Option<blake_round::Claim>,
    pub blake_g: Option<blake_g::Claim>,
    pub blake_round_sigma: Option<blake_round_sigma::Claim>,
    pub triple_xor_32: Option<triple_xor_32::Claim>,
    pub verify_bitwise_xor_12: Option<verify_bitwise_xor_12::Claim>,
    pub add_mod_builtin: Option<add_mod_builtin::Claim>,
    pub bitwise_builtin: Option<bitwise_builtin::Claim>,
    pub mul_mod_builtin: Option<mul_mod_builtin::Claim>,
    pub pedersen_builtin: Option<pedersen_builtin::Claim>,
    pub pedersen_builtin_narrow_windows: Option<pedersen_builtin_narrow_windows::Claim>,
    pub poseidon_builtin: Option<poseidon_builtin::Claim>,
    pub range_check96_builtin: Option<range_check96_builtin::Claim>,
    pub range_check_builtin: Option<range_check_builtin::Claim>,
    pub pedersen_aggregator_window_bits_18: Option<pedersen_aggregator_window_bits_18::Claim>,
    pub partial_ec_mul_window_bits_18: Option<partial_ec_mul_window_bits_18::Claim>,
    pub pedersen_points_table_window_bits_18: Option<pedersen_points_table_window_bits_18::Claim>,
    pub pedersen_aggregator_window_bits_9: Option<pedersen_aggregator_window_bits_9::Claim>,
    pub partial_ec_mul_window_bits_9: Option<partial_ec_mul_window_bits_9::Claim>,
    pub pedersen_points_table_window_bits_9: Option<pedersen_points_table_window_bits_9::Claim>,
    pub poseidon_aggregator: Option<poseidon_aggregator::Claim>,
    pub poseidon_3_partial_rounds_chain: Option<poseidon_3_partial_rounds_chain::Claim>,
    pub poseidon_full_round_chain: Option<poseidon_full_round_chain::Claim>,
    pub cube_252: Option<cube_252::Claim>,
    pub poseidon_round_keys: Option<poseidon_round_keys::Claim>,
    pub range_check_252_width_27: Option<range_check_252_width_27::Claim>,
    pub memory_address_to_id: Option<memory_address_to_id::Claim>,
    pub memory_id_to_big: Option<memory_id_to_big::Claim>,
    pub range_check_6: Option<range_check_6::Claim>,
    pub range_check_8: Option<range_check_8::Claim>,
    pub range_check_11: Option<range_check_11::Claim>,
    pub range_check_12: Option<range_check_12::Claim>,
    pub range_check_18: Option<range_check_18::Claim>,
    pub range_check_20: Option<range_check_20::Claim>,
    pub range_check_4_3: Option<range_check_4_3::Claim>,
    pub range_check_4_4: Option<range_check_4_4::Claim>,
    pub range_check_9_9: Option<range_check_9_9::Claim>,
    pub range_check_7_2_5: Option<range_check_7_2_5::Claim>,
    pub range_check_3_6_6_3: Option<range_check_3_6_6_3::Claim>,
    pub range_check_4_4_4_4: Option<range_check_4_4_4_4::Claim>,
    pub range_check_3_3_3_3_3: Option<range_check_3_3_3_3_3::Claim>,
    pub verify_bitwise_xor_4: Option<verify_bitwise_xor_4::Claim>,
    pub verify_bitwise_xor_7: Option<verify_bitwise_xor_7::Claim>,
    pub verify_bitwise_xor_8: Option<verify_bitwise_xor_8::Claim>,
    pub verify_bitwise_xor_9: Option<verify_bitwise_xor_9::Claim>,
}

impl CairoClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        let flat_claim = self.flatten_claim();
        flat_claim.mix_into(channel);
    }

    pub fn accumulate_relation_uses(&self, relation_uses: &mut RelationUsesDict) {
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
        self.assert_eq_opcode.as_ref().inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                assert_eq_opcode::RELATION_USES_PER_ROW,
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
        self.blake_compress_opcode.as_ref().inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                blake_compress_opcode::RELATION_USES_PER_ROW,
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
        self.call_opcode_rel_imm.as_ref().inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                call_opcode_rel_imm::RELATION_USES_PER_ROW,
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
        self.verify_instruction.as_ref().inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                verify_instruction::RELATION_USES_PER_ROW,
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
        self.triple_xor_32.as_ref().inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                triple_xor_32::RELATION_USES_PER_ROW,
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
        self.bitwise_builtin.as_ref().inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                bitwise_builtin::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        self.mul_mod_builtin.as_ref().inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                mul_mod_builtin::RELATION_USES_PER_ROW,
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
        self.pedersen_builtin_narrow_windows.as_ref().inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                pedersen_builtin_narrow_windows::RELATION_USES_PER_ROW,
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
        self.pedersen_aggregator_window_bits_9
            .as_ref()
            .inspect(|c| {
                accumulate_relation_uses(
                    relation_uses,
                    pedersen_aggregator_window_bits_9::RELATION_USES_PER_ROW,
                    c.log_size,
                )
            });
        self.partial_ec_mul_window_bits_9.as_ref().inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                partial_ec_mul_window_bits_9::RELATION_USES_PER_ROW,
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
        self.range_check_252_width_27.as_ref().inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                range_check_252_width_27::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        self.memory_address_to_id.as_ref().inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                memory_address_to_id::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        accumulate_relation_memory(relation_uses, &self.memory_id_to_big);
    }

    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let mut log_sizes_list = vec![];
        self.add_opcode
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.add_opcode_small
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.add_ap_opcode
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.assert_eq_opcode
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.assert_eq_opcode_imm
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.assert_eq_opcode_double_deref
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.blake_compress_opcode
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.call_opcode_abs
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.call_opcode_rel_imm
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.generic_opcode
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.jnz_opcode_non_taken
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.jnz_opcode_taken
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.jump_opcode_abs
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.jump_opcode_double_deref
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.jump_opcode_rel
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.jump_opcode_rel_imm
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.mul_opcode
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.mul_opcode_small
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.qm_31_add_mul_opcode
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.ret_opcode
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.verify_instruction
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.blake_round
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.blake_g
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.blake_round_sigma
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.triple_xor_32
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.verify_bitwise_xor_12
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.add_mod_builtin
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.bitwise_builtin
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.mul_mod_builtin
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.pedersen_builtin
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.pedersen_builtin_narrow_windows
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.poseidon_builtin
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.range_check96_builtin
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.range_check_builtin
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.pedersen_aggregator_window_bits_18
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.partial_ec_mul_window_bits_18
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.pedersen_points_table_window_bits_18
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.pedersen_aggregator_window_bits_9
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.partial_ec_mul_window_bits_9
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.pedersen_points_table_window_bits_9
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.poseidon_aggregator
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.poseidon_3_partial_rounds_chain
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.poseidon_full_round_chain
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.cube_252
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.poseidon_round_keys
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.range_check_252_width_27
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.memory_address_to_id
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.memory_id_to_big
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.range_check_6
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.range_check_8
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.range_check_11
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.range_check_12
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.range_check_18
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.range_check_20
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.range_check_4_3
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.range_check_4_4
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.range_check_9_9
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.range_check_7_2_5
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.range_check_3_6_6_3
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.range_check_4_4_4_4
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.range_check_3_3_3_3_3
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.verify_bitwise_xor_4
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.verify_bitwise_xor_7
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.verify_bitwise_xor_8
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.verify_bitwise_xor_9
            .as_ref()
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        TreeVec::concat_cols(log_sizes_list.into_iter())
    }

    pub fn flatten_claim(&self) -> FlatClaim {
        let mut component_enable_bits = vec![];
        let mut component_log_sizes = vec![];

        if let Some(c) = self.add_opcode {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = self.add_opcode_small {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = self.add_ap_opcode {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = self.assert_eq_opcode {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = self.assert_eq_opcode_imm {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = self.assert_eq_opcode_double_deref {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = self.blake_compress_opcode {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = self.call_opcode_abs {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = self.call_opcode_rel_imm {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = self.generic_opcode {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = self.jnz_opcode_non_taken {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = self.jnz_opcode_taken {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = self.jump_opcode_abs {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = self.jump_opcode_double_deref {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = self.jump_opcode_rel {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = self.jump_opcode_rel_imm {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = self.mul_opcode {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = self.mul_opcode_small {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = self.qm_31_add_mul_opcode {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = self.ret_opcode {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = self.verify_instruction {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = self.blake_round {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = self.blake_g {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(_c) = self.blake_round_sigma {
            component_log_sizes.push(blake_round_sigma::LOG_SIZE);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = self.triple_xor_32 {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(_c) = self.verify_bitwise_xor_12 {
            component_log_sizes.push(verify_bitwise_xor_12::LOG_SIZE);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = self.add_mod_builtin {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = self.bitwise_builtin {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = self.mul_mod_builtin {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = self.pedersen_builtin {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = self.pedersen_builtin_narrow_windows {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = self.poseidon_builtin {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = self.range_check96_builtin {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = self.range_check_builtin {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = self.pedersen_aggregator_window_bits_18 {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = self.partial_ec_mul_window_bits_18 {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(_c) = self.pedersen_points_table_window_bits_18 {
            component_log_sizes.push(pedersen_points_table_window_bits_18::LOG_SIZE);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = self.pedersen_aggregator_window_bits_9 {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = self.partial_ec_mul_window_bits_9 {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(_c) = self.pedersen_points_table_window_bits_9 {
            component_log_sizes.push(pedersen_points_table_window_bits_9::LOG_SIZE);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = self.poseidon_aggregator {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = self.poseidon_3_partial_rounds_chain {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = self.poseidon_full_round_chain {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = self.cube_252 {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(_c) = self.poseidon_round_keys {
            component_log_sizes.push(poseidon_round_keys::LOG_SIZE);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = self.range_check_252_width_27 {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = self.memory_address_to_id {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        let memory_id_to_big::Claim {
            big_log_sizes,
            small_log_size,
        } = self.memory_id_to_big.as_ref().unwrap();
        assert!(big_log_sizes.len() <= MEMORY_ADDRESS_TO_ID_SPLIT);
        for log_size in big_log_sizes {
            component_log_sizes.push(*log_size);
            component_enable_bits.push(true);
        }
        for _ in 0..(MEMORY_ADDRESS_TO_ID_SPLIT - big_log_sizes.len()) {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        component_log_sizes.push(*small_log_size);
        component_enable_bits.push(true);
        if let Some(_c) = self.range_check_6 {
            component_log_sizes.push(range_check_6::LOG_SIZE);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(_c) = self.range_check_8 {
            component_log_sizes.push(range_check_8::LOG_SIZE);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(_c) = self.range_check_11 {
            component_log_sizes.push(range_check_11::LOG_SIZE);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(_c) = self.range_check_12 {
            component_log_sizes.push(range_check_12::LOG_SIZE);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(_c) = self.range_check_18 {
            component_log_sizes.push(range_check_18::LOG_SIZE);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(_c) = self.range_check_20 {
            component_log_sizes.push(range_check_20::LOG_SIZE);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(_c) = self.range_check_4_3 {
            component_log_sizes.push(range_check_4_3::LOG_SIZE);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(_c) = self.range_check_4_4 {
            component_log_sizes.push(range_check_4_4::LOG_SIZE);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(_c) = self.range_check_9_9 {
            component_log_sizes.push(range_check_9_9::LOG_SIZE);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(_c) = self.range_check_7_2_5 {
            component_log_sizes.push(range_check_7_2_5::LOG_SIZE);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(_c) = self.range_check_3_6_6_3 {
            component_log_sizes.push(range_check_3_6_6_3::LOG_SIZE);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(_c) = self.range_check_4_4_4_4 {
            component_log_sizes.push(range_check_4_4_4_4::LOG_SIZE);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(_c) = self.range_check_3_3_3_3_3 {
            component_log_sizes.push(range_check_3_3_3_3_3::LOG_SIZE);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(_c) = self.verify_bitwise_xor_4 {
            component_log_sizes.push(verify_bitwise_xor_4::LOG_SIZE);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(_c) = self.verify_bitwise_xor_7 {
            component_log_sizes.push(verify_bitwise_xor_7::LOG_SIZE);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(_c) = self.verify_bitwise_xor_8 {
            component_log_sizes.push(verify_bitwise_xor_8::LOG_SIZE);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(_c) = self.verify_bitwise_xor_9 {
            component_log_sizes.push(verify_bitwise_xor_9::LOG_SIZE);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }

        FlatClaim {
            component_enable_bits,
            component_log_sizes,
            public_data: self.public_data.clone(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct CairoInteractionClaim {
    pub add_opcode: Option<add_opcode::InteractionClaim>,
    pub add_opcode_small: Option<add_opcode_small::InteractionClaim>,
    pub add_ap_opcode: Option<add_ap_opcode::InteractionClaim>,
    pub assert_eq_opcode: Option<assert_eq_opcode::InteractionClaim>,
    pub assert_eq_opcode_imm: Option<assert_eq_opcode_imm::InteractionClaim>,
    pub assert_eq_opcode_double_deref: Option<assert_eq_opcode_double_deref::InteractionClaim>,
    pub blake_compress_opcode: Option<blake_compress_opcode::InteractionClaim>,
    pub call_opcode_abs: Option<call_opcode_abs::InteractionClaim>,
    pub call_opcode_rel_imm: Option<call_opcode_rel_imm::InteractionClaim>,
    pub generic_opcode: Option<generic_opcode::InteractionClaim>,
    pub jnz_opcode_non_taken: Option<jnz_opcode_non_taken::InteractionClaim>,
    pub jnz_opcode_taken: Option<jnz_opcode_taken::InteractionClaim>,
    pub jump_opcode_abs: Option<jump_opcode_abs::InteractionClaim>,
    pub jump_opcode_double_deref: Option<jump_opcode_double_deref::InteractionClaim>,
    pub jump_opcode_rel: Option<jump_opcode_rel::InteractionClaim>,
    pub jump_opcode_rel_imm: Option<jump_opcode_rel_imm::InteractionClaim>,
    pub mul_opcode: Option<mul_opcode::InteractionClaim>,
    pub mul_opcode_small: Option<mul_opcode_small::InteractionClaim>,
    pub qm_31_add_mul_opcode: Option<qm_31_add_mul_opcode::InteractionClaim>,
    pub ret_opcode: Option<ret_opcode::InteractionClaim>,
    pub verify_instruction: Option<verify_instruction::InteractionClaim>,
    pub blake_round: Option<blake_round::InteractionClaim>,
    pub blake_g: Option<blake_g::InteractionClaim>,
    pub blake_round_sigma: Option<blake_round_sigma::InteractionClaim>,
    pub triple_xor_32: Option<triple_xor_32::InteractionClaim>,
    pub verify_bitwise_xor_12: Option<verify_bitwise_xor_12::InteractionClaim>,
    pub add_mod_builtin: Option<add_mod_builtin::InteractionClaim>,
    pub bitwise_builtin: Option<bitwise_builtin::InteractionClaim>,
    pub mul_mod_builtin: Option<mul_mod_builtin::InteractionClaim>,
    pub pedersen_builtin: Option<pedersen_builtin::InteractionClaim>,
    pub pedersen_builtin_narrow_windows: Option<pedersen_builtin_narrow_windows::InteractionClaim>,
    pub poseidon_builtin: Option<poseidon_builtin::InteractionClaim>,
    pub range_check96_builtin: Option<range_check96_builtin::InteractionClaim>,
    pub range_check_builtin: Option<range_check_builtin::InteractionClaim>,
    pub pedersen_aggregator_window_bits_18:
        Option<pedersen_aggregator_window_bits_18::InteractionClaim>,
    pub partial_ec_mul_window_bits_18: Option<partial_ec_mul_window_bits_18::InteractionClaim>,
    pub pedersen_points_table_window_bits_18:
        Option<pedersen_points_table_window_bits_18::InteractionClaim>,
    pub pedersen_aggregator_window_bits_9:
        Option<pedersen_aggregator_window_bits_9::InteractionClaim>,
    pub partial_ec_mul_window_bits_9: Option<partial_ec_mul_window_bits_9::InteractionClaim>,
    pub pedersen_points_table_window_bits_9:
        Option<pedersen_points_table_window_bits_9::InteractionClaim>,
    pub poseidon_aggregator: Option<poseidon_aggregator::InteractionClaim>,
    pub poseidon_3_partial_rounds_chain: Option<poseidon_3_partial_rounds_chain::InteractionClaim>,
    pub poseidon_full_round_chain: Option<poseidon_full_round_chain::InteractionClaim>,
    pub cube_252: Option<cube_252::InteractionClaim>,
    pub poseidon_round_keys: Option<poseidon_round_keys::InteractionClaim>,
    pub range_check_252_width_27: Option<range_check_252_width_27::InteractionClaim>,
    pub memory_address_to_id: Option<memory_address_to_id::InteractionClaim>,
    pub memory_id_to_big: Option<memory_id_to_big::InteractionClaim>,
    pub range_check_6: Option<range_check_6::InteractionClaim>,
    pub range_check_8: Option<range_check_8::InteractionClaim>,
    pub range_check_11: Option<range_check_11::InteractionClaim>,
    pub range_check_12: Option<range_check_12::InteractionClaim>,
    pub range_check_18: Option<range_check_18::InteractionClaim>,
    pub range_check_20: Option<range_check_20::InteractionClaim>,
    pub range_check_4_3: Option<range_check_4_3::InteractionClaim>,
    pub range_check_4_4: Option<range_check_4_4::InteractionClaim>,
    pub range_check_9_9: Option<range_check_9_9::InteractionClaim>,
    pub range_check_7_2_5: Option<range_check_7_2_5::InteractionClaim>,
    pub range_check_3_6_6_3: Option<range_check_3_6_6_3::InteractionClaim>,
    pub range_check_4_4_4_4: Option<range_check_4_4_4_4::InteractionClaim>,
    pub range_check_3_3_3_3_3: Option<range_check_3_3_3_3_3::InteractionClaim>,
    pub verify_bitwise_xor_4: Option<verify_bitwise_xor_4::InteractionClaim>,
    pub verify_bitwise_xor_7: Option<verify_bitwise_xor_7::InteractionClaim>,
    pub verify_bitwise_xor_8: Option<verify_bitwise_xor_8::InteractionClaim>,
    pub verify_bitwise_xor_9: Option<verify_bitwise_xor_9::InteractionClaim>,
}

impl CairoInteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        let claim = self.flatten_interaction_claim();
        channel.mix_felts(claim.as_slice());
    }

    /// Extracts the claimed sums from a [CairoInteractionClaim].
    /// Returns a vector of all claimed sums for the logup argument, one per component.
    /// The order must match the order of components as they appear in
    /// [cairo_air::air::CairoComponents].
    pub fn flatten_interaction_claim(&self) -> Vec<SecureField> {
        let mut claimed_sums = vec![];

        if let Some(c) = self.add_opcode {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.add_opcode_small {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.add_ap_opcode {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.assert_eq_opcode {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.assert_eq_opcode_imm {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.assert_eq_opcode_double_deref {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.blake_compress_opcode {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.call_opcode_abs {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.call_opcode_rel_imm {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.generic_opcode {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.jnz_opcode_non_taken {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.jnz_opcode_taken {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.jump_opcode_abs {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.jump_opcode_double_deref {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.jump_opcode_rel {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.jump_opcode_rel_imm {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.mul_opcode {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.mul_opcode_small {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.qm_31_add_mul_opcode {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.ret_opcode {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.verify_instruction {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.blake_round {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.blake_g {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.blake_round_sigma {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.triple_xor_32 {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.verify_bitwise_xor_12 {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.add_mod_builtin {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.bitwise_builtin {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.mul_mod_builtin {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.pedersen_builtin {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.pedersen_builtin_narrow_windows {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.poseidon_builtin {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.range_check96_builtin {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.range_check_builtin {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.pedersen_aggregator_window_bits_18 {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.partial_ec_mul_window_bits_18 {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.pedersen_points_table_window_bits_18 {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.pedersen_aggregator_window_bits_9 {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.partial_ec_mul_window_bits_9 {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.pedersen_points_table_window_bits_9 {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.poseidon_aggregator {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.poseidon_3_partial_rounds_chain {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.poseidon_full_round_chain {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.cube_252 {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.poseidon_round_keys {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.range_check_252_width_27 {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.memory_address_to_id {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        let memory_id_to_big::InteractionClaim {
            big_claimed_sums,
            small_claimed_sum,
            claimed_sum: _,
        } = self.memory_id_to_big.as_ref().unwrap();
        assert!(big_claimed_sums.len() <= MEMORY_ADDRESS_TO_ID_SPLIT);
        for claimed_sum in big_claimed_sums {
            claimed_sums.push(*claimed_sum);
        }
        for _ in 0..(MEMORY_ADDRESS_TO_ID_SPLIT - big_claimed_sums.len()) {
            claimed_sums.push(SecureField::zero());
        }
        claimed_sums.push(*small_claimed_sum);
        if let Some(c) = self.range_check_6 {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.range_check_8 {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.range_check_11 {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.range_check_12 {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.range_check_18 {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.range_check_20 {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.range_check_4_3 {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.range_check_4_4 {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.range_check_9_9 {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.range_check_7_2_5 {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.range_check_3_6_6_3 {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.range_check_4_4_4_4 {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.range_check_3_3_3_3_3 {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.verify_bitwise_xor_4 {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.verify_bitwise_xor_7 {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.verify_bitwise_xor_8 {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }
        if let Some(c) = self.verify_bitwise_xor_9 {
            claimed_sums.push(c.claimed_sum);
        } else {
            claimed_sums.push(SecureField::zero());
        }

        claimed_sums
    }
}

pub fn lookup_sum(
    claim: &CairoClaim,
    common_lookup_elements: &CommonLookupElements,
    interaction_claim: &CairoInteractionClaim,
) -> SecureField {
    let mut sum = SecureField::zero();
    sum += claim.public_data.logup_sum(common_lookup_elements);
    interaction_claim.add_opcode.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim.add_opcode_small.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim.add_ap_opcode.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim.assert_eq_opcode.as_ref().inspect(|ic| {
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
    interaction_claim
        .blake_compress_opcode
        .as_ref()
        .inspect(|ic| {
            sum += ic.claimed_sum;
        });
    interaction_claim.call_opcode_abs.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim
        .call_opcode_rel_imm
        .as_ref()
        .inspect(|ic| {
            sum += ic.claimed_sum;
        });
    interaction_claim.generic_opcode.as_ref().inspect(|ic| {
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
    interaction_claim.mul_opcode.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim.mul_opcode_small.as_ref().inspect(|ic| {
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
    interaction_claim.verify_instruction.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim.blake_round.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim.blake_g.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim.blake_round_sigma.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim.triple_xor_32.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim
        .verify_bitwise_xor_12
        .as_ref()
        .inspect(|ic| {
            sum += ic.claimed_sum;
        });
    interaction_claim.add_mod_builtin.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim.bitwise_builtin.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim.mul_mod_builtin.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim.pedersen_builtin.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim
        .pedersen_builtin_narrow_windows
        .as_ref()
        .inspect(|ic| {
            sum += ic.claimed_sum;
        });
    interaction_claim.poseidon_builtin.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim
        .range_check96_builtin
        .as_ref()
        .inspect(|ic| {
            sum += ic.claimed_sum;
        });
    interaction_claim
        .range_check_builtin
        .as_ref()
        .inspect(|ic| {
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
    interaction_claim
        .pedersen_aggregator_window_bits_9
        .as_ref()
        .inspect(|ic| {
            sum += ic.claimed_sum;
        });
    interaction_claim
        .partial_ec_mul_window_bits_9
        .as_ref()
        .inspect(|ic| {
            sum += ic.claimed_sum;
        });
    interaction_claim
        .pedersen_points_table_window_bits_9
        .as_ref()
        .inspect(|ic| {
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
    interaction_claim
        .poseidon_full_round_chain
        .as_ref()
        .inspect(|ic| {
            sum += ic.claimed_sum;
        });
    interaction_claim.cube_252.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim
        .poseidon_round_keys
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
        .memory_address_to_id
        .as_ref()
        .inspect(|ic| {
            sum += ic.claimed_sum;
        });
    interaction_claim.memory_id_to_big.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim.range_check_6.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim.range_check_8.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim.range_check_11.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim.range_check_12.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim.range_check_18.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim.range_check_20.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim.range_check_4_3.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim.range_check_4_4.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim.range_check_9_9.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim.range_check_7_2_5.as_ref().inspect(|ic| {
        sum += ic.claimed_sum;
    });
    interaction_claim
        .range_check_3_6_6_3
        .as_ref()
        .inspect(|ic| {
            sum += ic.claimed_sum;
        });
    interaction_claim
        .range_check_4_4_4_4
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
        .verify_bitwise_xor_4
        .as_ref()
        .inspect(|ic| {
            sum += ic.claimed_sum;
        });
    interaction_claim
        .verify_bitwise_xor_7
        .as_ref()
        .inspect(|ic| {
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
    sum
}
