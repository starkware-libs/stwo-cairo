use std::array::from_fn;
use std::ops::Neg;
use std::sync::LazyLock;

use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use starknet_curve::curve_params::{
    PEDERSEN_P0, PEDERSEN_P1, PEDERSEN_P2, PEDERSEN_P3, SHIFT_POINT,
};
use starknet_types_core::curve::{AffinePoint, ProjectivePoint};
use starknet_types_core::felt::Felt;
use stwo_cairo_common::preprocessed_consts::pedersen::{
    BITS_PER_WINDOW, NUM_WINDOWS, PEDERSEN_TABLE_N_ROWS, ROWS_PER_WINDOW,
};
use stwo_cairo_common::prover_types::cpu::{Felt252, FELT252_N_WORDS, M31};
// use stwo_cairo_common::prover_types::simd::PackedFelt252;
use stwo_prover::constraint_framework::preprocessed_columns::PreProcessedColumnId;
use stwo_prover::core::backend::simd::column::BaseColumn;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::fields::m31::BaseField;
use stwo_prover::core::poly::circle::{CanonicCoset, CircleEvaluation};
use stwo_prover::core::poly::BitReversedOrder;

use super::utils::felt_batch_inverse;
use crate::cairo_air::preprocessed::PreProcessedColumn;
use crate::cairo_air::preprocessed_utils::pad;

pub(super) static PEDERSEN_TABLE: LazyLock<PedersenPointsTable> =
    LazyLock::new(PedersenPointsTable::new);
pub const PEDERSEN_TABLE_N_COLUMNS: usize = FELT252_N_WORDS * 2;

use stwo_prover::core::backend::simd::m31::{PackedM31, N_LANES}; //

const LOG_N_ROWS: u32 = (PEDERSEN_TABLE_N_ROWS as u32).next_power_of_two().ilog2();
const N_PACKED_ROWS: usize = (2_u32.pow(LOG_N_ROWS)) as usize / N_LANES;

// pub fn pedersen_points_table_m31(row: usize, col: usize) -> M31 {
//     assert!(col < PEDERSEN_TABLE_N_COLUMNS);
//     assert!(row < PEDERSEN_TABLE_N_ROWS); // PEDERSEN_TABLE_N_ROWS

//     // let felt252_index = col / FELT252WIDTH27_N_WORDS;
//     // let m31_index = col % FELT252WIDTH27_N_WORDS;
//     // round_keys(row)[felt252_index].get_m31(m31_index)
//     PEDERSEN_TABLE.column_data[col][row] // correct?
// }

pub fn pedersen_points_table_m31(row: usize, col: usize) -> M31 {
    assert!(col < PEDERSEN_TABLE_N_COLUMNS);
    assert!(row < PEDERSEN_TABLE_N_ROWS); // PEDERSEN_TABLE_N_ROWS

    let felt252_index = col / FELT252_N_WORDS;
    let m31_index = col % FELT252_N_WORDS;
    pedersen_points_table_f252(row)[felt252_index].get_m31(m31_index)
    // PEDERSEN_TABLE.column_data[col][row] // correct?
}

pub fn pedersen_points_table_f252(index: usize) -> [Felt252; 2] {
    // POSEIDON_ROUND_KEYS[round].map(|k| Felt252Width27 { limbs: k })
    // let index_usize = index.0 as usize;
    let array1: [M31; FELT252_N_WORDS] = from_fn(|i| PEDERSEN_TABLE.column_data[i][index]);
    let array2: [M31; FELT252_N_WORDS] =
        from_fn(|i| PEDERSEN_TABLE.column_data[i + FELT252_N_WORDS][index]);
    let output1 = Felt252::from_limbs(&array1);
    let output2 = Felt252::from_limbs(&array2);

    [output1, output2]
}

#[derive(Debug)]
pub struct PedersenPoints {
    pub packed_limbs: [PackedM31; N_PACKED_ROWS],
    pub col: usize,
    // zxc: PedersenPointsTable,
    // zxc: PedersenPointsTable, //debug trait?
}

impl PedersenPoints {
    pub fn new(col: usize) -> Self {
        let packed_limbs =
            BaseColumn::from_iter(pad(pedersen_points_table_m31, PEDERSEN_TABLE_N_ROWS, col)).data;
        Self {
            packed_limbs: packed_limbs.try_into().unwrap(),
            col,
        }
    }

    pub fn packed_at(&self, vec_row: usize) -> PackedM31 {
        // // // PedersenPointsTable::new().column_data[self.index][vec_row]
        // // // let zxc = PEDERSEN_TABLE.column_data[self.index].clone();
        // // let zxc = PEDERSEN_TABLE.column_data[self.index][vec_row * N_LANES].clone();
        // // // unsafe { PackedM31::from_simd_unchecked(simd) }
        // let array = PEDERSEN_TABLE.column_data[self.index]
        //     [(vec_row * N_LANES)..((vec_row + 1) * N_LANES)]
        //     .try_into()
        //     .expect("Slice has incorrect length");
        // PackedM31::from_array(array)
        self.packed_limbs[vec_row] //
    }
}

impl PreProcessedColumn for PedersenPoints {
    fn log_size(&self) -> u32 {
        // PEDERSEN_TABLE_N_ROWS.next_power_of_two().ilog2()
        LOG_N_ROWS
    }

    fn id(&self) -> PreProcessedColumnId {
        PreProcessedColumnId {
            id: format!("pedersen_points_{}", self.col),
        }
    }

    // fn gen_column_simd(&self) -> CircleEvaluation<SimdBackend, BaseField, BitReversedOrder> {
    //     CircleEvaluation::new(
    //         CanonicCoset::new(self.log_size()).circle_domain(),
    //         BaseColumn::from_iter(pad(
    //             pedersen_points_table_m31,
    //             PEDERSEN_TABLE_N_ROWS,
    //             self.index,
    //         )), /* PEDERSEN_TABLE.
    //              * column_data[self.index].
    //              * clone()
    //              * BaseColumn::from_cpu( */
    //     )
    // }
    fn gen_column_simd(&self) -> CircleEvaluation<SimdBackend, BaseField, BitReversedOrder> {
        CircleEvaluation::new(
            CanonicCoset::new(LOG_N_ROWS).circle_domain(),
            BaseColumn::from_simd(self.packed_limbs.to_vec()),
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

// //(round: M31) -> [Felt252Width27; 3]
// pub fn pedersen_points_table_deduce_output(index: M31) -> [Felt252; 2] {
//     let index_usize = index.0 as usize;
//     let array1: [M31; FELT252_N_WORDS] = from_fn(|i| PEDERSEN_TABLE.column_data[i][index_usize]);
// //.clone()?     let array2: [M31; FELT252_N_WORDS] =
//         from_fn(|i| PEDERSEN_TABLE.column_data[i + FELT252_N_WORDS][index_usize]);
//     let output1 = Felt252::from_limbs(&array1);
//     let output2 = Felt252::from_limbs(&array2);
//     // self.get_row(index)
//     [output1, output2]
// }
// // fn deduce_output(&self, index: usize) -> AffinePoint {
// //     self.get_row(index)
// // }
// pub fn pedersen_points_table_deduce_output_packed(indexes: PackedM31) -> [PackedFelt252; 2] {
//     let index_array = indexes.to_array();

//     // let zxc = index_array
//     //     .iter()
//     //     .map(|&index| self.deduce_output(index))
//     //     .collect::<Vec<[Felt252; 2]>>();

//     let (output1_vec, output2_vec): (Vec<Felt252>, Vec<Felt252>) = index_array
//         .iter()
//         .map(|&index| pedersen_points_table_deduce_output(index))
//         .map(|[a, b]| (a, b))
//         .unzip();

//     let output1 = PackedFelt252::from_array(&output1_vec.try_into().unwrap());
//     let output2 = PackedFelt252::from_array(&output2_vec.try_into().unwrap());

//     [output1, output2]
// }

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
        .zip_eq(block_points_ys.iter())
        .zip_eq(z_inverses.iter())
        .map(|((x, y), z_inv)| AffinePoint::new_unchecked(x * z_inv, y * z_inv))
        .collect()
}

fn create_p0_or_p2_section(point: &ProjectivePoint) -> Vec<AffinePoint> {
    (0..NUM_WINDOWS)
        .into_par_iter()
        .map(|window| {
            let mut base_point = point.clone();
            for _ in 0..(window * BITS_PER_WINDOW) {
                base_point = base_point.double();
            }
            create_block(&base_point, ROWS_PER_WINDOW)
        })
        .collect::<Vec<_>>()
        .into_iter()
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
