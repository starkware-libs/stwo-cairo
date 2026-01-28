use num_traits::Zero;
use stwo::core::channel::Channel;
use stwo::core::fields::qm31::SecureField;
use stwo_cairo_common::prover_types::cpu::{CasmState, FELT252_BITS_PER_WORD, FELT252_N_WORDS};
use stwo_cairo_common::prover_types::felt::split;

use crate::air::{
    CairoClaim, CairoInteractionClaim, PublicData, PublicMemory, PublicSegmentRanges, SegmentRange,
};
// use crate::blake::air::{Claim as BlakeClaim, InteractionClaim as BlakeInteractionClaim};
// use crate::builtins_air::{BuiltinsClaim, BuiltinsInteractionClaim};
// use crate::components::memory_address_to_id::MEMORY_ADDRESS_TO_ID_SPLIT;
use crate::components::{
    // add_mod_builtin, bitwise_builtin, blake_round_sigma,
    memory_address_to_id,
    memory_id_to_big, /* mul_mod_builtin, pedersen_builtin,
                       * pedersen_points_table_window_bits_18, poseidon_builtin,
                       * poseidon_round_keys, range_check96_builtin, */
    range_check_11, // range_check_12,
    range_check_18,
    // range_check_20, range_check_3_3_3_3_3, range_check_3_6_6_3,
    range_check_4_3,
    // range_check_4_4, range_check_4_4_4_4, range_check_6,
    range_check_7_2_5,
    // range_check_8,
    range_check_9_9,
    // range_check_builtin, verify_bitwise_xor_12,
    // verify_bitwise_xor_4, verify_bitwise_xor_7, verify_bitwise_xor_8, verify_bitwise_xor_9,
};
use crate::opcodes_air::{OpcodeClaim, OpcodeInteractionClaim};
// use crate::pedersen::air::{Claim as PedersenClaim, InteractionClaim as
// PedersenInteractionClaim}; use crate::poseidon::air::{Claim as PoseidonClaim,
// InteractionClaim as PoseidonInteractionClaim};
use crate::range_checks_air::RangeChecksClaim;
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

    pub fn into_secure_felts(&self) -> Vec<SecureField> {
        let mut u32s = vec![];
        u32s.extend(enable_bits_to_u32s(&self.component_enable_bits));
        u32s.extend(self.component_log_sizes.clone());
        u32s.extend(public_data_to_u32s(&self.public_data));
        pack_into_secure_felts(u32s.into_iter())
    }

    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_felts(&pack_into_secure_felts(
            enable_bits_to_u32s(&self.component_enable_bits).into_iter(),
        ));
        channel.mix_felts(&pack_into_secure_felts(
            self.component_log_sizes.iter().cloned(),
        ));
        channel.mix_felts(&pack_into_secure_felts(
            public_data_to_u32s(&self.public_data).into_iter(),
        ));
    }
}

fn enable_bits_to_u32s(enable_bits: &[bool]) -> Vec<u32> {
    let mut res = vec![];
    for bits in enable_bits.chunks(1) {
        let mut v: u32 = 0;
        for (i, &bit) in bits.iter().enumerate() {
            if bit {
                v |= 1 << i;
            }
        }
        res.push(v);
    }
    res
}

pub fn public_data_to_u32s(public_data: &PublicData) -> Vec<u32> {
    let mut public_claim = vec![];

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
    } = public_data;
    public_claim.push(initial_pc.0);
    public_claim.push(initial_ap.0);
    public_claim.push(initial_fp.0);
    public_claim.push(final_pc.0);
    public_claim.push(final_ap.0);
    public_claim.push(final_fp.0);

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
    single_segment_range(Some(*output_ranges), &mut public_claim);
    single_segment_range(*pedersen, &mut public_claim);
    single_segment_range(*range_check_128, &mut public_claim);
    single_segment_range(*ecdsa, &mut public_claim);
    single_segment_range(*bitwise, &mut public_claim);
    single_segment_range(*ec_op, &mut public_claim);
    single_segment_range(*keccak, &mut public_claim);
    single_segment_range(*poseidon, &mut public_claim);
    single_segment_range(*range_check_96, &mut public_claim);
    single_segment_range(*add_mod, &mut public_claim);
    single_segment_range(*mul_mod, &mut public_claim);
    public_claim.extend(safe_call_ids);

    for (id, value) in output {
        public_claim.push(*id);
        public_claim
            .extend::<[u32; FELT252_N_WORDS]>(split(*value, (1 << FELT252_BITS_PER_WORD) - 1));
    }

    for (id, value) in program {
        public_claim.push(*id);
        let limbs = split(*value, (1 << FELT252_BITS_PER_WORD) - 1);
        eprintln!("program: {:?}", (*id, limbs));
        public_claim.extend::<[u32; FELT252_N_WORDS]>(limbs);
    }

    public_claim
}

/// Extracts component enable bits, and component log sizes from a [CairoClaim] and returns it as
/// vectors of [bool] and [u32] respectively.
#[allow(clippy::too_many_lines)]
pub fn flatten_claim(claim: &CairoClaim) -> (Vec<bool>, Vec<u32>) {
    let mut component_enable_bits = vec![];
    let mut component_log_sizes = vec![];

    let CairoClaim {
        public_data: _,
        opcodes,
        verify_instruction,
        // blake_context,
        // builtins,
        // pedersen_context,
        // poseidon_context,
        memory_address_to_id,
        memory_id_to_value,
        range_checks,
        // verify_bitwise_xor_4,
        // verify_bitwise_xor_7,
        // verify_bitwise_xor_8,
        // verify_bitwise_xor_9,
    } = claim;

    // Opcodes
    let OpcodeClaim {
        // add,
        // add_small,
        add_ap,
        assert_eq,
        // assert_eq_imm,
        // assert_eq_double_deref,
        // blake,
        // call,
        call_rel_imm,
        // generic,
        // jnz,
        // jnz_taken,
        // jump,
        // jump_double_deref,
        // jump_rel,
        // jump_rel_imm,
        // mul,
        // mul_small,
        // qm31,
        ret,
    } = opcodes;

    // single_log_size(
    //     add,
    //     |c| c.log_size,
    //     &mut component_log_sizes,
    //     &mut component_enable_bits,
    // );
    // single_log_size(
    //     add_small,
    //     |c| c.log_size,
    //     &mut component_log_sizes,
    //     &mut component_enable_bits,
    // );
    single_log_size(
        add_ap,
        |c| c.log_size,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );

    single_log_size(
        assert_eq,
        |c| c.log_size,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );

    // single_log_size(
    //     assert_eq_imm,
    //     |c| c.log_size,
    //     &mut component_log_sizes,
    //     &mut component_enable_bits,
    // );
    // single_log_size(
    //     assert_eq_double_deref,
    //     |c| c.log_size,
    //     &mut component_log_sizes,
    //     &mut component_enable_bits,
    // );
    // single_log_size(
    //     blake,
    //     |c| c.log_size,
    //     &mut component_log_sizes,
    //     &mut component_enable_bits,
    // );
    // single_log_size(
    //     call,
    //     |c| c.log_size,
    //     &mut component_log_sizes,
    //     &mut component_enable_bits,
    // );
    single_log_size(
        call_rel_imm,
        |c| c.log_size,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );

    // single_log_size(
    //     generic,
    //     |c| c.log_size,
    //     &mut component_log_sizes,
    //     &mut component_enable_bits,
    // );
    // single_log_size(
    //     jnz,
    //     |c| c.log_size,
    //     &mut component_log_sizes,
    //     &mut component_enable_bits,
    // );
    // println!(
    //     "component enable bit for jnz: {}",
    //     component_enable_bits.last().unwrap()
    // );
    // single_log_size(
    //     jnz_taken,
    //     |c| c.log_size,
    //     &mut component_log_sizes,
    //     &mut component_enable_bits,
    // );
    // println!(
    //     "component enable bit for jnz_taken: {}",
    //     component_enable_bits.last().unwrap()
    // );
    // single_log_size(
    //     jump,
    //     |c| c.log_size,
    //     &mut component_log_sizes,
    //     &mut component_enable_bits,
    // );
    // single_log_size(
    //     jump_double_deref,
    //     |c| c.log_size,
    //     &mut component_log_sizes,
    //     &mut component_enable_bits,
    // );
    // single_log_size(
    //     jump_rel,
    //     |c| c.log_size,
    //     &mut component_log_sizes,
    //     &mut component_enable_bits,
    // );
    // single_log_size(
    //     jump_rel_imm,
    //     |c| c.log_size,
    //     &mut component_log_sizes,
    //     &mut component_enable_bits,
    // );
    // println!(
    //     "component enable bit for jump_rel_imm: {}",
    //     component_enable_bits.last().unwrap()
    // );
    // single_log_size(
    //     mul,
    //     |c| c.log_size,
    //     &mut component_log_sizes,
    //     &mut component_enable_bits,
    // );
    // single_log_size(
    //     mul_small,
    //     |c| c.log_size,
    //     &mut component_log_sizes,
    //     &mut component_enable_bits,
    // );
    // single_log_size(
    //     qm31,
    //     |c| c.log_size,
    //     &mut component_log_sizes,
    //     &mut component_enable_bits,
    // );
    single_log_size(
        ret,
        |c| c.log_size,
        &mut component_log_sizes,
        &mut component_enable_bits,
    );

    // Verify instruction
    component_log_sizes.push(verify_instruction.log_size);
    component_enable_bits.push(true);

    // Blake context
    // if let Some(BlakeClaim {
    //     blake_g,
    //     triple_xor_32,
    //     blake_sigma: blake_round_sigma::Claim {},
    //     blake_round,
    //     verify_bitwise_xor_12: verify_bitwise_xor_12::Claim {},
    // }) = &blake_context.claim
    // {
    //     component_log_sizes.push(blake_g.log_size);
    //     component_log_sizes.push(triple_xor_32.log_size);
    //     component_log_sizes.push(blake_round_sigma::LOG_SIZE);
    //     component_log_sizes.push(blake_round.log_size);
    //     component_log_sizes.push(verify_bitwise_xor_12::LOG_SIZE);
    //     component_enable_bits.extend([true; 5]);
    // } else {
    //     component_log_sizes.extend([0; 5]);
    //     component_enable_bits.extend([false; 5]);
    // }

    // Builtins
    // let BuiltinsClaim {
    //     range_check_128_builtin,
    //     range_check_96_builtin,
    //     bitwise_builtin,
    //     add_mod_builtin,
    //     mul_mod_builtin,
    //     pedersen_builtin,
    //     poseidon_builtin,
    // } = builtins;
    // if let Some(range_check_builtin::Claim {
    //     log_size,
    //     range_check_builtin_segment_start: _,
    // }) = range_check_128_builtin
    // {
    //     component_log_sizes.push(*log_size);
    //     component_enable_bits.push(true);
    // } else {
    //     component_log_sizes.push(0_u32);
    //     component_enable_bits.push(false);
    // }
    // if let Some(range_check96_builtin::Claim {
    //     log_size,
    //     range_check96_builtin_segment_start: _,
    // }) = range_check_96_builtin
    // {
    //     component_log_sizes.push(*log_size);
    //     component_enable_bits.push(true);
    // } else {
    //     component_log_sizes.push(0_u32);
    //     component_enable_bits.push(false);
    // }
    // if let Some(bitwise_builtin::Claim {
    //     log_size,
    //     bitwise_builtin_segment_start: _,
    // }) = bitwise_builtin
    // {
    //     component_log_sizes.push(*log_size);
    //     component_enable_bits.push(true);
    // } else {
    //     component_log_sizes.push(0_u32);
    //     component_enable_bits.push(false);
    // }
    // if let Some(add_mod_builtin::Claim {
    //     log_size,
    //     add_mod_builtin_segment_start: _,
    // }) = add_mod_builtin
    // {
    //     component_log_sizes.push(*log_size);
    //     component_enable_bits.push(true);
    // } else {
    //     component_log_sizes.push(0_u32);
    //     component_enable_bits.push(false);
    // }
    // if let Some(mul_mod_builtin::Claim {
    //     log_size,
    //     mul_mod_builtin_segment_start: _,
    // }) = mul_mod_builtin
    // {
    //     component_log_sizes.push(*log_size);
    //     component_enable_bits.push(true);
    // } else {
    //     component_log_sizes.push(0_u32);
    //     component_enable_bits.push(false);
    // }
    // if let Some(pedersen_builtin::Claim {
    //     log_size,
    //     pedersen_builtin_segment_start: _,
    // }) = pedersen_builtin
    // {
    //     component_log_sizes.push(*log_size);
    //     component_enable_bits.push(true);
    // } else {
    //     component_log_sizes.push(0_u32);
    //     component_enable_bits.push(false);
    // }
    // if let Some(poseidon_builtin::Claim {
    //     log_size,
    //     poseidon_builtin_segment_start: _,
    // }) = poseidon_builtin
    // {
    //     component_log_sizes.push(*log_size);
    //     component_enable_bits.push(true);
    // } else {
    //     component_log_sizes.push(0_u32);
    //     component_enable_bits.push(false);
    // }

    // Pedersen context
    // if let Some(PedersenClaim {
    //     pedersen_aggregator,
    //     partial_ec_mul,
    //     pedersen_points_table: pedersen_points_table_window_bits_18::Claim {},
    // }) = &pedersen_context.claim
    // {
    //     component_log_sizes.push(pedersen_aggregator.log_size);
    //     component_log_sizes.push(partial_ec_mul.log_size);
    //     component_log_sizes.push(pedersen_points_table_window_bits_18::LOG_SIZE);
    //     component_enable_bits.extend([true; 3]);
    // } else {
    //     component_log_sizes.extend([0; 3]);
    //     component_enable_bits.extend([false; 3]);
    // }

    // Poseidon context
    // if let Some(PoseidonClaim {
    //     poseidon_aggregator,
    //     poseidon_3_partial_rounds_chain,
    //     poseidon_full_round_chain,
    //     cube_252,
    //     poseidon_round_keys: poseidon_round_keys::Claim {},
    //     range_check_252_width_27,
    // }) = &poseidon_context.claim
    // {
    //     component_log_sizes.push(poseidon_aggregator.log_size);
    //     component_log_sizes.push(poseidon_3_partial_rounds_chain.log_size);
    //     component_log_sizes.push(poseidon_full_round_chain.log_size);
    //     component_log_sizes.push(cube_252.log_size);
    //     component_log_sizes.push(poseidon_round_keys::LOG_SIZE);
    //     component_log_sizes.push(range_check_252_width_27.log_size);
    //     component_enable_bits.extend([true; 6]);
    // } else {
    //     component_log_sizes.extend([0; 6]);
    //     component_enable_bits.extend([false; 6]);
    // }

    // Memory
    let memory_address_to_id::Claim { log_size } = memory_address_to_id;
    component_log_sizes.push(*log_size);
    component_enable_bits.push(true);

    let memory_id_to_big::Claim {
        // big_log_sizes,
        small_log_size,
    } = memory_id_to_value;
    // assert!(big_log_sizes.len() <= MEMORY_ADDRESS_TO_ID_SPLIT);
    // for log_size in big_log_sizes {
    //     component_log_sizes.push(*log_size);
    //     component_enable_bits.push(true);
    //     println!(
    //         "component enable bit for memory_id_to_big: {}",
    //         component_enable_bits.last().unwrap()
    //     );
    // }
    // for _ in 0..(MEMORY_ADDRESS_TO_ID_SPLIT - big_log_sizes.len()) {
    //     component_log_sizes.push(0_u32);
    //     component_enable_bits.push(false);
    //     println!(
    //         "component enable bit for memory_id_to_big: {}",
    //         component_enable_bits.last().unwrap()
    //     );
    // }
    component_log_sizes.push(*small_log_size);

    component_enable_bits.push(true);
    println!(
        "component enable bit for memory_id_to_small: {}",
        component_enable_bits.last().unwrap()
    );

    let RangeChecksClaim {
        // rc_6,
        // rc_8,
        rc_11,
        // rc_12,
        rc_18,
        // rc_20,
        rc_4_3,
        // rc_4_4,
        rc_9_9,
        rc_7_2_5,
        // rc_3_6_6_3,
        // rc_4_4_4_4,
        // rc_3_3_3_3_3,
    } = range_checks;
    // let range_check_6::Claim {} = rc_6;
    // component_log_sizes.push(range_check_6::LOG_SIZE);
    // component_enable_bits.push(true);
    // let range_check_8::Claim {} = rc_8;
    // component_log_sizes.push(range_check_8::LOG_SIZE);
    // component_enable_bits.push(true);
    let range_check_11::Claim {} = rc_11;
    component_log_sizes.push(range_check_11::LOG_SIZE);
    component_enable_bits.push(true);
    println!(
        "component enable bit for range_check_11: {}",
        component_enable_bits.last().unwrap()
    );
    // let range_check_12::Claim {} = rc_12;
    // component_log_sizes.push(range_check_12::LOG_SIZE);
    // component_enable_bits.push(true);
    let range_check_18::Claim {} = rc_18;
    component_log_sizes.push(range_check_18::LOG_SIZE);
    component_enable_bits.push(true);
    println!(
        "component enable bit for range_check_18: {}",
        component_enable_bits.last().unwrap()
    );
    // let range_check_20::Claim {} = rc_20;
    // component_log_sizes.push(range_check_20::LOG_SIZE);
    // component_enable_bits.push(true);
    let range_check_4_3::Claim {} = rc_4_3;
    component_log_sizes.push(range_check_4_3::LOG_SIZE);
    component_enable_bits.push(true);
    println!(
        "component enable bit for range_check_4_3: {}",
        component_enable_bits.last().unwrap()
    );
    // let range_check_4_4::Claim {} = rc_4_4;
    // component_log_sizes.push(range_check_4_4::LOG_SIZE);
    // component_enable_bits.push(true);
    let range_check_9_9::Claim {} = rc_9_9;
    component_log_sizes.push(range_check_9_9::LOG_SIZE);
    component_enable_bits.push(true);
    println!(
        "component enable bit for range_check_9_9: {}",
        component_enable_bits.last().unwrap()
    );
    let range_check_7_2_5::Claim {} = rc_7_2_5;
    component_log_sizes.push(range_check_7_2_5::LOG_SIZE);
    component_enable_bits.push(true);
    println!(
        "component enable bit for range_check_7_2_5: {}",
        component_enable_bits.last().unwrap()
    );
    // let range_check_3_6_6_3::Claim {} = rc_3_6_6_3;
    // component_log_sizes.push(range_check_3_6_6_3::LOG_SIZE);
    // component_enable_bits.push(true);
    // let range_check_4_4_4_4::Claim {} = rc_4_4_4_4;
    // component_log_sizes.push(range_check_4_4_4_4::LOG_SIZE);
    // component_enable_bits.push(true);
    // let range_check_3_3_3_3_3::Claim {} = rc_3_3_3_3_3;
    // component_log_sizes.push(range_check_3_3_3_3_3::LOG_SIZE);
    // component_enable_bits.push(true);

    // Verify bitwise xor
    // let verify_bitwise_xor_4::Claim {} = verify_bitwise_xor_4;
    // component_log_sizes.push(verify_bitwise_xor_4::LOG_SIZE);
    // component_enable_bits.push(true);
    // let verify_bitwise_xor_7::Claim {} = verify_bitwise_xor_7;
    // component_log_sizes.push(verify_bitwise_xor_7::LOG_SIZE);
    // component_enable_bits.push(true);
    // let verify_bitwise_xor_8::Claim {} = verify_bitwise_xor_8;
    // component_log_sizes.push(verify_bitwise_xor_8::LOG_SIZE);
    // component_enable_bits.push(true);
    // let verify_bitwise_xor_9::Claim {} = verify_bitwise_xor_9;
    // component_log_sizes.push(verify_bitwise_xor_9::LOG_SIZE);
    // component_enable_bits.push(true);
    // println!("Component enable bits: {:?}", component_enable_bits);
    // println!("Component log sizes: {:?}", component_log_sizes);

    (component_enable_bits, component_log_sizes)
}

fn single_segment_range(segment: Option<SegmentRange>, public_claim: &mut Vec<u32>) {
    if let Some(segment) = segment {
        public_claim.extend([
            segment.start_ptr.id,
            segment.start_ptr.value,
            segment.stop_ptr.id,
            segment.stop_ptr.value,
        ]);
    } else {
        public_claim.extend([0_u32; 4]);
    }
}

/// Returns the log size from a single-element slice.
/// Panics if the slice does not contain exactly one element.
fn single_log_size<T>(
    claims: &[T],
    get_log_size: impl FnOnce(&T) -> u32,
    component_log_sizes: &mut Vec<u32>,
    component_enable_bits: &mut Vec<bool>,
) {
    if claims.is_empty() {
        component_log_sizes.push(0_u32);
        component_enable_bits.push(false);
    } else if let [claim] = claims {
        component_log_sizes.push(get_log_size(claim));
        component_enable_bits.push(true);
    } else {
        panic!("expected up to one component")
    }
}

/// Returns the claimed sum from a single-element slice.
/// Panics if the slice does not contain exactly one element.
fn single_claimed_sum<T>(
    claims: &[T],
    get_claimed_sum: impl FnOnce(&T) -> SecureField,
) -> SecureField {
    if claims.is_empty() {
        SecureField::zero()
    } else if let [claim] = claims {
        get_claimed_sum(claim)
    } else {
        panic!("expected up to one component")
    }
}

/// Extracts the claimed sums from a [CairoInteractionClaim].
///
/// Returns a vector of all claimed sums for the logup argument, one per component.
/// The order must match the order of components as they appear in
/// [cairo_air::air::CairoComponents].
#[allow(clippy::too_many_lines)]
pub fn flatten_interaction_claim(interaction_claim: &CairoInteractionClaim) -> Vec<SecureField> {
    let CairoInteractionClaim {
        opcodes,
        verify_instruction,
        // blake_context,
        // builtins,
        // pedersen_context,
        // poseidon_context,
        memory_address_to_id,
        memory_id_to_value,
        range_checks,
        // verify_bitwise_xor_4,
        // verify_bitwise_xor_7,
        // verify_bitwise_xor_8,
        // verify_bitwise_xor_9,
    } = interaction_claim;
    let mut claimed_sums = Vec::new();

    // Opcodes
    let OpcodeInteractionClaim {
        // add,
        // add_small,
        add_ap,
        assert_eq,
        // assert_eq_imm,
        // assert_eq_double_deref,
        // blake,
        // call,
        call_rel_imm,
        // generic,
        // jnz,
        // jnz_taken,
        // jump,
        // jump_double_deref,
        // jump_rel,
        // jump_rel_imm,
        // mul,
        // mul_small,
        // qm31,
        ret,
    } = opcodes;
    // claimed_sums.push(single_claimed_sum(add, |c| c.claimed_sum));
    // claimed_sums.push(single_claimed_sum(add_small, |c| c.claimed_sum));
    claimed_sums.push(single_claimed_sum(add_ap, |c| c.claimed_sum));
    claimed_sums.push(single_claimed_sum(assert_eq, |c| c.claimed_sum));
    // claimed_sums.push(single_claimed_sum(assert_eq_imm, |c| c.claimed_sum));
    // claimed_sums.push(single_claimed_sum(assert_eq_double_deref, |c| {
    //     c.claimed_sum
    // }));
    // claimed_sums.push(single_claimed_sum(blake, |c| c.claimed_sum));
    // claimed_sums.push(single_claimed_sum(call, |c| c.claimed_sum));
    claimed_sums.push(single_claimed_sum(call_rel_imm, |c| c.claimed_sum));
    // claimed_sums.push(single_claimed_sum(generic, |c| c.claimed_sum));
    // claimed_sums.push(single_claimed_sum(jnz, |c| c.claimed_sum));
    // claimed_sums.push(single_claimed_sum(jnz_taken, |c| c.claimed_sum));
    // claimed_sums.push(single_claimed_sum(jump, |c| c.claimed_sum));
    // claimed_sums.push(single_claimed_sum(jump_double_deref, |c| c.claimed_sum));
    // claimed_sums.push(single_claimed_sum(jump_rel, |c| c.claimed_sum));
    // claimed_sums.push(single_claimed_sum(jump_rel_imm, |c| c.claimed_sum));
    // claimed_sums.push(single_claimed_sum(mul, |c| c.claimed_sum));
    // claimed_sums.push(single_claimed_sum(mul_small, |c| c.claimed_sum));
    // claimed_sums.push(single_claimed_sum(qm31, |c| c.claimed_sum));
    claimed_sums.push(single_claimed_sum(ret, |c| c.claimed_sum));

    // Verify instruction
    claimed_sums.push(verify_instruction.claimed_sum);

    // Blake context
    // if let Some(BlakeInteractionClaim {
    //     blake_g,
    //     triple_xor_32,
    //     blake_sigma,
    //     blake_round,
    //     verify_bitwise_xor_12,
    // }) = &blake_context.claim
    // {
    //     claimed_sums.push(blake_g.claimed_sum);
    //     claimed_sums.push(triple_xor_32.claimed_sum);
    //     claimed_sums.push(blake_sigma.claimed_sum);
    //     claimed_sums.push(blake_round.claimed_sum);
    //     claimed_sums.push(verify_bitwise_xor_12.claimed_sum);
    // } else {
    //     claimed_sums.extend([SecureField::zero(); 5]);
    // }

    // Builtins
    // let BuiltinsInteractionClaim {
    //     range_check_128_builtin,
    //     range_check_96_builtin,
    //     bitwise_builtin,
    //     add_mod_builtin,
    //     mul_mod_builtin,
    //     pedersen_builtin,
    //     poseidon_builtin,
    // } = builtins;
    // claimed_sums.push(
    //     range_check_128_builtin
    //         .as_ref()
    //         .map_or(SecureField::zero(), |c| c.claimed_sum),
    // );
    // claimed_sums.push(
    //     range_check_96_builtin
    //         .as_ref()
    //         .map_or(SecureField::zero(), |c| c.claimed_sum),
    // );
    // claimed_sums.push(
    //     bitwise_builtin
    //         .as_ref()
    //         .map_or(SecureField::zero(), |c| c.claimed_sum),
    // );
    // claimed_sums.push(
    //     add_mod_builtin
    //         .as_ref()
    //         .map_or(SecureField::zero(), |c| c.claimed_sum),
    // );
    // claimed_sums.push(
    //     mul_mod_builtin
    //         .as_ref()
    //         .map_or(SecureField::zero(), |c| c.claimed_sum),
    // );
    // claimed_sums.push(
    //     pedersen_builtin
    //         .as_ref()
    //         .map_or(SecureField::zero(), |c| c.claimed_sum),
    // );
    // claimed_sums.push(
    //     poseidon_builtin
    //         .as_ref()
    //         .map_or(SecureField::zero(), |c| c.claimed_sum),
    // );

    // Pedersen context
    // if let Some(PedersenInteractionClaim {
    //     pedersen_aggregator,
    //     partial_ec_mul,
    //     pedersen_points_table,
    // }) = &pedersen_context.claim
    // {
    //     claimed_sums.push(pedersen_aggregator.claimed_sum);
    //     claimed_sums.push(partial_ec_mul.claimed_sum);
    //     claimed_sums.push(pedersen_points_table.claimed_sum);
    // } else {
    //     claimed_sums.extend([SecureField::zero(); 3]);
    // }

    // Poseidon context
    // if let Some(PoseidonInteractionClaim {
    //     poseidon_aggregator,
    //     poseidon_3_partial_rounds_chain,
    //     poseidon_full_round_chain,
    //     cube_252,
    //     poseidon_round_keys,
    //     range_check_252_width_27,
    // }) = &poseidon_context.claim
    // {
    //     claimed_sums.push(poseidon_aggregator.claimed_sum);
    //     claimed_sums.push(poseidon_3_partial_rounds_chain.claimed_sum);
    //     claimed_sums.push(poseidon_full_round_chain.claimed_sum);
    //     claimed_sums.push(cube_252.claimed_sum);
    //     claimed_sums.push(poseidon_round_keys.claimed_sum);
    //     claimed_sums.push(range_check_252_width_27.claimed_sum);
    // } else {
    //     claimed_sums.extend([SecureField::zero(); 6]);
    // }

    // Memory address to id
    claimed_sums.push(memory_address_to_id.claimed_sum);

    // Memory id to value
    // claimed_sums.extend(memory_id_to_value.big_claimed_sums.iter().copied());
    claimed_sums.push(memory_id_to_value.small_claimed_sum);

    // Range checks
    // claimed_sums.push(range_checks.rc_6.claimed_sum);
    // claimed_sums.push(range_checks.rc_8.claimed_sum);
    claimed_sums.push(range_checks.rc_11.claimed_sum);
    // claimed_sums.push(range_checks.rc_12.claimed_sum);
    claimed_sums.push(range_checks.rc_18.claimed_sum);
    // claimed_sums.push(range_checks.rc_20.claimed_sum);
    claimed_sums.push(range_checks.rc_4_3.claimed_sum);
    // claimed_sums.push(range_checks.rc_4_4.claimed_sum);
    claimed_sums.push(range_checks.rc_9_9.claimed_sum);
    claimed_sums.push(range_checks.rc_7_2_5.claimed_sum);
    // claimed_sums.push(range_checks.rc_3_6_6_3.claimed_sum);
    // claimed_sums.push(range_checks.rc_4_4_4_4.claimed_sum);
    // claimed_sums.push(range_checks.rc_3_3_3_3_3.claimed_sum);

    // Verify bitwise xor
    // claimed_sums.push(verify_bitwise_xor_4.claimed_sum);
    // claimed_sums.push(verify_bitwise_xor_7.claimed_sum);
    // claimed_sums.push(verify_bitwise_xor_8.claimed_sum);
    // claimed_sums.push(verify_bitwise_xor_9.claimed_sum);

    claimed_sums
}
