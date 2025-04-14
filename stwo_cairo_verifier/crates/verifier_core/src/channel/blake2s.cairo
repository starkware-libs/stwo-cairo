use core::blake::{blake2s_compress, blake2s_finalize};
use core::box::BoxImpl;
use core::traits::DivRem;
use crate::SecureField;
use crate::fields::m31::{M31, m31};
use crate::fields::qm31::QM31Trait;
use crate::utils::gen_bit_mask;
use crate::vcs::blake2s_hasher::Blake2sHash;
use super::{ChannelTime, ChannelTimeImpl, ChannelTrait};

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
        const NZ_2_POW_32: NonZero<u64> = 0x100000000;
        let (q, r) = DivRem::div_rem(nonce, NZ_2_POW_32);
        let nonce_hi = q.try_into().unwrap();
        let nonce_lo = r.try_into().unwrap();
        let msg = [nonce_lo, nonce_hi, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let res = blake2s_compress(self.digest.hash, 0, BoxImpl::new(msg));
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

    fn check_proof_of_work(self: @Blake2sChannel, n_bits: u32) -> bool {
        const U128_2_POW_32: u128 = 0x100000000;
        let [d0, d1, d2, d3, _, _, _, _] = self.digest.hash.unbox();
        let v = d3.into();
        let v = v * U128_2_POW_32 + d2.into();
        let v = v * U128_2_POW_32 + d1.into();
        let v = v * U128_2_POW_32 + d0.into();
        v & gen_bit_mask(n_bits) == 0
    }
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
                [m31(w0), m31(w1), m31(w2), m31(w3), m31(w4), m31(w5), m31(w6), m31(w7)],
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

#[cfg(test)]
mod tests {
    use core::box::BoxImpl;
    use crate::fields::qm31::qm31_const;
    use crate::vcs::blake2s_hasher::Blake2sHash;
    use super::{Blake2sChannel, ChannelTrait, new_channel};

    #[test]
    fn test_blake_bytes() {
        let mut channel: Blake2sChannel = Default::default();

        let result = channel.draw_random_bytes();

        // Tested against velue produced from Rust code.
        // https://github.com/starkware-libs/stwo/blob/dev/crates/prover/src/core/channel/blake2s.rs
        assert_eq!(
            result,
            array![
                174, 9, 219, 124, 213, 79, 66, 180, 144, 239, 9, 182, 188, 84, 26, 246, 136, 228,
                149, 155, 184, 197, 63, 53, 154, 111, 86, 227, 138, 180, 84, 163,
            ],
        );
    }

    #[test]
    fn test_draw_felt() {
        let mut channel: Blake2sChannel = Default::default();

        let felt = channel.draw_felt();

        // Tested against values produced from Rust code.
        // https://github.com/starkware-libs/stwo/blob/dev/crates/prover/src/core/channel/blake2s.rs
        assert_eq!(felt, qm31_const::<2094729646, 876761046, 906620817, 1981437117>());
    }

    #[test]
    fn test_draw_felts() {
        let mut channel: Blake2sChannel = Default::default();

        let felts = channel.draw_felts(8);

        // Tested against values produced from Rust code.
        // https://github.com/starkware-libs/stwo/blob/dev/crates/prover/src/core/channel/blake2s.rs
        assert_eq!(
            felts,
            array![
                qm31_const::<2094729646, 876761046, 906620817, 1981437117>(),
                qm31_const::<462808201, 893371832, 1666609051, 592753803>(),
                qm31_const::<2092874317, 1414799646, 202729759, 1138457893>(),
                qm31_const::<740261418, 1566411288, 1094134286, 1085813917>(),
                qm31_const::<1782652641, 591937235, 375882621, 687600507>(),
                qm31_const::<417708784, 676515713, 1053713500, 313648782>(),
                qm31_const::<1896458727, 242850046, 267152034, 827396985>(),
                qm31_const::<1959202869, 765813487, 1783334404, 305015811>(),
            ],
        );
    }

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

        channel.mix_u64(0x1211109876543210);

        // Tested against values produced from Rust code.
        // https://github.com/starkware-libs/stwo/blob/dev/crates/prover/src/core/channel/blake2s.rs
        assert_eq!(
            channel.digest.hash.unbox(),
            [
                743523477, 161816109, 3300966720, 3503887744, 929103465, 2486638855, 1826907926,
                3137305201,
            ],
        );
    }

    #[test]
    fn test_check_proof_of_work() {
        let channel = new_channel(
            Blake2sHash { hash: BoxImpl::new([0b1000, 0, 0, 0, 0, 0, 0, 0]) },
        );

        let res = channel.check_proof_of_work(3);

        assert!(res);
    }

    #[test]
    fn test_check_proof_of_work_with_invalid_n_bits() {
        let channel = new_channel(
            Blake2sHash { hash: BoxImpl::new([0b1000, 0, 0, 0, 0, 0, 0, 0]) },
        );

        let res = channel.check_proof_of_work(4);

        assert!(!res);
    }
}
