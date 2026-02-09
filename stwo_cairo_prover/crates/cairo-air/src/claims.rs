// This file was created by the AIR team.

use std::array::from_fn;

use num_traits::Zero;
use serde::{Deserialize, Serialize};
use stwo::core::channel::Channel;
use stwo::core::fields::qm31::QM31;
use stwo::core::pcs::TreeVec;
use stwo_cairo_serialize::{CairoDeserialize, CairoSerialize};

use super::flat_claims::{flatten_interaction_claim, FlatClaim};
use crate::air::{accumulate_relation_uses, PublicData, RelationUsesDict};
use crate::claims::prelude::SecureField;
use crate::components::memory_address_to_id::MEMORY_ADDRESS_TO_ID_SPLIT;
use crate::components::*;
use crate::relations::CommonLookupElements;

pub type CairoInteractionClaim = Vec<Option<QM31>>;

#[derive(Clone, Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct CairoClaim {
    pub public_data: PublicData,
    pub components_log_sizes: Vec<Option<u32>>,
}

impl CairoClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        let claim = FlatClaim::from_cairo_claim(self);
        claim.mix_into(channel);
    }

    pub fn accumulate_relation_uses(&self, relation_uses: &mut RelationUsesDict) {
        let mut iter = self.components_log_sizes.iter();
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| {
            accumulate_relation_uses(relation_uses, add_opcode::RELATION_USES_PER_ROW, *c)
        });
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| {
            accumulate_relation_uses(relation_uses, add_opcode_small::RELATION_USES_PER_ROW, *c)
        });
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| {
            accumulate_relation_uses(relation_uses, add_ap_opcode::RELATION_USES_PER_ROW, *c)
        });
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| {
            accumulate_relation_uses(relation_uses, assert_eq_opcode::RELATION_USES_PER_ROW, *c)
        });
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                assert_eq_opcode_imm::RELATION_USES_PER_ROW,
                *c,
            )
        });
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                assert_eq_opcode_double_deref::RELATION_USES_PER_ROW,
                *c,
            )
        });
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                blake_compress_opcode::RELATION_USES_PER_ROW,
                *c,
            )
        });
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| {
            accumulate_relation_uses(relation_uses, call_opcode_abs::RELATION_USES_PER_ROW, *c)
        });
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                call_opcode_rel_imm::RELATION_USES_PER_ROW,
                *c,
            )
        });
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| {
            accumulate_relation_uses(relation_uses, generic_opcode::RELATION_USES_PER_ROW, *c)
        });
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                jnz_opcode_non_taken::RELATION_USES_PER_ROW,
                *c,
            )
        });
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| {
            accumulate_relation_uses(relation_uses, jnz_opcode_taken::RELATION_USES_PER_ROW, *c)
        });
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| {
            accumulate_relation_uses(relation_uses, jump_opcode_abs::RELATION_USES_PER_ROW, *c)
        });
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                jump_opcode_double_deref::RELATION_USES_PER_ROW,
                *c,
            )
        });
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| {
            accumulate_relation_uses(relation_uses, jump_opcode_rel::RELATION_USES_PER_ROW, *c)
        });
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                jump_opcode_rel_imm::RELATION_USES_PER_ROW,
                *c,
            )
        });
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| {
            accumulate_relation_uses(relation_uses, mul_opcode::RELATION_USES_PER_ROW, *c)
        });
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| {
            accumulate_relation_uses(relation_uses, mul_opcode_small::RELATION_USES_PER_ROW, *c)
        });
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                qm_31_add_mul_opcode::RELATION_USES_PER_ROW,
                *c,
            )
        });
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| {
            accumulate_relation_uses(relation_uses, ret_opcode::RELATION_USES_PER_ROW, *c)
        });
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| {
            accumulate_relation_uses(relation_uses, verify_instruction::RELATION_USES_PER_ROW, *c)
        });
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| {
            accumulate_relation_uses(relation_uses, blake_round::RELATION_USES_PER_ROW, *c)
        });
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| {
            accumulate_relation_uses(relation_uses, blake_g::RELATION_USES_PER_ROW, *c)
        });
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| {
            accumulate_relation_uses(relation_uses, triple_xor_32::RELATION_USES_PER_ROW, *c)
        });
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| {
            accumulate_relation_uses(relation_uses, add_mod_builtin::RELATION_USES_PER_ROW, *c)
        });
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| {
            accumulate_relation_uses(relation_uses, bitwise_builtin::RELATION_USES_PER_ROW, *c)
        });
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| {
            accumulate_relation_uses(relation_uses, mul_mod_builtin::RELATION_USES_PER_ROW, *c)
        });
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| {
            accumulate_relation_uses(relation_uses, pedersen_builtin::RELATION_USES_PER_ROW, *c)
        });
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| {
            accumulate_relation_uses(relation_uses, poseidon_builtin::RELATION_USES_PER_ROW, *c)
        });
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                range_check96_builtin::RELATION_USES_PER_ROW,
                *c,
            )
        });
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                range_check_builtin::RELATION_USES_PER_ROW,
                *c,
            )
        });
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                pedersen_aggregator_window_bits_18::RELATION_USES_PER_ROW,
                *c,
            )
        });
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                partial_ec_mul_window_bits_18::RELATION_USES_PER_ROW,
                *c,
            )
        });
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                poseidon_aggregator::RELATION_USES_PER_ROW,
                *c,
            )
        });
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                poseidon_3_partial_rounds_chain::RELATION_USES_PER_ROW,
                *c,
            )
        });
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                poseidon_full_round_chain::RELATION_USES_PER_ROW,
                *c,
            )
        });
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| {
            accumulate_relation_uses(relation_uses, cube_252::RELATION_USES_PER_ROW, *c)
        });
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                range_check_252_width_27::RELATION_USES_PER_ROW,
                *c,
            )
        });
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                memory_address_to_id::RELATION_USES_PER_ROW,
                *c,
            )
        });
        let big_log_sizes: [Option<u32>; MEMORY_ADDRESS_TO_ID_SPLIT] =
            from_fn(|_| *iter.next().unwrap());
        let big_log_sizes: Vec<u32> = big_log_sizes
            .into_iter()
            .flatten()
            .collect();
        let small_log_size = iter.next().unwrap().unwrap();
        let claim = memory_id_to_big::Claim {
            big_log_sizes,
            small_log_size,
        };
        for log_size in claim.big_log_sizes {
            accumulate_relation_uses(
                relation_uses,
                memory_id_to_big::RELATION_USES_PER_ROW_BIG,
                log_size,
            );
        }
        accumulate_relation_uses(
            relation_uses,
            memory_id_to_big::RELATION_USES_PER_ROW_SMALL,
            claim.small_log_size,
        );
        assert!(iter.next().is_none());
    }

    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let mut log_sizes_list = vec![];
        let mut iter = self.components_log_sizes.iter();
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| log_sizes_list.push(add_opcode::log_sizes(*c)));
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| log_sizes_list.push(add_opcode_small::log_sizes(*c)));
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| log_sizes_list.push(add_ap_opcode::log_sizes(*c)));
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| log_sizes_list.push(assert_eq_opcode::log_sizes(*c)));
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| log_sizes_list.push(assert_eq_opcode_imm::log_sizes(*c)));
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| log_sizes_list.push(assert_eq_opcode_double_deref::log_sizes(*c)));
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| log_sizes_list.push(blake_compress_opcode::log_sizes(*c)));
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| log_sizes_list.push(call_opcode_abs::log_sizes(*c)));
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| log_sizes_list.push(call_opcode_rel_imm::log_sizes(*c)));
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| log_sizes_list.push(generic_opcode::log_sizes(*c)));
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| log_sizes_list.push(jnz_opcode_non_taken::log_sizes(*c)));
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| log_sizes_list.push(jnz_opcode_taken::log_sizes(*c)));
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| log_sizes_list.push(jump_opcode_abs::log_sizes(*c)));
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| log_sizes_list.push(jump_opcode_double_deref::log_sizes(*c)));
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| log_sizes_list.push(jump_opcode_rel::log_sizes(*c)));
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| log_sizes_list.push(jump_opcode_rel_imm::log_sizes(*c)));
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| log_sizes_list.push(mul_opcode::log_sizes(*c)));
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| log_sizes_list.push(mul_opcode_small::log_sizes(*c)));
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| log_sizes_list.push(qm_31_add_mul_opcode::log_sizes(*c)));
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| log_sizes_list.push(ret_opcode::log_sizes(*c)));
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| log_sizes_list.push(verify_instruction::log_sizes(*c)));
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| log_sizes_list.push(blake_round::log_sizes(*c)));
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| log_sizes_list.push(blake_g::log_sizes(*c)));
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| log_sizes_list.push(blake_round_sigma::log_sizes(*c)));
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| log_sizes_list.push(triple_xor_32::log_sizes(*c)));
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| log_sizes_list.push(verify_bitwise_xor_12::log_sizes(*c)));
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| log_sizes_list.push(add_mod_builtin::log_sizes(*c)));
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| log_sizes_list.push(bitwise_builtin::log_sizes(*c)));
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| log_sizes_list.push(mul_mod_builtin::log_sizes(*c)));
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| log_sizes_list.push(pedersen_builtin::log_sizes(*c)));
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| log_sizes_list.push(poseidon_builtin::log_sizes(*c)));
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| log_sizes_list.push(range_check96_builtin::log_sizes(*c)));
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| log_sizes_list.push(range_check_builtin::log_sizes(*c)));
        let log_size = iter.next().unwrap();
        log_size
            .inspect(|c| log_sizes_list.push(pedersen_aggregator_window_bits_18::log_sizes(*c)));
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| log_sizes_list.push(partial_ec_mul_window_bits_18::log_sizes(*c)));
        let log_size = iter.next().unwrap();
        log_size
            .inspect(|c| log_sizes_list.push(pedersen_points_table_window_bits_18::log_sizes(*c)));
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| log_sizes_list.push(poseidon_aggregator::log_sizes(*c)));
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| log_sizes_list.push(poseidon_3_partial_rounds_chain::log_sizes(*c)));
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| log_sizes_list.push(poseidon_full_round_chain::log_sizes(*c)));
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| log_sizes_list.push(cube_252::log_sizes(*c)));
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| log_sizes_list.push(poseidon_round_keys::log_sizes(*c)));
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| log_sizes_list.push(range_check_252_width_27::log_sizes(*c)));
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| log_sizes_list.push(memory_address_to_id::log_sizes(*c)));
        let big_log_sizes: [Option<u32>; MEMORY_ADDRESS_TO_ID_SPLIT] =
            from_fn(|_| *iter.next().unwrap());
        let big_log_sizes: Vec<u32> = big_log_sizes
            .into_iter()
            .flatten()
            .collect();
        let small_log_size = iter.next().unwrap().unwrap();
        let claim = memory_id_to_big::Claim {
            big_log_sizes,
            small_log_size,
        };
        log_sizes_list.push(claim.log_sizes());
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| log_sizes_list.push(range_check_6::log_sizes(*c)));
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| log_sizes_list.push(range_check_8::log_sizes(*c)));
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| log_sizes_list.push(range_check_11::log_sizes(*c)));
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| log_sizes_list.push(range_check_12::log_sizes(*c)));
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| log_sizes_list.push(range_check_18::log_sizes(*c)));
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| log_sizes_list.push(range_check_20::log_sizes(*c)));
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| log_sizes_list.push(range_check_4_3::log_sizes(*c)));
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| log_sizes_list.push(range_check_4_4::log_sizes(*c)));
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| log_sizes_list.push(range_check_9_9::log_sizes(*c)));
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| log_sizes_list.push(range_check_7_2_5::log_sizes(*c)));
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| log_sizes_list.push(range_check_3_6_6_3::log_sizes(*c)));
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| log_sizes_list.push(range_check_4_4_4_4::log_sizes(*c)));
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| log_sizes_list.push(range_check_3_3_3_3_3::log_sizes(*c)));
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| log_sizes_list.push(verify_bitwise_xor_4::log_sizes(*c)));
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| log_sizes_list.push(verify_bitwise_xor_7::log_sizes(*c)));
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| log_sizes_list.push(verify_bitwise_xor_8::log_sizes(*c)));
        let log_size = iter.next().unwrap();
        log_size.inspect(|c| log_sizes_list.push(verify_bitwise_xor_9::log_sizes(*c)));
        assert!(iter.next().is_none());
        TreeVec::concat_cols(log_sizes_list.into_iter())
    }
}

pub fn interaction_claim_mix_into(interaction_claim: &Vec<Option<QM31>>, channel: &mut impl Channel) {
    let claim = flatten_interaction_claim(interaction_claim);
    channel.mix_felts(claim.as_slice());
}

pub fn lookup_sum(
    claim: &CairoClaim,
    common_lookup_elements: &CommonLookupElements,
    interaction_claim: &CairoInteractionClaim,
) -> SecureField {
    let mut sum = QM31::zero();
    sum += claim.public_data.logup_sum(common_lookup_elements);
    for claimed_sum in interaction_claim
        .into_iter()
        .flatten()
    {
        sum += *claimed_sum;
    }
    sum
}
