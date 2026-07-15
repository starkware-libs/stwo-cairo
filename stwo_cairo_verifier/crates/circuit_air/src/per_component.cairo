use crate::components::*;

pub const N_COMPONENTS: usize = 11;

/// Holds one `T` per circuit component, in the multiverifier's committed component order — the
/// order in which the prover commits trace/interaction columns and the verifier evaluates
/// constraints. Components are sorted by ascending trace log size, ties broken by the order passed
/// to `define_component_list!` in the `circuit-verifier` crate (stwo-circuits repo).
///
/// Note: the field order here (log-size order) differs from the field order of
/// `PerComponent` in the `circuit-verifier` crate, which follows `ComponentList` declaration order.
#[derive(Copy, Drop, Serde)]
pub struct PerComponent<T> {
    pub verify_bitwise_xor_4: T,
    pub verify_bitwise_xor_7: T,
    pub verify_bitwise_xor_8: T,
    pub range_check_16: T,
    pub eq: T,
    pub triple_xor: T,
    pub m_31_to_u_32: T,
    pub verify_bitwise_xor_9: T,
    pub blake_g_gate: T,
    pub verify_bitwise_xor_12: T,
    pub qm31_ops: T,
}

#[generate_trait]
pub impl PerComponentImpl<T, +Copy<T>, +Drop<T>> of PerComponentTrait<T> {
    /// Returns the per-component values as a fixed-size array in committed component order.
    fn to_fixed_array(self: @PerComponent<T>) -> [T; N_COMPONENTS] {
        [
            *self.verify_bitwise_xor_4, *self.verify_bitwise_xor_7, *self.verify_bitwise_xor_8,
            *self.range_check_16, *self.eq, *self.triple_xor, *self.m_31_to_u_32,
            *self.verify_bitwise_xor_9, *self.blake_g_gate, *self.verify_bitwise_xor_12,
            *self.qm31_ops,
        ]
    }
}

/// Each component's log size.
pub const COMPONENT_LOG_SIZES: PerComponent<u32> = PerComponent {
    verify_bitwise_xor_4: 8,
    verify_bitwise_xor_7: 14,
    verify_bitwise_xor_8: 16,
    range_check_16: 16,
    eq: 17,
    triple_xor: 17,
    m_31_to_u_32: 18,
    verify_bitwise_xor_9: 18,
    blake_g_gate: 20,
    verify_bitwise_xor_12: 20,
    qm31_ops: 21,
};

/// The same log sizes as `COMPONENT_LOG_SIZES`, but in `ComponentList` declaration order — the
/// canonical order used by `PerComponent` in the `circuit-verifier` crate (stwo-circuits repo),
/// as opposed to the committed size-sorted order of `COMPONENT_LOG_SIZES`.
/// Used to compute the circuit hash.
pub const CANONICAL_ORDER_COMPONENT_LOG_SIZES: [u32; N_COMPONENTS] = [
    17, // eq
    21, // qm31_ops
    17, // triple_xor
    18, // m_31_to_u_32
    20, // blake_g_gate
    16, // verify_bitwise_xor_8
    20, // verify_bitwise_xor_12
    8, // verify_bitwise_xor_4
    14, // verify_bitwise_xor_7
    18, // verify_bitwise_xor_9
    16 // range_check_16
];

/// Number of trace columns per component.
pub const N_TRACE_COLUMNS_PER_COMPONENT: PerComponent<usize> = PerComponent {
    verify_bitwise_xor_4: verify_bitwise_xor_4::N_TRACE_COLUMNS,
    verify_bitwise_xor_7: verify_bitwise_xor_7::N_TRACE_COLUMNS,
    verify_bitwise_xor_8: verify_bitwise_xor_8::N_TRACE_COLUMNS,
    range_check_16: range_check_16::N_TRACE_COLUMNS,
    eq: eq::N_TRACE_COLUMNS,
    triple_xor: triple_xor::N_TRACE_COLUMNS,
    m_31_to_u_32: m_31_to_u_32::N_TRACE_COLUMNS,
    verify_bitwise_xor_9: verify_bitwise_xor_9::N_TRACE_COLUMNS,
    blake_g_gate: blake_g_gate::N_TRACE_COLUMNS,
    verify_bitwise_xor_12: verify_bitwise_xor_12::N_TRACE_COLUMNS,
    qm31_ops: qm31_ops::N_TRACE_COLUMNS,
};

/// Number of interaction columns per component.
pub const N_INTERACTION_COLUMNS_PER_COMPONENT: PerComponent<usize> = PerComponent {
    verify_bitwise_xor_4: verify_bitwise_xor_4::N_INTERACTION_COLUMNS,
    verify_bitwise_xor_7: verify_bitwise_xor_7::N_INTERACTION_COLUMNS,
    verify_bitwise_xor_8: verify_bitwise_xor_8::N_INTERACTION_COLUMNS,
    range_check_16: range_check_16::N_INTERACTION_COLUMNS,
    eq: eq::N_INTERACTION_COLUMNS,
    triple_xor: triple_xor::N_INTERACTION_COLUMNS,
    m_31_to_u_32: m_31_to_u_32::N_INTERACTION_COLUMNS,
    verify_bitwise_xor_9: verify_bitwise_xor_9::N_INTERACTION_COLUMNS,
    blake_g_gate: blake_g_gate::N_INTERACTION_COLUMNS,
    verify_bitwise_xor_12: verify_bitwise_xor_12::N_INTERACTION_COLUMNS,
    qm31_ops: qm31_ops::N_INTERACTION_COLUMNS,
};

#[cfg(test)]
mod tests {
    use stwo_verifier_utils::zip_eq::zip_eq;
    use crate::components;
    use crate::preprocessed_columns::{
        BLAKE_G_GATE_INPUT_ADDR_A_IDX, EQ_IN0_ADDRESS_IDX, M_31_TO_U_32_INPUT_ADDR_IDX,
        OP_0_ADDR_IDX, TRIPLE_XOR_INPUT_ADDR_0_IDX,
    };
    use crate::privacy_consts::PREPROCESSED_COLUMN_LOG_SIZES;
    use super::{
        CANONICAL_ORDER_COMPONENT_LOG_SIZES, COMPONENT_LOG_SIZES, N_COMPONENTS, PerComponentTrait,
    };

    /// Derives each component's log size from the preprocessed column log sizes, in `ComponentList`
    /// declaration order: variable-size components read the log size of one of their preprocessed
    /// columns (all columns of a component share its log size), and fixed-size components return
    /// their `LOG_SIZE` constant.
    fn derive_canonical_order_component_log_sizes(
        preprocessed_column_log_sizes: Span<u32>,
    ) -> [u32; N_COMPONENTS] {
        [
            *preprocessed_column_log_sizes.at(EQ_IN0_ADDRESS_IDX), // eq
            *preprocessed_column_log_sizes.at(OP_0_ADDR_IDX), // qm31_ops
            *preprocessed_column_log_sizes.at(TRIPLE_XOR_INPUT_ADDR_0_IDX), // triple_xor
            *preprocessed_column_log_sizes.at(M_31_TO_U_32_INPUT_ADDR_IDX), // m_31_to_u_32
            *preprocessed_column_log_sizes.at(BLAKE_G_GATE_INPUT_ADDR_A_IDX), // blake_g_gate
            components::verify_bitwise_xor_8::LOG_SIZE, // verify_bitwise_xor_8
            components::verify_bitwise_xor_12::LOG_SIZE, // verify_bitwise_xor_12
            components::verify_bitwise_xor_4::LOG_SIZE, // verify_bitwise_xor_4
            components::verify_bitwise_xor_7::LOG_SIZE, // verify_bitwise_xor_7
            components::verify_bitwise_xor_9::LOG_SIZE, // verify_bitwise_xor_9
            components::range_check_16::LOG_SIZE // range_check_16
        ]
    }

    /// The hardcoded `CANONICAL_ORDER_COMPONENT_LOG_SIZES` must equal the values derived from the
    /// preprocessed column log sizes.
    #[test]
    fn hardcoded_canonical_order_log_sizes_match_derived() {
        let derived = derive_canonical_order_component_log_sizes(
            PREPROCESSED_COLUMN_LOG_SIZES.span(),
        );
        for (expected, actual) in zip_eq(
            CANONICAL_ORDER_COMPONENT_LOG_SIZES.span(), derived.span(),
        ) {
            assert!(*expected == *actual);
        }
    }

    /// Returns `values` sorted in ascending order (insertion sort; the arrays are tiny).
    fn sorted_ascending(values: Span<u32>) -> Array<u32> {
        let mut sorted: Array<u32> = array![];
        for value in values {
            let value = *value;
            let mut next: Array<u32> = array![];
            let mut inserted = false;
            for s in sorted.span() {
                if !inserted && value <= *s {
                    next.append(value);
                    inserted = true;
                }
                next.append(*s);
            }
            if !inserted {
                next.append(value);
            }
            sorted = next;
        }
        sorted
    }

    /// Sorting the canonical (declaration-order) log sizes must reproduce the committed
    /// (size-sorted) `COMPONENT_LOG_SIZES`.
    #[test]
    fn sorted_canonical_order_matches_committed_order() {
        let sorted = sorted_ascending(CANONICAL_ORDER_COMPONENT_LOG_SIZES.span());
        for (expected, actual) in zip_eq(
            COMPONENT_LOG_SIZES.to_fixed_array().span(), sorted.span(),
        ) {
            assert!(*expected == *actual);
        }
    }
}
