use stwo_constraint_framework::{CommonLookupElements, LookupElementsTrait};
use stwo_verifier_core::channel::{Channel, ChannelTrait};
use stwo_verifier_core::fields::qm31::{QM31, QM31Serde, QM31Trait};
use crate::component_indices::N_COMPONENTS;
use crate::prelude::{Invertible, M31, Zero, m31};
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

/// Circuit interaction claim, holding every component's `claimed_sum` in `ComponentList`
/// (i.e. COMPONENT_IDX) order. The circuit is fixed-size, so all components are always
/// present and the array always has exactly `N_COMPONENTS` entries.
#[derive(Drop, Serde)]
pub struct CircuitInteractionClaim {
    pub claimed_sum: Array<QM31>,
}

#[generate_trait]
pub impl CircuitInteractionClaimImpl of CircuitInteractionClaimTrait {
    fn mix_into(self: @CircuitInteractionClaim, ref channel: Channel) {
        assert!(self.claimed_sum.len() == N_COMPONENTS, "CircuitInteractionClaim: bad length");
        // Mix every component's claimed sum in `ComponentList` order, in a single call.
        channel.mix_felts(self.claimed_sum.span());
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
    let mut component_sum: QM31 = Zero::zero();
    for claimed_sum in interaction_claim.claimed_sum.span() {
        component_sum = component_sum + *claimed_sum;
    }

    // u_sum = the `u` input is yielded at `U_VAR_IDX` with value `U_VALUE = (0, 0, 1, 0)`.
    let gate_relation_id = GATE_RELATION_ID;
    let u_denom = common_lookup_elements
        .combine([gate_relation_id, m31(U_VAR_IDX), m31(0), m31(0), m31(1), m31(0)].span());
    let u_sum = u_denom.inverse();

    // output_sum = Σ (1 / combine([GATE_RELATION_ID, addr, a, b, c, d])).
    // Each public output is a circuit variable whose value is yielded into the `Gate` logup
    // relation, keyed by its variable index (addr).
    let mut output_sum: QM31 = Zero::zero();
    let mut addr: M31 = m31(U_VAR_IDX + 1);
    for value in claim.public_data.output_values.span() {
        let [a, b, c, d] = QM31Trait::to_fixed_array(*value);
        let denom = common_lookup_elements
            .combine([gate_relation_id, addr, a, b, c, d].span());
        output_sum = output_sum + denom.inverse();
        addr += m31(1);
    }

    component_sum + u_sum + output_sum
}
