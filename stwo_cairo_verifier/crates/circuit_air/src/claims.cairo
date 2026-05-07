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
