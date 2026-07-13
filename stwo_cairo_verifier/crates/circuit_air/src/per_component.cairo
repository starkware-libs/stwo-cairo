use crate::components::*;

pub const N_COMPONENTS: usize = 11;

/// Holds one `T` per circuit component, in `ComponentList` order — the order in which the prover
/// commits trace/interaction columns and the verifier evaluates constraints. Replaces indexing a
/// shared `[T; N_COMPONENTS]` array by a per-component index constant with named field access.
#[derive(Copy, Drop, Serde)]
pub struct PerComponent<T> {
    pub eq: T,
    pub qm31_ops: T,
    pub triple_xor: T,
    pub m_31_to_u_32: T,
    pub blake_g_gate: T,
    pub verify_bitwise_xor_8: T,
    pub verify_bitwise_xor_12: T,
    pub verify_bitwise_xor_4: T,
    pub verify_bitwise_xor_7: T,
    pub verify_bitwise_xor_9: T,
    pub range_check_16: T,
}

#[generate_trait]
pub impl PerComponentImpl<T, +Copy<T>, +Drop<T>> of PerComponentTrait<T> {
    /// Returns the per-component values as a fixed-size array in `ComponentList` order.
    fn to_fixed_array(self: @PerComponent<T>) -> [T; N_COMPONENTS] {
        [
            *self.eq, *self.qm31_ops, *self.triple_xor, *self.m_31_to_u_32, *self.blake_g_gate,
            *self.verify_bitwise_xor_8, *self.verify_bitwise_xor_12, *self.verify_bitwise_xor_4,
            *self.verify_bitwise_xor_7, *self.verify_bitwise_xor_9, *self.range_check_16,
        ]
    }
}

/// Number of trace columns per component.
pub const N_TRACE_COLUMNS_PER_COMPONENT: PerComponent<usize> = PerComponent {
    eq: eq::N_TRACE_COLUMNS,
    qm31_ops: qm31_ops::N_TRACE_COLUMNS,
    triple_xor: triple_xor::N_TRACE_COLUMNS,
    m_31_to_u_32: m_31_to_u_32::N_TRACE_COLUMNS,
    blake_g_gate: blake_g_gate::N_TRACE_COLUMNS,
    verify_bitwise_xor_8: verify_bitwise_xor_8::N_TRACE_COLUMNS,
    verify_bitwise_xor_12: verify_bitwise_xor_12::N_TRACE_COLUMNS,
    verify_bitwise_xor_4: verify_bitwise_xor_4::N_TRACE_COLUMNS,
    verify_bitwise_xor_7: verify_bitwise_xor_7::N_TRACE_COLUMNS,
    verify_bitwise_xor_9: verify_bitwise_xor_9::N_TRACE_COLUMNS,
    range_check_16: range_check_16::N_TRACE_COLUMNS,
};

/// Number of interaction columns per component.
pub const N_INTERACTION_COLUMNS_PER_COMPONENT: PerComponent<usize> = PerComponent {
    eq: eq::N_INTERACTION_COLUMNS,
    qm31_ops: qm31_ops::N_INTERACTION_COLUMNS,
    triple_xor: triple_xor::N_INTERACTION_COLUMNS,
    m_31_to_u_32: m_31_to_u_32::N_INTERACTION_COLUMNS,
    blake_g_gate: blake_g_gate::N_INTERACTION_COLUMNS,
    verify_bitwise_xor_8: verify_bitwise_xor_8::N_INTERACTION_COLUMNS,
    verify_bitwise_xor_12: verify_bitwise_xor_12::N_INTERACTION_COLUMNS,
    verify_bitwise_xor_4: verify_bitwise_xor_4::N_INTERACTION_COLUMNS,
    verify_bitwise_xor_7: verify_bitwise_xor_7::N_INTERACTION_COLUMNS,
    verify_bitwise_xor_9: verify_bitwise_xor_9::N_INTERACTION_COLUMNS,
    range_check_16: range_check_16::N_INTERACTION_COLUMNS,
};
