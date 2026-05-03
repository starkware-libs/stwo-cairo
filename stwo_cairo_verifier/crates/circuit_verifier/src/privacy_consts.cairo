//! Hardcoded structural constants for the privacy/recursion circuit verifier.
//!
//! These must match the recursive-verification circuit topology produced by the rust
//! prover (`stwo-circuits/crates/cairo_verifier/src/privacy.rs` plus the preprocessed
//! trace built for that circuit). They are baked in at build time so the executable's
//! `main` takes only a proof — mirroring how `cairo_air::verify_cairo` hardcodes its
//! preprocessed root.
//!
//! TODO(circuit-verifier): every constant in this file is currently a placeholder.
//! Replace each one with the real value sourced from a `prove_recursive_verification`
//! run (see `proving-utils/crates/circuit_verifier_e2e/src/recurse.rs::RecursiveProveOutput`,
//! whose fields list exactly the constants this module needs to emit). Until then, this
//! binary compiles but cannot verify any real proof.

use core::box::BoxImpl;
use stwo_circuit_air::CircuitVerifierConfig;
use stwo_verifier_core::Hash;
#[cfg(not(feature: "poseidon252_verifier"))]
use stwo_verifier_core::vcs::blake2s_hasher::Blake2sHash;

/// Lifting log size for the privacy circuit. Mirrors
/// `stwo-circuits/crates/cairo_verifier/src/privacy.rs:46`
/// (`let lifting_log_size = 20 + log_blowup_factor;`) with the privacy circuit's
/// `log_blowup_factor = 1`.
pub const LIFTING_LOG_SIZE: u32 = 21;

/// Number of public output values of the recursive verification circuit. Mirrors
/// `stwo-circuits/crates/cairo_verifier/src/privacy.rs:68` (`n_outputs: 1`).
pub const N_OUTPUTS: u32 = 1;

/// Per-column log sizes of the recursive verification circuit's preprocessed trace,
/// in the canonical (prover-side) column order.
/// TODO(circuit-verifier): replace with real values.
pub fn preprocessed_column_log_sizes() -> Array<u32> {
    array![]
}

/// Expected preprocessed-trace root (Merkle commitment) for the recursive verification
/// circuit at `LIFTING_LOG_SIZE`. TODO(circuit-verifier): replace with the real
/// precomputed root.
#[cfg(not(feature: "poseidon252_verifier"))]
pub fn preprocessed_root() -> Hash {
    Blake2sHash { hash: BoxImpl::new([0_u32; 8]) }
}

#[cfg(feature: "poseidon252_verifier")]
pub fn preprocessed_root() -> Hash {
    0
}

pub fn privacy_config() -> CircuitVerifierConfig {
    CircuitVerifierConfig {
        n_outputs: N_OUTPUTS,
        preprocessed_root: preprocessed_root(),
        preprocessed_column_log_sizes: preprocessed_column_log_sizes(),
        lifting_log_size: LIFTING_LOG_SIZE,
    }
}
