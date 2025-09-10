use bounded_int::M31_SHIFT_NZ_U256;
use bounded_int::impls::*;
use core::array::SpanTrait;
use core::poseidon::{hades_permutation, poseidon_hash_span};
use core::traits::DivRem;
use stwo_verifier_utils::{MemorySection, deconstruct_f252, hash_u32s_with_state};
use crate::SecureField;
use crate::fields::m31::{M31, M31Trait};
use crate::fields::qm31::QM31Trait;
use crate::utils::{pack_qm31, pow2_u64};
use super::ChannelTrait;

#[cfg(test)]
mod test;

/// Number of `M31` per hash.
pub const FELTS_PER_HASH: usize = 8;

pub const BYTES_PER_HASH: usize = 31;

/// A channel with Poseidon252 hash as the non-interactive random oracle.
/// By convention, at the end of every `mix_*` function we reset the number of draws `n_draws`
/// to zero. Every draw of one `felt252` increments `n_draws` by one.
#[derive(Drop, Default)]
pub struct Poseidon252Channel {
    digest: felt252,
    /// Number of consecutive draws since the last value was mixed into the channel.
    n_draws: usize,
}

#[generate_trait]
impl Posidon252ChannelHelperImpl of Poseidon252ChannelHelper {
    /// Helper function to mix a single `felt252` into the channel.
    #[inline(always)]
    fn mix_felt252(ref self: Poseidon252Channel, x: felt252) {
        let (s0, _, _) = hades_permutation(self.digest, x, 2);
        update_digest(ref self, s0);
    }
}

/// Every mix should call `update_digest` as final step, and every draw should
/// increment the `n_draws` counter. In the current implementation, every draw method
/// invokes `draw_secure_felt252` internally, which increments `n_draws` by one.
pub impl Poseidon252ChannelImpl of ChannelTrait {
    fn mix_commitment(ref self: Poseidon252Channel, commitment: felt252) {
        self.mix_felt252(commitment);
    }

    fn mix_felts(ref self: Poseidon252Channel, mut felts: Span<SecureField>) {
        let mut res = array![self.digest];
        while let Some(pair) = felts.multi_pop_front::<2>() {
            let [x, y] = (*pair).unbox();
            // The first argument of `pack_qm31` is 1 so that the felt252 that
            // we will append to `res` can separate the case of a pair (x, y) of QM31
            // with x = 0 from a singleton y (which would enter the other match arm).
            // For any pair, the msb of the resulting felt252 is 2^248, while for
            // any singleton, the msb of the resulting felt252 is 2^124.
            // This also implies that we never overflow the 252-bit prime.
            let cur = pack_qm31(1, x);
            res.append(pack_qm31(cur, y));
        }

        if let Some(x) = felts.pop_front() {
            res.append(pack_qm31(1, *x));
        }

        let next_digest = poseidon_hash_span(res.span());

        update_digest(ref self, next_digest);
    }

    fn mix_u64(ref self: Poseidon252Channel, nonce: u64) {
        self.mix_felt252(nonce.into());
    }

    fn mix_memory_section(ref self: Poseidon252Channel, section: MemorySection) {
        // TODO(Gali): Make this more efficient, use hash_memory_section.
        let mut ids = array![];
        let mut flat_values = array![];
        for entry in section {
            let (id, val) = entry;
            ids.append(*id);
            flat_values.append_span((*val).span());
        }
        let ids_hash = hash_u32s_with_state(self.digest, ids.span());
        let values_hash = hash_u32s_with_state(ids_hash, flat_values.span());

        update_digest(ref self, values_hash);
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

    /// Draws 7 random u32s, constructed from the first 224 bytes
    /// of a random felt252 in little endian.
    fn draw_u32s(ref self: Poseidon252Channel) -> Span<u32> {
        let secure_felt = draw_secure_felt252(ref self).into();
        let x = deconstruct_f252(secure_felt);
        let mut res = x.span();
        // The top limb's 4 most significant bits are zero,
        // hence we discard it since it's not uniformly random.
        let _ = res.pop_back();
        res
    }

    /// Check that `H(H(POW_PREFIX, digest, n_bits), nonce)` has `n_bits` starting zeros.
    fn verify_pow_nonce(self: @Poseidon252Channel, n_bits: u32, nonce: u64) -> bool {
        const POW_PREFIX: u32 = 0x012345678;
        let prefix_hash = poseidon_hash_span(
            [POW_PREFIX.into(), *self.digest, n_bits.into()].span(),
        );
        let (hash, _, _) = hades_permutation(prefix_hash, nonce.into(), 2);
        check_leading_zeros(hash, n_bits)
    }
}

/// Checks that the last `n_bits` bits of the digest are zero.
/// `n_bits` is in range [0, 63].
///
/// # Panics
///
/// Panics if `n_bits` >= 64.
fn check_leading_zeros(digest: felt252, n_bits: u32) -> bool {
    let u256 { low, .. } = digest.into();
    let two_pow_n_bits: u128 = pow2_u64(n_bits).into();
    let nonzero_divisor: NonZero<u128> = two_pow_n_bits.try_into().unwrap();
    let (_, r) = DivRem::div_rem(low, nonzero_divisor);
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

fn draw_secure_felt252(ref channel: Poseidon252Channel) -> felt252 {
    let counter: felt252 = channel.n_draws.into();
    // Use 3 as the capacity for domain separation between mix and draw operations.
    // In all mix functions, the capacity is set to 0 or 2.
    let (res, _, _) = hades_permutation(channel.digest, counter, 3);
    channel.n_draws += 1;
    res
}

#[inline]
fn extract_m31(ref num: u256) -> M31 {
    let (q, r) = DivRem::div_rem(num, M31_SHIFT_NZ_U256);
    num = q;
    M31Trait::reduce_u128(r.low)
}

fn update_digest(ref channel: Poseidon252Channel, new_digest: felt252) {
    channel.digest = new_digest;
    channel.n_draws = 0;
}
