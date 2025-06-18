use crate::fields::m31::M31;
use crate::fields::qm31::QM31Trait;
use crate::{Hash, SecureField};

#[cfg(not(feature: "poseidon252_verifier"))]
pub mod blake2s;
#[cfg(test)]
#[cfg(not(feature: "poseidon252_verifier"))]
mod blake2s_test;

#[cfg(feature: "poseidon252_verifier")]
pub mod poseidon252;

#[cfg(not(feature: "poseidon252_verifier"))]
pub type Channel = blake2s::Blake2sChannel;
#[cfg(feature: "poseidon252_verifier")]
pub type Channel = poseidon252::Poseidon252Channel;
#[cfg(not(feature: "poseidon252_verifier"))]
pub use blake2s::Blake2sChannelImpl as ChannelImpl;
#[cfg(feature: "poseidon252_verifier")]
pub use poseidon252::Poseidon252ChannelImpl as ChannelImpl;

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

pub trait ChannelTrait {
    fn mix_felts(ref self: Channel, felts: Span<SecureField>);

    fn mix_u64(ref self: Channel, nonce: u64);

    fn mix_u32s(ref self: Channel, data: Span<u32>);

    fn draw_random_base_felts(ref self: Channel) -> [M31; FELTS_PER_HASH];

    fn draw_felt(
        ref self: Channel,
    ) -> SecureField {
        let [r0, r1, r2, r3, _, _, _, _] = self.draw_random_base_felts();
        QM31Trait::from_fixed_array([r0, r1, r2, r3])
    }

    /// Generates a uniform random vector of SecureField elements.
    fn draw_felts(
        ref self: Channel, n_felts: usize,
    ) -> Array<
        SecureField,
    > {
        let mut res = array![];
        let mut n_felts = n_felts;

        while n_felts != 0 {
            let [r0, r1, r2, r3, r4, r5, r6, r7] = self.draw_random_base_felts();
            res.append(QM31Trait::from_fixed_array([r0, r1, r2, r3]));
            if n_felts == 1 {
                break;
            }
            res.append(QM31Trait::from_fixed_array([r4, r5, r6, r7]));
            n_felts -= 2;
        }

        res
    }

    /// Returns a vector of random bytes of length `BYTES_PER_HASH`.
    fn draw_random_bytes(ref self: Channel) -> Array<u8>;

    fn mix_root(ref self: Channel, root: Hash);

    /// Mixes a nonce into the channel and checks a proof-of-work (PoW) on it.
    /// Returns false if the nonce fails to meet the PoW requirement.
    ///
    /// `n_bits` is the number of leading zero bits that must be in the target
    /// PoW hash. `nonce` is the value used to generate the target PoW hash
    /// and is also mixed into the channel.
    ///
    /// This interface allows the channel to use different hash function for the
    /// PoW than the one used by the channel.
    fn mix_and_check_pow_nonce(ref self: Channel, n_bits: u32, nonce: u64) -> bool;
}
