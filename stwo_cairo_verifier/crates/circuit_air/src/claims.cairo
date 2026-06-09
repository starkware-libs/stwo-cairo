use stwo_verifier_core::channel::{Channel, ChannelTrait};
use stwo_verifier_core::fields::qm31::{QM31, QM31Serde};
use crate::component_indices::N_COMPONENTS;

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
