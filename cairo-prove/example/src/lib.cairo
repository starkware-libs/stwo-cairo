use core::blake::{blake2s_compress, blake2s_finalize};

#[executable]
fn main(mut n: felt252) -> [u32; 8] {
    chain_hash(n)
}

pub const BLAKE2S_256_INITIAL_STATE: [u32; 8] = [
    0x6B08E647, 0xBB67AE85, 0x3C6EF372, 0xA54FF53A, 0x510E527F, 0x9B05688C, 0x1F83D9AB, 0x5BE0CD19,
];

//  Given n performs hash_chain on n 0-messages.
fn chain_hash(mut n: felt252) -> [u32; 8] {
    assert!(n != 0);
    let mut state = BoxTrait::new(BLAKE2S_256_INITIAL_STATE);
    let mut byte_count = 0;
    while n != 1 {
        byte_count = byte_count + 64;
        state = blake2s_compress(state, byte_count, BoxTrait::new([0; 16]));
        n = n - 1;
    }
    state = blake2s_finalize(state, byte_count + 64, BoxTrait::new([0; 16]));
    state.unbox()
}
