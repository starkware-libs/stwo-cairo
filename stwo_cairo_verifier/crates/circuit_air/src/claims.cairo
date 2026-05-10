use core::array::Span;
use stwo_constraint_framework::claim::ClaimTrait;
use stwo_constraint_framework::{
    CommonLookupElements, LookupElementsTrait, RelationUsesDict, accumulate_relation_uses,
};
use stwo_verifier_core::TreeArray;
use stwo_verifier_core::channel::{Channel, ChannelTrait};
use stwo_verifier_core::fields::qm31::{QM31, QM31Serde, QM31Trait};
use stwo_verifier_core::utils::pack_into_qm31s;
use stwo_verifier_utils::zip_eq::zip_eq;
use crate::blake2s_consts::blake2s_initial_state;
use crate::component_indices::*;
use crate::components::*;
use crate::prelude::{Invertible, M31, Zero, m31};
use crate::relations::{BLAKE_STATE_RELATION_ID, GATE_RELATION_ID};

#[derive(Clone, Drop, Serde)]
pub struct CircuitPublicData {
    pub output_values: Array<QM31>,
}

#[generate_trait]
pub impl CircuitPublicDataImpl of CircuitPublicDataTrait {
    fn mix_into(self: @CircuitPublicData, ref channel: Channel) {
        channel.mix_felts(self.output_values.span());
    }
}

#[derive(Drop, Serde, Clone)]
pub struct CircuitClaim {
    pub component_log_sizes: Array<Option<u32>>,
    pub public_data: CircuitPublicData,
}

#[derive(Drop, Serde)]
pub struct CircuitInteractionClaim {
    pub component_claimed_sums: Array<Option<QM31>>,
}

pub impl CircuitClaimImpl of ClaimTrait<CircuitClaim> {
    fn mix_into(self: @CircuitClaim, ref channel: Channel) {
        let log_sizes = self.component_log_sizes.span();
        let mut enable_bits_as_u32s = array![];
        let mut dense_log_sizes = array![];
        for opt in log_sizes {
            if let Option::Some(log_size) = *opt {
                enable_bits_as_u32s.append(1_u32);
                dense_log_sizes.append(log_size);
            } else {
                enable_bits_as_u32s.append(0_u32);
            }
        }
        channel.mix_felts(pack_into_qm31s(array![log_sizes.len()].span()));
        channel.mix_felts(pack_into_qm31s(enable_bits_as_u32s.span()));
        channel.mix_felts(pack_into_qm31s(dense_log_sizes.span()));
        self.public_data.mix_into(ref channel);
    }

    fn log_sizes(self: @CircuitClaim) -> TreeArray<Span<u32>> {
        let log_size_per_component = self.component_log_sizes.span();
        let mut trace_log_sizes = array![];
        let mut interaction_trace_log_sizes = array![];
        for component_idx in 0..N_COMPONENTS {
            if let Option::Some(log_size) = *log_size_per_component.at(component_idx) {
                for _ in 0..*N_TRACE_COLUMNS_PER_COMPONENT_IDX.span().at(component_idx) {
                    trace_log_sizes.append(log_size);
                }
                for _ in 0..*N_INTERACTION_COLUMNS_PER_COMPONENT_IDX.span().at(component_idx) {
                    interaction_trace_log_sizes.append(log_size);
                }
            }
        }
        array![array![].span(), trace_log_sizes.span(), interaction_trace_log_sizes.span()]
    }

    fn accumulate_relation_uses(self: @CircuitClaim, ref relation_uses: RelationUsesDict) {
        // Const-size components export no RELATION_USES_PER_ROW, so they are skipped here.
        if let Option::Some(log_size) = *self.component_log_sizes.at(QM31_OPS_COMPONENT_IDX) {
            accumulate_relation_uses(
                ref relation_uses, qm31_ops::RELATION_USES_PER_ROW.span(), log_size,
            );
        }
        if let Option::Some(log_size) = *self.component_log_sizes.at(BLAKE_G_GATE_COMPONENT_IDX) {
            accumulate_relation_uses(
                ref relation_uses, blake_g_gate::RELATION_USES_PER_ROW.span(), log_size,
            );
        }
        if let Option::Some(log_size) = *self.component_log_sizes.at(M_31_TO_U_32_COMPONENT_IDX) {
            accumulate_relation_uses(
                ref relation_uses, m_31_to_u_32::RELATION_USES_PER_ROW.span(), log_size,
            );
        }
        if let Option::Some(log_size) = *self.component_log_sizes.at(TRIPLE_XOR_COMPONENT_IDX) {
            accumulate_relation_uses(
                ref relation_uses, triple_xor::RELATION_USES_PER_ROW.span(), log_size,
            );
        }
    }
}

#[generate_trait]
pub impl CircuitInteractionClaimImpl of CircuitInteractionClaimTrait {
    fn mix_into(self: @CircuitInteractionClaim, ref channel: Channel) {
        // Preserve the previous wire format: mix the dense list of present claimed sums.
        let mut dense_claimed_sums = array![];
        for opt in self.component_claimed_sums.span() {
            if let Option::Some(cs) = *opt {
                dense_claimed_sums.append(cs);
            }
        }
        channel.mix_felts(dense_claimed_sums.span());
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
    output_addresses: Array<u32>,
    n_blake_gates: u32,
) -> QM31 {
    // component_sum = Σ claimed_sums[i]
    let mut component_sum: QM31 = Zero::zero();
    for s in interaction_claim.component_claimed_sums.span() {
        if let Option::Some(cs) = *s {
            component_sum = component_sum + cs;
        }
    }

    // Compute the public logup sum from output gates.
    // The prover yields each output value at the output gate. Each contributes
    // 1 / combine([GATE_RELATION_ID, addr, a, b, c, d]).
    let mut output_sum: QM31 = Zero::zero();
    let gate_relation_id = GATE_RELATION_ID;
    for (addr, value) in zip_eq(output_addresses.span(), claim.public_data.output_values.span()) {
        let addr_m31: M31 = m31(*addr);
        let [a, b, c, d] = QM31Trait::to_fixed_array(*value);
        let denom = common_lookup_elements.combine([gate_relation_id, addr_m31, a, b, c, d].span());
        output_sum = output_sum + denom.inverse();
    }

    // Subtract the Blake IV public logup sum (Blake IV state is used but never yielded).
    let blake_iv_sum = blake_iv_public_logup_sum(n_blake_gates, common_lookup_elements);

    component_sum + output_sum - blake_iv_sum
}

/// Each Blake gate uses the initial state once and creates one row in `blake_output`. Then
/// `blake_output` is padded to a power of two, and each padding row uses the initial state
/// once — total `next_power_of_two(n_blake_gates)` uses.
fn blake_iv_public_logup_sum(
    n_blake_gates: u32, common_lookup_elements: @CommonLookupElements,
) -> QM31 {
    let initial_state_uses: u32 = next_power_of_two(n_blake_gates);

    let [w0, w1, w2, w3, w4, w5, w6, w7] = blake2s_initial_state();
    let denom = common_lookup_elements
        .combine(
            [
                BLAKE_STATE_RELATION_ID, m31(0), m31(w0 & 0xffff), m31((w0 / 0x10000) & 0xffff),
                m31(w1 & 0xffff), m31((w1 / 0x10000) & 0xffff), m31(w2 & 0xffff),
                m31((w2 / 0x10000) & 0xffff), m31(w3 & 0xffff), m31((w3 / 0x10000) & 0xffff),
                m31(w4 & 0xffff), m31((w4 / 0x10000) & 0xffff), m31(w5 & 0xffff),
                m31((w5 / 0x10000) & 0xffff), m31(w6 & 0xffff), m31((w6 / 0x10000) & 0xffff),
                m31(w7 & 0xffff), m31((w7 / 0x10000) & 0xffff),
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
