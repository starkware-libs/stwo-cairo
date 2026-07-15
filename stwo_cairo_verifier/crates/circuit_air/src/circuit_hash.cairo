//! Circuit hash: `blake2s(log_blowup_factor || component_log_sizes || preprocessed_root)`.
//!
//! Cairo port of `compute_circuit_hash_host` in the `circuit-verifier` crate (stwo-circuits repo).
//! The component log sizes are taken in `ComponentList` (canonical) order, so this must match the
//! packing there byte-for-byte for the Fiat-Shamir transcript to agree.
use stwo_verifier_utils::blake2s::hash_u32s;
use crate::per_component::CANONICAL_ORDER_COMPONENT_LOG_SIZES;

/// Number of 32-bit words in a Blake2s-256 digest.
pub const BLAKE2S_DIGEST_N_WORDS: usize = 8;

/// Packs `log_blowup_factor` (byte 0) followed by each component's preprocessed log size (one byte
/// each, in canonical order) into little-endian u32 words. The total byte count `1 + N_COMPONENTS`
/// must be a multiple of 4.
fn config_words(log_blowup_factor: u32) -> Array<u32> {
    let mut config_bytes = array![log_blowup_factor];
    for log_size in CANONICAL_ORDER_COMPONENT_LOG_SIZES.span() {
        config_bytes.append(*log_size);
    }

    let mut words = array![];
    let mut i = 0;
    while i != config_bytes.len() {
        let b0 = *config_bytes.at(i);
        let b1 = *config_bytes.at(i + 1);
        let b2 = *config_bytes.at(i + 2);
        let b3 = *config_bytes.at(i + 3);
        words.append(b0 + b1 * 0x100 + b2 * 0x10000 + b3 * 0x1000000);
        i += 4;
    }
    words
}

/// Computes the circuit hash: `blake2s(log_blowup_factor || component_log_sizes ||
/// preprocessed_root)`, packing each value as little-endian bytes.
pub fn compute_circuit_hash(
    preprocessed_root: [u32; BLAKE2S_DIGEST_N_WORDS], log_blowup_factor: u32,
) -> Box<[u32; BLAKE2S_DIGEST_N_WORDS]> {
    let mut words = config_words(log_blowup_factor);
    words.append_span(preprocessed_root.span());
    hash_u32s(words.span())
}

#[cfg(test)]
mod tests {
    use super::{BLAKE2S_DIGEST_N_WORDS, compute_circuit_hash};

    /// Known-answer test: `blake2s` over `log_blowup_factor = 3`, the canonical component log
    /// sizes, and `preprocessed_root = [0, 1, .., 7]`, cross-checked against an independent
    /// `blake2s` reference. This pins the byte packing against `compute_circuit_hash_host` in the
    /// stwo-circuits repo.
    #[test]
    fn compute_circuit_hash_matches_reference() {
        let preprocessed_root = [0, 1, 2, 3, 4, 5, 6, 7];
        let expected: [u32; BLAKE2S_DIGEST_N_WORDS] = [
            0xa8810641, 0x52391285, 0x90b37fd2, 0x905b887a, 0x7db7dc81, 0xa7c3a731, 0xd0d46b34,
            0x8fa6a471,
        ];
        assert!(compute_circuit_hash(preprocessed_root, 3).unbox() == expected);
    }
}
