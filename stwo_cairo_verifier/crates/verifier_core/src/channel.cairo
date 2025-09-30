use stwo_verifier_utils::MemorySection;
use crate::{Hash, SecureField};

#[cfg(not(feature: "poseidon252_verifier"))]
pub mod blake2s;
#[cfg(not(feature: "poseidon252_verifier"))]
mod feature_dependent_uses {
    pub type Channel = super::blake2s::Blake2sChannel;
    pub use super::blake2s::Blake2sChannelImpl as ChannelImpl;
}


#[cfg(feature: "poseidon252_verifier")]
pub mod poseidon252;
#[cfg(feature: "poseidon252_verifier")]
mod feature_dependent_uses {
    pub type Channel = super::poseidon252::Poseidon252Channel;
    pub use super::poseidon252::Poseidon252ChannelImpl as ChannelImpl;
}

pub use feature_dependent_uses::*;

pub trait ChannelTrait {
    fn mix_felts(ref self: Channel, felts: Span<SecureField>);

    fn mix_u64(ref self: Channel, nonce: u64);

    fn mix_u32s(ref self: Channel, data: Span<u32>);

    /// Mixes the values of a memory section (id-value pairs) into the channel.
    fn mix_memory_section(ref self: Channel, data: MemorySection);

    fn draw_secure_felt(ref self: Channel) -> SecureField;

    /// Generates a uniform random vector of SecureField elements.
    fn draw_secure_felts(ref self: Channel, n_felts: usize) -> Array<SecureField>;

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
