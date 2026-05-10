use stwo_constraint_framework::{CommonLookupElements, LookupElementsTrait};
use stwo_verifier_core::channel::{Channel, ChannelTrait};
use stwo_verifier_core::fields::qm31::{QM31, QM31Serde, QM31Trait};
use crate::components;
use crate::prelude::{Invertible, Zero, m31};
use crate::relations::GATE_RELATION_ID;

/// Variable index of the public input `u`.
const U_VAR_IDX: u32 = 2;

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

#[derive(Drop, Serde)]
pub struct CircuitClaim {
    pub public_data: CircuitPublicData,
}

#[generate_trait]
pub impl CircuitClaimImpl of CircuitClaimTrait {
    fn mix_into(self: @CircuitClaim, ref channel: Channel) {
        self.public_data.mix_into(ref channel);
    }
}

/// Structured circuit interaction claim, holding the interaction claim of every
/// component, in `ComponentList` order. The circuit is fixed-size, so all components are
/// always present.
#[derive(Drop, Serde)]
pub struct CircuitInteractionClaim {
    pub eq: components::eq::InteractionClaim,
    pub qm31_ops: components::qm31_ops::InteractionClaim,
    pub triple_xor: components::triple_xor::InteractionClaim,
    pub m_31_to_u_32: components::m_31_to_u_32::InteractionClaim,
    pub blake_g_gate: components::blake_g_gate::InteractionClaim,
    pub verify_bitwise_xor_8: components::verify_bitwise_xor_8::InteractionClaim,
    pub verify_bitwise_xor_12: components::verify_bitwise_xor_12::InteractionClaim,
    pub verify_bitwise_xor_4: components::verify_bitwise_xor_4::InteractionClaim,
    pub verify_bitwise_xor_7: components::verify_bitwise_xor_7::InteractionClaim,
    pub verify_bitwise_xor_9: components::verify_bitwise_xor_9::InteractionClaim,
    pub range_check_16: components::range_check_16::InteractionClaim,
}

#[generate_trait]
pub impl CircuitInteractionClaimImpl of CircuitInteractionClaimTrait {
    fn mix_into(self: @CircuitInteractionClaim, ref channel: Channel) {
        // Mix every component's claimed sum in `ComponentList` order, in a single call.
        channel
            .mix_felts(
                array![
                    *self.eq.claimed_sum, *self.qm31_ops.claimed_sum, *self.triple_xor.claimed_sum,
                    *self.m_31_to_u_32.claimed_sum, *self.blake_g_gate.claimed_sum,
                    *self.verify_bitwise_xor_8.claimed_sum, *self.verify_bitwise_xor_12.claimed_sum,
                    *self.verify_bitwise_xor_4.claimed_sum, *self.verify_bitwise_xor_7.claimed_sum,
                    *self.verify_bitwise_xor_9.claimed_sum, *self.range_check_16.claimed_sum,
                ]
                    .span(),
            );
    }
}

/// Public logup sum determined by the public statement.
//.
///   `component_sum + u_sum + output_sum`
/// where:
///   - `component_sum` is the sum of all components' `claimed_sums`;
///   - `u_sum` is the `u` term at `U_VAR_IDX` with value `U_VALUE = (0, 0, 1, 0)`.
///   - `output_sum` is the sum over all public output values;
///
/// Assumes that the circuit lays out its variables in a fixed order:`var[0] = 0`, `var[1] = 1`,
/// `var[2] = u`, with the public output values placed in the variable slots immediately after `u`.
/// A proof is valid only when this value equals zero.
pub fn lookup_sum(
    claim: @CircuitClaim,
    common_lookup_elements: @CommonLookupElements,
    interaction_claim: @CircuitInteractionClaim,
) -> QM31 {
    // component_sum = Σ claimed_sums.
    let component_sum = *interaction_claim.eq.claimed_sum
        + *interaction_claim.qm31_ops.claimed_sum
        + *interaction_claim.triple_xor.claimed_sum
        + *interaction_claim.m_31_to_u_32.claimed_sum
        + *interaction_claim.blake_g_gate.claimed_sum
        + *interaction_claim.verify_bitwise_xor_8.claimed_sum
        + *interaction_claim.verify_bitwise_xor_12.claimed_sum
        + *interaction_claim.verify_bitwise_xor_4.claimed_sum
        + *interaction_claim.verify_bitwise_xor_7.claimed_sum
        + *interaction_claim.verify_bitwise_xor_9.claimed_sum
        + *interaction_claim.range_check_16.claimed_sum;

    // u_sum = the `u` input is yielded at `U_VAR_IDX` with value `U_VALUE = (0, 0, 1, 0)`.
    let gate_relation_id = GATE_RELATION_ID;
    let u_denom = common_lookup_elements
        .combine([gate_relation_id, m31(U_VAR_IDX), m31(0), m31(0), m31(1), m31(0)].span());
    let u_sum = u_denom.inverse();

    // output_sum = Σ (1 / combine([GATE_RELATION_ID, addr, a, b, c, d])).
    // Each public output is a circuit variable whose value is yielded into the `Gate` logup
    // relation, keyed by its variable index (addr).
    let mut output_sum: QM31 = Zero::zero();
    let mut addr: u32 = U_VAR_IDX + 1;
    for value in claim.public_data.output_values.span() {
        let [a, b, c, d] = QM31Trait::to_fixed_array(*value);
        let denom = common_lookup_elements
            .combine([gate_relation_id, m31(addr), a, b, c, d].span());
        output_sum = output_sum + denom.inverse();
        addr += 1;
    }

    component_sum + u_sum + output_sum
}
