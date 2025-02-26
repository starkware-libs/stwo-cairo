use crate::{Hash, SecureField};

#[cfg(feature: "blake2s_verifier")]
pub mod blake2s;
#[cfg(not(feature: "blake2s_verifier"))]
pub mod poseidon252;

#[cfg(feature: "blake2s_verifier")]
pub type Channel = blake2s::Blake2sChannel;
#[cfg(not(feature: "blake2s_verifier"))]
pub type Channel = poseidon252::Poseidon252Channel;
#[cfg(feature: "blake2s_verifier")]
pub use blake2s::Blake2sChannelImpl as ChannelImpl;
#[cfg(not(feature: "blake2s_verifier"))]
pub use poseidon252::Poseidon252ChannelImpl as ChannelImpl;

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

    fn draw_felt(ref self: Channel) -> SecureField;

    /// Generates a uniform random vector of SecureField elements.
    fn draw_felts(ref self: Channel, n_felts: usize) -> Array<SecureField>;

    /// Returns a vector of random bytes of length `BYTES_PER_HASH`.
    fn draw_random_bytes(ref self: Channel) -> Array<u8>;

    fn mix_root(ref self: Channel, root: Hash);

    fn check_proof_of_work(self: @Channel, n_bits: u32) -> bool;
}
