use core::num::traits::Zero;
use stwo_constraint_framework::claim::ClaimTrait;
use stwo_constraint_framework::{
    CommonLookupElements, LookupElementsImpl, RelationUsesDict, tree_array_concat_cols,
};
use stwo_verifier_core::TreeArray;
use stwo_verifier_core::channel::{Channel, ChannelTrait};
use stwo_verifier_core::fields::Invertible;
use stwo_verifier_core::fields::m31::{M31, m31};
use stwo_verifier_core::fields::qm31::{QM31, QM31Serde, QM31Trait};
use stwo_verifier_core::utils::pack_into_qm31s;
use stwo_verifier_utils::zip_eq::zip_eq;
use crate::CircuitVerifierConfig;
use crate::blake2s_consts::blake2s_initial_state;
use crate::components::{
    blake_g, blake_gate, blake_output, blake_round, blake_round_sigma, eq, m_31_to_u_32, qm31_ops,
    range_check_15, range_check_16, triple_xor_32, verify_bitwise_xor_12, verify_bitwise_xor_4,
    verify_bitwise_xor_7, verify_bitwise_xor_8, verify_bitwise_xor_9,
};
use crate::relations::{BLAKE_STATE_RELATION_ID, GATE_RELATION_ID};

/// Number of components in the circuit AIR.
pub const N_CIRCUIT_COMPONENTS: u32 = 16;

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

#[derive(Drop, Serde)]
pub struct CircuitClaim {
    pub public_data: CircuitPublicData,
    /// Per-component `log_size` values in canonical `ComponentList` order
    /// (matches `circuit_components::ComponentList`). Length is `N_CIRCUIT_COMPONENTS`.
    /// Including components whose log size is fixed.
    pub log_sizes: Array<u32>,
}

pub impl CircuitClaimImpl of ClaimTrait<CircuitClaim> {
    fn mix_into(self: @CircuitClaim, ref channel: Channel) {
        channel.mix_felts(pack_into_qm31s(array![N_CIRCUIT_COMPONENTS].span()));
        channel.mix_felts(pack_into_qm31s(array![1_u32; N_CIRCUIT_COMPONENTS].span()));
        channel.mix_felts(pack_into_qm31s(self.log_sizes.span()));
        channel.mix_felts(self.public_data.output_values.span());
    }

    // TODO(Gali): Don't use the component claims here, but rather add interaction/trace_columns()
    // to AirComponentTrait and use it directly.
    fn log_sizes(self: @CircuitClaim) -> TreeArray<Span<u32>> {
        let mut log_sizes_list = array![];
        log_sizes_list.append(eq::Claim { log_size: *self.log_sizes.at(0) }.log_sizes());
        log_sizes_list.append(qm31_ops::Claim { log_size: *self.log_sizes.at(1) }.log_sizes());
        log_sizes_list.append(blake_gate::Claim { log_size: *self.log_sizes.at(2) }.log_sizes());
        log_sizes_list.append(blake_round::Claim { log_size: *self.log_sizes.at(3) }.log_sizes());
        log_sizes_list.append(blake_round_sigma::Claim {}.log_sizes());
        log_sizes_list.append(blake_g::Claim { log_size: *self.log_sizes.at(5) }.log_sizes());
        log_sizes_list.append(blake_output::Claim { log_size: *self.log_sizes.at(6) }.log_sizes());
        log_sizes_list.append(triple_xor_32::Claim { log_size: *self.log_sizes.at(7) }.log_sizes());
        log_sizes_list.append(m_31_to_u_32::Claim { log_size: *self.log_sizes.at(8) }.log_sizes());
        log_sizes_list.append(verify_bitwise_xor_8::Claim {}.log_sizes());
        log_sizes_list.append(verify_bitwise_xor_12::Claim {}.log_sizes());
        log_sizes_list.append(verify_bitwise_xor_4::Claim {}.log_sizes());
        log_sizes_list.append(verify_bitwise_xor_7::Claim {}.log_sizes());
        log_sizes_list.append(verify_bitwise_xor_9::Claim {}.log_sizes());
        log_sizes_list.append(range_check_15::Claim {}.log_sizes());
        log_sizes_list.append(range_check_16::Claim {}.log_sizes());
        tree_array_concat_cols(log_sizes_list)
    }

    // TODO(Gali): Consider taking out of the component claims.
    fn accumulate_relation_uses(self: @CircuitClaim, ref relation_uses: RelationUsesDict) {
        eq::Claim { log_size: *self.log_sizes.at(0) }.accumulate_relation_uses(ref relation_uses);
        qm31_ops::Claim { log_size: *self.log_sizes.at(1) }
            .accumulate_relation_uses(ref relation_uses);
        blake_gate::Claim { log_size: *self.log_sizes.at(2) }
            .accumulate_relation_uses(ref relation_uses);
        blake_round::Claim { log_size: *self.log_sizes.at(3) }
            .accumulate_relation_uses(ref relation_uses);
        blake_round_sigma::Claim {}.accumulate_relation_uses(ref relation_uses);
        blake_g::Claim { log_size: *self.log_sizes.at(5) }
            .accumulate_relation_uses(ref relation_uses);
        blake_output::Claim { log_size: *self.log_sizes.at(6) }
            .accumulate_relation_uses(ref relation_uses);
        triple_xor_32::Claim { log_size: *self.log_sizes.at(7) }
            .accumulate_relation_uses(ref relation_uses);
        m_31_to_u_32::Claim { log_size: *self.log_sizes.at(8) }
            .accumulate_relation_uses(ref relation_uses);
        verify_bitwise_xor_8::Claim {}.accumulate_relation_uses(ref relation_uses);
        verify_bitwise_xor_12::Claim {}.accumulate_relation_uses(ref relation_uses);
        verify_bitwise_xor_4::Claim {}.accumulate_relation_uses(ref relation_uses);
        verify_bitwise_xor_7::Claim {}.accumulate_relation_uses(ref relation_uses);
        verify_bitwise_xor_9::Claim {}.accumulate_relation_uses(ref relation_uses);
        range_check_15::Claim {}.accumulate_relation_uses(ref relation_uses);
        range_check_16::Claim {}.accumulate_relation_uses(ref relation_uses);
    }
}

#[derive(Drop, Serde)]
pub struct CircuitInteractionClaim {
    /// Per-component `claimed_sum` values in canonical `ComponentList` order. Length is
    /// `N_CIRCUIT_COMPONENTS`.
    pub claimed_sums: Array<QM31>,
}

#[generate_trait]
pub impl CircuitInteractionClaimImpl of CircuitInteractionClaimTrait {
    fn mix_into(self: @CircuitInteractionClaim, ref channel: Channel) {
        channel.mix_felts(self.claimed_sums.span());
    }
}

/// Public logup sum determined by the public statement.
///
/// Ports `stwo-circuits/crates/circuit_verifier/src/circuit_claim.rs::lookup_sum`:
///   `component_sum + output_sum − blake_iv_sum`
/// where:
///   - `component_sum` is the sum of all components' `claimed_sums`;
///   - `output_sum` is the sum of `1 / combine(GATE_RELATION_ID, addr, a, b, c, d)` over each
///     `(output_address, output_value)` pair (QM31 decomposed into its four M31 limbs
///     `(a, b, c, d)`);
///   - `blake_iv_sum` is subtracted because the Blake IV state is USED by the Blake gates
///     but never YIELDED by any gate; it must be cancelled out of the public sum.
///
/// A proof is valid only when this value equals zero.
pub fn lookup_sum(
    claim: @CircuitClaim,
    common_lookup_elements: @CommonLookupElements,
    interaction_claim: @CircuitInteractionClaim,
    config: @CircuitVerifierConfig,
) -> QM31 {
    // component_sum = Σ claimed_sums[i]
    let mut component_sum: QM31 = Zero::zero();
    for s in interaction_claim.claimed_sums.span() {
        component_sum = component_sum + *s;
    }

    // Compute the public logup sum from output gates.
    // The prover yields each output value at the output gate. Each contributes
    // 1 / combine([GATE_RELATION_ID, addr, a, b, c, d]).
    let mut output_sum: QM31 = Zero::zero();
    let gate_relation_id = GATE_RELATION_ID;
    for (addr, value) in zip_eq(
        config.output_addresses.span(), claim.public_data.output_values.span(),
    ) {
        let addr_m31: M31 = m31(*addr);
        let [a, b, c, d] = QM31Trait::to_fixed_array(*value);
        let denom = common_lookup_elements.combine([gate_relation_id, addr_m31, a, b, c, d].span());
        output_sum = output_sum + denom.inverse();
    }

    // Subtract the Blake IV public logup sum (Blake IV state is used but never yielded).
    let blake_iv_sum = blake_iv_public_logup_sum(*config.n_blake_gates, common_lookup_elements);

    component_sum + output_sum - blake_iv_sum;
}

/// Each Blake gate uses the initial state once and creates one row in `blake_output`. Then
/// `blake_output` is padded to a power of two, and each padding row uses the initial state
/// once — total `next_power_of_two(n_blake_gates)` uses.
fn blake_iv_public_logup_sum(
    n_blake_gates: u32, common_lookup_elements: @CommonLookupElements,
) -> QM31 {
    let initial_state_uses: u32 = next_power_of_two(n_blake_gates);

    let [w0, w1, w2, w3, w4, w5, w6, w7] = blake2s_initial_state();
    let initial_state_limbs = array![m31 (
        w0 & 0xffff,
    ), m31 ((w0 > > 16) & 0xffff), m31 (
        w1 & 0xffff,
    ), m31 ((w1 > > 16) & 0xffff), m31 (
        w2 & 0xffff,
    ), m31 ((w2 > > 16) & 0xffff), m31 (
        w3 & 0xffff,
    ), m31 ((w3 > > 16) & 0xffff), m31 (
        w4 & 0xffff,
    ), m31 ((w4 > > 16) & 0xffff), m31 (
        w5 & 0xffff,
    ), m31 ((w5 > > 16) & 0xffff), m31 (
        w6 & 0xffff,
    ), m31 ((w6 > > 16) & 0xffff), m31 (w7 & 0xffff), m31 ((w7 > > 16) & 0xffff),]
        .span();

    let denom = common_lookup_elements
        .combine(
            [
                BLAKE_STATE_RELATION_ID, m31(0), *initial_state_limbs.at(0),
                *initial_state_limbs.at(1), *initial_state_limbs.at(2), *initial_state_limbs.at(3),
                *initial_state_limbs.at(4), *initial_state_limbs.at(5), *initial_state_limbs.at(6),
                *initial_state_limbs.at(7), *initial_state_limbs.at(8), *initial_state_limbs.at(9),
                *initial_state_limbs.at(10), *initial_state_limbs.at(11),
                *initial_state_limbs.at(12), *initial_state_limbs.at(13),
                *initial_state_limbs.at(14), *initial_state_limbs.at(15),
            ]
                .span(),
        );
    denom.inverse() * m31(initial_state_uses).into()
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
