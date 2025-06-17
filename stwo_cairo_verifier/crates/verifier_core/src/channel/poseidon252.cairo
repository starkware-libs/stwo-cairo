use core::array::SpanTrait;
use core::poseidon::{hades_permutation, poseidon_hash_span};
use core::traits::DivRem;
use crate::SecureField;
use crate::fields::m31::{M31, M31Trait};
use crate::fields::qm31::QM31Trait;
use crate::utils::{gen_bit_mask, pack4};
use super::{ChannelTime, ChannelTimeImpl, ChannelTrait};

/// Equals `2^31`.
const M31_SHIFT_NZ_U256: NonZero<u256> = 0x80000000;

/// Number of `M31` per hash.
pub const FELTS_PER_HASH: usize = 8;

pub const BYTES_PER_HASH: usize = 31;

/// Constructs a `felt252` from 7 u32 big-endian limbs.
pub fn construct_f252_be(x: Box<[u32; 7]>) -> felt252 {
    let [l0, l1, l2, l3, l4, l5, l6] = x.unbox();
    let offset = 0x100000000;
    let result: felt252 = l0.into();
    let result = result * offset + l1.into();
    let result = result * offset + l2.into();
    let result = result * offset + l3.into();
    let result = result * offset + l4.into();
    let result = result * offset + l5.into();
    result * offset + l6.into()
}

#[derive(Drop, Default)]
pub struct Poseidon252Channel {
    digest: felt252,
    channel_time: ChannelTime,
}

#[generate_trait]
impl Posidon252ChannelHelperImpl of Poseidon252ChannelHelper {
    /// Helper function to mix a single `felt252` into the channel.
    #[inline(always)]
    fn mix_felt252(ref self: Poseidon252Channel, x: felt252) {
        let (s0, _, _) = hades_permutation(self.digest, x, 2);
        self.digest = s0;
        self.channel_time.inc_challenges();
    }
}
pub impl Poseidon252ChannelImpl of ChannelTrait {
    fn mix_root(ref self: Poseidon252Channel, root: felt252) {
        self.mix_felt252(root);
    }

    fn mix_felts(ref self: Poseidon252Channel, mut felts: Span<SecureField>) {
        let mut res = array![self.digest];
        loop {
            match (felts.pop_front(), felts.pop_front()) {
                (None, _) => { break; },
                (Some(x), None) => {
                    res.append(pack4(1, (*x).to_fixed_array()));
                    break;
                },
                (
                    Some(x), Some(y),
                ) => {
                    let cur = pack4(1, (*x).to_fixed_array());
                    res.append(pack4(cur, (*y).to_fixed_array()));
                },
            };
        }

        self.digest = poseidon_hash_span(res.span());

        // TODO(spapini): do we need length padding?
        self.channel_time.inc_challenges();
    }

    fn mix_u64(ref self: Poseidon252Channel, nonce: u64) {
        self.mix_felt252(nonce.into());
    }

    fn mix_u32s(ref self: Poseidon252Channel, data: Span<u32>) {
        let mut res = array![self.digest];

        let mut data = data;

        while let Some(chunk) = data.multi_pop_front::<7>() {
            res.append(construct_f252_be(*chunk));
        }

        if !data.is_empty() {
            let mut chunk: Array<u32> = array![];
            chunk.append_span(data);
            for _ in data.len()..7 {
                chunk.append(0);
            }
            res.append(construct_f252_be(*chunk.span().try_into().unwrap()));
        }

        self.digest = poseidon_hash_span(res.span());

        // TODO(spapini): do we need length padding?
        self.channel_time.inc_challenges();
    }

    fn draw_felt(ref self: Poseidon252Channel) -> SecureField {
        let [r0, r1, r2, r3, _, _, _, _] = draw_base_felts(ref self);
        QM31Trait::from_fixed_array([r0, r1, r2, r3])
    }

    fn draw_felts(ref self: Poseidon252Channel, mut n_felts: usize) -> Array<SecureField> {
        let mut res: Array = Default::default();
        loop {
            if n_felts == 0 {
                break;
            }
            let [r0, r1, r2, r3, r4, r5, r6, r7] = draw_base_felts(ref self);
            res.append(QM31Trait::from_fixed_array([r0, r1, r2, r3]));
            if n_felts == 1 {
                break;
            }
            res.append(QM31Trait::from_fixed_array([r4, r5, r6, r7]));
            n_felts -= 2;
        }
        res
    }

    /// Returns 31 random bytes computed as the first 31 bytes of the representative of
    /// `self.draw_felt252()` in little endian.
    /// The distribution for each byte is epsilon close to uniform with epsilon bounded by 2^(-60).
    fn draw_random_bytes(ref self: Poseidon252Channel) -> Array<u8> {
        let mut cur: u256 = draw_felt252(ref self).into();
        let mut bytes = array![];
        for _ in 0_usize..BYTES_PER_HASH {
            let (q, r) = DivRem::div_rem(cur, 0x100);
            bytes.append(r.try_into().unwrap());
            cur = q;
        }
        bytes
    }

    fn mix_and_check_pow_nonce(ref self: Poseidon252Channel, n_bits: u32, nonce: u64) -> bool {
        // TODO(andrew): Use blake for proof of work.
        self.mix_u64(nonce);
        check_proof_of_work(self.digest, n_bits)
    }
}

fn check_proof_of_work(digest: felt252, n_bits: u32) -> bool {
    let u256 { low, .. } = digest.into();
    low & gen_bit_mask(n_bits).into() == 0
}

// TODO(spapini): Check that this is sound.
fn draw_base_felts(ref channel: Poseidon252Channel) -> [M31; FELTS_PER_HASH] {
    let mut cur: u256 = draw_felt252(ref channel).into();
    [
        extract_m31(ref cur), extract_m31(ref cur), extract_m31(ref cur), extract_m31(ref cur),
        extract_m31(ref cur), extract_m31(ref cur), extract_m31(ref cur), extract_m31(ref cur),
    ]
}

fn draw_felt252(ref channel: Poseidon252Channel) -> felt252 {
    let (res, _, _) = hades_permutation(channel.digest, channel.channel_time.n_sent.into(), 2);
    channel.channel_time.inc_sent();
    res
}

#[inline]
fn extract_m31(ref num: u256) -> M31 {
    let (q, r) = DivRem::div_rem(num, M31_SHIFT_NZ_U256);
    num = q;
    M31Trait::reduce_u128(r.low)
}

#[cfg(test)]
mod tests {
    use crate::fields::qm31::qm31_const;
    use super::{ChannelTrait, Poseidon252Channel, Poseidon252ChannelImpl, check_proof_of_work};

    #[test]
    fn test_initialize_channel() {
        let channel: Poseidon252Channel = Default::default();

        // Assert that the channel is initialized correctly.
        assert_eq!(channel.digest, 0);
        assert_eq!(channel.channel_time.n_challenges, 0);
        assert_eq!(channel.channel_time.n_sent, 0);
    }

    #[test]
    fn test_channel_time() {
        let mut channel: Poseidon252Channel = Default::default();

        assert_eq!(channel.channel_time.n_challenges, 0);
        assert_eq!(channel.channel_time.n_sent, 0);

        channel.draw_felt();
        assert_eq!(channel.channel_time.n_challenges, 0);
        assert_eq!(channel.channel_time.n_sent, 1);

        channel.draw_felts(9);
        assert_eq!(channel.channel_time.n_challenges, 0);
        assert_eq!(channel.channel_time.n_sent, 6);

        channel.mix_root(0);
        assert_eq!(channel.channel_time.n_challenges, 1);
        assert_eq!(channel.channel_time.n_sent, 0);

        channel.draw_felt();
        assert_eq!(channel.channel_time.n_challenges, 1);
        assert_eq!(channel.channel_time.n_sent, 1);
        assert_ne!(channel.digest, 0);
    }


    #[test]
    pub fn test_draw_felt() {
        let initial_digest = 0;
        let mut channel = new_channel(initial_digest);

        let first_random_felt = channel.draw_felt();

        // Assert that next random felt is different.
        assert_ne!(first_random_felt, channel.draw_felt());
    }

    #[test]
    pub fn test_draw_felts() {
        let initial_digest = 0;
        let mut channel = new_channel(initial_digest);

        let mut random_felts = channel.draw_felts(5);
        random_felts.append_span(channel.draw_felts(4).span());

        // Assert that all the random felts are unique.
        assert_ne!(random_felts[0], random_felts[5]);
    }

    #[test]
    pub fn test_mix_root() {
        let initial_digest = 0;
        let mut channel = new_channel(initial_digest);

        for _ in 0_usize..10 {
            channel.draw_felt();
        }

        let prev_digest = channel.digest;
        channel.mix_root(0);
        assert_ne!(prev_digest, channel.digest);
    }

    #[test]
    pub fn test_mix_felts() {
        let initial_digest = 0;
        let mut channel = new_channel(initial_digest);

        channel
            .mix_felts(
                array![
                    qm31_const::<1, 2, 3, 4>(), qm31_const::<5, 6, 7, 8>(),
                    qm31_const::<9, 10, 11, 12>(),
                ]
                    .span(),
            );

        assert_ne!(initial_digest, channel.digest);
    }

    #[test]
    pub fn test_mix_u32s() {
        let initial_digest = 0;
        let mut channel = new_channel(initial_digest);

        channel.mix_u32s(array![1, 2, 3, 4, 5, 6, 7, 8, 9].span());

        assert_eq!(
            channel.digest, 0x078f5cf6a2e7362b75fc1f94daeae7ebddd64e6b2db771717519af7193dfa80b,
        );
    }

    #[test]
    pub fn test_draw_random_bytes_1() {
        let initial_digest = 0;
        let mut channel = new_channel(initial_digest);
        let result = channel.draw_random_bytes();
        let expected_result = array![
            197, 20, 139, 143, 49, 135, 207, 202, 93, 167, 20, 244, 184, 186, 20, 136, 204, 43, 46,
            147, 213, 253, 175, 170, 13, 64, 15, 168, 232, 211, 147,
        ];
        assert_eq!(expected_result, result);
    }

    #[test]
    pub fn test_draw_random_bytes_2() {
        let initial_digest = 0xdeadbeef;
        let mut channel = new_channel(initial_digest);
        let result = channel.draw_random_bytes();
        let expected_result = array![
            168, 175, 85, 209, 218, 65, 155, 212, 165, 88, 130, 167, 44, 242, 17, 127, 75, 251, 142,
            180, 157, 176, 27, 167, 179, 247, 27, 113, 149, 41, 12,
        ];
        assert_eq!(expected_result, result);
    }

    #[test]
    pub fn test_draw_random_bytes_3() {
        let initial_digest = 0xcafecafe;
        let mut channel = new_channel(initial_digest);
        let first_result = channel.draw_random_bytes();
        let second_result = channel.draw_random_bytes();
        assert_ne!(first_result, second_result);
    }

    #[test]
    fn test_check_proof_of_work() {
        let digest = 0b1000;

        let res = check_proof_of_work(digest, 3);

        assert!(res);
    }

    #[test]
    fn test_check_proof_of_work_with_invalid_n_bits() {
        let digest = 0b1000;

        let res = check_proof_of_work(digest, 4);

        assert!(!res);
    }

    fn new_channel(digest: felt252) -> Poseidon252Channel {
        Poseidon252Channel { digest, channel_time: Default::default() }
    }
}
