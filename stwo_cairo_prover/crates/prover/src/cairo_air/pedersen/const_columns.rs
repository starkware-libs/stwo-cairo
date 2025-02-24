use std::array::from_fn;
use std::ops::Neg;
use std::sync::LazyLock;
use std::thread::spawn;

use itertools::Itertools;
use starknet_curve::curve_params::{
    PEDERSEN_P0, PEDERSEN_P1, PEDERSEN_P2, PEDERSEN_P3, SHIFT_POINT,
};
use starknet_types_core::curve::{AffinePoint, ProjectivePoint};
use starknet_types_core::felt::Felt;
use stwo_cairo_common::preprocessed_consts::pedersen::{
    BITS_PER_WINDOW, NUM_WINDOWS, PEDERSEN_TABLE_N_ROWS, ROWS_PER_WINDOW,
};
use stwo_cairo_common::prover_types::cpu::{Felt252, FELT252_N_WORDS};
use stwo_prover::constraint_framework::preprocessed_columns::PreProcessedColumnId;
use stwo_prover::core::backend::simd::column::BaseColumn;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::fields::m31::BaseField;
use stwo_prover::core::poly::circle::{CanonicCoset, CircleEvaluation};
use stwo_prover::core::poly::BitReversedOrder;

use crate::cairo_air::preprocessed::PreProcessedColumn;

pub(super) static PEDERSEN_TABLE: LazyLock<PedersenPointsTable> =
    LazyLock::new(PedersenPointsTable::new);
pub const PEDERSEN_TABLE_N_COLUMNS: usize = FELT252_N_WORDS * 2;

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

    fn gen_column_simd(&self) -> CircleEvaluation<SimdBackend, BaseField, BitReversedOrder> {
        CircleEvaluation::new(
            CanonicCoset::new(self.log_size()).circle_domain(),
            BaseColumn::from_cpu(PEDERSEN_TABLE.column_data[self.index].clone()),
        )
    }
}

// A table with 2**23 rows, each containing a point on the Stark curve.
// The table is divided into 4 sections:
// 1. First 14 blocks of 2 ** 18 rows: Row k of block b contains -P_shift + 2**(18*b) * k * P_0
// 2. Next 16 rows: Row k contains -P_shift + k * P_1
// 3. Next 14 blocks of 2 ** 18 rows: Row k of block b contains -P_shift + 2**(18*b) * k * P_2
// 4. Next 16 rows: Row k contains -P_shift + k * P_3
pub(super) struct PedersenPointsTable {
    // The one copy of the column contents. Shared by all column instances.
    column_data: [Vec<BaseField>; PEDERSEN_TABLE_N_COLUMNS],

    rows: Vec<AffinePoint>,
}

impl PedersenPointsTable {
    #[allow(dead_code)] //  Will be used by the deduce_output of PartialEcMul
    pub fn get_row(&self, index: usize) -> AffinePoint {
        self.rows[index].clone()
    }

    fn new() -> Self {
        let rows = create_table_rows();
        Self {
            column_data: rows_to_columns(&rows),
            rows,
        }
    }
}

fn felt_batch_inverse(values: &[Felt]) -> Vec<Felt> {
    let mut partial_prods = Vec::with_capacity(values.len()); // Partial_prods[k] is the product of the first <k> elements
    let mut prod = Felt::ONE;
    for value in values.iter() {
        partial_prods.push(prod);
        prod *= value;
    }

    let mut partial_inverse = prod.inverse().expect("Input values should be invertible");

    let mut inverses_reversed = Vec::with_capacity(values.len());

    for i in 0..values.len() {
        // Loop invariant: partial_inverse is the inverse of the first N - i elements
        let inverse = partial_inverse * partial_prods[values.len() - i - 1];
        inverses_reversed.push(inverse);
        partial_inverse *= values[values.len() - i - 1];
    }

    inverses_reversed.reverse();
    inverses_reversed
}

fn create_block(point: &ProjectivePoint, n_rows: usize) -> Vec<AffinePoint> {
    // Initialize the accumulator to -SHIFT_POINT
    let mut p = ProjectivePoint::new(SHIFT_POINT.x(), SHIFT_POINT.y(), Felt::ONE).neg();

    // Compute the points in projective representation
    let mut block_points_xs: Vec<Felt> = Vec::with_capacity(n_rows);
    let mut block_points_ys: Vec<Felt> = Vec::with_capacity(n_rows);
    let mut block_points_zs: Vec<Felt> = Vec::with_capacity(n_rows);
    for _ in 0..n_rows {
        block_points_xs.push(p.x());
        block_points_ys.push(p.y());
        block_points_zs.push(p.z());
        p += point.clone();
    }

    // Batch-inverse the Z coordinates
    let z_inverses = felt_batch_inverse(&block_points_zs);

    // Compute the affine coordinates as (x / z, y / z)
    block_points_xs
        .iter()
        .zip(block_points_ys.iter())
        .zip(z_inverses.iter())
        .map(|((x, y), z_inv)| AffinePoint::new_unchecked(x * z_inv, y * z_inv))
        .collect()
}

fn create_p0_or_p2_section(point: &ProjectivePoint) -> Vec<AffinePoint> {
    let mut shifted_point = point.clone();
    let mut threads = vec![];
    for _ in 0..NUM_WINDOWS {
        let point_for_thread = shifted_point.clone();
        threads.push(spawn(move || {
            create_block(&point_for_thread, ROWS_PER_WINDOW)
        }));
        for _ in 0..BITS_PER_WINDOW {
            shifted_point = shifted_point.double();
        }
    }

    threads
        .into_iter()
        .map(|t| t.join().expect("Thread succeeded"))
        .concat()
}

fn create_table_rows() -> Vec<AffinePoint> {
    let mut rows = vec![];
    rows.extend(create_p0_or_p2_section(
        &ProjectivePoint::from_affine(PEDERSEN_P0.x(), PEDERSEN_P0.y()).expect("P0 is on curve"),
    ));
    rows.extend(create_block(
        &ProjectivePoint::from_affine(PEDERSEN_P1.x(), PEDERSEN_P1.y()).expect("P1 is on curve"),
        16,
    ));
    rows.extend(create_p0_or_p2_section(
        &ProjectivePoint::from_affine(PEDERSEN_P2.x(), PEDERSEN_P2.y()).expect("P2 is on curve"),
    ));
    rows.extend(create_block(
        &ProjectivePoint::from_affine(PEDERSEN_P3.x(), PEDERSEN_P3.y()).expect("P3 is on curve"),
        16,
    ));

    let padded_size = rows.len().next_power_of_two();
    for _ in 0..(padded_size - rows.len()) {
        rows.push(rows[0].clone());
    }

    rows
}

fn rows_to_columns(rows: &[AffinePoint]) -> [Vec<BaseField>; PEDERSEN_TABLE_N_COLUMNS] {
    let mut columns_data: [Vec<BaseField>; PEDERSEN_TABLE_N_COLUMNS] =
        from_fn(|_| Vec::with_capacity(rows.len()));
    for row in rows {
        let x_f252: Felt252 = row.x().into();
        let y_f252: Felt252 = row.y().into();
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
