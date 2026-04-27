use stwo_verifier_core::fields::m31::M31;

// Mirrors relation IDs defined in stwo-circuits/crates/circuit_air/src/relations.rs.
// TODO(circuit-verifier): keep in sync with the Rust constants.
pub const GATE_RELATION_ID: M31 = M31 { inner: 378353459 };
pub const BLAKE_STATE_RELATION_ID: M31 = M31 { inner: 1061955672 };
pub const BLAKE_G_RELATION_ID: M31 = M31 { inner: 1139985212 };
pub const BLAKE_ROUND_RELATION_ID: M31 = M31 { inner: 40528774 };
pub const BLAKE_ROUND_SIGMA_RELATION_ID: M31 = M31 { inner: 1805967942 };
pub const BLAKE_MESSAGE_RELATION_ID: M31 = M31 { inner: 1492981981 };
pub const TRIPLE_XOR_32_RELATION_ID: M31 = M31 { inner: 990559919 };
pub const RANGE_CHECK_15_RELATION_ID: M31 = M31 { inner: 1058718565 };
pub const RANGE_CHECK_16_RELATION_ID: M31 = M31 { inner: 1008385708 };
pub const VERIFY_BITWISE_XOR_4_RELATION_ID: M31 = M31 { inner: 45448144 };
pub const VERIFY_BITWISE_XOR_7_RELATION_ID: M31 = M31 { inner: 62225763 };
pub const VERIFY_BITWISE_XOR_8_RELATION_ID: M31 = M31 { inner: 112558620 };
pub const VERIFY_BITWISE_XOR_8_B_RELATION_ID: M31 = M31 { inner: 521092554 };
pub const VERIFY_BITWISE_XOR_9_RELATION_ID: M31 = M31 { inner: 95781001 };
pub const VERIFY_BITWISE_XOR_12_RELATION_ID: M31 = M31 { inner: 648362599 };
