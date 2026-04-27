use core::num::traits::Zero;
use stwo_constraint_framework::{
    CommonLookupElements, LookupElementsImpl, RelationUsesDict, tree_array_concat_cols,
};
use stwo_verifier_core::TreeArray;
use stwo_verifier_core::channel::{Channel, ChannelTrait};
use stwo_verifier_core::fields::Invertible;
use stwo_verifier_core::fields::m31::{M31, m31};
use stwo_verifier_core::fields::qm31::{QM31, QM31Serde, QM31Trait};
use stwo_verifier_core::utils::pack_into_qm31s;
use crate::CircuitVerifierConfig;
use crate::blake2s_consts::blake2s_initial_state;
use crate::claim::ClaimTrait;
use crate::components::{
    blake_g, blake_gate, blake_output, blake_round, blake_round_sigma, eq, m_31_to_u_32, qm31_ops,
    range_check_15, range_check_16, triple_xor_32, verify_bitwise_xor_12, verify_bitwise_xor_4,
    verify_bitwise_xor_7, verify_bitwise_xor_8, verify_bitwise_xor_9,
};
use crate::relations::{BLAKE_STATE_RELATION_ID, GATE_RELATION_ID};

/// Public data for a circuit proof: the values produced at each `Output` gate of the circuit.
#[derive(Drop, Serde)]
pub struct CircuitPublicData {
    pub output_values: Array<QM31>,
}

#[generate_trait]
pub impl CircuitPublicDataImpl of CircuitPublicDataTrait {
    fn mix_into(self: @CircuitPublicData, ref channel: Channel) {
        channel.mix_felts(self.output_values.span());
    }
}

/// Number of components in the circuit AIR.
pub const N_CIRCUIT_COMPONENTS: u32 = 16;

/// Per-component claims for the circuit AIR. All 16 components are always present
/// (unlike the Cairo AIR, which has optional components).
#[derive(Drop, Serde)]
pub struct CircuitClaim {
    pub public_data: CircuitPublicData,
    pub eq: eq::Claim,
    pub qm31_ops: qm31_ops::Claim,
    pub blake_gate: blake_gate::Claim,
    pub blake_round: blake_round::Claim,
    pub blake_round_sigma: blake_round_sigma::Claim,
    pub blake_g: blake_g::Claim,
    pub blake_output: blake_output::Claim,
    pub triple_xor_32: triple_xor_32::Claim,
    pub m_31_to_u_32: m_31_to_u_32::Claim,
    pub verify_bitwise_xor_8: verify_bitwise_xor_8::Claim,
    pub verify_bitwise_xor_12: verify_bitwise_xor_12::Claim,
    pub verify_bitwise_xor_4: verify_bitwise_xor_4::Claim,
    pub verify_bitwise_xor_7: verify_bitwise_xor_7::Claim,
    pub verify_bitwise_xor_9: verify_bitwise_xor_9::Claim,
    pub range_check_15: range_check_15::Claim,
    pub range_check_16: range_check_16::Claim,
}

pub impl CircuitClaimImpl of ClaimTrait<CircuitClaim> {
    fn log_sizes(self: @CircuitClaim) -> TreeArray<Span<u32>> {
        let mut log_sizes_list = array![];
        log_sizes_list.append(self.eq.log_sizes());
        log_sizes_list.append(self.qm31_ops.log_sizes());
        log_sizes_list.append(self.blake_gate.log_sizes());
        log_sizes_list.append(self.blake_round.log_sizes());
        log_sizes_list.append(self.blake_round_sigma.log_sizes());
        log_sizes_list.append(self.blake_g.log_sizes());
        log_sizes_list.append(self.blake_output.log_sizes());
        log_sizes_list.append(self.triple_xor_32.log_sizes());
        log_sizes_list.append(self.m_31_to_u_32.log_sizes());
        log_sizes_list.append(self.verify_bitwise_xor_8.log_sizes());
        log_sizes_list.append(self.verify_bitwise_xor_12.log_sizes());
        log_sizes_list.append(self.verify_bitwise_xor_4.log_sizes());
        log_sizes_list.append(self.verify_bitwise_xor_7.log_sizes());
        log_sizes_list.append(self.verify_bitwise_xor_9.log_sizes());
        log_sizes_list.append(self.range_check_15.log_sizes());
        log_sizes_list.append(self.range_check_16.log_sizes());
        // Aggregate raw per-component log sizes. The preprocessed-tree entry of this
        // result is a concatenation of per-component preprocessed columns and does NOT
        // reflect the actual flat preprocessed trace; callers must override it via
        // `override_preprocessed_trace_log_sizes` using the real preprocessed column
        // layout supplied by the verifier config.
        tree_array_concat_cols(log_sizes_list)
    }

    /// Channel-mixing pattern matching
    /// `stwo-circuits/crates/circuit_air/src/circuit_claim.rs::CircuitClaim::mix_into`:
    ///   1. n_components (as a QM31)
    ///   2. enable bits (all 16 true), packed into QM31s
    ///   3. per-component log_sizes, packed into QM31s
    ///   4. public output_values (already QM31)
    fn mix_into(self: @CircuitClaim, ref channel: Channel) {
        channel.mix_felts(pack_into_qm31s(array![N_CIRCUIT_COMPONENTS].span()));
        channel.mix_felts(pack_into_qm31s(circuit_enable_bits_u32s().span()));
        channel.mix_felts(pack_into_qm31s(circuit_component_log_sizes(self).span()));
        channel.mix_felts(self.public_data.output_values.span());
    }

    fn accumulate_relation_uses(self: @CircuitClaim, ref relation_uses: RelationUsesDict) {
        self.eq.accumulate_relation_uses(ref relation_uses);
        self.qm31_ops.accumulate_relation_uses(ref relation_uses);
        self.blake_gate.accumulate_relation_uses(ref relation_uses);
        self.blake_round.accumulate_relation_uses(ref relation_uses);
        self.blake_round_sigma.accumulate_relation_uses(ref relation_uses);
        self.blake_g.accumulate_relation_uses(ref relation_uses);
        self.blake_output.accumulate_relation_uses(ref relation_uses);
        self.triple_xor_32.accumulate_relation_uses(ref relation_uses);
        self.m_31_to_u_32.accumulate_relation_uses(ref relation_uses);
        self.verify_bitwise_xor_8.accumulate_relation_uses(ref relation_uses);
        self.verify_bitwise_xor_12.accumulate_relation_uses(ref relation_uses);
        self.verify_bitwise_xor_4.accumulate_relation_uses(ref relation_uses);
        self.verify_bitwise_xor_7.accumulate_relation_uses(ref relation_uses);
        self.verify_bitwise_xor_9.accumulate_relation_uses(ref relation_uses);
        self.range_check_15.accumulate_relation_uses(ref relation_uses);
        self.range_check_16.accumulate_relation_uses(ref relation_uses);
    }
}

#[derive(Drop, Serde)]
pub struct CircuitInteractionClaim {
    pub eq: eq::InteractionClaim,
    pub qm31_ops: qm31_ops::InteractionClaim,
    pub blake_gate: blake_gate::InteractionClaim,
    pub blake_round: blake_round::InteractionClaim,
    pub blake_round_sigma: blake_round_sigma::InteractionClaim,
    pub blake_g: blake_g::InteractionClaim,
    pub blake_output: blake_output::InteractionClaim,
    pub triple_xor_32: triple_xor_32::InteractionClaim,
    pub m_31_to_u_32: m_31_to_u_32::InteractionClaim,
    pub verify_bitwise_xor_8: verify_bitwise_xor_8::InteractionClaim,
    pub verify_bitwise_xor_12: verify_bitwise_xor_12::InteractionClaim,
    pub verify_bitwise_xor_4: verify_bitwise_xor_4::InteractionClaim,
    pub verify_bitwise_xor_7: verify_bitwise_xor_7::InteractionClaim,
    pub verify_bitwise_xor_9: verify_bitwise_xor_9::InteractionClaim,
    pub range_check_15: range_check_15::InteractionClaim,
    pub range_check_16: range_check_16::InteractionClaim,
}

#[generate_trait]
pub impl CircuitInteractionClaimImpl of CircuitInteractionClaimTrait {
    /// Matches `CircuitInteractionClaim::mix_into` in
    /// `stwo-circuits/crates/circuit_air/src/circuit_claim.rs`: mix all `claimed_sums` in
    /// a single `mix_felts` call.
    fn mix_into(self: @CircuitInteractionClaim, ref channel: Channel) {
        channel.mix_felts(self.claimed_sums().span());
    }

    fn claimed_sums(self: @CircuitInteractionClaim) -> Array<QM31> {
        array![
            *self.eq.claimed_sum, *self.qm31_ops.claimed_sum, *self.blake_gate.claimed_sum,
            *self.blake_round.claimed_sum, *self.blake_round_sigma.claimed_sum,
            *self.blake_g.claimed_sum, *self.blake_output.claimed_sum,
            *self.triple_xor_32.claimed_sum, *self.m_31_to_u_32.claimed_sum,
            *self.verify_bitwise_xor_8.claimed_sum, *self.verify_bitwise_xor_12.claimed_sum,
            *self.verify_bitwise_xor_4.claimed_sum, *self.verify_bitwise_xor_7.claimed_sum,
            *self.verify_bitwise_xor_9.claimed_sum, *self.range_check_15.claimed_sum,
            *self.range_check_16.claimed_sum,
        ]
    }
}

/// The 16-element `[true; N_CIRCUIT_COMPONENTS]` expressed as `u32` `{0, 1}` values, ready
/// for `pack_into_qm31s`. All circuit components are always enabled.
fn circuit_enable_bits_u32s() -> Array<u32> {
    array![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
}

/// Collects the per-component `log_size` values in canonical `ComponentList` order
/// (matches `stwo-circuits/crates/circuit_air/src/circuit_components.rs::ComponentList`).
fn circuit_component_log_sizes(claim: @CircuitClaim) -> Array<u32> {
    array![
        *claim.eq.log_size, *claim.qm31_ops.log_size, *claim.blake_gate.log_size,
        *claim.blake_round.log_size, *claim.blake_round_sigma.log_size, *claim.blake_g.log_size,
        *claim.blake_output.log_size, *claim.triple_xor_32.log_size, *claim.m_31_to_u_32.log_size,
        *claim.verify_bitwise_xor_8.log_size, *claim.verify_bitwise_xor_12.log_size,
        *claim.verify_bitwise_xor_4.log_size, *claim.verify_bitwise_xor_7.log_size,
        *claim.verify_bitwise_xor_9.log_size, *claim.range_check_15.log_size,
        *claim.range_check_16.log_size,
    ]
}

/// Public logup sum determined by the public statement.
///
/// Ports `stwo-circuits/crates/circuit_air/src/circuit_claim.rs::lookup_sum`:
///   component_sum + output_sum − blake_iv_sum
/// where:
///   - `component_sum` is the sum of all components' claimed sums;
///   - `output_sum` is the sum of `1 / combine(GATE_RELATION_ID, addr, a, b, c, d)` over each
///     `(output_address, output_value)` pair (QM31 decomposed into its four M31 limbs
///     `(a, b, c, d)`);
///   - `blake_iv_sum` is subtracted because the Blake IV state is USED by the Blake gates but
///     never YIELDED by any gate; it must be cancelled out of the public sum.
///
/// A proof is valid only when this value equals zero.
pub fn lookup_sum(
    claim: @CircuitClaim,
    common_lookup_elements: @CommonLookupElements,
    interaction_claim: @CircuitInteractionClaim,
    config: @CircuitVerifierConfig,
) -> QM31 {
    // component_sum = Σ claimed_sums[i]
    let mut sum: QM31 = Zero::zero();
    for s in interaction_claim.claimed_sums() {
        sum = sum + s;
    }

    // output_sum: the prover yields each output value at the output gate. Each contributes
    // 1 / combine([GATE_RELATION_ID, addr, a, b, c, d]).
    let gate_relation_id = GATE_RELATION_ID;
    let output_values = claim.public_data.output_values.span();
    let output_addresses = config.output_addresses.span();
    assert!(output_values.len() == output_addresses.len());
    let mut i = 0;
    while i != output_values.len() {
        let addr_m31: M31 = m31(*output_addresses.at(i));
        let [a, b, c, d] = QM31Trait::to_fixed_array(*output_values.at(i));
        let denom = common_lookup_elements.combine([gate_relation_id, addr_m31, a, b, c, d].span());
        sum = sum + denom.inverse();
        i = i + 1;
    }

    // blake_iv_sum: Blake IV state is used once per BlakeOutput row. BlakeOutput is padded
    // to a power of two, so the total uses count is `next_power_of_two(n_blake_gates)`.
    if *config.n_blake_gates > 0 {
        let n_iv_uses: u32 = next_power_of_two(*config.n_blake_gates);
        let state_limbs = blake_iv_limbs();
        let denom = common_lookup_elements
            .combine(
                [
                    BLAKE_STATE_RELATION_ID, m31(0), // iv_state_address
                    *state_limbs.at(0),
                    *state_limbs.at(1), *state_limbs.at(2), *state_limbs.at(3), *state_limbs.at(4),
                    *state_limbs.at(5), *state_limbs.at(6), *state_limbs.at(7), *state_limbs.at(8),
                    *state_limbs.at(9), *state_limbs.at(10), *state_limbs.at(11),
                    *state_limbs.at(12), *state_limbs.at(13), *state_limbs.at(14),
                    *state_limbs.at(15),
                ]
                    .span(),
            );
        let n_iv_uses_m31: M31 = m31(n_iv_uses);
        sum = sum - denom.inverse() * n_iv_uses_m31.into();
    }

    sum
}

/// Returns the Blake2s initial-state 16-bit limbs (low, high) in the order expected by
/// the `BLAKE_STATE_RELATION_ID` lookup.
fn blake_iv_limbs() -> Array<M31> {
    let initial_state = blake2s_initial_state();
    let [w0, w1, w2, w3, w4, w5, w6, w7] = initial_state;
    array![
        m31(w0 & 0xffff), m31((w0 / 0x10000) & 0xffff), m31(w1 & 0xffff),
        m31((w1 / 0x10000) & 0xffff), m31(w2 & 0xffff), m31((w2 / 0x10000) & 0xffff),
        m31(w3 & 0xffff), m31((w3 / 0x10000) & 0xffff), m31(w4 & 0xffff),
        m31((w4 / 0x10000) & 0xffff), m31(w5 & 0xffff), m31((w5 / 0x10000) & 0xffff),
        m31(w6 & 0xffff), m31((w6 / 0x10000) & 0xffff), m31(w7 & 0xffff),
        m31((w7 / 0x10000) & 0xffff),
    ]
}

/// Smallest power of two >= `x`, with `next_power_of_two(0) = 1`.
fn next_power_of_two(x: u32) -> u32 {
    if x <= 1 {
        return 1;
    }
    let mut p: u32 = 1;
    while p < x {
        p = p * 2;
    }
    p
}
