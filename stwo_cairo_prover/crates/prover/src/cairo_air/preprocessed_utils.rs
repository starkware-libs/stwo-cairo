use stwo_prover::core::backend::simd::column::BaseColumn;
use stwo_prover::core::backend::simd::m31::PackedM31;
use stwo_prover::core::fields::m31::M31;

pub fn table_id_to_col_id(table_id: &str, col_index: usize) -> String {
    format!("{}_{}", table_id, col_index)
}

// Pads all rows below <padding_offset> with the first row. Uses the <get_m31> function to get the
// value in a given row and column.
pub fn pad<F>(get_m31: F, padding_offset: usize, col: usize) -> Vec<M31>
where
    F: Fn(usize, usize) -> M31,
{
    let n = padding_offset.next_power_of_two();
    (0..padding_offset)
        .map(|i| get_m31(i, col))
        .chain((padding_offset..n).map(|_| get_m31(0, col)))
        .collect()
}

// Packs <values> into a column of PackedM31.
pub fn pack(values: Vec<M31>) -> Vec<PackedM31> {
    BaseColumn::from_iter(values).data
}
