//! Hardcoded structural constants for the privacy/recursion circuit verifier.
//!
//! These must match the recursive-verification circuit topology produced by the rust
//! prover (`stwo-circuits/crates/circuit_verifier/src/verify.rs::build_verification_circuit`
//! on the privacy circuit, plus the preprocessed trace built for that circuit). They are
//! baked in at build time so the executable's `main` takes only a proof — mirroring how
//! `cairo_air::verify_cairo` hardcodes its preprocessed root.
//!
// TODO(Gali): Change to MultiVerifier consts.

use core::box::BoxImpl;
use stwo_verifier_core::Hash;
use stwo_verifier_core::fri::FriConfig;
use stwo_verifier_core::pcs::PcsConfig;
#[cfg(not(feature: "poseidon252_verifier"))]
use stwo_verifier_core::vcs::blake2s_hasher::Blake2sHash;

/// Lifting log size of the multiverifier circuit's proof: the value carried in
/// the proof's `pcs_config.lifting_log_size` (trace_log_size 21 + log_blowup_factor 3).
pub const LIFTING_LOG_SIZE: u32 = 24;

/// Expected PCS config of the recursive verification circuit's proof.
///
/// Hardcoded so the verifier accepts only proofs produced with the circuit's canonical
/// configuration. This pins every FRI security parameter (a weaker config — fewer queries,
/// smaller blowup, or less proof-of-work — is rejected, independently of stwo's
/// `security_bits >= SECURITY_BITS` floor) and ties `log_blowup_factor` to the blowup the
/// hardcoded `preprocessed_root()` was committed at. Must match the rust prover's
/// multiverifier `PCS_CONFIG` (`get_pcs_config(21, 3)`), with the prover-clamped
/// `lifting_log_size`.
/// Note `pow_bits + log_blowup_factor * n_queries = 27 + 3 * 23 = 96 = SECURITY_BITS`.
pub fn circuit_pcs_config() -> PcsConfig {
    PcsConfig {
        pow_bits: 27,
        fri_config: FriConfig {
            log_blowup_factor: 3, log_last_layer_degree_bound: 0, n_queries: 23, fold_step: 4,
        },
        lifting_log_size: Option::Some(LIFTING_LOG_SIZE),
    }
}

/// Number of public output values of the multiverifier circuit.
///
/// The multiverifier outputs the full unreduced Blake2s digest of its two verified inputs as
/// `N_RESERVED` = 8 QM31 words. (The logup anchor `u` is appended internally by the verifier and
/// is not part of the public outputs.)
pub const N_OUTPUTS: u32 = 8;

/// Per-column log sizes of the multiverifier circuit's preprocessed trace,
/// in the canonical (prover-side, size-sorted) column order — the same order as the
/// index constants in `crate::preprocessed_columns`.
pub const PREPROCESSED_COLUMN_LOG_SIZES: [u32; 45] = [
    8, // bitwise_xor_4_0
    8, // bitwise_xor_4_1
    8, // bitwise_xor_4_2
    14, // bitwise_xor_7_0
    14, // bitwise_xor_7_1
    14, // bitwise_xor_7_2
    16, // seq_16
    16, // bitwise_xor_8_0
    16, // bitwise_xor_8_1
    16, // bitwise_xor_8_2
    17, // eq_in0_address
    17, // eq_in1_address
    17, // triple_xor_input_addr_0
    17, // triple_xor_input_addr_1
    17, // triple_xor_input_addr_2
    17, // triple_xor_output_addr
    17, // triple_xor_multiplicity
    18, // m31_to_u32_input_addr
    18, // m31_to_u32_output_addr
    18, // m31_to_u32_multiplicity
    18, // bitwise_xor_9_0
    18, // bitwise_xor_9_1
    18, // bitwise_xor_9_2
    20, // blake_g_gate_input_addr_a
    20, // blake_g_gate_input_addr_b
    20, // blake_g_gate_input_addr_c
    20, // blake_g_gate_input_addr_d
    20, // blake_g_gate_input_addr_f0
    20, // blake_g_gate_input_addr_f1
    20, // blake_g_gate_output_addr_a
    20, // blake_g_gate_output_addr_b
    20, // blake_g_gate_output_addr_c
    20, // blake_g_gate_output_addr_d
    20, // blake_g_gate_multiplicity
    20, // bitwise_xor_10_0
    20, // bitwise_xor_10_1
    20, // bitwise_xor_10_2
    21, // qm31_ops_add_flag
    21, // qm31_ops_sub_flag
    21, // qm31_ops_mul_flag
    21, // qm31_ops_pointwise_mul_flag
    21, // qm31_ops_in0_address
    21, // qm31_ops_in1_address
    21, // qm31_ops_out_address
    21 // qm31_ops_mults
];

/// Expected preprocessed-trace root (Merkle commitment, tree 0 of the proof's
/// commitments) for the multiverifier circuit at `LIFTING_LOG_SIZE`,
/// as 8 little-endian u32 words of the Blake2s digest. Matches the prover-side
/// `MULTIVERIFIER_PREPROCESSED_ROOT` in
/// `stwo-circuits/crates/circuit_multiverifier/src/verify_test.rs`.
#[cfg(not(feature: "poseidon252_verifier"))]
pub fn preprocessed_root() -> Hash {
    Blake2sHash {
        hash: BoxImpl::new(
            [
                4268871180, 1648605015, 1518856044, 936813334, 8391980, 3571729286, 3315525509,
                1034558230,
            ],
        ),
    }
}

#[cfg(feature: "poseidon252_verifier")]
pub fn preprocessed_root() -> Hash {
    panic!("the privacy recursive circuit verifier only supports the blake2s hasher")
}
