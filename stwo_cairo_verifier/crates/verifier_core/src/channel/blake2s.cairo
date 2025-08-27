use bounded_int::impls::*;
use bounded_int::{NZ_U32_SHIFT, NZ_U8_SHIFT, div_rem, upcast};
use core::blake::{blake2s_compress, blake2s_finalize};
use core::box::BoxImpl;
use stwo_verifier_utils::{
    BLAKE2S_256_INITIAL_STATE, MemorySection, hash_memory_section_with_digest,
};
use crate::SecureField;
use crate::fields::m31::{M31, M31Trait};
use crate::fields::qm31::QM31Trait;
use crate::utils::pow2_u64;
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
pub impl Blake2sChannelImpl of ChannelTrait {
    fn mix_root(ref self: Blake2sChannel, root: Blake2sHash) {
        let [d0, d1, d2, d3, d4, d5, d6, d7] = self.digest.hash.unbox();
        let [r0, r1, r2, r3, r4, r5, r6, r7] = root.hash.unbox();
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
        self.mix_u32s(array![nonce_lo, nonce_hi].span());
    }

    fn mix_u32s(ref self: Blake2sChannel, data: Span<u32>) {
        let [d0, d1, d2, d3, d4, d5, d6, d7] = self.digest.hash.unbox();
        let mut state = BoxImpl::new(BLAKE2S_256_INITIAL_STATE);

        let mut buffer = array![d0, d1, d2, d3, d4, d5, d6, d7];
        let mut byte_count = 32;

        let mut data = data;

        while let Some(head) = data.multi_pop_front::<8>() {
            // Compress whenever the buffer reaches capacity.
            if let Some(msg) = buffer.span().try_into() {
                state = blake2s_compress(state, byte_count, *msg);
                buffer = array![];
            }

            buffer.append_span(head.unbox().span());
            byte_count += 32;
        }

        // Compress again if buffer is full but there is more data for finalize.
        if !data.is_empty() {
            if let Some(msg) = buffer.span().try_into() {
                state = blake2s_compress(state, byte_count, *msg);
                buffer = array![];
            }
        }

        buffer.append_span(data);
        byte_count += data.len() * 4;
        for _ in buffer.len()..16 {
            buffer.append(0);
        }

        let res = blake2s_finalize(state, byte_count, *buffer.span().try_into().unwrap());
        update_digest(ref self, Blake2sHash { hash: res });
    }

    fn mix_memory_section(ref self: Blake2sChannel, data: MemorySection) {
        let res = hash_memory_section_with_digest(data, self.digest.hash);
        update_digest(ref self, Blake2sHash { hash: res });
    }

    fn draw_secure_felt(ref self: Blake2sChannel) -> SecureField {
        let [r0, r1, r2, r3, _, _, _, _] = draw_random_base_felts(ref self).unbox();
        QM31Trait::from_fixed_array([r0, r1, r2, r3])
    }

    fn draw_secure_felts(ref self: Blake2sChannel, mut n_felts: usize) -> Array<SecureField> {
        let mut res = array![];

        while n_felts != 0 {
            let [r0, r1, r2, r3, r4, r5, r6, r7] = draw_random_base_felts(ref self).unbox();
            res.append(QM31Trait::from_fixed_array([r0, r1, r2, r3]));
            if n_felts == 1 {
                break;
            }
            res.append(QM31Trait::from_fixed_array([r4, r5, r6, r7]));
            n_felts -= 2;
        }

        res
    }

    fn draw_random_bytes(ref self: Blake2sChannel) -> Array<u8> {
        let words = draw_random_words(ref self).hash.unbox();
        let mut bytes = array![];

        for word in words.span() {
            let (q, r) = div_rem(*word, NZ_U8_SHIFT);
            bytes.append(upcast(r));
            let (q, r) = div_rem(q, NZ_U8_SHIFT);
            bytes.append(upcast(r));
            let (q, r) = div_rem(q, NZ_U8_SHIFT);
            bytes.append(upcast(r));
            bytes.append(upcast(q));
        }

        bytes
    }

    fn mix_and_check_pow_nonce(ref self: Blake2sChannel, n_bits: u32, nonce: u64) -> bool {
        self.mix_u64(nonce);
        check_proof_of_work(self.digest, n_bits)
    }
}

/// Checks that the last `n_bits` bits of the digest are zero.
/// `n_bits` is in range [0, 64].
fn check_proof_of_work(digest: Blake2sHash, n_bits: u32) -> bool {
    const U64_2_POW_32: u64 = 0x100000000;
    let [d0, d1, _, _, _, _, _, _] = digest.hash.unbox();
    let v = d1.into() * U64_2_POW_32 + d0.into();
    // Handle the special case of 64 bits.
    if n_bits == 64 {
        return v == 0;
    }
    // If n_bits > 64 pow2_u64 will panic with index out of bounds.
    let divisor = pow2_u64(n_bits).try_into().unwrap();
    let (_, r) = DivRem::div_rem(v, divisor);
    r == 0
}

fn update_digest(ref channel: Blake2sChannel, new_digest: Blake2sHash) {
    channel.digest = new_digest;
    channel.n_draws = 0;
}

// TODO: Consider just returning secure felts.
fn draw_random_base_felts(ref channel: Blake2sChannel) -> Box<[M31; 8]> {
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
    Blake2sHash { hash: blake2s_finalize(BoxImpl::new(BLAKE2S_256_INITIAL_STATE), 36, msg) }
}
