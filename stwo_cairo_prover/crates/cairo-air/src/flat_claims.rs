// This file was created by the AIR team.

use num_traits::Zero;
use stwo::core::channel::Channel;
use stwo::core::fields::qm31::SecureField;

use crate::air::PublicData;
use crate::claims::{CairoClaim, CairoInteractionClaim};
use crate::components::memory_address_to_id::MEMORY_ADDRESS_TO_ID_SPLIT;
use crate::components::{
    blake_round_sigma, pedersen_points_table_window_bits_18, poseidon_round_keys, range_check_11,
    range_check_12, range_check_18, range_check_20, range_check_3_3_3_3_3, range_check_3_6_6_3,
    range_check_4_3, range_check_4_4, range_check_4_4_4_4, range_check_6, range_check_7_2_5,
    range_check_8, range_check_9_9, verify_bitwise_xor_12, verify_bitwise_xor_4,
    verify_bitwise_xor_7, verify_bitwise_xor_8, verify_bitwise_xor_9, *,
};
use crate::utils::pack_into_secure_felts;

pub struct FlatClaim {
    pub component_enable_bits: Vec<bool>,
    pub component_log_sizes: Vec<u32>,
    pub public_data: PublicData,
}

impl FlatClaim {
    pub fn from_cairo_claim(claim: &CairoClaim) -> Self {
        let mut component_enable_bits = vec![];
        let mut component_log_sizes = vec![];

        if let Some(c) = &claim.add_opcode {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = &claim.add_opcode_small {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = &claim.add_ap_opcode {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = &claim.assert_eq_opcode {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = &claim.assert_eq_opcode_imm {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = &claim.assert_eq_opcode_double_deref {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = &claim.blake_compress_opcode {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = &claim.call_opcode_abs {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = &claim.call_opcode_rel_imm {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = &claim.generic_opcode {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = &claim.jnz_opcode_non_taken {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = &claim.jnz_opcode_taken {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = &claim.jump_opcode_abs {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = &claim.jump_opcode_double_deref {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = &claim.jump_opcode_rel {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = &claim.jump_opcode_rel_imm {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = &claim.mul_opcode {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = &claim.mul_opcode_small {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = &claim.qm_31_add_mul_opcode {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = &claim.ret_opcode {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = &claim.verify_instruction {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = &claim.blake_round {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = &claim.blake_g {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(_c) = &claim.blake_round_sigma {
            component_log_sizes.push(blake_round_sigma::LOG_SIZE);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = &claim.triple_xor_32 {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(_c) = &claim.verify_bitwise_xor_12 {
            component_log_sizes.push(verify_bitwise_xor_12::LOG_SIZE);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = &claim.add_mod_builtin {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = &claim.bitwise_builtin {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = &claim.mul_mod_builtin {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = &claim.pedersen_builtin {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = &claim.poseidon_builtin {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = &claim.range_check96_builtin {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = &claim.range_check_builtin {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = &claim.pedersen_aggregator_window_bits_18 {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = &claim.partial_ec_mul_window_bits_18 {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(_c) = &claim.pedersen_points_table_window_bits_18 {
            component_log_sizes.push(pedersen_points_table_window_bits_18::LOG_SIZE);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = &claim.poseidon_aggregator {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = &claim.poseidon_3_partial_rounds_chain {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = &claim.poseidon_full_round_chain {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = &claim.cube_252 {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(_c) = &claim.poseidon_round_keys {
            component_log_sizes.push(poseidon_round_keys::LOG_SIZE);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = &claim.range_check_252_width_27 {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(c) = &claim.memory_address_to_id {
            component_log_sizes.push(c.log_size);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        let memory_id_to_big::Claim {
            big_log_sizes,
            small_log_size,
        } = claim.memory_id_to_big.as_ref().unwrap();
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
        if let Some(_c) = &claim.range_check_6 {
            component_log_sizes.push(range_check_6::LOG_SIZE);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(_c) = &claim.range_check_8 {
            component_log_sizes.push(range_check_8::LOG_SIZE);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(_c) = &claim.range_check_11 {
            component_log_sizes.push(range_check_11::LOG_SIZE);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(_c) = &claim.range_check_12 {
            component_log_sizes.push(range_check_12::LOG_SIZE);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(_c) = &claim.range_check_18 {
            component_log_sizes.push(range_check_18::LOG_SIZE);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(_c) = &claim.range_check_20 {
            component_log_sizes.push(range_check_20::LOG_SIZE);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(_c) = &claim.range_check_4_3 {
            component_log_sizes.push(range_check_4_3::LOG_SIZE);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(_c) = &claim.range_check_4_4 {
            component_log_sizes.push(range_check_4_4::LOG_SIZE);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(_c) = &claim.range_check_9_9 {
            component_log_sizes.push(range_check_9_9::LOG_SIZE);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(_c) = &claim.range_check_7_2_5 {
            component_log_sizes.push(range_check_7_2_5::LOG_SIZE);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(_c) = &claim.range_check_3_6_6_3 {
            component_log_sizes.push(range_check_3_6_6_3::LOG_SIZE);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(_c) = &claim.range_check_4_4_4_4 {
            component_log_sizes.push(range_check_4_4_4_4::LOG_SIZE);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(_c) = &claim.range_check_3_3_3_3_3 {
            component_log_sizes.push(range_check_3_3_3_3_3::LOG_SIZE);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(_c) = &claim.verify_bitwise_xor_4 {
            component_log_sizes.push(verify_bitwise_xor_4::LOG_SIZE);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(_c) = &claim.verify_bitwise_xor_7 {
            component_log_sizes.push(verify_bitwise_xor_7::LOG_SIZE);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(_c) = &claim.verify_bitwise_xor_8 {
            component_log_sizes.push(verify_bitwise_xor_8::LOG_SIZE);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }
        if let Some(_c) = &claim.verify_bitwise_xor_9 {
            component_log_sizes.push(verify_bitwise_xor_9::LOG_SIZE);
            component_enable_bits.push(true);
        } else {
            component_log_sizes.push(0_u32);
            component_enable_bits.push(false);
        }

        Self {
            component_enable_bits,
            component_log_sizes,
            public_data: claim.public_data.clone(),
        }
    }

    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_felts(&pack_into_secure_felts(
            [self.component_enable_bits.len() as u32].into_iter(),
        ));

        // Converts enable bits to [u32], where each u32 is at most 2^31 - 1.
        channel.mix_felts(&pack_into_secure_felts(
            self.component_enable_bits
                .iter()
                .map(|&b| if b { 1 } else { 0 }),
        ));
        channel.mix_felts(&pack_into_secure_felts(
            self.component_log_sizes.iter().cloned(),
        ));
        channel.mix_felts(&pack_into_secure_felts(
            [self.public_data.public_memory.program.len() as u32].into_iter(),
        ));
        self.public_data.mix_into(channel);
    }
}

/// Extracts the claimed sums from a [CairoInteractionClaim].
/// Returns a vector of all claimed sums for the logup argument, one per component.
/// The order must match the order of components as they appear in
/// [cairo_air::air::CairoComponents].
pub fn flatten_interaction_claim(interaction_claim: &CairoInteractionClaim) -> Vec<SecureField> {
    let mut claimed_sums = vec![];

    if let Some(c) = &interaction_claim.add_opcode {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    if let Some(c) = &interaction_claim.add_opcode_small {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    if let Some(c) = &interaction_claim.add_ap_opcode {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    if let Some(c) = &interaction_claim.assert_eq_opcode {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    if let Some(c) = &interaction_claim.assert_eq_opcode_imm {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    if let Some(c) = &interaction_claim.assert_eq_opcode_double_deref {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    if let Some(c) = &interaction_claim.blake_compress_opcode {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    if let Some(c) = &interaction_claim.call_opcode_abs {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    if let Some(c) = &interaction_claim.call_opcode_rel_imm {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    if let Some(c) = &interaction_claim.generic_opcode {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    if let Some(c) = &interaction_claim.jnz_opcode_non_taken {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    if let Some(c) = &interaction_claim.jnz_opcode_taken {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    if let Some(c) = &interaction_claim.jump_opcode_abs {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    if let Some(c) = &interaction_claim.jump_opcode_double_deref {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    if let Some(c) = &interaction_claim.jump_opcode_rel {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    if let Some(c) = &interaction_claim.jump_opcode_rel_imm {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    if let Some(c) = &interaction_claim.mul_opcode {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    if let Some(c) = &interaction_claim.mul_opcode_small {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    if let Some(c) = &interaction_claim.qm_31_add_mul_opcode {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    if let Some(c) = &interaction_claim.ret_opcode {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    if let Some(c) = &interaction_claim.verify_instruction {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    if let Some(c) = &interaction_claim.blake_round {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    if let Some(c) = &interaction_claim.blake_g {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    if let Some(c) = &interaction_claim.blake_round_sigma {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    if let Some(c) = &interaction_claim.triple_xor_32 {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    if let Some(c) = &interaction_claim.verify_bitwise_xor_12 {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    if let Some(c) = &interaction_claim.add_mod_builtin {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    if let Some(c) = &interaction_claim.bitwise_builtin {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    if let Some(c) = &interaction_claim.mul_mod_builtin {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    if let Some(c) = &interaction_claim.pedersen_builtin {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    if let Some(c) = &interaction_claim.poseidon_builtin {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    if let Some(c) = &interaction_claim.range_check96_builtin {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    if let Some(c) = &interaction_claim.range_check_builtin {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    if let Some(c) = &interaction_claim.pedersen_aggregator_window_bits_18 {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    if let Some(c) = &interaction_claim.partial_ec_mul_window_bits_18 {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    if let Some(c) = &interaction_claim.pedersen_points_table_window_bits_18 {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    if let Some(c) = &interaction_claim.poseidon_aggregator {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    if let Some(c) = &interaction_claim.poseidon_3_partial_rounds_chain {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    if let Some(c) = &interaction_claim.poseidon_full_round_chain {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    if let Some(c) = &interaction_claim.cube_252 {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    if let Some(c) = &interaction_claim.poseidon_round_keys {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    if let Some(c) = &interaction_claim.range_check_252_width_27 {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    if let Some(c) = &interaction_claim.memory_address_to_id {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    let memory_id_to_big::InteractionClaim {
        big_claimed_sums,
        small_claimed_sum,
        claimed_sum: _,
    } = interaction_claim.memory_id_to_big.as_ref().unwrap();
    assert!(big_claimed_sums.len() <= MEMORY_ADDRESS_TO_ID_SPLIT);
    for claimed_sum in big_claimed_sums {
        claimed_sums.push(*claimed_sum);
    }
    for _ in 0..(MEMORY_ADDRESS_TO_ID_SPLIT - big_claimed_sums.len()) {
        claimed_sums.push(SecureField::zero());
    }
    claimed_sums.push(*small_claimed_sum);
    if let Some(c) = &interaction_claim.range_check_6 {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    if let Some(c) = &interaction_claim.range_check_8 {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    if let Some(c) = &interaction_claim.range_check_11 {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    if let Some(c) = &interaction_claim.range_check_12 {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    if let Some(c) = &interaction_claim.range_check_18 {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    if let Some(c) = &interaction_claim.range_check_20 {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    if let Some(c) = &interaction_claim.range_check_4_3 {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    if let Some(c) = &interaction_claim.range_check_4_4 {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    if let Some(c) = &interaction_claim.range_check_9_9 {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    if let Some(c) = &interaction_claim.range_check_7_2_5 {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    if let Some(c) = &interaction_claim.range_check_3_6_6_3 {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    if let Some(c) = &interaction_claim.range_check_4_4_4_4 {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    if let Some(c) = &interaction_claim.range_check_3_3_3_3_3 {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    if let Some(c) = &interaction_claim.verify_bitwise_xor_4 {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    if let Some(c) = &interaction_claim.verify_bitwise_xor_7 {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    if let Some(c) = &interaction_claim.verify_bitwise_xor_8 {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }
    if let Some(c) = &interaction_claim.verify_bitwise_xor_9 {
        claimed_sums.push(c.claimed_sum);
    } else {
        claimed_sums.push(SecureField::zero());
    }

    claimed_sums
}
