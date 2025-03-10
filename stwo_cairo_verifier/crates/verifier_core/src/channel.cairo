use core::array::SpanTrait;
use core::num::traits::{WrappingMul, WrappingSub};
use core::poseidon::{hades_permutation, poseidon_hash_span};
use core::traits::DivRem;
use crate::fields::m31::M31Trait;
use crate::fields::qm31::QM31Trait;
use crate::utils::pack4;
use crate::{BaseField, SecureField};

/// Equals `2^31`.
const M31_SHIFT_NZ_U256: NonZero<u256> = 0x80000000;

/// Number of `M31` per hash.
pub const FELTS_PER_HASH: usize = 8;

#[derive(Default, Drop)]
pub struct ChannelTime {
    n_challenges: usize,
    n_sent: usize,
}

#[generate_trait]
impl ChannelTimeImpl of ChannelTimeTrait {
    fn inc_sent(ref self: ChannelTime) {
        self.n_sent += 1;
    }

    fn inc_challenges(ref self: ChannelTime) {
        self.n_challenges += 1;
        self.n_sent = 0;
    }
}

#[derive(Drop)]
pub struct Channel {
    digest: felt252,
    channel_time: ChannelTime,
}

#[generate_trait]
pub impl ChannelImpl of ChannelTrait {
    fn new(digest: felt252) -> Channel {
        Channel { digest, channel_time: Default::default() }
    }

    fn get_digest(ref self: Channel) -> felt252 {
        self.digest
    }

    fn draw_felt252(ref self: Channel) -> felt252 {
        let (res, _, _) = hades_permutation(self.digest, self.channel_time.n_sent.into(), 2);
        self.channel_time.inc_sent();
        res
    }

    // TODO(spapini): Check that this is sound.
    #[inline]
    fn draw_base_felts(ref self: Channel) -> [BaseField; FELTS_PER_HASH] {
        let mut cur = self.draw_felt252().into();
        [
            extract_m31(ref cur), extract_m31(ref cur), extract_m31(ref cur), extract_m31(ref cur),
            extract_m31(ref cur), extract_m31(ref cur), extract_m31(ref cur), extract_m31(ref cur),
        ]
    }

    fn mix_digest(ref self: Channel, digest: felt252) {
        let (s0, _, _) = hades_permutation(self.digest, digest, 2);
        self.digest = s0;
        self.channel_time.inc_challenges();
    }

    fn mix_felts(ref self: Channel, mut felts: Span<SecureField>) {
        let mut res = array![self.digest];
        loop {
            match (felts.pop_front(), felts.pop_front()) {
                (None, _) => { break; },
                (Some(x), None) => {
                    res.append(pack4(0, (*x).to_array()));
                    break;
                },
                (
                    Some(x), Some(y),
                ) => {
                    let cur = pack4(0, (*x).to_array());
                    res.append(pack4(cur, (*y).to_array()));
                },
            };
        }

        self.digest = poseidon_hash_span(res.span());

        // TODO(spapini): do we need length padding?
        self.channel_time.inc_challenges();
    }

    fn mix_nonce(ref self: Channel, nonce: u64) {
        self.mix_digest(nonce.into())
    }

    fn draw_felt(ref self: Channel) -> SecureField {
        let [r0, r1, r2, r3, _, _, _, _] = self.draw_base_felts();
        QM31Trait::from_array([r0, r1, r2, r3])
    }

    fn draw_felts(ref self: Channel, mut n_felts: usize) -> Array<SecureField> {
        let mut res: Array = Default::default();
        loop {
            if n_felts == 0 {
                break;
            }
            let [r0, r1, r2, r3, r4, r5, r6, r7] = self.draw_base_felts();
            res.append(QM31Trait::from_array([r0, r1, r2, r3]));
            if n_felts == 1 {
                break;
            }
            res.append(QM31Trait::from_array([r4, r5, r6, r7]));
            n_felts -= 2;
        }
        res
    }

    /// Returns 31 random bytes computed as the first 31 bytes of the representative of
    /// `self.draw_felt252()` in little endian.
    /// TODO: check that this distribution is good enough, as it is only close to uniform.
    fn draw_random_bytes(ref self: Channel) -> Array<u8> {
        let mut cur: u256 = self.draw_felt252().into();
        let mut bytes = array![];
        for _ in 0_usize..31 {
            let (q, r) = DivRem::div_rem(cur, 256);
            bytes.append(r.try_into().unwrap());
            cur = q;
        }
        bytes
    }

    fn check_proof_of_work(self: @Channel, n_bits: u32) -> bool {
        let u256 { low, .. } = (*self.digest).into();
        low & gen_bit_mask(n_bits) == 0
    }
}

/// Generates a bit mask with the least significant `n_bits` set to 1.
fn gen_bit_mask(n_bits: u32) -> u128 {
    assert!(n_bits <= 128);
    let mut mask = 1;
    for _ in 0..n_bits {
        mask = mask.wrapping_mul(2);
    }
    mask = mask.wrapping_sub(1);
    mask
}

#[inline]
fn extract_m31<const N: usize>(ref num: u256) -> BaseField {
    let (q, r) = DivRem::div_rem(num, M31_SHIFT_NZ_U256);
    num = q;
    M31Trait::reduce_u128(r.low)
}


#[cfg(test)]
mod tests {
    use crate::fields::qm31::qm31;
    use super::{Channel, ChannelTrait, gen_bit_mask};

    #[test]
    fn test_initialize_channel() {
        let initial_digest = 0;
        let channel = ChannelTrait::new(initial_digest);

        // Assert that the channel is initialized correctly.
        assert_eq!(channel.digest, initial_digest);
        assert_eq!(channel.channel_time.n_challenges, 0);
        assert_eq!(channel.channel_time.n_sent, 0);
    }

    #[test]
    fn test_channel_time() {
        let initial_digest = 0;
        let mut channel = ChannelTrait::new(initial_digest);

        assert_eq!(channel.channel_time.n_challenges, 0);
        assert_eq!(channel.channel_time.n_sent, 0);

        channel.draw_felt();
        assert_eq!(channel.channel_time.n_challenges, 0);
        assert_eq!(channel.channel_time.n_sent, 1);

        channel.draw_felts(9);
        assert_eq!(channel.channel_time.n_challenges, 0);
        assert_eq!(channel.channel_time.n_sent, 6);

        channel.mix_digest(0);
        assert_eq!(channel.channel_time.n_challenges, 1);
        assert_eq!(channel.channel_time.n_sent, 0);

        channel.draw_felt();
        assert_eq!(channel.channel_time.n_challenges, 1);
        assert_eq!(channel.channel_time.n_sent, 1);
        assert_ne!(channel.digest, initial_digest);
    }


    #[test]
    pub fn test_draw_felt() {
        let initial_digest = 0;
        let mut channel = ChannelTrait::new(initial_digest);

        let first_random_felt = channel.draw_felt();

        // Assert that next random felt is different.
        assert_ne!(first_random_felt, channel.draw_felt());
    }

    #[test]
    pub fn test_draw_felts() {
        let initial_digest = 0;
        let mut channel = ChannelTrait::new(initial_digest);

        let mut random_felts = channel.draw_felts(5);
        random_felts.append_span(channel.draw_felts(4).span());

        // Assert that all the random felts are unique.
        assert_ne!(random_felts[0], random_felts[5]);
    }

    #[test]
    pub fn test_mix_digest() {
        let initial_digest = 0;
        let mut channel = ChannelTrait::new(initial_digest);

        for _ in 0_usize..10 {
            channel.draw_felt();
        }

        let prev_digest = channel.digest;
        channel.mix_digest(0);
        assert_ne!(prev_digest, channel.digest);
    }

    #[test]
    pub fn test_mix_felts() {
        let initial_digest = 0;
        let mut channel = ChannelTrait::new(initial_digest);

        channel.mix_felts(array![qm31(1, 2, 3, 4), qm31(5, 6, 7, 8), qm31(9, 10, 11, 12)].span());

        assert_ne!(initial_digest, channel.digest);
    }

    #[test]
    pub fn test_draw_random_bytes_1() {
        let initial_digest = 0;
        let mut channel = ChannelTrait::new(initial_digest);
        let result = channel.draw_random_bytes();
        let expected_result = array![
            197,
            20,
            139,
            143,
            49,
            135,
            207,
            202,
            93,
            167,
            20,
            244,
            184,
            186,
            20,
            136,
            204,
            43,
            46,
            147,
            213,
            253,
            175,
            170,
            13,
            64,
            15,
            168,
            232,
            211,
            147,
        ];
        assert_eq!(expected_result, result);
    }

    #[test]
    pub fn test_draw_random_bytes_2() {
        let initial_digest = 0xdeadbeef;
        let mut channel = ChannelTrait::new(initial_digest);
        let result = channel.draw_random_bytes();
        let expected_result = array![
            168,
            175,
            85,
            209,
            218,
            65,
            155,
            212,
            165,
            88,
            130,
            167,
            44,
            242,
            17,
            127,
            75,
            251,
            142,
            180,
            157,
            176,
            27,
            167,
            179,
            247,
            27,
            113,
            149,
            41,
            12,
        ];
        assert_eq!(expected_result, result);
    }

    #[test]
    pub fn test_draw_random_bytes_3() {
        let initial_digest = 0xcafecafe;
        let mut channel = ChannelTrait::new(initial_digest);
        let first_result = channel.draw_random_bytes();
        let second_result = channel.draw_random_bytes();
        assert_ne!(first_result, second_result);
    }

    #[test]
    fn test_gen_bit_mask_with_0_bits() {
        assert_eq!(gen_bit_mask(0), 0);
    }

    #[test]
    fn test_gen_bit_mask_with_8_bits() {
        assert_eq!(gen_bit_mask(8), 0b11111111);
    }

    #[test]
    fn test_gen_bit_mask_with_128_bits() {
        assert_eq!(gen_bit_mask(128), 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF);
    }

    #[test]
    fn test_check_proof_of_work() {
        let channel = Channel { digest: 0b1000, channel_time: Default::default() };

        let res = channel.check_proof_of_work(3);

        assert!(res);
    }

    #[test]
    fn test_check_proof_of_work_with_invalid_n_bits() {
        let channel = Channel { digest: 0b1000, channel_time: Default::default() };

        let res = channel.check_proof_of_work(4);

        assert!(!res);
    }
}
