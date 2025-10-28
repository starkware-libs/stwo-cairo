use core::box::BoxImpl;
use stwo_constraint_framework::PreprocessedColumnIdx;
use stwo_verifier_core::Hash;

// IMPORTANT: This file must exactly match the output and ordering of the prover preprocessed
// trace declaration. If the function changes, this array must be updated to stay in sync.
// https://github.com/starkware-libs/stwo-cairo/blob/d8b2aff6ac7c4e991b9e91140ff20d6e0d9ea0b3/stwo_cairo_prover/crates/common/src/preprocessed_columns/preprocessed_trace.rs#L43

pub const NUM_PREPROCESSED_COLUMNS: u32 = 163;

pub const PREPROCESSED_COLUMN_LOG_SIZE: [u32; NUM_PREPROCESSED_COLUMNS] = [
    25, // Seq(25)
    24, // Seq(24)
    23, // Seq(23)
    23, // PedersenPoints(0)
    23, // PedersenPoints(1)
    23, // PedersenPoints(2)
    23, // PedersenPoints(3)
    23, // PedersenPoints(4)
    23, // PedersenPoints(5)
    23, // PedersenPoints(6)
    23, // PedersenPoints(7)
    23, // PedersenPoints(8)
    23, // PedersenPoints(9)
    23, // PedersenPoints(10)
    23, // PedersenPoints(11)
    23, // PedersenPoints(12)
    23, // PedersenPoints(13)
    23, // PedersenPoints(14)
    23, // PedersenPoints(15)
    23, // PedersenPoints(16)
    23, // PedersenPoints(17)
    23, // PedersenPoints(18)
    23, // PedersenPoints(19)
    23, // PedersenPoints(20)
    23, // PedersenPoints(21)
    23, // PedersenPoints(22)
    23, // PedersenPoints(23)
    23, // PedersenPoints(24)
    23, // PedersenPoints(25)
    23, // PedersenPoints(26)
    23, // PedersenPoints(27)
    23, // PedersenPoints(28)
    23, // PedersenPoints(29)
    23, // PedersenPoints(30)
    23, // PedersenPoints(31)
    23, // PedersenPoints(32)
    23, // PedersenPoints(33)
    23, // PedersenPoints(34)
    23, // PedersenPoints(35)
    23, // PedersenPoints(36)
    23, // PedersenPoints(37)
    23, // PedersenPoints(38)
    23, // PedersenPoints(39)
    23, // PedersenPoints(40)
    23, // PedersenPoints(41)
    23, // PedersenPoints(42)
    23, // PedersenPoints(43)
    23, // PedersenPoints(44)
    23, // PedersenPoints(45)
    23, // PedersenPoints(46)
    23, // PedersenPoints(47)
    23, // PedersenPoints(48)
    23, // PedersenPoints(49)
    23, // PedersenPoints(50)
    23, // PedersenPoints(51)
    23, // PedersenPoints(52)
    23, // PedersenPoints(53)
    23, // PedersenPoints(54)
    23, // PedersenPoints(55)
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

pub const SEQ_25_IDX: u32 = 0; // Seq(25)
pub const SEQ_24_IDX: u32 = 1; // Seq(24)
pub const SEQ_23_IDX: u32 = 2; // Seq(23)
pub const PEDERSEN_POINTS_0_IDX: u32 = 3; // PedersenPoints(0)
pub const PEDERSEN_POINTS_1_IDX: u32 = 4; // PedersenPoints(1)
pub const PEDERSEN_POINTS_2_IDX: u32 = 5; // PedersenPoints(2)
pub const PEDERSEN_POINTS_3_IDX: u32 = 6; // PedersenPoints(3)
pub const PEDERSEN_POINTS_4_IDX: u32 = 7; // PedersenPoints(4)
pub const PEDERSEN_POINTS_5_IDX: u32 = 8; // PedersenPoints(5)
pub const PEDERSEN_POINTS_6_IDX: u32 = 9; // PedersenPoints(6)
pub const PEDERSEN_POINTS_7_IDX: u32 = 10; // PedersenPoints(7)
pub const PEDERSEN_POINTS_8_IDX: u32 = 11; // PedersenPoints(8)
pub const PEDERSEN_POINTS_9_IDX: u32 = 12; // PedersenPoints(9)
pub const PEDERSEN_POINTS_10_IDX: u32 = 13; // PedersenPoints(10)
pub const PEDERSEN_POINTS_11_IDX: u32 = 14; // PedersenPoints(11)
pub const PEDERSEN_POINTS_12_IDX: u32 = 15; // PedersenPoints(12)
pub const PEDERSEN_POINTS_13_IDX: u32 = 16; // PedersenPoints(13)
pub const PEDERSEN_POINTS_14_IDX: u32 = 17; // PedersenPoints(14)
pub const PEDERSEN_POINTS_15_IDX: u32 = 18; // PedersenPoints(15)
pub const PEDERSEN_POINTS_16_IDX: u32 = 19; // PedersenPoints(16)
pub const PEDERSEN_POINTS_17_IDX: u32 = 20; // PedersenPoints(17)
pub const PEDERSEN_POINTS_18_IDX: u32 = 21; // PedersenPoints(18)
pub const PEDERSEN_POINTS_19_IDX: u32 = 22; // PedersenPoints(19)
pub const PEDERSEN_POINTS_20_IDX: u32 = 23; // PedersenPoints(20)
pub const PEDERSEN_POINTS_21_IDX: u32 = 24; // PedersenPoints(21)
pub const PEDERSEN_POINTS_22_IDX: u32 = 25; // PedersenPoints(22)
pub const PEDERSEN_POINTS_23_IDX: u32 = 26; // PedersenPoints(23)
pub const PEDERSEN_POINTS_24_IDX: u32 = 27; // PedersenPoints(24)
pub const PEDERSEN_POINTS_25_IDX: u32 = 28; // PedersenPoints(25)
pub const PEDERSEN_POINTS_26_IDX: u32 = 29; // PedersenPoints(26)
pub const PEDERSEN_POINTS_27_IDX: u32 = 30; // PedersenPoints(27)
pub const PEDERSEN_POINTS_28_IDX: u32 = 31; // PedersenPoints(28)
pub const PEDERSEN_POINTS_29_IDX: u32 = 32; // PedersenPoints(29)
pub const PEDERSEN_POINTS_30_IDX: u32 = 33; // PedersenPoints(30)
pub const PEDERSEN_POINTS_31_IDX: u32 = 34; // PedersenPoints(31)
pub const PEDERSEN_POINTS_32_IDX: u32 = 35; // PedersenPoints(32)
pub const PEDERSEN_POINTS_33_IDX: u32 = 36; // PedersenPoints(33)
pub const PEDERSEN_POINTS_34_IDX: u32 = 37; // PedersenPoints(34)
pub const PEDERSEN_POINTS_35_IDX: u32 = 38; // PedersenPoints(35)
pub const PEDERSEN_POINTS_36_IDX: u32 = 39; // PedersenPoints(36)
pub const PEDERSEN_POINTS_37_IDX: u32 = 40; // PedersenPoints(37)
pub const PEDERSEN_POINTS_38_IDX: u32 = 41; // PedersenPoints(38)
pub const PEDERSEN_POINTS_39_IDX: u32 = 42; // PedersenPoints(39)
pub const PEDERSEN_POINTS_40_IDX: u32 = 43; // PedersenPoints(40)
pub const PEDERSEN_POINTS_41_IDX: u32 = 44; // PedersenPoints(41)
pub const PEDERSEN_POINTS_42_IDX: u32 = 45; // PedersenPoints(42)
pub const PEDERSEN_POINTS_43_IDX: u32 = 46; // PedersenPoints(43)
pub const PEDERSEN_POINTS_44_IDX: u32 = 47; // PedersenPoints(44)
pub const PEDERSEN_POINTS_45_IDX: u32 = 48; // PedersenPoints(45)
pub const PEDERSEN_POINTS_46_IDX: u32 = 49; // PedersenPoints(46)
pub const PEDERSEN_POINTS_47_IDX: u32 = 50; // PedersenPoints(47)
pub const PEDERSEN_POINTS_48_IDX: u32 = 51; // PedersenPoints(48)
pub const PEDERSEN_POINTS_49_IDX: u32 = 52; // PedersenPoints(49)
pub const PEDERSEN_POINTS_50_IDX: u32 = 53; // PedersenPoints(50)
pub const PEDERSEN_POINTS_51_IDX: u32 = 54; // PedersenPoints(51)
pub const PEDERSEN_POINTS_52_IDX: u32 = 55; // PedersenPoints(52)
pub const PEDERSEN_POINTS_53_IDX: u32 = 56; // PedersenPoints(53)
pub const PEDERSEN_POINTS_54_IDX: u32 = 57; // PedersenPoints(54)
pub const PEDERSEN_POINTS_55_IDX: u32 = 58; // PedersenPoints(55)
pub const SEQ_22_IDX: u32 = 59; // Seq(22)
pub const SEQ_21_IDX: u32 = 60; // Seq(21)
pub const SEQ_20_IDX: u32 = 61; // Seq(20)
pub const BITWISE_XOR_10_0_IDX: u32 = 62; // BitwiseXor((10, 0))
pub const BITWISE_XOR_10_1_IDX: u32 = 63; // BitwiseXor((10, 1))
pub const BITWISE_XOR_10_2_IDX: u32 = 64; // BitwiseXor((10, 2))
pub const SEQ_19_IDX: u32 = 65; // Seq(19)
pub const SEQ_18_IDX: u32 = 66; // Seq(18)
pub const BITWISE_XOR_9_0_IDX: u32 = 67; // BitwiseXor((9, 0))
pub const BITWISE_XOR_9_1_IDX: u32 = 68; // BitwiseXor((9, 1))
pub const BITWISE_XOR_9_2_IDX: u32 = 69; // BitwiseXor((9, 2))
pub const RANGE_CHECK_9_9_COLUMN_0_IDX: u32 = 70; // RangeCheck2(([9, 9], 0))
pub const RANGE_CHECK_9_9_COLUMN_1_IDX: u32 = 71; // RangeCheck2(([9, 9], 1))
pub const RANGE_CHECK_3_6_6_3_COLUMN_0_IDX: u32 = 72; // RangeCheck4(([3, 6, 6, 3], 0))
pub const RANGE_CHECK_3_6_6_3_COLUMN_1_IDX: u32 = 73; // RangeCheck4(([3, 6, 6, 3], 1))
pub const RANGE_CHECK_3_6_6_3_COLUMN_2_IDX: u32 = 74; // RangeCheck4(([3, 6, 6, 3], 2))
pub const RANGE_CHECK_3_6_6_3_COLUMN_3_IDX: u32 = 75; // RangeCheck4(([3, 6, 6, 3], 3))
pub const SEQ_17_IDX: u32 = 76; // Seq(17)
pub const SEQ_16_IDX: u32 = 77; // Seq(16)
pub const BITWISE_XOR_8_0_IDX: u32 = 78; // BitwiseXor((8, 0))
pub const BITWISE_XOR_8_1_IDX: u32 = 79; // BitwiseXor((8, 1))
pub const BITWISE_XOR_8_2_IDX: u32 = 80; // BitwiseXor((8, 2))
pub const RANGE_CHECK_4_4_4_4_COLUMN_0_IDX: u32 = 81; // RangeCheck4(([4, 4, 4, 4], 0))
pub const RANGE_CHECK_4_4_4_4_COLUMN_1_IDX: u32 = 82; // RangeCheck4(([4, 4, 4, 4], 1))
pub const RANGE_CHECK_4_4_4_4_COLUMN_2_IDX: u32 = 83; // RangeCheck4(([4, 4, 4, 4], 2))
pub const RANGE_CHECK_4_4_4_4_COLUMN_3_IDX: u32 = 84; // RangeCheck4(([4, 4, 4, 4], 3))
pub const SEQ_15_IDX: u32 = 85; // Seq(15)
pub const RANGE_CHECK_3_3_3_3_3_COLUMN_0_IDX: u32 = 86; // RangeCheck5(([3, 3, 3, 3, 3], 0))
pub const RANGE_CHECK_3_3_3_3_3_COLUMN_1_IDX: u32 = 87; // RangeCheck5(([3, 3, 3, 3, 3], 1))
pub const RANGE_CHECK_3_3_3_3_3_COLUMN_2_IDX: u32 = 88; // RangeCheck5(([3, 3, 3, 3, 3], 2))
pub const RANGE_CHECK_3_3_3_3_3_COLUMN_3_IDX: u32 = 89; // RangeCheck5(([3, 3, 3, 3, 3], 3))
pub const RANGE_CHECK_3_3_3_3_3_COLUMN_4_IDX: u32 = 90; // RangeCheck5(([3, 3, 3, 3, 3], 4))
pub const SEQ_14_IDX: u32 = 91; // Seq(14)
pub const BITWISE_XOR_7_0_IDX: u32 = 92; // BitwiseXor((7, 0))
pub const BITWISE_XOR_7_1_IDX: u32 = 93; // BitwiseXor((7, 1))
pub const BITWISE_XOR_7_2_IDX: u32 = 94; // BitwiseXor((7, 2))
pub const RANGE_CHECK_7_2_5_COLUMN_0_IDX: u32 = 95; // RangeCheck3(([7, 2, 5], 0))
pub const RANGE_CHECK_7_2_5_COLUMN_1_IDX: u32 = 96; // RangeCheck3(([7, 2, 5], 1))
pub const RANGE_CHECK_7_2_5_COLUMN_2_IDX: u32 = 97; // RangeCheck3(([7, 2, 5], 2))
pub const SEQ_13_IDX: u32 = 98; // Seq(13)
pub const SEQ_12_IDX: u32 = 99; // Seq(12)
pub const SEQ_11_IDX: u32 = 100; // Seq(11)
pub const SEQ_10_IDX: u32 = 101; // Seq(10)
pub const SEQ_9_IDX: u32 = 102; // Seq(9)
pub const RANGE_CHECK_5_4_COLUMN_0_IDX: u32 = 103; // RangeCheck2(([5, 4], 0))
pub const RANGE_CHECK_5_4_COLUMN_1_IDX: u32 = 104; // RangeCheck2(([5, 4], 1))
pub const SEQ_8_IDX: u32 = 105; // Seq(8)
pub const BITWISE_XOR_4_0_IDX: u32 = 106; // BitwiseXor((4, 0))
pub const BITWISE_XOR_4_1_IDX: u32 = 107; // BitwiseXor((4, 1))
pub const BITWISE_XOR_4_2_IDX: u32 = 108; // BitwiseXor((4, 2))
pub const RANGE_CHECK_4_4_COLUMN_0_IDX: u32 = 109; // RangeCheck2(([4, 4], 0))
pub const RANGE_CHECK_4_4_COLUMN_1_IDX: u32 = 110; // RangeCheck2(([4, 4], 1))
pub const SEQ_7_IDX: u32 = 111; // Seq(7)
pub const RANGE_CHECK_4_3_COLUMN_0_IDX: u32 = 112; // RangeCheck2(([4, 3], 0))
pub const RANGE_CHECK_4_3_COLUMN_1_IDX: u32 = 113; // RangeCheck2(([4, 3], 1))
pub const SEQ_6_IDX: u32 = 114; // Seq(6)
pub const POSEIDON_ROUND_KEYS_0_IDX: u32 = 115; // PoseidonRoundKeys(0)
pub const POSEIDON_ROUND_KEYS_1_IDX: u32 = 116; // PoseidonRoundKeys(1)
pub const POSEIDON_ROUND_KEYS_2_IDX: u32 = 117; // PoseidonRoundKeys(2)
pub const POSEIDON_ROUND_KEYS_3_IDX: u32 = 118; // PoseidonRoundKeys(3)
pub const POSEIDON_ROUND_KEYS_4_IDX: u32 = 119; // PoseidonRoundKeys(4)
pub const POSEIDON_ROUND_KEYS_5_IDX: u32 = 120; // PoseidonRoundKeys(5)
pub const POSEIDON_ROUND_KEYS_6_IDX: u32 = 121; // PoseidonRoundKeys(6)
pub const POSEIDON_ROUND_KEYS_7_IDX: u32 = 122; // PoseidonRoundKeys(7)
pub const POSEIDON_ROUND_KEYS_8_IDX: u32 = 123; // PoseidonRoundKeys(8)
pub const POSEIDON_ROUND_KEYS_9_IDX: u32 = 124; // PoseidonRoundKeys(9)
pub const POSEIDON_ROUND_KEYS_10_IDX: u32 = 125; // PoseidonRoundKeys(10)
pub const POSEIDON_ROUND_KEYS_11_IDX: u32 = 126; // PoseidonRoundKeys(11)
pub const POSEIDON_ROUND_KEYS_12_IDX: u32 = 127; // PoseidonRoundKeys(12)
pub const POSEIDON_ROUND_KEYS_13_IDX: u32 = 128; // PoseidonRoundKeys(13)
pub const POSEIDON_ROUND_KEYS_14_IDX: u32 = 129; // PoseidonRoundKeys(14)
pub const POSEIDON_ROUND_KEYS_15_IDX: u32 = 130; // PoseidonRoundKeys(15)
pub const POSEIDON_ROUND_KEYS_16_IDX: u32 = 131; // PoseidonRoundKeys(16)
pub const POSEIDON_ROUND_KEYS_17_IDX: u32 = 132; // PoseidonRoundKeys(17)
pub const POSEIDON_ROUND_KEYS_18_IDX: u32 = 133; // PoseidonRoundKeys(18)
pub const POSEIDON_ROUND_KEYS_19_IDX: u32 = 134; // PoseidonRoundKeys(19)
pub const POSEIDON_ROUND_KEYS_20_IDX: u32 = 135; // PoseidonRoundKeys(20)
pub const POSEIDON_ROUND_KEYS_21_IDX: u32 = 136; // PoseidonRoundKeys(21)
pub const POSEIDON_ROUND_KEYS_22_IDX: u32 = 137; // PoseidonRoundKeys(22)
pub const POSEIDON_ROUND_KEYS_23_IDX: u32 = 138; // PoseidonRoundKeys(23)
pub const POSEIDON_ROUND_KEYS_24_IDX: u32 = 139; // PoseidonRoundKeys(24)
pub const POSEIDON_ROUND_KEYS_25_IDX: u32 = 140; // PoseidonRoundKeys(25)
pub const POSEIDON_ROUND_KEYS_26_IDX: u32 = 141; // PoseidonRoundKeys(26)
pub const POSEIDON_ROUND_KEYS_27_IDX: u32 = 142; // PoseidonRoundKeys(27)
pub const POSEIDON_ROUND_KEYS_28_IDX: u32 = 143; // PoseidonRoundKeys(28)
pub const POSEIDON_ROUND_KEYS_29_IDX: u32 = 144; // PoseidonRoundKeys(29)
pub const SEQ_5_IDX: u32 = 145; // Seq(5)
pub const SEQ_4_IDX: u32 = 146; // Seq(4)
pub const BLAKE_SIGMA_0_IDX: u32 = 147; // BlakeSigma(0)
pub const BLAKE_SIGMA_1_IDX: u32 = 148; // BlakeSigma(1)
pub const BLAKE_SIGMA_2_IDX: u32 = 149; // BlakeSigma(2)
pub const BLAKE_SIGMA_3_IDX: u32 = 150; // BlakeSigma(3)
pub const BLAKE_SIGMA_4_IDX: u32 = 151; // BlakeSigma(4)
pub const BLAKE_SIGMA_5_IDX: u32 = 152; // BlakeSigma(5)
pub const BLAKE_SIGMA_6_IDX: u32 = 153; // BlakeSigma(6)
pub const BLAKE_SIGMA_7_IDX: u32 = 154; // BlakeSigma(7)
pub const BLAKE_SIGMA_8_IDX: u32 = 155; // BlakeSigma(8)
pub const BLAKE_SIGMA_9_IDX: u32 = 156; // BlakeSigma(9)
pub const BLAKE_SIGMA_10_IDX: u32 = 157; // BlakeSigma(10)
pub const BLAKE_SIGMA_11_IDX: u32 = 158; // BlakeSigma(11)
pub const BLAKE_SIGMA_12_IDX: u32 = 159; // BlakeSigma(12)
pub const BLAKE_SIGMA_13_IDX: u32 = 160; // BlakeSigma(13)
pub const BLAKE_SIGMA_14_IDX: u32 = 161; // BlakeSigma(14)
pub const BLAKE_SIGMA_15_IDX: u32 = 162; // BlakeSigma(15)

pub fn seq_column_idx(log_size: u32) -> PreprocessedColumnIdx {
    match log_size {
        0 | 1 | 2 | 3 => panic!(),
        4 => SEQ_4_IDX,
        5 => SEQ_5_IDX,
        6 => SEQ_6_IDX,
        7 => SEQ_7_IDX,
        8 => SEQ_8_IDX,
        9 => SEQ_9_IDX,
        10 => SEQ_10_IDX,
        11 => SEQ_11_IDX,
        12 => SEQ_12_IDX,
        13 => SEQ_13_IDX,
        14 => SEQ_14_IDX,
        15 => SEQ_15_IDX,
        16 => SEQ_16_IDX,
        17 => SEQ_17_IDX,
        18 => SEQ_18_IDX,
        19 => SEQ_19_IDX,
        20 => SEQ_20_IDX,
        21 => SEQ_21_IDX,
        22 => SEQ_22_IDX,
        23 => SEQ_23_IDX,
        24 => SEQ_24_IDX,
        25 => SEQ_25_IDX,
        _ => panic!(),
    }
}

/// Returns PreProcessedTrace::canonical root for the given blowup factor.
pub fn preprocessed_root(log_blowup_factor: u32) -> Hash {
    match log_blowup_factor - 1 {
        0 => Hash {
            hash: BoxImpl::new(
                [
                    0xa7e4ed57, 0xdfae42b4, 0xa669cd01, 0x094a86f8, 0x91f39aaa, 0x20ad98e3,
                    0xa82ebfed, 0xc771c404,
                ],
            ),
        },
        1 => Hash {
            hash: BoxImpl::new(
                [
                    0xf6889ffd, 0x695f7abc, 0xd498d674, 0x522bb6d6, 0x52dd6e27, 0x1edfbacb,
                    0x34e72017, 0x1990f2fa,
                ],
            ),
        },
        2 => Hash {
            hash: BoxImpl::new(
                [
                    0xf937429a, 0x97988c3c, 0x8bc41d5d, 0xeed48adf, 0x0faaade2, 0x49c43f67,
                    0x76ace5e2, 0xbbd1592b,
                ],
            ),
        },
        3 => Hash {
            hash: BoxImpl::new(
                [
                    0xe8de9fb1, 0x1496075a, 0x409c7ae5, 0x92531e0d, 0xd6442f98, 0x65210522,
                    0x8d9b085d, 0xa296d316,
                ],
            ),
        },
        4 => Hash {
            hash: BoxImpl::new(
                [
                    0xdfd138d0, 0x6f7446ef, 0x25c15cb8, 0x098dcd06, 0x2f478905, 0xca8fe8ff,
                    0x73484cd8, 0x8a711ab7,
                ],
            ),
        },
        _ => panic!("invalid blowup factor"),
    }
}
