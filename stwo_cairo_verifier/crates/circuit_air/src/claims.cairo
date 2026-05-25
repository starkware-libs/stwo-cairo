use core::array::Span;
use stwo_constraint_framework::claim::ClaimTrait;
use stwo_constraint_framework::{RelationUsesDict, accumulate_relation_uses};
use stwo_verifier_core::TreeArray;
use stwo_verifier_core::channel::{Channel, ChannelTrait};
use stwo_verifier_core::fields::qm31::{QM31, QM31Serde};
use stwo_verifier_core::utils::pack_into_qm31s;
use crate::component_indices::*;
use crate::components::*;

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
