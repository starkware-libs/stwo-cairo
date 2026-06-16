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
#[cfg(not(feature: "poseidon252_verifier"))]
use stwo_verifier_core::vcs::blake2s_hasher::Blake2sHash;

/// Lifting log size of the recursive verification circuit's proof: the value carried in
/// the proof's `pcs_config.lifting_log_size` (trace_log_size 20 + log_blowup_factor 2).
pub const LIFTING_LOG_SIZE: u32 = 22;

/// Number of public output values of the recursive verification circuit.
///
/// The bootloader output is a single felt. The circuit verifier hashes that felt
/// (represented as 28 9-bit limbs) and outputs the resulting hash packed as 2 QM31s.
/// (The logup anchor `u` is appended internally by the verifier and is not part of the
/// public outputs.)
pub const N_OUTPUTS: u32 = 2;

/// Per-column log sizes of the recursive verification circuit's preprocessed trace,
/// in the canonical (prover-side, size-sorted) column order — the same order as the
/// index constants in `crate::preprocessed_columns`.
pub const PREPROCESSED_COLUMN_LOG_SIZES: [u32; 45] = [
    8, // bitwise_xor_4_0
    8, // bitwise_xor_4_1
    8, // bitwise_xor_4_2
    14, // eq_in0_address
    14, // eq_in1_address
    14, // bitwise_xor_7_0
    14, // bitwise_xor_7_1
    14, // bitwise_xor_7_2
    16, // triple_xor_input_addr_0
    16, // triple_xor_input_addr_1
    16, // triple_xor_input_addr_2
    16, // triple_xor_output_addr
    16, // triple_xor_multiplicity
    16, // seq_16
    16, // bitwise_xor_8_0
    16, // bitwise_xor_8_1
    16, // bitwise_xor_8_2
    17, // m31_to_u32_input_addr
    17, // m31_to_u32_output_addr
    17, // m31_to_u32_multiplicity
    18, // bitwise_xor_9_0
    18, // bitwise_xor_9_1
    18, // bitwise_xor_9_2
    20, // qm31_ops_add_flag
    20, // qm31_ops_sub_flag
    20, // qm31_ops_mul_flag
    20, // qm31_ops_pointwise_mul_flag
    20, // qm31_ops_in0_address
    20, // qm31_ops_in1_address
    20, // qm31_ops_out_address
    20, // qm31_ops_mults
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
    20 // bitwise_xor_10_2
];

/// Expected preprocessed-trace root (Merkle commitment, tree 0 of the proof's
/// commitments) for the recursive verification circuit at `LIFTING_LOG_SIZE`,
/// as 8 little-endian u32 words of the Blake2s digest.
#[cfg(not(feature: "poseidon252_verifier"))]
pub fn preprocessed_root() -> Hash {
    Blake2sHash {
        hash: BoxImpl::new(
            [
                247397557, 1894101695, 712239034, 2297070075, 1735326805, 878474250, 432574511,
                3468968259,
            ],
        ),
    }
}

#[cfg(feature: "poseidon252_verifier")]
pub fn preprocessed_root() -> Hash {
    panic!("the privacy recursive circuit verifier only supports the blake2s hasher")
}
