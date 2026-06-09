use stwo_verifier_core::channel::{Channel, ChannelTrait};
use stwo_verifier_core::fields::qm31::{QM31, QM31Serde};
use crate::components;

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
