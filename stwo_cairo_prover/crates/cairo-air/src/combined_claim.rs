use num_traits::Zero;
use stwo::core::fields::m31::BaseField;
use stwo::core::fields::qm31::SecureField;
use stwo_cairo_common::prover_types::cpu::CasmState;

use crate::air::{PublicData, PublicMemory, PublicSegmentRanges, SegmentRange};
use crate::claims::{CairoClaim, CairoInteractionClaim};
use crate::components::{
    blake_round_sigma, memory_id_to_big, pedersen_points_table_window_bits_18, poseidon_round_keys,
    range_check_11, range_check_12, range_check_18, range_check_20, range_check_3_3_3_3_3,
    range_check_3_6_6_3, range_check_4_3, range_check_4_4, range_check_4_4_4_4, range_check_6,
    range_check_7_2_5, range_check_8, range_check_9_9, verify_bitwise_xor_12,
    verify_bitwise_xor_4, verify_bitwise_xor_7, verify_bitwise_xor_8, verify_bitwise_xor_9,
};

pub struct CombinedClaim {
    pub component_enable_bits: Vec<bool>,
    pub component_log_sizes: Vec<u32>,
    pub component_claimed_sums: Vec<SecureField>,
    pub public_claim: Vec<BaseField>,
}
impl CombinedClaim {
    pub fn from_cairo_claims(
        claim: &CairoClaim,
        interaction_claim: &CairoInteractionClaim,
    ) -> Self {
        let (public_claim, component_enable_bits, component_log_sizes) = get_public_claim(claim);
        let component_claimed_sums = get_cairo_claimed_sums(interaction_claim);
        Self {
            component_log_sizes,
            component_claimed_sums,
            public_claim,
            component_enable_bits,
        }
    }
}

/// Extracts public claim data and, component enable bits and component log sizes from a
/// [CairoClaim] and returns it as vectors of [BaseField], [bool] and [u32] respectively.
fn get_public_claim(claim: &CairoClaim) -> (Vec<BaseField>, Vec<bool>, Vec<u32>) {
    let mut public_claim = vec![];
    let mut component_enable_bits = vec![];
    let mut component_log_sizes = vec![];

    let PublicData {
        public_memory:
            PublicMemory {
                program,
                public_segments,
                output,
                safe_call_ids,
            },
        initial_state:
            CasmState {
                pc: initial_pc,
                ap: initial_ap,
                fp: initial_fp,
            },
        final_state:
            CasmState {
                pc: final_pc,
                ap: final_ap,
                fp: final_fp,
            },
    } = &claim.public_data;
    for (id, value) in program {
        public_claim.push(BaseField::from_u32_unchecked(*id));
        public_claim.extend(value.iter().map(|&v| BaseField::from_u32_unchecked(v)));
    }
    let PublicSegmentRanges {
        output: output_ranges,
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
    } = public_segments;
    singe_segment_range(Some(*output_ranges), &mut public_claim);
    singe_segment_range(*pedersen, &mut public_claim);
    singe_segment_range(*range_check_128, &mut public_claim);
    singe_segment_range(*ecdsa, &mut public_claim);
    singe_segment_range(*bitwise, &mut public_claim);
    singe_segment_range(*ec_op, &mut public_claim);
    singe_segment_range(*keccak, &mut public_claim);
    singe_segment_range(*poseidon, &mut public_claim);
    singe_segment_range(*range_check_96, &mut public_claim);
    singe_segment_range(*add_mod, &mut public_claim);
    singe_segment_range(*mul_mod, &mut public_claim);
    for (id, value) in output {
        public_claim.push(BaseField::from_u32_unchecked(*id));
        public_claim.extend(value.iter().map(|&v| BaseField::from_u32_unchecked(v)));
    }
    for safe_call_id in safe_call_ids {
        public_claim.push(BaseField::from_u32_unchecked(*safe_call_id));
    }
    public_claim.push(*initial_pc);
    public_claim.push(*initial_ap);
    public_claim.push(*initial_fp);
    public_claim.push(*final_ap);
    public_claim.push(*final_fp);
    public_claim.push(*final_pc);

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
        &claim.blake_g,
        |c| c.log_size,
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
        &claim.blake_round_sigma,
        |_| blake_round_sigma::LOG_SIZE,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );
    option_log_size(
        &claim.blake_round,
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
        &claim.range_check_builtin,
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
        &claim.bitwise_builtin,
        |c| c.log_size,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );
    option_log_size(
        &claim.add_mod_builtin,
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
        &claim.poseidon_builtin,
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
    assert!(big_log_sizes.len() <= 4);
    for log_size in big_log_sizes {
        component_log_sizes.push(*log_size);
        component_enable_bits.push(true);
    }
    for _ in 0..(4 - big_log_sizes.len()) {
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

    (public_claim, component_enable_bits, component_log_sizes)
}

fn singe_segment_range(segment: Option<SegmentRange>, public_claim: &mut Vec<BaseField>) {
    if let Some(segment) = segment {
        public_claim.extend([
            BaseField::from_u32_unchecked(segment.start_ptr.id),
            BaseField::from_u32_unchecked(segment.start_ptr.value),
            BaseField::from_u32_unchecked(segment.stop_ptr.id),
            BaseField::from_u32_unchecked(segment.stop_ptr.value),
        ]);
    } else {
        public_claim.extend([BaseField::zero(); 4]);
    }
}

/// Update the `component_log_sizes` and `component_enable_bits` with the log size of the given
/// option if it is some.
fn option_log_size<T>(
    claims: &Option<T>,
    log: impl FnOnce(&T) -> u32,
    component_log_sizes: &mut Vec<u32>,
    component_enable_bits: &mut Vec<bool>,
) {
    if let Some(claim) = claims {
        component_log_sizes.push(log(claim));
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
fn get_cairo_claimed_sums(interaction_claim: &CairoInteractionClaim) -> Vec<SecureField> {
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
    claimed_sums.push(option_claimed_sum(&interaction_claim.blake_g, |c| {
        c.claimed_sum
    }));
    claimed_sums.push(option_claimed_sum(&interaction_claim.triple_xor_32, |c| {
        c.claimed_sum
    }));
    claimed_sums.push(option_claimed_sum(
        &interaction_claim.blake_round_sigma,
        |c| c.claimed_sum,
    ));
    claimed_sums.push(option_claimed_sum(&interaction_claim.blake_round, |c| {
        c.claimed_sum
    }));
    claimed_sums.push(option_claimed_sum(
        &interaction_claim.verify_bitwise_xor_12,
        |c| c.claimed_sum,
    ));

    // Builtins
    claimed_sums.push(option_claimed_sum(
        &interaction_claim.range_check_builtin,
        |c| c.claimed_sum,
    ));
    claimed_sums.push(option_claimed_sum(
        &interaction_claim.range_check96_builtin,
        |c| c.claimed_sum,
    ));
    claimed_sums.push(option_claimed_sum(
        &interaction_claim.bitwise_builtin,
        |c| c.claimed_sum,
    ));
    claimed_sums.push(option_claimed_sum(
        &interaction_claim.add_mod_builtin,
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
        &interaction_claim.poseidon_builtin,
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
    let memory_id_to_value = interaction_claim.memory_id_to_big.as_ref().unwrap();
    claimed_sums.extend(memory_id_to_value.big_claimed_sums.iter().copied());
    claimed_sums.push(memory_id_to_value.small_claimed_sum);

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
