use num_traits::Zero;
use stwo::core::channel::Channel;
use stwo::core::fields::qm31::SecureField;

use crate::air::PublicData;
use crate::claims::{CairoClaim, CairoInteractionClaim};
use crate::components::memory_address_to_id::MEMORY_ADDRESS_TO_ID_SPLIT;
use crate::components::{
    blake_round_sigma, memory_id_to_big, pedersen_points_table_window_bits_18,
    pedersen_points_table_window_bits_9, poseidon_round_keys, range_check_11, range_check_12,
    range_check_18, range_check_20, range_check_3_3_3_3_3, range_check_3_6_6_3, range_check_4_3,
    range_check_4_4, range_check_4_4_4_4, range_check_6, range_check_7_2_5, range_check_8,
    range_check_9_9, verify_bitwise_xor_12, verify_bitwise_xor_4, verify_bitwise_xor_7,
    verify_bitwise_xor_8, verify_bitwise_xor_9,
};
use crate::utils::pack_into_secure_felts;

pub struct FlatClaim {
    pub component_enable_bits: Vec<bool>,
    pub component_log_sizes: Vec<u32>,
    pub public_data: PublicData,
}
impl FlatClaim {
    pub fn from_cairo_claim(claim: &CairoClaim) -> Self {
        let (component_enable_bits, component_log_sizes) = flatten_claim(claim);
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
        channel.mix_felts(&pack_into_secure_felts(
            enable_bits_to_u32s(&self.component_enable_bits).into_iter(),
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

/// Converts enable bits to [u32], where each u32 is at most 2^31 - 1.
fn enable_bits_to_u32s(enable_bits: &[bool]) -> Vec<u32> {
    enable_bits.iter().map(|&b| if b { 1 } else { 0 }).collect()
}

/// Extracts component enable bits, and component log sizes from a [CairoClaim] and returns it as
/// vectors of [bool] and [u32] respectively.
/// The order must match the order of components as they appear in
/// [cairo_air::air::CairoComponents].
fn flatten_claim(claim: &CairoClaim) -> (Vec<bool>, Vec<u32>) {
    let mut component_enable_bits = vec![];
    let mut component_log_sizes = vec![];

    // Opcodes
    option_log_size(
        &claim.add_opcode,
        |c| c.log_size,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );
    option_log_size(
        &claim.add_opcode_small,
        |c| c.log_size,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );
    option_log_size(
        &claim.add_ap_opcode,
        |c| c.log_size,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );
    option_log_size(
        &claim.assert_eq_opcode,
        |c| c.log_size,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );
    option_log_size(
        &claim.assert_eq_opcode_imm,
        |c| c.log_size,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );
    option_log_size(
        &claim.assert_eq_opcode_double_deref,
        |c| c.log_size,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );
    option_log_size(
        &claim.blake_compress_opcode,
        |c| c.log_size,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );
    option_log_size(
        &claim.call_opcode_abs,
        |c| c.log_size,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );
    option_log_size(
        &claim.call_opcode_rel_imm,
        |c| c.log_size,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );
    option_log_size(
        &claim.generic_opcode,
        |c| c.log_size,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );
    option_log_size(
        &claim.jnz_opcode_non_taken,
        |c| c.log_size,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );
    option_log_size(
        &claim.jnz_opcode_taken,
        |c| c.log_size,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );
    option_log_size(
        &claim.jump_opcode_abs,
        |c| c.log_size,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );
    option_log_size(
        &claim.jump_opcode_double_deref,
        |c| c.log_size,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );
    option_log_size(
        &claim.jump_opcode_rel,
        |c| c.log_size,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );
    option_log_size(
        &claim.jump_opcode_rel_imm,
        |c| c.log_size,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );
    option_log_size(
        &claim.mul_opcode,
        |c| c.log_size,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );
    option_log_size(
        &claim.mul_opcode_small,
        |c| c.log_size,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );
    option_log_size(
        &claim.qm_31_add_mul_opcode,
        |c| c.log_size,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );
    option_log_size(
        &claim.ret_opcode,
        |c| c.log_size,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );

    // Verify instruction
    option_log_size(
        &claim.verify_instruction,
        |c| c.log_size,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );

    // Blake
    option_log_size(
        &claim.blake_round,
        |c| c.log_size,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );
    option_log_size(
        &claim.blake_g,
        |c| c.log_size,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );
    option_log_size(
        &claim.blake_round_sigma,
        |_| blake_round_sigma::LOG_SIZE,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );
    option_log_size(
        &claim.triple_xor_32,
        |c| c.log_size,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );
    option_log_size(
        &claim.verify_bitwise_xor_12,
        |_| verify_bitwise_xor_12::LOG_SIZE,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );

    // Builtins
    option_log_size(
        &claim.add_mod_builtin,
        |c| c.log_size,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );
    option_log_size(
        &claim.bitwise_builtin,
        |c| c.log_size,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );

    option_log_size(
        &claim.mul_mod_builtin,
        |c| c.log_size,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );
    option_log_size(
        &claim.pedersen_builtin,
        |c| c.log_size,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );
    option_log_size(
        &claim.pedersen_builtin_narrow_windows,
        |c| c.log_size,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );
    option_log_size(
        &claim.poseidon_builtin,
        |c| c.log_size,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );
    option_log_size(
        &claim.range_check96_builtin,
        |c| c.log_size,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );
    option_log_size(
        &claim.range_check_builtin,
        |c| c.log_size,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );

    // Pedersen context
    option_log_size(
        &claim.pedersen_aggregator_window_bits_18,
        |c| c.log_size,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );
    option_log_size(
        &claim.partial_ec_mul_window_bits_18,
        |c| c.log_size,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );
    option_log_size(
        &claim.pedersen_points_table_window_bits_18,
        |_| pedersen_points_table_window_bits_18::LOG_SIZE,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );

    // Pedersen narrow windows context
    option_log_size(
        &claim.pedersen_aggregator_window_bits_9,
        |c| c.log_size,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );
    option_log_size(
        &claim.partial_ec_mul_window_bits_9,
        |c| c.log_size,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );
    option_log_size(
        &claim.pedersen_points_table_window_bits_9,
        |_| pedersen_points_table_window_bits_9::LOG_SIZE,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );

    // Poseidon context
    option_log_size(
        &claim.poseidon_aggregator,
        |c| c.log_size,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );
    option_log_size(
        &claim.poseidon_3_partial_rounds_chain,
        |c| c.log_size,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );
    option_log_size(
        &claim.poseidon_full_round_chain,
        |c| c.log_size,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );
    option_log_size(
        &claim.cube_252,
        |c| c.log_size,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );
    option_log_size(
        &claim.poseidon_round_keys,
        |_| poseidon_round_keys::LOG_SIZE,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );
    option_log_size(
        &claim.range_check_252_width_27,
        |c| c.log_size,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );

    // Memory
    option_log_size(
        &claim.memory_address_to_id,
        |c| c.log_size,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );

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

    // Range checks
    option_log_size(
        &claim.range_check_6,
        |_| range_check_6::LOG_SIZE,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );
    option_log_size(
        &claim.range_check_8,
        |_| range_check_8::LOG_SIZE,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );
    option_log_size(
        &claim.range_check_11,
        |_| range_check_11::LOG_SIZE,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );
    option_log_size(
        &claim.range_check_12,
        |_| range_check_12::LOG_SIZE,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );
    option_log_size(
        &claim.range_check_18,
        |_| range_check_18::LOG_SIZE,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );
    option_log_size(
        &claim.range_check_20,
        |_| range_check_20::LOG_SIZE,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );
    option_log_size(
        &claim.range_check_4_3,
        |_| range_check_4_3::LOG_SIZE,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );
    option_log_size(
        &claim.range_check_4_4,
        |_| range_check_4_4::LOG_SIZE,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );
    option_log_size(
        &claim.range_check_9_9,
        |_| range_check_9_9::LOG_SIZE,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );
    option_log_size(
        &claim.range_check_7_2_5,
        |_| range_check_7_2_5::LOG_SIZE,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );
    option_log_size(
        &claim.range_check_3_6_6_3,
        |_| range_check_3_6_6_3::LOG_SIZE,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );
    option_log_size(
        &claim.range_check_4_4_4_4,
        |_| range_check_4_4_4_4::LOG_SIZE,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );
    option_log_size(
        &claim.range_check_3_3_3_3_3,
        |_| range_check_3_3_3_3_3::LOG_SIZE,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );

    // Verify bitwise xor
    option_log_size(
        &claim.verify_bitwise_xor_4,
        |_| verify_bitwise_xor_4::LOG_SIZE,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );
    option_log_size(
        &claim.verify_bitwise_xor_7,
        |_| verify_bitwise_xor_7::LOG_SIZE,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );
    option_log_size(
        &claim.verify_bitwise_xor_8,
        |_| verify_bitwise_xor_8::LOG_SIZE,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );
    option_log_size(
        &claim.verify_bitwise_xor_9,
        |_| verify_bitwise_xor_9::LOG_SIZE,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );

    (component_enable_bits, component_log_sizes)
}

/// Update the `component_log_sizes` and `component_enable_bits` with the log size of the given
/// option if it is some.
fn option_log_size<T>(
    claims: &Option<T>,
    get_log_size: impl FnOnce(&T) -> u32,
    component_log_sizes: &mut Vec<u32>,
    component_enable_bits: &mut Vec<bool>,
) {
    if let Some(claim) = claims {
        component_log_sizes.push(get_log_size(claim));
        component_enable_bits.push(true);
    } else {
        component_log_sizes.push(0_u32);
        component_enable_bits.push(false);
    }
}

/// Returns the claimed sum of an option.
fn option_claimed_sum<T>(
    claims: &Option<T>,
    get_claimed_sum: impl FnOnce(&T) -> SecureField,
) -> SecureField {
    if let Some(claim) = claims {
        get_claimed_sum(claim)
    } else {
        SecureField::zero()
    }
}

/// Extracts the claimed sums from a [CairoInteractionClaim].
///
/// Returns a vector of all claimed sums for the logup argument, one per component.
/// The order must match the order of components as they appear in
/// [cairo_air::air::CairoComponents].
pub fn flatten_interaction_claim(interaction_claim: &CairoInteractionClaim) -> Vec<SecureField> {
    let mut claimed_sums = Vec::new();

    claimed_sums.push(option_claimed_sum(&interaction_claim.add_opcode, |c| {
        c.claimed_sum
    }));
    claimed_sums.push(option_claimed_sum(
        &interaction_claim.add_opcode_small,
        |c| c.claimed_sum,
    ));
    claimed_sums.push(option_claimed_sum(&interaction_claim.add_ap_opcode, |c| {
        c.claimed_sum
    }));
    claimed_sums.push(option_claimed_sum(
        &interaction_claim.assert_eq_opcode,
        |c| c.claimed_sum,
    ));
    claimed_sums.push(option_claimed_sum(
        &interaction_claim.assert_eq_opcode_imm,
        |c| c.claimed_sum,
    ));
    claimed_sums.push(option_claimed_sum(
        &interaction_claim.assert_eq_opcode_double_deref,
        |c| c.claimed_sum,
    ));
    claimed_sums.push(option_claimed_sum(
        &interaction_claim.blake_compress_opcode,
        |c| c.claimed_sum,
    ));
    claimed_sums.push(option_claimed_sum(
        &interaction_claim.call_opcode_abs,
        |c| c.claimed_sum,
    ));
    claimed_sums.push(option_claimed_sum(
        &interaction_claim.call_opcode_rel_imm,
        |c| c.claimed_sum,
    ));
    claimed_sums.push(option_claimed_sum(&interaction_claim.generic_opcode, |c| {
        c.claimed_sum
    }));
    claimed_sums.push(option_claimed_sum(
        &interaction_claim.jnz_opcode_non_taken,
        |c| c.claimed_sum,
    ));
    claimed_sums.push(option_claimed_sum(
        &interaction_claim.jnz_opcode_taken,
        |c| c.claimed_sum,
    ));
    claimed_sums.push(option_claimed_sum(
        &interaction_claim.jump_opcode_abs,
        |c| c.claimed_sum,
    ));
    claimed_sums.push(option_claimed_sum(
        &interaction_claim.jump_opcode_double_deref,
        |c| c.claimed_sum,
    ));
    claimed_sums.push(option_claimed_sum(
        &interaction_claim.jump_opcode_rel,
        |c| c.claimed_sum,
    ));
    claimed_sums.push(option_claimed_sum(
        &interaction_claim.jump_opcode_rel_imm,
        |c| c.claimed_sum,
    ));
    claimed_sums.push(option_claimed_sum(&interaction_claim.mul_opcode, |c| {
        c.claimed_sum
    }));
    claimed_sums.push(option_claimed_sum(
        &interaction_claim.mul_opcode_small,
        |c| c.claimed_sum,
    ));
    claimed_sums.push(option_claimed_sum(
        &interaction_claim.qm_31_add_mul_opcode,
        |c| c.claimed_sum,
    ));
    claimed_sums.push(option_claimed_sum(&interaction_claim.ret_opcode, |c| {
        c.claimed_sum
    }));

    // Verify instruction
    claimed_sums.push(option_claimed_sum(
        &interaction_claim.verify_instruction,
        |c| c.claimed_sum,
    ));

    // Blake context
    claimed_sums.push(option_claimed_sum(&interaction_claim.blake_round, |c| {
        c.claimed_sum
    }));
    claimed_sums.push(option_claimed_sum(&interaction_claim.blake_g, |c| {
        c.claimed_sum
    }));
    claimed_sums.push(option_claimed_sum(
        &interaction_claim.blake_round_sigma,
        |c| c.claimed_sum,
    ));
    claimed_sums.push(option_claimed_sum(&interaction_claim.triple_xor_32, |c| {
        c.claimed_sum
    }));
    claimed_sums.push(option_claimed_sum(
        &interaction_claim.verify_bitwise_xor_12,
        |c| c.claimed_sum,
    ));

    // Builtins
    claimed_sums.push(option_claimed_sum(
        &interaction_claim.add_mod_builtin,
        |c| c.claimed_sum,
    ));
    claimed_sums.push(option_claimed_sum(
        &interaction_claim.bitwise_builtin,
        |c| c.claimed_sum,
    ));
    claimed_sums.push(option_claimed_sum(
        &interaction_claim.mul_mod_builtin,
        |c| c.claimed_sum,
    ));
    claimed_sums.push(option_claimed_sum(
        &interaction_claim.pedersen_builtin,
        |c| c.claimed_sum,
    ));
    claimed_sums.push(option_claimed_sum(
        &interaction_claim.pedersen_builtin_narrow_windows,
        |c| c.claimed_sum,
    ));
    claimed_sums.push(option_claimed_sum(
        &interaction_claim.poseidon_builtin,
        |c| c.claimed_sum,
    ));
    claimed_sums.push(option_claimed_sum(
        &interaction_claim.range_check_builtin,
        |c| c.claimed_sum,
    ));
    claimed_sums.push(option_claimed_sum(
        &interaction_claim.range_check96_builtin,
        |c| c.claimed_sum,
    ));

    // Pedersen context
    claimed_sums.push(option_claimed_sum(
        &interaction_claim.pedersen_aggregator_window_bits_18,
        |c| c.claimed_sum,
    ));
    claimed_sums.push(option_claimed_sum(
        &interaction_claim.partial_ec_mul_window_bits_18,
        |c| c.claimed_sum,
    ));
    claimed_sums.push(option_claimed_sum(
        &interaction_claim.pedersen_points_table_window_bits_18,
        |c| c.claimed_sum,
    ));

    // Pedersen narrow windows context
    claimed_sums.push(option_claimed_sum(
        &interaction_claim.pedersen_aggregator_window_bits_9,
        |c| c.claimed_sum,
    ));
    claimed_sums.push(option_claimed_sum(
        &interaction_claim.partial_ec_mul_window_bits_9,
        |c| c.claimed_sum,
    ));
    claimed_sums.push(option_claimed_sum(
        &interaction_claim.pedersen_points_table_window_bits_9,
        |c| c.claimed_sum,
    ));

    // Poseidon context
    claimed_sums.push(option_claimed_sum(
        &interaction_claim.poseidon_aggregator,
        |c| c.claimed_sum,
    ));
    claimed_sums.push(option_claimed_sum(
        &interaction_claim.poseidon_3_partial_rounds_chain,
        |c| c.claimed_sum,
    ));
    claimed_sums.push(option_claimed_sum(
        &interaction_claim.poseidon_full_round_chain,
        |c| c.claimed_sum,
    ));
    claimed_sums.push(option_claimed_sum(&interaction_claim.cube_252, |c| {
        c.claimed_sum
    }));
    claimed_sums.push(option_claimed_sum(
        &interaction_claim.poseidon_round_keys,
        |c| c.claimed_sum,
    ));
    claimed_sums.push(option_claimed_sum(
        &interaction_claim.range_check_252_width_27,
        |c| c.claimed_sum,
    ));

    // Memory address to id
    claimed_sums.push(option_claimed_sum(
        &interaction_claim.memory_address_to_id,
        |c| c.claimed_sum,
    ));

    // Memory id to big
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

    // Range checks
    claimed_sums.push(option_claimed_sum(&interaction_claim.range_check_6, |c| {
        c.claimed_sum
    }));
    claimed_sums.push(option_claimed_sum(&interaction_claim.range_check_8, |c| {
        c.claimed_sum
    }));
    claimed_sums.push(option_claimed_sum(&interaction_claim.range_check_11, |c| {
        c.claimed_sum
    }));
    claimed_sums.push(option_claimed_sum(&interaction_claim.range_check_12, |c| {
        c.claimed_sum
    }));
    claimed_sums.push(option_claimed_sum(&interaction_claim.range_check_18, |c| {
        c.claimed_sum
    }));
    claimed_sums.push(option_claimed_sum(&interaction_claim.range_check_20, |c| {
        c.claimed_sum
    }));
    claimed_sums.push(option_claimed_sum(
        &interaction_claim.range_check_4_3,
        |c| c.claimed_sum,
    ));
    claimed_sums.push(option_claimed_sum(
        &interaction_claim.range_check_4_4,
        |c| c.claimed_sum,
    ));
    claimed_sums.push(option_claimed_sum(
        &interaction_claim.range_check_9_9,
        |c| c.claimed_sum,
    ));
    claimed_sums.push(option_claimed_sum(
        &interaction_claim.range_check_7_2_5,
        |c| c.claimed_sum,
    ));
    claimed_sums.push(option_claimed_sum(
        &interaction_claim.range_check_3_6_6_3,
        |c| c.claimed_sum,
    ));
    claimed_sums.push(option_claimed_sum(
        &interaction_claim.range_check_4_4_4_4,
        |c| c.claimed_sum,
    ));
    claimed_sums.push(option_claimed_sum(
        &interaction_claim.range_check_3_3_3_3_3,
        |c| c.claimed_sum,
    ));

    // Verify bitwise xor
    claimed_sums.push(option_claimed_sum(
        &interaction_claim.verify_bitwise_xor_4,
        |c| c.claimed_sum,
    ));
    claimed_sums.push(option_claimed_sum(
        &interaction_claim.verify_bitwise_xor_7,
        |c| c.claimed_sum,
    ));
    claimed_sums.push(option_claimed_sum(
        &interaction_claim.verify_bitwise_xor_8,
        |c| c.claimed_sum,
    ));
    claimed_sums.push(option_claimed_sum(
        &interaction_claim.verify_bitwise_xor_9,
        |c| c.claimed_sum,
    ));

    claimed_sums
}
