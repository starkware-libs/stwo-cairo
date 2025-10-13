use std::sync::LazyLock;

use stwo::core::fields::m31::BaseField;
use stwo::core::poly::circle::CanonicCoset;
use stwo::prover::backend::simd::column::BaseColumn;
use stwo::prover::backend::simd::m31::PackedM31;
use stwo::prover::backend::simd::SimdBackend;
use stwo::prover::poly::circle::CircleEvaluation;
use stwo::prover::poly::BitReversedOrder;
use stwo_constraint_framework::preprocessed_columns::PreProcessedColumnId;

use crate::preprocessed_columns::preprocessed_trace::PreProcessedColumn;
use crate::preprocessed_columns::preprocessed_utils::pad;
use crate::prover_types::cpu::M31;
use crate::prover_types::simd::N_LANES;

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

pub const SHA256_K_TABLE: &str = "sha256_k";
const K_LOG_N_ROWS: u32 = N_SHA256_ROUNDS.ilog2();

#[derive(Debug)]
pub struct Sha256K {
    pub col: usize,
}

impl Sha256K {
    pub fn new(col: usize) -> Self {
        Self { col }
    }
    pub fn packed_at(&self, vec_row: usize) -> PackedM31 {
        let array = K[(vec_row * N_LANES)..((vec_row + 1) * N_LANES)]
            .iter()
            .map(|&x| {
                M31::from_u32_unchecked(if self.col == 0 {
                    x & u16::MAX as u32
                } else {
                    x >> 16
                })
            })
            .collect::<Vec<M31>>()
            .try_into()
            .unwrap();
        PackedM31::from_array(array)
    }
}

impl PreProcessedColumn for Sha256K {
    fn log_size(&self) -> u32 {
        K_LOG_N_ROWS
    }

    fn gen_column_simd(&self) -> CircleEvaluation<SimdBackend, BaseField, BitReversedOrder> {
        let table = K
            .into_iter()
            .map(|x| {
                if self.col == 0 {
                    M31::from_u32_unchecked(x & u16::MAX as u32)
                } else {
                    M31::from_u32_unchecked(x >> 16)
                }
            })
            .collect::<Vec<M31>>();
        CircleEvaluation::new(
            CanonicCoset::new(K_LOG_N_ROWS).circle_domain(),
            BaseColumn::from_iter(table),
        )
    }

    fn id(&self) -> PreProcessedColumnId {
        PreProcessedColumnId {
            id: format!("{}_{}", SHA256_K_TABLE, self.col),
        }
    }
}

pub const SHA256_SIGMA_TABLE: &str = "sha256_sigma";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sha256SigmaType {
    BigSigma0O0,
    BigSigma0O1,
    BigSigma1O0,
    BigSigma1O1,
    SmallSigma0O0,
    SmallSigma0O1,
    SmallSigma1O0,
    SmallSigma1O1,
}

impl std::fmt::Display for Sha256SigmaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Sha256SigmaType::BigSigma0O0 => write!(f, "big_sigma0_O0"),
            Sha256SigmaType::BigSigma0O1 => write!(f, "big_sigma0_O1"),
            Sha256SigmaType::BigSigma1O0 => write!(f, "big_sigma1_O0"),
            Sha256SigmaType::BigSigma1O1 => write!(f, "big_sigma1_O1"),
            Sha256SigmaType::SmallSigma0O0 => write!(f, "small_sigma0_O0"),
            Sha256SigmaType::SmallSigma0O1 => write!(f, "small_sigma0_O1"),
            Sha256SigmaType::SmallSigma1O0 => write!(f, "small_sigma1_O0"),
            Sha256SigmaType::SmallSigma1O1 => write!(f, "small_sigma1_O1"),
        }
    }
}
pub static SMALL_SIGMA0_0_COLUMNS: LazyLock<[Vec<M31>; 6]> =
    LazyLock::new(|| generate_sigma_columns(Sha256SigmaType::SmallSigma0O0));
pub static SMALL_SIGMA0_1_COLUMNS: LazyLock<[Vec<M31>; 6]> =
    LazyLock::new(|| generate_sigma_columns(Sha256SigmaType::SmallSigma0O1));
pub static SMALL_SIGMA1_0_COLUMNS: LazyLock<[Vec<M31>; 6]> =
    LazyLock::new(|| generate_sigma_columns(Sha256SigmaType::SmallSigma1O0));
pub static SMALL_SIGMA1_1_COLUMNS: LazyLock<[Vec<M31>; 6]> =
    LazyLock::new(|| generate_sigma_columns(Sha256SigmaType::SmallSigma1O1));
pub static BIG_SIGMA0_0_COLUMNS: LazyLock<[Vec<M31>; 6]> =
    LazyLock::new(|| generate_sigma_columns(Sha256SigmaType::BigSigma0O0));
pub static BIG_SIGMA0_1_COLUMNS: LazyLock<[Vec<M31>; 6]> =
    LazyLock::new(|| generate_sigma_columns(Sha256SigmaType::BigSigma0O1));
pub static BIG_SIGMA1_0_COLUMNS: LazyLock<[Vec<M31>; 6]> =
    LazyLock::new(|| generate_sigma_columns(Sha256SigmaType::BigSigma1O0));
pub static BIG_SIGMA1_1_COLUMNS: LazyLock<[Vec<M31>; 6]> =
    LazyLock::new(|| generate_sigma_columns(Sha256SigmaType::BigSigma1O1));

fn get_m31_big_sigma00(round: usize, col: usize) -> M31 {
    BIG_SIGMA0_0_COLUMNS[col][round]
}
fn get_m31_big_sigma01(round: usize, col: usize) -> M31 {
    BIG_SIGMA0_1_COLUMNS[col][round]
}
fn get_m31_big_sigma10(round: usize, col: usize) -> M31 {
    BIG_SIGMA1_0_COLUMNS[col][round]
}
fn get_m31_big_sigma11(round: usize, col: usize) -> M31 {
    BIG_SIGMA1_1_COLUMNS[col][round]
}

fn get_m31_small_sigma00(round: usize, col: usize) -> M31 {
    SMALL_SIGMA0_0_COLUMNS[col][round]
}
fn get_m31_small_sigma01(round: usize, col: usize) -> M31 {
    SMALL_SIGMA0_1_COLUMNS[col][round]
}
fn get_m31_small_sigma10(round: usize, col: usize) -> M31 {
    SMALL_SIGMA1_0_COLUMNS[col][round]
}
fn get_m31_small_sigma11(round: usize, col: usize) -> M31 {
    SMALL_SIGMA1_1_COLUMNS[col][round]
}
pub struct Sha256SigmaTable {
    col: usize,
    sigma: Sha256SigmaType,
}

impl Sha256SigmaTable {
    pub fn new(sigma: Sha256SigmaType, col: usize) -> Self {
        Self { col, sigma }
    }
    pub fn packed_at(&self, vec_row: usize) -> PackedM31 {
        let array = match self.sigma {
            Sha256SigmaType::BigSigma0O0 => &BIG_SIGMA0_0_COLUMNS[self.col],
            Sha256SigmaType::BigSigma0O1 => &BIG_SIGMA0_1_COLUMNS[self.col],
            Sha256SigmaType::BigSigma1O0 => &BIG_SIGMA1_0_COLUMNS[self.col],
            Sha256SigmaType::BigSigma1O1 => &BIG_SIGMA1_1_COLUMNS[self.col],
            Sha256SigmaType::SmallSigma0O0 => &SMALL_SIGMA0_0_COLUMNS[self.col],
            Sha256SigmaType::SmallSigma0O1 => &SMALL_SIGMA0_1_COLUMNS[self.col],
            Sha256SigmaType::SmallSigma1O0 => &SMALL_SIGMA1_0_COLUMNS[self.col],
            Sha256SigmaType::SmallSigma1O1 => &SMALL_SIGMA1_1_COLUMNS[self.col],
        }[(vec_row * N_LANES)..((vec_row + 1) * N_LANES)]
            .try_into()
            .unwrap();

        PackedM31::from_array(array)
    }
}
fn enumerate_variants(indexes: &[u8], base: u32) -> Vec<u32> {
    let vary = indexes.iter().fold(0u32, |m, &i| m | (1u32 << i));
    let fixed = base & !vary;
    let mut sub = vary;
    let mut res = vec![];
    loop {
        res.push(fixed | sub);
        if sub == 0 {
            break;
        }
        sub = (sub - 1) & vary; // next submask
    }
    res
}
fn generate_sigma_columns(sigma: Sha256SigmaType) -> [Vec<M31>; 6] {
    let mut input_indexes = vec![];
    match sigma {
        Sha256SigmaType::SmallSigma0O0 => {
            input_indexes.extend(small_sigma0::L1_INDEXES);
            input_indexes.extend(small_sigma0::L2_INDEXES);
            input_indexes.extend(small_sigma0::H2_INDEXES);
        }
        Sha256SigmaType::SmallSigma0O1 => {
            input_indexes.extend(small_sigma0::L0_INDEXES);
            input_indexes.extend(small_sigma0::H0_INDEXES);
            input_indexes.extend(small_sigma0::H1_INDEXES);
        }
        Sha256SigmaType::SmallSigma1O0 => {
            input_indexes.extend(small_sigma1::L0_INDEXES);
            input_indexes.extend(small_sigma1::H0_INDEXES);
        }
        Sha256SigmaType::SmallSigma1O1 => {
            input_indexes.extend(small_sigma1::L1_INDEXES);
            input_indexes.extend(small_sigma1::L2_INDEXES);
            input_indexes.extend(small_sigma1::H1_INDEXES);
            input_indexes.extend(small_sigma1::H2_INDEXES);
        }
        Sha256SigmaType::BigSigma0O0 => {
            input_indexes.extend(big_sigma0::L0_INDEXES);
            input_indexes.extend(big_sigma0::H0_INDEXES);
            input_indexes.extend(big_sigma0::H1_INDEXES);
        }
        Sha256SigmaType::BigSigma0O1 => {
            input_indexes.extend(big_sigma0::L1_INDEXES);
            input_indexes.extend(big_sigma0::L2_INDEXES);
            input_indexes.extend(big_sigma0::H2_INDEXES);
        }
        Sha256SigmaType::BigSigma1O0 => {
            input_indexes.extend(big_sigma1::L1_INDEXES);
            input_indexes.extend(big_sigma1::L2_INDEXES);
            input_indexes.extend(big_sigma1::H2_INDEXES);
        }
        Sha256SigmaType::BigSigma1O1 => {
            input_indexes.extend(big_sigma1::L0_INDEXES);
            input_indexes.extend(big_sigma1::H0_INDEXES);
            input_indexes.extend(big_sigma1::H1_INDEXES);
        }
    };
    let res: Vec<[M31; 6]> = enumerate_variants(&input_indexes, 0)
        .iter()
        .map(|v| {
            let [n1, n2, n3] = match sigma {
                Sha256SigmaType::BigSigma0O0 | Sha256SigmaType::BigSigma0O1 => big_sigma0::N,
                Sha256SigmaType::BigSigma1O0 | Sha256SigmaType::BigSigma1O1 => big_sigma1::N,
                Sha256SigmaType::SmallSigma0O0 | Sha256SigmaType::SmallSigma0O1 => small_sigma0::N,
                Sha256SigmaType::SmallSigma1O0 | Sha256SigmaType::SmallSigma1O1 => small_sigma1::N,
            };
            let is_big = matches!(
                sigma,
                Sha256SigmaType::BigSigma0O0
                    | Sha256SigmaType::BigSigma0O1
                    | Sha256SigmaType::BigSigma1O0
                    | Sha256SigmaType::BigSigma1O1
            );
            let rotated = v.rotate_right(n1)
                ^ v.rotate_right(n2)
                ^ if is_big { v.rotate_right(n3) } else { v >> n3 };

            let (oxl_mask, oxh_mask, o2l_mask, o2h_mask) = match sigma {
                Sha256SigmaType::BigSigma0O0 => (
                    big_sigma0::O0L,
                    big_sigma0::O0H,
                    big_sigma0::O2L,
                    big_sigma0::O2H,
                ),
                Sha256SigmaType::BigSigma0O1 => (
                    big_sigma0::O1L,
                    big_sigma0::O1H,
                    big_sigma0::O2L,
                    big_sigma0::O2H,
                ),
                Sha256SigmaType::BigSigma1O0 => (
                    big_sigma1::O0L,
                    big_sigma1::O0H,
                    big_sigma1::O2L,
                    big_sigma1::O2H,
                ),
                Sha256SigmaType::BigSigma1O1 => (
                    big_sigma1::O1L,
                    big_sigma1::O1H,
                    big_sigma1::O2L,
                    big_sigma1::O2H,
                ),
                Sha256SigmaType::SmallSigma0O0 => (
                    small_sigma0::O0L,
                    small_sigma0::O0H,
                    small_sigma0::O2L,
                    small_sigma0::O2H,
                ),
                Sha256SigmaType::SmallSigma0O1 => (
                    small_sigma0::O1L,
                    small_sigma0::O1H,
                    small_sigma0::O2L,
                    small_sigma0::O2H,
                ),
                Sha256SigmaType::SmallSigma1O0 => (
                    small_sigma1::O0L,
                    small_sigma1::O0H,
                    small_sigma1::O2L,
                    small_sigma1::O2H,
                ),
                Sha256SigmaType::SmallSigma1O1 => (
                    small_sigma1::O1L,
                    small_sigma1::O1H,
                    small_sigma1::O2L,
                    small_sigma1::O2H,
                ),
            };
            [
                M31::from(v & u16::MAX as u32),
                M31::from(v >> 16),
                M31::from(rotated & oxl_mask as u32),
                M31::from(rotated >> 16 & oxh_mask as u32),
                M31::from(rotated & o2l_mask as u32),
                M31::from(rotated >> 16 & o2h_mask as u32),
            ]
        })
        .collect();
    [
        res.iter().map(|v| v[0]).collect(),
        res.iter().map(|v| v[1]).collect(),
        res.iter().map(|v| v[2]).collect(),
        res.iter().map(|v| v[3]).collect(),
        res.iter().map(|v| v[4]).collect(),
        res.iter().map(|v| v[5]).collect(),
    ]
}

impl PreProcessedColumn for Sha256SigmaTable {
    fn log_size(&self) -> u32 {
        match self.sigma {
            Sha256SigmaType::SmallSigma0O0 => 16,
            Sha256SigmaType::SmallSigma0O1 => 16,
            Sha256SigmaType::SmallSigma1O0 => 12,
            Sha256SigmaType::SmallSigma1O1 => 20,
            Sha256SigmaType::BigSigma0O0 => 16,
            Sha256SigmaType::BigSigma0O1 => 16,
            Sha256SigmaType::BigSigma1O0 => 16,
            Sha256SigmaType::BigSigma1O1 => 16,
        }
    }

    fn gen_column_simd(&self) -> CircleEvaluation<SimdBackend, BaseField, BitReversedOrder> {
        let table = match self.sigma {
            Sha256SigmaType::SmallSigma0O0 => &*SMALL_SIGMA0_0_COLUMNS,
            Sha256SigmaType::SmallSigma0O1 => &*SMALL_SIGMA0_1_COLUMNS,
            Sha256SigmaType::SmallSigma1O0 => &*SMALL_SIGMA1_0_COLUMNS,
            Sha256SigmaType::SmallSigma1O1 => &*SMALL_SIGMA1_1_COLUMNS,
            Sha256SigmaType::BigSigma0O0 => &*BIG_SIGMA0_0_COLUMNS,
            Sha256SigmaType::BigSigma0O1 => &*BIG_SIGMA0_1_COLUMNS,
            Sha256SigmaType::BigSigma1O0 => &*BIG_SIGMA1_0_COLUMNS,
            Sha256SigmaType::BigSigma1O1 => &*BIG_SIGMA1_1_COLUMNS,
        };

        let get_m31 = match self.sigma {
            Sha256SigmaType::BigSigma0O0 => get_m31_big_sigma00,
            Sha256SigmaType::BigSigma0O1 => get_m31_big_sigma01,
            Sha256SigmaType::BigSigma1O0 => get_m31_big_sigma10,
            Sha256SigmaType::BigSigma1O1 => get_m31_big_sigma11,
            Sha256SigmaType::SmallSigma0O0 => get_m31_small_sigma00,
            Sha256SigmaType::SmallSigma0O1 => get_m31_small_sigma01,
            Sha256SigmaType::SmallSigma1O0 => get_m31_small_sigma10,
            Sha256SigmaType::SmallSigma1O1 => get_m31_small_sigma11,
        };

        CircleEvaluation::new(
            CanonicCoset::new(self.log_size()).circle_domain(),
            BaseColumn::from_cpu(pad(get_m31, table[self.col].len(), self.col)),
        )
    }

    fn id(&self) -> PreProcessedColumnId {
        PreProcessedColumnId {
            id: format!("{}_{}_{}", SHA256_SIGMA_TABLE, self.sigma, self.col),
        }
    }
}
