use core::array::SpanTrait;
use core::poseidon::{hades_permutation, poseidon_hash_span};
use core::traits::DivRem;
use crate::fields::m31::M31Trait;
use crate::fields::qm31::QM31Trait;
use crate::utils::{gen_bit_mask, pack4};
use crate::{BaseField, SecureField};
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

pub impl Poseidon252ChannelImpl of ChannelTrait {
    fn mix_root(ref self: Poseidon252Channel, root: felt252) {
        let (s0, _, _) = hades_permutation(self.digest, root, 2);
        self.digest = s0;
        self.channel_time.inc_challenges();
    }

    fn mix_felts(ref self: Poseidon252Channel, mut felts: Span<SecureField>) {
        let mut res = array![self.digest];
        loop {
            match (felts.pop_front(), felts.pop_front()) {
                (None, _) => { break; },
                (Some(x), None) => {
                    res.append(pack4(0, (*x).to_fixed_array()));
                    break;
                },
                (
                    Some(x), Some(y),
                ) => {
                    let cur = pack4(0, (*x).to_fixed_array());
                    res.append(pack4(cur, (*y).to_fixed_array()));
                },
            };
        }

        self.digest = poseidon_hash_span(res.span());

        // TODO(spapini): do we need length padding?
        self.channel_time.inc_challenges();
    }

    fn mix_u64(ref self: Poseidon252Channel, nonce: u64) {
        self.mix_root(nonce.into())
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
    /// TODO: check that this distribution is good enough, as it is only close to uniform.
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
    low & gen_bit_mask(n_bits) == 0
}

// TODO(spapini): Check that this is sound.
fn draw_base_felts(ref channel: Poseidon252Channel) -> [M31; FELTS_PER_HASH] {
    let mut cur = draw_felt252(ref channel).into();
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
fn extract_m31<const N: usize>(ref num: u256) -> M31 {
    let (q, r) = DivRem::div_rem(num, M31_SHIFT_NZ_U256);
    num = q;
    M31Trait::reduce_u128(r.low)
}
