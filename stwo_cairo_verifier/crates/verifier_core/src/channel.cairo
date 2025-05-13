use crate::{Hash, SecureField};

pub mod poseidon252;

pub type Channel = poseidon252::Poseidon252Channel;
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

    fn mix_u32s(ref self: Channel, data: Span<u32>);

    fn draw_felt(ref self: Channel) -> SecureField;

    /// Generates a uniform random vector of SecureField elements.
    fn draw_felts(ref self: Channel, n_felts: usize) -> Array<SecureField>;

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
    #[allow(clippy::doc_markdown)]
    fn mix_and_check_pow_nonce(ref self: Channel, n_bits: u32, nonce: u64) -> bool;
}
