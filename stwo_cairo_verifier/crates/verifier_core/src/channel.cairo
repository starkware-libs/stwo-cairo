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

/// An interface for performing the Fiat-Shamir transformation on an interactive protocol.
/// The methods `mix_*` take a part of the protocol transcript and mix it into the state
/// of the pseudo-random oracle, so that subsequent queries are dependent on the transcript.
/// The methods `draw_*` simulate the random queries of the verifier.
pub trait ChannelTrait {
    fn mix_felts(ref self: Channel, felts: Span<SecureField>);

    fn mix_u64(ref self: Channel, nonce: u64);

    /// Mixes a memory section (id-value pairs) into the channel.
    /// All the ids are mixed first, then all the values, each of them in the order it appears in
    /// the section.
    fn mix_memory_section(ref self: Channel, section: MemorySection);

    /// Mixes a commitment (typically the root of a Merkle tree) into the channel.
    fn mix_commitment(ref self: Channel, commitment: Hash);

    fn draw_secure_felt(ref self: Channel) -> SecureField;

    /// Generates a uniform random vector of SecureField elements.
    fn draw_secure_felts(ref self: Channel, n_felts: usize) -> Array<SecureField>;

    fn draw_u32s(ref self: Channel) -> Span<u32>;

    /// Verifies the proof of work nonce.
    ///
    /// `n_bits` is the number of leading zero bits that must be in the target
    /// PoW hash. `nonce` is the value used to generate the target PoW hash
    /// and is also mixed into the channel.
    ///
    /// This interface allows the channel to use different hash function for the
    /// PoW than the one used by the channel.
    fn verify_pow_nonce(self: @Channel, n_bits: u32, nonce: u64) -> bool;
}
