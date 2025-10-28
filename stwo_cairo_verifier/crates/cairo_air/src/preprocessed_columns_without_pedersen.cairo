use stwo_constraint_framework::PreprocessedColumnIdx;
use stwo_verifier_core::Hash;

// IMPORTANT: This file must exactly match the output and ordering of the prover preprocessed
// trace declaration. If the function changes, this array must be updated to stay in sync.
// https://github.com/starkware-libs/stwo-cairo/blob/d8b2aff6ac7c4e991b9e91140ff20d6e0d9ea0b3/stwo_cairo_prover/crates/common/src/preprocessed_columns/preprocessed_trace.rs#L43

pub const NUM_PREPROCESSED_COLUMNS: u32 = 107;

pub const PREPROCESSED_COLUMN_LOG_SIZE: [u32; NUM_PREPROCESSED_COLUMNS] = [
    25, // Seq(25)
    24, // Seq(24)
    23, // Seq(23)
    22, // Seq(22)
    21, // Seq(21)
    20, // Seq(20)
    20, // BitwiseXor((10, 0))
    20, // BitwiseXor((10, 1))
    20, // BitwiseXor((10, 2))
    19, // Seq(19)
    18, // Seq(18)
    18, // BitwiseXor((9, 0))
    18, // BitwiseXor((9, 1))
    18, // BitwiseXor((9, 2))
    18, // RangeCheck2(([9, 9], 0))
    18, // RangeCheck2(([9, 9], 1))
    18, // RangeCheck4(([3, 6, 6, 3], 0))
    18, // RangeCheck4(([3, 6, 6, 3], 1))
    18, // RangeCheck4(([3, 6, 6, 3], 2))
    18, // RangeCheck4(([3, 6, 6, 3], 3))
    17, // Seq(17)
    16, // Seq(16)
    16, // BitwiseXor((8, 0))
    16, // BitwiseXor((8, 1))
    16, // BitwiseXor((8, 2))
    16, // RangeCheck4(([4, 4, 4, 4], 0))
    16, // RangeCheck4(([4, 4, 4, 4], 1))
    16, // RangeCheck4(([4, 4, 4, 4], 2))
    16, // RangeCheck4(([4, 4, 4, 4], 3))
    15, // Seq(15)
    15, // RangeCheck5(([3, 3, 3, 3, 3], 0))
    15, // RangeCheck5(([3, 3, 3, 3, 3], 1))
    15, // RangeCheck5(([3, 3, 3, 3, 3], 2))
    15, // RangeCheck5(([3, 3, 3, 3, 3], 3))
    15, // RangeCheck5(([3, 3, 3, 3, 3], 4))
    14, // Seq(14)
    14, // BitwiseXor((7, 0))
    14, // BitwiseXor((7, 1))
    14, // BitwiseXor((7, 2))
    14, // RangeCheck3(([7, 2, 5], 0))
    14, // RangeCheck3(([7, 2, 5], 1))
    14, // RangeCheck3(([7, 2, 5], 2))
    13, // Seq(13)
    12, // Seq(12)
    11, // Seq(11)
    10, // Seq(10)
    9, // Seq(9)
    9, // RangeCheck2(([5, 4], 0))
    9, // RangeCheck2(([5, 4], 1))
    8, // Seq(8)
    8, // BitwiseXor((4, 0))
    8, // BitwiseXor((4, 1))
    8, // BitwiseXor((4, 2))
    8, // RangeCheck2(([4, 4], 0))
    8, // RangeCheck2(([4, 4], 1))
    7, // Seq(7)
    7, // RangeCheck2(([4, 3], 0))
    7, // RangeCheck2(([4, 3], 1))
    6, // Seq(6)
    6, // PoseidonRoundKeys(0)
    6, // PoseidonRoundKeys(1)
    6, // PoseidonRoundKeys(2)
    6, // PoseidonRoundKeys(3)
    6, // PoseidonRoundKeys(4)
    6, // PoseidonRoundKeys(5)
    6, // PoseidonRoundKeys(6)
    6, // PoseidonRoundKeys(7)
    6, // PoseidonRoundKeys(8)
    6, // PoseidonRoundKeys(9)
    6, // PoseidonRoundKeys(10)
    6, // PoseidonRoundKeys(11)
    6, // PoseidonRoundKeys(12)
    6, // PoseidonRoundKeys(13)
    6, // PoseidonRoundKeys(14)
    6, // PoseidonRoundKeys(15)
    6, // PoseidonRoundKeys(16)
    6, // PoseidonRoundKeys(17)
    6, // PoseidonRoundKeys(18)
    6, // PoseidonRoundKeys(19)
    6, // PoseidonRoundKeys(20)
    6, // PoseidonRoundKeys(21)
    6, // PoseidonRoundKeys(22)
    6, // PoseidonRoundKeys(23)
    6, // PoseidonRoundKeys(24)
    6, // PoseidonRoundKeys(25)
    6, // PoseidonRoundKeys(26)
    6, // PoseidonRoundKeys(27)
    6, // PoseidonRoundKeys(28)
    6, // PoseidonRoundKeys(29)
    5, // Seq(5)
    4, // Seq(4)
    4, // BlakeSigma(0)
    4, // BlakeSigma(1)
    4, // BlakeSigma(2)
    4, // BlakeSigma(3)
    4, // BlakeSigma(4)
    4, // BlakeSigma(5)
    4, // BlakeSigma(6)
    4, // BlakeSigma(7)
    4, // BlakeSigma(8)
    4, // BlakeSigma(9)
    4, // BlakeSigma(10)
    4, // BlakeSigma(11)
    4, // BlakeSigma(12)
    4, // BlakeSigma(13)
    4, // BlakeSigma(14)
    4 // BlakeSigma(15)
];


pub const SEQ__25_0_IDX: u32 = 0; // Seq(25)
pub const SEQ__24_0_IDX: u32 = 1; // Seq(24)
pub const SEQ__23_0_IDX: u32 = 2; // Seq(23)
pub const SEQ__22_0_IDX: u32 = 3; // Seq(22)
pub const SEQ__21_0_IDX: u32 = 4; // Seq(21)
pub const SEQ__20_0_IDX: u32 = 5; // Seq(20)
pub const BITWISE_XOR__10_0_IDX: u32 = 6; // BitwiseXor((10, 0))
pub const BITWISE_XOR__10_1_IDX: u32 = 7; // BitwiseXor((10, 1))
pub const BITWISE_XOR__10_2_IDX: u32 = 8; // BitwiseXor((10, 2))
pub const SEQ__19_0_IDX: u32 = 9; // Seq(19)
pub const SEQ__18_0_IDX: u32 = 10; // Seq(18)
pub const BITWISE_XOR__9_0_IDX: u32 = 11; // BitwiseXor((9, 0))
pub const BITWISE_XOR__9_1_IDX: u32 = 12; // BitwiseXor((9, 1))
pub const BITWISE_XOR__9_2_IDX: u32 = 13; // BitwiseXor((9, 2))
pub const RANGE_CHECK_2_9_9_0_IDX: u32 = 14; // RangeCheck2(([9, 9], 0))
pub const RANGE_CHECK_2_9_9_1_IDX: u32 = 15; // RangeCheck2(([9, 9], 1))
pub const RANGE_CHECK_4_3_6_6_3_0_IDX: u32 = 16; // RangeCheck4(([3, 6, 6, 3], 0))
pub const RANGE_CHECK_4_3_6_6_3_1_IDX: u32 = 17; // RangeCheck4(([3, 6, 6, 3], 1))
pub const RANGE_CHECK_4_3_6_6_3_2_IDX: u32 = 18; // RangeCheck4(([3, 6, 6, 3], 2))
pub const RANGE_CHECK_4_3_6_6_3_3_IDX: u32 = 19; // RangeCheck4(([3, 6, 6, 3], 3))
pub const SEQ__17_0_IDX: u32 = 20; // Seq(17)
pub const SEQ__16_0_IDX: u32 = 21; // Seq(16)
pub const BITWISE_XOR__8_0_IDX: u32 = 22; // BitwiseXor((8, 0))
pub const BITWISE_XOR__8_1_IDX: u32 = 23; // BitwiseXor((8, 1))
pub const BITWISE_XOR__8_2_IDX: u32 = 24; // BitwiseXor((8, 2))
pub const RANGE_CHECK_4_4_4_4_4_0_IDX: u32 = 25; // RangeCheck4(([4, 4, 4, 4], 0))
pub const RANGE_CHECK_4_4_4_4_4_1_IDX: u32 = 26; // RangeCheck4(([4, 4, 4, 4], 1))
pub const RANGE_CHECK_4_4_4_4_4_2_IDX: u32 = 27; // RangeCheck4(([4, 4, 4, 4], 2))
pub const RANGE_CHECK_4_4_4_4_4_3_IDX: u32 = 28; // RangeCheck4(([4, 4, 4, 4], 3))
pub const SEQ__15_0_IDX: u32 = 29; // Seq(15)
pub const RANGE_CHECK_5_3_3_3_3_3_0_IDX: u32 = 30; // RangeCheck5(([3, 3, 3, 3, 3], 0))
pub const RANGE_CHECK_5_3_3_3_3_3_1_IDX: u32 = 31; // RangeCheck5(([3, 3, 3, 3, 3], 1))
pub const RANGE_CHECK_5_3_3_3_3_3_2_IDX: u32 = 32; // RangeCheck5(([3, 3, 3, 3, 3], 2))
pub const RANGE_CHECK_5_3_3_3_3_3_3_IDX: u32 = 33; // RangeCheck5(([3, 3, 3, 3, 3], 3))
pub const RANGE_CHECK_5_3_3_3_3_3_4_IDX: u32 = 34; // RangeCheck5(([3, 3, 3, 3, 3], 4))
pub const SEQ__14_0_IDX: u32 = 35; // Seq(14)
pub const BITWISE_XOR__7_0_IDX: u32 = 36; // BitwiseXor((7, 0))
pub const BITWISE_XOR__7_1_IDX: u32 = 37; // BitwiseXor((7, 1))
pub const BITWISE_XOR__7_2_IDX: u32 = 38; // BitwiseXor((7, 2))
pub const RANGE_CHECK_3_7_2_5_0_IDX: u32 = 39; // RangeCheck3(([7, 2, 5], 0))
pub const RANGE_CHECK_3_7_2_5_1_IDX: u32 = 40; // RangeCheck3(([7, 2, 5], 1))
pub const RANGE_CHECK_3_7_2_5_2_IDX: u32 = 41; // RangeCheck3(([7, 2, 5], 2))
pub const SEQ__13_0_IDX: u32 = 42; // Seq(13)
pub const SEQ__12_0_IDX: u32 = 43; // Seq(12)
pub const SEQ__11_0_IDX: u32 = 44; // Seq(11)
pub const SEQ__10_0_IDX: u32 = 45; // Seq(10)
pub const SEQ__9_0_IDX: u32 = 46; // Seq(9)
pub const RANGE_CHECK_2_5_4_0_IDX: u32 = 47; // RangeCheck2(([5, 4], 0))
pub const RANGE_CHECK_2_5_4_1_IDX: u32 = 48; // RangeCheck2(([5, 4], 1))
pub const SEQ__8_0_IDX: u32 = 49; // Seq(8)
pub const BITWISE_XOR__4_0_IDX: u32 = 50; // BitwiseXor((4, 0))
pub const BITWISE_XOR__4_1_IDX: u32 = 51; // BitwiseXor((4, 1))
pub const BITWISE_XOR__4_2_IDX: u32 = 52; // BitwiseXor((4, 2))
pub const RANGE_CHECK_2_4_4_0_IDX: u32 = 53; // RangeCheck2(([4, 4], 0))
pub const RANGE_CHECK_2_4_4_1_IDX: u32 = 54; // RangeCheck2(([4, 4], 1))
pub const SEQ__7_0_IDX: u32 = 55; // Seq(7)
pub const RANGE_CHECK_2_4_3_0_IDX: u32 = 56; // RangeCheck2(([4, 3], 0))
pub const RANGE_CHECK_2_4_3_1_IDX: u32 = 57; // RangeCheck2(([4, 3], 1))
pub const SEQ__6_0_IDX: u32 = 58; // Seq(6)
pub const POSEIDON_ROUND_KEYS__0_IDX: u32 = 59; // PoseidonRoundKeys(0)
pub const POSEIDON_ROUND_KEYS__1_IDX: u32 = 60; // PoseidonRoundKeys(1)
pub const POSEIDON_ROUND_KEYS__2_IDX: u32 = 61; // PoseidonRoundKeys(2)
pub const POSEIDON_ROUND_KEYS__3_IDX: u32 = 62; // PoseidonRoundKeys(3)
pub const POSEIDON_ROUND_KEYS__4_IDX: u32 = 63; // PoseidonRoundKeys(4)
pub const POSEIDON_ROUND_KEYS__5_IDX: u32 = 64; // PoseidonRoundKeys(5)
pub const POSEIDON_ROUND_KEYS__6_IDX: u32 = 65; // PoseidonRoundKeys(6)
pub const POSEIDON_ROUND_KEYS__7_IDX: u32 = 66; // PoseidonRoundKeys(7)
pub const POSEIDON_ROUND_KEYS__8_IDX: u32 = 67; // PoseidonRoundKeys(8)
pub const POSEIDON_ROUND_KEYS__9_IDX: u32 = 68; // PoseidonRoundKeys(9)
pub const POSEIDON_ROUND_KEYS__10_IDX: u32 = 69; // PoseidonRoundKeys(10)
pub const POSEIDON_ROUND_KEYS__11_IDX: u32 = 70; // PoseidonRoundKeys(11)
pub const POSEIDON_ROUND_KEYS__12_IDX: u32 = 71; // PoseidonRoundKeys(12)
pub const POSEIDON_ROUND_KEYS__13_IDX: u32 = 72; // PoseidonRoundKeys(13)
pub const POSEIDON_ROUND_KEYS__14_IDX: u32 = 73; // PoseidonRoundKeys(14)
pub const POSEIDON_ROUND_KEYS__15_IDX: u32 = 74; // PoseidonRoundKeys(15)
pub const POSEIDON_ROUND_KEYS__16_IDX: u32 = 75; // PoseidonRoundKeys(16)
pub const POSEIDON_ROUND_KEYS__17_IDX: u32 = 76; // PoseidonRoundKeys(17)
pub const POSEIDON_ROUND_KEYS__18_IDX: u32 = 77; // PoseidonRoundKeys(18)
pub const POSEIDON_ROUND_KEYS__19_IDX: u32 = 78; // PoseidonRoundKeys(19)
pub const POSEIDON_ROUND_KEYS__20_IDX: u32 = 79; // PoseidonRoundKeys(20)
pub const POSEIDON_ROUND_KEYS__21_IDX: u32 = 80; // PoseidonRoundKeys(21)
pub const POSEIDON_ROUND_KEYS__22_IDX: u32 = 81; // PoseidonRoundKeys(22)
pub const POSEIDON_ROUND_KEYS__23_IDX: u32 = 82; // PoseidonRoundKeys(23)
pub const POSEIDON_ROUND_KEYS__24_IDX: u32 = 83; // PoseidonRoundKeys(24)
pub const POSEIDON_ROUND_KEYS__25_IDX: u32 = 84; // PoseidonRoundKeys(25)
pub const POSEIDON_ROUND_KEYS__26_IDX: u32 = 85; // PoseidonRoundKeys(26)
pub const POSEIDON_ROUND_KEYS__27_IDX: u32 = 86; // PoseidonRoundKeys(27)
pub const POSEIDON_ROUND_KEYS__28_IDX: u32 = 87; // PoseidonRoundKeys(28)
pub const POSEIDON_ROUND_KEYS__29_IDX: u32 = 88; // PoseidonRoundKeys(29)
pub const SEQ__5_0_IDX: u32 = 89; // Seq(5)
pub const SEQ__4_0_IDX: u32 = 90; // Seq(4)
pub const BLAKE_SIGMA__0_IDX: u32 = 91; // BlakeSigma(0)
pub const BLAKE_SIGMA__1_IDX: u32 = 92; // BlakeSigma(1)
pub const BLAKE_SIGMA__2_IDX: u32 = 93; // BlakeSigma(2)
pub const BLAKE_SIGMA__3_IDX: u32 = 94; // BlakeSigma(3)
pub const BLAKE_SIGMA__4_IDX: u32 = 95; // BlakeSigma(4)
pub const BLAKE_SIGMA__5_IDX: u32 = 96; // BlakeSigma(5)
pub const BLAKE_SIGMA__6_IDX: u32 = 97; // BlakeSigma(6)
pub const BLAKE_SIGMA__7_IDX: u32 = 98; // BlakeSigma(7)
pub const BLAKE_SIGMA__8_IDX: u32 = 99; // BlakeSigma(8)
pub const BLAKE_SIGMA__9_IDX: u32 = 100; // BlakeSigma(9)
pub const BLAKE_SIGMA__10_IDX: u32 = 101; // BlakeSigma(10)
pub const BLAKE_SIGMA__11_IDX: u32 = 102; // BlakeSigma(11)
pub const BLAKE_SIGMA__12_IDX: u32 = 103; // BlakeSigma(12)
pub const BLAKE_SIGMA__13_IDX: u32 = 104; // BlakeSigma(13)
pub const BLAKE_SIGMA__14_IDX: u32 = 105; // BlakeSigma(14)
pub const BLAKE_SIGMA__15_IDX: u32 = 106; // BlakeSigma(15)

pub fn seq_column_idx(log_size: u32) -> PreprocessedColumnIdx {
    match log_size {
        0 | 1 | 2 | 3 => panic!(),
        4 => SEQ__4_0_IDX,
        5 => SEQ__5_0_IDX,
        6 => SEQ__6_0_IDX,
        7 => SEQ__7_0_IDX,
        8 => SEQ__8_0_IDX,
        9 => SEQ__9_0_IDX,
        10 => SEQ__10_0_IDX,
        11 => SEQ__11_0_IDX,
        12 => SEQ__12_0_IDX,
        13 => SEQ__13_0_IDX,
        14 => SEQ__14_0_IDX,
        15 => SEQ__15_0_IDX,
        16 => SEQ__16_0_IDX,
        17 => SEQ__17_0_IDX,
        18 => SEQ__18_0_IDX,
        19 => SEQ__19_0_IDX,
        20 => SEQ__20_0_IDX,
        21 => SEQ__21_0_IDX,
        22 => SEQ__22_0_IDX,
        23 => SEQ__23_0_IDX,
        24 => SEQ__24_0_IDX,
        25 => SEQ__25_0_IDX,
        _ => panic!(),
    }
}

/// Returns PreProcessedTrace::canonical_without_pedersen root for the given blowup factor.
pub fn preprocessed_root(log_blowup_factor: u32) -> Hash {
    match log_blowup_factor - 1 {
        0 => 0x3ab732352bb777ef4a2297a6f4f41b362f2f3bda902645c5cf7a3f33803ad13,
        1 => 0x6a03b3005906fc43caea6272a5abb3678b07eefd595fd2afb5ddb980e682781,
        2 => 0x71944a190ebd4721f696f8d1c5e7a95e8efc70d104d7391b0de4ac965baba70,
        3 => 0x5d83b175ae26614b615d51c92927272a7b5b50a74afa6b84536142db0a25728,
        4 => 0x6ba5cf14072a327807b33acc97bb365a796f9bedfcabde9461a25131205ff83,
        _ => panic!("invalid blowup factor"),
    }
}
