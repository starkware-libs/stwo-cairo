//! `stwo_circuit_air`: AIR-specific verifier-side logic written in Cairo for the stwo-circuits
//! circuit.
use stwo_verifier_core::Hash;

pub mod circuit_air;
pub mod claims;
pub mod component_indices;
pub mod components;
pub mod prelude;
pub mod preprocessed_columns;
pub mod relations;

/// Configuration parameters that describe the circuit topology. They are
/// NOT mixed into the channel.
#[derive(Drop, Serde)]
pub struct CircuitVerifierConfig {
    /// Number of public output values the circuit produces (its `Output` gates).
    pub n_outputs: u32,
    /// Expected preprocessed trace root.
    pub preprocessed_root: Hash,
    /// Per-column log sizes in the circuit's preprocessed trace, in `crate::preprocessed_columns`
    /// order.
    pub preprocessed_column_log_sizes: Array<u32>,
    /// lifting log size = trace_log_size + log_blowup_factor.
    pub lifting_log_size: u32,
}
