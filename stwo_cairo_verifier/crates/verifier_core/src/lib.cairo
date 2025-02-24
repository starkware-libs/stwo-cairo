pub mod channel;
pub mod circle;
mod circle_mul_table;
pub mod fields;
pub mod fri;
pub mod pcs;
pub mod poly;
mod queries;
pub mod utils;
pub mod vcs;
pub mod verifier;

pub use fields::{BaseField, SecureField};

/// An array in which each element relates (by index) to a column in the trace.
pub type ColumnArray<T> = Array<T>;

/// A span in which each element relates (by index) to a column in the trace.
pub type ColumnSpan<T> = Span<T>;

/// An array in which each element relates (by index) to a commitment tree.
pub type TreeArray<T> = Array<T>;

/// A span in which each element relates (by index) to a commitment tree.
pub type TreeSpan<T> = Span<T>;

trait ChannelTrait {
    type Channel;

    fn draw_felt(ref self: Self::Channel) -> felt252;
}

struct PoseidonChannel {}

impl PoseidonChannelImpl of ChannelTrait {
    type Channel = PoseidonChannel;

    fn draw_felt(ref self: PoseidonChannel) -> felt252 {
        5
    }
}

fn test_fn() {
    let channel = PoseidonChannel {};
    let felt = channel.draw_felt();
    assert_eq!(felt, 5);
}
