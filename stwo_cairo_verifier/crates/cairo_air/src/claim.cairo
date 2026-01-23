use core::iter::{Extend, Iterator};
use core::num::traits::{Pow, Zero};
use core::traits::TryInto;
use stwo_cairo_air::blake::*;
use stwo_cairo_air::builtins::*;
use stwo_cairo_air::cairo_air::{CairoClaim, CairoInteractionClaim};
use stwo_cairo_air::components::memory_address_to_id::MEMORY_ADDRESS_TO_ID_SPLIT;
use stwo_cairo_air::opcodes::*;
use stwo_cairo_air::pedersen::*;
use stwo_cairo_air::poseidon::*;
use stwo_cairo_air::{
    CasmState, PublicData, PublicDataImpl, PublicMemory, PublicSegmentRanges, RelationUsesDict,
};
use stwo_verifier_core::TreeArray;
use stwo_verifier_core::channel::Channel;
use stwo_verifier_core::fields::m31::{M31, M31Trait};
use stwo_verifier_core::fields::qm31::{QM31, QM31Trait, QM31_EXTENSION_DEGREE};

/// Trait that defines the functionality required by a "claim",
/// where a "claim" is an object that holds public information about
/// one or multiple components whose trace needs to be verified.
pub trait ClaimTrait<T> {
    /// Mix this claim’s public data into the verification transcript (`channel`),
    /// ensuring it influences all subsequently derived challenges.
    fn mix_into(self: @T, ref channel: Channel);
    /// Return the log₂ sizes of the columns in all components of this claim.
    ///
    /// Columns are grouped first by tree, then by column within each tree.
    /// For example, if `claim.log_sizes()[i][j] == n`, the `j`-th column in the
    /// `i`-th tree has size `2^n`.
    fn log_sizes(self: @T) -> TreeArray<Span<u32>>;
    /// Record the lookups used by the components associated with the claim.
    fn accumulate_relation_uses(self: @T, ref relation_uses: RelationUsesDict);
}

#[derive(Serde, Drop)]
pub struct FlatClaim {
    pub component_enable_bits: Span<bool>,
    pub component_log_sizes: Span<u32>,
    pub public_data: PublicData,
}

#[generate_trait]
pub impl FlatClaimImpl of FlatClaimTrait {
    fn from_cairo_claim(claim: @CairoClaim) -> FlatClaim {
        let (component_enable_bits, component_log_sizes) = flatten_claim(claim);
        FlatClaim {
            component_enable_bits, component_log_sizes, public_data: claim.public_data.clone(),
        }
    }

    fn into_qm31s(self: @FlatClaim) -> Span<QM31> {
        let mut u32s = array![];
        u32s.extend(enable_bits_to_u32s(*self.component_enable_bits));
        u32s.extend(self.component_log_sizes);
        u32s.extend(public_data_to_u32s(self.public_data));

        let mut res = array![];
        let mut chunk = array![];
        for v in u32s {
            if chunk.len() == QM31_EXTENSION_DEGREE {
                let fixed_arr: [M31; QM31_EXTENSION_DEGREE] = (*chunk.span().try_into().unwrap())
                    .unbox();
                res.append(QM31Trait::from_fixed_array(fixed_arr));
                chunk = array![];
            }
            chunk.append(M31Trait::reduce_u32(*v));
        }
        if !chunk.is_empty() {
            for _ in chunk.len()..QM31_EXTENSION_DEGREE {
                chunk.append(Zero::zero());
            }
            let fixed_arr: [M31; QM31_EXTENSION_DEGREE] = (*chunk.span().try_into().unwrap())
                .unbox();
            res.append(QM31Trait::from_fixed_array(fixed_arr));
        }
        res.span()
    }
}


fn enable_bits_to_u32s(enable_bits: Span<bool>) -> Span<u32> {
    let mut res = array![];
    let mut v: u32 = 0;
    let mut v_len: usize = 0;

    for bit in enable_bits {
        if v_len == 31 {
            res.append(v);
            v = 0;
            v_len = 0;
        }
        if *bit {
            v = v | 2_u32.pow(v_len);
        }
        v_len += 1;
    }
    res.span()
}

fn public_data_to_u32s(public_data: @PublicData) -> Span<u32> {
    let mut public_claim = array![];
    let PublicData {
        public_memory: PublicMemory {
            program, public_segments, output, safe_call_ids,
            }, initial_state: CasmState {
            pc: initial_pc, ap: initial_ap, fp: initial_fp,
            }, final_state: CasmState {
            pc: final_pc, ap: final_ap, fp: final_fp,
        },
    } = public_data;
    for (id, value) in program {
        public_claim.append(*id);
        let arr: Array<u32> = value.into_iter().map(|x| *x).collect();
        public_claim.extend(arr);
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
    public_claim
        .extend(
            array![
                *output_ranges.start_ptr.id, *output_ranges.start_ptr.value,
                *output_ranges.stop_ptr.id, *output_ranges.stop_ptr.value,
            ],
        );
    public_claim
        .extend(
            array![
                *pedersen.start_ptr.id, *pedersen.start_ptr.value, *pedersen.stop_ptr.id,
                *pedersen.stop_ptr.value,
            ],
        );
    public_claim
        .extend(
            array![
                *range_check_128.start_ptr.id, *range_check_128.start_ptr.value,
                *range_check_128.stop_ptr.id, *range_check_128.stop_ptr.value,
            ],
        );
    public_claim
        .extend(
            array![
                *ecdsa.start_ptr.id, *ecdsa.start_ptr.value, *ecdsa.stop_ptr.id,
                *ecdsa.stop_ptr.value,
            ],
        );
    public_claim
        .extend(
            array![
                *bitwise.start_ptr.id, *bitwise.start_ptr.value, *bitwise.stop_ptr.id,
                *bitwise.stop_ptr.value,
            ],
        );
    public_claim
        .extend(
            array![
                *ec_op.start_ptr.id, *ec_op.start_ptr.value, *ec_op.stop_ptr.id,
                *ec_op.stop_ptr.value,
            ],
        );
    public_claim
        .extend(
            array![
                *keccak.start_ptr.id, *keccak.start_ptr.value, *keccak.stop_ptr.id,
                *keccak.stop_ptr.value,
            ],
        );
    public_claim
        .extend(
            array![
                *poseidon.start_ptr.id, *poseidon.start_ptr.value, *poseidon.stop_ptr.id,
                *poseidon.stop_ptr.value,
            ],
        );
    public_claim
        .extend(
            array![
                *range_check_96.start_ptr.id, *range_check_96.start_ptr.value,
                *range_check_96.stop_ptr.id, *range_check_96.stop_ptr.value,
            ],
        );
    public_claim
        .extend(
            array![
                *add_mod.start_ptr.id, *add_mod.start_ptr.value, *add_mod.stop_ptr.id,
                *add_mod.stop_ptr.value,
            ],
        );
    public_claim
        .extend(
            array![
                *mul_mod.start_ptr.id, *mul_mod.start_ptr.value, *mul_mod.stop_ptr.id,
                *mul_mod.stop_ptr.value,
            ],
        );

    for (id, value) in output {
        public_claim.append(*id);
        let arr: Array<u32> = value.into_iter().map(|x| *x).collect();
        public_claim.extend(arr);
    }
    let arr: Array<u32> = safe_call_ids.into_iter().map(|x| *x).collect();
    public_claim.extend(arr);
    public_claim.append((*initial_pc).into());
    public_claim.append((*initial_ap).into());
    public_claim.append((*initial_fp).into());
    public_claim.append((*final_ap).into());
    public_claim.append((*final_fp).into());
    public_claim.append((*final_pc).into());

    public_claim.span()
}

/// Extracts component enable bits, and component log sizes from a [CairoClaim] and returns it as
/// vectors of [bool] and [u32] respectively.
fn flatten_claim(claim: @CairoClaim) -> (Span<bool>, Span<u32>) {
    let mut component_enable_bits = array![];
    let mut component_log_sizes = array![];

    let CairoClaim {
        public_data: _,
        opcodes,
        verify_instruction,
        blake_context,
        builtins,
        pedersen_context,
        poseidon_context,
        memory_address_to_id,
        memory_id_to_value,
        range_checks: _,
        verify_bitwise_xor_4,
        verify_bitwise_xor_7,
        verify_bitwise_xor_8,
        verify_bitwise_xor_9,
    } = claim;

    // Opcodes
    let OpcodeClaim {
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
    } = opcodes;

    if add.is_empty() {
        component_log_sizes.append(0_u32);
        component_enable_bits.append(false);
    } else if add.len() == 1 {
        let claim = add[0];
        component_log_sizes.append(*claim.log_size);
        component_enable_bits.append(true);
    } else {
        panic!("expected up to one component")
    }
    if add_small.is_empty() {
        component_log_sizes.append(0_u32);
        component_enable_bits.append(false);
    } else if add_small.len() == 1 {
        let claim = add_small[0];
        component_log_sizes.append(*claim.log_size);
        component_enable_bits.append(true);
    } else {
        panic!("expected up to one component")
    }
    if add_ap.is_empty() {
        component_log_sizes.append(0_u32);
        component_enable_bits.append(false);
    } else if add_ap.len() == 1 {
        let claim = add_ap[0];
        component_log_sizes.append(*claim.log_size);
        component_enable_bits.append(true);
    } else {
        panic!("expected up to one component")
    }
    if assert_eq.is_empty() {
        component_log_sizes.append(0_u32);
        component_enable_bits.append(false);
    } else if assert_eq.len() == 1 {
        let claim = assert_eq[0];
        component_log_sizes.append(*claim.log_size);
        component_enable_bits.append(true);
    } else {
        panic!("expected up to one component")
    }
    if assert_eq_imm.is_empty() {
        component_log_sizes.append(0_u32);
        component_enable_bits.append(false);
    } else if assert_eq_imm.len() == 1 {
        let claim = assert_eq_imm[0];
        component_log_sizes.append(*claim.log_size);
        component_enable_bits.append(true);
    } else {
        panic!("expected up to one component")
    }
    if assert_eq_double_deref.is_empty() {
        component_log_sizes.append(0_u32);
        component_enable_bits.append(false);
    } else if assert_eq_double_deref.len() == 1 {
        let claim = assert_eq_double_deref[0];
        component_log_sizes.append(*claim.log_size);
        component_enable_bits.append(true);
    } else {
        panic!("expected up to one component")
    }
    if blake.is_empty() {
        component_log_sizes.append(0_u32);
        component_enable_bits.append(false);
    } else if blake.len() == 1 {
        let claim = blake[0];
        component_log_sizes.append(*claim.log_size);
        component_enable_bits.append(true);
    } else {
        panic!("expected up to one component")
    }
    if call.is_empty() {
        component_log_sizes.append(0_u32);
        component_enable_bits.append(false);
    } else if call.len() == 1 {
        let claim = call[0];
        component_log_sizes.append(*claim.log_size);
        component_enable_bits.append(true);
    } else {
        panic!("expected up to one component")
    }
    if call_rel_imm.is_empty() {
        component_log_sizes.append(0_u32);
        component_enable_bits.append(false);
    } else if call_rel_imm.len() == 1 {
        let claim = call_rel_imm[0];
        component_log_sizes.append(*claim.log_size);
        component_enable_bits.append(true);
    } else {
        panic!("expected up to one component")
    }
    if generic.is_empty() {
        component_log_sizes.append(0_u32);
        component_enable_bits.append(false);
    } else if generic.len() == 1 {
        let claim = generic[0];
        component_log_sizes.append(*claim.log_size);
        component_enable_bits.append(true);
    } else {
        panic!("expected up to one component")
    }
    if jnz.is_empty() {
        component_log_sizes.append(0_u32);
        component_enable_bits.append(false);
    } else if jnz.len() == 1 {
        let claim = jnz[0];
        component_log_sizes.append(*claim.log_size);
        component_enable_bits.append(true);
    } else {
        panic!("expected up to one component")
    }
    if jnz_taken.is_empty() {
        component_log_sizes.append(0_u32);
        component_enable_bits.append(false);
    } else if jnz_taken.len() == 1 {
        let claim = jnz_taken[0];
        component_log_sizes.append(*claim.log_size);
        component_enable_bits.append(true);
    } else {
        panic!("expected up to one component")
    }
    if jump.is_empty() {
        component_log_sizes.append(0_u32);
        component_enable_bits.append(false);
    } else if jump.len() == 1 {
        let claim = jump[0];
        component_log_sizes.append(*claim.log_size);
        component_enable_bits.append(true);
    } else {
        panic!("expected up to one component")
    }
    if jump_double_deref.is_empty() {
        component_log_sizes.append(0_u32);
        component_enable_bits.append(false);
    } else if jump_double_deref.len() == 1 {
        let claim = jump_double_deref[0];
        component_log_sizes.append(*claim.log_size);
        component_enable_bits.append(true);
    } else {
        panic!("expected up to one component")
    }
    if jump_rel.is_empty() {
        component_log_sizes.append(0_u32);
        component_enable_bits.append(false);
    } else if jump_rel.len() == 1 {
        let claim = jump_rel[0];
        component_log_sizes.append(*claim.log_size);
        component_enable_bits.append(true);
    } else {
        panic!("expected up to one component")
    }
    if jump_rel_imm.is_empty() {
        component_log_sizes.append(0_u32);
        component_enable_bits.append(false);
    } else if jump_rel_imm.len() == 1 {
        let claim = jump_rel_imm[0];
        component_log_sizes.append(*claim.log_size);
        component_enable_bits.append(true);
    } else {
        panic!("expected up to one component")
    }
    if mul.is_empty() {
        component_log_sizes.append(0_u32);
        component_enable_bits.append(false);
    } else if mul.len() == 1 {
        let claim = mul[0];
        component_log_sizes.append(*claim.log_size);
        component_enable_bits.append(true);
    } else {
        panic!("expected up to one component")
    }
    if mul_small.is_empty() {
        component_log_sizes.append(0_u32);
        component_enable_bits.append(false);
    } else if mul_small.len() == 1 {
        let claim = mul_small[0];
        component_log_sizes.append(*claim.log_size);
        component_enable_bits.append(true);
    } else {
        panic!("expected up to one component")
    }
    if qm31.is_empty() {
        component_log_sizes.append(0_u32);
        component_enable_bits.append(false);
    } else if qm31.len() == 1 {
        let claim = qm31[0];
        component_log_sizes.append(*claim.log_size);
        component_enable_bits.append(true);
    } else {
        panic!("expected up to one component")
    }
    if ret.is_empty() {
        component_log_sizes.append(0_u32);
        component_enable_bits.append(false);
    } else if ret.len() == 1 {
        let claim = ret[0];
        component_log_sizes.append(*claim.log_size);
        component_enable_bits.append(true);
    } else {
        panic!("expected up to one component")
    }

    // Verify instruction
    component_log_sizes.append(*verify_instruction.log_size);
    component_enable_bits.append(true);

    // Blake context
    if let Some(BlakeClaim {
        blake_g,
        triple_xor_32,
        blake_round_sigma: crate::components::blake_round_sigma::Claim { },
        blake_round,
        verify_bitwise_xor_12: crate::components::verify_bitwise_xor_12::Claim { },
    }) = blake_context.claim {
        component_log_sizes.append(*blake_g.log_size);
        component_log_sizes.append(*triple_xor_32.log_size);
        component_log_sizes.append(crate::components::blake_round_sigma::LOG_SIZE);
        component_log_sizes.append(*blake_round.log_size);
        component_log_sizes.append(crate::components::verify_bitwise_xor_12::LOG_SIZE);
        component_enable_bits.extend(array![true, true, true, true, true]);
    } else {
        component_log_sizes.extend(array![0_u32, 0_u32, 0_u32, 0_u32, 0_u32]);
        component_enable_bits.extend(array![false, false, false, false, false]);
    }

    // Builtins
    let BuiltinsClaim {
        range_check_128_builtin,
        range_check_96_builtin,
        bitwise_builtin,
        add_mod_builtin,
        mul_mod_builtin,
        pedersen_builtin,
        poseidon_builtin,
    } = builtins;
    if let Some(crate::components::range_check_builtin::Claim {
        log_size, range_check_builtin_segment_start: _,
    }) = range_check_128_builtin {
        component_log_sizes.append(*log_size);
        component_enable_bits.append(true);
    } else {
        component_log_sizes.append(0_u32);
        component_enable_bits.append(false);
    }
    if let Some(crate::components::range_check96_builtin::Claim {
        log_size, range_check96_builtin_segment_start: _,
    }) = range_check_96_builtin {
        component_log_sizes.append(*log_size);
        component_enable_bits.append(true);
    } else {
        component_log_sizes.append(0_u32);
        component_enable_bits.append(false);
    }
    if let Some(crate::components::bitwise_builtin::Claim {
        log_size, bitwise_builtin_segment_start: _,
    }) = bitwise_builtin {
        component_log_sizes.append(*log_size);
        component_enable_bits.append(true);
    } else {
        component_log_sizes.append(0_u32);
        component_enable_bits.append(false);
    }
    if let Some(crate::components::add_mod_builtin::Claim {
        log_size, add_mod_builtin_segment_start: _,
    }) = add_mod_builtin {
        component_log_sizes.append(*log_size);
        component_enable_bits.append(true);
    } else {
        component_log_sizes.append(0_u32);
        component_enable_bits.append(false);
    }
    if let Some(crate::components::mul_mod_builtin::Claim {
        log_size, mul_mod_builtin_segment_start: _,
    }) = mul_mod_builtin {
        component_log_sizes.append(*log_size);
        component_enable_bits.append(true);
    } else {
        component_log_sizes.append(0_u32);
        component_enable_bits.append(false);
    }
    if let Some(crate::components::pedersen_builtin::Claim {
        log_size, pedersen_builtin_segment_start: _,
    }) = pedersen_builtin {
        component_log_sizes.append(*log_size);
        component_enable_bits.append(true);
    } else {
        component_log_sizes.append(0_u32);
        component_enable_bits.append(false);
    }
    if let Some(crate::components::poseidon_builtin::Claim {
        log_size, poseidon_builtin_segment_start: _,
    }) = poseidon_builtin {
        component_log_sizes.append(*log_size);
        component_enable_bits.append(true);
    } else {
        component_log_sizes.append(0_u32);
        component_enable_bits.append(false);
    }

    // Pedersen context
    if let Some(PedersenClaim {
        pedersen_aggregator,
        partial_ec_mul,
        pedersen_points_table: crate::components::pedersen_points_table_window_bits_18::Claim { },
    }) = pedersen_context.claim {
        component_log_sizes.append(*pedersen_aggregator.log_size);
        component_log_sizes.append(*partial_ec_mul.log_size);
        component_log_sizes
            .append(crate::components::pedersen_points_table_window_bits_18::LOG_SIZE);
        component_enable_bits.extend(array![true, true, true]);
    } else {
        component_log_sizes.extend(array![0_u32, 0_u32, 0_u32]);
        component_enable_bits.extend(array![false, false, false]);
    }

    // Poseidon context
    if let Some(PoseidonClaim {
        poseidon_aggregator,
        poseidon_3_partial_rounds_chain,
        poseidon_full_round_chain,
        cube_252,
        poseidon_round_keys: crate::components::poseidon_round_keys::Claim { },
        range_check_252_width_27,
    }) = poseidon_context.claim {
        component_log_sizes.append(*poseidon_aggregator.log_size);
        component_log_sizes.append(*poseidon_3_partial_rounds_chain.log_size);
        component_log_sizes.append(*poseidon_full_round_chain.log_size);
        component_log_sizes.append(*cube_252.log_size);
        component_log_sizes.append(crate::components::poseidon_round_keys::LOG_SIZE);
        component_log_sizes.append(*range_check_252_width_27.log_size);
        component_enable_bits.extend(array![true, true, true, true, true, true]);
    } else {
        component_log_sizes.extend(array![0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32]);
        component_enable_bits.extend(array![false, false, false, false, false, false]);
    }

    // Memory
    let crate::components::memory_address_to_id::Claim { log_size } = memory_address_to_id;
    component_log_sizes.append(*log_size);
    component_enable_bits.append(true);
    let crate::components::memory_id_to_big::Claim {
        big_log_sizes, small_log_size,
    } = memory_id_to_value;
    assert!(big_log_sizes.len() <= MEMORY_ADDRESS_TO_ID_SPLIT);
    for log_size in big_log_sizes {
        component_log_sizes.append(*log_size);
        component_enable_bits.append(true);
    }
    for _ in 0..(MEMORY_ADDRESS_TO_ID_SPLIT - big_log_sizes.len()) {
        component_log_sizes.append(0_u32);
        component_enable_bits.append(false);
    }
    component_log_sizes.append(*small_log_size);
    component_enable_bits.append(true);

    component_log_sizes.append(crate::components::range_check_6::LOG_SIZE);
    component_enable_bits.append(true);
    component_log_sizes.append(crate::components::range_check_8::LOG_SIZE);
    component_enable_bits.append(true);
    component_log_sizes.append(crate::components::range_check_11::LOG_SIZE);
    component_enable_bits.append(true);
    component_log_sizes.append(crate::components::range_check_12::LOG_SIZE);
    component_enable_bits.append(true);
    component_log_sizes.append(crate::components::range_check_18::LOG_SIZE);
    component_enable_bits.append(true);
    component_log_sizes.append(crate::components::range_check_20::LOG_SIZE);
    component_enable_bits.append(true);
    component_log_sizes.append(crate::components::range_check_4_3::LOG_SIZE);
    component_enable_bits.append(true);
    component_log_sizes.append(crate::components::range_check_4_4::LOG_SIZE);
    component_enable_bits.append(true);
    component_log_sizes.append(crate::components::range_check_9_9::LOG_SIZE);
    component_enable_bits.append(true);
    component_log_sizes.append(crate::components::range_check_7_2_5::LOG_SIZE);
    component_enable_bits.append(true);
    component_log_sizes.append(crate::components::range_check_3_6_6_3::LOG_SIZE);
    component_enable_bits.append(true);
    component_log_sizes.append(crate::components::range_check_4_4_4_4::LOG_SIZE);
    component_enable_bits.append(true);
    component_log_sizes.append(crate::components::range_check_3_3_3_3_3::LOG_SIZE);
    component_enable_bits.append(true);

    // Verify bitwise xor
    let crate::components::verify_bitwise_xor_4::Claim { } = verify_bitwise_xor_4;
    component_log_sizes.append(crate::components::verify_bitwise_xor_4::LOG_SIZE);
    component_enable_bits.append(true);
    let crate::components::verify_bitwise_xor_7::Claim { } = verify_bitwise_xor_7;
    component_log_sizes.append(crate::components::verify_bitwise_xor_7::LOG_SIZE);
    component_enable_bits.append(true);
    let crate::components::verify_bitwise_xor_8::Claim { } = verify_bitwise_xor_8;
    component_log_sizes.append(crate::components::verify_bitwise_xor_8::LOG_SIZE);
    component_enable_bits.append(true);
    let crate::components::verify_bitwise_xor_9::Claim { } = verify_bitwise_xor_9;
    component_log_sizes.append(crate::components::verify_bitwise_xor_9::LOG_SIZE);
    component_enable_bits.append(true);

    (component_enable_bits.span(), component_log_sizes.span())
}

/// Extracts the claimed sums from a [CairoInteractionClaim].
///
/// Returns a vector of all claimed sums for the logup argument, one per component.
/// The order must match the order of components as they appear in
/// [cairo_air::air::CairoComponents].
pub fn flatten_interaction_claim(interaction_claim: @CairoInteractionClaim) -> Span<QM31> {
    let CairoInteractionClaim {
        opcodes,
        verify_instruction,
        blake_context,
        builtins,
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
    let mut claimed_sums = array![];

    // Opcodes
    let OpcodeInteractionClaim {
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
    } = opcodes;

    if add.is_empty() {
        claimed_sums.append(Zero::zero());
    } else if add.len() == 1 {
        let claim = add[0];
        claimed_sums.append(*claim.claimed_sum);
    } else {
        panic!("expected up to one component")
    }
    if add_small.is_empty() {
        claimed_sums.append(Zero::zero());
    } else if add_small.len() == 1 {
        let claim = add_small[0];
        claimed_sums.append(*claim.claimed_sum);
    } else {
        panic!("expected up to one component")
    }
    if add_ap.is_empty() {
        claimed_sums.append(Zero::zero());
    } else if add_ap.len() == 1 {
        let claim = add_ap[0];
        claimed_sums.append(*claim.claimed_sum);
    } else {
        panic!("expected up to one component")
    }
    if assert_eq.is_empty() {
        claimed_sums.append(Zero::zero());
    } else if assert_eq.len() == 1 {
        let claim = assert_eq[0];
        claimed_sums.append(*claim.claimed_sum);
    } else {
        panic!("expected up to one component")
    }
    if assert_eq_imm.is_empty() {
        claimed_sums.append(Zero::zero());
    } else if assert_eq_imm.len() == 1 {
        let claim = assert_eq_imm[0];
        claimed_sums.append(*claim.claimed_sum);
    } else {
        panic!("expected up to one component")
    }
    if assert_eq_double_deref.is_empty() {
        claimed_sums.append(Zero::zero());
    } else if assert_eq_double_deref.len() == 1 {
        let claim = assert_eq_double_deref[0];
        claimed_sums.append(*claim.claimed_sum);
    } else {
        panic!("expected up to one component")
    }
    if blake.is_empty() {
        claimed_sums.append(Zero::zero());
    } else if blake.len() == 1 {
        let claim = blake[0];
        claimed_sums.append(*claim.claimed_sum);
    } else {
        panic!("expected up to one component")
    }
    if call.is_empty() {
        claimed_sums.append(Zero::zero());
    } else if call.len() == 1 {
        let claim = call[0];
        claimed_sums.append(*claim.claimed_sum);
    } else {
        panic!("expected up to one component")
    }
    if call_rel_imm.is_empty() {
        claimed_sums.append(Zero::zero());
    } else if call_rel_imm.len() == 1 {
        let claim = call_rel_imm[0];
        claimed_sums.append(*claim.claimed_sum);
    } else {
        panic!("expected up to one component")
    }
    if generic.is_empty() {
        claimed_sums.append(Zero::zero());
    } else if generic.len() == 1 {
        let claim = generic[0];
        claimed_sums.append(*claim.claimed_sum);
    } else {
        panic!("expected up to one component")
    }
    if jnz.is_empty() {
        claimed_sums.append(Zero::zero());
    } else if jnz.len() == 1 {
        let claim = jnz[0];
        claimed_sums.append(*claim.claimed_sum);
    } else {
        panic!("expected up to one component")
    }
    if jnz_taken.is_empty() {
        claimed_sums.append(Zero::zero());
    } else if jnz_taken.len() == 1 {
        let claim = jnz_taken[0];
        claimed_sums.append(*claim.claimed_sum);
    } else {
        panic!("expected up to one component")
    }
    if jump.is_empty() {
        claimed_sums.append(Zero::zero());
    } else if jump.len() == 1 {
        let claim = jump[0];
        claimed_sums.append(*claim.claimed_sum);
    } else {
        panic!("expected up to one component")
    }
    if jump_double_deref.is_empty() {
        claimed_sums.append(Zero::zero());
    } else if jump_double_deref.len() == 1 {
        let claim = jump_double_deref[0];
        claimed_sums.append(*claim.claimed_sum);
    } else {
        panic!("expected up to one component")
    }
    if jump_rel.is_empty() {
        claimed_sums.append(Zero::zero());
    } else if jump_rel.len() == 1 {
        let claim = jump_rel[0];
        claimed_sums.append(*claim.claimed_sum);
    } else {
        panic!("expected up to one component")
    }
    if jump_rel_imm.is_empty() {
        claimed_sums.append(Zero::zero());
    } else if jump_rel_imm.len() == 1 {
        let claim = jump_rel_imm[0];
        claimed_sums.append(*claim.claimed_sum);
    } else {
        panic!("expected up to one component")
    }
    if mul.is_empty() {
        claimed_sums.append(Zero::zero());
    } else if mul.len() == 1 {
        let claim = mul[0];
        claimed_sums.append(*claim.claimed_sum);
    } else {
        panic!("expected up to one component")
    }
    if mul_small.is_empty() {
        claimed_sums.append(Zero::zero());
    } else if mul_small.len() == 1 {
        let claim = mul_small[0];
        claimed_sums.append(*claim.claimed_sum);
    } else {
        panic!("expected up to one component")
    }
    if qm31.is_empty() {
        claimed_sums.append(Zero::zero());
    } else if qm31.len() == 1 {
        let claim = qm31[0];
        claimed_sums.append(*claim.claimed_sum);
    } else {
        panic!("expected up to one component")
    }
    if ret.is_empty() {
        claimed_sums.append(Zero::zero());
    } else if ret.len() == 1 {
        let claim = ret[0];
        claimed_sums.append(*claim.claimed_sum);
    } else {
        panic!("expected up to one component")
    }

    // Verify instruction
    claimed_sums.append(*verify_instruction.claimed_sum);

    // Blake context
    if let Some(BlakeInteractionClaim {
        blake_g, triple_xor_32, blake_round_sigma, blake_round, verify_bitwise_xor_12,
    }) = blake_context.interaction_claim {
        claimed_sums.append(*blake_g.claimed_sum);
        claimed_sums.append(*triple_xor_32.claimed_sum);
        claimed_sums.append(*blake_round_sigma.claimed_sum);
        claimed_sums.append(*blake_round.claimed_sum);
        claimed_sums.append(*verify_bitwise_xor_12.claimed_sum);
    } else {
        claimed_sums
            .extend(
                array![
                    Zero::<QM31>::zero(), Zero::zero(), Zero::zero(), Zero::zero(), Zero::zero(),
                ],
            );
    }

    // Builtins
    let BuiltinsInteractionClaim {
        range_check_128_builtin,
        range_check_96_builtin,
        bitwise_builtin,
        add_mod_builtin,
        mul_mod_builtin,
        pedersen_builtin,
        poseidon_builtin,
    } = builtins;
    if let Some(range_check_128_builtin) = range_check_128_builtin {
        claimed_sums.append(*range_check_128_builtin.claimed_sum);
    } else {
        claimed_sums.append(Zero::zero());
    }
    if let Some(range_check_96_builtin) = range_check_96_builtin {
        claimed_sums.append(*range_check_96_builtin.claimed_sum);
    } else {
        claimed_sums.append(Zero::zero());
    }
    if let Some(bitwise_builtin) = bitwise_builtin {
        claimed_sums.append(*bitwise_builtin.claimed_sum);
    } else {
        claimed_sums.append(Zero::zero());
    }
    if let Some(add_mod_builtin) = add_mod_builtin {
        claimed_sums.append(*add_mod_builtin.claimed_sum);
    } else {
        claimed_sums.append(Zero::zero());
    }
    if let Some(mul_mod_builtin) = mul_mod_builtin {
        claimed_sums.append(*mul_mod_builtin.claimed_sum);
    } else {
        claimed_sums.append(Zero::zero());
    }
    if let Some(pedersen_builtin) = pedersen_builtin {
        claimed_sums.append(*pedersen_builtin.claimed_sum);
    } else {
        claimed_sums.append(Zero::zero());
    }
    if let Some(poseidon_builtin) = poseidon_builtin {
        claimed_sums.append(*poseidon_builtin.claimed_sum);
    } else {
        claimed_sums.append(Zero::zero());
    }

    // Pedersen context
    if let Some(PedersenInteractionClaim {
        pedersen_aggregator, partial_ec_mul, pedersen_points_table,
    }) = pedersen_context.interaction_claim {
        claimed_sums.append(*pedersen_aggregator.claimed_sum);
        claimed_sums.append(*partial_ec_mul.claimed_sum);
        claimed_sums.append(*pedersen_points_table.claimed_sum);
    } else {
        claimed_sums.extend(array![Zero::<QM31>::zero(), Zero::zero(), Zero::zero()]);
    }

    // Poseidon context
    if let Some(PoseidonInteractionClaim {
        poseidon_aggregator,
        poseidon_3_partial_rounds_chain,
        poseidon_full_round_chain,
        cube_252,
        poseidon_round_keys,
        range_check_252_width_27,
    }) = poseidon_context.interaction_claim {
        claimed_sums.append(*poseidon_aggregator.claimed_sum);
        claimed_sums.append(*poseidon_3_partial_rounds_chain.claimed_sum);
        claimed_sums.append(*poseidon_full_round_chain.claimed_sum);
        claimed_sums.append(*cube_252.claimed_sum);
        claimed_sums.append(*poseidon_round_keys.claimed_sum);
        claimed_sums.append(*range_check_252_width_27.claimed_sum);
    } else {
        claimed_sums
            .extend(
                array![
                    Zero::<QM31>::zero(), Zero::zero(), Zero::zero(), Zero::zero(), Zero::zero(),
                    Zero::zero(),
                ],
            );
    }

    // Memory address to id
    claimed_sums.append(*memory_address_to_id.claimed_sum);

    // Memory id to value
    let arr: Array<QM31> = memory_id_to_value.big_claimed_sums.into_iter().map(|x| *x).collect();
    claimed_sums.extend(arr);
    claimed_sums.append(*memory_id_to_value.small_claimed_sum);

    // Range checks
    claimed_sums.append(*range_checks.rc_6.claimed_sum);
    claimed_sums.append(*range_checks.rc_8.claimed_sum);
    claimed_sums.append(*range_checks.rc_11.claimed_sum);
    claimed_sums.append(*range_checks.rc_12.claimed_sum);
    claimed_sums.append(*range_checks.rc_18.claimed_sum);
    claimed_sums.append(*range_checks.rc_20.claimed_sum);
    claimed_sums.append(*range_checks.rc_4_3.claimed_sum);
    claimed_sums.append(*range_checks.rc_4_4.claimed_sum);
    claimed_sums.append(*range_checks.rc_9_9.claimed_sum);
    claimed_sums.append(*range_checks.rc_7_2_5.claimed_sum);
    claimed_sums.append(*range_checks.rc_3_6_6_3.claimed_sum);
    claimed_sums.append(*range_checks.rc_4_4_4_4.claimed_sum);
    claimed_sums.append(*range_checks.rc_3_3_3_3_3.claimed_sum);

    // Verify bitwise xor
    claimed_sums.append(*verify_bitwise_xor_4.claimed_sum);
    claimed_sums.append(*verify_bitwise_xor_7.claimed_sum);
    claimed_sums.append(*verify_bitwise_xor_8.claimed_sum);
    claimed_sums.append(*verify_bitwise_xor_9.claimed_sum);

    claimed_sums.span()
}
