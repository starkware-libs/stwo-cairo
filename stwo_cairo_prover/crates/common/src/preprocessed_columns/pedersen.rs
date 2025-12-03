use std::array::from_fn;
use std::ops::Neg;
use std::sync::LazyLock;

use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use starknet_curve::curve_params::{
    PEDERSEN_P0, PEDERSEN_P1, PEDERSEN_P2, PEDERSEN_P3, SHIFT_POINT,
};
use starknet_types_core::curve::ProjectivePoint;
use starknet_types_core::felt::Felt;
use stwo::core::fields::m31::{BaseField, M31};
use stwo::core::poly::circle::CanonicCoset;
use stwo::prover::backend::simd::column::BaseColumn;
use stwo::prover::backend::simd::m31::{PackedM31, N_LANES};
use stwo::prover::backend::simd::SimdBackend;
use stwo::prover::poly::circle::CircleEvaluation;
use stwo::prover::poly::BitReversedOrder;
use stwo_constraint_framework::preprocessed_columns::PreProcessedColumnId;

use super::felt_batch_inverse::felt_batch_inverse;
use super::preprocessed_trace::PreProcessedColumn;
use crate::prover_types::cpu::{Felt252, FELT252_N_WORDS};

pub static PEDERSEN_TABLE_9: LazyLock<PedersenPointsTable<9>> =
    LazyLock::new(PedersenPointsTable::new);
pub static PEDERSEN_TABLE_18: LazyLock<PedersenPointsTable<18>> =
    LazyLock::new(PedersenPointsTable::new);

pub const PEDERSEN_TABLE_N_COLUMNS: usize = FELT252_N_WORDS * 2;

pub type PedersenPointsWindowBits9 = PedersenPoints<9>;
pub type PedersenPointsWindowBits18 = PedersenPoints<18>;

// We don't use starknet_types_core::curve::AffinePoint because, as of 10/2025,
// its .x() and .y() getters are slow.
#[derive(Clone)]
pub struct SimpleAffinePoint {
    pub x: Felt,
    pub y: Felt,
}

#[derive(Debug)]
pub struct PedersenPoints<const BITS_PER_WINDOW: usize> {
    index: usize,
}

impl<const BITS_PER_WINDOW: usize> PedersenPoints<BITS_PER_WINDOW> {
    pub fn new(col: usize) -> Self {
        Self { index: col }
    }

    pub fn get_data(&self) -> Vec<M31> {
        match BITS_PER_WINDOW {
            9 => PEDERSEN_TABLE_9.column_data[self.index].clone(),
            18 => PEDERSEN_TABLE_18.column_data[self.index].clone(),
            _ => panic!("Unsupported window_bits value {BITS_PER_WINDOW}"),
        }
    }
}

impl<const BITS_PER_WINDOW: usize> PreProcessedColumn for PedersenPoints<BITS_PER_WINDOW> {
    fn log_size(&self) -> u32 {
        let n_rows = (2 * 252 / BITS_PER_WINDOW) << BITS_PER_WINDOW;
        n_rows.next_power_of_two().ilog2()
    }

    fn id(&self) -> PreProcessedColumnId {
        PreProcessedColumnId {
            id: format!("pedersen_points_{}", self.index),
        }
    }

    fn packed_at(&self, vec_row: usize) -> PackedM31 {
        let array = self.get_data()[(vec_row * N_LANES)..((vec_row + 1) * N_LANES)]
            .try_into()
            .unwrap();
        PackedM31::from_array(array)
    }

    fn gen_column_simd(&self) -> CircleEvaluation<SimdBackend, BaseField, BitReversedOrder> {
        CircleEvaluation::new(
            CanonicCoset::new(self.log_size()).circle_domain(),
            BaseColumn::from_cpu(self.get_data().clone()),
        )
    }
}

// TODO(DanC): Fix documentation.
// A table with 2**15 rows, each containing a point on the Pedersen elliptic curve.
// The table is divided into 2 sections:
// 1a. First 27 blocks of 2**9 rows: Row k of block b contains -P_shift + 2**(9*b) * k * P_0
// 1b. The 28th block of 2**9 rows: Row k + (l << 5) contains
//       -P_shift + 2**(9*27) * k * P_0 + l * P_1
// 2a. Next 27 blocks of 2**9 rows: Row k of block b contains -P_shift + 2**(9*b) * k * P_2
// 2b. The last block of 2**9 rows: Row k + (l << 5) contains
//       -P_shift + 2**(9*13) * k * P_2 + l * P_3
pub struct PedersenPointsTable<const BITS_PER_WINDOW: usize> {
    // The one copy of the column contents. Shared by all column instances.
    column_data: [Vec<BaseField>; PEDERSEN_TABLE_N_COLUMNS],

    rows: Vec<SimpleAffinePoint>,
}

impl<const BITS_PER_WINDOW: usize> PedersenPointsTable<BITS_PER_WINDOW> {
    #[allow(dead_code)] //  Will be used by the deduce_output of PartialEcMul
    pub fn get_row(&self, index: usize) -> SimpleAffinePoint {
        self.rows[index].clone()
    }

    pub fn get_row_coordinates(&self, index: usize) -> [Felt252; 2] {
        match BITS_PER_WINDOW {
            9 => {
                let x_f252: Felt252 = PEDERSEN_TABLE_9.rows[index].x.into();
                let y_f252: Felt252 = PEDERSEN_TABLE_9.rows[index].y.into();
                [x_f252, y_f252]
            }
            18 => {
                let x_f252: Felt252 = PEDERSEN_TABLE_18.rows[index].x.into();
                let y_f252: Felt252 = PEDERSEN_TABLE_18.rows[index].y.into();
                [x_f252, y_f252]
            }
            _ => panic!("Unsupported window_bits value {BITS_PER_WINDOW}"),
        }
    }

    fn new() -> Self {
        let rows = create_table_rows(BITS_PER_WINDOW);
        Self {
            column_data: rows_to_columns(&rows),
            rows,
        }
    }
}

fn create_block(
    start_point: &ProjectivePoint,
    base_point: &ProjectivePoint,
    n_rows: usize,
) -> Vec<SimpleAffinePoint> {
    let mut p = start_point.clone();

    // Compute the points in projective representation
    let mut block_points_xs: Vec<Felt> = Vec::with_capacity(n_rows);
    let mut block_points_ys: Vec<Felt> = Vec::with_capacity(n_rows);
    let mut block_points_zs: Vec<Felt> = Vec::with_capacity(n_rows);
    for _ in 0..n_rows {
        block_points_xs.push(p.x());
        block_points_ys.push(p.y());
        block_points_zs.push(p.z());
        p += base_point.clone();
    }

    // Batch-inverse the Z coordinates
    let z_inverses = felt_batch_inverse(&block_points_zs);

    // Compute the affine coordinates as (x / z, y / z)
    block_points_xs
        .iter()
        .zip_eq(block_points_ys.iter())
        .zip_eq(z_inverses.iter())
        .map(|((x, y), z_inv)| SimpleAffinePoint {
            x: x * z_inv,
            y: y * z_inv,
        })
        .collect()
}

fn create_low_section(window_bits: usize, point: &ProjectivePoint) -> Vec<SimpleAffinePoint> {
    let num_windows = 252 / window_bits;
    let rows_per_window = 1 << window_bits;

    let start_point =
        ProjectivePoint::new_unchecked(SHIFT_POINT.x(), SHIFT_POINT.y(), Felt::ONE).neg();
    (0..(num_windows - 1))
        .into_par_iter()
        .map(|window| {
            let mut base_point = point.clone();
            for _ in 0..(window * window_bits) {
                base_point = base_point.double();
            }
            create_block(&start_point, &base_point, rows_per_window)
        })
        .collect::<Vec<_>>()
        .into_iter()
        .concat()
}

fn create_high_section(
    window_bits: usize,
    low_point: &ProjectivePoint,
    high_point: &ProjectivePoint,
) -> Vec<SimpleAffinePoint> {
    let num_windows = 252 / window_bits;
    let bits_in_last_window = window_bits - 4;
    let rows_in_last_window = 1 << bits_in_last_window;

    let mut raised_low_point = low_point.clone();
    for _ in 0..((num_windows - 1) * window_bits) {
        raised_low_point = raised_low_point.double();
    }
    let first_start_point =
        &ProjectivePoint::new_unchecked(SHIFT_POINT.x(), SHIFT_POINT.y(), Felt::ONE).neg();
    (0..16)
        .into_par_iter()
        .map(|window: u32| {
            let start_point = first_start_point.clone() + (high_point * window);
            create_block(&start_point.clone(), &raised_low_point, rows_in_last_window)
        })
        .collect::<Vec<_>>()
        .into_iter()
        .concat()
}

fn create_table_rows(window_bits: usize) -> Vec<SimpleAffinePoint> {
    assert_eq!(252 % window_bits, 0);
    let num_windows = 252 / window_bits;
    let mut rows = vec![];
    let points = [
        &ProjectivePoint::from_affine(PEDERSEN_P0.x(), PEDERSEN_P0.y()).expect("P0 is on curve"),
        &ProjectivePoint::from_affine(PEDERSEN_P1.x(), PEDERSEN_P1.y()).expect("P1 is on curve"),
        &ProjectivePoint::from_affine(PEDERSEN_P2.x(), PEDERSEN_P2.y()).expect("P2 is on curve"),
        &ProjectivePoint::from_affine(PEDERSEN_P3.x(), PEDERSEN_P3.y()).expect("P3 is on curve"),
    ];
    rows.extend(create_low_section(window_bits, points[0]));
    rows.extend(create_high_section(window_bits, points[0], points[1]));
    rows.extend(create_low_section(window_bits, points[2]));
    rows.extend(create_high_section(window_bits, points[2], points[3]));

    assert!(rows.len() == ((2 * num_windows) << window_bits));

    let padded_size = rows.len().next_power_of_two();
    for _ in 0..(padded_size - rows.len()) {
        rows.push(rows[0].clone());
    }

    rows
}

fn rows_to_columns(rows: &[SimpleAffinePoint]) -> [Vec<BaseField>; PEDERSEN_TABLE_N_COLUMNS] {
    let mut columns_data: [Vec<BaseField>; PEDERSEN_TABLE_N_COLUMNS] =
        from_fn(|_| Vec::with_capacity(rows.len()));
    for row in rows {
        let x_f252: Felt252 = row.x.into();
        let y_f252: Felt252 = row.y.into();
        for (col_idx, value) in x_f252
            .get_limbs()
            .iter()
            .chain(y_f252.get_limbs().iter())
            .enumerate()
        {
            columns_data[col_idx].push(*value)
        }
    }

    columns_data
}
