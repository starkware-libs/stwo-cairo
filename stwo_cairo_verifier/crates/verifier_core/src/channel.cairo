use crate::{Hash, SecureField};

#[cfg(not(feature: "poseidon252_verifier"))]
pub mod blake2s;
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

// document fields
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

    // rename e.g. next_challenge
    fn inc_challenges(ref self: ChannelTime) {
        // is n_challenges ever used?
        self.n_challenges += 1;
        // why set to zero?
        self.n_sent = 0;
    }
}

pub trait ChannelTrait {
    fn mix_felts(ref self: Channel, felts: Span<SecureField>);

    fn mix_u64(ref self: Channel, nonce: u64);

    fn mix_root(ref self: Channel, root: Hash);

    // rename e.g. draw_secure_felt/draw_qm31
    fn draw_felt(ref self: Channel) -> SecureField;

    /// Generates a uniform random vector of SecureField elements.
    // rename e.g. draw_secure_felts/draw_qm31s
    fn draw_felts(ref self: Channel, n_felts: usize) -> Array<SecureField>;

    /// Returns a vector of random bytes of length `BYTES_PER_HASH`.
    fn draw_random_bytes(ref self: Channel) -> Array<u8>;

    // TODO: Consider adding nonce to this function so channel could use different hash function.
    // document please
    fn check_proof_of_work(self: @Channel, n_bits: u32) -> bool;
}
