use bounded_int::impls::*;
use bounded_int::{NZ_U32_SHIFT, div_rem, upcast};
use core::blake::{blake2s_compress, blake2s_finalize};
use core::box::BoxImpl;
use stwo_verifier_utils::{
    BLAKE2S_256_INITIAL_STATE, MemorySection, hash_memory_section_ids, hash_memory_section_values,
};
use crate::SecureField;
use crate::fields::m31::{M31, M31Trait};
use crate::fields::qm31::QM31Trait;
use crate::utils::{ArrayImpl, pow2_u64};
use crate::vcs::blake2s_hasher::Blake2sHash;
use super::ChannelTrait;

#[cfg(test)]
mod test;

/// Number of `M31` per hash.
pub const FELTS_PER_HASH: usize = 8;

const BYTES_PER_HASH: usize = 32;

/// A channel with Blake2s hash as the non-interactive random oracle.
/// By convention, at the end of every `mix_*` function we reset the number of draws `n_draws`
/// to zero. Every draw of 8 words increments `n_draws` by one.
#[derive(Drop)]
pub struct Blake2sChannel {
    digest: Blake2sHash,
    /// Number of consecutive draws since the last value was mixed into the channel.
    n_draws: usize,
}

pub fn new_channel(digest: Blake2sHash) -> Blake2sChannel {
    Blake2sChannel { digest, n_draws: Default::default() }
}

impl Blake2sChannelDefault of Default<Blake2sChannel> {
    fn default() -> Blake2sChannel {
        Blake2sChannel {
            digest: Blake2sHash { hash: BoxImpl::new([0_u32; 8]) }, n_draws: Default::default(),
        }
    }
}

/// Every mix should call `update_digest` as final step, and every draw should
/// increment the `n_draws` counter. In the current implementation, every draw method
/// invokes `draw_random_words` internally, which increments the `n_draws` by one.
// TODO(Gali): Consider simplifying the implementation of the mixing methods by hashing the current
// digest at the end.
pub impl Blake2sChannelImpl of ChannelTrait {
    fn mix_commitment(ref self: Blake2sChannel, commitment: Blake2sHash) {
        let [d0, d1, d2, d3, d4, d5, d6, d7] = self.digest.hash.unbox();
        let [r0, r1, r2, r3, r4, r5, r6, r7] = commitment.hash.unbox();
        let msg = [d0, d1, d2, d3, d4, d5, d6, d7, r0, r1, r2, r3, r4, r5, r6, r7];
        let res = blake2s_finalize(BoxImpl::new(BLAKE2S_256_INITIAL_STATE), 64, BoxImpl::new(msg));
        update_digest(ref self, Blake2sHash { hash: res });
    }

    fn mix_felts(ref self: Blake2sChannel, felts: Span<SecureField>) {
        let [d0, d1, d2, d3, d4, d5, d6, d7] = self.digest.hash.unbox();
        let mut state = BoxImpl::new(BLAKE2S_256_INITIAL_STATE);
        let mut buffer = array![d0, d1, d2, d3, d4, d5, d6, d7];
        let mut byte_count = 32;

        for felt in felts {
            // Compress whenever the buffer reaches capacity.
            let msg_opt: Option<@Box<[u32; 16]>> = buffer.span().try_into();
            if let Some(msg) = msg_opt {
                state = blake2s_compress(state, byte_count, *msg);
                buffer = array![];
            }

            let [r0, r1, r2, r3] = felt.to_fixed_array();
            buffer.append(r0.into());
            buffer.append(r1.into());
            buffer.append(r2.into());
            buffer.append(r3.into());
            byte_count += 16;
        }

        for _ in buffer.len()..16 {
            buffer.append(0);
        }

        let res = blake2s_finalize(state, byte_count, *buffer.span().try_into().unwrap());
        update_digest(ref self, Blake2sHash { hash: res });
    }

    fn mix_u64(ref self: Blake2sChannel, nonce: u64) {
        let (q, r) = div_rem(nonce, NZ_U32_SHIFT);
        let nonce_hi = upcast(q);
        let nonce_lo = upcast(r);

        let [d0, d1, d2, d3, d4, d5, d6, d7] = self.digest.hash.unbox();
        let mut state = BoxImpl::new(BLAKE2S_256_INITIAL_STATE);
        let mut buffer = [d0, d1, d2, d3, d4, d5, d6, d7, nonce_lo, nonce_hi, 0, 0, 0, 0, 0, 0];
        let mut byte_count = 40;

        let res = blake2s_finalize(state, byte_count, BoxImpl::new(buffer));
        update_digest(ref self, Blake2sHash { hash: res });
    }

    fn mix_memory_section(ref self: Blake2sChannel, section: MemorySection) {
        let digest = self.digest.hash.unbox();

        // Mix ids hash
        let ids_hash = hash_memory_section_ids(section, digest);

        // Mix values hash
        let values_hash = hash_memory_section_values(section, ids_hash.unbox());
        update_digest(ref self, Blake2sHash { hash: values_hash });
    }

    fn draw_secure_felt(ref self: Blake2sChannel) -> SecureField {
        let [r0, r1, r2, r3, _, _, _, _] = draw_base_felts(ref self).unbox();
        QM31Trait::from_fixed_array([r0, r1, r2, r3])
    }

    fn draw_secure_felts(ref self: Blake2sChannel, mut n_felts: usize) -> Array<SecureField> {
        let mut res = array![];

        while n_felts != 0 {
            let [r0, r1, r2, r3, r4, r5, r6, r7] = draw_base_felts(ref self).unbox();
            res.append(QM31Trait::from_fixed_array([r0, r1, r2, r3]));
            if n_felts == 1 {
                break;
            }
            res.append(QM31Trait::from_fixed_array([r4, r5, r6, r7]));
            n_felts -= 2;
        }

        res
    }

    /// Draws 8 random u32s.
    fn draw_u32s(ref self: Blake2sChannel) -> Span<u32> {
        draw_random_words(ref self).hash.span()
    }

    /// Check that `H(H(POW_PREFIX || digest || n_bits) || nonce)` has `n_bits` starting zeros.
    fn verify_pow_nonce(self: @Blake2sChannel, n_bits: u32, nonce: u64) -> bool {
        const POW_PREFIX: u32 = 0x12345678;
        let [d0, d1, d2, d3, d4, d5, d6, d7] = self.digest.hash.unbox();
        // Compute `POW_PREFIX || zeros  || digest || n_bits`.
        //          1 u32      || 6 u32s || 8 u32  || 1 u32.
        let msg = BoxImpl::new(
            [POW_PREFIX, 0, 0, 0, d0, d1, d2, d3, d4, d5, d6, d7, n_bits, 0, 0, 0],
        );
        let [q0, q1, q2, q3, q4, q5, q6, q7] = blake2s_finalize(
            BoxImpl::new(BLAKE2S_256_INITIAL_STATE), 52, msg,
        )
            .unbox();

        let (q, r) = div_rem(nonce, NZ_U32_SHIFT);
        let nonce_hi = upcast(q);
        let nonce_lo = upcast(r);
        let msg = BoxImpl::new(
            [q0, q1, q2, q3, q4, q5, q6, q7, nonce_lo, nonce_hi, 0, 0, 0, 0, 0, 0],
        );
        let digest = Blake2sHash {
            hash: blake2s_finalize(BoxImpl::new(BLAKE2S_256_INITIAL_STATE), 40, msg),
        };
        check_leading_zeros(digest, n_bits)
    }
}

/// Checks that the leading `n_bits` bits of the digest are zero.
/// `n_bits` is in range [0, 63].
///
/// # Panics
///
/// Panics if `n_bits` >= 64.
fn check_leading_zeros(digest: Blake2sHash, n_bits: u32) -> bool {
    const U64_2_POW_32: u64 = 0x100000000;
    let [d0, d1, _, _, _, _, _, _] = digest.hash.unbox();
    let v = d1.into() * U64_2_POW_32 + d0.into();

    let nonzero_divisor: NonZero<u64> = pow2_u64(n_bits).try_into().unwrap();
    let (_, r) = DivRem::div_rem(v, nonzero_divisor);
    r == 0
}

fn update_digest(ref channel: Blake2sChannel, new_digest: Blake2sHash) {
    channel.digest = new_digest;
    channel.n_draws = 0;
}

// TODO: Consider just returning secure felts.
fn draw_base_felts(ref channel: Blake2sChannel) -> Box<[M31; 8]> {
    loop {
        let [w0, w1, w2, w3, w4, w5, w6, w7] = draw_random_words(ref channel).hash.unbox();

        // Retry if not all the u32 are in the range [0, 2P).
        const P2: u32 = 0x7FFFFFFF * 2;
        if w0 < P2 && w1 < P2 && w2 < P2 && w3 < P2 && w4 < P2 && w5 < P2 && w6 < P2 && w7 < P2 {
            break BoxImpl::new(
                [
                    M31Trait::reduce_u32(w0), M31Trait::reduce_u32(w1), M31Trait::reduce_u32(w2),
                    M31Trait::reduce_u32(w3), M31Trait::reduce_u32(w4), M31Trait::reduce_u32(w5),
                    M31Trait::reduce_u32(w6), M31Trait::reduce_u32(w7),
                ],
            );
        }
    }
}

fn draw_random_words(ref channel: Blake2sChannel) -> Blake2sHash {
    let [d0, d1, d2, d3, d4, d5, d6, d7] = channel.digest.hash.unbox();
    let counter = channel.n_draws;
    let msg = BoxImpl::new([d0, d1, d2, d3, d4, d5, d6, d7, counter, 0, 0, 0, 0, 0, 0, 0]);
    channel.n_draws += 1;

    // Append a zero byte for domain separation between generating randomness and mixing a
    // single u32.
    Blake2sHash { hash: blake2s_finalize(BoxImpl::new(BLAKE2S_256_INITIAL_STATE), 37, msg) }
}
