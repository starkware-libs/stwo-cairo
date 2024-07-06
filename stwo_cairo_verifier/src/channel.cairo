use core::array::SpanTrait;
use core::poseidon::{poseidon_hash_span, hades_permutation};
use core::traits::DivRem;

use stwo_cairo_verifier::{BaseField, SecureField};
use stwo_cairo_verifier::fields::qm31::QM31Trait;

const M31_IN_HASH_SHIFT: felt252 = 0x80000000; // 2**31.
const M31_IN_HASH_SHIFT_NZ_U256: NonZero<u256> = 0x80000000; // 2**31.
pub const EXTENSION_FELTS_PER_HASH: usize = 2;
pub const FELTS_PER_HASH: usize = 8;

#[derive(Default, Drop, Debug)]
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

#[derive(Drop, Debug)]
pub struct Channel {
    digest: felt252,
    channel_time: ChannelTime,
}

#[generate_trait]
pub impl ChannelImpl of ChannelTrait {
    fn new(digest: felt252) -> Channel {
        Channel { digest: 0, channel_time: Default::default(), }
    }

    fn get_digest(ref self: Channel) -> felt252 {
        self.digest
    }

    fn draw_felt252(ref self: Channel) -> felt252 {
        let (s0, _, _) = hades_permutation(self.digest, self.channel_time.n_sent.into(), 2);
        self.channel_time.inc_sent();
        s0
    }

    #[inline]
    fn draw_base_felts(ref self: Channel) -> [BaseField; FELTS_PER_HASH] {
        let mut cur = self.draw_felt252().into();
        [
            extract_m31(ref cur),
            extract_m31(ref cur),
            extract_m31(ref cur),
            extract_m31(ref cur),
            extract_m31(ref cur),
            extract_m31(ref cur),
            extract_m31(ref cur),
            extract_m31(ref cur),
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
                (Option::None, _) => { break; },
                (
                    Option::Some(x), Option::None
                ) => {
                    let [x0, x1, x2, x3] = (*x).to_array();
                    res
                        .append(
                            ((x0.into() * M31_IN_HASH_SHIFT + x1.into()) * M31_IN_HASH_SHIFT
                                + x2.into())
                                * M31_IN_HASH_SHIFT
                                + x3.into()
                        );
                    break;
                },
                (
                    Option::Some(x), Option::Some(y)
                ) => {
                    let [x0, x1, x2, x3] = (*x).to_array();
                    let [y0, y1, y2, y3] = (*y).to_array();
                    res
                        .append(
                            ((x0.into() * M31_IN_HASH_SHIFT + x1.into()) * M31_IN_HASH_SHIFT
                                + x2.into())
                                * M31_IN_HASH_SHIFT
                                + x3.into()
                        );
                    res
                        .append(
                            ((y0.into() * M31_IN_HASH_SHIFT + y1.into()) * M31_IN_HASH_SHIFT
                                + y2.into())
                                * M31_IN_HASH_SHIFT
                                + y3.into()
                        );
                },
            };
        };

        self.digest = poseidon_hash_span(res.span());

        // TODO(spapini): do we need length padding?
        self.channel_time.inc_challenges();
    }

    fn mix_nonce(ref self: Channel, nonce: u64) {
        self.mix_digest(nonce.into())
    }

    fn draw_felt(ref self: Channel) -> SecureField {
        let [a, b, c, d, _, _, _, _] = self.draw_base_felts();
        QM31Trait::from_array([a, b, c, d])
    }

    fn draw_felts(ref self: Channel, mut n_felts: usize) -> Array<SecureField> {
        let mut res: Array = Default::default();
        loop {
            if n_felts == 0 {
                break;
            }
            let [a, b, c, d, e, f, g, h] = self.draw_base_felts();
            res.append(QM31Trait::from_array([a, b, c, d]));
            if n_felts == 1 {
                break;
            }
            res.append(QM31Trait::from_array([e, f, g, h]));
            n_felts -= 2;
        };
        res
    }
}

#[inline]
fn extract_m31<const N: usize>(ref num: u256) -> BaseField {
    let (q, r) = DivRem::div_rem(num, M31_IN_HASH_SHIFT_NZ_U256);
    num = q;
    let r: u32 = r.try_into().unwrap();
    if r.into() == M31_IN_HASH_SHIFT - 1 {
        BaseField { inner: 0 }
    } else {
        BaseField { inner: r }
    }
}


#[cfg(test)]
mod tests {
    use super::{Channel, ChannelTrait};
    use stwo_cairo_verifier::fields::qm31::qm31;

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

        let mut n: usize = 10;
        while n > 0 {
            n -= 1;
            channel.draw_felt();
        };

        // Reseed channel and check the digest was changed.
        channel.mix_digest(0);
        assert_ne!(initial_digest, channel.digest);
    }

    #[test]
    pub fn test_mix_felts() {
        let initial_digest = 0;
        let mut channel = ChannelTrait::new(initial_digest);

        channel.mix_felts(array![qm31(1, 2, 3, 4), qm31(5, 6, 7, 8), qm31(9, 10, 11, 12)].span());

        assert_ne!(initial_digest, channel.digest);
    }
}
