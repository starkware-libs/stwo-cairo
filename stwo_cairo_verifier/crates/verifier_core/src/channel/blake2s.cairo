use bounded_int::*;
use core::blake::{blake2s_compress, blake2s_finalize};
use core::box::BoxImpl;
use crate::SecureField;
use crate::fields::m31::{M31, M31Trait};
use crate::fields::qm31::QM31Trait;
use crate::utils::gen_bit_mask;
use crate::vcs::blake2s_hasher::Blake2sHash;
use super::{ChannelTime, ChannelTimeImpl, ChannelTrait};

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

    fn draw_felt(ref self: Blake2sChannel) -> SecureField {
        let [r0, r1, r2, r3, _, _, _, _] = draw_random_base_felts(ref self).unbox();
        QM31Trait::from_fixed_array([r0, r1, r2, r3])
    }

    fn draw_felts(ref self: Blake2sChannel, mut n_felts: usize) -> Array<SecureField> {
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

fn check_proof_of_work(digest: Blake2sHash, n_bits: u32) -> bool {
    const U64_2_POW_32: u64 = 0x100000000;
    let [d0, d1, _, _, _, _, _, _] = digest.hash.unbox();
    let v = d1.into() * U64_2_POW_32 + d0.into();
    v & gen_bit_mask(n_bits) == 0
}

fn update_digest(ref channel: Blake2sChannel, new_digest: Blake2sHash) {
    channel.digest = new_digest;
    channel.channel_time.inc_challenges();
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
    let counter = channel.channel_time.n_sent.into();
    let msg = BoxImpl::new([d0, d1, d2, d3, d4, d5, d6, d7, counter, 0, 0, 0, 0, 0, 0, 0]);
    channel.channel_time.inc_sent();
    Blake2sHash { hash: blake2s_finalize(BoxImpl::new(BLAKE2S_256_INITIAL_STATE), 36, msg) }
}

#[cfg(test)]
mod tests {
    use core::box::BoxImpl;
    use crate::fields::qm31::qm31_const;
    use crate::vcs::blake2s_hasher::Blake2sHash;
    use super::{Blake2sChannel, ChannelTrait, check_proof_of_work};
    #[test]
    fn test_mix_felts_with_1_felt() {
        let mut channel: Blake2sChannel = Default::default();

        channel.mix_felts([qm31_const::<1, 2, 3, 4>()].span());

        // Tested against values produced from Rust code.
        // https://github.com/starkware-libs/stwo/blob/dev/crates/prover/src/core/channel/blake2s.rs
        assert_eq!(
            channel.digest.hash.unbox(),
            [
                1586304710, 1167332849, 1688630032, 429142330, 4001363212, 2013799503, 180553907,
                2044853257,
            ],
        );
    }

    #[test]
    fn test_mix_felts_with_2_felts() {
        let mut channel: Blake2sChannel = Default::default();

        channel.mix_felts([qm31_const::<1, 2, 3, 4>(), qm31_const::<5, 6, 7, 8>()].span());

        // Tested against values produced from Rust code.
        // https://github.com/starkware-libs/stwo/blob/dev/crates/prover/src/core/channel/blake2s.rs
        assert_eq!(
            channel.digest.hash.unbox(),
            [
                1835698174, 2969628929, 1758616107, 158303712, 3820231193, 179192886, 4063347398,
                3332297509,
            ],
        );
    }

    #[test]
    fn test_mix_felts_with_3_felts() {
        let mut channel: Blake2sChannel = Default::default();

        channel
            .mix_felts(
                [
                    qm31_const::<1, 2, 3, 4>(), qm31_const::<5, 6, 7, 8>(),
                    qm31_const::<9, 10, 11, 12>(),
                ]
                    .span(),
            );

        // Tested against values produced from Rust code.
        // https://github.com/starkware-libs/stwo/blob/dev/crates/prover/src/core/channel/blake2s.rs
        assert_eq!(
            channel.digest.hash.unbox(),
            [
                2116479765, 3227507660, 1737697798, 2518684651, 1068812914, 1858078313, 1722202885,
                2198022752,
            ],
        );
    }

    #[test]
    fn test_mix_felts_with_4_felts() {
        let mut channel: Blake2sChannel = Default::default();

        channel
            .mix_felts(
                [
                    qm31_const::<1, 2, 3, 4>(), qm31_const::<5, 6, 7, 8>(),
                    qm31_const::<9, 10, 11, 12>(), qm31_const::<13, 14, 15, 16>(),
                ]
                    .span(),
            );

        // Tested against values produced from Rust code.
        // https://github.com/starkware-libs/stwo/blob/dev/crates/prover/src/core/channel/blake2s.rs
        assert_eq!(
            channel.digest.hash.unbox(),
            [
                940149128, 1354728945, 2816315586, 1690943110, 210254904, 3746481728, 1339132640,
                3760408575,
            ],
        );
    }

    #[test]
    fn test_mix_felts_with_5_felts() {
        let mut channel: Blake2sChannel = Default::default();

        channel
            .mix_felts(
                [
                    qm31_const::<1, 2, 3, 4>(), qm31_const::<5, 6, 7, 8>(),
                    qm31_const::<9, 10, 11, 12>(), qm31_const::<13, 14, 15, 16>(),
                    qm31_const::<17, 18, 19, 20>(),
                ]
                    .span(),
            );

        // Tested against values produced from Rust code.
        // https://github.com/starkware-libs/stwo/blob/dev/crates/prover/src/core/channel/blake2s.rs
        assert_eq!(
            channel.digest.hash.unbox(),
            [
                3425911356, 1462327982, 3241135902, 4212900065, 3145879221, 3413011910, 3946733048,
                4081152200,
            ],
        );
    }

    #[test]
    fn test_mix_u64() {
        let mut channel: Blake2sChannel = Default::default();

        channel.mix_u64(0x1111222233334444);

        // Tested against values produced from Rust code.
        // https://github.com/starkware-libs/stwo/blob/dev/crates/prover/src/core/channel/blake2s.rs
        assert_eq!(
            channel.digest.hash.unbox(),
            [
                0xc13f9ebc, 0x97884ed2, 0x59336d95, 0x24977332, 0xcdca6b9d, 0x74924d22, 0x4abae704,
                0xce6edc77,
            ],
        );
    }

    #[test]
    pub fn test_mix_u32s() {
        let mut channel: Blake2sChannel = Default::default();

        channel.mix_u32s(array![1, 2, 3, 4, 5, 6, 7, 8, 9].span());

        // Tested against values produced from Rust code.
        // https://github.com/starkware-libs/stwo/blob/dev/crates/prover/src/core/channel/blake2s.rs
        assert_eq!(
            channel.digest.hash.unbox(),
            [
                0x83769170, 0xb31bbb57, 0xb6da6f34, 0xfad757b3, 0xe3fbb846, 0x24432e2c, 0x94c2ffa0,
                0xc7a1f9cb,
            ],
        );
    }

    #[test]
    fn test_check_proof_of_work() {
        let digest = Blake2sHash { hash: BoxImpl::new([0b1000, 0, 0, 0, 0, 0, 0, 0]) };

        let res = check_proof_of_work(digest, 3);

        assert!(res);
    }

    #[test]
    fn test_check_proof_of_work_with_invalid_n_bits() {
        let digest = Blake2sHash { hash: BoxImpl::new([0b1000, 0, 0, 0, 0, 0, 0, 0]) };

        let res = check_proof_of_work(digest, 4);

        assert!(!res);
    }
}
