// This file was created by the AIR team.

use crate::components::*;

// Component index constants.
pub const QM31_OPS_COMPONENT_IDX: usize = 0;
pub const VERIFY_BITWISE_XOR_8_COMPONENT_IDX: usize = 1;
pub const VERIFY_BITWISE_XOR_12_COMPONENT_IDX: usize = 2;
pub const VERIFY_BITWISE_XOR_4_COMPONENT_IDX: usize = 3;
pub const VERIFY_BITWISE_XOR_9_COMPONENT_IDX: usize = 4;
pub const VERIFY_BITWISE_XOR_7_COMPONENT_IDX: usize = 5;
pub const BLAKE_G_GATE_COMPONENT_IDX: usize = 6;
pub const RANGE_CHECK_16_COMPONENT_IDX: usize = 7;
pub const M_31_TO_U_32_COMPONENT_IDX: usize = 8;
pub const TRIPLE_XOR_COMPONENT_IDX: usize = 9;

pub const N_COMPONENTS: usize = 10;

/// Number of trace columns per component, indexed by COMPONENT_IDX.
pub const N_TRACE_COLUMNS_PER_COMPONENT_IDX: [usize; N_COMPONENTS] = [
    qm31_ops::N_TRACE_COLUMNS, verify_bitwise_xor_8::N_TRACE_COLUMNS,
    verify_bitwise_xor_12::N_TRACE_COLUMNS, verify_bitwise_xor_4::N_TRACE_COLUMNS,
    verify_bitwise_xor_9::N_TRACE_COLUMNS, verify_bitwise_xor_7::N_TRACE_COLUMNS,
    blake_g_gate::N_TRACE_COLUMNS, range_check_16::N_TRACE_COLUMNS, m_31_to_u_32::N_TRACE_COLUMNS,
    triple_xor::N_TRACE_COLUMNS,
];

/// Number of interaction columns per component, indexed by COMPONENT_IDX.
pub const N_INTERACTION_COLUMNS_PER_COMPONENT_IDX: [usize; N_COMPONENTS] = [
    qm31_ops::N_INTERACTION_COLUMNS, verify_bitwise_xor_8::N_INTERACTION_COLUMNS,
    verify_bitwise_xor_12::N_INTERACTION_COLUMNS, verify_bitwise_xor_4::N_INTERACTION_COLUMNS,
    verify_bitwise_xor_9::N_INTERACTION_COLUMNS, verify_bitwise_xor_7::N_INTERACTION_COLUMNS,
    blake_g_gate::N_INTERACTION_COLUMNS, range_check_16::N_INTERACTION_COLUMNS,
    m_31_to_u_32::N_INTERACTION_COLUMNS, triple_xor::N_INTERACTION_COLUMNS,
];
