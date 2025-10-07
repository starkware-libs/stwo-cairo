use std::sync::LazyLock;

use stwo::core::fields::m31::BaseField;
use stwo::core::poly::circle::CanonicCoset;
use stwo::prover::backend::simd::column::BaseColumn;
use stwo::prover::backend::simd::m31::PackedM31;
use stwo::prover::backend::simd::SimdBackend;
use stwo::prover::poly::circle::CircleEvaluation;
use stwo::prover::poly::BitReversedOrder;
use stwo_cairo_common::preprocessed_consts::sha256::{
    big_sigma0, big_sigma1, small_sigma0, small_sigma1, K, N_SHA256_ROUNDS,
};
use stwo_cairo_common::prover_types::cpu::M31;
use stwo_cairo_common::prover_types::simd::N_LANES;
use stwo_constraint_framework::preprocessed_columns::PreProcessedColumnId;

use crate::components::{
    sha_256_big_sigma_0_o_0, sha_256_big_sigma_0_o_1, sha_256_big_sigma_1_o_0,
    sha_256_big_sigma_1_o_1, sha_256_small_sigma_0_o_0, sha_256_small_sigma_0_o_1,
    sha_256_small_sigma_1_o_0, sha_256_small_sigma_1_o_1,
};
use crate::preprocessed::PreProcessedColumn;
use crate::preprocessed_utils::pad;

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
            Sha256SigmaType::SmallSigma0O0 => sha_256_small_sigma_0_o_0::LOG_SIZE,
            Sha256SigmaType::SmallSigma0O1 => sha_256_small_sigma_0_o_1::LOG_SIZE,
            Sha256SigmaType::SmallSigma1O0 => sha_256_small_sigma_1_o_0::LOG_SIZE,
            Sha256SigmaType::SmallSigma1O1 => sha_256_small_sigma_1_o_1::LOG_SIZE,
            Sha256SigmaType::BigSigma0O0 => sha_256_big_sigma_0_o_0::LOG_SIZE,
            Sha256SigmaType::BigSigma0O1 => sha_256_big_sigma_0_o_1::LOG_SIZE,
            Sha256SigmaType::BigSigma1O0 => sha_256_big_sigma_1_o_0::LOG_SIZE,
            Sha256SigmaType::BigSigma1O1 => sha_256_big_sigma_1_o_1::LOG_SIZE,
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
