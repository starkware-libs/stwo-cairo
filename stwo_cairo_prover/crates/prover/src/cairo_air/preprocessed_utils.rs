use std::array::from_fn;

use prover_types::simd::N_LANES;
use stwo_prover::core::backend::simd::m31::PackedM31;
use stwo_prover::core::fields::m31::M31;

pub fn table_id_to_col_id(table_id: &str, col_index: usize) -> String {
    format!("{}_{}", table_id, col_index)
}

// Packs the values of column <col> and rows <vec_row * N_LANES..vec_row * N_LANES + N_LANES> into a
// PackedM31. Pads all rows below <n_rows> with the first row. Uses the <get_m31> function to get
// the value in a given row and column.
pub fn pack_and_pad<F>(get_m31: F, n_rows: usize, vec_row: usize, col: usize) -> PackedM31
where
    F: Fn(usize, usize) -> M31,
{
    PackedM31::from_array(from_fn(|i| {
        let row = (vec_row * N_LANES) + i;
        match row {
            r if r < n_rows => get_m31(r, col),
            _ => get_m31(0, col),
        }
    }))
}
