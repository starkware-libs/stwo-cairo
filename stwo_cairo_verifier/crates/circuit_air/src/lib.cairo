//! `stwo_circuit_air`: AIR-specific verifier-side logic written in Cairo for the stwo-circuits
//! circuit.
use stwo_verifier_core::Hash;

pub mod blake2s_consts;
pub mod circuit_air;
pub mod claims;
pub mod component_indices;
pub mod components;
pub mod prelude;
pub mod preprocessed_columns;
pub mod relations;

/// Verifier-side configuration parameters that describe the circuit topology. They are
/// NOT mixed into the channel. Mirrors the fields of 
/// `stwo-circuits/crates/circuit_air/src/verify.rs::CircuitConfig` that the
/// verifier consumes.
#[derive(Drop, Serde)]
pub struct CircuitVerifierConfig {
    /// Variable indices of the circuit's `Output` gates. One entry per public output value.
    pub output_addresses: Array<u32>,
    /// Number of Blake gates in the circuit (drives the Blake-IV public logup contribution).
    pub n_blake_gates: u32,
    /// Expected preprocessed trace root. Matches the prover-side commitment that is produced
    /// during preprocessing; the verifier re-checks the commitment read from the proof
    /// against this value.
    pub preprocessed_root: Hash,
    /// Per-column log sizes in the circuit's preprocessed trace, in the canonical
    /// (prover-side) column order. Used to override the per-component aggregate when
    /// computing the PCS tree log sizes.
    pub preprocessed_column_log_sizes: Array<u32>,
    /// `trace_log_size + log_blowup_factor` — the rust circuit prover packs this into the
    /// channel via `PcsConfig::mix_into`, but cairo's `PcsConfig` does not carry the
    /// field, so it is supplied here out-of-band. Analogous to the rust in-circuit
    /// verifier reading it from `ProofConfig.fri.log_trace_size`.
    pub lifting_log_size: u32,
}
