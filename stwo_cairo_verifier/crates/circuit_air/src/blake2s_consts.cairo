// Mirrors stwo-circuits/crates/circuit_air/src/blake2s_consts.rs.

pub const BLAKE2S_IV: [u32; 8] = [
    0x6A09E667, 0xBB67AE85, 0x3C6EF372, 0xA54FF53A, 0x510E527F, 0x9B05688C, 0x1F83D9AB, 0x5BE0CD19,
];

pub fn blake2s_initial_state() -> [u32; 8] {
    let [h0, h1, h2, h3, h4, h5, h6, h7] = BLAKE2S_IV;
    [h0 ^ 0x01010020, h1, h2, h3, h4, h5, h6, h7]
}
