use stwo_constraint_framework::{
    CommonLookupElements, LookupElementsTrait, RelationUsesDict, accumulate_relation_uses,
};
use stwo_verifier_core::TreeArray;
use stwo_verifier_core::channel::{Channel, ChannelTrait};
use stwo_verifier_core::fields::qm31::{QM31, QM31Serde, QM31Trait};
use stwo_verifier_utils::zip_eq::zip_eq;
use crate::component_indices::{
    BLAKE_G_GATE_COMPONENT_IDX, EQ_COMPONENT_IDX, M_31_TO_U_32_COMPONENT_IDX, N_COMPONENTS,
    N_INTERACTION_COLUMNS_PER_COMPONENT_IDX, N_TRACE_COLUMNS_PER_COMPONENT_IDX,
    QM31_OPS_COMPONENT_IDX, TRIPLE_XOR_COMPONENT_IDX,
};
use crate::components;
use crate::prelude::{Invertible, Zero, m31};
use crate::preprocessed_columns::{
    BLAKE_G_GATE_INPUT_ADDR_A_IDX, EQ_IN0_ADDRESS_IDX, M_31_TO_U_32_INPUT_ADDR_IDX, OP_0_ADDR_IDX,
    TRIPLE_XOR_INPUT_ADDR_0_IDX,
};
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

/// Derives each component's log size from the preprocessed column log sizes (supplied by the
/// verifier config). variable-size components read the log size of one of their preprocessed
/// columns (all columns of a component share its log size), and fixed-size components return their
/// `LOG_SIZE` constant. Indexed by COMPONENT_IDX.
pub fn derive_component_log_sizes(preprocessed_column_log_sizes: Span<u32>) -> [u32; N_COMPONENTS] {
    [
        *preprocessed_column_log_sizes.at(EQ_IN0_ADDRESS_IDX),
        *preprocessed_column_log_sizes.at(OP_0_ADDR_IDX),
        *preprocessed_column_log_sizes.at(TRIPLE_XOR_INPUT_ADDR_0_IDX),
        *preprocessed_column_log_sizes.at(M_31_TO_U_32_INPUT_ADDR_IDX),
        *preprocessed_column_log_sizes.at(BLAKE_G_GATE_INPUT_ADDR_A_IDX),
        components::verify_bitwise_xor_8::LOG_SIZE, components::verify_bitwise_xor_12::LOG_SIZE,
        components::verify_bitwise_xor_4::LOG_SIZE, components::verify_bitwise_xor_7::LOG_SIZE,
        components::verify_bitwise_xor_9::LOG_SIZE, components::range_check_16::LOG_SIZE,
    ]
}

/// Builds `[preprocessed (empty placeholder), trace, interaction]` column log sizes from the
/// per-component log sizes, repeating each component's log size by its trace/interaction column
/// count, in COMPONENT_IDX order. The preprocessed tree is replaced separately via
/// `override_preprocessed_trace_log_sizes`.
pub fn column_log_sizes_per_tree(component_log_sizes: [u32; N_COMPONENTS]) -> TreeArray<Span<u32>> {
    let mut trace_log_sizes = array![];
    let mut interaction_log_sizes = array![];
    for (log_size, (n_trace, n_interaction)) in zip_eq(
        component_log_sizes.span(),
        zip_eq(
            N_TRACE_COLUMNS_PER_COMPONENT_IDX.span(),
            N_INTERACTION_COLUMNS_PER_COMPONENT_IDX.span(),
        ),
    ) {
        for _ in 0..*n_trace {
            trace_log_sizes.append(*log_size);
        }
        for _ in 0..*n_interaction {
            interaction_log_sizes.append(*log_size);
        }
    }
    array![array![].span(), trace_log_sizes.span(), interaction_log_sizes.span()]
}

/// Accumulates lookup-relation uses across components from the derived per-component log sizes.
/// Only the variable-size components export `RELATION_USES_PER_ROW`; fixed-size components
/// (lookup tables) use no relations.
pub fn accumulate_circuit_relation_uses(
    component_log_sizes: [u32; N_COMPONENTS], ref relation_uses: RelationUsesDict,
) {
    let log_sizes = component_log_sizes.span();
    accumulate_relation_uses(
        ref relation_uses,
        components::eq::RELATION_USES_PER_ROW.span(),
        *log_sizes.at(EQ_COMPONENT_IDX),
    );
    accumulate_relation_uses(
        ref relation_uses,
        components::qm31_ops::RELATION_USES_PER_ROW.span(),
        *log_sizes.at(QM31_OPS_COMPONENT_IDX),
    );
    accumulate_relation_uses(
        ref relation_uses,
        components::triple_xor::RELATION_USES_PER_ROW.span(),
        *log_sizes.at(TRIPLE_XOR_COMPONENT_IDX),
    );
    accumulate_relation_uses(
        ref relation_uses,
        components::m_31_to_u_32::RELATION_USES_PER_ROW.span(),
        *log_sizes.at(M_31_TO_U_32_COMPONENT_IDX),
    );
    accumulate_relation_uses(
        ref relation_uses,
        components::blake_g_gate::RELATION_USES_PER_ROW.span(),
        *log_sizes.at(BLAKE_G_GATE_COMPONENT_IDX),
    );
}
