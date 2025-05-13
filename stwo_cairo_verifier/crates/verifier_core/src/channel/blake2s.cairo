use bounded_int::upcast;
use core::blake::{blake2s_compress, blake2s_finalize};
use core::box::BoxImpl;
use core::traits::DivRem;
use crate::SecureField;
use crate::fields::m31::{M31InnerT, M31Trait};
use crate::fields::qm31::QM31Trait;
use crate::utils::gen_bit_mask;
use crate::vcs::blake2s_hasher::Blake2sHash;
pub use super::{ChannelTime, ChannelTimeImpl, ChannelTrait};

/// Equals `2^31`.
const M31_SHIFT_NZ_U256: NonZero<u256> = 0x80000000;

/// Number of `M31` per hash.
pub const FELTS_PER_HASH: usize = 8;

const BYTES_PER_HASH: usize = 32;

// TODO: Stone uses a different initial state with the key set to 0.
// Consider using this initial state instead.
pub const BLAKE2S_256_INITIAL_STATE: [u32; 8] = [
    0x6B08E647, 0xBB67AE85, 0x3C6EF372, 0xA54FF53A, 0x510E527F, 0x9B05688C, 0x1F83D9AB, 0x5BE0CD19,
];

#[derive(Drop)]
pub struct Blake2sChannel {
    digest: Blake2sHash,
    channel_time: ChannelTime,
}

pub fn new_channel(digest: Blake2sHash) -> Blake2sChannel {
    Blake2sChannel { digest, channel_time: Default::default() }
}

impl Blake2sChannelDefault of Default<Blake2sChannel> {
    fn default() -> Blake2sChannel {
        Blake2sChannel {
            digest: Blake2sHash { hash: BoxImpl::new([0; 8]) }, channel_time: Default::default(),
        }
    }
}

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
            if let Some(msg) = buffer.span().try_into() {
                state = blake2s_compress(state, byte_count, *msg);
                buffer = array![];
            }

            let [r0, r1, r2, r3] = felt.to_array();
            buffer.append(upcast(r0));
            buffer.append(upcast(r1));
            buffer.append(upcast(r2));
            buffer.append(upcast(r3));
            byte_count += 16;
        }

        for _ in buffer.len()..16 {
            buffer.append(0);
        }

        let res = blake2s_finalize(state, byte_count, *buffer.span().try_into().unwrap());
        update_digest(ref self, Blake2sHash { hash: res });
    }

    fn mix_u64(ref self: Blake2sChannel, nonce: u64) {
        const NZ_2_POW_32: NonZero<u64> = 0x100000000;
        let (q, r) = DivRem::div_rem(nonce, NZ_2_POW_32);
        let nonce_hi = q.try_into().unwrap();
        let nonce_lo = r.try_into().unwrap();
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

    fn draw_felt(ref self: Blake2sChannel) -> SecureField {
        let [r0, r1, r2, r3, _, _, _, _] = draw_random_base_felts(ref self).unbox();
        QM31Trait::from_array([r0, r1, r2, r3])
    }

    fn draw_felts(ref self: Blake2sChannel, mut n_felts: usize) -> Array<SecureField> {
        let mut res = array![];

        while n_felts != 0 {
            let [r0, r1, r2, r3, r4, r5, r6, r7] = draw_random_base_felts(ref self).unbox();
            res.append(QM31Trait::from_array([r0, r1, r2, r3]));
            if n_felts == 1 {
                break;
            }
            res.append(QM31Trait::from_array([r4, r5, r6, r7]));
            n_felts -= 2;
        }

        res
    }

    fn draw_random_bytes(ref self: Blake2sChannel) -> Array<u8> {
        let words = draw_random_words(ref self).hash.unbox();
        let mut bytes = array![];

        for word in words.span() {
            let (q, r) = DivRem::div_rem(*word, 0x100);
            bytes.append(r.try_into().unwrap());
            let (q, r) = DivRem::div_rem(q, 0x100);
            bytes.append(r.try_into().unwrap());
            let (q, r) = DivRem::div_rem(q, 0x100);
            bytes.append(r.try_into().unwrap());
            let (_, r) = DivRem::div_rem(q, 0x100);
            bytes.append(r.try_into().unwrap());
        }

        bytes
    }

    fn mix_and_check_pow_nonce(ref self: Blake2sChannel, n_bits: u32, nonce: u64) -> bool {
        self.mix_u64(nonce);
        check_proof_of_work(self.digest, n_bits)
    }
}
pub fn get_digest(ref channel: Blake2sChannel) -> [u32; 8] {
    channel.digest.hash.unbox()
}

pub fn check_proof_of_work(digest: Blake2sHash, n_bits: u32) -> bool {
    const U128_2_POW_32: u128 = 0x100000000;
    let [d0, d1, d2, d3, _, _, _, _] = digest.hash.unbox();
    let v = d3.into();
    let v = v * U128_2_POW_32 + d2.into();
    let v = v * U128_2_POW_32 + d1.into();
    let v = v * U128_2_POW_32 + d0.into();
    v & gen_bit_mask(n_bits) == 0
}

fn update_digest(ref channel: Blake2sChannel, new_digest: Blake2sHash) {
    channel.digest = new_digest;
    channel.channel_time.inc_challenges();
}

// TODO: Consider just returning secure felts.
fn draw_random_base_felts(ref channel: Blake2sChannel) -> Box<[M31InnerT; 8]> {
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
    let counter = channel.channel_time.n_sent.into();
    let msg = BoxImpl::new([d0, d1, d2, d3, d4, d5, d6, d7, counter, 0, 0, 0, 0, 0, 0, 0]);
    channel.channel_time.inc_sent();
    Blake2sHash { hash: blake2s_finalize(BoxImpl::new(BLAKE2S_256_INITIAL_STATE), 64, msg) }
}
