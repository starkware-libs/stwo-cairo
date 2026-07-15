//! Circuit hash: `blake2s(log_blowup_factor || component_log_sizes || preprocessed_root)`.
//!
//! Cairo port of `compute_circuit_hash_host` in the `circuit-verifier` crate (stwo-circuits repo).
//! The component log sizes are taken in `ComponentList` (canonical) order, so this must match the
//! packing there byte-for-byte for the Fiat-Shamir transcript to agree.
#[cfg(not(feature: "poseidon252_verifier"))]
use stwo_verifier_core::Hash;
#[cfg(not(feature: "poseidon252_verifier"))]
use stwo_verifier_utils::blake2s::hash_u32s;
#[cfg(not(feature: "poseidon252_verifier"))]
use crate::multiverifier_consts::COMPONENT_LOG_SIZES;

/// Number of 32-bit words in a Blake2s-256 digest.
pub const BLAKE2S_DIGEST_N_WORDS: usize = 8;

/// Packs `log_blowup_factor` (byte 0) followed by each component's preprocessed log size (one byte
/// each, in canonical order) into little-endian u32 words. The total byte count `1 + N_COMPONENTS`
/// must be a multiple of 4.
#[cfg(not(feature: "poseidon252_verifier"))]
fn config_words(log_blowup_factor: u32) -> Array<u32> {
    let mut config_bytes = [
        log_blowup_factor, COMPONENT_LOG_SIZES.eq, COMPONENT_LOG_SIZES.qm31_ops,
        COMPONENT_LOG_SIZES.triple_xor, COMPONENT_LOG_SIZES.m_31_to_u_32,
        COMPONENT_LOG_SIZES.blake_g_gate, COMPONENT_LOG_SIZES.verify_bitwise_xor_8,
        COMPONENT_LOG_SIZES.verify_bitwise_xor_12, COMPONENT_LOG_SIZES.verify_bitwise_xor_4,
        COMPONENT_LOG_SIZES.verify_bitwise_xor_7, COMPONENT_LOG_SIZES.verify_bitwise_xor_9,
        COMPONENT_LOG_SIZES.range_check_16,
    ]
        .span();

    let mut words = array![];
    while let Some(boxed) = config_bytes.multi_pop_front::<4>() {
        let [b0, b1, b2, b3] = boxed.unbox();
        words.append(b0 + b1 * 0x100 + b2 * 0x10000 + b3 * 0x1000000);
    }
    assert!(config_bytes.is_empty());
    words
}

/// Computes the circuit hash: `blake2s(log_blowup_factor || component_log_sizes ||
/// preprocessed_root)`, packing each value as little-endian bytes. The log blowup factor and
/// component log sizes are the circuit's hardcoded constants; only the preprocessed root varies.
#[cfg(not(feature: "poseidon252_verifier"))]
pub fn compute_circuit_hash(log_blowup_factor: u32, preprocessed_root: Hash) -> Hash {
    let mut words = config_words(log_blowup_factor);
    words.append_span(preprocessed_root.hash.unbox().span());
    Hash { hash: hash_u32s(words.span()) }
}

#[cfg(test)]
#[cfg(not(feature: "poseidon252_verifier"))]
mod tests {
    use core::box::BoxImpl;
    use stwo_verifier_core::vcs::blake2s_hasher::Blake2sHash;
    use super::{BLAKE2S_DIGEST_N_WORDS, compute_circuit_hash};

    /// Known-answer test: `blake2s` over the canonical `log_blowup_factor` (3) and component log
    /// sizes, and `preprocessed_root = [0, 1, .., 7]`, cross-checked against an independent
    /// `blake2s` reference. This pins the byte packing against `compute_circuit_hash_host` in the
    /// stwo-circuits repo.
    #[test]
    fn compute_circuit_hash_matches_reference() {
        let log_blowup_factor = 3;
        let preprocessed_root = Blake2sHash { hash: BoxImpl::new([0, 1, 2, 3, 4, 5, 6, 7]) };
        let expected: [u32; BLAKE2S_DIGEST_N_WORDS] = [
            0xa8810641, 0x52391285, 0x90b37fd2, 0x905b887a, 0x7db7dc81, 0xa7c3a731, 0xd0d46b34,
            0x8fa6a471,
        ];
        assert!(
            compute_circuit_hash(log_blowup_factor, preprocessed_root).hash.unbox() == expected,
        );
    }
}
