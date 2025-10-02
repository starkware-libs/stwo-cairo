pub const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];
pub const N_SHA256_ROUNDS: usize = 64;

pub mod small_sigma0 {
    pub const N: [u32; 3] = [7, 18, 3];
    pub const L0_INDEXES: [u8; 7] = [1, 3, 5, 7, 9, 11, 14];
    pub const L1_INDEXES: [u8; 4] = [0, 2, 4, 6];
    pub const L2_INDEXES: [u8; 5] = [8, 10, 12, 13, 15];
    pub const H0_INDEXES: [u8; 4] = [16, 18, 20, 22];
    pub const H1_INDEXES: [u8; 5] = [24, 26, 28, 29, 31];
    pub const H2_INDEXES: [u8; 7] = [17, 19, 21, 23, 25, 27, 30];

    pub const O0L: u16 = 0b0101000000101010;
    pub const O0H: u16 = 0b1010100000010101;
    pub const O1L: u16 = 0b1010100000010101;
    pub const O1H: u16 = 0b0101000000101010;
    pub const O2L: u16 = 0b0000011111000000;
    pub const O2H: u16 = 0b0000011111000000;
}
pub mod small_sigma1 {
    pub const N: [u32; 3] = [17, 19, 10];
    pub const L0_INDEXES: [u8; 5] = [0, 2, 7, 9, 14];
    pub const L1_INDEXES: [u8; 5] = [1, 3, 4, 5, 6];
    pub const L2_INDEXES: [u8; 6] = [8, 10, 11, 12, 13, 15];
    pub const H0_INDEXES: [u8; 7] = [16, 18, 21, 23, 25, 27, 30];
    pub const H1_INDEXES: [u8; 5] = [17, 19, 20, 22, 24];
    pub const H2_INDEXES: [u8; 4] = [26, 28, 29, 31];

    pub const O0L: u16 = 0b1010000101010000;
    pub const O0H: u16 = 0b1010000001000000;
    pub const O1L: u16 = 0b0101001010101001;
    pub const O1H: u16 = 0b0101011010101101;
    pub const O2L: u16 = 0b0000110000000110;
    pub const O2H: u16 = 0b0000100100010010;
}
pub mod big_sigma0 {
    pub const N: [u32; 3] = [2, 13, 22];
    pub const L0_INDEXES: [u8; 7] = [0, 1, 7, 8, 9, 10, 11];
    pub const L1_INDEXES: [u8; 5] = [2, 3, 4, 5, 6];
    pub const L2_INDEXES: [u8; 4] = [12, 13, 14, 15];
    pub const H0_INDEXES: [u8; 5] = [18, 19, 20, 21, 22];
    pub const H1_INDEXES: [u8; 4] = [28, 29, 30, 31];
    pub const H2_INDEXES: [u8; 7] = [16, 17, 23, 24, 25, 26, 27];

    pub const O0L: u16 = 0b0000_0011_1100_0000;
    pub const O0H: u16 = 0b0111_0000_0001_1110;
    pub const O1L: u16 = 0b0111_0000_0001_1110;
    pub const O1H: u16 = 0b0000_0011_1100_0000;
    pub const O2L: u16 = 0b1000_1100_0010_0001;
    pub const O2H: u16 = 0b1000_1100_0010_0001;
}
pub mod big_sigma1 {
    pub const N: [u32; 3] = [6, 11, 25];
    pub const L0_INDEXES: [u8; 7] = [1, 4, 7, 9, 12, 13, 14];
    pub const L1_INDEXES: [u8; 5] = [0, 2, 3, 5, 6];
    pub const L2_INDEXES: [u8; 4] = [8, 10, 11, 15];
    pub const H0_INDEXES: [u8; 5] = [17, 18, 20, 22, 23];
    pub const H1_INDEXES: [u8; 4] = [26, 27, 28, 31];
    pub const H2_INDEXES: [u8; 7] = [16, 19, 21, 24, 25, 29, 30];

    pub const O0L: u16 = 0b0010_0100_0011_0001;
    pub const O0H: u16 = 0b1000_0100_1000_0100;
    pub const O1L: u16 = 0b0000_1000_0100_1010;
    pub const O1H: u16 = 0b0100_0010_0001_0001;
    pub const O2L: u16 = 0b1101_0011_1000_0100;
    pub const O2H: u16 = 0b0011_1001_0110_1010;
}
