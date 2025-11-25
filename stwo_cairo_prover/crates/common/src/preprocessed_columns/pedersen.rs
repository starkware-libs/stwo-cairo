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
use stwo::core::fields::m31::BaseField;
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

pub static PEDERSEN_TABLE: LazyLock<PedersenPointsTable> = LazyLock::new(PedersenPointsTable::new);

pub const PEDERSEN_TABLE_N_COLUMNS: usize = FELT252_N_WORDS * 2;

pub const BITS_PER_WINDOW: usize = 18;
pub const NUM_WINDOWS: usize = 252usize.div_ceil(BITS_PER_WINDOW);
pub const ROWS_PER_WINDOW: usize = 1 << BITS_PER_WINDOW;

pub const P_0_SECTION_START: usize = 0;
pub const P_2_SECTION_START: usize = P_0_SECTION_START + NUM_WINDOWS * ROWS_PER_WINDOW;
pub const P_13_SECTION_START: usize = P_2_SECTION_START + NUM_WINDOWS * ROWS_PER_WINDOW;
pub const PEDERSEN_TABLE_N_ROWS: usize = P_13_SECTION_START + 16 * 16;

// We don't use starknet_types_core::curve::AffinePoint because, as of 10/2025,
// its .x() and .y() getters are slow.
#[derive(Clone)]
pub struct SimpleAffinePoint {
    pub x: Felt,
    pub y: Felt,
}

#[derive(Debug)]
pub struct PedersenPoints {
    index: usize,
}

impl PedersenPoints {
    pub fn new(col: usize) -> Self {
        Self { index: col }
    }
}

impl PreProcessedColumn for PedersenPoints {
    fn log_size(&self) -> u32 {
        PEDERSEN_TABLE_N_ROWS.next_power_of_two().ilog2()
    }

    fn id(&self) -> PreProcessedColumnId {
        PreProcessedColumnId {
            id: format!("pedersen_points_{}", self.index),
        }
    }

    fn packed_at(&self, vec_row: usize) -> PackedM31 {
        let array = PEDERSEN_TABLE.column_data[self.index]
            [(vec_row * N_LANES)..((vec_row + 1) * N_LANES)]
            .try_into()
            .unwrap();
        PackedM31::from_array(array)
    }

    fn gen_column_simd(&self) -> CircleEvaluation<SimdBackend, BaseField, BitReversedOrder> {
        CircleEvaluation::new(
            CanonicCoset::new(self.log_size()).circle_domain(),
            BaseColumn::from_cpu(PEDERSEN_TABLE.column_data[self.index].clone()),
        )
    }
}

// A table with 2**23 rows, each containing a point on the Stark curve.
// The table is divided into 3 sections:
// 1. First 14 blocks of 2 ** 18 rows: Row k of block b contains -P_shift + 2**(18*b) * k * P_0
// 2. Next 14 blocks of 2 ** 18 rows: Row k of block b contains -P_shift + 2**(18*b) * k * P_2
// 3. Next 256 rows: Row k + (16 * l) contains 29 * P_shift + k * P_1 + l * P_3
pub struct PedersenPointsTable {
    // The one copy of the column contents. Shared by all column instances.
    column_data: [Vec<BaseField>; PEDERSEN_TABLE_N_COLUMNS],

    rows: Vec<SimpleAffinePoint>,
}

impl PedersenPointsTable {
    #[allow(dead_code)] //  Will be used by the deduce_output of PartialEcMul
    pub fn get_row(&self, index: usize) -> SimpleAffinePoint {
        self.rows[index].clone()
    }

    pub fn get_row_coordinates(&self, index: usize) -> [Felt252; 2] {
        let x_f252: Felt252 = PEDERSEN_TABLE.rows[index].x.into();
        let y_f252: Felt252 = PEDERSEN_TABLE.rows[index].y.into();
        [x_f252, y_f252]
    }

    fn new() -> Self {
        let rows = create_table_rows();
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

fn create_p1_and_p3_section() -> Vec<SimpleAffinePoint> {
    let first_start_point = &ProjectivePoint::from_affine(SHIFT_POINT.x(), SHIFT_POINT.y())
        .expect("SHIFT_POINT is on curve")
        * (2 * NUM_WINDOWS + 1);
    let p1 =
        ProjectivePoint::from_affine(PEDERSEN_P1.x(), PEDERSEN_P1.y()).expect("P1 is on curve");
    let p3 =
        ProjectivePoint::from_affine(PEDERSEN_P3.x(), PEDERSEN_P3.y()).expect("P3 is on curve");
    (0..16)
        .into_par_iter()
        .map(|window: u32| {
            let start_point = first_start_point.clone() + (&p3 * window);
            create_block(&start_point, &p1, 16)
        })
        .collect::<Vec<_>>()
        .into_iter()
        .concat()
}

fn create_p0_or_p2_section(point: &ProjectivePoint) -> Vec<SimpleAffinePoint> {
    let start_point =
        ProjectivePoint::new_unchecked(SHIFT_POINT.x(), SHIFT_POINT.y(), Felt::ONE).neg();
    (0..NUM_WINDOWS)
        .into_par_iter()
        .map(|window| {
            let mut base_point = point.clone();
            for _ in 0..(window * BITS_PER_WINDOW) {
                base_point = base_point.double();
            }
            create_block(&start_point, &base_point, ROWS_PER_WINDOW)
        })
        .collect::<Vec<_>>()
        .into_iter()
        .concat()
}

fn create_table_rows() -> Vec<SimpleAffinePoint> {
    let mut rows = vec![];
    rows.extend(create_p0_or_p2_section(
        &ProjectivePoint::from_affine(PEDERSEN_P0.x(), PEDERSEN_P0.y()).expect("P0 is on curve"),
    ));
    rows.extend(create_p0_or_p2_section(
        &ProjectivePoint::from_affine(PEDERSEN_P2.x(), PEDERSEN_P2.y()).expect("P2 is on curve"),
    ));
    rows.extend(create_p1_and_p3_section());

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
