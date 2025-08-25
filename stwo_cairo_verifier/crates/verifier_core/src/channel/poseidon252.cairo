use bounded_int::impls::*;
use bounded_int::{M31_SHIFT_NZ_U256, NZ_U8_SHIFT, div_rem, upcast};
use core::array::SpanTrait;
use core::poseidon::{hades_permutation, poseidon_hash_span};
use core::traits::DivRem;
use stwo_verifier_utils::MemorySection;
use crate::SecureField;
use crate::fields::m31::{M31, M31Trait};
use crate::fields::qm31::QM31Trait;
use crate::utils::{pack4, pow2_u64};
use super::{ChannelTime, ChannelTimeImpl, ChannelTrait};

#[cfg(test)]
mod test;

/// Number of `M31` per hash.
pub const FELTS_PER_HASH: usize = 8;

pub const BYTES_PER_HASH: usize = 31;
// consider move to utils.
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
        // Add update_digest in this file. (maybe in a trait, and use the trait also in blake2s).
        self.digest = s0;
        self.channel_time.next_challenges();
    }
}
pub impl Poseidon252ChannelImpl of ChannelTrait {
    fn mix_root(ref self: Poseidon252Channel, root: felt252) {
        self.mix_felt252(root);
    }

    fn mix_felts(ref self: Poseidon252Channel, mut felts: Span<SecureField>) {
        let mut res = array![self.digest];
        loop {
            match felts.multi_pop_front::<2>() {
                Option::Some(pair) => {
                    let [x, y] = (*pair).unbox();
                    // Need to doc why the cur starts with 1. It seperates the cases of 1 qm31 and 2 qm31. How do you differentiate between two qm31s that are 0 and one qm31 that is 0?  so if you add 1, you get 100000000 vs 000010000.

                    // Gil: pack4 can be in the interface of qm31 without using to_fix_array.
                    let cur = pack4(cur: 1, values: x.to_fixed_array());
                    res.append(pack4(:cur, values: y.to_fixed_array()));
                },
                Option::None => {
                    if !felts.is_empty() {
                        let x = felts.pop_front().unwrap();
                        res.append(pack4(1, (*x).to_fixed_array()));
                    }
                    break;
                },
            };
        }

        self.digest = poseidon_hash_span(res.span());

        // That cur = 1 is probably enough, but confirm and remove TODO.
        // TODO(spapini): do we need length padding?
        self.channel_time.next_challenges();
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
            // Pad number of limbs, might not be sound.
            chunk.append_span(data);
            for _ in data.len()..7 {
                chunk.append(0);
            }
            res.append(construct_f252_be(*chunk.span().try_into().unwrap()));
        }

        self.digest = poseidon_hash_span(res.span());

        self.channel_time.next_challenges();
    }
    // rename to section instead of data. GALI
    fn mix_memory_section(ref self: Poseidon252Channel, data: MemorySection) {
        // TODO(Gali): Do the TODO.

        // TODO(Gil): Make sure Gali does her TODO.

        // TODO(Gali): Make this more efficient, use hash_memory_section.
        // Mix the id here, and EVERYWHERE.
        let mut flat_data = array![];
        for entry in data {
            let (_, val) = entry;
            flat_data.append_span((*val).span());
        }
        self.mix_u32s(flat_data.span());
    }

    fn draw_secure_felt(ref self: Poseidon252Channel) -> SecureField {
        let [r0, r1, r2, r3, _, _, _, _] = draw_base_felts(ref self);
        QM31Trait::from_fixed_array([r0, r1, r2, r3])
    }

    fn draw_secure_felts(ref self: Poseidon252Channel, mut n_felts: usize) -> Array<SecureField> {
        let mut res: Array = Default::default();

        while n_felts != 0 {
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


    // Consider removing and implementing draw_queries(queries_num, log_domain_size). ILYA.
    /// Returns 31 random bytes computed as the first 31 bytes of the representative of
    /// `self.draw_secure_felt252()` in little endian.
    /// The distribution for each byte is epsilon close to uniform with epsilon bounded by 2^(-60).
    fn draw_random_bytes(ref self: Poseidon252Channel) -> Array<u8> {
        let u256 { low, high } = draw_secure_felt252(ref self).into();

        let mut bytes = array![];
        // Doc it's manual implementation for efficiency.
        // Extract the 16 bytes from the low 128 bits of the felt 252.
        let (q, r) = div_rem(low, NZ_U8_SHIFT);
        bytes.append(upcast(r));
        let (q, r) = div_rem(q, NZ_U8_SHIFT);
        bytes.append(upcast(r));
        let (q, r) = div_rem(q, NZ_U8_SHIFT);
        bytes.append(upcast(r));
        let (q, r) = div_rem(q, NZ_U8_SHIFT);
        bytes.append(upcast(r));
        let (q, r) = div_rem(q, NZ_U8_SHIFT);
        bytes.append(upcast(r));
        let (q, r) = div_rem(q, NZ_U8_SHIFT);
        bytes.append(upcast(r));
        let (q, r) = div_rem(q, NZ_U8_SHIFT);
        bytes.append(upcast(r));
        let (q, r) = div_rem(q, NZ_U8_SHIFT);
        bytes.append(upcast(r));
        let (q, r) = div_rem(q, NZ_U8_SHIFT);
        bytes.append(upcast(r));
        let (q, r) = div_rem(q, NZ_U8_SHIFT);
        bytes.append(upcast(r));
        let (q, r) = div_rem(q, NZ_U8_SHIFT);
        bytes.append(upcast(r));
        let (q, r) = div_rem(q, NZ_U8_SHIFT);
        bytes.append(upcast(r));
        let (q, r) = div_rem(q, NZ_U8_SHIFT);
        bytes.append(upcast(r));
        let (q, r) = div_rem(q, NZ_U8_SHIFT);
        bytes.append(upcast(r));
        let (q, r) = div_rem(q, NZ_U8_SHIFT);
        bytes.append(upcast(r));
        bytes.append(upcast(q));

        // Extract the first 15 bytes from the high 128 bits of the felt 252.
        let (q, r) = div_rem(high, NZ_U8_SHIFT);
        bytes.append(upcast(r));
        let (q, r) = div_rem(q, NZ_U8_SHIFT);
        bytes.append(upcast(r));
        let (q, r) = div_rem(q, NZ_U8_SHIFT);
        bytes.append(upcast(r));
        let (q, r) = div_rem(q, NZ_U8_SHIFT);
        bytes.append(upcast(r));
        let (q, r) = div_rem(q, NZ_U8_SHIFT);
        bytes.append(upcast(r));
        let (q, r) = div_rem(q, NZ_U8_SHIFT);
        bytes.append(upcast(r));
        let (q, r) = div_rem(q, NZ_U8_SHIFT);
        bytes.append(upcast(r));
        let (q, r) = div_rem(q, NZ_U8_SHIFT);
        bytes.append(upcast(r));
        let (q, r) = div_rem(q, NZ_U8_SHIFT);
        bytes.append(upcast(r));
        let (q, r) = div_rem(q, NZ_U8_SHIFT);
        bytes.append(upcast(r));
        let (q, r) = div_rem(q, NZ_U8_SHIFT);
        bytes.append(upcast(r));
        let (q, r) = div_rem(q, NZ_U8_SHIFT);
        bytes.append(upcast(r));
        let (q, r) = div_rem(q, NZ_U8_SHIFT);
        bytes.append(upcast(r));
        let (q, r) = div_rem(q, NZ_U8_SHIFT);
        bytes.append(upcast(r));
        let (_, r) = div_rem(q, NZ_U8_SHIFT);
        bytes.append(upcast(r));

        bytes
    }

    fn mix_and_check_pow_nonce(ref self: Poseidon252Channel, n_bits: u32, nonce: u64) -> bool {
        // TODO(andrew): Use blake for proof of work.
        self.mix_u64(nonce);
        check_proof_of_work(self.digest, n_bits)
    }
}

// Remove support for 64 bit pow (also in blake).

/// Checks that the last `n_bits` bits of the digest are zero.
/// `n_bits` is in range [0, 64].
fn check_proof_of_work(digest: felt252, n_bits: u32) -> bool {
    let u256 { low, .. } = digest.into();
    let divisor = if n_bits == 64 {
        // Handle the special case of 64 bits.
        0x10000000000000000 // 2^64
    } else {
        // If n_bits > 64 pow2_u64 will panic with index out of bounds.
        let u128_2_pow_n_bits: u128 = pow2_u64(n_bits).into();
        u128_2_pow_n_bits.try_into().unwrap()
    };
    let (_, r) = DivRem::div_rem(low, divisor);
    r == 0
}

// TODO(spapini): Check that this is sound.
fn draw_base_felts(ref channel: Poseidon252Channel) -> [M31; FELTS_PER_HASH] {
    let mut cur: u256 = draw_secure_felt252(ref channel).into();
    [
        extract_m31(ref cur), extract_m31(ref cur), extract_m31(ref cur), extract_m31(ref cur),
        extract_m31(ref cur), extract_m31(ref cur), extract_m31(ref cur), extract_m31(ref cur),
    ]
}

// Rename to draw_f252.
fn draw_secure_felt252(ref channel: Poseidon252Channel) -> felt252 {
    let (res, _, _) = hades_permutation(channel.digest, channel.channel_time.n_sent.into(), 2);
    channel.channel_time.inc_sent();
    res
}

// Maybe split the extract u31 and reduce, so you can check all u31s are < P before reduce.
// Also remove spapini todo.

// Consider using bounded int div_rem.
#[inline]
fn extract_m31(ref num: u256) -> M31 {
    let (q, r) = DivRem::div_rem(num, M31_SHIFT_NZ_U256);
    num = q;
    M31Trait::reduce_u128(r.low)
}
