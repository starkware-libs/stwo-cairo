use crate::SecureField;

pub mod blake2s;
pub mod poseidon252;

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

// Note: Channel is a generic parameter instead of an associated type because implicit self methods
// weren't working e.g. `channel.mix_root()`.
pub trait ChannelTrait<Channel> {
    const BYTES_PER_HASH: usize;

    type Hash;

    fn mix_felts(ref self: Channel, felts: Span<SecureField>);

    fn mix_u64(ref self: Channel, nonce: u64);

    fn draw_felt(ref self: Channel) -> SecureField;

    /// Generates a uniform random vector of SecureField elements.
    fn draw_felts(ref self: Channel, n_felts: usize) -> Array<SecureField>;

    /// Returns a vector of random bytes of length `BYTES_PER_HASH`.
    fn draw_random_bytes(ref self: Channel) -> Array<u8>;

    fn mix_root(ref self: Channel, root: Self::Hash);

    fn check_proof_of_work(self: @Channel, n_bits: u32) -> bool;
}
